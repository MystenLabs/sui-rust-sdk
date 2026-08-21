use prost::bytes::Bytes;
use tonic::Request;
use tonic::codegen::BoxStream;

use super::super::super::Client;
use super::super::super::Result;
use super::super::observability::LedgerStreamFamily;
use super::ListScanDirection;
use super::RpcFuture;
use super::SubscriptionAdapter;
use crate::proto::sui::rpc::v2::ListEventsRequest;
use crate::proto::sui::rpc::v2::ListEventsResponse;
use crate::proto::sui::rpc::v2::QueryEnd;
use crate::proto::sui::rpc::v2::QueryOptions;
use crate::proto::sui::rpc::v2::Watermark;

pub(in crate::client::ledger_streams) struct EventAdapter;

impl SubscriptionAdapter for EventAdapter {
    const FAMILY: LedgerStreamFamily = LedgerStreamFamily::Event;

    type Cursor = Bytes;
    type ListRequest = ListEventsRequest;
    type ListResponse = ListEventsResponse;

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

    fn validate_checkpoint_bound(
        _request: &Self::ListRequest,
        _direction: ListScanDirection,
        _checkpoint: Option<u64>,
    ) -> Result<()> {
        Ok(())
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
}
