//! The client over a custom transport and over its built-in one, on every target: natively on
//! tokio, and on wasm32 with `wasm-pack test --node`.
//!
//! Offline tests drive the client over a scripted transport with [`Client::from_service`], so the
//! client's own layers run exactly as they do over the built-in transport: headers, the
//! response-headers timeout, the idle-body watchdog (a spawned task: tokio's natively, the host
//! event loop's on wasm32), and helpers that make several RPCs. The `#[ignore]`d tests use the
//! built-in transport (HTTP/2 natively, gRPC-web on wasm32) against a public fullnode
//! (`-- --include-ignored`).

use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;

use bytes::Buf;
use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use futures::StreamExt;
use futures::stream;
use futures::stream::BoxStream;
use http_body::Frame;
use http_body_util::BodyExt;
use http_body_util::StreamBody;
use prost::Message;
use sui_rpc::Client;
use sui_rpc::client::HeadersInterceptor;
use sui_rpc::field::FieldMask;
use sui_rpc::field::FieldMaskUtil;
use sui_rpc::proto::sui::rpc::v2::GetServiceInfoRequest;
use sui_rpc::proto::sui::rpc::v2::GetServiceInfoResponse;
use sui_rpc::proto::sui::rpc::v2::ListOwnedObjectsRequest;
use sui_rpc::proto::sui::rpc::v2::ListOwnedObjectsResponse;
use sui_rpc::proto::sui::rpc::v2::Object;
use sui_rpc::proto::sui::rpc::v2::SubscribeCheckpointsRequest;
use tonic::Code;
use tonic::body::Body;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;

const ORIGIN: &str = "https://fullnode.example";
const MAINNET_CHAIN_ID: &str = "4btiuiMPvEENsttpZC7CZ53DruC3MAgfznDbASZ7DR6S";

type ReplyBody = StreamBody<BoxStream<'static, Result<Frame<Bytes>, Infallible>>>;

/// How the scripted fullnode answers one RPC.
enum Reply {
    /// Response headers, one message, and an OK status.
    Message(Vec<u8>),
    /// Response headers and a body that never makes progress.
    StallBody,
    /// No response headers at all.
    StallHeaders,
}

/// A request as the transport received it.
struct Seen {
    uri: String,
    headers: http::HeaderMap,
}

#[cfg(not(target_arch = "wasm32"))]
type ScriptFn = dyn Fn(&str, Bytes) -> Reply + Send + Sync;
#[cfg(target_arch = "wasm32")]
type ScriptFn = dyn Fn(&str, Bytes) -> Reply;

#[cfg(not(target_arch = "wasm32"))]
type ReplyFuture =
    Pin<Box<dyn Future<Output = Result<http::Response<ReplyBody>, Infallible>> + Send>>;
#[cfg(target_arch = "wasm32")]
type ReplyFuture = Pin<Box<dyn Future<Output = Result<http::Response<ReplyBody>, Infallible>>>>;

/// A transport that answers each RPC with a scripted [`Reply`] for its path and request message.
#[derive(Clone)]
struct ScriptedFullnode {
    reply: Arc<ScriptFn>,
    seen: Arc<Mutex<Vec<Seen>>>,
}

impl ScriptedFullnode {
    #[cfg(not(target_arch = "wasm32"))]
    fn new(reply: impl Fn(&str, Bytes) -> Reply + Send + Sync + 'static) -> Self {
        Self {
            reply: Arc::new(reply),
            seen: Default::default(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn new(reply: impl Fn(&str, Bytes) -> Reply + 'static) -> Self {
        Self {
            reply: Arc::new(reply),
            seen: Default::default(),
        }
    }

    fn client(&self) -> Client {
        Client::from_service(ORIGIN.parse().unwrap(), self.clone()).unwrap()
    }
}

impl tower::Service<http::Request<Body>> for ScriptedFullnode {
    type Response = http::Response<ReplyBody>;
    type Error = Infallible;
    type Future = ReplyFuture;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: http::Request<Body>) -> Self::Future {
        let this = self.clone();
        Box::pin(async move {
            let (parts, body) = request.into_parts();
            this.seen.lock().unwrap().push(Seen {
                uri: parts.uri.to_string(),
                headers: parts.headers.clone(),
            });
            let mut framed = body.collect().await.unwrap().to_bytes();
            framed.advance(5); // gRPC message prefix: compression flag and length
            let reply = (this.reply)(parts.uri.path(), framed);
            let frames: BoxStream<'static, _> = match reply {
                Reply::Message(message) => {
                    let mut data = BytesMut::with_capacity(5 + message.len());
                    data.put_u8(0);
                    data.put_u32(message.len() as u32);
                    data.put_slice(&message);
                    let mut trailers = http::HeaderMap::new();
                    trailers.insert("grpc-status", "0".parse().unwrap());
                    stream::iter([
                        Ok(Frame::data(data.freeze())),
                        Ok(Frame::trailers(trailers)),
                    ])
                    .boxed()
                }
                Reply::StallBody => stream::pending().boxed(),
                Reply::StallHeaders => return std::future::pending().await,
            };
            Ok(http::Response::builder()
                .header("content-type", "application/grpc")
                .body(StreamBody::new(frames))
                .unwrap())
        })
    }
}

fn service_info(chain_id: &str) -> Vec<u8> {
    let mut response = GetServiceInfoResponse::default();
    response.chain_id = Some(chain_id.to_owned());
    response.encode_to_vec()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn unary_call_over_a_custom_transport() {
    let fullnode = ScriptedFullnode::new(|_, _| Reply::Message(service_info("scripted")));
    let mut headers = HeadersInterceptor::new();
    headers.headers_mut().insert("x-test", "1".parse().unwrap());
    let mut client = fullnode.client().with_headers(headers).request_layer(
        tower::ServiceBuilder::new().map_request(|mut request: http::Request<Body>| {
            request
                .headers_mut()
                .insert("x-layer", "1".parse().unwrap());
            request
        }),
    );

    let info = client
        .ledger_client()
        .get_service_info(GetServiceInfoRequest::default())
        .await
        .unwrap()
        .into_inner();
    assert_eq!(info.chain_id(), "scripted");

    let seen = fullnode.seen.lock().unwrap();
    let [request] = seen.as_slice() else {
        panic!("expected one request, got {}", seen.len());
    };
    // Custom transports receive absolute URIs, and every client layer has run.
    assert_eq!(
        request.uri,
        format!("{ORIGIN}/sui.rpc.v2.LedgerService/GetServiceInfo")
    );
    assert!(
        request
            .headers
            .contains_key(sui_rpc::headers::X_SUI_CLIENT_PROTOCOL_VERSION)
    );
    assert_eq!(request.headers["x-test"], "1");
    assert_eq!(request.headers["x-layer"], "1");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn only_the_scheme_and_authority_of_the_uri_are_used() {
    // As with the built-in channel, a path in the URI is not part of request URIs.
    let fullnode = ScriptedFullnode::new(|_, _| Reply::Message(service_info("scripted")));
    let mut client = Client::from_service(
        format!("{ORIGIN}/prefix/").parse().unwrap(),
        fullnode.clone(),
    )
    .unwrap();
    client
        .ledger_client()
        .get_service_info(GetServiceInfoRequest::default())
        .await
        .unwrap();
    assert_eq!(
        fullnode.seen.lock().unwrap()[0].uri,
        format!("{ORIGIN}/sui.rpc.v2.LedgerService/GetServiceInfo")
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn a_uri_without_scheme_or_authority_is_rejected() {
    for uri in ["fullnode.example:9000", "/sui.rpc.v2.LedgerService"] {
        let fullnode = ScriptedFullnode::new(|_, _| Reply::Message(service_info("unused")));
        let error = Client::from_service(uri.parse().unwrap(), fullnode)
            .err()
            .unwrap_or_else(|| panic!("{uri} was accepted"));
        assert_eq!(error.code(), Code::InvalidArgument, "{uri}: {error}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn an_unreachable_fullnode_is_unavailable() {
    // Nothing listens on the discard port; both built-in transports report `Unavailable`.
    let mut client = Client::new("http://127.0.0.1:9").unwrap();
    let status = client
        .ledger_client()
        .get_service_info(GetServiceInfoRequest::default())
        .await
        .unwrap_err();
    assert_eq!(status.code(), Code::Unavailable, "{status}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn paginating_helper_follows_page_tokens() {
    let fullnode = ScriptedFullnode::new(|path, request| {
        assert_eq!(path, "/sui.rpc.v2.StateService/ListOwnedObjects");
        let request = ListOwnedObjectsRequest::decode(request).unwrap();
        let mut response = ListOwnedObjectsResponse::default();
        let mut object = Object::default();
        match request.page_token.as_deref() {
            None => {
                object.object_id = Some("first".to_owned());
                response.next_page_token = Some(Bytes::from_static(b"page-2"));
            }
            Some(b"page-2") => object.object_id = Some("second".to_owned()),
            Some(other) => panic!("unexpected page token {other:?}"),
        }
        response.objects = vec![object];
        Reply::Message(response.encode_to_vec())
    });

    let objects: Vec<String> = fullnode
        .client()
        .list_owned_objects(ListOwnedObjectsRequest::default())
        .map(|object| object.unwrap().object_id().to_owned())
        .collect()
        .await;
    assert_eq!(objects, ["first", "second"]);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn idle_body_watchdog_cuts_a_stalled_response() {
    let fullnode = ScriptedFullnode::new(|_, _| Reply::StallBody);
    let mut client = fullnode
        .client()
        .with_body_idle_timeout(Duration::from_millis(50));

    let status = client
        .ledger_client()
        .get_service_info(GetServiceInfoRequest::default())
        .await
        .unwrap_err();
    assert_eq!(status.code(), Code::DeadlineExceeded, "{status}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn response_headers_timeout_bounds_a_custom_transport() {
    let fullnode = ScriptedFullnode::new(|_, _| Reply::StallHeaders);
    let mut client = fullnode
        .client()
        .with_response_headers_timeout(Duration::from_millis(50));

    let status = client
        .ledger_client()
        .get_service_info(GetServiceInfoRequest::default())
        .await
        .unwrap_err();
    assert_eq!(status.code(), Code::DeadlineExceeded, "{status}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn per_call_deadline_bounds_response_headers() {
    let fullnode = ScriptedFullnode::new(|_, _| Reply::StallHeaders);
    let mut request = tonic::Request::new(GetServiceInfoRequest::default());
    request.set_timeout(Duration::from_millis(50));

    let status = fullnode
        .client()
        .ledger_client()
        .get_service_info(request)
        .await
        .unwrap_err();
    assert_eq!(status.code(), Code::DeadlineExceeded, "{status}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[ignore = "needs network access to fullnode.mainnet.sui.io"]
async fn built_in_transport_unary() {
    let mut client = Client::new(Client::MAINNET_FULLNODE).unwrap();
    let info = client
        .ledger_client()
        .get_service_info(GetServiceInfoRequest::default())
        .await
        .unwrap()
        .into_inner();
    assert_eq!(info.chain_id(), MAINNET_CHAIN_ID);
    assert!(client.get_reference_gas_price().await.unwrap() > 0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[ignore = "needs network access to fullnode.mainnet.sui.io"]
async fn built_in_transport_server_streaming() {
    let mut client = Client::new(Client::MAINNET_FULLNODE).unwrap();
    let mut request = SubscribeCheckpointsRequest::default();
    request.read_mask = Some(FieldMask::from_paths(["sequence_number"]));
    let mut stream = client
        .subscription_client()
        .subscribe_checkpoints(request)
        .await
        .unwrap()
        .into_inner();

    let mut sequence_numbers = Vec::new();
    while sequence_numbers.len() < 2 {
        let response = stream.next().await.unwrap().unwrap();
        if let Some(checkpoint) = response.checkpoint {
            sequence_numbers.push(checkpoint.sequence_number());
        }
    }
    assert!(
        sequence_numbers[1] > sequence_numbers[0],
        "{sequence_numbers:?}"
    );
}
