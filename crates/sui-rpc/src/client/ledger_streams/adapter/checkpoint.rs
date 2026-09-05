use tonic::Request;
use tonic::Status;
use tonic::codegen::BoxStream;

use super::super::super::Client;
use super::super::super::Result;
use super::super::observability::LedgerStreamFamily;
use super::super::types::CheckpointStreamFrame;
use super::CHECKPOINT_CURSOR_OVERFLOW;
use super::ListResponseParts;
use super::ListScanDirection;
use super::LiveFrame;
use super::PositionedItem;
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
    const REQUIRED_READ_MASK_FIELDS: &'static [&'static str] = &["sequence_number"];
    const READ_MASK_REQUIREMENT: &'static str =
        "read_mask must include \"sequence_number\" or \"*\"";

    type Item = PositionedItem<Checkpoint, u64>;
    type ItemPosition = u64;
    type Cursor = u64;
    type Output = CheckpointStreamFrame;
    type ListRequest = ListCheckpointsRequest;
    type ListResponse = ListCheckpointsResponse;
    type SubscribeRequest = SubscribeCheckpointsRequest;
    type SubscribeResponse = SubscribeCheckpointsResponse;

    fn list_read_mask(request: &Self::ListRequest) -> Option<&prost_types::FieldMask> {
        request.read_mask.as_ref()
    }

    fn list_request_from_subscribe(request: &Self::SubscribeRequest) -> Self::ListRequest {
        ListCheckpointsRequest {
            read_mask: request.read_mask.clone(),
            filter: request.filter.clone(),
            start_checkpoint: None,
            end_checkpoint: None,
            options: None,
        }
    }

    fn options(request: &Self::ListRequest) -> Option<&QueryOptions> {
        request.options.as_ref()
    }

    fn options_mut(request: &mut Self::ListRequest) -> &mut QueryOptions {
        request.options.get_or_insert_with(QueryOptions::default)
    }

    fn start_checkpoint(request: &Self::ListRequest) -> Option<u64> {
        request.start_checkpoint
    }

    fn set_start_checkpoint(request: &mut Self::ListRequest, checkpoint: Option<u64>) {
        request.start_checkpoint = checkpoint;
    }

    fn end_checkpoint(request: &Self::ListRequest) -> Option<u64> {
        request.end_checkpoint
    }

    fn set_end_checkpoint(request: &mut Self::ListRequest, checkpoint: Option<u64>) {
        request.end_checkpoint = checkpoint;
    }

    fn set_ascending_resume(
        request: &mut Self::ListRequest,
        progress: &Progress<Self::Cursor>,
    ) -> Result<()> {
        request.start_checkpoint = Some(
            progress
                .cursor
                .checked_add(1)
                .ok_or_else(|| Status::out_of_range(CHECKPOINT_CURSOR_OVERFLOW))?,
        );
        Self::options_mut(request).after = None;
        Ok(())
    }

    fn request_resume_position(request: &Self::ListRequest) -> Option<Progress<Self::Cursor>> {
        request
            .start_checkpoint
            .and_then(|start| start.checked_sub(1))
            .map(|cursor| Progress {
                cursor,
                checkpoint: Some(cursor),
            })
    }

    fn validate_checkpoint_bound(
        request: &Self::ListRequest,
        direction: ListScanDirection,
        checkpoint: Option<u64>,
    ) -> Result<()> {
        super::validate_typed_checkpoint_bound(
            request.start_checkpoint,
            request.end_checkpoint,
            direction,
            checkpoint,
        )
    }
    fn item_position(item: &Self::Item) -> &Self::ItemPosition {
        item.position()
    }

    fn extract_metadata(
        response: &Self::ListResponse,
    ) -> (bool, Option<&Watermark>, Option<&QueryEnd>) {
        (
            response.checkpoint.is_some(),
            response.watermark.as_ref(),
            response.end.as_ref(),
        )
    }

    fn split_list(response: Self::ListResponse) -> Result<ListResponseParts<Self>> {
        let item = response.checkpoint.map(|checkpoint| -> Result<Self::Item> {
            let position = checkpoint.sequence_number.ok_or_else(|| {
                Status::data_loss("List checkpoint item is missing its sequence number")
            })?;
            Ok(PositionedItem::new(checkpoint, position))
        });
        Ok(ListResponseParts {
            item: item.transpose()?,
            watermark: response.watermark,
        })
    }

    fn item_required(request: &Self::SubscribeRequest) -> bool {
        request.filter.is_none()
    }

    fn parse_live(
        response: Self::SubscribeResponse,
        item_required: bool,
    ) -> Result<LiveFrame<Self::Item, Progress<Self::Cursor>>> {
        let cursor = response.cursor.ok_or_else(|| {
            Status::data_loss("checkpoint subscription frame is missing its cursor")
        })?;
        if response.checkpoint.is_none() && item_required {
            return Err(Status::data_loss(
                "unfiltered checkpoint subscription frame is missing its checkpoint",
            ));
        }
        let item = response.checkpoint.map(|checkpoint| -> Result<Self::Item> {
            let position = checkpoint.sequence_number.ok_or_else(|| {
                Status::data_loss("checkpoint subscription item is missing its sequence number")
            })?;
            if position != cursor {
                return Err(Status::data_loss(
                    "checkpoint subscription item sequence does not match its cursor",
                ));
            }
            Ok(PositionedItem::new(checkpoint, position))
        });
        Ok(LiveFrame {
            item: item.transpose()?,
            progress: Progress {
                cursor,
                checkpoint: Some(cursor),
            },
        })
    }

    fn into_output(item: Option<Self::Item>, progress: Progress<Self::Cursor>) -> Self::Output {
        let checkpoint = item.map(PositionedItem::into_payload);
        CheckpointStreamFrame {
            checkpoint,
            cursor: progress.cursor,
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
