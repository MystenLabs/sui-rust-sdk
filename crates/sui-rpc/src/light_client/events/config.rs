use std::time::Duration;

use sui_sdk_types::Address;

/// Configuration for [`super::AuthenticatedEventsClient`].
///
/// All fields are public so callers can construct one inline. The
/// defaults match the upstream reference client's choices unless noted
/// otherwise.
#[derive(Clone, Debug)]
pub struct AuthenticatedEventsConfig {
    /// The stream id (a Sui address, typically the publishing package's
    /// id) to subscribe to.
    pub stream_id: Address,

    /// First checkpoint to read from. `None` means "start from the
    /// network's current tip at construction time".
    pub start_checkpoint: Option<u64>,

    /// Maximum number of events to request per `ListEvents` page. The
    /// server clamps to 1000 per the v2 List API contract.
    pub page_size: u32,

    /// Buffer capacity of the mpsc channel between the streaming task
    /// and the consumer. Larger values smooth over bursts; smaller
    /// values apply more backpressure to the producer.
    pub channel_capacity: usize,

    /// Maximum number of consecutive RPC errors to tolerate before the
    /// streaming task gives up and aborts the stream with the last
    /// error.
    pub max_connect_retries: u32,

    /// Base backoff between RPC retry attempts. Each subsequent attempt
    /// waits an additional `retry_backoff`.
    pub retry_backoff: Duration,

    /// Upper bound on random jitter added to each retry sleep, to avoid
    /// thundering-herd reconnects when multiple consumers see the same
    /// failure mode.
    pub retry_jitter: Duration,

    /// Cadence at which the streaming task fetches the on-chain
    /// [`EventStreamHead`] and reconciles it against the locally
    /// replayed MMR. Smaller values release events to the consumer
    /// faster at the cost of more OCS inclusion-proof round trips.
    ///
    /// [`EventStreamHead`]: sui_sdk_types::framework::EventStreamHead
    pub head_check_interval: Duration,
}

impl AuthenticatedEventsConfig {
    /// Construct a config with the [`Default`] field values, scoped to
    /// the given stream.
    pub fn new(stream_id: Address) -> Self {
        Self {
            stream_id,
            ..Self::default_for_dummy_stream()
        }
    }

    /// Default field values for every field except `stream_id`, which
    /// has no sensible default. Used by [`Self::new`] and the inline
    /// builder pattern below.
    fn default_for_dummy_stream() -> Self {
        Self {
            stream_id: Address::ZERO,
            start_checkpoint: None,
            page_size: 1000,
            channel_capacity: 256,
            max_connect_retries: 5,
            retry_backoff: Duration::from_millis(500),
            retry_jitter: Duration::from_millis(250),
            head_check_interval: Duration::from_secs(30),
        }
    }
}
