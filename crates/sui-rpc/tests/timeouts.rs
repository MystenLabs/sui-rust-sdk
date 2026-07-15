//! Integration tests for the client's response-headers timeout
//! (`Client::with_response_headers_timeout`).
//!
//! The timeout feeds `tonic::transport::Endpoint::timeout`, which tonic
//! enforces locally against the wait for response headers only. These tests
//! run a mock fullnode built from this crate's own generated server stubs and
//! assert:
//!
//! - a unary call whose handler never responds (so response headers never
//!   arrive) fails at the timeout with `DeadlineExceeded` instead of hanging;
//! - a per-call `grpc-timeout` deadline combines with the timeout by taking
//!   the shorter of the two, so it can tighten the local bound but never
//!   extend it;
//! - the timer stops once response headers arrive, so a client-wide timeout
//!   does not cut off long-lived streaming responses;
//! - a healthy unary call is unaffected.

use std::time::Duration;

use futures::StreamExt;
use proto::ledger_service_server::LedgerService;
use proto::ledger_service_server::LedgerServiceServer;
use proto::subscription_service_server::SubscriptionService;
use proto::subscription_service_server::SubscriptionServiceServer;
use sui_rpc::Client;
use sui_rpc::proto::sui::rpc::v2 as proto;

/// Bound for calls that must complete or fail promptly; generous so slow CI
/// machines do not flake, but far below [`UNARY_STALL`], so hitting it means
/// a timeout that should have fired did not.
const FAILURE_BOUND: Duration = Duration::from_secs(5);

/// Response-headers timeout used by the tests that expect it to fire.
const HEADERS_TIMEOUT: Duration = Duration::from_millis(400);

/// How long the mock unary handler stalls before responding. A tonic server
/// does not send response headers for a unary call until the handler
/// returns, so this stalls the headers phase well past every bound above.
const UNARY_STALL: Duration = Duration::from_secs(60);

/// Interval between items on the mock subscription stream.
const ITEM_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Clone)]
struct MockServer {
    unary_delay: Duration,
}

#[tonic::async_trait]
impl LedgerService for MockServer {
    async fn get_service_info(
        &self,
        _request: tonic::Request<proto::GetServiceInfoRequest>,
    ) -> Result<tonic::Response<proto::GetServiceInfoResponse>, tonic::Status> {
        tokio::time::sleep(self.unary_delay).await;
        let mut info = proto::GetServiceInfoResponse::default();
        info.chain_id = Some("mock".to_owned());
        Ok(tonic::Response::new(info))
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
        // Response headers go out as soon as this handler returns; the items
        // then trickle out on an interval so the stream stays alive well past
        // the response-headers timeout.
        let stream = futures::stream::iter(0u64..).then(|seq| async move {
            tokio::time::sleep(ITEM_INTERVAL).await;
            let mut resp = proto::SubscribeCheckpointsResponse::default();
            resp.cursor = Some(seq);
            Ok(resp)
        });
        Ok(tonic::Response::new(Box::pin(stream)))
    }
}

async fn spawn_mock_server(unary_delay: Duration) -> std::net::SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock server listener");
    let addr = listener.local_addr().expect("mock server local addr");
    tokio::spawn(async move {
        let server = MockServer { unary_delay };
        tonic::transport::Server::builder()
            .add_service(LedgerServiceServer::new(server.clone()))
            .add_service(SubscriptionServiceServer::new(server))
            .serve_with_incoming(tonic::transport::server::TcpIncoming::from(listener))
            .await
            .expect("mock server exited with an error");
    });
    addr
}

/// Issue a unary call and expect it to fail before [`FAILURE_BOUND`],
/// returning the status it failed with.
async fn expect_prompt_failure(
    client: &mut Client,
    request: tonic::Request<proto::GetServiceInfoRequest>,
) -> tonic::Status {
    tokio::time::timeout(
        FAILURE_BOUND,
        client.ledger_client().get_service_info(request),
    )
    .await
    .expect("call ran past the failure bound instead of timing out")
    .expect_err("call against a stalled handler must fail")
}

/// The response-headers timeout bounds a unary call whose handler never
/// responds, and expiry surfaces as `DeadlineExceeded` (normalized from the
/// `Cancelled` code tonic uses for its timeout).
#[tokio::test(flavor = "multi_thread")]
async fn response_headers_timeout_bounds_a_stalled_unary_call() {
    let addr = spawn_mock_server(UNARY_STALL).await;
    let mut client = Client::new(format!("http://{addr}"))
        .expect("client")
        .with_response_headers_timeout(HEADERS_TIMEOUT);

    let request = tonic::Request::new(proto::GetServiceInfoRequest::default());
    let status = expect_prompt_failure(&mut client, request).await;
    assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
}

/// tonic enforces the shorter of the response-headers timeout and a per-call
/// `grpc-timeout` deadline, so a longer per-call deadline cannot extend the
/// locally enforced bound.
#[tokio::test(flavor = "multi_thread")]
async fn per_call_deadline_cannot_extend_the_timeout() {
    let addr = spawn_mock_server(UNARY_STALL).await;
    let mut client = Client::new(format!("http://{addr}"))
        .expect("client")
        .with_response_headers_timeout(HEADERS_TIMEOUT);

    let mut request = tonic::Request::new(proto::GetServiceInfoRequest::default());
    request.set_timeout(Duration::from_secs(60));

    let status = expect_prompt_failure(&mut client, request).await;
    assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
}

/// A per-call deadline shorter than the response-headers timeout tightens
/// the locally enforced bound below it.
#[tokio::test(flavor = "multi_thread")]
async fn per_call_deadline_tightens_below_the_timeout() {
    let addr = spawn_mock_server(UNARY_STALL).await;
    let mut client = Client::new(format!("http://{addr}"))
        .expect("client")
        .with_response_headers_timeout(Duration::from_secs(60));

    let mut request = tonic::Request::new(proto::GetServiceInfoRequest::default());
    request.set_timeout(HEADERS_TIMEOUT);

    let status = expect_prompt_failure(&mut client, request).await;
    assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
}

/// The timer stops once response headers arrive: a streaming call keeps
/// delivering items long past the response-headers timeout, so a client-wide
/// timeout does not cut off long-lived subscriptions.
#[tokio::test(flavor = "multi_thread")]
async fn streams_outlive_the_response_headers_timeout() {
    let addr = spawn_mock_server(UNARY_STALL).await;
    let mut client = Client::new(format!("http://{addr}"))
        .expect("client")
        .with_response_headers_timeout(HEADERS_TIMEOUT);

    let mut stream = client
        .subscription_client()
        .subscribe_checkpoints(proto::SubscribeCheckpointsRequest::default())
        .await
        .expect("subscribe")
        .into_inner();

    // 20 items at 50ms apart keep the stream alive for about a second, well
    // past the 400ms response-headers timeout.
    for expected in 0..20u64 {
        let item = tokio::time::timeout(FAILURE_BOUND, stream.next())
            .await
            .expect("stream made no progress")
            .expect("stream ended unexpectedly")
            .expect("stream item");
        assert_eq!(item.cursor, Some(expected));
    }
}

/// A healthy unary call completes normally with the timeout configured.
#[tokio::test(flavor = "multi_thread")]
async fn healthy_calls_are_unaffected() {
    let addr = spawn_mock_server(Duration::ZERO).await;
    let mut client = Client::new(format!("http://{addr}"))
        .expect("client")
        .with_response_headers_timeout(HEADERS_TIMEOUT);

    let info = tokio::time::timeout(
        FAILURE_BOUND,
        client
            .ledger_client()
            .get_service_info(proto::GetServiceInfoRequest::default()),
    )
    .await
    .expect("healthy call hung")
    .expect("healthy call failed")
    .into_inner();
    assert_eq!(info.chain_id.as_deref(), Some("mock"));
}
