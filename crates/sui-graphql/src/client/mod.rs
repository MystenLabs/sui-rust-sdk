//! GraphQL client for Sui blockchain.

mod objects;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::error::{Error, GraphQLError};
use crate::response::Response;

/// GraphQL client for Sui blockchain.
#[derive(Clone, Debug)]
pub struct Client {
    endpoint: String,
    http: reqwest::Client,
}

impl Client {
    /// Create a new GraphQL client with the given endpoint.
    ///
    /// # Example
    ///
    /// ```
    /// use sui_graphql::Client;
    ///
    /// let client = Client::new("https://sui-testnet.mystenlabs.com/graphql");
    /// ```
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            http: reqwest::Client::new(),
        }
    }

    /// Execute a GraphQL query and return the response.
    ///
    /// The response contains both data and any errors.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use sui_graphql::Client;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct MyResponse {
    ///     #[serde(rename = "chainIdentifier")]
    ///     chain_identifier: String,
    /// }
    ///
    /// let client = Client::new("https://sui-testnet.mystenlabs.com/graphql");
    /// let response = client.query::<MyResponse>(
    ///     "query { chainIdentifier }",
    ///     serde_json::json!({}),
    /// ).await?;
    ///
    ///
    /// // Access the data
    /// if let Some(data) = response.data {
    ///     println!("Chain: {}", data.chain_identifier);
    /// }
    /// ```
    pub async fn query<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<Response<T>, Error> {
        let request = GraphQLRequest { query, variables };

        let raw: GraphQLResponse<T> = self
            .http
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(Response::new(raw.data, raw.errors.unwrap_or_default()))
    }
}

#[derive(Serialize)]
struct GraphQLRequest<'a> {
    query: &'a str,
    variables: serde_json::Value,
}

#[derive(Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = Client::new("https://example.com/graphql");
        assert_eq!(client.endpoint, "https://example.com/graphql");
    }
}
