use std::time::Duration;
use tap::Pipe;
use tonic::codec::CompressionEncoding;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::channel::ClientTlsConfig;

mod response_ext;
pub use response_ext::ResponseExt;

mod byte_count;
pub use byte_count::ByteCount;

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
type Channel<'a> = byte_count::ByteCountService<
    InterceptedService<&'a mut tonic::transport::Channel, &'a HeadersInterceptor>,
>;

#[derive(Clone)]
pub struct Client {
    uri: http::Uri,
    channel: tonic::transport::Channel,
    headers: HeadersInterceptor,
    max_decoding_message_size: Option<usize>,
    byte_count: bool,
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
            .connect_timeout(Duration::from_secs(5))
            .http2_keep_alive_interval(Duration::from_secs(5))
            .connect_lazy();

        Ok(Self {
            uri,
            channel,
            headers: Default::default(),
            max_decoding_message_size: None,
            byte_count: false,
        })
    }

    pub fn with_headers(mut self, headers: HeadersInterceptor) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_max_decoding_message_size(mut self, limit: usize) -> Self {
        self.max_decoding_message_size = Some(limit);
        self
    }

    pub fn with_byte_count(mut self) -> Self {
        self.byte_count = true;
        self
    }

    pub fn uri(&self) -> &http::Uri {
        &self.uri
    }

    fn make_channel<'a>(
        channel: &'a mut tonic::transport::Channel,
        headers: &'a HeadersInterceptor,
        byte_count: bool,
    ) -> Channel<'a> {
        byte_count::ByteCountService::new(InterceptedService::new(channel, headers), byte_count)
    }

    pub fn ledger_client(&mut self) -> LedgerServiceClient<Channel<'_>> {
        LedgerServiceClient::new(Self::make_channel(
            &mut self.channel,
            &self.headers,
            self.byte_count,
        ))
        .accept_compressed(CompressionEncoding::Zstd)
        .pipe(|client| {
            if let Some(limit) = self.max_decoding_message_size {
                client.max_decoding_message_size(limit)
            } else {
                client
            }
        })
    }

    pub fn state_client(&mut self) -> StateServiceClient<Channel<'_>> {
        StateServiceClient::new(Self::make_channel(
            &mut self.channel,
            &self.headers,
            self.byte_count,
        ))
        .accept_compressed(CompressionEncoding::Zstd)
        .pipe(|client| {
            if let Some(limit) = self.max_decoding_message_size {
                client.max_decoding_message_size(limit)
            } else {
                client
            }
        })
    }

    pub fn execution_client(&mut self) -> TransactionExecutionServiceClient<Channel<'_>> {
        TransactionExecutionServiceClient::new(Self::make_channel(
            &mut self.channel,
            &self.headers,
            self.byte_count,
        ))
        .accept_compressed(CompressionEncoding::Zstd)
        .pipe(|client| {
            if let Some(limit) = self.max_decoding_message_size {
                client.max_decoding_message_size(limit)
            } else {
                client
            }
        })
    }

    pub fn package_client(&mut self) -> MovePackageServiceClient<Channel<'_>> {
        MovePackageServiceClient::new(Self::make_channel(
            &mut self.channel,
            &self.headers,
            self.byte_count,
        ))
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
    ) -> SignatureVerificationServiceClient<Channel<'_>> {
        SignatureVerificationServiceClient::new(Self::make_channel(
            &mut self.channel,
            &self.headers,
            self.byte_count,
        ))
        .accept_compressed(CompressionEncoding::Zstd)
        .pipe(|client| {
            if let Some(limit) = self.max_decoding_message_size {
                client.max_decoding_message_size(limit)
            } else {
                client
            }
        })
    }

    pub fn subscription_client(&mut self) -> SubscriptionServiceClient<Channel<'_>> {
        SubscriptionServiceClient::new(Self::make_channel(
            &mut self.channel,
            &self.headers,
            self.byte_count,
        ))
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
