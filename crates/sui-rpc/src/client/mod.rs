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

mod staking_rewards;
pub use staking_rewards::DelegatedStake;

mod coin_selection;
mod lists;

mod transaction_execution;
pub use transaction_execution::ExecuteAndWaitError;

use crate::proto::sui::rpc::v2::ledger_service_client::LedgerServiceClient;
use crate::proto::sui::rpc::v2::move_package_service_client::MovePackageServiceClient;
use crate::proto::sui::rpc::v2::signature_verification_service_client::SignatureVerificationServiceClient;
use crate::proto::sui::rpc::v2::state_service_client::StateServiceClient;
use crate::proto::sui::rpc::v2::subscription_service_client::SubscriptionServiceClient;
use crate::proto::sui::rpc::v2::transaction_execution_service_client::TransactionExecutionServiceClient;

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

#[derive(Clone)]
pub struct Client {
    uri: http::Uri,
    channel: tonic::transport::Channel,
    headers: HeadersInterceptor,
    max_decoding_message_size: Option<usize>,

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

    pub fn from_endpoint(endpoint: &tonic::transport::Endpoint) -> Self {
        let uri = endpoint.uri().clone();
        let channel = endpoint.connect_lazy();
        Self {
            uri,
            channel,
            headers: Default::default(),
            max_decoding_message_size: None,
            request_layer: None,
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
            .map_err(tonic::Status::from_error)?;
        let mut endpoint = tonic::transport::Endpoint::from(uri.clone());
        if uri.scheme() == Some(&http::uri::Scheme::HTTPS) {
            endpoint = endpoint
                .tls_config(ClientTlsConfig::new().with_enabled_roots())
                .map_err(Into::into)
                .map_err(tonic::Status::from_error)?;
        }

        let channel = endpoint
            .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
            .tcp_keepalive(Some(DEFAULT_TCP_KEEPALIVE_IDLE))
            .tcp_keepalive_interval(Some(DEFAULT_TCP_KEEPALIVE_INTERVAL))
            .tcp_keepalive_retries(Some(DEFAULT_TCP_KEEPALIVE_RETRIES))
            .http2_keep_alive_interval(DEFAULT_HTTP2_KEEP_ALIVE_INTERVAL)
            .keep_alive_timeout(DEFAULT_HTTP2_KEEP_ALIVE_TIMEOUT)
            .connect_lazy();

        Ok(Self {
            uri,
            channel,
            headers: Default::default(),
            max_decoding_message_size: None,
            request_layer: None,
        })
    }

    pub fn with_headers(mut self, headers: HeadersInterceptor) -> Self {
        self.headers = headers;
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
        self.request_layer = Some(layer);
        self
    }

    pub fn with_max_decoding_message_size(mut self, limit: usize) -> Self {
        self.max_decoding_message_size = Some(limit);
        self
    }

    pub fn uri(&self) -> &http::Uri {
        &self.uri
    }

    fn channel(&self) -> BoxedChannel {
        let headers = self.headers.clone();

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

        // Apply the user's outbound request layer if present.
        let layered = if let Some(layer) = &self.request_layer {
            layer.layer(base)
        } else {
            base
        };

        // Map the final error to tonic::Status (a concrete type) so that
        // downstream users of the tonic-generated clients don't run into
        // lifetime-inference issues with async_trait and Box<dyn Error>.
        BoxService::new(
            ServiceBuilder::new()
                .map_err(tonic::Status::from_error)
                .service(layered),
        )
    }

    pub fn ledger_client(&mut self) -> LedgerServiceClient<BoxedChannel> {
        LedgerServiceClient::new(self.channel())
            .accept_compressed(CompressionEncoding::Zstd)
            .pipe(|client| {
                if let Some(limit) = self.max_decoding_message_size {
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
                if let Some(limit) = self.max_decoding_message_size {
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
                if let Some(limit) = self.max_decoding_message_size {
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
                if let Some(limit) = self.max_decoding_message_size {
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
                if let Some(limit) = self.max_decoding_message_size {
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
                if let Some(limit) = self.max_decoding_message_size {
                    client.max_decoding_message_size(limit)
                } else {
                    client
                }
            })
    }
}
