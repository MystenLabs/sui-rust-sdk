// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfoResponse {
    /// The chain identifier of the chain that this Node is on
    #[prost(message, optional, tag = "1")]
    pub chain_id: ::core::option::Option<super::super::types::Digest>,
    /// Human readable name of the chain that this Node is on
    #[prost(string, tag = "2")]
    pub chain: ::prost::alloc::string::String,
    /// Current epoch of the Node based on its highest executed checkpoint
    #[prost(uint64, tag = "3")]
    pub epoch: u64,
    /// Checkpoint height of the most recently executed checkpoint
    #[prost(uint64, tag = "4")]
    pub checkpoint_height: u64,
    /// Unix timestamp of the most recently executed checkpoint
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The lowest checkpoint for which checkpoints and transaction data is available
    #[prost(uint64, optional, tag = "6")]
    pub lowest_available_checkpoint: ::core::option::Option<u64>,
    /// The lowest checkpoint for which object data is available
    #[prost(uint64, optional, tag = "7")]
    pub lowest_available_checkpoint_objects: ::core::option::Option<u64>,
    #[prost(string, tag = "8")]
    pub software_version: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SubscribeRequest {}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SubscribeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResponse {
    /// The digest of this transaction
    #[prost(message, optional, tag = "1")]
    pub digest: ::core::option::Option<super::super::types::Digest>,
    #[prost(message, optional, tag = "2")]
    pub transaction: ::core::option::Option<super::super::types::Transaction>,
    #[prost(message, repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<super::super::types::UserSignature>,
    #[prost(message, optional, tag = "4")]
    pub effects: ::core::option::Option<super::super::types::TransactionEffects>,
    #[prost(message, optional, tag = "5")]
    pub events: ::core::option::Option<super::super::types::TransactionEvents>,
    #[prost(uint64, optional, tag = "6")]
    pub checkpoint: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "7")]
    pub timestamp_ms: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectResponse {
    /// The digest of this object
    #[prost(message, optional, tag = "1")]
    pub digest: ::core::option::Option<super::super::types::Digest>,
    #[prost(message, optional, tag = "2")]
    pub object: ::core::option::Option<super::super::types::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCheckpointResponse {
    /// The digest of this CheckpointSummary
    #[prost(message, optional, tag = "1")]
    pub digest: ::core::option::Option<super::super::types::Digest>,
    #[prost(message, optional, tag = "2")]
    pub summary: ::core::option::Option<super::super::types::CheckpointSummary>,
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<
        super::super::types::ValidatorAggregatedSignature,
    >,
    #[prost(message, optional, tag = "4")]
    pub contents: ::core::option::Option<super::super::types::CheckpointContents>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Checkpoint {
    #[prost(message, optional, tag = "1")]
    pub summary: ::core::option::Option<super::super::types::CheckpointSummary>,
    #[prost(message, optional, tag = "2")]
    pub signature: ::core::option::Option<
        super::super::types::ValidatorAggregatedSignature,
    >,
    #[prost(message, optional, tag = "3")]
    pub contents: ::core::option::Option<super::super::types::CheckpointContents>,
    #[prost(message, repeated, tag = "4")]
    pub transactions: ::prost::alloc::vec::Vec<CheckpointTransaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckpointTransaction {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<super::super::types::Transaction>,
    #[prost(message, repeated, tag = "2")]
    pub signatures: ::prost::alloc::vec::Vec<super::super::types::UserSignature>,
    #[prost(message, optional, tag = "3")]
    pub effects: ::core::option::Option<super::super::types::TransactionEffects>,
    #[prost(message, optional, tag = "4")]
    pub events: ::core::option::Option<super::super::types::TransactionEvents>,
    #[prost(message, repeated, tag = "5")]
    pub input_objects: ::prost::alloc::vec::Vec<super::super::types::Object>,
    #[prost(message, repeated, tag = "6")]
    pub output_objects: ::prost::alloc::vec::Vec<super::super::types::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCheckpointResponse {
    #[prost(message, repeated, tag = "1")]
    pub checkpoints: ::prost::alloc::vec::Vec<GetCheckpointResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<GetTransactionResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct I128 {
    #[prost(bytes = "bytes", tag = "1")]
    pub bytes: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::super::types::Address>,
    #[prost(message, optional, tag = "2")]
    pub coin_type: ::core::option::Option<super::super::types::TypeTag>,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<I128>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChanges {
    #[prost(message, repeated, tag = "4")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectsFinality {
    #[prost(message, optional, tag = "1")]
    pub signature: ::core::option::Option<
        super::super::types::ValidatorAggregatedSignature,
    >,
    #[prost(uint64, optional, tag = "2")]
    pub checkpoint: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "3")]
    pub quorum_executed: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionExecutionResponse {
    #[prost(message, optional, tag = "1")]
    pub effects: ::core::option::Option<super::super::types::TransactionEffects>,
    #[prost(message, optional, tag = "2")]
    pub finality: ::core::option::Option<EffectsFinality>,
    #[prost(message, optional, tag = "3")]
    pub events: ::core::option::Option<super::super::types::TransactionEvents>,
    #[prost(message, optional, tag = "4")]
    pub balance_changes: ::core::option::Option<BalanceChanges>,
    #[prost(message, repeated, tag = "5")]
    pub input_objects: ::prost::alloc::vec::Vec<super::super::types::Object>,
    #[prost(message, repeated, tag = "6")]
    pub output_objects: ::prost::alloc::vec::Vec<super::super::types::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionSimulationResponse {
    #[prost(message, optional, tag = "1")]
    pub effects: ::core::option::Option<super::super::types::TransactionEffects>,
    #[prost(message, optional, tag = "2")]
    pub events: ::core::option::Option<super::super::types::TransactionEvents>,
    #[prost(message, repeated, tag = "3")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChange>,
    #[prost(message, repeated, tag = "4")]
    pub input_objects: ::prost::alloc::vec::Vec<super::super::types::Object>,
    #[prost(message, repeated, tag = "5")]
    pub output_objects: ::prost::alloc::vec::Vec<super::super::types::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveTransactionResponse {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<super::super::types::Transaction>,
    #[prost(message, optional, tag = "2")]
    pub simulation: ::core::option::Option<TransactionSimulationResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteTransactionRequest {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<super::super::types::Transaction>,
    #[prost(message, repeated, tag = "2")]
    pub signatures: ::prost::alloc::vec::Vec<super::super::types::UserSignature>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulateTransactionRequest {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<super::super::types::Transaction>,
}
/// Generated client implementations.
pub mod node_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Rpc Node interface
    #[derive(Debug, Clone)]
    pub struct NodeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NodeClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> NodeClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> NodeClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            NodeClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_node_info(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::NodeInfoResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sui.node.v2.Node/GetNodeInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sui.node.v2.Node", "GetNodeInfo"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod subscriptions_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SubscriptionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SubscriptionsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SubscriptionsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SubscriptionsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            SubscriptionsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SubscribeResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sui.node.v2.Subscriptions/Subscribe",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sui.node.v2.Subscriptions", "Subscribe"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod node_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with NodeServer.
    #[async_trait]
    pub trait Node: std::marker::Send + std::marker::Sync + 'static {
        async fn get_node_info(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::NodeInfoResponse>,
            tonic::Status,
        >;
    }
    /// Rpc Node interface
    #[derive(Debug)]
    pub struct NodeServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> NodeServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NodeServer<T>
    where
        T: Node,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/sui.node.v2.Node/GetNodeInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetNodeInfoSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<()> for GetNodeInfoSvc<T> {
                        type Response = super::NodeInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Node>::get_node_info(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetNodeInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for NodeServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "sui.node.v2.Node";
    impl<T> tonic::server::NamedService for NodeServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated server implementations.
pub mod subscriptions_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SubscriptionsServer.
    #[async_trait]
    pub trait Subscriptions: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the Subscribe method.
        type SubscribeStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::SubscribeResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn subscribe(
            &self,
            request: tonic::Request<super::SubscribeRequest>,
        ) -> std::result::Result<tonic::Response<Self::SubscribeStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SubscriptionsServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> SubscriptionsServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SubscriptionsServer<T>
    where
        T: Subscriptions,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/sui.node.v2.Subscriptions/Subscribe" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSvc<T: Subscriptions>(pub Arc<T>);
                    impl<
                        T: Subscriptions,
                    > tonic::server::ServerStreamingService<super::SubscribeRequest>
                    for SubscribeSvc<T> {
                        type Response = super::SubscribeResponse;
                        type ResponseStream = T::SubscribeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Subscriptions>::subscribe(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SubscribeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for SubscriptionsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "sui.node.v2.Subscriptions";
    impl<T> tonic::server::NamedService for SubscriptionsServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
