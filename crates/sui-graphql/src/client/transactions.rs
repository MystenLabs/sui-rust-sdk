//! Transaction-related convenience methods.

use base64ct::Base64;
use base64ct::Encoding;
use sui_graphql_macros::Response;
use sui_sdk_types::Transaction;
use sui_sdk_types::TransactionEffects;

use super::Client;
use crate::error::Error;

/// A transaction response containing the transaction data and its effects.
///
/// This struct combines the transaction data with its execution results.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct TransactionResponse {
    /// The transaction data (sender, commands, gas payment, etc.)
    pub transaction: Transaction,
    /// The execution effects (status, gas used, object changes, etc.)
    pub effects: TransactionEffects,
}

impl Client {
    /// Fetch a transaction by its digest and deserialize from BCS.
    ///
    /// Returns:
    /// - `Ok(Some(response))` if the transaction exists
    /// - `Ok(None)` if the transaction does not exist
    /// - `Err(Error::Request)` for network errors
    /// - `Err(Error::Base64)` / `Err(Error::Bcs)` for decoding errors
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use sui_graphql::Client;
    ///
    /// let client = Client::new("https://sui-mainnet.mystenlabs.com/graphql")?;
    /// let digest = "ABC123..."; // transaction digest
    ///
    /// match client.get_transaction(digest).await? {
    ///     Some(tx) => {
    ///         println!("Sender: {}", tx.transaction.sender);
    ///         println!("Status: {:?}", tx.effects.status());
    ///     }
    ///     None => println!("Transaction not found"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_transaction(
        &self,
        digest: &str,
    ) -> Result<Option<TransactionResponse>, Error> {
        #[derive(Response)]
        struct Response {
            #[field(path = "transaction.transactionBcs")]
            transaction_bcs: Option<String>,
            #[field(path = "transaction.effects.effectsBcs")]
            effects_bcs: Option<String>,
        }

        const QUERY: &str = r#"
            query($digest: String!) {
                transaction(digest: $digest) {
                    transactionBcs
                    effects {
                        effectsBcs
                    }
                }
            }
        "#;

        let variables = serde_json::json!({ "digest": digest });

        let response = self.query::<Response>(QUERY, variables).await?;

        let Some(data) = response.into_data() else {
            return Ok(None);
        };

        let (Some(tx_bcs), Some(effects_bcs)) = (data.transaction_bcs, data.effects_bcs) else {
            return Ok(None);
        };

        let tx_bytes = Base64::decode_vec(&tx_bcs)?;
        let transaction: Transaction = bcs::from_bytes(&tx_bytes)?;

        let effects_bytes = Base64::decode_vec(&effects_bcs)?;
        let effects: TransactionEffects = bcs::from_bytes(&effects_bytes)?;

        Ok(Some(TransactionResponse {
            transaction,
            effects,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    #[tokio::test]
    async fn test_get_transaction_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "transaction": null
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();

        let result = client.get_transaction("nonexistent").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
