use prost::bytes::Bytes;
use tonic::Request;
use tonic::codegen::BoxStream;

use super::super::super::Client;
use super::super::super::Result;
use super::super::observability::LedgerStreamFamily;
use super::super::types::EventStreamFrame;
use super::BoundFields;
use super::KeyGranularity;
use super::LedgerFrame;
use super::Progress;
use super::RpcFuture;
use super::SubscriptionAdapter;
use super::parse_opaque_live_frame;
use crate::proto::sui::rpc::v2::Event;
use crate::proto::sui::rpc::v2::ListEventsRequest;
use crate::proto::sui::rpc::v2::ListEventsResponse;
use crate::proto::sui::rpc::v2::QueryEnd;
use crate::proto::sui::rpc::v2::QueryOptions;
use crate::proto::sui::rpc::v2::SubscribeEventsRequest;
use crate::proto::sui::rpc::v2::SubscribeEventsResponse;
use crate::proto::sui::rpc::v2::Watermark;

pub(in crate::client::ledger_streams) struct EventAdapter;

impl SubscriptionAdapter for EventAdapter {
    const FAMILY: LedgerStreamFamily = LedgerStreamFamily::Event;
    const GRANULARITY: KeyGranularity = KeyGranularity::Event;

    type Item = Event;
    type Cursor = Bytes;
    type Output = EventStreamFrame;
    type ListRequest = ListEventsRequest;
    type ListResponse = ListEventsResponse;
    type SubscribeRequest = SubscribeEventsRequest;
    type SubscribeResponse = SubscribeEventsResponse;

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
        request.start_transaction_index = fields.transaction_index;
        request.start_event_index = fields.event_index;
    }

    fn set_end_fields(request: &mut Self::ListRequest, fields: BoundFields) {
        request.end_checkpoint = Some(fields.checkpoint);
        request.end_transaction_index = fields.transaction_index;
        request.end_event_index = fields.event_index;
    }

    fn cursor_bytes(cursor: &Self::Cursor) -> Option<Bytes> {
        Some(cursor.clone())
    }

    fn list_cursor(cursor: Bytes) -> Self::Cursor {
        cursor
    }

    fn extract_metadata(
        response: &Self::ListResponse,
    ) -> (bool, Option<&Watermark>, Option<&QueryEnd>) {
        (
            response.event.is_some(),
            response.watermark.as_ref(),
            response.end.as_ref(),
        )
    }

    fn into_list_item(response: Self::ListResponse) -> Option<Self::Item> {
        response.event
    }

    fn parse_live(
        response: Self::SubscribeResponse,
        _item_required: bool,
    ) -> Result<LedgerFrame<Self::Item, Self::Cursor>> {
        parse_opaque_live_frame(response.event, response.watermark, Self::GRANULARITY)
    }

    fn into_output(item: Option<Self::Item>, progress: Progress<Self::Cursor>) -> Self::Output {
        EventStreamFrame {
            event: item,
            cursor: progress.cursor,
            position: progress.position,
            covered_checkpoint: progress.checkpoint,
        }
    }

    fn dispatch_list(
        mut client: Client,
        request: Request<Self::ListRequest>,
    ) -> RpcFuture<Self::ListResponse> {
        Box::pin(async move {
            let stream = client
                .ledger_client()
                .list_events(request)
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
                .subscribe_events(request)
                .await?
                .into_inner();
            Ok(Box::pin(stream) as BoxStream<Self::SubscribeResponse>)
        })
    }
}
