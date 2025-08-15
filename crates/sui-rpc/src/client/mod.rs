use std::time::Duration;
use tap::Pipe;
use tonic::codec::CompressionEncoding;
use tonic::transport::channel::ClientTlsConfig;

mod response_ext;
pub use response_ext::ResponseExt;

mod auth;
pub use auth::AuthInterceptor;

mod staking_rewards;
pub use staking_rewards::DelegatedStake;

use crate::proto::sui::rpc::v2beta2::ledger_service_client::LedgerServiceClient;
use crate::proto::sui::rpc::v2beta2::live_data_service_client::LiveDataServiceClient;
use crate::proto::sui::rpc::v2beta2::move_package_service_client::MovePackageServiceClient;
use crate::proto::sui::rpc::v2beta2::signature_verification_service_client::SignatureVerificationServiceClient;
use crate::proto::sui::rpc::v2beta2::subscription_service_client::SubscriptionServiceClient;
use crate::proto::sui::rpc::v2beta2::transaction_execution_service_client::TransactionExecutionServiceClient;

type Result<T, E = tonic::Status> = std::result::Result<T, E>;
type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Channel<'a> = tonic::service::interceptor::InterceptedService<
    &'a mut tonic::transport::Channel,
    &'a mut AuthInterceptor,
>;

#[derive(Clone)]
pub struct Client {
    uri: http::Uri,
    channel: tonic::transport::Channel,
    auth: AuthInterceptor,
    max_decoding_message_size: Option<usize>,
}

impl Client {
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
            auth: Default::default(),
            max_decoding_message_size: None,
        })
    }

    pub fn with_auth(mut self, auth: AuthInterceptor) -> Self {
        self.auth = auth;
        self
    }

    pub fn with_max_decoding_message_size(mut self, limit: usize) -> Self {
        self.max_decoding_message_size = Some(limit);
        self
    }

    pub fn uri(&self) -> &http::Uri {
        &self.uri
    }

    pub fn ledger_client(&mut self) -> LedgerServiceClient<Channel<'_>> {
        LedgerServiceClient::with_interceptor(&mut self.channel, &mut self.auth)
            .accept_compressed(CompressionEncoding::Zstd)
            .pipe(|client| {
                if let Some(limit) = self.max_decoding_message_size {
                    client.max_decoding_message_size(limit)
                } else {
                    client
                }
            })
    }

    pub fn live_data_client(&mut self) -> LiveDataServiceClient<Channel<'_>> {
        LiveDataServiceClient::with_interceptor(&mut self.channel, &mut self.auth)
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
        TransactionExecutionServiceClient::with_interceptor(&mut self.channel, &mut self.auth)
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
        MovePackageServiceClient::with_interceptor(&mut self.channel, &mut self.auth)
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
        SignatureVerificationServiceClient::with_interceptor(&mut self.channel, &mut self.auth)
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
        SubscriptionServiceClient::with_interceptor(&mut self.channel, &mut self.auth)
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
