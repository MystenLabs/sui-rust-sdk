use super::Client;
use crate::field::FieldMaskUtil;
use crate::proto::TryFromProtoError;
use crate::proto::sui::rpc::v2::ExecuteTransactionRequest;
use crate::proto::sui::rpc::v2::ExecuteTransactionResponse;
use crate::proto::sui::rpc::v2::ExecutionError;
use crate::proto::sui::rpc::v2::GetEpochRequest;
use crate::proto::sui::rpc::v2::GetTransactionRequest;
use crate::proto::sui::rpc::v2::GetTransactionResponse;
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
    ///
    /// # Duplicate submissions
    /// Submitting a transaction that has already been executed is handled
    /// gracefully. While the execution RPC is in flight the ledger is probed
    /// for the transaction, and a transaction that is already in a checkpoint
    /// is returned without waiting for execution to finish. Likewise, if
    /// execution fails but the ledger shows the transaction in a checkpoint
    /// (for example when a resubmission races the original submission), the
    /// execution error is discarded and the committed transaction is
    /// returned. In both cases the response is assembled from
    /// `GetTransaction` using the request's read mask, so it carries the same
    /// fields an execution response would, with `digest`, `checkpoint`, and
    /// `timestamp` always populated.
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

        // Read mask for answering from GetTransaction when execution cannot
        // provide the response: a duplicate submission that already
        // committed, or an execution error after the transaction landed.
        // Both RPCs' masks select fields of `ExecutedTransaction`, so the
        // caller's mask passes through unchanged; when the caller didn't set
        // one, mirror ExecuteTransaction's documented default. `digest`,
        // `checkpoint`, and `timestamp` are always included since this
        // method's contract populates them.
        let lookup_mask = {
            let caller_paths = match &request.get_ref().read_mask {
                Some(mask) => mask.paths.clone(),
                None => vec!["effects.status".to_owned()],
            };
            FieldMask::from_paths(caller_paths.iter().map(String::as_str).chain([
                "digest",
                "checkpoint",
                "timestamp",
            ]))
            .normalize()
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
        //
        // Both this future and the execution future below are boxed: their
        // combined state (two full tonic call chains alive at once) would
        // otherwise be inlined into this method's future, making it large
        // enough to threaten a stack overflow in callers that hold it in
        // deeply nested or spawned futures.
        let mut scan = Box::pin(async {
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
        });

        // The concurrent futures below each need a service client, and a
        // single `&mut self` cannot back all of them at once, so give each
        // its own client over the shared channel.
        let mut execution_client = self.execution_client();
        let mut post_exec_lookup_client = self.ledger_client();
        let mut probe_client = self.ledger_client();

        // Execute, then query the fullnode directly to see if it already has
        // the txn in a checkpoint. This is to handle the case where an
        // already executed transaction is sent multiple times.
        let mut exec_and_check = Box::pin(async {
            let response = execution_client.execute_transaction(request).await?;

            let already_checkpointed = match post_exec_lookup_client
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
        });

        // Probe the ledger while execution is in flight: a resubmission of a
        // transaction that already committed can be answered from the ledger
        // without waiting for, or succeeding at, execution.
        let mut probe = Box::pin(async {
            probe_client
                .get_transaction(
                    GetTransactionRequest::default()
                        .with_digest(&executed_txn_digest)
                        .with_read_mask(lookup_mask.clone()),
                )
                .await
        });

        // Drive execution, the scan, and the probe together. The scan can
        // complete first (for example, when a duplicate of an already
        // executed transaction lands in a checkpoint mid-execution), so
        // remember its outcome; the guards keep completed futures from being
        // polled again. A probe that finds the transaction in a checkpoint
        // resolves the call on the spot; any other probe outcome (not found,
        // not yet checkpointed, or an RPC error) means execution has to
        // provide the answer.
        let mut scan_result = None;
        let mut probe_done = false;
        let exec_result = loop {
            tokio::select! {
                exec = &mut exec_and_check => break exec,
                result = &mut scan, if scan_result.is_none() => {
                    scan_result = Some(result);
                }
                result = &mut probe, if !probe_done => {
                    probe_done = true;
                    if let Ok(lookup) = result
                        && lookup.get_ref().transaction().checkpoint_opt().is_some()
                    {
                        return Ok(lookup_into_execute_response(lookup));
                    }
                }
            }
        };

        let (mut response, already_checkpointed) = match exec_result {
            Ok(ok) => ok,
            Err(error) => {
                // Execution can fail for a transaction that nonetheless
                // committed, for example when a resubmission races the
                // original submission. Consult the ledger before surfacing
                // the error.
                drop(probe);
                if let Ok(lookup) = probe_client
                    .get_transaction(
                        GetTransactionRequest::default()
                            .with_digest(&executed_txn_digest)
                            .with_read_mask(lookup_mask),
                    )
                    .await
                    && lookup.get_ref().transaction().checkpoint_opt().is_some()
                {
                    return Ok(lookup_into_execute_response(lookup));
                }
                return Err(ExecuteAndWaitError::RpcError(error));
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
