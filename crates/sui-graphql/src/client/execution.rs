//! Transaction execution methods.

use base64ct::Base64;
use base64ct::Encoding;
use sui_graphql_macros::Response;
use sui_sdk_types::TransactionEffects;

use super::Client;
use crate::bcs::Bcs;
use crate::error::Error;

/// The result of executing a transaction on chain.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ExecutionResult {
    /// The transaction effects if execution was successful.
    pub effects: Option<TransactionEffects>,
    /// Errors that occurred during execution (e.g., network errors, validation failures).
    /// These are distinct from execution failures within the transaction itself.
    pub errors: Option<Vec<String>>,
}

impl Client {
    /// Execute a signed transaction on chain.
    ///
    /// This commits the transaction to the blockchain and waits for finality.
    ///
    /// # Arguments
    ///
    /// * `transaction_data` - BCS-encoded `TransactionData` bytes
    /// * `signatures` - List of signatures (each is `flag || signature || pubkey` bytes)
    ///
    /// # Returns
    ///
    /// - `Ok(result)` with `effects` if successful, or `errors` if execution failed
    /// - `Err(...)` for network or decoding errors
    pub async fn execute_transaction(
        &self,
        transaction_data: &[u8],
        signatures: &[Vec<u8>],
    ) -> Result<ExecutionResult, Error> {
        #[derive(Response)]
        #[response(mutation)]
        struct Response {
            #[field(path = "executeTransaction.effects.effectsBcs")]
            effects_bcs: Option<String>,
            #[field(path = "executeTransaction.errors")]
            errors: Option<Vec<String>>,
        }

        const MUTATION: &str = r#"
            mutation($txDataBcs: Base64!, $signatures: [Base64!]!) {
                executeTransaction(transactionDataBcs: $txDataBcs, signatures: $signatures) {
                    effects {
                        effectsBcs
                    }
                    errors
                }
            }
        "#;

        let tx_data_base64 = Base64::encode_string(transaction_data);
        let signatures_base64: Vec<String> = signatures
            .iter()
            .map(|sig| Base64::encode_string(sig))
            .collect();

        let variables = serde_json::json!({
            "txDataBcs": tx_data_base64,
            "signatures": signatures_base64,
        });

        let response = self.query::<Response>(MUTATION, variables).await?;

        // Check for GraphQL-level errors first
        let graphql_errors: Vec<String> = response
            .errors()
            .iter()
            .map(|e| e.message().to_string())
            .collect();

        let Some(data) = response.into_data() else {
            // If no data, return GraphQL errors or a generic message
            let errors = if graphql_errors.is_empty() {
                vec!["No data in response".to_string()]
            } else {
                graphql_errors
            };
            return Ok(ExecutionResult {
                effects: None,
                errors: Some(errors),
            });
        };

        let effects = if let Some(effects_bcs) = data.effects_bcs {
            Some(Bcs::<TransactionEffects>::decode(&effects_bcs)?.into_inner())
        } else {
            None
        };

        Ok(ExecutionResult {
            effects,
            errors: data.errors,
        })
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
    async fn test_execute_transaction_with_errors() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "executeTransaction": {
                        "effects": null,
                        "errors": ["Insufficient gas", "Invalid signature"]
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();

        let result = client
            .execute_transaction(&[1, 2, 3], &[vec![4, 5, 6]])
            .await
            .unwrap();

        assert!(result.effects.is_none());
        assert!(result.errors.is_some());
        let errors = result.errors.unwrap();
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0], "Insufficient gas");
    }
}
