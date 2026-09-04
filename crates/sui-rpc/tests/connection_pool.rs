//! End-to-end tests for `Client::with_num_connections`.
//!
//! A single HTTP/2 connection caps throughput at what its flow-control window
//! and its one driver task can sustain, so a client can be limited by its own
//! transport well before the server is. Opening several connections and
//! rotating requests across them lifts that cap.
//!
//! These tests run a mock fullnode over a listener that counts accepted TCP
//! connections, which is the only way to observe how many connections a client
//! actually opened, and assert:
//!
//! - the default client still opens exactly one connection, so the pool is
//!   opt-in and the existing transport behavior is unchanged;
//! - a pooled client opens one connection per configured slot and rotates
//!   evenly across them;
//! - streams held concurrently through a single service client land on
//!   different connections, which is what separates per-request assignment
//!   from assigning a whole service client to one connection.

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use futures::StreamExt;
use proto::ledger_service_server::LedgerService;
use proto::ledger_service_server::LedgerServiceServer;
use proto::subscription_service_server::SubscriptionService;
use proto::subscription_service_server::SubscriptionServiceServer;
use sui_rpc::Client;
use sui_rpc::proto::sui::rpc::v2 as proto;

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
        // One item so the caller can await response headers, then quiet
        // forever: the stream stays open and in flight without pinning any
        // meaningful amount of the connection's window.
        let stream =
            futures::stream::once(async { Ok(proto::SubscribeCheckpointsResponse::default()) })
                .chain(futures::stream::pending());
        Ok(tonic::Response::new(Box::pin(stream)))
    }
}

/// Serve the mock fullnode, returning its address and a counter of accepted
/// TCP connections.
async fn spawn_counting_mock_server() -> (SocketAddr, Arc<AtomicUsize>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock server listener");
    let addr = listener.local_addr().expect("mock server local addr");
    let accepted = Arc::new(AtomicUsize::new(0));

    let incoming = futures::stream::unfold(
        (listener, accepted.clone()),
        |(listener, accepted)| async move {
            let accept = listener.accept().await.map(|(socket, _)| {
                accepted.fetch_add(1, Ordering::Relaxed);
                socket
            });
            Some((accept, (listener, accepted)))
        },
    );

    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(LedgerServiceServer::new(MockServer))
            .add_service(SubscriptionServiceServer::new(MockServer))
            .serve_with_incoming(incoming)
            .await
            .expect("mock server exited with an error");
    });

    (addr, accepted)
}

async fn call(client: &mut Client) {
    client
        .ledger_client()
        .get_service_info(proto::GetServiceInfoRequest::default())
        .await
        .expect("get_service_info");
}

/// The pool is opt-in: an unconfigured client multiplexes everything over one
/// connection, as it always has.
#[tokio::test(flavor = "multi_thread")]
async fn default_client_opens_one_connection() {
    let (addr, accepted) = spawn_counting_mock_server().await;
    let mut client = Client::new(format!("http://{addr}")).expect("client");

    for _ in 0..8 {
        call(&mut client).await;
    }

    assert_eq!(accepted.load(Ordering::Relaxed), 1);
}

/// Requests rotate across the configured connections, so the pool reaches its
/// full width and stays evenly loaded rather than leaving connections idle.
#[tokio::test(flavor = "multi_thread")]
async fn pooled_client_rotates_across_every_connection() {
    let (addr, accepted) = spawn_counting_mock_server().await;
    let mut client = Client::new(format!("http://{addr}"))
        .expect("client")
        .with_num_connections(4);

    // Connections are lazy, so the fourth is only dialed once the rotation
    // reaches it; a further four calls must reuse rather than keep dialing.
    for _ in 0..8 {
        call(&mut client).await;
    }

    assert_eq!(accepted.load(Ordering::Relaxed), 4);
}

/// Assignment happens per request, not per service client: streams opened
/// through one service client and held concurrently occupy different
/// connections. This is the bulk-read case the pool exists for.
#[tokio::test(flavor = "multi_thread")]
async fn concurrent_streams_from_one_service_client_spread() {
    let (addr, accepted) = spawn_counting_mock_server().await;
    let mut client = Client::new(format!("http://{addr}"))
        .expect("client")
        .with_num_connections(4);

    let mut subscriptions = client.subscription_client();
    let mut held = Vec::new();
    for _ in 0..4 {
        held.push(
            subscriptions
                .subscribe_checkpoints(proto::SubscribeCheckpointsRequest::default())
                .await
                .expect("subscribe")
                .into_inner(),
        );
    }

    assert_eq!(accepted.load(Ordering::Relaxed), 4);
    drop(held);
}

/// A count of zero is meaningless for a transport and would make the rotation
/// modulo divide by zero, so it is clamped to a usable client.
#[tokio::test(flavor = "multi_thread")]
async fn zero_connections_is_clamped() {
    let (addr, accepted) = spawn_counting_mock_server().await;
    let mut client = Client::new(format!("http://{addr}"))
        .expect("client")
        .with_num_connections(0);

    call(&mut client).await;
    call(&mut client).await;

    assert_eq!(accepted.load(Ordering::Relaxed), 1);
}
