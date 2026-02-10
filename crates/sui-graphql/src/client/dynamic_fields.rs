//! Dynamic field related convenience methods.

use futures::Stream;
use serde::Deserialize;
use serde::Serialize;
use sui_graphql_macros::Response;
use sui_sdk_types::Address;
use sui_sdk_types::TypeTag;

use super::Client;
use crate::bcs::Bcs;
use crate::error::Error;
use crate::move_value::MoveValue;
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

/// A dynamic field value that handles the MoveValue/MoveObject union.
///
/// This type detects which case and extracts the MoveValue accordingly.
#[derive(Debug, Clone)]
pub struct DynamicFieldValue {
    /// Whether this is a Field or ObjectField.
    pub field_type: DynamicFieldType,
    /// The extracted Move value.
    pub value: MoveValue,
}

impl<'de> Deserialize<'de> for DynamicFieldValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = serde_json::Value::deserialize(deserializer)?;

        // Detect MoveObject by checking for "contents" field
        let is_object = raw.get("contents").is_some();
        let field_type = if is_object {
            DynamicFieldType::Object
        } else {
            DynamicFieldType::Field
        };

        // Extract MoveValue from correct location
        let move_value_json = if is_object {
            raw.get("contents").cloned().unwrap_or_default()
        } else {
            raw
        };

        // Use MoveValue's from_value to deserialize
        let value = MoveValue::from_value(move_value_json).map_err(serde::de::Error::custom)?;

        Ok(DynamicFieldValue { field_type, value })
    }
}

/// A dynamic field entry with its name and value.
#[derive(Debug, Clone, Response)]
#[response(root_type = "DynamicField")]
#[non_exhaustive]
pub struct DynamicField {
    /// The field name (includes type_tag and optional json/bcs).
    #[field(path = "name")]
    pub name: MoveValue,
    /// The field value (includes field_type and the underlying MoveValue).
    #[field(path = "value")]
    pub value: DynamicFieldValue,
}

impl Client {
    /// List all dynamic fields on an object as a paginated stream.
    ///
    /// Returns a stream that yields each dynamic field entry with its name and value.
    /// The stream handles pagination automatically.
    pub fn get_dynamic_fields(
        &self,
        parent: Address,
    ) -> impl Stream<Item = Result<DynamicField, Error>> + '_ {
        let client = self.clone();
        paginate(move |cursor| {
            let client = client.clone();
            async move {
                client
                    .fetch_dynamic_fields_page(parent, cursor.as_deref())
                    .await
            }
        })
    }

    /// Get a single dynamic field by name.
    ///
    /// Returns `None` if the dynamic field does not exist.
    pub async fn get_dynamic_field<N: Serialize>(
        &self,
        parent: Address,
        name_type: TypeTag,
        name: Bcs<N>,
    ) -> Result<Option<DynamicField>, Error> {
        #[derive(Response)]
        struct DynamicFieldResponse {
            #[field(path = "object.dynamicField")]
            field: Option<DynamicField>,
        }

        const QUERY: &str = r#"
            fragment MoveValueFields on MoveValue {
                type { repr }
                json
                bcs
            }
            query($parent: SuiAddress!, $name: DynamicFieldName!) {
                object(address: $parent) {
                    dynamicField(name: $name) {
                        name { ...MoveValueFields }
                        value {
                            ... on MoveValue { ...MoveValueFields }
                            ... on MoveObject {
                                contents { ...MoveValueFields }
                            }
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "parent": parent,
            "name": {
                "type": name_type.to_string(),
                "bcs": name,
            },
        });

        let response = self.query::<DynamicFieldResponse>(QUERY, variables).await?;
        Ok(response.into_data().and_then(|d| d.field))
    }

    /// Get a single dynamic object field by name.
    ///
    /// Returns `None` if the dynamic object field does not exist.
    pub async fn get_dynamic_object_field<N: Serialize>(
        &self,
        parent: Address,
        name_type: TypeTag,
        name: Bcs<N>,
    ) -> Result<Option<DynamicField>, Error> {
        #[derive(Response)]
        struct DynamicObjectFieldResponse {
            #[field(path = "object.dynamicObjectField")]
            field: Option<DynamicField>,
        }

        const QUERY: &str = r#"
            fragment MoveValueFields on MoveValue {
                type { repr }
                json
                bcs
            }
            query($parent: SuiAddress!, $name: DynamicFieldName!) {
                object(address: $parent) {
                    dynamicObjectField(name: $name) {
                        name { ...MoveValueFields }
                        value {
                            ... on MoveValue { ...MoveValueFields }
                            ... on MoveObject {
                                contents { ...MoveValueFields }
                            }
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "parent": parent,
            "name": {
                "type": name_type.to_string(),
                "bcs": name,
            },
        });

        let response = self
            .query::<DynamicObjectFieldResponse>(QUERY, variables)
            .await?;
        Ok(response.into_data().and_then(|d| d.field))
    }

    /// Fetch a single page of dynamic fields.
    async fn fetch_dynamic_fields_page(
        &self,
        parent: Address,
        cursor: Option<&str>,
    ) -> Result<Page<DynamicField>, Error> {
        #[derive(Response)]
        struct Response {
            #[field(path = "object.dynamicFields.nodes[]")]
            nodes: Option<Vec<DynamicField>>,
            #[field(path = "object.dynamicFields.pageInfo")]
            page_info: Option<PageInfo>,
        }

        const QUERY: &str = r#"
            fragment MoveValueFields on MoveValue {
                type { repr }
                json
                bcs
            }
            query($parent: SuiAddress!, $cursor: String) {
                object(address: $parent) {
                    dynamicFields(after: $cursor) {
                        nodes {
                            name { ...MoveValueFields }
                            value {
                                ... on MoveValue { ...MoveValueFields }
                                ... on MoveObject {
                                    contents { ...MoveValueFields }
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

        let items = data.nodes.unwrap_or_default();

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
    use sui_sdk_types::TypeTag;
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

        let parent: Address = "0x123".parse().unwrap();
        let mut stream = pin!(client.get_dynamic_fields(parent));
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
                                        "json": "123",
                                        "bcs": null
                                    },
                                    "value": {
                                        "type": { "repr": "0x2::coin::Coin<0x2::sui::SUI>" },
                                        "json": { "balance": "1000" },
                                        "bcs": null
                                    }
                                },
                                {
                                    "name": {
                                        "type": { "repr": "0x2::kiosk::Listing" },
                                        "json": { "id": "0xabc" },
                                        "bcs": null
                                    },
                                    "value": {
                                        "contents": {
                                            "type": { "repr": "0x2::kiosk::Item" },
                                            "json": { "price": "500" },
                                            "bcs": null
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

        let parent: Address = "0x123".parse().unwrap();
        let mut stream = pin!(client.get_dynamic_fields(parent));

        // First field - MoveValue (no "contents" field)
        let field1 = stream.next().await.unwrap().unwrap();
        assert_eq!(field1.name.type_tag, TypeTag::U64);
        assert_eq!(
            &field1.name.json.as_ref().unwrap().0,
            &serde_json::json!("123")
        );
        assert_eq!(field1.value.field_type, DynamicFieldType::Field);
        assert_eq!(
            &field1.value.value.json.as_ref().unwrap().0,
            &serde_json::json!({ "balance": "1000" })
        );
        assert_eq!(
            field1.value.value.type_tag,
            "0x2::coin::Coin<0x2::sui::SUI>".parse::<TypeTag>().unwrap()
        );

        // Second field - MoveObject (has "contents" field)
        let field2 = stream.next().await.unwrap().unwrap();
        assert_eq!(
            field2.name.type_tag,
            "0x2::kiosk::Listing".parse::<TypeTag>().unwrap()
        );
        assert_eq!(field2.value.field_type, DynamicFieldType::Object);
        assert_eq!(
            &field2.value.value.json.as_ref().unwrap().0,
            &serde_json::json!({ "price": "500" })
        );
        assert_eq!(
            field2.value.value.type_tag,
            "0x2::kiosk::Item".parse::<TypeTag>().unwrap()
        );

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

        let parent: Address = "0x999".parse().unwrap();
        let mut stream = pin!(client.get_dynamic_fields(parent));
        let result = stream.next().await;
        assert!(result.is_none());
    }
}
