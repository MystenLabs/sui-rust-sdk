use anyhow::Result;
use async_trait::async_trait;
use cynic::{serde, GraphQlResponse, Operation};
use sui_graphql_client::{HttpClient, SuiClient};
use surf::Client as SurfClient;

pub struct SurfHttpClient {
    client: SurfClient,
}

impl Default for SurfHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfHttpClient {
    pub fn new() -> Self {
        Self {
            client: SurfClient::new(),
        }
    }
}

#[async_trait]
impl HttpClient for SurfHttpClient {
    async fn post<
        T: serde::de::DeserializeOwned + Send,
        V: serde::Serialize + Send + std::marker::Sync,
    >(
        &self,
        url: &str,
        operation: &Operation<T, V>,
    ) -> Result<GraphQlResponse<T>> {
        let mut res = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(surf::Body::from_json(&operation).expect("Failed to serialize operation"))
            .await
            .expect("Failed to send request");
        let graphql_response: cynic::GraphQlResponse<T> =
            res.body_json().await.expect("Failed to parse response");

        Ok(graphql_response)
    }
}

#[tokio::main]
async fn main() {
    let client = SuiClient::new_with_http_client(SurfHttpClient::new());
    let chain_id = client.chain_id().await;
    println!("{:?}", chain_id);
}
