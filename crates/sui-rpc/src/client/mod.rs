use futures::TryStreamExt;
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

use crate::field::FieldMaskUtil;
use crate::proto::sui::rpc::v2beta2::ledger_service_client::LedgerServiceClient;
use crate::proto::sui::rpc::v2beta2::live_data_service_client::LiveDataServiceClient;
use crate::proto::sui::rpc::v2beta2::move_package_service_client::MovePackageServiceClient;
use crate::proto::sui::rpc::v2beta2::signature_verification_service_client::SignatureVerificationServiceClient;
use crate::proto::sui::rpc::v2beta2::subscription_service_client::SubscriptionServiceClient;
use crate::proto::sui::rpc::v2beta2::transaction_execution_service_client::TransactionExecutionServiceClient;
use crate::proto::sui::rpc::v2beta2::ExecuteTransactionRequest;
use crate::proto::sui::rpc::v2beta2::ExecutedTransaction;
use crate::proto::sui::rpc::v2beta2::SubscribeCheckpointsRequest;
use prost_types::FieldMask;

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

    /// Executes a transaction and waits for it to be included in a checkpoint.
    ///
    /// This method provides "read your writes" consistency by executing the transaction
    /// and waiting for it to appear in a checkpoint, which gauruntees indexes have been updated on
    /// this node.
    ///
    /// # Arguments
    /// * `request` - The transaction execution request (ExecuteTransactionRequest)
    /// * `timeout` - Maximum time to wait for indexing confirmation
    ///
    /// # Returns
    /// The executed transaction from the execution response, but only after confirming
    /// it has been included in a checkpoint and indexes have been updated.
    pub async fn execute_transaction_and_wait_for_checkpoint(
        &mut self,
        request: impl tonic::IntoRequest<ExecuteTransactionRequest>,
        timeout: Duration,
    ) -> Result<ExecutedTransaction> {
        // Subscribe to checkpoint stream before execution to avoid missing the transaction.
        // Uses minimal read mask for efficiency since we only nee digest confirmation.
        // Once server-side filtering is available, we should filter by transaction digest to
        // further reduce bandwidth.
        let mut checkpoint_stream = self
            .subscription_client()
            .subscribe_checkpoints(SubscribeCheckpointsRequest {
                read_mask: Some(FieldMask::from_str("transactions.digest,sequence_number")),
            })
            .await?
            .into_inner();

        let executed_transaction = self
            .execution_client()
            .execute_transaction(request)
            .await?
            .into_inner()
            .transaction()
            .to_owned();

        let executed_txn_digest = executed_transaction.digest();

        // Wait for the transaction to appear in a checkpoint. At this point indexes have been
        // updated.
        let timeout_future = tokio::time::sleep(timeout);
        let checkpoint_future = async {
            while let Some(response) = checkpoint_stream.try_next().await? {
                let checkpoint = response.checkpoint();

                for tx in checkpoint.transactions() {
                    let digest = tx.digest();

                    if digest == executed_txn_digest {
                        return Ok(());
                    }
                }
            }
            Err(tonic::Status::aborted(
                "checkpoint stream ended unexpectedly",
            ))
        };

        tokio::select! {
            result = checkpoint_future => {
                result?;
                Ok(executed_transaction)
            },
            _ = timeout_future => Err(tonic::Status::deadline_exceeded(format!("timeout waiting for checkpoint after {timeout:?}"))),
        }
    }
}
