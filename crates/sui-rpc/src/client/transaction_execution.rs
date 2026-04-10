use super::Client;
use crate::field::FieldMaskUtil;
use crate::proto::TryFromProtoError;
use crate::proto::sui::rpc::v2::ExecuteTransactionRequest;
use crate::proto::sui::rpc::v2::ExecuteTransactionResponse;
use crate::proto::sui::rpc::v2::ExecutionError;
use crate::proto::sui::rpc::v2::GetEpochRequest;
use crate::proto::sui::rpc::v2::GetTransactionRequest;
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
    // NOTE: body aggressively Box::pins/Boxes locals and inner futures to shrink the async
    // state machine for callers running on small-stack threads (for example, macOS default 2MB).
    pub async fn execute_transaction_and_wait_for_checkpoint(
        &mut self,
        request: impl tonic::IntoRequest<ExecuteTransactionRequest>,
        timeout: Duration,
    ) -> Result<Response<ExecuteTransactionResponse>, ExecuteAndWaitError> {
        // Hoist owned sub-clients up front so later awaits don't re-borrow &mut self, and so
        // that `self` is not part of the state machine across the big awaits.
        let mut subscription_client = self.subscription_client();
        let mut execution_client = self.execution_client();
        let mut ledger_client = self.ledger_client();

        // Subscribe to checkpoint stream before execution to avoid missing the transaction.
        // Uses minimal read mask for efficiency since we only need digest confirmation.
        // Once server-side filtering is available, we should filter by transaction digest to
        // further reduce bandwidth.
        let subscribe_fut = Box::pin(subscription_client.subscribe_checkpoints(
            SubscribeCheckpointsRequest::default().with_read_mask(FieldMask::from_str(
                "transactions.digest,sequence_number,summary.timestamp",
            )),
        ));
        let checkpoint_stream: Box<tonic::Streaming<_>> = match subscribe_fut.await {
            Ok(stream) => Box::new(stream.into_inner()),
            Err(e) => return Err(ExecuteAndWaitError::RpcError(e)),
        };

        // Calculate digest from the input transaction to avoid relying on response read mask.
        // Scope the unboxed request tightly so it's dropped before the next await.
        let (request, executed_txn_digest) = {
            let request = request.into_request();
            let transaction = match request.get_ref().transaction_opt() {
                Some(tx) => tx,
                None => return Err(ExecuteAndWaitError::MissingTransaction),
            };
            let digest = match sui_sdk_types::Transaction::try_from(transaction) {
                Ok(tx) => tx.digest().to_string(),
                Err(e) => return Err(ExecuteAndWaitError::ProtoConversionError(e)),
            };
            (request, digest)
        };

        let execute_fut = Box::pin(execution_client.execute_transaction(request));
        // Box the Response immediately so the large inlined proto body lives on the heap, not
        // in subsequent await-point state.
        let mut response: Box<Response<ExecuteTransactionResponse>> = match execute_fut.await {
            Ok(resp) => Box::new(resp),
            Err(e) => return Err(ExecuteAndWaitError::RpcError(e)),
        };

        // First query the fullnode directly to see if it already has the txn. This is to handle
        // the case where an already executed transaction is sent multiple times.
        let get_tx_fut = Box::pin(ledger_client.get_transaction(
            GetTransactionRequest::default()
                .with_digest(&executed_txn_digest)
                .with_read_mask(FieldMask::from_str("digest,checkpoint,timestamp")),
        ));
        if let Ok(resp) = get_tx_fut.await
            && resp.get_ref().transaction().checkpoint_opt().is_some()
        {
            let checkpoint = resp.get_ref().transaction().checkpoint();
            let timestamp = resp.get_ref().transaction().timestamp;
            drop(resp);
            response
                .get_mut()
                .transaction_mut()
                .set_checkpoint(checkpoint);
            response.get_mut().transaction_mut().timestamp = timestamp;
            return Ok(*response);
        }

        // Wait for the transaction to appear in a checkpoint, at which point indexes will have
        // been updated. Box::pin both arms so the outer select! state only holds pointers.
        let mut timeout_future = Box::pin(tokio::time::sleep(timeout));
        let mut checkpoint_future = Box::pin(async move {
            let mut checkpoint_stream = checkpoint_stream;
            while let Some(msg) = checkpoint_stream.try_next().await? {
                let checkpoint = msg.checkpoint();
                for tx in checkpoint.transactions() {
                    if tx.digest() == executed_txn_digest {
                        return Ok((checkpoint.sequence_number(), checkpoint.summary().timestamp));
                    }
                }
            }
            Err(tonic::Status::aborted(
                "checkpoint stream ended unexpectedly",
            ))
        });

        tokio::select! {
            result = &mut checkpoint_future => {
                match result {
                    Ok((checkpoint, timestamp)) => {
                        response
                            .get_mut()
                            .transaction_mut()
                            .set_checkpoint(checkpoint);
                        response.get_mut().transaction_mut().timestamp = timestamp;
                        Ok(*response)
                    }
                    Err(e) => Err(ExecuteAndWaitError::CheckpointStreamError {
                        response: *response,
                        error: e,
                    }),
                }
            },
            _ = &mut timeout_future => {
                Err(ExecuteAndWaitError::CheckpointTimeout(*response))
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
