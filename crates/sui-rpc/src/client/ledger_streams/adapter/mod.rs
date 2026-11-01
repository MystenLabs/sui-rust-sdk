use std::future::Future;
use std::pin::Pin;

use prost::Message;
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

pub(super) use super::types::Position;
pub(super) use checkpoint::CheckpointAdapter;
pub(super) use event::EventAdapter;
pub(super) use progress::BoundFields;
pub(super) use progress::KeyGranularity;
pub(super) use progress::LedgerFrame;
pub(super) use progress::Progress;
pub(super) use progress::RecoveryGap;
pub(super) use progress::missing_cursor_position;
pub(super) use transaction::TransactionAdapter;

pub(super) type RpcFuture<T> = Pin<Box<dyn Future<Output = Result<BoxStream<T>>> + Send + 'static>>;

/// Connects one checkpoint, transaction, or event protocol family to the shared driver.
pub(super) trait SubscriptionAdapter: Send + Sync + 'static {
    const FAMILY: LedgerStreamFamily;
    /// Fields of this family's item keys and positions.
    const GRANULARITY: KeyGranularity;

    type Item: Send + 'static;
    /// Opaque cursor carried with progress and sent back with its position.
    type Cursor: Clone + Send + 'static;
    type Output: Send + 'static;
    type ListRequest: Clone + Send + Sync + 'static;
    type ListResponse: Send + 'static;
    type SubscribeRequest: Clone + Send + 'static;
    type SubscribeResponse: Message + Send + 'static;

    /// Returns the List request's pagination and ordering options when present.
    fn options(request: &Self::ListRequest) -> Option<&QueryOptions>;
    /// Returns mutable List pagination and ordering options, inserting defaults when absent.
    fn options_mut(request: &mut Self::ListRequest) -> &mut QueryOptions;
    /// Returns the List request's inclusive lower checkpoint bound.
    fn start_checkpoint(request: &Self::ListRequest) -> Option<u64>;
    /// Returns the List request's exclusive upper checkpoint bound.
    fn end_checkpoint(request: &Self::ListRequest) -> Option<u64>;
    /// Replaces the List request's start checkpoint and start indexes.
    fn set_start_fields(request: &mut Self::ListRequest, fields: BoundFields);
    /// Replaces the List request's end checkpoint and end indexes.
    fn set_end_fields(request: &mut Self::ListRequest, fields: BoundFields);
    /// Validates the coverage a `CheckpointBound` ending reports against the request bounds.
    fn validate_checkpoint_bound(
        _request: &Self::ListRequest,
        _direction: ListScanDirection,
        _checkpoint: Option<u64>,
    ) -> Result<()> {
        Ok(())
    }
    /// The opaque bytes to send with a position, when this family's cursor has them.
    fn cursor_bytes(cursor: &Self::Cursor) -> Option<Bytes>;
    /// Converts a List watermark cursor to this family's cursor.
    fn list_cursor(cursor: Bytes) -> Self::Cursor;
    /// Extracts `(has_item, watermark, query_end)` metadata from a list response frame.
    fn extract_metadata(
        response: &Self::ListResponse,
    ) -> (bool, Option<&Watermark>, Option<&QueryEnd>);
    /// Takes the item payload from a List response.
    fn into_list_item(response: Self::ListResponse) -> Option<Self::Item>;
    /// Whether successful live frames must contain a payload item.
    fn item_required(_request: &Self::SubscribeRequest) -> bool {
        false
    }
    /// Validates and converts a Subscribe response to a keyed frame.
    fn parse_live(
        response: Self::SubscribeResponse,
        item_required: bool,
    ) -> Result<LedgerFrame<Self::Item, Self::Cursor>>;
    /// Converts an optional payload and its progress to a public frame.
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

/// Starts `request` at `position`, sending `cursor`, which names the same point, as
/// `options.after`.
pub(super) fn set_start<A: SubscriptionAdapter>(
    request: &mut A::ListRequest,
    position: Position,
    cursor: Option<Bytes>,
) {
    A::set_start_fields(request, position.start_fields());
    A::options_mut(request).after = cursor;
}

/// Ends `request` at `position`, sending `cursor`, which names the same point, as
/// `options.before`.
pub(super) fn set_end<A: SubscriptionAdapter>(
    request: &mut A::ListRequest,
    position: Position,
    cursor: Option<Bytes>,
) -> Result<()> {
    A::set_end_fields(request, position.end_fields()?);
    A::options_mut(request).before = cursor;
    Ok(())
}

/// Reads an ascending subscription frame, whose watermark must carry a cursor and its position.
fn parse_opaque_live_frame<I>(
    item: Option<I>,
    watermark: Option<Watermark>,
    granularity: KeyGranularity,
) -> Result<LedgerFrame<I, Bytes>> {
    let watermark = watermark
        .ok_or_else(|| Status::data_loss("subscription frame is missing its watermark"))?;
    let position = watermark
        .cursor_position
        .as_ref()
        .map(|position| granularity.position(position))
        .ok_or_else(|| missing_cursor_position("subscription"))?;
    let cursor = watermark
        .cursor
        .ok_or_else(|| Status::data_loss("subscription watermark is missing its cursor"))?;
    let progress = Progress {
        position,
        cursor,
        checkpoint: watermark.checkpoint,
    };
    LedgerFrame::new(item, progress, granularity)
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

    /// Whether `next` lies strictly further than `previous` in this scan direction.
    pub(super) fn beyond(self, next: Position, previous: Position) -> bool {
        match self {
            Self::Ascending => next > previous,
            Self::Descending => next < previous,
        }
    }

    /// Continues `request` in this direction from `progress`: ascending, starting at its
    /// position; descending, ending there. Sends the cursor in the matching slot.
    pub(super) fn set_resume<A: SubscriptionAdapter>(
        self,
        request: &mut A::ListRequest,
        progress: &Progress<Bytes>,
    ) -> Result<()> {
        match self {
            Self::Ascending => {
                set_start::<A>(request, progress.position, Some(progress.cursor.clone()));
                Ok(())
            }
            Self::Descending => {
                set_end::<A>(request, progress.position, Some(progress.cursor.clone()))
            }
        }
    }

    /// Whether `options` carries an opaque cursor in this direction's resume slot.
    pub(super) fn has_resume_cursor(self, options: &QueryOptions) -> bool {
        match self {
            Self::Ascending => options.after.is_some(),
            Self::Descending => options.before.is_some(),
        }
    }

    /// Whether `options` carries an opaque cursor in the slot that ends a scan in this direction.
    pub(super) fn has_far_cursor(self, options: &QueryOptions) -> bool {
        match self {
            Self::Ascending => options.before.is_some(),
            Self::Descending => options.after.is_some(),
        }
    }
}
