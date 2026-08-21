use std::future::Future;
use std::pin::Pin;

use prost::Message;
use prost::bytes::Bytes;
use prost_types::FieldMask;
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
pub(super) use progress::CursorGapUpper;
pub(super) use progress::Progress;
pub(super) use progress::ProgressAdvance;
pub(super) use progress::Recovery;
pub(super) use progress::RecoveryGap;
pub(super) use transaction::TransactionAdapter;

const CHECKPOINT_CURSOR_OVERFLOW: &str = "checkpoint subscription cursor cannot be resumed";

pub(super) type RpcFuture<T> = Pin<Box<dyn Future<Output = Result<BoxStream<T>>> + Send + 'static>>;

pub(super) struct PositionedItem<I, P> {
    payload: I,
    position: P,
}

impl<I, P> PositionedItem<I, P> {
    fn new(payload: I, position: P) -> Self {
        Self { payload, position }
    }

    pub(super) fn position(&self) -> &P {
        &self.position
    }

    fn into_payload(self) -> I {
        self.payload
    }
}

pub(super) struct ListResponseParts<A: SubscriptionAdapter + ?Sized> {
    pub(super) item: Option<A::Item>,
    pub(super) watermark: Option<Watermark>,
}

pub(super) struct LiveFrame<I, P> {
    pub(super) item: Option<I>,
    pub(super) progress: P,
}

pub(super) fn validate_caller_read_mask<A: SubscriptionAdapter>(
    read_mask: Option<&FieldMask>,
) -> Result<()> {
    let Some(read_mask) = read_mask else {
        return Err(Status::invalid_argument(A::READ_MASK_REQUIREMENT));
    };
    if read_mask.paths.is_empty() {
        return Err(Status::invalid_argument(A::READ_MASK_REQUIREMENT));
    }
    if read_mask.paths.iter().any(|path| path == "*") {
        return Ok(());
    }
    if A::REQUIRED_READ_MASK_FIELDS.iter().all(|required| {
        read_mask
            .paths
            .iter()
            .any(|path| path.as_str() == *required)
    }) {
        Ok(())
    } else {
        Err(Status::invalid_argument(A::READ_MASK_REQUIREMENT))
    }
}

/// Connects one checkpoint, transaction, or event protocol family to the shared driver.
pub(super) trait SubscriptionAdapter: Send + Sync + 'static {
    const FAMILY: LedgerStreamFamily;
    /// Scalar payload fields the state machine needs for ordering, deduplication, and restart.
    const REQUIRED_READ_MASK_FIELDS: &'static [&'static str];
    /// Error returned when a caller mask omits required identity fields.
    const READ_MASK_REQUIREMENT: &'static str;

    type Item: Send + 'static;
    /// Ledger identity used for ordering, deduplication, and resume.
    type ItemPosition: Clone + Eq + Ord + Send + 'static;
    /// Cursor carried by payload and progress-only frames.
    type Cursor: CursorDomain;
    type Output: Send + 'static;
    type ListRequest: Clone + Send + Sync + 'static;
    type ListResponse: Send + 'static;
    type SubscribeRequest: Clone + Send + 'static;
    type SubscribeResponse: Message + Send + 'static;

    /// Returns the caller's List projection unchanged.
    fn list_read_mask(request: &Self::ListRequest) -> Option<&FieldMask>;
    /// Builds an unbounded ascending recovery request from a live request.
    fn list_request_from_subscribe(request: &Self::SubscribeRequest) -> Self::ListRequest;
    /// Returns the List request's pagination and ordering options when present.
    fn options(request: &Self::ListRequest) -> Option<&QueryOptions>;
    /// Returns mutable List pagination and ordering options, inserting defaults when absent.
    fn options_mut(request: &mut Self::ListRequest) -> &mut QueryOptions;
    /// Returns the List request's inclusive lower checkpoint bound.
    fn start_checkpoint(request: &Self::ListRequest) -> Option<u64>;
    /// Sets the List request's inclusive lower checkpoint bound.
    fn set_start_checkpoint(request: &mut Self::ListRequest, checkpoint: Option<u64>);
    /// Returns the List request's exclusive upper checkpoint bound.
    fn end_checkpoint(request: &Self::ListRequest) -> Option<u64>;
    /// Sets the List request's exclusive upper checkpoint bound.
    fn set_end_checkpoint(request: &mut Self::ListRequest, checkpoint: Option<u64>);
    /// Advances an ascending List request strictly past `progress`.
    fn set_ascending_resume(
        request: &mut Self::ListRequest,
        progress: &Progress<Self::Cursor>,
    ) -> Result<()>;
    /// Returns progress covered by the request's resume bound, if any.
    fn request_resume_position(request: &Self::ListRequest) -> Option<Progress<Self::Cursor>>;
    /// Validates a `CheckpointBound` ending against the request bounds.
    fn validate_checkpoint_bound(
        request: &Self::ListRequest,
        direction: ListScanDirection,
        checkpoint: Option<u64>,
    ) -> Result<()>;
    /// Returns the item identity used for ordering and deduplication.
    fn item_position(item: &Self::Item) -> &Self::ItemPosition;
    /// Extracts `(has_item, watermark, query_end)` metadata from a list response frame.
    fn extract_metadata(
        response: &Self::ListResponse,
    ) -> (bool, Option<&Watermark>, Option<&QueryEnd>);
    /// Splits payload from progress and validates projected identity fields.
    fn split_list(response: Self::ListResponse) -> Result<ListResponseParts<Self>>;
    /// Whether successful live frames must contain a payload item.
    fn item_required(_request: &Self::SubscribeRequest) -> bool {
        false
    }
    /// Validates and converts a Subscribe response to a normalized live frame.
    fn parse_live(
        response: Self::SubscribeResponse,
        item_required: bool,
    ) -> Result<LiveFrame<Self::Item, Progress<Self::Cursor>>>;
    /// Converts an optional payload and committed progress to a public frame.
    fn into_output(item: Option<Self::Item>, progress: Progress<Self::Cursor>) -> Self::Output;
    fn dispatch_list(
        client: Client,
        request: Request<Self::ListRequest>,
    ) -> RpcFuture<Self::ListResponse>;
    fn dispatch_subscribe(
        client: Client,
        request: Request<Self::SubscribeRequest>,
    ) -> RpcFuture<Self::SubscribeResponse>;
}

fn parse_opaque_live_frame<I, P>(
    item: Option<PositionedItem<I, P>>,
    watermark: Option<Watermark>,
) -> Result<LiveFrame<PositionedItem<I, P>, Progress<Bytes>>> {
    let watermark = watermark
        .ok_or_else(|| Status::data_loss("subscription frame is missing its watermark"))?;
    let cursor = watermark
        .cursor
        .ok_or_else(|| Status::data_loss("subscription watermark is missing its cursor"))?;
    Ok(LiveFrame {
        item,
        progress: Progress {
            cursor,
            checkpoint: watermark.checkpoint,
        },
    })
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
