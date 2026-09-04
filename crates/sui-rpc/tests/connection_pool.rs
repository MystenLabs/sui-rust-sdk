//! End-to-end tests for `Client::with_num_connections`.
//!
//! A single HTTP/2 connection caps throughput at what its flow-control window
//! and its one driver task can sustain, so a client can be limited by its own
//! transport well before the server is. Opening several connections and
//! balancing requests across them lifts that cap.
//!
//! These tests run a mock fullnode over a listener that counts accepted TCP
//! connections and a service that records which connection each request
//! arrived on, since neither is observable from the client side. They assert:
//!
//! - the default client still opens exactly one connection, so the pool is
//!   opt-in and the existing transport behavior is unchanged;
//! - a pooled client opens one connection per configured slot;
//! - requests genuinely reach every connection rather than piling onto one.
//!
//! Placement is random per request (tonic's balancer reports a constant load,
//! so its pick-two-choose-least degenerates to a random pick), so the
//! distribution test asserts coverage over many requests rather than an even
//! split, which would be flaky.

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use futures::StreamExt;
use proto::ledger_service_server::LedgerService;
use proto::ledger_service_server::LedgerServiceServer;
use proto::subscription_service_server::SubscriptionService;
use proto::subscription_service_server::SubscriptionServiceServer;
use sui_rpc::Client;
use sui_rpc::proto::sui::rpc::v2 as proto;

/// Requests seen per client connection, keyed by the client's source port.
type PerConnection = Arc<Mutex<BTreeMap<u16, usize>>>;

#[derive(Clone)]
struct MockServer {
    per_connection: PerConnection,
}

#[tonic::async_trait]
impl LedgerService for MockServer {
    async fn get_service_info(
        &self,
        request: tonic::Request<proto::GetServiceInfoRequest>,
    ) -> Result<tonic::Response<proto::GetServiceInfoResponse>, tonic::Status> {
        let port = request.remote_addr().expect("peer address").port();
        *self.per_connection.lock().unwrap().entry(port).or_insert(0) += 1;

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

struct MockFullnode {
    addr: SocketAddr,
    accepted: Arc<AtomicUsize>,
    per_connection: PerConnection,
}

impl MockFullnode {
    async fn spawn() -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind mock server listener");
        let addr = listener.local_addr().expect("mock server local addr");
        let accepted = Arc::new(AtomicUsize::new(0));
        let per_connection: PerConnection = Default::default();

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

        let server = MockServer {
            per_connection: per_connection.clone(),
        };
        tokio::spawn(async move {
            tonic::transport::Server::builder()
                .add_service(LedgerServiceServer::new(server.clone()))
                .add_service(SubscriptionServiceServer::new(server))
                .serve_with_incoming(incoming)
                .await
                .expect("mock server exited with an error");
        });

        Self {
            addr,
            accepted,
            per_connection,
        }
    }

    fn client(&self) -> Client {
        Client::new(format!("http://{}", self.addr)).expect("client")
    }

    fn connections_opened(&self) -> usize {
        self.accepted.load(Ordering::Relaxed)
    }

    /// Request counts per connection, one entry per connection that served at
    /// least one request.
    fn requests_per_connection(&self) -> Vec<usize> {
        self.per_connection
            .lock()
            .unwrap()
            .values()
            .copied()
            .collect()
    }
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
    let server = MockFullnode::spawn().await;
    let mut client = server.client();

    for _ in 0..8 {
        call(&mut client).await;
    }

    assert_eq!(server.connections_opened(), 1);
    assert_eq!(server.requests_per_connection(), vec![8]);
}

/// A pooled client opens exactly the configured number of connections, and no
/// more as traffic continues.
#[tokio::test(flavor = "multi_thread")]
async fn pooled_client_opens_one_connection_per_slot() {
    let server = MockFullnode::spawn().await;
    let mut client = server.client().with_num_connections(4);

    for _ in 0..16 {
        call(&mut client).await;
    }

    assert_eq!(server.connections_opened(), 4);
}

/// Requests reach every connection rather than piling onto one, which is the
/// property the pool exists for. Placement is random per request, so this
/// asserts coverage over enough requests that missing a connection is
/// vanishingly unlikely (~4 * 0.75^200) rather than asserting an even split.
#[tokio::test(flavor = "multi_thread")]
async fn requests_reach_every_connection() {
    let server = MockFullnode::spawn().await;
    let mut client = server.client().with_num_connections(4);

    for _ in 0..200 {
        call(&mut client).await;
    }

    let per_connection = server.requests_per_connection();
    assert_eq!(per_connection.len(), 4, "every connection served requests");
    assert_eq!(per_connection.iter().sum::<usize>(), 200);
}

/// Streams held open concurrently are spread rather than all landing on the
/// connection that served the first one.
#[tokio::test(flavor = "multi_thread")]
async fn concurrently_held_streams_are_spread() {
    let server = MockFullnode::spawn().await;
    let mut client = server.client().with_num_connections(4);

    let mut subscriptions = client.subscription_client();
    let mut held = Vec::new();
    for _ in 0..32 {
        held.push(
            subscriptions
                .subscribe_checkpoints(proto::SubscribeCheckpointsRequest::default())
                .await
                .expect("subscribe")
                .into_inner(),
        );
    }

    assert_eq!(server.connections_opened(), 4);
    drop(held);
}

/// A count of zero is meaningless for a transport, so it is clamped to a
/// usable client rather than producing an empty pool.
#[tokio::test(flavor = "multi_thread")]
async fn zero_connections_is_clamped() {
    let server = MockFullnode::spawn().await;
    let mut client = server.client().with_num_connections(0);

    call(&mut client).await;
    call(&mut client).await;

    assert_eq!(server.connections_opened(), 1);
}
