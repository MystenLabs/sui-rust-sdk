use std::time::Duration;

use sui_sdk_types::Address;

use crate::client::LedgerStreamConfig;
use crate::client::ListConfig;

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

    /// Maximum number of items to request per page, for both the
    /// `ListEvents` pages of the event stream and the `ListTransactions`
    /// pages of the settlement listing. The server clamps to 1000 per
    /// the v2 List API contract.
    pub page_size: u32,

    /// Buffer capacity of the mpsc channel between the streaming task
    /// and the consumer. Larger values smooth over bursts; smaller
    /// values apply more backpressure to the producer.
    pub channel_capacity: usize,

    /// Maximum number of consecutive retries of a transient RPC failure
    /// before the stream yields the failure as its terminal error. The
    /// count restarts each time the failing call makes progress, so it
    /// bounds how long the stream keeps retrying one outage, not how
    /// many retries it makes in total. It applies to the ledger event
    /// stream, the settlement listing, and the head proofs made while
    /// reconciling; the startup reads are not retried.
    /// `Some(0)` never retries. `None`, the default, retries
    /// indefinitely.
    pub max_consecutive_retries: Option<u32>,

    /// Delay before the first retry of a transient RPC failure. The
    /// ledger event stream and the settlement listing double the delay
    /// on each consecutive retry; reconciliation head proofs wait an
    /// additional `retry_backoff` per consecutive retry. Either way the
    /// delay is capped at 30 seconds before jitter.
    pub retry_backoff: Duration,

    /// Upper bound on random jitter added to each retry sleep, to avoid
    /// thundering-herd reconnects when multiple consumers see the same
    /// failure mode.
    pub retry_jitter: Duration,

    /// Longest time events at checkpoints the event stream has claimed
    /// complete stay buffered before the streaming task fetches the
    /// on-chain [`EventStreamHead`], reconciles it against the locally
    /// replayed MMR, and releases them. Once the event stream reports
    /// that it has caught up with the server, buffered events are
    /// reconciled without waiting for this interval, so it mainly
    /// bounds buffering while the stream is catching up. Smaller values
    /// release events to the consumer sooner at the cost of more OCS
    /// inclusion-proof round trips.
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
            max_consecutive_retries: None,
            retry_backoff: Duration::from_millis(500),
            retry_jitter: Duration::from_millis(250),
            head_check_interval: Duration::from_secs(30),
        }
    }

    /// Configuration for the ledger event stream.
    pub(super) fn event_stream_config(&self) -> LedgerStreamConfig {
        let mut stream_config = LedgerStreamConfig::default();
        stream_config.list_page_limit = Some(self.page_size);
        stream_config.base_retry_delay = self.retry_backoff;
        stream_config.retry_jitter = self.retry_jitter;
        stream_config.max_consecutive_retries = self.max_consecutive_retries;
        stream_config
    }

    /// Configuration for the settlement `ListTransactions` listing.
    pub(super) fn settlement_list_config(&self) -> ListConfig {
        let mut list_config = ListConfig::default();
        list_config.base_retry_delay = self.retry_backoff;
        list_config.retry_jitter = self.retry_jitter;
        list_config.max_consecutive_retries = self.max_consecutive_retries;
        list_config
    }
}
