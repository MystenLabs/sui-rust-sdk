use futures::Stream;

use super::super::Client;
use super::super::Result;
use super::types::CheckpointStreamFrame;
use super::types::CheckpointStreamRequest;
use super::types::EventStreamFrame;
use super::types::EventStreamRequest;
use super::types::LedgerStreamConfig;
use super::types::ListConfig;
use super::types::TransactionStreamFrame;
use super::types::TransactionStreamRequest;
use crate::proto::sui::rpc::v2::ListCheckpointsRequest;
use crate::proto::sui::rpc::v2::ListCheckpointsResponse;
use crate::proto::sui::rpc::v2::ListEventsRequest;
use crate::proto::sui::rpc::v2::ListEventsResponse;
use crate::proto::sui::rpc::v2::ListTransactionsRequest;
use crate::proto::sui::rpc::v2::ListTransactionsResponse;

impl Client {
    /// Paginates `ListCheckpoints` requests into a stream of raw response pages.
    ///
    /// Automatically follows `ItemLimit` and `ScanLimit` pagination until reaching the
    /// request's end bound or the ledger tip.
    pub fn list_checkpoints(
        &self,
        request: ListCheckpointsRequest,
    ) -> impl Stream<Item = Result<ListCheckpointsResponse>> + Send + 'static {
        self.list_checkpoints_with_config(request, ListConfig::default())
    }
    /// Performs [`Client::list_checkpoints`] using `config`.
    pub fn list_checkpoints_with_config(
        &self,
        request: ListCheckpointsRequest,
        config: ListConfig,
    ) -> impl Stream<Item = Result<ListCheckpointsResponse>> + Send + 'static {
        let _ = (request, config);
        unimplemented_stream()
    }

    /// Streams checkpoints indefinitely, automatically retrying transient errors.
    ///
    /// The request's read mask must include `sequence_number` (or `*`). To resume after a
    /// previous checkpoint, pass `cursor + 1` to [`CheckpointStreamStart::Checkpoint`].
    ///
    /// For a finite read that stops at a bound, use [`Client::list_checkpoints`].
    pub fn stream_checkpoints(
        &self,
        request: CheckpointStreamRequest,
    ) -> impl Stream<Item = Result<CheckpointStreamFrame>> + Send + 'static {
        self.stream_checkpoints_with_config(request, LedgerStreamConfig::default())
    }

    /// Performs [`Client::stream_checkpoints`] using `config`.
    pub fn stream_checkpoints_with_config(
        &self,
        request: CheckpointStreamRequest,
        config: LedgerStreamConfig,
    ) -> impl Stream<Item = Result<CheckpointStreamFrame>> + Send + 'static {
        let _ = (request, config);
        unimplemented_stream()
    }

    /// Paginates `ListTransactions` requests into a stream of raw response pages.
    ///
    /// Automatically follows `ItemLimit` and `ScanLimit` pagination until reaching the
    /// request's end bound or the ledger tip.
    pub fn list_transactions(
        &self,
        request: ListTransactionsRequest,
    ) -> impl Stream<Item = Result<ListTransactionsResponse>> + Send + 'static {
        self.list_transactions_with_config(request, ListConfig::default())
    }
    /// Performs [`Client::list_transactions`] using `config`.
    pub fn list_transactions_with_config(
        &self,
        request: ListTransactionsRequest,
        config: ListConfig,
    ) -> impl Stream<Item = Result<ListTransactionsResponse>> + Send + 'static {
        let _ = (request, config);
        unimplemented_stream()
    }

    /// Streams transactions indefinitely, automatically retrying transient errors.
    ///
    /// The request's read mask must include `checkpoint` and `transaction_index` (or `*`).
    /// To resume from a previous frame, pass `frame.cursor` to [`TransactionStreamStart::Resume`].
    ///
    /// For a finite read that stops at a bound, use [`Client::list_transactions`].
    pub fn stream_transactions(
        &self,
        request: TransactionStreamRequest,
    ) -> impl Stream<Item = Result<TransactionStreamFrame>> + Send + 'static {
        self.stream_transactions_with_config(request, LedgerStreamConfig::default())
    }

    /// Performs [`Client::stream_transactions`] using `config`.
    pub fn stream_transactions_with_config(
        &self,
        request: TransactionStreamRequest,
        config: LedgerStreamConfig,
    ) -> impl Stream<Item = Result<TransactionStreamFrame>> + Send + 'static {
        let _ = (request, config);
        unimplemented_stream()
    }

    /// Paginates `ListEvents` requests into a stream of raw response pages.
    ///
    /// Automatically follows `ItemLimit` and `ScanLimit` pagination until reaching the
    /// request's end bound or the ledger tip.
    pub fn list_events(
        &self,
        request: ListEventsRequest,
    ) -> impl Stream<Item = Result<ListEventsResponse>> + Send + 'static {
        self.list_events_with_config(request, ListConfig::default())
    }
    /// Performs [`Client::list_events`] using `config`.
    pub fn list_events_with_config(
        &self,
        request: ListEventsRequest,
        config: ListConfig,
    ) -> impl Stream<Item = Result<ListEventsResponse>> + Send + 'static {
        let _ = (request, config);
        unimplemented_stream()
    }

    /// Streams events indefinitely, automatically retrying transient errors.
    ///
    /// The request's read mask must include `checkpoint`, `transaction_index`, and `event_index`
    /// (or `*`). To resume from a previous frame, pass `frame.cursor` to
    /// [`EventStreamStart::Resume`].
    ///
    /// For a finite read that stops at a bound, use [`Client::list_events`].
    pub fn stream_events(
        &self,
        request: EventStreamRequest,
    ) -> impl Stream<Item = Result<EventStreamFrame>> + Send + 'static {
        self.stream_events_with_config(request, LedgerStreamConfig::default())
    }

    /// Performs [`Client::stream_events`] using `config`.
    pub fn stream_events_with_config(
        &self,
        request: EventStreamRequest,
        config: LedgerStreamConfig,
    ) -> impl Stream<Item = Result<EventStreamFrame>> + Send + 'static {
        let _ = (request, config);
        unimplemented_stream()
    }
}

fn unimplemented_stream<T>() -> futures::stream::Empty<Result<T>> {
    unimplemented!()
}
