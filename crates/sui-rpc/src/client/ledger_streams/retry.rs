use std::collections::hash_map::RandomState;
use std::error::Error as _;
use std::hash::BuildHasher;
use std::time::Duration;

use tonic::Code;
use tonic::Status;

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
/// All other statuses are terminal.
fn transient(status: &Status, phase: FailurePhase) -> bool {
    let dispatch_transient = matches!(
        status.code(),
        Code::Unavailable | Code::DeadlineExceeded | Code::ResourceExhausted | Code::Aborted
    );
    let reset_code = matches!(status.code(), Code::Cancelled | Code::Unknown);
    let transport_reset = reset_code && (phase == FailurePhase::Body || status.source().is_some());
    dispatch_transient || transport_reset
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
}
