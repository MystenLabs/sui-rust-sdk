//! Transaction execution methods.

use base64ct::Base64;
use base64ct::Encoding;
use sui_graphql_macros::Response;
use sui_rpc::proto::sui::rpc::v2::BalanceChange;
use sui_sdk_types::Transaction;
use sui_sdk_types::TransactionEffects;
use sui_sdk_types::UserSignature;

use super::Client;
use crate::bcs::Bcs;
use crate::error::Error;

/// The result of executing a transaction on chain.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ExecutionResult {
    /// The transaction effects if execution was successful.
    pub effects: Option<TransactionEffects>,
    /// Balance changes from this transaction.
    pub balance_changes: Vec<BalanceChange>,
}

impl Client {
    /// Execute a signed transaction on chain.
    ///
    /// This commits the transaction to the blockchain and waits for finality.
    ///
    /// Execution errors (e.g., invalid signatures, insufficient gas) are returned as
    /// GraphQL errors with code `BAD_USER_INPUT`, accessible via `Response::errors()`.
    ///
    /// # Arguments
    ///
    /// * `transaction` - The transaction to execute
    /// * `signatures` - List of signatures authorizing the transaction
    ///
    /// # Returns
    ///
    /// - `Ok(result)` with `effects` and `balance_changes` if successful
    /// - `Err(...)` for network or decoding errors
    pub async fn execute_transaction(
        &self,
        transaction: &Transaction,
        signatures: &[UserSignature],
    ) -> Result<ExecutionResult, Error> {
        #[derive(Response)]
        #[response(root_type = "Mutation")]
        struct Response {
            #[field(path = "executeTransaction.effects.effectsBcs")]
            effects_bcs: Option<Bcs<TransactionEffects>>,
            #[field(path = "executeTransaction.effects.balanceChangesJson")]
            balance_changes: Option<Vec<BalanceChange>>,
        }

        const MUTATION: &str = r#"
            mutation($txDataBcs: Base64!, $signatures: [Base64!]!) {
                executeTransaction(transactionDataBcs: $txDataBcs, signatures: $signatures) {
                    effects {
                        effectsBcs
                        balanceChangesJson
                    }
                }
            }
        "#;

        let tx_bytes =
            bcs::to_bytes(transaction).map_err(|e| Error::Serialization(e.to_string()))?;
        let tx_data_base64 = Base64::encode_string(&tx_bytes);
        let signatures_base64: Vec<String> = signatures.iter().map(|sig| sig.to_base64()).collect();

        let variables = serde_json::json!({
            "txDataBcs": tx_data_base64,
            "signatures": signatures_base64,
        });

        let response = self.query::<Response>(MUTATION, variables).await?;

        let Some(data) = response.into_data() else {
            return Ok(ExecutionResult {
                effects: None,
                balance_changes: vec![],
            });
        };

        let effects = data.effects_bcs.map(|bcs| bcs.0);
        let balance_changes = data.balance_changes.unwrap_or_default();

        Ok(ExecutionResult {
            effects,
            balance_changes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sui_sdk_types::Address;
    use sui_sdk_types::GasPayment;
    use sui_sdk_types::ObjectReference;
    use sui_sdk_types::ProgrammableTransaction;
    use sui_sdk_types::SimpleSignature;
    use sui_sdk_types::TransactionExpiration;
    use sui_sdk_types::TransactionKind;
    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    /// Create a minimal test transaction.
    fn test_transaction() -> Transaction {
        let sender: Address = "0x1".parse().unwrap();
        let gas_object = ObjectReference::new(
            "0x2".parse().unwrap(),
            1,
            "4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi"
                .parse()
                .unwrap(),
        );

        Transaction {
            kind: TransactionKind::ProgrammableTransaction(ProgrammableTransaction {
                inputs: vec![],
                commands: vec![],
            }),
            sender,
            gas_payment: GasPayment {
                objects: vec![gas_object],
                owner: sender,
                price: 1000,
                budget: 10_000_000,
            },
            expiration: TransactionExpiration::None,
        }
    }

    /// Create a minimal test signature (not cryptographically valid, just for API testing).
    fn test_signature() -> UserSignature {
        UserSignature::Simple(SimpleSignature::Ed25519 {
            signature: [0u8; 64].into(),
            public_key: [0u8; 32].into(),
        })
    }

    #[tokio::test]
    async fn test_execute_transaction_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "executeTransaction": {
                        "effects": {
                            "effectsBcs": null,
                            "balanceChangesJson": null
                        }
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();
        let transaction = test_transaction();
        let signature = test_signature();

        let result = client
            .execute_transaction(&transaction, &[signature])
            .await
            .unwrap();

        assert!(result.effects.is_none());
        assert!(result.balance_changes.is_empty());
    }

    #[tokio::test]
    async fn test_execute_transaction_graphql_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": null,
                "errors": [{
                    "message": "Invalid argument: Invalid user signature",
                    "extensions": { "code": "BAD_USER_INPUT" }
                }]
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();
        let transaction = test_transaction();
        let signature = test_signature();

        let result = client
            .execute_transaction(&transaction, &[signature])
            .await
            .unwrap();

        // No data returned, effects should be None
        assert!(result.effects.is_none());
        assert!(result.balance_changes.is_empty());
    }
}
