use std::fmt;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;

use super::observability::LedgerStreamEvent;
use super::observability::LedgerStreamObserver;
use super::observability::ListEvent;
use super::observability::ListObserver;
use crate::proto::sui::rpc::v2::Checkpoint;
use crate::proto::sui::rpc::v2::Event;
use crate::proto::sui::rpc::v2::EventFilter;
use crate::proto::sui::rpc::v2::ExecutedTransaction;
use crate::proto::sui::rpc::v2::TransactionFilter;
use prost::bytes::Bytes;
use prost_types::FieldMask;
/// How a stream follows the ledger.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Delivery {
    /// Follows the tip through SubscriptionService, repairing gaps through List.
    #[default]
    Subscribe,
    /// Follows the ledger via the List APIs, useful on endpoints without SubscriptionService.
    Poll,
}

/// Starting position for a checkpoint stream.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum CheckpointStreamStart {
    /// Begins at the chain tip (live for `Subscribe`, indexed for `Poll`) and emits an initial
    /// progress frame before subsequent items.
    #[default]
    Tip,
    /// Starts at and includes the checkpoint with this sequence number.
    Checkpoint(u64),
}

/// Starting position for a transaction stream.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum TransactionStreamStart {
    /// Begins at the chain tip (live for `Subscribe`, indexed for `Poll`) and emits an initial
    /// progress frame before subsequent items.
    #[default]
    Tip,
    /// Starts at and includes transactions from this checkpoint.
    Checkpoint(u64),
    /// Starts strictly after this server-validated opaque cursor.
    Resume(Bytes),
}

/// Starting position for an event stream.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum EventStreamStart {
    /// Begins at the chain tip (live for `Subscribe`, indexed for `Poll`) and emits an initial
    /// progress frame before subsequent items.
    #[default]
    Tip,
    /// Starts at and includes events from this checkpoint.
    Checkpoint(u64),
    /// Starts strictly after this server-validated opaque cursor.
    Resume(Bytes),
}

/// Request for a resumable checkpoint stream.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CheckpointStreamRequest {
    /// Projection applied to both List and Subscribe requests.
    pub read_mask: Option<FieldMask>,
    /// Transaction filter applied to checkpoints.
    pub filter: Option<TransactionFilter>,
    /// Logical starting position.
    pub start: CheckpointStreamStart,
    /// How the stream follows the ledger.
    pub delivery: Delivery,
}

impl CheckpointStreamRequest {
    /// Creates a request with the default start and delivery.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the response projection.
    pub fn with_read_mask(mut self, read_mask: impl Into<FieldMask>) -> Self {
        self.read_mask = Some(read_mask.into());
        self
    }

    /// Sets the transaction filter.
    pub fn with_filter(mut self, filter: impl Into<TransactionFilter>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Sets the logical starting position.
    pub fn with_start(mut self, start: impl Into<CheckpointStreamStart>) -> Self {
        self.start = start.into();
        self
    }

    /// Sets how the stream follows the ledger.
    pub fn with_delivery(mut self, delivery: impl Into<Delivery>) -> Self {
        self.delivery = delivery.into();
        self
    }
}

/// Request for a resumable transaction stream.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransactionStreamRequest {
    /// Projection applied to both List and Subscribe requests.
    pub read_mask: Option<FieldMask>,
    /// Filter applied to transactions.
    pub filter: Option<TransactionFilter>,
    /// Logical starting position.
    pub start: TransactionStreamStart,
    /// How the stream follows the ledger.
    pub delivery: Delivery,
}

impl TransactionStreamRequest {
    /// Creates a request with the default start and delivery.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the response projection.
    pub fn with_read_mask(mut self, read_mask: impl Into<FieldMask>) -> Self {
        self.read_mask = Some(read_mask.into());
        self
    }

    /// Sets the transaction filter.
    pub fn with_filter(mut self, filter: impl Into<TransactionFilter>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Sets the logical starting position.
    pub fn with_start(mut self, start: impl Into<TransactionStreamStart>) -> Self {
        self.start = start.into();
        self
    }

    /// Sets how the stream follows the ledger.
    pub fn with_delivery(mut self, delivery: impl Into<Delivery>) -> Self {
        self.delivery = delivery.into();
        self
    }
}

/// Request for a resumable event stream.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EventStreamRequest {
    /// Projection applied to both List and Subscribe requests.
    pub read_mask: Option<FieldMask>,
    /// Filter applied to events.
    pub filter: Option<EventFilter>,
    /// Logical starting position.
    pub start: EventStreamStart,
    /// How the stream follows the ledger.
    pub delivery: Delivery,
}

impl EventStreamRequest {
    /// Creates a request with the default start and delivery.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the response projection.
    pub fn with_read_mask(mut self, read_mask: impl Into<FieldMask>) -> Self {
        self.read_mask = Some(read_mask.into());
        self
    }

    /// Sets the event filter.
    pub fn with_filter(mut self, filter: impl Into<EventFilter>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Sets the logical starting position.
    pub fn with_start(mut self, start: impl Into<EventStreamStart>) -> Self {
        self.start = start.into();
        self
    }

    /// Sets how the stream follows the ledger.
    pub fn with_delivery(mut self, delivery: impl Into<Delivery>) -> Self {
        self.delivery = delivery.into();
        self
    }
}

/// A checkpoint frame with an optional payload and inclusive restart cursor.
///
/// Process `checkpoint` before persisting `cursor`. To resume after this checkpoint,
/// start the next stream from `CheckpointStreamStart::Checkpoint(cursor + 1)`.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct CheckpointStreamFrame {
    /// The checkpoint payload, or `None` for a progress-only frame.
    pub checkpoint: Option<Checkpoint>,
    /// The inclusive checkpoint cursor represented by this frame.
    pub cursor: u64,
}

/// A transaction frame with an optional payload and opaque exclusive restart cursor.
///
/// Process `transaction` before persisting `cursor`, then resume with
/// `TransactionStreamStart::Resume(cursor)`.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct TransactionStreamFrame {
    /// The executed transaction payload, or `None` for a progress-only frame.
    pub transaction: Option<ExecutedTransaction>,
    /// The server's opaque resume cursor as of this frame, passed through verbatim.
    pub cursor: Bytes,
    /// Checkpoint coverage reported with this frame, when known, for progress observability.
    pub covered_checkpoint: Option<u64>,
}

/// An event frame with an optional payload and opaque exclusive restart cursor.
///
/// Process `event` before persisting `cursor`, then resume with `EventStreamStart::Resume(cursor)`.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct EventStreamFrame {
    /// The event payload, or `None` for a progress-only frame.
    pub event: Option<Event>,
    /// The server's opaque resume cursor as of this frame, passed through verbatim.
    pub cursor: Bytes,
    /// Checkpoint coverage reported with this frame, when known, for progress observability.
    pub covered_checkpoint: Option<u64>,
}

/// Polling, retry, buffering, and observability controls for `stream_*` operations.
///
/// Client-level timeouts bound individual RPCs, not the total duration of the stream.
/// Finite `list_*` operations use the retry and observer controls in [`ListConfig`].
#[non_exhaustive]
#[derive(Clone)]
pub struct LedgerStreamConfig {
    /// Delay between polls when using `Poll` delivery. Must be non-zero.
    pub ledger_tip_poll_interval: Duration,
    /// Maximum items per internally built List request.
    pub list_page_limit: Option<u32>,
    /// Delay before the first transient retry.
    pub base_retry_delay: Duration,
    /// Maximum exponential portion of a retry delay.
    pub max_retry_delay: Duration,
    /// Maximum random jitter added to a retry delay.
    pub retry_jitter: Duration,
    /// Maximum number of live subscription items buffered during gap replay before reconnecting.
    pub max_buffered_live_items: NonZeroUsize,
    observer: Option<LedgerStreamObserver>,
}

impl fmt::Debug for LedgerStreamConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LedgerStreamConfig")
            .field("ledger_tip_poll_interval", &self.ledger_tip_poll_interval)
            .field("list_page_limit", &self.list_page_limit)
            .field("base_retry_delay", &self.base_retry_delay)
            .field("max_retry_delay", &self.max_retry_delay)
            .field("retry_jitter", &self.retry_jitter)
            .field("max_buffered_live_items", &self.max_buffered_live_items)
            .field("observer_configured", &self.observer.is_some())
            .finish()
    }
}

impl Default for LedgerStreamConfig {
    fn default() -> Self {
        Self {
            ledger_tip_poll_interval: Duration::from_secs(1),
            list_page_limit: None,
            base_retry_delay: Duration::from_millis(250),
            max_retry_delay: Duration::from_secs(30),
            retry_jitter: Duration::from_millis(500),
            max_buffered_live_items: NonZeroUsize::new(1_024).expect("non-zero constant"),
            observer: None,
        }
    }
}

impl LedgerStreamConfig {
    /// Installs the synchronous stream observer, replacing any prior callback.
    ///
    /// The callback must return quickly and not block; its panic propagates to the polling task.
    pub fn with_observer(
        mut self,
        observer: impl Fn(LedgerStreamEvent) + Send + Sync + 'static,
    ) -> Self {
        self.observer = Some(Arc::new(observer));
        self
    }

    pub(super) fn observer(&self) -> Option<LedgerStreamObserver> {
        self.observer.clone()
    }
}

/// Retry and observability controls for finite `list_*` operations.
#[non_exhaustive]
#[derive(Clone)]
pub struct ListConfig {
    /// Delay before the first transient retry.
    pub base_retry_delay: Duration,
    /// Maximum exponential portion of a retry delay.
    pub max_retry_delay: Duration,
    /// Maximum random jitter added to a retry delay.
    pub retry_jitter: Duration,
    observer: Option<ListObserver>,
}

impl fmt::Debug for ListConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ListConfig")
            .field("base_retry_delay", &self.base_retry_delay)
            .field("max_retry_delay", &self.max_retry_delay)
            .field("retry_jitter", &self.retry_jitter)
            .field("observer_configured", &self.observer.is_some())
            .finish()
    }
}

impl Default for ListConfig {
    fn default() -> Self {
        let stream_defaults = LedgerStreamConfig::default();
        Self {
            base_retry_delay: stream_defaults.base_retry_delay,
            max_retry_delay: stream_defaults.max_retry_delay,
            retry_jitter: stream_defaults.retry_jitter,
            observer: None,
        }
    }
}

impl ListConfig {
    /// Installs the synchronous List observer, replacing any prior callback.
    ///
    /// The callback must return quickly and not block; its panic propagates to the polling task.
    pub fn with_observer(mut self, observer: impl Fn(ListEvent) + Send + Sync + 'static) -> Self {
        self.observer = Some(Arc::new(observer));
        self
    }

    pub(super) fn into_stream_config(self) -> LedgerStreamConfig {
        let observer: Option<LedgerStreamObserver> = self.observer.map(|list_observer| {
            Arc::new(move |event: LedgerStreamEvent| {
                if let Some(list_event) = ListEvent::from_stream_event(event) {
                    list_observer(list_event);
                }
            }) as LedgerStreamObserver
        });

        LedgerStreamConfig {
            base_retry_delay: self.base_retry_delay,
            max_retry_delay: self.max_retry_delay,
            retry_jitter: self.retry_jitter,
            observer,
            ..LedgerStreamConfig::default()
        }
    }
}
