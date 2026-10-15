//! End-to-end tests for resending requests the server never processed
//! (gRPC transparent retry, gRFC A6).
//!
//! A server or load balancer draining an HTTP/2 connection sends GOAWAY with
//! the id of the last stream it will process; streams above it were never
//! processed and are safe to resend on a new connection. Streams at or below
//! it may have been processed and must not be resent.
//!
//! The first connections to the mock fullnode are served by a minimal raw
//! HTTP/2 peer that sends GOAWAY exactly when each scenario needs it, which
//! no off-the-shelf server API allows. Later connections are served by a real
//! gRPC server, so a resent request succeeds there.

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;

use proto::ledger_service_server::LedgerService;
use proto::ledger_service_server::LedgerServiceServer;
use sui_rpc::Client;
use sui_rpc::proto::sui::rpc::v2 as proto;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::oneshot;

const CALL_TIMEOUT: Duration = Duration::from_secs(10);

const CLIENT_PREFACE_LEN: usize = 24;
const FRAME_HEADER_LEN: usize = 9;
const FRAME_HEADERS: u8 = 0x1;
const FRAME_SETTINGS: u8 = 0x4;
const FRAME_PING: u8 = 0x6;
const FRAME_GOAWAY: u8 = 0x7;
const FLAG_ACK: u8 = 0x1;

const NO_ERROR: u32 = 0x0;
const ENHANCE_YOUR_CALM: u32 = 0xb;

#[derive(Clone, Default)]
struct MockServer {
    requests_served: Arc<AtomicUsize>,
    /// The `grpc-timeout` header of each request served, if it had one.
    deadlines_received: Arc<Mutex<Vec<String>>>,
    response_delay: Duration,
}

#[tonic::async_trait]
impl LedgerService for MockServer {
    async fn get_service_info(
        &self,
        request: tonic::Request<proto::GetServiceInfoRequest>,
    ) -> Result<tonic::Response<proto::GetServiceInfoResponse>, tonic::Status> {
        self.requests_served.fetch_add(1, Ordering::SeqCst);
        if let Some(deadline) = request.metadata().get("grpc-timeout") {
            let deadline = deadline.to_str().expect("ascii grpc-timeout").to_owned();
            self.deadlines_received.lock().unwrap().push(deadline);
        }
        tokio::time::sleep(self.response_delay).await;
        let mut info = proto::GetServiceInfoResponse::default();
        info.chain_id = Some("mock".to_owned());
        Ok(tonic::Response::new(info))
    }
}

/// How a raw HTTP/2 peer treats the first request on its connection. It
/// never answers a request.
#[derive(Clone, Copy)]
enum RawPeer {
    /// After `delay`, answer the first request with GOAWAY(last_stream_id =
    /// 0, `error_code`): the server processed nothing on this connection.
    Refuse { error_code: u32, delay: Duration },
    /// Accept the first request, then send GOAWAY(last_stream_id = that
    /// stream, NO_ERROR). The connection stays open for the in-flight
    /// request, as a draining load balancer keeps it.
    DrainWithRequestInFlight,
    /// Accept the first request, send GOAWAY(last_stream_id = that stream,
    /// NO_ERROR), and close the connection: the request may have been
    /// processed.
    CloseAfterAccepting,
}

/// A graceful drain that processed nothing on the connection.
const REFUSE: RawPeer = RawPeer::Refuse {
    error_code: NO_ERROR,
    delay: Duration::ZERO,
};

struct MockFullnode {
    addr: SocketAddr,
    connections_opened: Arc<AtomicUsize>,
    raw_peer_streams: Arc<AtomicUsize>,
    requests_served: Arc<AtomicUsize>,
    deadlines_received: Arc<Mutex<Vec<String>>>,
}

impl MockFullnode {
    /// Starts the mock. The first connections are served by `raw_peers`, in
    /// order; later ones by the gRPC server, which answers after
    /// `response_delay`. The returned receiver fires once the first raw peer
    /// has sent its GOAWAY.
    async fn spawn(
        raw_peers: Vec<RawPeer>,
        response_delay: Duration,
    ) -> (Self, oneshot::Receiver<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("local addr");
        let connections_opened = Arc::new(AtomicUsize::new(0));
        let raw_peer_streams = Arc::new(AtomicUsize::new(0));
        let (go_away_sent, go_away_received) = oneshot::channel();
        let server = MockServer {
            response_delay,
            ..MockServer::default()
        };
        let requests_served = server.requests_served.clone();
        let deadlines_received = server.deadlines_received.clone();

        let state = (
            listener,
            raw_peers.into_iter(),
            Some(go_away_sent),
            connections_opened.clone(),
            raw_peer_streams.clone(),
        );
        let incoming = futures::stream::unfold(state, |state| async move {
            let (listener, mut raw_peers, mut go_away_sent, opened, streams) = state;
            loop {
                let socket = match listener.accept().await {
                    Ok((socket, _)) => socket,
                    Err(error) => {
                        let state = (listener, raw_peers, go_away_sent, opened, streams);
                        return Some((Err(error), state));
                    }
                };
                opened.fetch_add(1, Ordering::SeqCst);
                match raw_peers.next() {
                    Some(peer) => {
                        tokio::spawn(serve_raw_connection(
                            socket,
                            peer,
                            streams.clone(),
                            go_away_sent.take(),
                        ));
                    }
                    None => {
                        let state = (listener, raw_peers, go_away_sent, opened, streams);
                        return Some((Ok(socket), state));
                    }
                }
            }
        });

        tokio::spawn(
            tonic::transport::Server::builder()
                .add_service(LedgerServiceServer::new(server))
                .serve_with_incoming(incoming),
        );

        let fullnode = Self {
            addr,
            connections_opened,
            raw_peer_streams,
            requests_served,
            deadlines_received,
        };
        (fullnode, go_away_received)
    }

    fn client(&self) -> Client {
        Client::new(format!("http://{}", self.addr)).expect("client")
    }

    fn connections_opened(&self) -> usize {
        self.connections_opened.load(Ordering::SeqCst)
    }

    /// Request streams received by the raw peers, across all their
    /// connections.
    fn raw_peer_streams(&self) -> usize {
        self.raw_peer_streams.load(Ordering::SeqCst)
    }

    fn requests_served(&self) -> usize {
        self.requests_served.load(Ordering::SeqCst)
    }

    /// The `grpc-timeout` headers the gRPC server received, in order.
    fn deadlines_received(&self) -> Vec<Duration> {
        let deadlines = self.deadlines_received.lock().unwrap();
        deadlines
            .iter()
            .map(|value| parse_grpc_timeout(value))
            .collect()
    }
}

fn frame_header(length: u32, frame_type: u8, flags: u8, stream_id: u32) -> Vec<u8> {
    let mut header = length.to_be_bytes()[1..].to_vec();
    header.extend_from_slice(&[frame_type, flags]);
    header.extend_from_slice(&stream_id.to_be_bytes());
    header
}

fn go_away(last_stream_id: u32, error_code: u32) -> Vec<u8> {
    let mut frame = frame_header(8, FRAME_GOAWAY, 0, 0);
    frame.extend_from_slice(&last_stream_id.to_be_bytes());
    frame.extend_from_slice(&error_code.to_be_bytes());
    frame
}

/// A minimal HTTP/2 server peer: it completes the connection preface,
/// acknowledges SETTINGS and PING, counts request streams, and sends GOAWAY
/// as `peer` dictates.
async fn serve_raw_connection(
    mut socket: TcpStream,
    peer: RawPeer,
    streams: Arc<AtomicUsize>,
    mut go_away_sent: Option<oneshot::Sender<()>>,
) {
    let mut preface = [0; CLIENT_PREFACE_LEN];
    socket.read_exact(&mut preface).await.expect("preface");
    let settings = frame_header(0, FRAME_SETTINGS, 0, 0);
    socket.write_all(&settings).await.expect("settings");

    let mut sent_go_away = false;
    loop {
        let mut header = [0; FRAME_HEADER_LEN];
        if socket.read_exact(&mut header).await.is_err() {
            return;
        }
        let length = u32::from_be_bytes([0, header[0], header[1], header[2]]) as usize;
        let (frame_type, flags) = (header[3], header[4]);
        let stream_id =
            u32::from_be_bytes([header[5], header[6], header[7], header[8]]) & !(1 << 31);
        let mut payload = vec![0; length];
        if socket.read_exact(&mut payload).await.is_err() {
            return;
        }

        match frame_type {
            FRAME_SETTINGS if flags & FLAG_ACK == 0 => {
                let ack = frame_header(0, FRAME_SETTINGS, FLAG_ACK, 0);
                socket.write_all(&ack).await.expect("settings ack");
            }
            FRAME_PING if flags & FLAG_ACK == 0 => {
                let mut ack = frame_header(8, FRAME_PING, FLAG_ACK, 0);
                ack.extend_from_slice(&payload);
                socket.write_all(&ack).await.expect("ping ack");
            }
            FRAME_HEADERS => {
                streams.fetch_add(1, Ordering::SeqCst);
                if sent_go_away {
                    continue;
                }
                sent_go_away = true;
                let frame = match peer {
                    RawPeer::Refuse { error_code, delay } => {
                        tokio::time::sleep(delay).await;
                        go_away(0, error_code)
                    }
                    RawPeer::DrainWithRequestInFlight | RawPeer::CloseAfterAccepting => {
                        go_away(stream_id, NO_ERROR)
                    }
                };
                socket.write_all(&frame).await.expect("goaway");
                if let Some(sent) = go_away_sent.take() {
                    let _ = sent.send(());
                }
                if let RawPeer::CloseAfterAccepting = peer {
                    let _ = socket.shutdown().await;
                    return;
                }
            }
            _ => {}
        }
    }
}

async fn get_service_info(
    client: &mut Client,
    request: tonic::Request<proto::GetServiceInfoRequest>,
) -> Result<proto::GetServiceInfoResponse, tonic::Status> {
    let mut ledger = client.ledger_client();
    let call = ledger.get_service_info(request);
    tokio::time::timeout(CALL_TIMEOUT, call)
        .await
        .expect("call timed out")
        .map(tonic::Response::into_inner)
}

fn request() -> tonic::Request<proto::GetServiceInfoRequest> {
    tonic::Request::new(proto::GetServiceInfoRequest::default())
}

/// A request the server refused with a graceful GOAWAY is resent on a new
/// connection and succeeds there.
#[tokio::test(flavor = "multi_thread")]
async fn request_refused_by_goaway_is_resent_on_a_new_connection() {
    let (fullnode, _) = MockFullnode::spawn(vec![REFUSE], Duration::ZERO).await;
    let mut client = fullnode.client();

    let info = get_service_info(&mut client, request())
        .await
        .expect("resent request");

    assert_eq!(info.chain_id(), "mock");
    assert_eq!(fullnode.raw_peer_streams(), 1);
    assert_eq!(fullnode.connections_opened(), 2);
    assert_eq!(fullnode.requests_served(), 1);
}

/// A request issued while the connection is draining, with an earlier
/// request still in flight on it, is resent on a new connection rather than
/// failing on the draining one.
#[tokio::test(flavor = "multi_thread")]
async fn request_racing_a_drain_is_resent_on_a_new_connection() {
    let (fullnode, go_away_received) =
        MockFullnode::spawn(vec![RawPeer::DrainWithRequestInFlight], Duration::ZERO).await;
    let mut client = fullnode.client();

    let mut in_flight_client = client.clone();
    let in_flight = tokio::spawn(async move {
        in_flight_client
            .ledger_client()
            .get_service_info(proto::GetServiceInfoRequest::default())
            .await
    });
    tokio::time::timeout(CALL_TIMEOUT, go_away_received)
        .await
        .expect("goaway not sent")
        .expect("goaway sender dropped");

    let info = get_service_info(&mut client, request())
        .await
        .expect("resent request");

    assert_eq!(info.chain_id(), "mock");
    assert_eq!(fullnode.connections_opened(), 2);
    assert_eq!(fullnode.requests_served(), 1);
    in_flight.abort();
}

/// A request the server rejected unprocessed is resent only once, as in
/// gRPC's transparent retry: a second refusal reaches the caller.
#[tokio::test(flavor = "multi_thread")]
async fn refused_request_is_resent_only_once() {
    let (fullnode, _) = MockFullnode::spawn(vec![REFUSE, REFUSE], Duration::ZERO).await;
    let mut client = fullnode.client();

    get_service_info(&mut client, request())
        .await
        .expect_err("second refusal");

    assert_eq!(fullnode.raw_peer_streams(), 2);
    assert_eq!(fullnode.connections_opened(), 2);
    assert_eq!(fullnode.requests_served(), 0);
}

/// A GOAWAY carrying an error code may report a wrong last stream id, so a
/// request it fails may have been processed and is not resent.
#[tokio::test(flavor = "multi_thread")]
async fn request_failed_by_goaway_with_an_error_code_is_not_resent() {
    let peer = RawPeer::Refuse {
        error_code: ENHANCE_YOUR_CALM,
        delay: Duration::ZERO,
    };
    let (fullnode, _) = MockFullnode::spawn(vec![peer], Duration::ZERO).await;
    let mut client = fullnode.client();

    get_service_info(&mut client, request())
        .await
        .expect_err("request failed by an error GOAWAY");

    assert_eq!(fullnode.raw_peer_streams(), 1);
    assert_eq!(fullnode.connections_opened(), 1);
    assert_eq!(fullnode.requests_served(), 0);
}

/// A request the server accepted before the connection closed may have been
/// processed, so it fails rather than being resent.
#[tokio::test(flavor = "multi_thread")]
async fn request_accepted_before_connection_closed_is_not_resent() {
    let (fullnode, _) =
        MockFullnode::spawn(vec![RawPeer::CloseAfterAccepting], Duration::ZERO).await;
    let mut client = fullnode.client();

    get_service_info(&mut client, request())
        .await
        .expect_err("request on a closed connection");

    assert_eq!(fullnode.raw_peer_streams(), 1);
    assert_eq!(fullnode.connections_opened(), 1);
    assert_eq!(fullnode.requests_served(), 0);
}

/// The per-call deadline covers all attempts: a resend tells the server, and
/// enforces locally, only the time the refused attempt left.
#[tokio::test(flavor = "multi_thread")]
async fn deadline_covers_all_attempts() {
    let deadline = Duration::from_secs(1);
    let refused_after = Duration::from_millis(800);
    let peer = RawPeer::Refuse {
        error_code: NO_ERROR,
        delay: refused_after,
    };
    let (fullnode, _) = MockFullnode::spawn(vec![peer], Duration::from_secs(5)).await;
    let mut client = fullnode.client();

    let mut request = request();
    request.set_timeout(deadline);
    let start = Instant::now();
    get_service_info(&mut client, request)
        .await
        .expect_err("slow response past the deadline");
    let elapsed = start.elapsed();

    // The resend reached the server carrying the remaining time, not a fresh
    // deadline.
    let deadlines = fullnode.deadlines_received();
    assert_eq!(deadlines.len(), 1);
    assert!(
        deadlines[0] <= deadline - refused_after,
        "resend carried {:?} of a {deadline:?} deadline after {refused_after:?}",
        deadlines[0]
    );
    // Locally the call ends at the original deadline; a fresh deadline for
    // the resend would end it near 1.8s.
    assert!(
        elapsed < deadline + Duration::from_millis(400),
        "call took {elapsed:?} against a {deadline:?} deadline"
    );
}

/// Parse a `grpc-timeout` header value (digits followed by a unit).
fn parse_grpc_timeout(value: &str) -> Duration {
    let (magnitude, unit) = value.split_at(value.len() - 1);
    let magnitude: u64 = magnitude.parse().expect("grpc-timeout magnitude");
    match unit {
        "H" => Duration::from_secs(magnitude * 60 * 60),
        "M" => Duration::from_secs(magnitude * 60),
        "S" => Duration::from_secs(magnitude),
        "m" => Duration::from_millis(magnitude),
        "u" => Duration::from_micros(magnitude),
        "n" => Duration::from_nanos(magnitude),
        _ => panic!("grpc-timeout unit {unit:?}"),
    }
}
