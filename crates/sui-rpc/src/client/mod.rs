use std::sync::Arc;
use std::time::Duration;
use tap::Pipe;
use tonic::body::Body;
use tonic::codec::CompressionEncoding;
use tonic::transport::channel::ClientTlsConfig;
use tower::Layer;
use tower::Service;
use tower::ServiceBuilder;
use tower::util::BoxLayer;
use tower::util::BoxService;

mod response_ext;
pub use response_ext::ResponseExt;

mod interceptors;
pub use interceptors::HeadersInterceptor;

mod watchdog;
pub use watchdog::BodyIdleTimeout;
use watchdog::DEFAULT_BODY_IDLE_TIMEOUT;
use watchdog::WatchdogLayer;

mod staking_rewards;
pub use staking_rewards::DelegatedStake;

mod coin_selection;
mod lists;

mod signature_verification;
pub use signature_verification::VerifySignatureError;

mod transaction_execution;
pub use transaction_execution::ExecuteAndWaitError;

use crate::proto::sui::rpc::v2::ledger_service_client::LedgerServiceClient;
use crate::proto::sui::rpc::v2::move_package_service_client::MovePackageServiceClient;
use crate::proto::sui::rpc::v2::signature_verification_service_client::SignatureVerificationServiceClient;
use crate::proto::sui::rpc::v2::state_service_client::StateServiceClient;
use crate::proto::sui::rpc::v2::subscription_service_client::SubscriptionServiceClient;
use crate::proto::sui::rpc::v2::transaction_execution_service_client::TransactionExecutionServiceClient;
#[cfg(feature = "unstable")]
use crate::proto::sui::rpc::v2alpha::proof_service_client::ProofServiceClient;

type Result<T, E = tonic::Status> = std::result::Result<T, E>;
type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxedChannel = BoxService<http::Request<Body>, http::Response<Body>, tonic::Status>;

type RequestLayer = BoxLayer<
    BoxService<http::Request<Body>, http::Response<Body>, BoxError>,
    http::Request<Body>,
    http::Response<Body>,
    BoxError,
>;

const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_TCP_KEEPALIVE_IDLE: Duration = Duration::from_secs(15);
const DEFAULT_TCP_KEEPALIVE_INTERVAL: Duration = Duration::from_secs(5);
const DEFAULT_TCP_KEEPALIVE_RETRIES: u32 = 3;
const DEFAULT_HTTP2_KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(5);
const DEFAULT_HTTP2_KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(20);

// All RPCs made through a `Client` (and all of its clones) are multiplexed
// over a single HTTP/2 connection, so the connection-level receive window is
// shared by every in-flight response. A streaming response that the
// application holds without polling pins up to a full stream window of that
// shared budget; once the connection window is exhausted, every RPC on the
// channel hangs indefinitely while TCP and HTTP/2 keepalives stay healthy.
// hyper's client defaults (2 MiB stream / 5 MiB connection) let ~3 stalled
// streams starve the connection. Keep the stream window at hyper's default
// but raise the connection window so ~32 concurrently stalled streams are
// needed instead.
const DEFAULT_HTTP2_STREAM_WINDOW_SIZE: u32 = 2 * 1024 * 1024;
const DEFAULT_HTTP2_CONNECTION_WINDOW_SIZE: u32 = 64 * 1024 * 1024;

/// A gRPC client for the Sui fullnode RPC interface.
///
/// All RPCs made through a client and its clones are multiplexed over a
/// single HTTP/2 connection.
///
/// # Timeouts and deadlines
///
/// No default bounds the total duration of a call. Two opt-in bounds are
/// available:
///
/// - A per-call deadline set with [`tonic::Request::set_timeout`]. This
///   attaches the standard `grpc-timeout` header, so a server that supports
///   it enforces the deadline too, and the client enforces it locally end to
///   end: tonic bounds the wait for response headers, and the client's
///   watchdog (see [`with_body_idle_timeout`](Client::with_body_idle_timeout))
///   bounds the response body against the same deadline.
/// - A client-wide response-headers timeout set with
///   [`with_response_headers_timeout`](Client::with_response_headers_timeout).
///   This is enforced locally only and its timer stops once response headers
///   arrive, so it bounds every unary call (whose headers are not sent until
///   the handler completes) without cutting off long-lived streams.
///
/// Independent of any deadline, the watchdog resets RPCs whose response body
/// makes no progress for 30 seconds (configurable), so a call on a stalled
/// connection fails with `DeadlineExceeded` instead of hanging forever.
#[derive(Clone)]
pub struct Client {
    channel: tonic::transport::Channel,

    // Everything other than the channel is only consulted when building a
    // per-service client or reconfiguring, so it lives behind an `Arc` to
    // keep `Client` itself small; it is cloned by value into futures
    // throughout the SDK. The `Endpoint` alone is over 500 bytes.
    config: Arc<ClientConfig>,
}

#[derive(Clone)]
struct ClientConfig {
    uri: http::Uri,
    endpoint: tonic::transport::Endpoint,
    headers: HeadersInterceptor,
    max_decoding_message_size: Option<usize>,
    body_idle_timeout: Option<Duration>,

    /// Layer to apply to all RPC requests
    request_layer: Option<RequestLayer>,
}

impl Client {
    /// URL for the public-good, Sui Foundation provided fullnodes for mainnet.
    pub const MAINNET_FULLNODE: &str = "https://fullnode.mainnet.sui.io";

    /// URL for the public-good, Sui Foundation provided fullnodes for testnet.
    pub const TESTNET_FULLNODE: &str = "https://fullnode.testnet.sui.io";

    /// URL for the public-good, Sui Foundation provided fullnodes for devnet.
    pub const DEVNET_FULLNODE: &str = "https://fullnode.devnet.sui.io";

    /// URL for the public-good, Sui Foundation provided archive for mainnet.
    pub const MAINNET_ARCHIVE: &str = "https://archive.mainnet.sui.io";

    /// URL for the public-good, Sui Foundation provided archive for testnet.
    pub const TESTNET_ARCHIVE: &str = "https://archive.testnet.sui.io";

    /// Build a client from a fully custom [`tonic::transport::Endpoint`].
    ///
    /// This bypasses every transport default that [`Client::new`] applies,
    /// including the HTTP/2 flow-control windows that protect the shared
    /// connection from starvation by stalled streaming responses. Prefer
    /// [`Client::new`] plus the `with_*` configuration methods unless an
    /// endpoint setting is needed that the client does not expose. The
    /// idle-body watchdog (see [`Client::with_body_idle_timeout`]) is part of
    /// the client rather than the endpoint and stays enabled.
    ///
    /// In particular, do not rely on
    /// [`http2_adaptive_window`](tonic::transport::Endpoint::http2_adaptive_window)
    /// as a substitute for large static windows: with adaptive windowing,
    /// hyper starts the connection window at the 64 KiB HTTP/2 spec default
    /// until bandwidth-delay probing ramps up, so a single stalled stream can
    /// starve the whole connection.
    pub fn from_endpoint(endpoint: &tonic::transport::Endpoint) -> Self {
        let uri = endpoint.uri().clone();
        let channel = endpoint.connect_lazy();
        Self {
            channel,
            config: Arc::new(ClientConfig {
                uri,
                endpoint: endpoint.clone(),
                headers: Default::default(),
                max_decoding_message_size: None,
                body_idle_timeout: Some(DEFAULT_BODY_IDLE_TIMEOUT),
                request_layer: None,
            }),
        }
    }

    #[allow(clippy::result_large_err)]
    pub fn new<T>(uri: T) -> Result<Self>
    where
        T: TryInto<http::Uri>,
        T::Error: Into<BoxError>,
    {
        let uri = uri
            .try_into()
            .map_err(Into::into)
            .map_err(status_from_error)?;
        let mut endpoint = tonic::transport::Endpoint::from(uri.clone());
        if uri.scheme() == Some(&http::uri::Scheme::HTTPS) {
            endpoint = endpoint
                .tls_config(ClientTlsConfig::new().with_enabled_roots())
                .map_err(Into::into)
                .map_err(status_from_error)?;
        }

        let endpoint = endpoint
            .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
            .tcp_keepalive(Some(DEFAULT_TCP_KEEPALIVE_IDLE))
            .tcp_keepalive_interval(Some(DEFAULT_TCP_KEEPALIVE_INTERVAL))
            .tcp_keepalive_retries(Some(DEFAULT_TCP_KEEPALIVE_RETRIES))
            .http2_keep_alive_interval(DEFAULT_HTTP2_KEEP_ALIVE_INTERVAL)
            .keep_alive_timeout(DEFAULT_HTTP2_KEEP_ALIVE_TIMEOUT)
            .initial_stream_window_size(DEFAULT_HTTP2_STREAM_WINDOW_SIZE)
            .initial_connection_window_size(DEFAULT_HTTP2_CONNECTION_WINDOW_SIZE);
        let channel = endpoint.connect_lazy();

        Ok(Self {
            channel,
            config: Arc::new(ClientConfig {
                uri,
                endpoint,
                headers: Default::default(),
                max_decoding_message_size: None,
                body_idle_timeout: Some(DEFAULT_BODY_IDLE_TIMEOUT),
                request_layer: None,
            }),
        })
    }

    /// Set the idle timeout for the client's response-body watchdog.
    /// Defaults to 30 seconds.
    ///
    /// The watchdog bounds the time between response-body progress events: if
    /// a whole idle period passes without a frame of the response being
    /// delivered to the caller -- because the connection is starved or dead,
    /// or because the caller has parked a streaming response without polling
    /// it -- the watchdog resets the stream, releasing the HTTP/2
    /// flow-control window it had pinned, and the call observes a
    /// [`DeadlineExceeded`](tonic::Code::DeadlineExceeded) status on its next
    /// poll. This is what turns "an RPC on a starved connection hangs
    /// forever" into a bounded failure, and what keeps an abandoned stream
    /// from starving the shared connection in the first place.
    ///
    /// Streams that are legitimately quiet for longer than the timeout (the
    /// fullnode's checkpoint subscription is not: it emits watermarks every
    /// few seconds) should raise or disable the watchdog for that call with a
    /// [`BodyIdleTimeout`] request extension.
    pub fn with_body_idle_timeout(mut self, timeout: Duration) -> Self {
        Arc::make_mut(&mut self.config).body_idle_timeout = Some(timeout);
        self
    }

    /// Disable the client's response-body watchdog (see
    /// [`with_body_idle_timeout`](Self::with_body_idle_timeout)).
    ///
    /// Without it, an RPC whose response can no longer make progress hangs
    /// indefinitely; only disable the watchdog when every call is bounded by
    /// the caller. It can be re-enabled for individual requests with a
    /// [`BodyIdleTimeout`] request extension.
    pub fn without_body_idle_timeout(mut self) -> Self {
        Arc::make_mut(&mut self.config).body_idle_timeout = None;
        self
    }

    /// Set a timeout for the response-headers phase of every RPC made
    /// through this client. Disabled by default.
    ///
    /// The timer covers a request from dispatch on the connection until
    /// response headers arrive and is dropped once they do, so a client-wide
    /// value does not cut off long-lived streaming responses. Because a
    /// server does not send response headers for a unary call until the
    /// handler completes, this effectively bounds the total duration of
    /// unary calls; the body that follows is bounded by the idle-body
    /// watchdog (see [`with_body_idle_timeout`](Self::with_body_idle_timeout))
    /// and, when set, the per-call deadline. Connection establishment is
    /// bounded separately by the connect timeout.
    ///
    /// This timeout is enforced locally only; it is not communicated to the
    /// server. When a per-call deadline ([`tonic::Request::set_timeout`]) is
    /// also set, the shorter of the two bounds the headers phase locally, so
    /// a per-call deadline can tighten this bound but never extend it --
    /// size the timeout for the slowest expected RPC. Expiry surfaces as
    /// [`DeadlineExceeded`](tonic::Code::DeadlineExceeded).
    ///
    /// This rebuilds the underlying channel, so it must be called before the
    /// client is used or cloned; earlier clones keep the previous
    /// configuration.
    pub fn with_response_headers_timeout(mut self, timeout: Duration) -> Self {
        let config = Arc::make_mut(&mut self.config);
        config.endpoint = config.endpoint.clone().timeout(timeout);
        self.channel = config.endpoint.connect_lazy();
        self
    }

    /// Set the HTTP/2 per-stream receive window, in bytes.
    ///
    /// This bounds how much unread response data a single RPC can buffer
    /// before the server must stop sending on that stream. It also bounds how
    /// much of the shared connection window (see
    /// [`with_initial_connection_window_size`](Self::with_initial_connection_window_size))
    /// one stalled stream can pin. Defaults to 2 MiB.
    ///
    /// This rebuilds the underlying channel, so it must be called before the
    /// client is used or cloned; earlier clones keep the previous
    /// configuration.
    pub fn with_initial_stream_window_size(mut self, size: u32) -> Self {
        let config = Arc::make_mut(&mut self.config);
        config.endpoint = config.endpoint.clone().initial_stream_window_size(size);
        self.channel = config.endpoint.connect_lazy();
        self
    }

    /// Set the HTTP/2 connection-level receive window, in bytes.
    ///
    /// This window is shared by every RPC multiplexed over the client's
    /// single HTTP/2 connection, including all clones of the client. Response
    /// data that the application has not yet read counts against it, so it
    /// determines how many concurrently stalled streaming responses it takes
    /// to starve the connection and hang every other RPC on it. Defaults to
    /// 64 MiB (~32 stalled streams at the default 2 MiB stream window).
    ///
    /// This rebuilds the underlying channel, so it must be called before the
    /// client is used or cloned; earlier clones keep the previous
    /// configuration.
    pub fn with_initial_connection_window_size(mut self, size: u32) -> Self {
        let config = Arc::make_mut(&mut self.config);
        config.endpoint = config.endpoint.clone().initial_connection_window_size(size);
        self.channel = config.endpoint.connect_lazy();
        self
    }

    pub fn with_headers(mut self, headers: HeadersInterceptor) -> Self {
        Arc::make_mut(&mut self.config).headers = headers;
        self
    }

    /// Provide an optional [`Layer`] that will be used to wrap all RPC
    /// requests.
    ///
    /// This could be helpful in providing global metrics and logging
    /// for all outbound requests.
    ///
    /// The layer's service may return any response body that implements
    /// [`http_body::Body<Data = bytes::Bytes>`] and any error type that
    /// implements `Into<Box<dyn Error + Send + Sync>>`. Both are mapped
    /// to the internal types automatically.
    ///
    /// # Example
    ///
    /// Add a layer that logs each request URI:
    ///
    /// ```
    /// # let _rt = tokio::runtime::Builder::new_current_thread()
    /// #     .build()
    /// #     .unwrap();
    /// # let _guard = _rt.enter();
    /// use sui_rpc::Client;
    /// use tower::ServiceBuilder;
    ///
    /// let client = Client::new(Client::MAINNET_FULLNODE)
    ///     .unwrap()
    ///     .request_layer(ServiceBuilder::new().map_request(|req: http::Request<_>| {
    ///         println!("request to {}", req.uri());
    ///         req
    ///     }));
    /// ```
    pub fn request_layer<L, ResBody, E>(mut self, layer: L) -> Self
    where
        L: Layer<BoxService<http::Request<Body>, http::Response<Body>, BoxError>>
            + Send
            + Sync
            + 'static,
        L::Service: Service<http::Request<Body>, Response = http::Response<ResBody>, Error = E>
            + Send
            + 'static,
        <L::Service as Service<http::Request<Body>>>::Future: Send + 'static,
        ResBody: http_body::Body<Data = bytes::Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError>,
        E: Into<BoxError> + Send + 'static,
    {
        let layer = BoxLayer::new(
            ServiceBuilder::new()
                .map_response(|resp: http::Response<ResBody>| resp.map(Body::new))
                .map_err(Into::<BoxError>::into)
                .layer(layer),
        );
        Arc::make_mut(&mut self.config).request_layer = Some(layer);
        self
    }

    pub fn with_max_decoding_message_size(mut self, limit: usize) -> Self {
        Arc::make_mut(&mut self.config).max_decoding_message_size = Some(limit);
        self
    }

    pub fn uri(&self) -> &http::Uri {
        &self.config.uri
    }

    fn channel(&self) -> BoxedChannel {
        let headers = self.config.headers.clone();

        // Build the base service with headers applied at the HTTP level and the
        // transport error mapped to BoxError for compatibility with user layers.
        let base = BoxService::new(
            ServiceBuilder::new()
                .map_err(|e: tonic::transport::Error| -> BoxError { Box::new(e) })
                .map_request(move |mut req: http::Request<Body>| {
                    if !headers.headers().is_empty() {
                        req.headers_mut()
                            .extend(headers.headers().clone().into_headers());
                    }
                    req
                })
                .service(self.channel.clone()),
        );

        // Guard every response body with the idle-body watchdog, beneath any
        // user layers so their view of the response goes through the
        // watchdog's bridge.
        let base = BoxService::new(WatchdogLayer::new(self.config.body_idle_timeout).layer(base));

        // Apply the user's outbound request layer if present.
        let layered = if let Some(layer) = &self.config.request_layer {
            layer.layer(base)
        } else {
            base
        };

        // Map the final error to tonic::Status (a concrete type) so that
        // downstream users of the tonic-generated clients don't run into
        // lifetime-inference issues with async_trait and Box<dyn Error>.
        BoxService::new(
            ServiceBuilder::new()
                .map_err(status_from_error)
                .service(layered),
        )
    }

    pub fn ledger_client(&mut self) -> LedgerServiceClient<BoxedChannel> {
        LedgerServiceClient::new(self.channel())
            .accept_compressed(CompressionEncoding::Zstd)
            .pipe(|client| {
                if let Some(limit) = self.config.max_decoding_message_size {
                    client.max_decoding_message_size(limit)
                } else {
                    client
                }
            })
    }

    pub fn state_client(&mut self) -> StateServiceClient<BoxedChannel> {
        StateServiceClient::new(self.channel())
            .accept_compressed(CompressionEncoding::Zstd)
            .pipe(|client| {
                if let Some(limit) = self.config.max_decoding_message_size {
                    client.max_decoding_message_size(limit)
                } else {
                    client
                }
            })
    }

    pub fn execution_client(&mut self) -> TransactionExecutionServiceClient<BoxedChannel> {
        TransactionExecutionServiceClient::new(self.channel())
            .accept_compressed(CompressionEncoding::Zstd)
            .pipe(|client| {
                if let Some(limit) = self.config.max_decoding_message_size {
                    client.max_decoding_message_size(limit)
                } else {
                    client
                }
            })
    }

    pub fn package_client(&mut self) -> MovePackageServiceClient<BoxedChannel> {
        MovePackageServiceClient::new(self.channel())
            .accept_compressed(CompressionEncoding::Zstd)
            .pipe(|client| {
                if let Some(limit) = self.config.max_decoding_message_size {
                    client.max_decoding_message_size(limit)
                } else {
                    client
                }
            })
    }

    pub fn signature_verification_client(
        &mut self,
    ) -> SignatureVerificationServiceClient<BoxedChannel> {
        SignatureVerificationServiceClient::new(self.channel())
            .accept_compressed(CompressionEncoding::Zstd)
            .pipe(|client| {
                if let Some(limit) = self.config.max_decoding_message_size {
                    client.max_decoding_message_size(limit)
                } else {
                    client
                }
            })
    }

    pub fn subscription_client(&mut self) -> SubscriptionServiceClient<BoxedChannel> {
        SubscriptionServiceClient::new(self.channel())
            .accept_compressed(CompressionEncoding::Zstd)
            .pipe(|client| {
                if let Some(limit) = self.config.max_decoding_message_size {
                    client.max_decoding_message_size(limit)
                } else {
                    client
                }
            })
    }

    /// Returns a client for the unstable alpha `ProofService`, which serves
    /// Object Checkpoint State (OCS) inclusion proofs.
    #[cfg(feature = "unstable")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "unstable")))]
    pub fn proof_client(&mut self) -> ProofServiceClient<BoxedChannel> {
        ProofServiceClient::new(self.channel())
            .accept_compressed(CompressionEncoding::Zstd)
            .pipe(|client| {
                if let Some(limit) = self.config.max_decoding_message_size {
                    client.max_decoding_message_size(limit)
                } else {
                    client
                }
            })
    }
}

/// Map a transport error to a [`tonic::Status`].
///
/// tonic surfaces an expired headers-phase timeout (the client's
/// response-headers timeout, or a `grpc-timeout` deadline expiring before
/// response headers arrive) as `Cancelled`. The gRPC code for an expired
/// deadline is `DeadlineExceeded`, and the watchdog already uses it for
/// body-phase expiry, so normalize before delegating to tonic's own mapping.
fn status_from_error(error: BoxError) -> tonic::Status {
    let mut source: Option<&(dyn std::error::Error + 'static)> = Some(error.as_ref());
    while let Some(err) = source {
        // An embedded `Status` takes precedence, as in tonic's own mapping.
        if err.is::<tonic::Status>() {
            break;
        }
        if err.is::<tonic::TimeoutExpired>() {
            return tonic::Status::deadline_exceeded(
                "timeout expired before response headers were received",
            );
        }
        source = err.source();
    }
    tonic::Status::from_error(error)
}
