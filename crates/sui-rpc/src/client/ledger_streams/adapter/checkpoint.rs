use prost::bytes::Bytes;
use tonic::Request;
use tonic::Status;
use tonic::codegen::BoxStream;

use super::super::super::Client;
use super::super::super::Result;
use super::super::observability::LedgerStreamFamily;
use super::super::types::CheckpointStreamFrame;
use super::BoundFields;
use super::KeyGranularity;
use super::LedgerFrame;
use super::ListScanDirection;
use super::Position;
use super::Progress;
use super::RpcFuture;
use super::SubscriptionAdapter;
use crate::proto::sui::rpc::v2::Checkpoint;
use crate::proto::sui::rpc::v2::ListCheckpointsRequest;
use crate::proto::sui::rpc::v2::ListCheckpointsResponse;
use crate::proto::sui::rpc::v2::QueryEnd;
use crate::proto::sui::rpc::v2::QueryOptions;
use crate::proto::sui::rpc::v2::SubscribeCheckpointsRequest;
use crate::proto::sui::rpc::v2::SubscribeCheckpointsResponse;
use crate::proto::sui::rpc::v2::Watermark;

pub(in crate::client::ledger_streams) struct CheckpointAdapter;

impl SubscriptionAdapter for CheckpointAdapter {
    const FAMILY: LedgerStreamFamily = LedgerStreamFamily::Checkpoint;
    const GRANULARITY: KeyGranularity = KeyGranularity::Checkpoint;

    type Item = Checkpoint;
    /// Checkpoint positions are dense, so `start_checkpoint` and `end_checkpoint` bound requests
    /// without opaque cursors.
    type Cursor = ();
    type Output = CheckpointStreamFrame;
    type ListRequest = ListCheckpointsRequest;
    type ListResponse = ListCheckpointsResponse;
    type SubscribeRequest = SubscribeCheckpointsRequest;
    type SubscribeResponse = SubscribeCheckpointsResponse;

    fn options(request: &Self::ListRequest) -> Option<&QueryOptions> {
        request.options.as_ref()
    }

    fn options_mut(request: &mut Self::ListRequest) -> &mut QueryOptions {
        request.options.get_or_insert_with(QueryOptions::default)
    }

    fn start_checkpoint(request: &Self::ListRequest) -> Option<u64> {
        request.start_checkpoint
    }

    fn end_checkpoint(request: &Self::ListRequest) -> Option<u64> {
        request.end_checkpoint
    }

    fn set_start_fields(request: &mut Self::ListRequest, fields: BoundFields) {
        request.start_checkpoint = Some(fields.checkpoint);
    }

    fn set_end_fields(request: &mut Self::ListRequest, fields: BoundFields) {
        request.end_checkpoint = Some(fields.checkpoint);
    }

    /// A checkpoint scan that ends at its checkpoint bound has covered the checkpoint at that
    /// bound: `end_checkpoint - 1` ascending, `start_checkpoint` descending.
    fn validate_checkpoint_bound(
        request: &Self::ListRequest,
        direction: ListScanDirection,
        checkpoint: Option<u64>,
    ) -> Result<()> {
        let expected_checkpoint = match direction {
            ListScanDirection::Ascending => {
                request.end_checkpoint.and_then(|end| end.checked_sub(1))
            }
            ListScanDirection::Descending => Some(request.start_checkpoint.unwrap_or(0)),
        };
        if checkpoint.is_some() && checkpoint != expected_checkpoint {
            Err(Status::data_loss(
                "List CheckpointBound watermark does not match the requested bound",
            ))
        } else {
            Ok(())
        }
    }

    fn cursor_bytes(_cursor: &Self::Cursor) -> Option<Bytes> {
        None
    }

    fn list_cursor(_cursor: Bytes) -> Self::Cursor {}

    fn extract_metadata(
        response: &Self::ListResponse,
    ) -> (bool, Option<&Watermark>, Option<&QueryEnd>) {
        (
            response.checkpoint.is_some(),
            response.watermark.as_ref(),
            response.end.as_ref(),
        )
    }

    fn into_list_item(response: Self::ListResponse) -> Option<Self::Item> {
        response.checkpoint
    }

    fn item_required(request: &Self::SubscribeRequest) -> bool {
        request.filter.is_none()
    }

    fn parse_live(
        response: Self::SubscribeResponse,
        item_required: bool,
    ) -> Result<LedgerFrame<Self::Item, Self::Cursor>> {
        let cursor = response.cursor.ok_or_else(|| {
            Status::data_loss("checkpoint subscription frame is missing its cursor")
        })?;
        if response.checkpoint.is_none() && item_required {
            return Err(Status::data_loss(
                "unfiltered checkpoint subscription frame is missing its checkpoint",
            ));
        }
        // `cursor` is the inclusive checkpoint covered, so the frame's position is the start of
        // the next checkpoint.
        let next_checkpoint = cursor.checked_add(1).ok_or_else(|| {
            Status::out_of_range("checkpoint subscription cursor cannot be resumed")
        })?;
        LedgerFrame::new(
            response.checkpoint,
            Progress {
                position: Position::checkpoint_start(next_checkpoint),
                cursor: (),
                checkpoint: Some(cursor),
            },
            Self::GRANULARITY,
        )
    }

    fn into_output(item: Option<Self::Item>, progress: Progress<Self::Cursor>) -> Self::Output {
        CheckpointStreamFrame {
            checkpoint: item,
            // Output frames lie past the start of the stream, so their position is at least 1.
            cursor: progress.position.checkpoint.saturating_sub(1),
        }
    }

    fn dispatch_list(
        mut client: Client,
        request: Request<Self::ListRequest>,
    ) -> RpcFuture<Self::ListResponse> {
        Box::pin(async move {
            let stream = client
                .ledger_client()
                .list_checkpoints(request)
                .await?
                .into_inner();
            Ok(Box::pin(stream) as BoxStream<Self::ListResponse>)
        })
    }

    fn dispatch_subscribe(
        mut client: Client,
        request: Request<Self::SubscribeRequest>,
    ) -> RpcFuture<Self::SubscribeResponse> {
        Box::pin(async move {
            let stream = client
                .subscription_client()
                .subscribe_checkpoints(request)
                .await?
                .into_inner();
            Ok(Box::pin(stream) as BoxStream<Self::SubscribeResponse>)
        })
    }
}
