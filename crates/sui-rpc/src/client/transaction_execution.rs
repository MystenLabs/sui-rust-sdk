use super::Client;
use crate::field::FieldMaskUtil;
use crate::proto::TryFromProtoError;
use crate::proto::sui::rpc::v2::ExecuteTransactionRequest;
use crate::proto::sui::rpc::v2::ExecuteTransactionResponse;
use crate::proto::sui::rpc::v2::ExecutionError;
use crate::proto::sui::rpc::v2::GetEpochRequest;
use crate::proto::sui::rpc::v2::SubscribeCheckpointsRequest;
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
    /// Transaction executed but checkpoint stream had an error
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
