//! End-to-end regression tests for HTTP/2 connection-level flow-control
//! starvation.
//!
//! Every RPC made through a `Client` is multiplexed over one HTTP/2
//! connection. Response data counts against the connection-level receive
//! window until the application polls it out of the stream, so streaming
//! responses that are held without being polled pin the shared window. With
//! hyper's default windows (2 MiB stream / 5 MiB connection), a handful of
//! parked streams exhaust the connection window and every RPC on the channel
//! hangs indefinitely while the TCP connection and HTTP/2 keepalives stay
//! healthy.
//!
//! These tests run a mock fullnode built from this crate's own generated
//! server stubs, whose checkpoint subscription pushes large frames as fast as
//! the transport accepts them, and assert:
//!
//! - the failure mode exists (a control test with the old, small windows and
//!   the watchdog disabled), so the other tests are known to create real
//!   starvation pressure;
//! - the `Client::new` window defaults absorb parked streams that would
//!   starve the old configuration;
//! - the idle-body watchdog reaps parked streams, returning their pinned
//!   window and un-sticking the connection without any help from the caller;
//! - a per-call `grpc-timeout` deadline (`tonic::Request::set_timeout`)
//!   bounds a streaming call end to end even while items are flowing.

use std::time::Duration;

use futures::StreamExt;
use proto::ledger_service_server::LedgerService;
use proto::ledger_service_server::LedgerServiceServer;
use proto::subscription_service_server::SubscriptionService;
use proto::subscription_service_server::SubscriptionServiceServer;
use sui_rpc::Client;
use sui_rpc::proto::sui::rpc::v2 as proto;

/// Generous bound for calls that must complete; failure mode is a hang, so
/// hitting this means the regression is back.
const VICTIM_TIMEOUT: Duration = Duration::from_secs(5);

/// Bound for calls that must hang in the control test. Locally the victim
/// completes in single-digit milliseconds when the connection is healthy, so
/// this is orders of magnitude past success while keeping the test fast.
const HANG_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Clone)]
struct MockServer;

#[tonic::async_trait]
impl LedgerService for MockServer {
    async fn get_service_info(
        &self,
        _request: tonic::Request<proto::GetServiceInfoRequest>,
    ) -> Result<tonic::Response<proto::GetServiceInfoResponse>, tonic::Status> {
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
        // Emit large frames as fast as the transport will take them,
        // mimicking a checkpoint subscription on a busy network. Compression
        // is not enabled server-side, so the payload's compressibility is
        // irrelevant: each response occupies its full size in the client's
        // receive windows.
        let stream = futures::stream::iter(0u64..).map(|seq| {
            let mut checkpoint = proto::Checkpoint::default();
            checkpoint.sequence_number = Some(seq);
            checkpoint.digest = Some("x".repeat(512 * 1024));
            let mut resp = proto::SubscribeCheckpointsResponse::default();
            resp.cursor = Some(seq);
            resp.checkpoint = Some(checkpoint);
            Ok(resp)
        });
        Ok(tonic::Response::new(Box::pin(stream)))
    }
}

async fn spawn_mock_server() -> std::net::SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock server listener");
    let addr = listener.local_addr().expect("mock server local addr");
    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(LedgerServiceServer::new(MockServer))
            .add_service(SubscriptionServiceServer::new(MockServer))
            .serve_with_incoming(tonic::transport::server::TcpIncoming::from(listener))
            .await
            .expect("mock server exited with an error");
    });
    addr
}

/// A client with the pre-fix transport configuration: hyper's default
/// flow-control windows (2 MiB stream / 5 MiB connection) and no watchdog.
fn legacy_client(addr: std::net::SocketAddr) -> Client {
    let endpoint = tonic::transport::Endpoint::new(format!("http://{addr}")).expect("endpoint");
    Client::from_endpoint(&endpoint).without_body_idle_timeout()
}

/// Open `count` checkpoint subscriptions and hold them without polling. The
/// server starts pushing as soon as each response stream opens, so the unread
/// data pins the client's receive windows whether or not the streams are ever
/// polled.
async fn park_subscriptions(
    client: &mut Client,
    count: usize,
) -> Vec<tonic::Streaming<proto::SubscribeCheckpointsResponse>> {
    let mut parked = Vec::with_capacity(count);
    for _ in 0..count {
        let stream = client
            .subscription_client()
            .subscribe_checkpoints(proto::SubscribeCheckpointsRequest::default())
            .await
            .expect("subscribe")
            .into_inner();
        parked.push(stream);
    }
    // Give the server time to fill every parked stream's window.
    tokio::time::sleep(Duration::from_secs(1)).await;
    parked
}

async fn victim_call(client: &mut Client, bound: Duration) -> Result<(), String> {
    match tokio::time::timeout(
        bound,
        client
            .ledger_client()
            .get_service_info(proto::GetServiceInfoRequest::default()),
    )
    .await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(status)) => Err(format!("victim call failed with status: {status}")),
        Err(_) => Err(format!("victim call hung for more than {bound:?}")),
    }
}

/// Control: with the pre-fix configuration the starvation reproduces, and
/// dropping the parked streams recovers the connection. This proves the mock
/// server creates real starvation pressure, so the passing tests below are
/// known to be exercising the defenses rather than a toothless setup.
#[tokio::test(flavor = "multi_thread")]
async fn control_small_windows_starve_without_watchdog() {
    let addr = spawn_mock_server().await;
    let mut client = legacy_client(addr);

    victim_call(&mut client, VICTIM_TIMEOUT)
        .await
        .expect("baseline call on a fresh connection");

    let parked = park_subscriptions(&mut client, 4).await;
    let starved = victim_call(&mut client, HANG_TIMEOUT).await;
    assert!(
        starved.is_err(),
        "victim call should hang with 4 parked streams on 5 MiB connection window"
    );

    // The workaround, mechanized: dropping the stalled calls releases their
    // pinned window and un-sticks the connection.
    drop(parked);
    victim_call(&mut client, VICTIM_TIMEOUT)
        .await
        .expect("victim call after dropping parked streams");
}

/// With the `Client::new` window defaults (64 MiB connection window), the
/// parked streams that starve the legacy configuration are absorbed: unary
/// calls and fresh, actively polled streams keep working. The watchdog is
/// disabled to prove the windows alone carry the load.
#[tokio::test(flavor = "multi_thread")]
async fn default_windows_absorb_parked_streams() {
    let addr = spawn_mock_server().await;
    let mut client = Client::new(format!("http://{addr}"))
        .expect("client")
        .without_body_idle_timeout();

    let _parked = park_subscriptions(&mut client, 8).await;

    victim_call(&mut client, VICTIM_TIMEOUT)
        .await
        .expect("victim call with 8 parked streams");

    // A fresh, actively polled subscription must also make progress.
    let mut fresh = client
        .subscription_client()
        .subscribe_checkpoints(proto::SubscribeCheckpointsRequest::default())
        .await
        .expect("subscribe")
        .into_inner();
    for _ in 0..3 {
        let item = tokio::time::timeout(VICTIM_TIMEOUT, fresh.next())
            .await
            .expect("fresh subscription made no progress")
            .expect("fresh subscription ended unexpectedly")
            .expect("fresh subscription item");
        assert!(item.checkpoint.is_some());
    }
}

/// The idle-body watchdog reaps parked streams on the legacy window
/// configuration: their pinned window is returned without the caller doing
/// anything, the connection un-sticks, and the parked calls observe
/// `DeadlineExceeded` when finally polled.
#[tokio::test(flavor = "multi_thread")]
async fn watchdog_reaps_parked_streams_and_recovers_the_connection() {
    let addr = spawn_mock_server().await;
    let endpoint = tonic::transport::Endpoint::new(format!("http://{addr}")).expect("endpoint");
    let mut client =
        Client::from_endpoint(&endpoint).with_body_idle_timeout(Duration::from_millis(500));

    let mut parked = park_subscriptions(&mut client, 4).await;

    // Wait past the idle timeout: the watchdog must reset every parked
    // stream, releasing the pinned connection window.
    tokio::time::sleep(Duration::from_secs(2)).await;

    victim_call(&mut client, VICTIM_TIMEOUT)
        .await
        .expect("victim call after the watchdog reaped the parked streams");

    // A parked stream, when finally polled, drains whatever the bridge had
    // buffered and then surfaces the watchdog's cut.
    let mut stream = parked.pop().expect("parked stream");
    let status = loop {
        match tokio::time::timeout(VICTIM_TIMEOUT, stream.next())
            .await
            .expect("parked stream made no progress after the watchdog cut")
        {
            Some(Ok(_buffered)) => continue,
            Some(Err(status)) => break status,
            None => panic!("parked stream ended cleanly instead of with a status"),
        }
    };
    assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
}

/// `tonic::Request::set_timeout` bounds a streaming call end to end: the
/// stream is cut at the deadline even though items are flowing steadily and
/// the idle watchdog never fires.
#[tokio::test(flavor = "multi_thread")]
async fn set_timeout_bounds_a_streaming_call() {
    let addr = spawn_mock_server().await;
    let mut client = Client::new(format!("http://{addr}")).expect("client");

    let mut request = tonic::Request::new(proto::SubscribeCheckpointsRequest::default());
    request.set_timeout(Duration::from_secs(1));

    let mut stream = client
        .subscription_client()
        .subscribe_checkpoints(request)
        .await
        .expect("subscribe")
        .into_inner();

    let mut items = 0u64;
    let status = loop {
        match tokio::time::timeout(VICTIM_TIMEOUT, stream.next())
            .await
            .expect("stream made no progress before the deadline")
        {
            Some(Ok(_)) => items += 1,
            Some(Err(status)) => break status,
            None => panic!("stream ended cleanly instead of at the deadline"),
        }
    };
    assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
    assert!(
        items > 0,
        "expected the stream to deliver items until the deadline"
    );
}
