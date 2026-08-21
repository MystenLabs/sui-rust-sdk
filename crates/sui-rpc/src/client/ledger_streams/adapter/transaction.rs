use prost::bytes::Bytes;
use tonic::Request;
use tonic::Status;
use tonic::codegen::BoxStream;

use super::super::super::Client;
use super::super::super::Result;
use super::super::observability::LedgerStreamFamily;
use super::super::types::TransactionStreamFrame;
use super::ListResponseParts;
use super::ListScanDirection;
use super::LiveFrame;
use super::PositionedItem;
use super::Progress;
use super::RpcFuture;
use super::SubscriptionAdapter;
use super::parse_opaque_live_frame;
use crate::proto::sui::rpc::v2::ExecutedTransaction;
use crate::proto::sui::rpc::v2::ListTransactionsRequest;
use crate::proto::sui::rpc::v2::ListTransactionsResponse;
use crate::proto::sui::rpc::v2::QueryEnd;
use crate::proto::sui::rpc::v2::QueryOptions;
use crate::proto::sui::rpc::v2::SubscribeTransactionsRequest;
use crate::proto::sui::rpc::v2::SubscribeTransactionsResponse;
use crate::proto::sui::rpc::v2::Watermark;

pub(in crate::client::ledger_streams) struct TransactionAdapter;

impl SubscriptionAdapter for TransactionAdapter {
    const FAMILY: LedgerStreamFamily = LedgerStreamFamily::Transaction;
    const REQUIRED_READ_MASK_FIELDS: &'static [&'static str] = &["checkpoint", "transaction_index"];
    const READ_MASK_REQUIREMENT: &'static str =
        "read_mask must include \"checkpoint\" and \"transaction_index\" or \"*\"";

    type Item = PositionedItem<ExecutedTransaction, (u64, u64)>;
    type ItemPosition = (u64, u64);
    type Cursor = Bytes;
    type Output = TransactionStreamFrame;
    type ListRequest = ListTransactionsRequest;
    type ListResponse = ListTransactionsResponse;
    type SubscribeRequest = SubscribeTransactionsRequest;
    type SubscribeResponse = SubscribeTransactionsResponse;

    fn list_read_mask(request: &Self::ListRequest) -> Option<&prost_types::FieldMask> {
        request.read_mask.as_ref()
    }

    fn list_request_from_subscribe(request: &Self::SubscribeRequest) -> Self::ListRequest {
        ListTransactionsRequest {
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
        Self::options_mut(request).after = Some(progress.cursor.clone());
        Ok(())
    }

    fn request_resume_position(request: &Self::ListRequest) -> Option<Progress<Self::Cursor>> {
        request
            .options
            .as_ref()
            .and_then(|options| options.after.clone())
            .map(|cursor| Progress {
                cursor,
                checkpoint: None,
            })
    }
    fn validate_checkpoint_bound(
        _request: &Self::ListRequest,
        _direction: ListScanDirection,
        _checkpoint: Option<u64>,
    ) -> Result<()> {
        Ok(())
    }
    fn item_position(item: &Self::Item) -> &Self::ItemPosition {
        item.position()
    }

    fn extract_metadata(
        response: &Self::ListResponse,
    ) -> (bool, Option<&Watermark>, Option<&QueryEnd>) {
        (
            response.transaction.is_some(),
            response.watermark.as_ref(),
            response.end.as_ref(),
        )
    }

    fn split_list(response: Self::ListResponse) -> Result<ListResponseParts<Self>> {
        let item = response.transaction.map(position_transaction).transpose()?;
        Ok(ListResponseParts {
            item,
            watermark: response.watermark,
        })
    }

    fn parse_live(
        response: Self::SubscribeResponse,
        _item_required: bool,
    ) -> Result<LiveFrame<Self::Item, Progress<Self::Cursor>>> {
        let item = response.transaction.map(position_transaction).transpose()?;
        parse_opaque_live_frame(item, response.watermark)
    }

    fn into_output(item: Option<Self::Item>, progress: Progress<Self::Cursor>) -> Self::Output {
        let transaction = item.map(PositionedItem::into_payload);
        TransactionStreamFrame {
            transaction,
            cursor: progress.cursor,
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
                .list_transactions(request)
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
                .subscribe_transactions(request)
                .await?
                .into_inner();
            Ok(Box::pin(stream) as BoxStream<Self::SubscribeResponse>)
        })
    }
}

fn position_transaction(
    transaction: ExecutedTransaction,
) -> Result<PositionedItem<ExecutedTransaction, (u64, u64)>> {
    let checkpoint = transaction
        .checkpoint
        .ok_or_else(|| Status::data_loss("transaction item is missing its checkpoint"))?;
    let transaction_index = transaction
        .transaction_index
        .ok_or_else(|| Status::data_loss("transaction item is missing its transaction index"))?;
    Ok(PositionedItem::new(
        transaction,
        (checkpoint, transaction_index),
    ))
}
