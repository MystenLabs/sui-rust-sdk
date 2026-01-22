//! Dynamic field related convenience methods.

use futures::Stream;
use sui_graphql_macros::Response;

use super::Client;
use crate::error::Error;
use crate::pagination::Page;
use crate::pagination::PageInfo;
use crate::pagination::paginate;

/// The type of a dynamic field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicFieldType {
    /// A regular dynamic field (value is wrapped, not accessible by ID).
    Field,
    /// A dynamic object field (child object remains accessible by ID).
    Object,
}

/// A dynamic field entry with its name and value.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct DynamicFieldEntry {
    /// The field name as JSON.
    pub name: serde_json::Value,
    /// The field name's Move type (e.g., "u64", "0x2::kiosk::Listing").
    pub name_type: String,
    /// The field value as JSON.
    pub value: serde_json::Value,
    /// Whether this is a Field or ObjectField.
    pub field_type: DynamicFieldType,
}

impl Client {
    /// List all dynamic fields on an object as a paginated stream.
    ///
    /// Returns a stream that yields each dynamic field entry with its name and value.
    /// The stream handles pagination automatically.
    pub fn get_dynamic_fields(
        &self,
        parent: &str,
    ) -> impl Stream<Item = Result<DynamicFieldEntry, Error>> + '_ {
        let client = self.clone();
        let parent = parent.to_string();
        paginate(move |cursor| {
            let client = client.clone();
            let parent = parent.clone();
            async move {
                client
                    .fetch_dynamic_fields_page(&parent, cursor.as_deref())
                    .await
            }
        })
    }

    /// Fetch a single page of dynamic fields.
    async fn fetch_dynamic_fields_page(
        &self,
        parent: &str,
        cursor: Option<&str>,
    ) -> Result<Page<DynamicFieldEntry>, Error> {
        #[derive(Response)]
        struct Response {
            #[field(path = "object.dynamicFields.nodes[].name.type.repr")]
            name_types: Option<Vec<String>>,
            #[field(path = "object.dynamicFields.nodes[].name.json")]
            names: Option<Vec<serde_json::Value>>,
            #[field(path = "object.dynamicFields.nodes[].value")]
            values: Option<Vec<serde_json::Value>>,
            #[field(path = "object.dynamicFields.pageInfo")]
            page_info: Option<PageInfo>,
        }

        const QUERY: &str = r#"
            query($parent: SuiAddress!, $cursor: String) {
                object(address: $parent) {
                    dynamicFields(after: $cursor) {
                        nodes {
                            name {
                                type { repr }
                                json
                            }
                            value {
                                ... on MoveValue {
                                    __typename
                                    json
                                }
                                ... on MoveObject {
                                    __typename
                                    contents { json }
                                }
                            }
                        }
                        pageInfo {
                            hasNextPage
                            endCursor
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "parent": parent,
            "cursor": cursor,
        });

        let response = self.query::<Response>(QUERY, variables).await?;

        let Some(data) = response.into_data() else {
            return Ok(Page {
                items: vec![],
                has_next_page: false,
                end_cursor: None,
            });
        };

        let page_info = data.page_info.unwrap_or(PageInfo {
            has_next_page: false,
            end_cursor: None,
        });

        let name_types = data.name_types.unwrap_or_default();
        let names = data.names.unwrap_or_default();
        let values = data.values.unwrap_or_default();

        let items: Vec<DynamicFieldEntry> = name_types
            .into_iter()
            .zip(names)
            .zip(values)
            .map(|((name_type, name), value)| {
                // Determine field type based on the value's __typename
                let field_type = if let Some(obj) = value.as_object() {
                    if obj.get("__typename").and_then(|t| t.as_str()) == Some("MoveObject") {
                        DynamicFieldType::Object
                    } else {
                        DynamicFieldType::Field
                    }
                } else {
                    DynamicFieldType::Field
                };

                // Extract the actual value (json field for MoveValue, contents.json for MoveObject)
                let extracted_value = if let Some(obj) = value.as_object() {
                    if let Some(json) = obj.get("json") {
                        json.clone()
                    } else if let Some(contents) = obj.get("contents") {
                        contents
                            .get("json")
                            .cloned()
                            .unwrap_or(serde_json::Value::Null)
                    } else {
                        value
                    }
                } else {
                    value
                };

                DynamicFieldEntry {
                    name,
                    name_type,
                    value: extracted_value,
                    field_type,
                }
            })
            .collect();

        Ok(Page {
            items,
            has_next_page: page_info.has_next_page,
            end_cursor: page_info.end_cursor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use std::pin::pin;
    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    #[tokio::test]
    async fn test_get_dynamic_fields_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "object": {
                        "dynamicFields": {
                            "nodes": [],
                            "pageInfo": {
                                "hasNextPage": false,
                                "endCursor": null
                            }
                        }
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();

        let mut stream = pin!(client.get_dynamic_fields("0x123"));
        let result = stream.next().await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_dynamic_fields_with_values() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "object": {
                        "dynamicFields": {
                            "nodes": [
                                {
                                    "name": {
                                        "type": { "repr": "u64" },
                                        "json": "123"
                                    },
                                    "value": {
                                        "__typename": "MoveValue",
                                        "json": { "balance": "1000" }
                                    }
                                },
                                {
                                    "name": {
                                        "type": { "repr": "0x2::kiosk::Listing" },
                                        "json": { "id": "0xabc" }
                                    },
                                    "value": {
                                        "__typename": "MoveObject",
                                        "contents": {
                                            "json": { "price": "500" }
                                        }
                                    }
                                }
                            ],
                            "pageInfo": {
                                "hasNextPage": false,
                                "endCursor": null
                            }
                        }
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();

        let mut stream = pin!(client.get_dynamic_fields("0x123"));

        // First field - MoveValue
        let field1 = stream.next().await.unwrap().unwrap();
        assert_eq!(field1.name_type, "u64");
        assert_eq!(field1.name, serde_json::json!("123"));
        assert_eq!(field1.field_type, DynamicFieldType::Field);
        assert_eq!(field1.value, serde_json::json!({ "balance": "1000" }));

        // Second field - MoveObject
        let field2 = stream.next().await.unwrap().unwrap();
        assert_eq!(field2.name_type, "0x2::kiosk::Listing");
        assert_eq!(field2.field_type, DynamicFieldType::Object);
        assert_eq!(field2.value, serde_json::json!({ "price": "500" }));

        // No more fields
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_get_dynamic_fields_object_not_found() {
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

        let client = Client::new(&mock_server.uri()).unwrap();

        let mut stream = pin!(client.get_dynamic_fields("0xnonexistent"));
        let result = stream.next().await;
        assert!(result.is_none());
    }
}
