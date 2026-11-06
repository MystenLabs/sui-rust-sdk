//! Regression tests for `execute_transaction_and_wait_for_checkpoint`.
//!
//! The mock is a small simulated ledger: once the transaction is submitted,
//! its tip advances one checkpoint per `TICK`, and the transaction under test
//! becomes visible once the tip reaches the checkpoint it was committed in.
//! The method reads the height, checks the ledger for a duplicate, then
//! executes while following the sender's transactions from that height with
//! the resumable transaction stream, so the mock serves `GetServiceInfo`,
//! `GetTransaction`, `ListTransactions`, `SubscribeTransactions`, and
//! `ExecuteTransaction`.

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;

use futures::StreamExt;
use prost::bytes::Bytes;
use prost_types::FieldMask;
use proto::ledger_service_server::LedgerService;
use proto::ledger_service_server::LedgerServiceServer;
use proto::subscription_service_server::SubscriptionService;
use proto::subscription_service_server::SubscriptionServiceServer;
use proto::transaction_execution_service_server::TransactionExecutionService;
use proto::transaction_execution_service_server::TransactionExecutionServiceServer;
use sui_rpc::Client;
use sui_rpc::client::ExecuteAndWaitError;
use sui_rpc::proto::sui::rpc::v2 as proto;
use tonic::codegen::BoxStream;

/// How often the mock ledger's tip advances by one checkpoint.
const TICK: Duration = Duration::from_millis(50);
/// The mock ledger's tip when it starts.
const START_TIP: u64 = 5;

#[derive(Clone)]
struct MockLedger {
    /// When the tip starts advancing: the first `ExecuteTransaction`. Until
    /// then the tip stays at `START_TIP`, so every read before execution
    /// sees the same ledger however slowly the client connects.
    execution_started: Arc<OnceLock<Instant>>,
    digest: String,
    /// Checkpoint the transaction under test is committed in. It becomes
    /// visible once the tip reaches it; `None` never commits it.
    committed_at: Option<u64>,
    /// How long `ExecuteTransaction` takes.
    exec_delay: Duration,
    /// When set, `ExecuteTransaction` fails with this status after
    /// `exec_delay`.
    exec_error: Option<tonic::Status>,
    /// How long `SubscribeTransactions` waits before fixing its start
    /// position at the then-current tip.
    subscribe_delay: Duration,
    executions: Arc<AtomicUsize>,
    lookups: Arc<Mutex<Vec<proto::GetTransactionRequest>>>,
}

impl MockLedger {
    fn new(digest: String) -> Self {
        Self {
            execution_started: Arc::default(),
            digest,
            committed_at: None,
            exec_delay: Duration::ZERO,
            exec_error: None,
            subscribe_delay: Duration::ZERO,
            executions: Arc::default(),
            lookups: Arc::default(),
        }
    }

    fn tip(&self) -> u64 {
        let Some(started) = self.execution_started.get() else {
            return START_TIP;
        };
        let ticks = started.elapsed().as_millis() / TICK.as_millis();
        START_TIP + u64::try_from(ticks).unwrap()
    }

    fn executed_transaction(&self, checkpoint: u64) -> proto::ExecutedTransaction {
        let mut transaction = proto::ExecutedTransaction::default();
        transaction.digest = Some(self.digest.clone());
        transaction.checkpoint = Some(checkpoint);
        transaction.transaction_index = Some(0);
        transaction.timestamp = Some(checkpoint_timestamp(checkpoint));
        transaction
    }
}

fn checkpoint_timestamp(checkpoint: u64) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: i64::try_from(checkpoint).unwrap(),
        nanos: 0,
    }
}

/// A boundary in the ledger: the point just before the transaction at
/// `(checkpoint, transaction_index)`. The mock commits at most one matching
/// transaction per checkpoint, at index 0, so `(c, 1)` lies just past it and
/// `(c + 1, 0)` is the start of the next checkpoint.
type Point = (u64, u64);

/// The watermark at `point`: its cursor names the point, `cursor_position`
/// carries it in ledger coordinates, and coverage is every checkpoint whose
/// matching transaction lies before it.
fn watermark((checkpoint, transaction_index): Point) -> proto::Watermark {
    let mut watermark = proto::Watermark::default();
    watermark.cursor = Some(Bytes::from(format!("{checkpoint}:{transaction_index}")));
    watermark.checkpoint = if transaction_index > 0 {
        Some(checkpoint)
    } else {
        checkpoint.checked_sub(1)
    };
    watermark.cursor_position = Some(
        proto::LedgerPosition::default()
            .with_checkpoint(checkpoint)
            .with_transaction_index(transaction_index),
    );
    watermark
}

/// The point a cursor minted by [`watermark`] names.
fn cursor_point(cursor: &Bytes) -> Point {
    let (checkpoint, transaction_index) = std::str::from_utf8(cursor)
        .unwrap()
        .split_once(':')
        .unwrap();
    (
        checkpoint.parse().unwrap(),
        transaction_index.parse().unwrap(),
    )
}

#[tonic::async_trait]
impl TransactionExecutionService for MockLedger {
    async fn execute_transaction(
        &self,
        _request: tonic::Request<proto::ExecuteTransactionRequest>,
    ) -> Result<tonic::Response<proto::ExecuteTransactionResponse>, tonic::Status> {
        self.executions.fetch_add(1, Ordering::SeqCst);
        self.execution_started.get_or_init(Instant::now);
        tokio::time::sleep(self.exec_delay).await;
        if let Some(status) = &self.exec_error {
            return Err(status.clone());
        }
        let mut transaction = proto::ExecutedTransaction::default();
        transaction.digest = Some(self.digest.clone());
        let mut response = proto::ExecuteTransactionResponse::default();
        response.transaction = Some(transaction);
        Ok(tonic::Response::new(response))
    }
}

#[tonic::async_trait]
impl LedgerService for MockLedger {
    async fn get_service_info(
        &self,
        _request: tonic::Request<proto::GetServiceInfoRequest>,
    ) -> Result<tonic::Response<proto::GetServiceInfoResponse>, tonic::Status> {
        let mut response = proto::GetServiceInfoResponse::default();
        response.checkpoint_height = Some(self.tip());
        Ok(tonic::Response::new(response))
    }

    async fn get_transaction(
        &self,
        request: tonic::Request<proto::GetTransactionRequest>,
    ) -> Result<tonic::Response<proto::GetTransactionResponse>, tonic::Status> {
        self.lookups.lock().unwrap().push(request.into_inner());
        match self.committed_at {
            Some(checkpoint) if checkpoint <= self.tip() => {
                let mut response = proto::GetTransactionResponse::default();
                response.transaction = Some(self.executed_transaction(checkpoint));
                Ok(tonic::Response::new(response))
            }
            _ => Err(tonic::Status::not_found("transaction not found")),
        }
    }

    async fn list_transactions(
        &self,
        request: tonic::Request<proto::ListTransactionsRequest>,
    ) -> Result<tonic::Response<BoxStream<proto::ListTransactionsResponse>>, tonic::Status> {
        let request = request.into_inner();
        let tip = self.tip();
        let options = request.options.unwrap_or_default();
        // Bounds intersect: the scan starts at the latest start and ends at
        // the earliest end.
        let mut start = (
            request.start_checkpoint.unwrap_or(0),
            request.start_transaction_index.unwrap_or(0),
        );
        if let Some(cursor) = &options.after {
            start = start.max(cursor_point(cursor));
        }
        // Reaching the tip first ends the scan with `LedgerTip`.
        let mut end = (tip + 1, 0);
        let mut reason = proto::QueryEndReason::LedgerTip;
        if let Some(end_checkpoint) = request.end_checkpoint {
            let bound = match request.end_transaction_index {
                Some(index) => (end_checkpoint - 1, index),
                None => (end_checkpoint, 0),
            };
            if bound <= end {
                end = bound;
                reason = proto::QueryEndReason::CheckpointBound;
            }
        }
        if let Some(cursor) = &options.before
            && cursor_point(cursor) <= end
        {
            end = cursor_point(cursor);
            reason = proto::QueryEndReason::CursorBound;
        }

        let mut frames = Vec::new();
        if let Some(checkpoint) = self.committed_at
            && (start..end).contains(&(checkpoint, 0))
        {
            let mut frame = proto::ListTransactionsResponse::default();
            frame.transaction = Some(self.executed_transaction(checkpoint));
            frame.watermark = Some(watermark((checkpoint, 1)));
            frames.push(Ok(frame));
        }
        let mut query_end = proto::QueryEnd::default();
        query_end.reason = Some(reason as i32);
        // The resume point never moves before the request's start.
        let mut frame = proto::ListTransactionsResponse::default();
        frame.watermark = Some(watermark(end.max(start)));
        frame.end = Some(query_end);
        frames.push(Ok(frame));
        Ok(tonic::Response::new(Box::pin(futures::stream::iter(
            frames,
        ))))
    }
}

#[tonic::async_trait]
impl SubscriptionService for MockLedger {
    async fn subscribe_transactions(
        &self,
        _request: tonic::Request<proto::SubscribeTransactionsRequest>,
    ) -> Result<tonic::Response<BoxStream<proto::SubscribeTransactionsResponse>>, tonic::Status>
    {
        tokio::time::sleep(self.subscribe_delay).await;
        // The first frame fixes the start position: the subscription covers
        // the current tip and delivers only what lands after it.
        let start = self.tip();
        let mut first = proto::SubscribeTransactionsResponse::default();
        first.watermark = Some(watermark((start + 1, 0)));
        let ledger = self.clone();
        let later = futures::stream::unfold(start + 1, move |next| {
            let ledger = ledger.clone();
            async move {
                while ledger.tip() < next {
                    tokio::time::sleep(TICK / 5).await;
                }
                let mut frame = proto::SubscribeTransactionsResponse::default();
                frame.transaction = ledger
                    .committed_at
                    .filter(|&checkpoint| checkpoint == next)
                    .map(|checkpoint| ledger.executed_transaction(checkpoint));
                frame.watermark = Some(if frame.transaction.is_some() {
                    watermark((next, 1))
                } else {
                    watermark((next + 1, 0))
                });
                Some((Ok(frame), next + 1))
            }
        });
        let frames = futures::stream::once(async move { Ok(first) }).chain(later);
        Ok(tonic::Response::new(Box::pin(frames)))
    }
}

async fn spawn_mock_ledger(ledger: MockLedger) -> Client {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock server listener");
    let addr = listener.local_addr().expect("mock server local addr");
    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(TransactionExecutionServiceServer::new(ledger.clone()))
            .add_service(LedgerServiceServer::new(ledger.clone()))
            .add_service(SubscriptionServiceServer::new(ledger))
            .serve_with_incoming(tonic::transport::server::TcpIncoming::from(listener))
            .await
            .expect("mock server exited with an error");
    });
    Client::new(format!("http://{addr}")).expect("client")
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

fn execute_request(transaction: proto::Transaction) -> proto::ExecuteTransactionRequest {
    proto::ExecuteTransactionRequest::default().with_transaction(transaction)
}

/// The common path: the transaction lands a few checkpoints after execution
/// and arrives on the live subscription. The execution response comes back
/// with the checkpoint and its timestamp filled in.
#[tokio::test(flavor = "multi_thread")]
async fn transaction_found_on_subscription() {
    let (transaction, digest) = test_transaction();
    let ledger = MockLedger {
        committed_at: Some(START_TIP + 6),
        ..MockLedger::new(digest.clone())
    };
    let executions = ledger.executions.clone();
    let mut client = spawn_mock_ledger(ledger).await;

    let response = client
        .execute_transaction_and_wait_for_checkpoint(
            execute_request(transaction),
            Duration::from_secs(5),
        )
        .await
        .expect("execution and checkpoint confirmation");
    let executed = response.get_ref().transaction();
    assert_eq!(executed.digest(), digest);
    assert_eq!(executed.checkpoint(), START_TIP + 6);
    assert_eq!(
        executed.timestamp,
        Some(checkpoint_timestamp(START_TIP + 6))
    );
    assert_eq!(executions.load(Ordering::SeqCst), 1);
}

/// A transaction that lands after the height was read but before the
/// subscription starts is behind the subscription's start position, so only
/// the stream's replay from that height can find it. A stream that started
/// at the tip would miss it and time out.
#[tokio::test(flavor = "multi_thread")]
async fn transaction_committed_before_subscription_starts_is_replayed() {
    let (transaction, digest) = test_transaction();
    let mut client = spawn_mock_ledger(MockLedger {
        committed_at: Some(START_TIP + 1),
        subscribe_delay: TICK * 6,
        ..MockLedger::new(digest)
    })
    .await;

    let response = client
        .execute_transaction_and_wait_for_checkpoint(
            execute_request(transaction),
            Duration::from_secs(5),
        )
        .await
        .expect("transaction replayed from the start height");
    assert_eq!(response.get_ref().transaction().checkpoint(), START_TIP + 1);
}

/// The transaction can land in a checkpoint while the execution RPC is still
/// in flight; that sighting must be kept and used once execution returns,
/// rather than waiting for a checkpoint that has already gone by.
#[tokio::test(flavor = "multi_thread")]
async fn transaction_checkpointed_during_slow_execution() {
    let (transaction, digest) = test_transaction();
    let mut client = spawn_mock_ledger(MockLedger {
        committed_at: Some(START_TIP + 2),
        exec_delay: TICK * 10,
        ..MockLedger::new(digest)
    })
    .await;

    let response = client
        .execute_transaction_and_wait_for_checkpoint(
            execute_request(transaction),
            Duration::from_millis(100),
        )
        .await
        .expect("checkpoint seen during execution");
    assert_eq!(response.get_ref().transaction().checkpoint(), START_TIP + 2);
}

/// A transaction already in a checkpoint below the start height (a retry, or
/// the same transaction sent by someone else) is answered from the ledger
/// without executing it again. The lookup carries the caller's read mask
/// plus the fields this method always populates.
#[tokio::test(flavor = "multi_thread")]
async fn already_checkpointed_transaction_is_not_executed() {
    let (transaction, digest) = test_transaction();
    let ledger = MockLedger {
        committed_at: Some(START_TIP - 2),
        ..MockLedger::new(digest.clone())
    };
    let executions = ledger.executions.clone();
    let lookups = ledger.lookups.clone();
    let mut client = spawn_mock_ledger(ledger).await;

    let request = execute_request(transaction).with_read_mask(FieldMask {
        paths: vec!["effects.status".to_owned()],
    });
    let response = client
        .execute_transaction_and_wait_for_checkpoint(request, Duration::from_secs(5))
        .await
        .expect("committed duplicate answered from the ledger");
    assert_eq!(response.get_ref().transaction().checkpoint(), START_TIP - 2);
    assert_eq!(response.get_ref().transaction().digest(), digest);
    assert_eq!(executions.load(Ordering::SeqCst), 0);

    let lookup_mask = lookups.lock().unwrap()[0]
        .read_mask
        .clone()
        .expect("lookup read mask");
    for path in ["effects.status", "digest", "checkpoint", "timestamp"] {
        assert!(
            lookup_mask.paths.iter().any(|p| p == path),
            "lookup read mask is missing `{path}`: {lookup_mask:?}"
        );
    }
}

/// An execution error for a transaction that nonetheless committed (for
/// example a resubmission racing the original submission) is downgraded to
/// success by a final ledger lookup.
#[tokio::test(flavor = "multi_thread")]
async fn execution_error_with_committed_transaction_succeeds() {
    let (transaction, digest) = test_transaction();
    let mut client = spawn_mock_ledger(MockLedger {
        committed_at: Some(START_TIP + 2),
        exec_delay: TICK * 6,
        exec_error: Some(tonic::Status::aborted("transaction already being executed")),
        ..MockLedger::new(digest.clone())
    })
    .await;

    let response = client
        .execute_transaction_and_wait_for_checkpoint(
            execute_request(transaction),
            Duration::from_secs(5),
        )
        .await
        .expect("execution error downgraded for a committed transaction");
    assert_eq!(response.get_ref().transaction().checkpoint(), START_TIP + 2);
    assert_eq!(response.get_ref().transaction().digest(), digest);
}

/// An execution error for a transaction the ledger has no checkpoint for
/// still surfaces as `RpcError`.
#[tokio::test(flavor = "multi_thread")]
async fn execution_error_without_commit_surfaces() {
    let (transaction, digest) = test_transaction();
    let mut client = spawn_mock_ledger(MockLedger {
        exec_error: Some(tonic::Status::internal("boom")),
        ..MockLedger::new(digest)
    })
    .await;

    let err = client
        .execute_transaction_and_wait_for_checkpoint(
            execute_request(transaction),
            Duration::from_secs(5),
        )
        .await
        .expect_err("execution error with no committed transaction");
    match err {
        ExecuteAndWaitError::RpcError(status) => {
            assert_eq!(status.code(), tonic::Code::Internal);
        }
        other => panic!("expected RpcError, got {other:?}"),
    }
}

/// A successful execution whose transaction never lands times out, and the
/// error still carries the execution response.
#[tokio::test(flavor = "multi_thread")]
async fn checkpoint_wait_times_out_with_execution_response() {
    let (transaction, digest) = test_transaction();
    let mut client = spawn_mock_ledger(MockLedger::new(digest.clone())).await;

    let err = client
        .execute_transaction_and_wait_for_checkpoint(
            execute_request(transaction),
            Duration::from_millis(300),
        )
        .await
        .expect_err("transaction never lands");
    match err {
        ExecuteAndWaitError::CheckpointTimeout(response) => {
            assert_eq!(response.get_ref().transaction().digest(), digest);
        }
        other => panic!("expected CheckpointTimeout, got {other:?}"),
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
    assert!(matches!(err, ExecuteAndWaitError::MissingTransaction));
}
