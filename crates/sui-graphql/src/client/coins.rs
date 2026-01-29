//! Coin-related convenience methods.

use futures::Stream;
use sui_graphql_macros::Response;
use sui_sdk_types::Address;
use sui_sdk_types::StructTag;

use super::Client;
use crate::error::Error;
use crate::pagination::Page;
use crate::pagination::PageInfo;
use crate::pagination::paginate;
use crate::scalars::BigInt;

/// Balance information for a coin type.
#[derive(Debug, Clone)]
pub struct Balance {
    /// The coin type (e.g., `0x2::sui::SUI`).
    pub coin_type: StructTag,
    /// The total balance in base units.
    pub total_balance: u64,
}

impl Client {
    /// Get the balance for a specific coin type owned by an address.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sui_graphql::Client;
    /// use sui_sdk_types::Address;
    /// use sui_sdk_types::StructTag;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new(Client::MAINNET)?;
    ///     let owner: Address = "0x123...".parse()?;
    ///
    ///     // Get SUI balance using the helper
    ///     let sui_balance = client.get_balance(owner, &StructTag::sui()).await?;
    ///     if let Some(bal) = sui_balance {
    ///         println!("SUI balance: {}", bal.total_balance);
    ///     }
    ///
    ///     // Get balance for other coin types by parsing the type string
    ///     let usdc_type: StructTag = "0xdba...::usdc::USDC".parse()?;
    ///     let usdc_balance = client.get_balance(owner, &usdc_type).await?;
    ///     if let Some(bal) = usdc_balance {
    ///         println!("USDC balance: {}", bal.total_balance);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_balance(
        &self,
        owner: Address,
        coin_type: &StructTag,
    ) -> Result<Option<Balance>, Error> {
        #[derive(Response)]
        struct Response {
            #[field(path = "address.balance.coinType.repr")]
            coin_type: Option<StructTag>,
            #[field(path = "address.balance.totalBalance")]
            total_balance: Option<BigInt>,
        }

        const QUERY: &str = r#"
            query($owner: SuiAddress!, $coinType: String!) {
                address(address: $owner) {
                    balance(coinType: $coinType) {
                        coinType {
                            repr
                        }
                        totalBalance
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "owner": owner,
            "coinType": coin_type.to_string(),
        });

        let response = self.query::<Response>(QUERY, variables).await?;

        let Some(data) = response.into_data() else {
            return Ok(None);
        };

        match (data.coin_type, data.total_balance) {
            (Some(coin_type), Some(total_balance)) => Ok(Some(Balance {
                coin_type,
                total_balance: total_balance.0,
            })),
            _ => Ok(None),
        }
    }

    /// Stream all coin balances owned by an address.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use futures::StreamExt;
    /// use std::pin::pin;
    /// use sui_graphql::Client;
    /// use sui_sdk_types::Address;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new(Client::MAINNET)?;
    ///     let owner: Address = "0x123...".parse()?;
    ///
    ///     let mut stream = pin!(client.list_balances(owner));
    ///     while let Some(result) = stream.next().await {
    ///         let balance = result?;
    ///         println!("{}: {}", balance.coin_type, balance.total_balance);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn list_balances(&self, owner: Address) -> impl Stream<Item = Result<Balance, Error>> + '_ {
        let client = self.clone();
        paginate(move |cursor| {
            let client = client.clone();
            async move { client.fetch_balances_page(owner, cursor.as_deref()).await }
        })
    }

    /// Fetch a single page of balances.
    async fn fetch_balances_page(
        &self,
        owner: Address,
        cursor: Option<&str>,
    ) -> Result<Page<Balance>, Error> {
        #[derive(Response)]
        struct Response {
            #[field(path = "address.balances.pageInfo")]
            page_info: Option<PageInfo>,
            #[field(path = "address.balances.nodes[].coinType.repr")]
            coin_types: Option<Vec<Option<StructTag>>>,
            #[field(path = "address.balances.nodes[].totalBalance")]
            total_balances: Option<Vec<Option<BigInt>>>,
        }

        const QUERY: &str = r#"
            query($owner: SuiAddress!, $after: String) {
                address(address: $owner) {
                    balances(after: $after) {
                        pageInfo {
                            hasNextPage
                            endCursor
                        }
                        nodes {
                            coinType {
                                repr
                            }
                            totalBalance
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "owner": owner,
            "after": cursor,
        });

        let response = self.query::<Response>(QUERY, variables).await?;

        let data = response.into_data();
        let page_info = data
            .as_ref()
            .and_then(|d| d.page_info.clone())
            .unwrap_or_default();

        let (coin_types, total_balances) = data
            .map(|d| {
                (
                    d.coin_types.unwrap_or_default(),
                    d.total_balances.unwrap_or_default(),
                )
            })
            .unwrap_or_default();

        // Zip coin_types and total_balances together
        let balances: Vec<Balance> = coin_types
            .into_iter()
            .zip(total_balances)
            .filter_map(|(ct, tb)| match (ct, tb) {
                (Some(coin_type), Some(total_balance)) => Some((coin_type, total_balance)),
                _ => None,
            })
            .map(|(coin_type, total_balance)| Balance {
                coin_type,
                total_balance: total_balance.0,
            })
            .collect();

        Ok(Page {
            items: balances,
            has_next_page: page_info.has_next_page,
            end_cursor: page_info.end_cursor,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    #[tokio::test]
    async fn test_get_balance_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "address": {
                        "balance": {
                            "coinType": {
                                "repr": "0x2::sui::SUI"
                            },
                            "totalBalance": "1000000000"
                        }
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();
        let owner: Address = "0x1".parse().unwrap();

        let result = client.get_balance(owner, &StructTag::sui()).await;
        assert!(result.is_ok());

        let balance = result.unwrap();
        assert!(balance.is_some());

        let balance = balance.unwrap();
        assert_eq!(balance.coin_type, StructTag::sui());
        assert_eq!(balance.total_balance, 1000000000);
    }

    #[tokio::test]
    async fn test_get_balance_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "address": {
                        "balance": null
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();
        let owner: Address = "0x1".parse().unwrap();

        let result = client.get_balance(owner, &StructTag::sui()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_balance_invalid_number() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "address": {
                        "balance": {
                            "coinType": {
                                "repr": "0x2::sui::SUI"
                            },
                            "totalBalance": "not_a_number"
                        }
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();
        let owner: Address = "0x1".parse().unwrap();

        let result = client.get_balance(owner, &StructTag::sui()).await;
        assert!(matches!(result, Err(Error::Request(e)) if e.is_decode()));
    }

    #[tokio::test]
    async fn test_list_balances_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "address": {
                        "balances": {
                            "pageInfo": {
                                "hasNextPage": false,
                                "endCursor": null
                            },
                            "nodes": []
                        }
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();
        let owner: Address = "0x1".parse().unwrap();

        let stream = client.list_balances(owner);
        let balances: Vec<_> = stream.collect().await;

        assert!(balances.is_empty());
    }

    #[tokio::test]
    async fn test_list_balances_multiple() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "address": {
                        "balances": {
                            "pageInfo": {
                                "hasNextPage": false,
                                "endCursor": null
                            },
                            "nodes": [
                                {
                                    "coinType": { "repr": "0x2::sui::SUI" },
                                    "totalBalance": "1000000000"
                                },
                                {
                                    "coinType": { "repr": "0xabc::token::USDC" },
                                    "totalBalance": "500000"
                                }
                            ]
                        }
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();
        let owner: Address = "0x1".parse().unwrap();

        let stream = client.list_balances(owner);
        let balances: Vec<_> = stream.collect().await;

        assert_eq!(balances.len(), 2);
        assert!(balances[0].is_ok());
        assert!(balances[1].is_ok());

        let bal1 = balances[0].as_ref().unwrap();
        assert_eq!(bal1.coin_type, StructTag::sui());
        assert_eq!(bal1.total_balance, 1000000000);

        let bal2 = balances[1].as_ref().unwrap();
        let usdc: StructTag = "0xabc::token::USDC".parse().unwrap();
        assert_eq!(bal2.coin_type, usdc);
        assert_eq!(bal2.total_balance, 500000);
    }

    #[tokio::test]
    async fn test_list_balances_with_pagination() {
        let mock_server = MockServer::start().await;
        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(move |_req: &wiremock::Request| {
                let count = call_count_clone.fetch_add(1, Ordering::SeqCst);
                match count {
                    0 => ResponseTemplate::new(200).set_body_json(serde_json::json!({
                        "data": {
                            "address": {
                                "balances": {
                                    "pageInfo": {
                                        "hasNextPage": true,
                                        "endCursor": "cursor1"
                                    },
                                    "nodes": [
                                        {
                                            "coinType": { "repr": "0x2::sui::SUI" },
                                            "totalBalance": "1000000000"
                                        }
                                    ]
                                }
                            }
                        }
                    })),
                    1 => ResponseTemplate::new(200).set_body_json(serde_json::json!({
                        "data": {
                            "address": {
                                "balances": {
                                    "pageInfo": {
                                        "hasNextPage": false,
                                        "endCursor": null
                                    },
                                    "nodes": [
                                        {
                                            "coinType": { "repr": "0xabc::token::USDC" },
                                            "totalBalance": "500000"
                                        }
                                    ]
                                }
                            }
                        }
                    })),
                    _ => ResponseTemplate::new(200).set_body_json(serde_json::json!({
                        "data": {
                            "address": {
                                "balances": {
                                    "pageInfo": { "hasNextPage": false, "endCursor": null },
                                    "nodes": []
                                }
                            }
                        }
                    })),
                }
            })
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();
        let owner: Address = "0x1".parse().unwrap();

        let stream = client.list_balances(owner);
        let balances: Vec<_> = stream.collect().await;

        assert_eq!(balances.len(), 2);
        assert_eq!(call_count.load(Ordering::SeqCst), 2);

        let bal1 = balances[0].as_ref().unwrap();
        assert_eq!(bal1.coin_type, StructTag::sui());
        assert_eq!(bal1.total_balance, 1000000000);

        let bal2 = balances[1].as_ref().unwrap();
        let usdc: StructTag = "0xabc::token::USDC".parse().unwrap();
        assert_eq!(bal2.coin_type, usdc);
        assert_eq!(bal2.total_balance, 500000);
    }
}
