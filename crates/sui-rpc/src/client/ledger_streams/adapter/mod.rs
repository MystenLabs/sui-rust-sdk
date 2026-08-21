use std::future::Future;
use std::pin::Pin;

use prost::bytes::Bytes;
use tonic::Request;
use tonic::Status;
use tonic::codegen::BoxStream;

use super::super::Client;
use super::super::Result;
use super::observability::LedgerStreamFamily;
use crate::proto::sui::rpc::v2::Ordering;
use crate::proto::sui::rpc::v2::QueryEnd;
use crate::proto::sui::rpc::v2::QueryOptions;
use crate::proto::sui::rpc::v2::Watermark;

mod checkpoint;
mod event;
mod progress;
mod transaction;

pub(super) use checkpoint::CheckpointAdapter;
pub(super) use event::EventAdapter;
pub(super) use progress::CursorDomain;
pub(super) use progress::Progress;
pub(super) use transaction::TransactionAdapter;
pub(super) type RpcFuture<T> = Pin<Box<dyn Future<Output = Result<BoxStream<T>>> + Send + 'static>>;

/// Connects one checkpoint, transaction, or event protocol family to the shared driver.
pub(super) trait SubscriptionAdapter: Send + Sync + 'static {
    const FAMILY: LedgerStreamFamily;
    type Cursor: CursorDomain;
    type ListRequest: Clone + Send + Sync + 'static;
    type ListResponse: Send + 'static;

    /// Returns the List request's pagination and ordering options when present.
    fn options(request: &Self::ListRequest) -> Option<&QueryOptions>;
    /// Returns mutable List pagination and ordering options, inserting defaults when absent.
    fn options_mut(request: &mut Self::ListRequest) -> &mut QueryOptions;
    /// Returns the List request's inclusive lower checkpoint bound.
    fn start_checkpoint(request: &Self::ListRequest) -> Option<u64>;
    /// Returns the List request's exclusive upper checkpoint bound.
    fn end_checkpoint(request: &Self::ListRequest) -> Option<u64>;
    /// Validates a `CheckpointBound` ending against the request bounds.
    fn validate_checkpoint_bound(
        request: &Self::ListRequest,
        direction: ListScanDirection,
        checkpoint: Option<u64>,
    ) -> Result<()>;
    /// Extracts `(has_item, watermark, query_end)` metadata from a list response frame.
    fn extract_metadata(
        response: &Self::ListResponse,
    ) -> (bool, Option<&Watermark>, Option<&QueryEnd>);
    fn dispatch_list(
        client: Client,
        request: Request<Self::ListRequest>,
    ) -> RpcFuture<Self::ListResponse>;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ListScanDirection {
    Ascending,
    Descending,
}

impl ListScanDirection {
    pub(super) fn from_request<A: SubscriptionAdapter>(request: &A::ListRequest) -> Result<Self> {
        match A::options(request).and_then(|options| options.ordering) {
            None => Ok(Self::Ascending),
            Some(ordering) => match Ordering::try_from(ordering) {
                Ok(Ordering::Ascending) => Ok(Self::Ascending),
                Ok(Ordering::Descending) => Ok(Self::Descending),
                Err(_) => Err(Status::invalid_argument("List ordering is unknown")),
            },
        }
    }

    pub(super) fn resume_bound(self, options: &QueryOptions) -> &Option<Bytes> {
        match self {
            Self::Ascending => &options.after,
            Self::Descending => &options.before,
        }
    }

    pub(super) fn set_resume_bound(self, options: &mut QueryOptions, cursor: Bytes) {
        match self {
            Self::Ascending => options.after = Some(cursor),
            Self::Descending => options.before = Some(cursor),
        }
    }
}

pub(super) fn validate_typed_checkpoint_bound(
    start_checkpoint: Option<u64>,
    end_checkpoint: Option<u64>,
    direction: ListScanDirection,
    watermark_checkpoint: Option<u64>,
) -> Result<()> {
    let expected_checkpoint = match direction {
        ListScanDirection::Ascending => end_checkpoint.and_then(|end| end.checked_sub(1)),
        ListScanDirection::Descending => Some(start_checkpoint.unwrap_or(0)),
    };
    if watermark_checkpoint.is_some() && watermark_checkpoint != expected_checkpoint {
        Err(Status::data_loss(
            "List CheckpointBound watermark does not match the requested bound",
        ))
    } else {
        Ok(())
    }
}
