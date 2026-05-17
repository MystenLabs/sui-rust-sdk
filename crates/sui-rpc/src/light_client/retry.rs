//! Exponential-backoff retry primitives for the light-client ratchet.
//!
//! [`step`] is the building block: a call site that wants retry
//! semantics inlines its own request/await loop and calls [`step`] on
//! each error to decide whether to sleep and reissue or propagate.
//! The retry decision is centralized so all ratchet RPCs share the
//! same retryable-status set and backoff schedule.
//!
//! Why not a higher-order `with_backoff(closure)` helper? The natural
//! shape — `FnMut() -> impl Future<…>` — doesn't compose with
//! borrowed RPC state. `LedgerServiceClient<BoxedChannel>` isn't
//! `Clone` (its `BoxService` backend isn't), and `client: &mut Client`
//! gets re-borrowed inside the closure body, which can't escape the
//! `FnMut` invocation. Switching to `AsyncFnMut` works at the type
//! level but the resulting future can't satisfy `Send` for
//! `tokio::spawn` from the events streaming task. Inlining the loop
//! and delegating only the retry decision to [`step`] dodges both.

use std::time::Duration;

use super::RatchetConfig;
use super::error::LightClientError;

/// Decide whether the current error should be retried.
///
/// If `err` is a transient RPC failure and `attempt` is still under
/// `config.max_retries`, sleep for the backoff delay, bump `attempt`,
/// and return `Ok(())` — the caller should reissue. Otherwise
/// propagate `err` unchanged.
pub(crate) async fn step(
    config: &RatchetConfig,
    err: LightClientError,
    attempt: &mut u32,
) -> Result<(), LightClientError> {
    if !is_retryable(&err) || *attempt >= config.max_retries {
        return Err(err);
    }
    tokio::time::sleep(backoff_delay(config, *attempt)).await;
    *attempt = attempt.saturating_add(1);
    Ok(())
}

/// True if `err` is a transient gRPC failure that's worth retrying.
///
/// The retryable codes match upstream's reference light client:
/// network blips and load-shedding statuses, not protocol violations.
fn is_retryable(err: &LightClientError) -> bool {
    let LightClientError::Rpc(status) = err else {
        return false;
    };
    matches!(
        status.code(),
        tonic::Code::Unavailable
            | tonic::Code::DeadlineExceeded
            | tonic::Code::ResourceExhausted
            | tonic::Code::Aborted
    )
}

fn backoff_delay(config: &RatchetConfig, attempt: u32) -> Duration {
    let shift = attempt.min(20);
    let base = config
        .base_retry_delay
        .saturating_mul(1u32 << shift)
        .min(config.max_retry_delay);
    base.saturating_add(pseudo_jitter(attempt, config.retry_jitter))
}

/// Deterministic per-attempt jitter. Keeps concurrent retriers from
/// landing on the same instant without bringing in a `rand`
/// dependency for a single call.
fn pseudo_jitter(attempt: u32, ceiling: Duration) -> Duration {
    if ceiling.is_zero() {
        return Duration::ZERO;
    }
    let mix = u64::from(attempt).wrapping_mul(0x9E37_79B9_7F4A_7C15) as u128;
    let ceiling_ms = ceiling.as_millis().max(1);
    let offset_ms = (mix % ceiling_ms) as u64;
    Duration::from_millis(offset_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_sleep_config(max_retries: u32) -> RatchetConfig {
        RatchetConfig {
            max_retries,
            base_retry_delay: Duration::ZERO,
            max_retry_delay: Duration::ZERO,
            retry_jitter: Duration::ZERO,
            ..RatchetConfig::default()
        }
    }

    /// `step` permits retries up to the configured cap, then refuses.
    #[tokio::test]
    async fn step_retries_until_cap_then_surfaces_last_error() {
        let config = no_sleep_config(3);
        let mut attempt = 0u32;
        for _ in 0..3 {
            let err = LightClientError::Rpc(tonic::Status::unavailable("down"));
            step(&config, err, &mut attempt)
                .await
                .expect("retry allowed");
        }
        // Fourth call: attempt is now 3, equal to max_retries; refuse.
        let err = LightClientError::Rpc(tonic::Status::unavailable("down"));
        let result = step(&config, err, &mut attempt).await;
        assert!(matches!(result, Err(LightClientError::Rpc(_))));
        assert_eq!(attempt, 3);
    }

    /// A non-retryable error is propagated immediately, without
    /// touching the attempt counter.
    #[tokio::test]
    async fn step_passes_non_retryable_errors_through() {
        let config = no_sleep_config(5);
        let mut attempt = 0u32;
        let err = LightClientError::Rpc(tonic::Status::invalid_argument("nope"));
        let result = step(&config, err, &mut attempt).await;
        assert!(matches!(result, Err(LightClientError::Rpc(_))));
        assert_eq!(attempt, 0);
    }

    /// Disabling retry (`max_retries = 0`) refuses to retry even the
    /// first failure.
    #[tokio::test]
    async fn step_disabled_retry_refuses_immediately() {
        let config = no_sleep_config(0);
        let mut attempt = 0u32;
        let err = LightClientError::Rpc(tonic::Status::unavailable("down"));
        let result = step(&config, err, &mut attempt).await;
        assert!(matches!(result, Err(LightClientError::Rpc(_))));
        assert_eq!(attempt, 0);
    }

    /// Backoff delay grows exponentially, then plateaus at
    /// `max_retry_delay`.
    #[test]
    fn backoff_delay_caps_at_max() {
        let config = RatchetConfig {
            base_retry_delay: Duration::from_millis(100),
            max_retry_delay: Duration::from_millis(800),
            retry_jitter: Duration::ZERO,
            ..RatchetConfig::default()
        };
        // attempt 0: 100ms
        assert_eq!(backoff_delay(&config, 0), Duration::from_millis(100));
        // attempt 1: 200ms
        assert_eq!(backoff_delay(&config, 1), Duration::from_millis(200));
        // attempt 2: 400ms
        assert_eq!(backoff_delay(&config, 2), Duration::from_millis(400));
        // attempt 3: 800ms (would be 800; equals cap)
        assert_eq!(backoff_delay(&config, 3), Duration::from_millis(800));
        // attempt 4: would be 1600ms, capped to 800ms
        assert_eq!(backoff_delay(&config, 4), Duration::from_millis(800));
    }
}
