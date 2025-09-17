use futures::TryStreamExt;
use std::time::Duration;
use tonic::Response;

use crate::client::v2::Client;
use crate::field::FieldMaskUtil;
use crate::proto::sui::rpc::v2::ExecuteTransactionRequest;
use crate::proto::sui::rpc::v2::ExecuteTransactionResponse;
use crate::proto::sui::rpc::v2::SubscribeCheckpointsRequest;
use crate::proto::TryFromProtoError;
use prost_types::FieldMask;

/// Error types that can occur when executing a transaction and waiting for checkpoint
#[derive(Debug)]
pub enum ExecuteAndWaitError {
    /// RPC Error (actual tonic::Status from the client/server)
    RpcError(tonic::Status),
    /// Request is missing the required transaction field
    MissingTransaction,
    /// Failed to parse/convert the transaction for digest calculation
    ProtoConversionError(TryFromProtoError),
    /// Transaction was sent but failed during execution (includes response with error details)
    ExecutionFailed(Response<ExecuteTransactionResponse>),
    /// Transaction executed but checkpoint wait timed out
    CheckpointTimeout(Response<ExecuteTransactionResponse>),
    /// Transaction executed but checkpoint stream had an error
    CheckpointStreamError {
        response: Response<ExecuteTransactionResponse>,
        error: tonic::Status,
    },
}

impl Client {
    /// Executes a transaction and waits for it to be included in a checkpoint.
    ///
    /// This method provides "read your writes" consistency by executing the transaction
    /// and waiting for it to appear in a checkpoint, which gauruntees indexes have been updated on
    /// this node.
    ///
    /// # Arguments
    /// * `request` - The transaction execution request (ExecuteTransactionRequest)
    /// * `timeout` - Maximum time to wait for indexing confirmation
    ///
    /// # Returns
    /// A `Result` containing the response if the transaction was executed and checkpoint confirmed,
    /// or an error that may still include the response if execution succeeded but checkpoint
    /// confirmation failed.
    pub async fn execute_transaction_and_wait_for_checkpoint(
        &mut self,
        request: impl tonic::IntoRequest<ExecuteTransactionRequest>,
        timeout: Duration,
    ) -> Result<Response<ExecuteTransactionResponse>, ExecuteAndWaitError> {
        // Subscribe to checkpoint stream before execution to avoid missing the transaction.
        // Uses minimal read mask for efficiency since we only nee digest confirmation.
        // Once server-side filtering is available, we should filter by transaction digest to
        // further reduce bandwidth.
        let mut checkpoint_stream = match self
            .subscription_client()
            .subscribe_checkpoints(
                SubscribeCheckpointsRequest::default()
                    .with_read_mask(FieldMask::from_str("transactions.digest,sequence_number")),
            )
            .await
        {
            Ok(stream) => stream.into_inner(),
            Err(e) => return Err(ExecuteAndWaitError::RpcError(e)),
        };

        // Calculate digest from the input transaction to avoid relying on response read mask
        let request = request.into_request();
        let transaction = match request.get_ref().transaction_opt() {
            Some(tx) => tx,
            None => return Err(ExecuteAndWaitError::MissingTransaction),
        };

        let executed_txn_digest = match sui_sdk_types::Transaction::try_from(transaction) {
            Ok(tx) => tx.digest().to_string(),
            Err(e) => return Err(ExecuteAndWaitError::ProtoConversionError(e)),
        };

        let response = match self.execution_client().execute_transaction(request).await {
            Ok(resp) => resp,
            Err(e) => return Err(ExecuteAndWaitError::RpcError(e)),
        };

        let status = response
            .get_ref()
            .transaction_opt()
            .and_then(|tx| tx.effects_opt())
            .and_then(|effects| effects.status_opt());

        match status.and_then(|s| s.success_opt()) {
            Some(true) => {} // Transaction successful, continue to wait for checkpoint
            _ => return Err(ExecuteAndWaitError::ExecutionFailed(response)),
        }

        // Wait for the transaction to appear in a checkpoint, at which point indexes will have been
        // updated.
        let timeout_future = tokio::time::sleep(timeout);
        let checkpoint_future = async {
            while let Some(response) = checkpoint_stream.try_next().await? {
                let checkpoint = response.checkpoint();

                for tx in checkpoint.transactions() {
                    let digest = tx.digest();

                    if digest == executed_txn_digest {
                        return Ok(());
                    }
                }
            }
            Err(tonic::Status::aborted(
                "checkpoint stream ended unexpectedly",
            ))
        };

        tokio::select! {
            result = checkpoint_future => {
                match result {
                    Ok(()) => Ok(response),
                    Err(e) => Err(ExecuteAndWaitError::CheckpointStreamError { response, error: e })
                }
            },
            _ = timeout_future => {
                Err(ExecuteAndWaitError::CheckpointTimeout ( response))
            }
        }
    }
}
