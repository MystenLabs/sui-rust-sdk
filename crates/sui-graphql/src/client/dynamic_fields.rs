//! Dynamic field related convenience methods.

use futures::Stream;
use serde::Serialize;
use sui_graphql_macros::Response;
use sui_sdk_types::Address;
use sui_sdk_types::TypeTag;

use super::Client;
use crate::bcs::Bcs;
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

impl DynamicFieldType {
    /// Returns the GraphQL field name for this dynamic field type.
    fn graphql_field_name(&self) -> &'static str {
        match self {
            DynamicFieldType::Field => "dynamicField",
            DynamicFieldType::Object => "dynamicObjectField",
        }
    }
}

/// A dynamic field entry with its name and value.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct DynamicFieldEntry {
    /// The field name as JSON.
    pub name: serde_json::Value,
    /// The field name's Move type (e.g., `u64`, `0x2::kiosk::Listing`).
    pub name_type: TypeTag,
    /// The field value as JSON.
    pub value: serde_json::Value,
    /// The field value's Move type (e.g., `0x2::coin::Coin<0x2::sui::SUI>`).
    pub value_type: TypeTag,
    /// Whether this is a Field or ObjectField.
    pub field_type: DynamicFieldType,
}

/// Parse the value union type (MoveValue | MoveObject) from a dynamic field response.
///
/// Returns `(extracted_value, value_type, field_type)`.
fn parse_dynamic_field_value(
    value: serde_json::Value,
) -> Result<(serde_json::Value, TypeTag, DynamicFieldType), Error> {
    let Some(obj) = value.as_object() else {
        return Err(Error::MissingData("dynamic field value"));
    };

    let is_move_object = obj.get("__typename").and_then(|t| t.as_str()) == Some("MoveObject");

    let field_type = if is_move_object {
        DynamicFieldType::Object
    } else {
        DynamicFieldType::Field
    };

    // TODO(DVX-1980): The macro doesn't currently support extracting fields from within GraphQL
    // fragments (`... on Type`). Once supported, we can replace this manual JSON parsing
    // with declarative field paths like `value.type.repr` that work across union types.
    let (extracted_value, value_type_str) = if is_move_object {
        // MoveObject: value is in contents.json, type is in contents.type.repr
        let contents = obj.get("contents");
        let json = contents
            .and_then(|c| c.get("json"))
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let type_repr = contents
            .and_then(|c| c.get("type"))
            .and_then(|t| t.get("repr"))
            .and_then(|r| r.as_str());
        (json, type_repr)
    } else {
        // MoveValue: value is in json, type is in type.repr
        let json = obj.get("json").cloned().unwrap_or(serde_json::Value::Null);
        let type_repr = obj
            .get("type")
            .and_then(|t| t.get("repr"))
            .and_then(|r| r.as_str());
        (json, type_repr)
    };

    let value_type: TypeTag = value_type_str
        .ok_or(Error::MissingData("dynamic field value type"))?
        .parse()?;

    Ok((extracted_value, value_type, field_type))
}

impl Client {
    /// List all dynamic fields on an object as a paginated stream.
    ///
    /// Returns a stream that yields each dynamic field entry with its name and value.
    /// The stream handles pagination automatically.
    pub fn get_dynamic_fields(
        &self,
        parent: Address,
    ) -> impl Stream<Item = Result<DynamicFieldEntry, Error>> + '_ {
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
    ) -> Result<Option<DynamicFieldEntry>, Error> {
        self.fetch_single_dynamic_field(parent, name_type, name, DynamicFieldType::Field)
            .await
    }

    /// Get a single dynamic object field by name.
    ///
    /// Returns `None` if the dynamic object field does not exist.
    pub async fn get_dynamic_object_field<N: Serialize>(
        &self,
        parent: Address,
        name_type: TypeTag,
        name: Bcs<N>,
    ) -> Result<Option<DynamicFieldEntry>, Error> {
        self.fetch_single_dynamic_field(parent, name_type, name, DynamicFieldType::Object)
            .await
    }

    /// Fetch a single dynamic field or dynamic object field by name.
    async fn fetch_single_dynamic_field<N: Serialize>(
        &self,
        parent: Address,
        name_type: TypeTag,
        name: Bcs<N>,
        field_type: DynamicFieldType,
    ) -> Result<Option<DynamicFieldEntry>, Error> {
        let field_name = field_type.graphql_field_name();
        let query = format!(
            r#"
            fragment MoveValueFields on MoveValue {{
                type {{ repr }}
                json
            }}
            query($parent: SuiAddress!, $name: DynamicFieldName!) {{
                object(address: $parent) {{
                    {field_name}(name: $name) {{
                        name {{ ...MoveValueFields }}
                        value {{
                            __typename
                            ... on MoveValue {{ ...MoveValueFields }}
                            ... on MoveObject {{
                                contents {{ ...MoveValueFields }}
                            }}
                        }}
                    }}
                }}
            }}
        "#
        );

        let variables = serde_json::json!({
            "parent": parent,
            "name": {
                "type": name_type.to_string(),
                "bcs": name,
            },
        });

        let response = self.query::<serde_json::Value>(&query, variables).await?;

        let Some(data) = response.into_data() else {
            return Ok(None);
        };

        // Extract: data.object.<field_name>.{name, value}
        let field = data.get("object").and_then(|o| o.get(field_name));

        let Some(field) = field else {
            return Ok(None);
        };

        let name_obj = field.get("name");
        let value = field.get("value").cloned();

        let (Some(name_obj), Some(value)) = (name_obj, value) else {
            return Ok(None);
        };

        let name_type: TypeTag = name_obj
            .get("type")
            .and_then(|t| t.get("repr"))
            .and_then(|r| r.as_str())
            .ok_or(Error::MissingData("dynamic field name type"))?
            .parse()?;

        let name = name_obj
            .get("json")
            .cloned()
            .ok_or(Error::MissingData("dynamic field name"))?;

        let (extracted_value, value_type, field_type) = parse_dynamic_field_value(value)?;

        Ok(Some(DynamicFieldEntry {
            name,
            name_type,
            value: extracted_value,
            value_type,
            field_type,
        }))
    }

    /// Fetch a single page of dynamic fields.
    async fn fetch_dynamic_fields_page(
        &self,
        parent: Address,
        cursor: Option<&str>,
    ) -> Result<Page<DynamicFieldEntry>, Error> {
        /// Extracts data from a single DynamicField node using `root_type = "DynamicField"`.
        #[derive(Response)]
        #[response(root_type = "DynamicField")]
        struct DynamicFieldData {
            #[field(path = "name.type.repr")]
            name_type: TypeTag,
            #[field(path = "name.json")]
            name: serde_json::Value,
            #[field(path = "value")]
            value: serde_json::Value,
        }

        #[derive(Response)]
        struct Response {
            #[field(path = "object.dynamicFields.nodes[]")]
            nodes: Option<Vec<DynamicFieldData>>,
            #[field(path = "object.dynamicFields.pageInfo")]
            page_info: Option<PageInfo>,
        }

        const QUERY: &str = r#"
            fragment MoveValueFields on MoveValue {
                type { repr }
                json
            }
            query($parent: SuiAddress!, $cursor: String) {
                object(address: $parent) {
                    dynamicFields(after: $cursor) {
                        nodes {
                            name { ...MoveValueFields }
                            value {
                                __typename
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

        let items: Vec<DynamicFieldEntry> = data
            .nodes
            .unwrap_or_default()
            .into_iter()
            .map(|field_data| {
                let (extracted_value, value_type, field_type) =
                    parse_dynamic_field_value(field_data.value)?;

                Ok(DynamicFieldEntry {
                    name: field_data.name,
                    name_type: field_data.name_type,
                    value: extracted_value,
                    value_type,
                    field_type,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

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
                                        "json": "123"
                                    },
                                    "value": {
                                        "__typename": "MoveValue",
                                        "type": { "repr": "0x2::coin::Coin<0x2::sui::SUI>" },
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
                                            "type": { "repr": "0x2::kiosk::Item" },
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

        let parent: Address = "0x123".parse().unwrap();
        let mut stream = pin!(client.get_dynamic_fields(parent));

        // First field - MoveValue
        let field1 = stream.next().await.unwrap().unwrap();
        assert_eq!(field1.name_type, TypeTag::U64);
        assert_eq!(field1.name, serde_json::json!("123"));
        assert_eq!(field1.field_type, DynamicFieldType::Field);
        assert_eq!(field1.value, serde_json::json!({ "balance": "1000" }));
        assert_eq!(
            field1.value_type,
            "0x2::coin::Coin<0x2::sui::SUI>".parse::<TypeTag>().unwrap()
        );

        // Second field - MoveObject
        let field2 = stream.next().await.unwrap().unwrap();
        assert_eq!(
            field2.name_type,
            "0x2::kiosk::Listing".parse::<TypeTag>().unwrap()
        );
        assert_eq!(field2.field_type, DynamicFieldType::Object);
        assert_eq!(field2.value, serde_json::json!({ "price": "500" }));
        assert_eq!(
            field2.value_type,
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
