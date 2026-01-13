//! Object-related convenience methods.

use base64ct::{Base64, Encoding};
use futures::Stream;
use sui_graphql_macros::QueryResponse;
use sui_sdk_types::{Address, Object};

use super::Client;
use crate::error::Error;
use crate::pagination::{paginate, Page, PageInfo};

impl Client {
    /// Fetch an object by its ID and deserialize from BCS.
    ///
    /// Returns:
    /// - `Ok(Some(object))` if the object exists
    /// - `Ok(None)` if the object does not exist
    /// - `Err(Error::Request)` for network errors
    /// - `Err(Error::Base64)` / `Err(Error::Bcs)` for decoding errors
    ///
    /// # Example
    ///
    /// ```ignore
    /// use sui_graphql::Client;
    /// use sui_sdk_types::Address;
    ///
    /// let client = Client::new("https://sui-mainnet.mystenlabs.com/graphql");
    /// let object_id: Address = "0x5".parse()?;
    ///
    /// match client.get_object(object_id).await? {
    ///     Some(object) => println!("Object version: {}", object.version()),
    ///     None => println!("Object not found"),
    /// }
    /// ```
    pub async fn get_object(&self, object_id: Address) -> Result<Option<Object>, Error> {
        #[derive(QueryResponse)]
        struct Response {
            #[field(path = "object.objectBcs")]
            object_bcs: Option<String>,
        }

        const QUERY: &str = r#"
            query($id: SuiAddress!) {
                object(address: $id) {
                    objectBcs
                }
            }
        "#;

        let variables = serde_json::json!({ "id": object_id });

        let response = self.query::<Response>(QUERY, variables).await?;

        let Some(bcs_data) = response.data.and_then(|d| d.object_bcs) else {
            return Ok(None);
        };

        let bytes = Base64::decode_vec(&bcs_data)?;
        let object: Object = bcs::from_bytes(&bytes)?;

        Ok(Some(object))
    }

    /// Stream all objects owned by an address.
    ///
    /// Handles pagination automatically, fetching pages as needed.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use sui_graphql::Client;
    /// use sui_sdk_types::Address;
    /// use futures::StreamExt;
    ///
    /// let client = Client::new("https://sui-testnet.mystenlabs.com/graphql");
    /// let owner: Address = "0x123...".parse()?;
    ///
    /// let mut stream = client.list_objects(owner);
    /// while let Some(result) = stream.next().await {
    ///     let object = result?;
    ///     println!("Object: {:?}", object.id());
    /// }
    /// ```
    pub fn list_objects(&self, owner: Address) -> impl Stream<Item = Result<Object, Error>> + '_ {
        let client = self.clone();
        paginate(move |cursor| {
            let client = client.clone();
            async move { client.fetch_objects_page(owner, cursor.as_deref()).await }
        })
    }

    /// Fetch a single page of objects owned by an address.
    async fn fetch_objects_page(
        &self,
        owner: Address,
        cursor: Option<&str>,
    ) -> Result<Page<Object>, Error> {
        #[derive(QueryResponse)]
        struct Response {
            #[field(path = "objects.pageInfo")]
            page_info: Option<PageInfo>,
            #[field(path = "objects.nodes[].objectBcs")]
            bcs_list: Vec<Option<String>>,
        }

        const QUERY: &str = r#"
            query($owner: SuiAddress!, $after: String) {
                objects(filter: { owner: $owner }, after: $after) {
                    pageInfo {
                        hasNextPage
                        endCursor
                    }
                    nodes {
                        objectBcs
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "owner": owner,
            "after": cursor,
        });

        let response = self.query::<Response>(QUERY, variables).await?;

        let page_info = response
            .data
            .as_ref()
            .and_then(|d| d.page_info.clone())
            .unwrap_or(PageInfo {
                has_next_page: false,
                end_cursor: None,
            });

        let bcs_list = response.data.map(|d| d.bcs_list).unwrap_or_default();

        // Decode BCS for each object
        let mut objects = Vec::with_capacity(bcs_list.len());
        for bcs_data in bcs_list.into_iter().flatten() {
            let bytes = Base64::decode_vec(&bcs_data)?;
            let object: Object = bcs::from_bytes(&bytes)?;
            objects.push(object);
        }

        Ok(Page {
            items: objects,
            has_next_page: page_info.has_next_page,
            end_cursor: page_info.end_cursor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// BCS-encoded SUI coin from sui-sdk-types test fixtures, encoded as base64.
    fn test_object_bcs() -> String {
        use base64ct::{Base64, Encoding};

        // From sui-sdk-types/src/object.rs test fixtures (SUI_COIN)
        const SUI_COIN_BCS: &[u8] = &[
            0, 1, 1, 32, 79, 43, 0, 0, 0, 0, 0, 40, 35, 95, 175, 213, 151, 87, 206, 190, 35, 131,
            79, 35, 254, 22, 15, 181, 40, 108, 28, 77, 68, 229, 107, 254, 191, 160, 196, 186, 42,
            2, 122, 53, 52, 133, 199, 58, 0, 0, 0, 0, 0, 79, 255, 208, 0, 85, 34, 190, 75, 192, 41,
            114, 76, 127, 15, 110, 215, 9, 58, 107, 243, 160, 155, 144, 230, 47, 97, 220, 21, 24,
            30, 26, 62, 32, 17, 197, 192, 38, 64, 173, 142, 143, 49, 111, 15, 211, 92, 84, 48, 160,
            243, 102, 229, 253, 251, 137, 210, 101, 119, 173, 228, 51, 141, 20, 15, 85, 96, 19, 15,
            0, 0, 0, 0, 0,
        ];
        Base64::encode_string(SUI_COIN_BCS)
    }

    #[tokio::test]
    async fn test_get_object_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "object": null
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(mock_server.uri());
        let object_id: Address = "0x5".parse().unwrap();

        let result = client.get_object(object_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_object_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "object": {
                        "objectBcs": test_object_bcs()
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(mock_server.uri());
        let object_id: Address = "0x5".parse().unwrap();

        let result = client.get_object(object_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_list_objects_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "objects": {
                        "pageInfo": {
                            "hasNextPage": false,
                            "endCursor": null
                        },
                        "nodes": []
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(mock_server.uri());
        let owner: Address = "0x1".parse().unwrap();

        let stream = client.list_objects(owner);
        let objects: Vec<_> = futures::StreamExt::collect(stream).await;

        assert!(objects.is_empty());
    }

    #[tokio::test]
    async fn test_list_objects_with_pagination() {
        let mock_server = MockServer::start().await;
        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(move |_req: &wiremock::Request| {
                let count = call_count_clone.fetch_add(1, Ordering::SeqCst);
                match count {
                    // Page 1: 3 objects
                    0 => ResponseTemplate::new(200).set_body_json(serde_json::json!({
                        "data": {
                            "objects": {
                                "pageInfo": {
                                    "hasNextPage": true,
                                    "endCursor": "cursor1"
                                },
                                "nodes": [
                                    { "objectBcs": test_object_bcs() },
                                    { "objectBcs": test_object_bcs() },
                                    { "objectBcs": test_object_bcs() }
                                ]
                            }
                        }
                    })),
                    // Page 2: 2 objects
                    1 => ResponseTemplate::new(200).set_body_json(serde_json::json!({
                        "data": {
                            "objects": {
                                "pageInfo": {
                                    "hasNextPage": false,
                                    "endCursor": null
                                },
                                "nodes": [
                                    { "objectBcs": test_object_bcs() },
                                    { "objectBcs": test_object_bcs() }
                                ]
                            }
                        }
                    })),
                    _ => ResponseTemplate::new(200).set_body_json(serde_json::json!({
                        "data": { "objects": { "pageInfo": { "hasNextPage": false, "endCursor": null }, "nodes": [] } }
                    })),
                }
            })
            .mount(&mock_server)
            .await;

        let client = Client::new(mock_server.uri());
        let owner: Address = "0x1".parse().unwrap();

        let stream = client.list_objects(owner);
        let objects: Vec<_> = futures::StreamExt::collect(stream).await;

        // Should have fetched 5 objects across 2 pages (3 + 2)
        assert_eq!(objects.len(), 5);
        assert_eq!(call_count.load(Ordering::SeqCst), 2);

        for result in objects {
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_list_objects_partial_consumption() {
        let mock_server = MockServer::start().await;
        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(move |_req: &wiremock::Request| {
                let count = call_count_clone.fetch_add(1, Ordering::SeqCst);
                match count {
                    // Page 1: 3 objects
                    0 => ResponseTemplate::new(200).set_body_json(serde_json::json!({
                        "data": {
                            "objects": {
                                "pageInfo": {
                                    "hasNextPage": true,
                                    "endCursor": "cursor1"
                                },
                                "nodes": [
                                    { "objectBcs": test_object_bcs() },
                                    { "objectBcs": test_object_bcs() },
                                    { "objectBcs": test_object_bcs() }
                                ]
                            }
                        }
                    })),
                    // Page 2: 2 objects
                    1 => ResponseTemplate::new(200).set_body_json(serde_json::json!({
                        "data": {
                            "objects": {
                                "pageInfo": {
                                    "hasNextPage": false,
                                    "endCursor": null
                                },
                                "nodes": [
                                    { "objectBcs": test_object_bcs() },
                                    { "objectBcs": test_object_bcs() }
                                ]
                            }
                        }
                    })),
                    _ => panic!("unexpected page request"),
                }
            })
            .mount(&mock_server)
            .await;

        let client = Client::new(mock_server.uri());
        let owner: Address = "0x1".parse().unwrap();

        // Only take 3 objects out of 5 available
        let stream = client.list_objects(owner).take(3);
        let objects: Vec<_> = stream.collect().await;

        // Should have only fetched 3 objects from the first page
        assert_eq!(objects.len(), 3);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        for result in objects {
            assert!(result.is_ok());
        }
    }
}
