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
    pub async fn execute_transaction_and_wait_for_checkpoint(
        &mut self,
        request: impl tonic::IntoRequest<ExecuteTransactionRequest>,
        timeout: Duration,
    ) -> Result<Response<ExecuteTransactionResponse>, ExecuteAndWaitError> {
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

        // Subscribe to checkpoint stream before execution to avoid missing the transaction.
        // Uses minimal read mask for efficiency since we only nee digest confirmation.
        // Once server-side filtering is available, we should filter by transaction digest to
        // further reduce bandwidth.
        let mut checkpoint_stream = match self
            .subscription_client()
            .subscribe_checkpoints(SubscribeCheckpointsRequest::default().with_read_mask(
                FieldMask::from_str("transactions.digest,sequence_number,summary.timestamp"),
            ))
            .await
        {
            Ok(stream) => stream.into_inner(),
            Err(e) => return Err(ExecuteAndWaitError::RpcError(e)),
        };

        // Scan the subscription for the transaction's digest. Every RPC on
        // this client shares one HTTP/2 connection, so this future must be
        // polled concurrently with the execution phase below: a subscription
        // parked while another call is awaited pins its flow-control window
        // (checkpoints keep arriving whether or not anyone reads them) and,
        // past the idle timeout, gets reset by the client's body watchdog.
        let scan = async {
            while let Some(response) = checkpoint_stream.try_next().await? {
                let checkpoint = response.checkpoint();

                for tx in checkpoint.transactions() {
                    if tx.digest() == executed_txn_digest {
                        return Ok((checkpoint.sequence_number(), checkpoint.summary().timestamp));
                    }
                }
            }
            Err(tonic::Status::aborted(
                "checkpoint stream ended unexpectedly",
            ))
        };
        tokio::pin!(scan);

        // Execute, then query the fullnode directly to see if it already has
        // the txn in a checkpoint. This is to handle the case where an
        // already executed transaction is sent multiple times.
        let exec_and_check = async {
            let response = self.execution_client().execute_transaction(request).await?;

            let already_checkpointed = match self
                .ledger_client()
                .get_transaction(
                    GetTransactionRequest::default()
                        .with_digest(&executed_txn_digest)
                        .with_read_mask(FieldMask::from_str("digest,checkpoint,timestamp")),
                )
                .await
            {
                Ok(resp) if resp.get_ref().transaction().checkpoint_opt().is_some() => Some((
                    resp.get_ref().transaction().checkpoint(),
                    resp.get_ref().transaction().timestamp,
                )),
                _ => None,
            };

            Ok::<_, tonic::Status>((response, already_checkpointed))
        };
        tokio::pin!(exec_and_check);

        // Drive execution and the scan together. The scan can complete first
        // (for example, when a duplicate of an already executed transaction
        // lands in a checkpoint mid-execution), so remember its outcome; the
        // guard keeps a completed scan from being polled again.
        let mut scan_result = None;
        let (mut response, already_checkpointed) = loop {
            tokio::select! {
                exec = &mut exec_and_check => {
                    match exec {
                        Ok(ok) => break ok,
                        Err(e) => return Err(ExecuteAndWaitError::RpcError(e)),
                    }
                }
                result = &mut scan, if scan_result.is_none() => {
                    scan_result = Some(result);
                }
            }
        };

        // Wait for the transaction to appear in a checkpoint, at which point
        // indexes will have been updated. The direct lookup takes precedence:
        // when it already places the transaction in a checkpoint there is
        // nothing to wait for, even if the scan failed in the meantime.
        let (checkpoint, timestamp) = if let Some(found) = already_checkpointed {
            found
        } else {
            let result = match scan_result {
                Some(result) => result,
                None => {
                    tokio::select! {
                        result = &mut scan => result,
                        _ = tokio::time::sleep(timeout) => {
                            return Err(ExecuteAndWaitError::CheckpointTimeout(response));
                        }
                    }
                }
            };
            match result {
                Ok(found) => found,
                Err(e) => {
                    return Err(ExecuteAndWaitError::CheckpointStreamError { response, error: e });
                }
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
