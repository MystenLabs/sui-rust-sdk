use std::collections::hash_map::RandomState;
use std::error::Error as _;
use std::hash::BuildHasher;
use std::time::Duration;

use prost::Message as _;
use tonic::Code;
use tonic::Status;

use crate::proto::google::rpc::ErrorInfo;
use crate::proto::sui::rpc::v2::ErrorReason;

use super::observability::LedgerStreamEvent;
use super::observability::LedgerStreamFamily;
use super::observability::LedgerStreamObservability;
use super::observability::LedgerStreamOperation;
use super::observability::LedgerStreamStage;
use super::types::LedgerStreamConfig;

/// Where an error was encountered during an RPC interaction.
///
/// Tonic surfaces errors differently depending on whether the RPC failed while initiating the call
/// or while reading frames from an already-open response stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FailurePhase {
    /// Failed while sending the initial RPC request.
    Dispatch,
    /// Failed while reading chunks from the response stream.
    Body,
}

/// Classifies failures eligible for automatic retry.
///
/// `Unavailable`, `DeadlineExceeded`, `ResourceExhausted`, and `Aborted` are always retried.
///
/// `Cancelled` and `Unknown` use two heuristics based on where the error was observed:
/// - During `Dispatch` (initiating the RPC), server-sent `Cancelled` and `Unknown` are
///   conservatively treated as terminal. We only retry if Tonic attached an underlying error source
///   (`status.source().is_some()`), treating that as evidence of a lower-layer transport or connection
///   failure.
/// - During `Body` (streaming responses), stream interruptions (such as Envoy, ALB, or NAT gateway
///   timeouts resetting an HTTP/2 stream) surface through Tonic as `Cancelled` or `Unknown`. Because
///   `stream.next()` surfaces transport drops and server trailers identically, we treat body-phase
///   `Cancelled` and `Unknown` as retryable based on the observation phase alone, allowing the stream
///   to reconnect and resume from the latest cursor.
///
/// `OutOfRange` is retried when it carries the [`ErrorReason::AboveLatestAvailable`] reason for a
/// cursor (see [`cursor_above_latest_available`]): the request reached a server, such as a lagging
/// replica, that has not yet indexed the cursor's position, and the same request succeeds once
/// that server catches up. Every other `OutOfRange` is terminal, including data below the pruned
/// floor ([`ErrorReason::BelowEarliestAvailable`]), a checkpoint bound above the tip, which can
/// name a checkpoint the chain has not reached, and an `OutOfRange` without a reason.
///
/// All other statuses are terminal.
fn transient(status: &Status, phase: FailurePhase) -> bool {
    let dispatch_transient = matches!(
        status.code(),
        Code::Unavailable | Code::DeadlineExceeded | Code::ResourceExhausted | Code::Aborted
    );
    let reset_code = matches!(status.code(), Code::Cancelled | Code::Unknown);
    let transport_reset = reset_code && (phase == FailurePhase::Body || status.source().is_some());
    dispatch_transient || transport_reset || cursor_above_latest_available(status)
}

/// Whether `status` is an `OutOfRange` whose status details carry an `ErrorInfo` with reason
/// [`ErrorReason::AboveLatestAvailable`] and `bound` metadata `"cursor"`. A cursor comes from a
/// server that had indexed its position, so the wait is bounded by how far replicas lag each
/// other.
fn cursor_above_latest_available(status: &Status) -> bool {
    if status.code() != Code::OutOfRange {
        return false;
    }
    let Ok(details) = crate::proto::google::rpc::Status::decode(status.details()) else {
        return false;
    };
    details
        .details
        .iter()
        .filter_map(|detail| detail.to_msg::<ErrorInfo>().ok())
        .any(|error_info| {
            error_info.domain == ErrorReason::DOMAIN
                && error_info.reason == ErrorReason::AboveLatestAvailable.as_str_name()
                && error_info
                    .metadata
                    .get("bound")
                    .is_some_and(|bound| bound == "cursor")
        })
}

pub(super) struct RetryState {
    family: LedgerStreamFamily,
    operation: LedgerStreamOperation,
    consecutive_errors: u32,
    first_failure: Option<tokio::time::Instant>,
    started_in: Option<LedgerStreamStage>,
}

impl RetryState {
    pub(super) fn new(family: LedgerStreamFamily, operation: LedgerStreamOperation) -> Self {
        Self {
            family,
            operation,
            consecutive_errors: 0,
            first_failure: None,
            started_in: None,
        }
    }

    pub(super) fn reset(&mut self, observability: &LedgerStreamObservability) {
        if self.consecutive_errors == 0 {
            return;
        }
        let consecutive_failures = self.consecutive_errors;
        self.consecutive_errors = 0;
        let first_failure = self.first_failure.take();
        let started_in = self.started_in.take();
        if let (Some(first_failure), Some(started_in)) = (first_failure, started_in) {
            observability.emit(|| LedgerStreamEvent::RetryRecovered {
                family: self.family,
                operation: self.operation,
                started_in,
                consecutive_failures,
                elapsed: first_failure.elapsed(),
            });
        }
    }

    pub(super) fn retry_delay(
        &mut self,
        status: &Status,
        phase: FailurePhase,
        config: &LedgerStreamConfig,
        observability: &LedgerStreamObservability,
        stage: LedgerStreamStage,
    ) -> Option<Duration> {
        if !transient(status, phase) {
            return None;
        }

        let attempt = self.consecutive_errors;
        self.consecutive_errors = self.consecutive_errors.saturating_add(1);
        if attempt == 0 {
            let first_failure = observability.start_timer();
            self.started_in = first_failure.as_ref().map(|_| stage);
            self.first_failure = first_failure;
        }
        let delay = backoff_delay(config, attempt);
        let consecutive_failures = self.consecutive_errors;
        observability.emit(|| LedgerStreamEvent::RetryScheduled {
            family: self.family,
            operation: self.operation,
            stage,
            status: status.clone(),
            consecutive_failures,
            delay,
        });
        Some(delay)
    }
}

pub(super) fn backoff_delay(config: &LedgerStreamConfig, attempt: u32) -> Duration {
    let multiplier = 2_u32.saturating_pow(attempt);
    let exponential = config.base_retry_delay.saturating_mul(multiplier);
    let capped = exponential.min(config.max_retry_delay);
    capped.saturating_add(random_jitter(config.retry_jitter))
}

fn random_jitter(maximum: Duration) -> Duration {
    if maximum.is_zero() {
        return Duration::ZERO;
    }
    let maximum_nanos = u64::try_from(maximum.as_nanos()).unwrap_or(u64::MAX);
    // Hashing with a fresh `RandomState`, randomly seeded per instance, yields an unpredictable
    // sample without a `rand` dependency.
    let sample = RandomState::new().hash_one(());
    let jitter_nanos = if maximum_nanos == u64::MAX {
        sample
    } else {
        sample % (maximum_nanos + 1)
    };
    Duration::from_nanos(jitter_nanos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_is_terminal_in_both_phases() {
        let status = Status::internal("client idle-body watchdog task panicked");
        assert!(!transient(&status, FailurePhase::Body));
        assert!(!transient(&status, FailurePhase::Dispatch));
    }

    #[test]
    fn server_reported_stream_reset_codes_are_transient_only_mid_body() {
        for status in [
            Status::cancelled("server cancelled request"),
            Status::unknown("server failed request"),
        ] {
            assert!(transient(&status, FailurePhase::Body));
            assert!(!transient(&status, FailurePhase::Dispatch));
        }
    }

    #[test]
    fn transport_sourced_unknown_is_transient_during_dispatch() {
        let status = Status::from_error(Box::new(std::io::Error::from(
            std::io::ErrorKind::ConnectionReset,
        )));
        assert_eq!(status.code(), Code::Unknown);
        assert!(status.source().is_some());
        assert!(transient(&status, FailurePhase::Dispatch));
    }

    #[test]
    fn dispatch_set_is_transient_in_both_phases() {
        let status = Status::unavailable("connection refused");
        assert!(transient(&status, FailurePhase::Dispatch));
        assert!(transient(&status, FailurePhase::Body));
    }

    fn out_of_range_with_reason(reason: ErrorReason, bound: Option<&str>) -> Status {
        let mut error_info = ErrorInfo::default()
            .with_domain(ErrorReason::DOMAIN)
            .with_reason(reason.as_str_name());
        if let Some(bound) = bound {
            error_info
                .metadata
                .insert("bound".to_owned(), bound.to_owned());
        }
        let details = crate::proto::google::rpc::Status {
            code: Code::OutOfRange.into(),
            message: "requested data out of range".to_owned(),
            details: vec![prost_types::Any::from_msg(&error_info).unwrap()],
        };
        Status::with_details(
            Code::OutOfRange,
            details.message.clone(),
            details.encode_to_vec().into(),
        )
    }

    fn above_latest_available(bound: &str) -> Status {
        out_of_range_with_reason(ErrorReason::AboveLatestAvailable, Some(bound))
    }

    #[test]
    fn cursor_above_latest_available_is_transient_in_both_phases() {
        let status = above_latest_available("cursor");
        assert!(transient(&status, FailurePhase::Dispatch));
        assert!(transient(&status, FailurePhase::Body));
    }

    #[test]
    fn checkpoint_bound_above_latest_available_is_terminal() {
        for bound in ["start_checkpoint", "end_checkpoint"] {
            let status = above_latest_available(bound);
            assert!(!transient(&status, FailurePhase::Dispatch), "{bound}");
            assert!(!transient(&status, FailurePhase::Body), "{bound}");
        }
    }

    #[test]
    fn out_of_range_without_a_cursor_past_the_tip_is_terminal() {
        let below_earliest_available =
            out_of_range_with_reason(ErrorReason::BelowEarliestAvailable, None);
        let missing_bound = out_of_range_with_reason(ErrorReason::AboveLatestAvailable, None);
        let without_reason = Status::out_of_range("requested data above latest available");
        for status in [below_earliest_available, missing_bound, without_reason] {
            assert!(!transient(&status, FailurePhase::Dispatch), "{status:?}");
            assert!(!transient(&status, FailurePhase::Body), "{status:?}");
        }
    }
}
