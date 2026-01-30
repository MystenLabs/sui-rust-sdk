//! Transaction-related convenience methods.

use sui_graphql_macros::Response;
use sui_sdk_types::Transaction;
use sui_sdk_types::TransactionEffects;

use super::Client;
use crate::bcs::Bcs;
use crate::error::Error;
use crate::scalars::DateTime;

/// A balance change from a transaction (re-exported from sui-rpc).
pub use sui_rpc::proto::sui::rpc::v2::BalanceChange;

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
    /// Balance changes from this transaction.
    pub balance_changes: Vec<BalanceChange>,
    /// The checkpoint sequence number this transaction was finalized in.
    pub checkpoint: u64,
    /// Timestamp when this transaction was finalized.
    pub timestamp: DateTime,
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
    /// let client = Client::new("https://graphql.mainnet.sui.io/graphql")?;
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
            transaction_bcs: Option<Bcs<Transaction>>,
            #[field(path = "transaction.effects.effectsBcs")]
            effects_bcs: Option<Bcs<TransactionEffects>>,
            #[field(path = "transaction.effects.balanceChangesJson")]
            balance_changes: Option<Vec<BalanceChange>>,
            #[field(path = "transaction.effects.checkpoint.sequenceNumber")]
            checkpoint: Option<u64>,
            #[field(path = "transaction.effects.timestamp")]
            timestamp: Option<DateTime>,
        }

        const QUERY: &str = r#"
            query($digest: String!) {
                transaction(digest: $digest) {
                    transactionBcs
                    effects {
                        effectsBcs
                        balanceChangesJson
                        checkpoint {
                            sequenceNumber
                        }
                        timestamp
                    }
                }
            }
        "#;

        let variables = serde_json::json!({ "digest": digest });

        let response = self.query::<Response>(QUERY, variables).await?;

        let Some(data) = response.into_data() else {
            return Ok(None);
        };

        let (Some(transaction), Some(effects)) = (data.transaction_bcs, data.effects_bcs) else {
            return Ok(None);
        };

        let transaction = transaction.0;
        let effects = effects.0;
        let balance_changes = data.balance_changes.unwrap_or_default();
        let checkpoint = data.checkpoint.ok_or(Error::MissingData("checkpoint"))?;
        let timestamp = data.timestamp.ok_or(Error::MissingData("timestamp"))?;

        Ok(Some(TransactionResponse {
            transaction,
            effects,
            balance_changes,
            checkpoint,
            timestamp,
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
