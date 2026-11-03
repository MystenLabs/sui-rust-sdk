use super::Client;
use super::TransactionStreamRequest;
use super::TransactionStreamStart;
use crate::field::FieldMaskUtil;
use crate::proto::TryFromProtoError;
use crate::proto::sui::rpc::v2::ExecuteTransactionRequest;
use crate::proto::sui::rpc::v2::ExecuteTransactionResponse;
use crate::proto::sui::rpc::v2::ExecutionError;
use crate::proto::sui::rpc::v2::GetEpochRequest;
use crate::proto::sui::rpc::v2::GetServiceInfoRequest;
use crate::proto::sui::rpc::v2::GetTransactionRequest;
use crate::proto::sui::rpc::v2::GetTransactionResponse;
use crate::proto::sui::rpc::v2::TransactionFilter;
use crate::proto::sui::rpc::v2::filter::transaction;
use futures::TryStreamExt;
use prost_types::FieldMask;
use std::fmt;
use std::time::Duration;
use tonic::Response;

/// Error types that can occur when executing a transaction and waiting for checkpoint
#[derive(Debug)]
#[non_exhaustive]
pub enum ExecuteAndWaitError {
    /// RPC Error (actual tonic::Status from the client/server)
    RpcError(tonic::Status),
    /// Request is missing the required transaction field
    MissingTransaction,
    /// Failed to parse/convert the transaction for digest calculation
    ProtoConversionError(TryFromProtoError),
    /// Transaction executed but checkpoint wait timed out
    CheckpointTimeout(Response<ExecuteTransactionResponse>),
    /// Transaction executed but the stream watching for its checkpoint failed
    CheckpointStreamError {
        response: Response<ExecuteTransactionResponse>,
        error: tonic::Status,
    },
}

impl std::fmt::Display for ExecuteAndWaitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RpcError(status) => write!(f, "RPC error: {status}"),
            Self::MissingTransaction => {
                write!(f, "Request is missing the required transaction field")
            }
            Self::ProtoConversionError(e) => write!(f, "Failed to convert transaction: {e}"),
            Self::CheckpointTimeout(_) => {
                write!(f, "Transaction executed but checkpoint wait timed out")
            }
            Self::CheckpointStreamError { error, .. } => {
                write!(
                    f,
                    "Transaction executed but checkpoint stream had an error: {error}"
                )
            }
        }
    }
}

impl std::error::Error for ExecuteAndWaitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::RpcError(status) => Some(status),
            Self::ProtoConversionError(e) => Some(e),
            Self::CheckpointStreamError { error, .. } => Some(error),
            Self::MissingTransaction => None,
            Self::CheckpointTimeout(_) => None,
        }
    }
}

impl Client {
    /// Executes a transaction and waits for it to be included in a checkpoint.
    ///
    /// Returns the execution response with `checkpoint` and `timestamp`
    /// populated.
    ///
    /// The method:
    /// 1. Reads the node's checkpoint height `C` with `GetServiceInfo`.
    /// 2. Looks the transaction up with `GetTransaction`. A transaction that is
    ///    already in a checkpoint (a resubmission, or the same transaction sent
    ///    by someone else) is returned without being executed again.
    /// 3. Executes the transaction while following the sender's transactions
    ///    from checkpoint `C` with [`Client::stream_transactions`], and returns
    ///    once the transaction appears. The stream replays checkpoints it has
    ///    not seen and reconnects after transient failures, and the sender
    ///    filter keeps it to the sender's own transactions.
    ///
    /// `timeout` bounds the wait that starts once execution returns. If it
    /// expires, or the stream fails, the error still carries the execution
    /// response.
    ///
    /// If execution fails, the ledger is checked once more: when the
    /// transaction is in a checkpoint anyway (for example, a resubmission
    /// raced the original submission), the committed transaction is returned
    /// instead of the error.
    ///
    /// A response answered from the ledger is assembled from `GetTransaction`
    /// using the request's read mask, so it carries the same fields an
    /// execution response would, with `digest`, `checkpoint`, and `timestamp`
    /// always populated.
    ///
    /// # Consistency
    ///
    /// The transaction being in a checkpoint does not mean every index
    /// includes it: the List APIs (`ListTransactions`, `ListEvents`, ...) read
    /// indexes that can trail the latest checkpoint.
    ///
    /// Each step can reach a different node when the client sits behind a load
    /// balancer. If the lookup in step 2 reaches a node that is behind `C` and
    /// misses a transaction committed at or before `C`, the stream never sees
    /// it and the call ends in [`ExecuteAndWaitError::CheckpointTimeout`].
    /// Sending `C` as the `x-sui-min-checkpoint` floor on these requests closes
    /// that gap on servers that enforce it; this method does not send it yet.
    #[allow(clippy::result_large_err)]
    pub async fn execute_transaction_and_wait_for_checkpoint(
        &mut self,
        request: impl tonic::IntoRequest<ExecuteTransactionRequest>,
        timeout: Duration,
    ) -> Result<Response<ExecuteTransactionResponse>, ExecuteAndWaitError> {
        // Calculate digest from the input transaction to avoid relying on response read mask
        let request = request.into_request();
        let transaction = request
            .get_ref()
            .transaction_opt()
            .ok_or(ExecuteAndWaitError::MissingTransaction)?;
        let transaction = sui_sdk_types::Transaction::try_from(transaction)
            .map_err(ExecuteAndWaitError::ProtoConversionError)?;
        let digest = transaction.digest().to_string();

        // Lookup for answering from the ledger instead of from execution.
        // Both RPCs' masks select fields of `ExecutedTransaction`, so the
        // caller's mask passes through unchanged; when the caller didn't set
        // one, mirror ExecuteTransaction's documented default. `digest`,
        // `checkpoint`, and `timestamp` are always included since this
        // method's contract populates them.
        let lookup_request = {
            let caller_paths = match &request.get_ref().read_mask {
                Some(mask) => mask.paths.clone(),
                None => vec!["effects.status".to_owned()],
            };
            let mask = FieldMask::from_paths(caller_paths.iter().map(String::as_str).chain([
                "digest",
                "checkpoint",
                "timestamp",
            ]))
            .normalize();
            GetTransactionRequest::default()
                .with_digest(&digest)
                .with_read_mask(mask)
        };

        // Read the height before the lookup. `GetTransaction` only finds
        // transactions in checkpoints up to the node's latest checkpoint,
        // which is the height `GetServiceInfo` reports and which only rises.
        // So when the lookup finds nothing, the transaction is in no
        // checkpoint at or below `start_checkpoint`, and a stream starting
        // there sees it whenever it lands.
        let start_checkpoint = self
            .ledger_client()
            .get_service_info(GetServiceInfoRequest::default())
            .await
            .map_err(ExecuteAndWaitError::RpcError)?
            .into_inner()
            .checkpoint_height
            .ok_or_else(|| {
                ExecuteAndWaitError::RpcError(tonic::Status::data_loss(
                    "GetServiceInfo response is missing checkpoint_height",
                ))
            })?;

        // A transaction submitted before (a retry, or the same transaction
        // sent by someone else) can already be in a checkpoint the stream
        // starts after.
        match self
            .ledger_client()
            .get_transaction(lookup_request.clone())
            .await
        {
            Ok(lookup) if lookup.get_ref().transaction().checkpoint_opt().is_some() => {
                return Ok(lookup_into_execute_response(lookup));
            }
            Ok(_) => {}
            Err(status) if status.code() == tonic::Code::NotFound => {}
            Err(status) => return Err(ExecuteAndWaitError::RpcError(status)),
        }

        // The stream and the execution future are boxed: their combined state
        // would otherwise be inlined into this method's future, making it
        // large enough to threaten a stack overflow in callers that hold it
        // in deeply nested or spawned futures.
        let mut transactions = Box::pin(
            self.stream_transactions(
                TransactionStreamRequest::new()
                    .with_read_mask(FieldMask::from_str(
                        "digest,checkpoint,transaction_index,timestamp",
                    ))
                    .with_filter(TransactionFilter::matching(transaction::sender(
                        transaction.sender,
                    )))
                    .with_start(TransactionStreamStart::Checkpoint(start_checkpoint)),
            ),
        );
        let mut find_checkpoint = Box::pin(async {
            while let Some(frame) = transactions.try_next().await? {
                if let Some(executed) = frame.transaction
                    && executed.digest() == digest
                {
                    return Ok((executed.checkpoint(), executed.timestamp));
                }
            }
            Err(tonic::Status::aborted(
                "transaction stream ended unexpectedly",
            ))
        });

        // Follow the stream while executing: the transaction can land in a
        // checkpoint before the execution RPC returns, so remember the
        // outcome; the guard keeps a completed future from being polled
        // again.
        let mut execution_client = self.execution_client();
        let mut execution = Box::pin(execution_client.execute_transaction(request));
        let mut found = None;
        let execution_result = loop {
            tokio::select! {
                result = &mut execution => break result,
                result = &mut find_checkpoint, if found.is_none() => found = Some(result),
            }
        };

        let mut response = match execution_result {
            Ok(response) => response,
            Err(error) => {
                // Execution can fail for a transaction that nonetheless
                // committed, for example when a resubmission races the
                // original submission. Consult the ledger before surfacing
                // the error.
                if let Ok(lookup) = self.ledger_client().get_transaction(lookup_request).await
                    && lookup.get_ref().transaction().checkpoint_opt().is_some()
                {
                    return Ok(lookup_into_execute_response(lookup));
                }
                return Err(ExecuteAndWaitError::RpcError(error));
            }
        };

        let found = match found {
            Some(result) => result,
            None => tokio::select! {
                result = &mut find_checkpoint => result,
                _ = tokio::time::sleep(timeout) => {
                    return Err(ExecuteAndWaitError::CheckpointTimeout(response));
                }
            },
        };
        let (checkpoint, timestamp) = match found {
            Ok(found) => found,
            Err(error) => {
                return Err(ExecuteAndWaitError::CheckpointStreamError { response, error });
            }
        };

        response
            .get_mut()
            .transaction_mut()
            .set_checkpoint(checkpoint);
        response.get_mut().transaction_mut().timestamp = timestamp;
        Ok(response)
    }

    /// Retrieves the current reference gas price from the latest epoch information.
    ///
    /// # Returns
    /// The reference gas price as a `u64`
    ///
    /// # Errors
    /// Returns an error if there is an RPC error when fetching the epoch information
    pub async fn get_reference_gas_price(&mut self) -> Result<u64, tonic::Status> {
        let request = GetEpochRequest::latest()
            .with_read_mask(FieldMask::from_paths(["reference_gas_price"]));
        let response = self.ledger_client().get_epoch(request).await?.into_inner();
        Ok(response.epoch().reference_gas_price())
    }
}

/// Builds the response for a transaction answered from the ledger instead of
/// from execution.
fn lookup_into_execute_response(
    response: Response<GetTransactionResponse>,
) -> Response<ExecuteTransactionResponse> {
    Response::new(ExecuteTransactionResponse {
        transaction: response.into_inner().transaction,
    })
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = self.description.as_deref().unwrap_or("No description");
        write!(
            f,
            "ExecutionError: Kind: {}, Description: {}",
            self.kind().as_str_name(),
            description
        )
    }
}
