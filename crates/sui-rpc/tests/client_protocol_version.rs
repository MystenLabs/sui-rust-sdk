//! Integration tests for the `x-sui-client-protocol-version` request header.

use proto::ledger_service_server::LedgerService;
use proto::ledger_service_server::LedgerServiceServer;
use sui_rpc::Client;
use sui_rpc::client::HeadersInterceptor;
use sui_rpc::headers::MAX_PROTOCOL_VERSION;
use sui_rpc::headers::X_SUI_CLIENT_PROTOCOL_VERSION;
use sui_rpc::proto::sui::rpc::v2 as proto;

/// Echoes the received header back as `server`.
struct MockServer;

#[tonic::async_trait]
impl LedgerService for MockServer {
    async fn get_service_info(
        &self,
        request: tonic::Request<proto::GetServiceInfoRequest>,
    ) -> Result<tonic::Response<proto::GetServiceInfoResponse>, tonic::Status> {
        let mut info = proto::GetServiceInfoResponse::default();
        info.server = request
            .metadata()
            .get(X_SUI_CLIENT_PROTOCOL_VERSION)
            .map(|v| v.to_str().unwrap().to_owned());
        Ok(tonic::Response::new(info))
    }
}

async fn received_header(client: impl FnOnce(String) -> Client) -> Option<String> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock server listener");
    let addr = listener.local_addr().expect("mock server local addr");
    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(LedgerServiceServer::new(MockServer))
            .serve_with_incoming(tonic::transport::server::TcpIncoming::from(listener))
            .await
            .expect("mock server exited with an error");
    });

    client(format!("http://{addr}"))
        .ledger_client()
        .get_service_info(proto::GetServiceInfoRequest::default())
        .await
        .expect("get_service_info")
        .into_inner()
        .server
}

#[tokio::test]
async fn client_sends_max_protocol_version() {
    let received = received_header(|uri| Client::new(uri).expect("client")).await;
    assert_eq!(received, Some(MAX_PROTOCOL_VERSION.to_string()));
}

#[tokio::test]
async fn configured_headers_override_protocol_version() {
    let received = received_header(|uri| {
        let mut headers = HeadersInterceptor::new();
        headers
            .headers_mut()
            .insert(X_SUI_CLIENT_PROTOCOL_VERSION, "7".parse().unwrap());
        Client::new(uri).expect("client").with_headers(headers)
    })
    .await;
    assert_eq!(received.as_deref(), Some("7"));
}
