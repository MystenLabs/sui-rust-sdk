//! Regression tests for `execute_transaction_and_wait_for_checkpoint`.
//!
//! The method subscribes to the checkpoint stream before executing so it
//! cannot miss the transaction's checkpoint. Every RPC on a `Client` shares
//! one HTTP/2 connection, so the subscription must be polled *concurrently*
//! with the execution phase: a subscription parked while `ExecuteTransaction`
//! is awaited pins its flow-control window (checkpoints keep arriving whether
//! or not anyone reads them) and, past the idle timeout, gets reset by the
//! client's body watchdog — turning a successful execution into a spurious
//! `CheckpointStreamError`.

use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use prost_types::FieldMask;
use proto::ledger_service_server::LedgerService;
use proto::ledger_service_server::LedgerServiceServer;
use proto::subscription_service_server::SubscriptionService;
use proto::subscription_service_server::SubscriptionServiceServer;
use proto::transaction_execution_service_server::TransactionExecutionService;
use proto::transaction_execution_service_server::TransactionExecutionServiceServer;
use sui_rpc::Client;
use sui_rpc::proto::sui::rpc::v2 as proto;

/// Cadence of the mock checkpoint subscription.
const CHECKPOINT_TICK: Duration = Duration::from_millis(100);

#[derive(Clone)]
struct MockServer {
    /// Digest of the transaction under test; emitted in the checkpoint at
    /// `digest_at_seq`, with filler digests before it.
    digest: String,
    digest_at_seq: u64,
    /// How long `ExecuteTransaction` takes.
    exec_delay: Duration,
    /// When set, `ExecuteTransaction` fails with this status after
    /// `exec_delay`.
    exec_error: Option<tonic::Status>,
    /// When set, `GetTransaction` reports the transaction as already
    /// included in this checkpoint (the duplicate-submission shortcut).
    shortcut_checkpoint: Option<u64>,
    /// The 1-based `GetTransaction` call from which `shortcut_checkpoint`
    /// applies; earlier calls report the transaction as not yet
    /// checkpointed.
    shortcut_from_lookup: usize,
    /// Every `GetTransaction` request received, in order.
    lookups: Arc<Mutex<Vec<proto::GetTransactionRequest>>>,
}

impl MockServer {
    fn new(digest: String) -> Self {
        Self {
            digest,
            // Far enough out that the subscription scan never resolves
            // within a test's wait timeout.
            digest_at_seq: 10_000,
            exec_delay: Duration::ZERO,
            exec_error: None,
            shortcut_checkpoint: None,
            shortcut_from_lookup: 1,
            lookups: Arc::default(),
        }
    }
}

#[tonic::async_trait]
impl TransactionExecutionService for MockServer {
    async fn execute_transaction(
        &self,
        _request: tonic::Request<proto::ExecuteTransactionRequest>,
    ) -> Result<tonic::Response<proto::ExecuteTransactionResponse>, tonic::Status> {
        tokio::time::sleep(self.exec_delay).await;
        if let Some(status) = &self.exec_error {
            return Err(status.clone());
        }
        Ok(tonic::Response::new(
            proto::ExecuteTransactionResponse::default(),
        ))
    }
}

#[tonic::async_trait]
impl LedgerService for MockServer {
    async fn get_transaction(
        &self,
        request: tonic::Request<proto::GetTransactionRequest>,
    ) -> Result<tonic::Response<proto::GetTransactionResponse>, tonic::Status> {
        let lookup_count = {
            let mut lookups = self.lookups.lock().unwrap();
            lookups.push(request.into_inner());
            lookups.len()
        };
        let mut response = proto::GetTransactionResponse::default();
        if let Some(checkpoint) = self.shortcut_checkpoint
            && lookup_count >= self.shortcut_from_lookup
        {
            let mut transaction = proto::ExecutedTransaction::default();
            transaction.digest = Some(self.digest.clone());
            transaction.checkpoint = Some(checkpoint);
            response.transaction = Some(transaction);
        }
        Ok(tonic::Response::new(response))
    }
}

#[tonic::async_trait]
impl SubscriptionService for MockServer {
    async fn subscribe_checkpoints(
        &self,
        _request: tonic::Request<proto::SubscribeCheckpointsRequest>,
    ) -> Result<
        tonic::Response<tonic::codegen::BoxStream<proto::SubscribeCheckpointsResponse>>,
        tonic::Status,
    > {
        let digest = self.digest.clone();
        let digest_at_seq = self.digest_at_seq;
        let stream = futures::stream::unfold(0u64, move |seq| {
            let digest = digest.clone();
            async move {
                tokio::time::sleep(CHECKPOINT_TICK).await;
                let mut transaction = proto::ExecutedTransaction::default();
                transaction.digest = Some(if seq == digest_at_seq {
                    digest
                } else {
                    "filler".to_owned()
                });
                let mut checkpoint = proto::Checkpoint::default();
                checkpoint.sequence_number = Some(seq);
                checkpoint.transactions = vec![transaction];
                let mut resp = proto::SubscribeCheckpointsResponse::default();
                resp.cursor = Some(seq);
                resp.checkpoint = Some(checkpoint);
                Some((Ok(resp), seq + 1))
            }
        });
        Ok(tonic::Response::new(Box::pin(stream)))
    }
}

async fn spawn_mock_server(mock: MockServer) -> std::net::SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock server listener");
    let addr = listener.local_addr().expect("mock server local addr");
    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(TransactionExecutionServiceServer::new(mock.clone()))
            .add_service(LedgerServiceServer::new(mock.clone()))
            .add_service(SubscriptionServiceServer::new(mock))
            .serve_with_incoming(tonic::transport::server::TcpIncoming::from(listener))
            .await
            .expect("mock server exited with an error");
    });
    addr
}

/// A minimal valid transaction, plus the digest the client will derive from
/// it.
fn test_transaction() -> (proto::Transaction, String) {
    let transaction = sui_sdk_types::Transaction {
        kind: sui_sdk_types::TransactionKind::ProgrammableTransaction(
            sui_sdk_types::ProgrammableTransaction {
                inputs: Vec::new(),
                commands: Vec::new(),
            },
        ),
        sender: sui_sdk_types::Address::ZERO,
        gas_payment: sui_sdk_types::GasPayment {
            objects: Vec::new(),
            owner: sui_sdk_types::Address::ZERO,
            price: 1,
            budget: 1,
        },
        expiration: sui_sdk_types::TransactionExpiration::None,
    };
    let digest = transaction.digest().to_string();
    (proto::Transaction::from(transaction), digest)
}

/// The subscription must survive a slow execution: with an idle timeout well
/// below the execution latency, the stream is only alive at scan time if it
/// was polled throughout. Before the concurrent-scan fix, the watchdog reset
/// the parked subscription during execution and this returned
/// `CheckpointStreamError` despite a successful execution.
#[tokio::test(flavor = "multi_thread")]
async fn subscription_survives_slow_execution() {
    let (transaction, digest) = test_transaction();
    let digest_at_seq = 50;
    let addr = spawn_mock_server(MockServer {
        digest_at_seq,
        exec_delay: Duration::from_secs(4),
        ..MockServer::new(digest)
    })
    .await;

    let mut client = Client::new(format!("http://{addr}"))
        .expect("client")
        .with_body_idle_timeout(Duration::from_millis(1500));

    let request = proto::ExecuteTransactionRequest::default().with_transaction(transaction);
    let response = client
        .execute_transaction_and_wait_for_checkpoint(request, Duration::from_secs(30))
        .await
        .expect("execution and checkpoint confirmation");
    assert_eq!(response.get_ref().transaction().checkpoint(), digest_at_seq);
}

/// The duplicate-submission shortcut still short-circuits the checkpoint
/// wait: when `GetTransaction` already places the transaction in a
/// checkpoint, that checkpoint is returned without waiting for the
/// subscription scan (whose digest checkpoint here would only arrive far
/// past the wait timeout).
#[tokio::test(flavor = "multi_thread")]
async fn already_checkpointed_transaction_short_circuits() {
    let (transaction, digest) = test_transaction();
    let addr = spawn_mock_server(MockServer {
        shortcut_checkpoint: Some(7),
        ..MockServer::new(digest)
    })
    .await;

    let mut client = Client::new(format!("http://{addr}")).expect("client");

    let request = proto::ExecuteTransactionRequest::default().with_transaction(transaction);
    let response = client
        .execute_transaction_and_wait_for_checkpoint(request, Duration::from_secs(5))
        .await
        .expect("execution with an already checkpointed transaction");
    assert_eq!(response.get_ref().transaction().checkpoint(), 7);
}

/// A resubmission of a transaction that is already in a checkpoint is
/// answered by the ledger probe that races execution: the call returns long
/// before the execution RPC completes, and the lookup carries the caller's
/// read mask plus the fields this method always populates.
#[tokio::test(flavor = "multi_thread")]
async fn committed_duplicate_returns_before_execution_finishes() {
    let (transaction, digest) = test_transaction();
    let lookups = Arc::new(Mutex::new(Vec::new()));
    let addr = spawn_mock_server(MockServer {
        exec_delay: Duration::from_secs(60),
        shortcut_checkpoint: Some(7),
        lookups: lookups.clone(),
        ..MockServer::new(digest.clone())
    })
    .await;

    let mut client = Client::new(format!("http://{addr}")).expect("client");

    let request = proto::ExecuteTransactionRequest::default()
        .with_transaction(transaction)
        .with_read_mask(FieldMask {
            paths: vec!["effects.status".to_owned()],
        });
    let start = std::time::Instant::now();
    let response = client
        .execute_transaction_and_wait_for_checkpoint(request, Duration::from_secs(5))
        .await
        .expect("committed duplicate resolved from the ledger probe");
    assert!(
        start.elapsed() < Duration::from_secs(30),
        "the probe must resolve the call without waiting for execution"
    );
    assert_eq!(response.get_ref().transaction().checkpoint(), 7);
    assert_eq!(response.get_ref().transaction().digest(), digest);

    let probe_mask = lookups.lock().unwrap()[0]
        .read_mask
        .clone()
        .expect("probe read mask");
    for path in ["effects.status", "digest", "checkpoint", "timestamp"] {
        assert!(
            probe_mask.paths.iter().any(|p| p == path),
            "probe read mask is missing `{path}`: {probe_mask:?}"
        );
    }
}

/// An execution error for a transaction that nonetheless committed (for
/// example a resubmission racing the original submission) is downgraded to
/// success by a final ledger lookup. The probe still sees the transaction as
/// not checkpointed here, so only the post-error lookup can resolve the call.
#[tokio::test(flavor = "multi_thread")]
async fn execution_error_with_committed_transaction_succeeds() {
    let (transaction, digest) = test_transaction();
    let addr = spawn_mock_server(MockServer {
        exec_delay: Duration::from_millis(500),
        exec_error: Some(tonic::Status::aborted("transaction already being executed")),
        shortcut_checkpoint: Some(9),
        shortcut_from_lookup: 2,
        ..MockServer::new(digest.clone())
    })
    .await;

    let mut client = Client::new(format!("http://{addr}")).expect("client");

    let request = proto::ExecuteTransactionRequest::default().with_transaction(transaction);
    let response = client
        .execute_transaction_and_wait_for_checkpoint(request, Duration::from_secs(5))
        .await
        .expect("execution error downgraded for a committed transaction");
    assert_eq!(response.get_ref().transaction().checkpoint(), 9);
    assert_eq!(response.get_ref().transaction().digest(), digest);
}

/// An execution error for a transaction the ledger has no checkpoint for
/// still surfaces as `RpcError`.
#[tokio::test(flavor = "multi_thread")]
async fn execution_error_without_commit_surfaces() {
    let (transaction, digest) = test_transaction();
    let addr = spawn_mock_server(MockServer {
        exec_error: Some(tonic::Status::internal("boom")),
        ..MockServer::new(digest)
    })
    .await;

    let mut client = Client::new(format!("http://{addr}")).expect("client");

    let request = proto::ExecuteTransactionRequest::default().with_transaction(transaction);
    let err = client
        .execute_transaction_and_wait_for_checkpoint(request, Duration::from_secs(5))
        .await
        .expect_err("execution error with no committed transaction");
    match err {
        sui_rpc::client::ExecuteAndWaitError::RpcError(status) => {
            assert_eq!(status.code(), tonic::Code::Internal);
        }
        other => panic!("expected RpcError, got {other:?}"),
    }
}

/// A request without a transaction fails fast, before any network work.
#[tokio::test(flavor = "multi_thread")]
async fn missing_transaction_fails_without_rpc() {
    // No server at this address: the validation error must surface before
    // any RPC is attempted.
    let mut client = Client::new("http://127.0.0.1:1").expect("client");

    let err = client
        .execute_transaction_and_wait_for_checkpoint(
            proto::ExecuteTransactionRequest::default(),
            Duration::from_secs(1),
        )
        .await
        .expect_err("request without a transaction");
    assert!(matches!(
        err,
        sui_rpc::client::ExecuteAndWaitError::MissingTransaction
    ));
}
