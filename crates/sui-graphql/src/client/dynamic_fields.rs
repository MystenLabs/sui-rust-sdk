//! Dynamic field related convenience methods.

use futures::Stream;
use serde::Serialize;
use sui_graphql_macros::Response;
use sui_sdk_types::Address;
use sui_sdk_types::TypeTag;

use super::Client;
use crate::bcs::Bcs;
use crate::error::Error;
use crate::move_value::MoveObject;
use crate::move_value::MoveValue;
use crate::pagination::Page;
use crate::pagination::PageInfo;
use crate::pagination::paginate;

/// The format to fetch for Move values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// JSON representation.
    Json,
    /// BCS (Binary Canonical Serialization) representation.
    Bcs,
}

// ============================================================================
// Request builders
// ============================================================================

/// Builder for listing dynamic fields on an object.
pub struct DynamicFieldsRequest<'a> {
    client: &'a Client,
    parent: Address,
    formats: Vec<Format>,
}

impl<'a> DynamicFieldsRequest<'a> {
    /// Add a format to fetch. Can be called multiple times.
    /// If not called, defaults to BCS.
    pub fn format(mut self, f: Format) -> Self {
        if !self.formats.contains(&f) {
            self.formats.push(f);
        }
        self
    }

    /// Execute the request and return a stream of dynamic fields.
    pub fn stream(self) -> impl Stream<Item = Result<DynamicField, Error>> + 'a {
        let client = self.client.clone();
        let formats = self.formats;
        let parent = self.parent;

        paginate(move |cursor| {
            let client = client.clone();
            let formats = formats.clone();
            async move {
                client
                    .fetch_dynamic_fields_page_with_formats(parent, cursor.as_deref(), &formats)
                    .await
            }
        })
    }
}

/// Builder for fetching a single dynamic field by name.
pub struct DynamicFieldRequest<'a, N> {
    client: &'a Client,
    parent: Address,
    name_type: TypeTag,
    name: Bcs<N>,
    field_type: DynamicFieldType,
    formats: Vec<Format>,
}

impl<'a, N: Serialize> DynamicFieldRequest<'a, N> {
    /// Add a format to fetch. Can be called multiple times.
    /// If not called, defaults to BCS.
    pub fn format(mut self, f: Format) -> Self {
        if !self.formats.contains(&f) {
            self.formats.push(f);
        }
        self
    }

    /// Execute the request and return the dynamic field if found.
    pub async fn fetch(self) -> Result<Option<DynamicField>, Error> {
        self.client
            .fetch_single_dynamic_field(
                self.parent,
                self.name_type,
                self.name,
                self.field_type,
                &self.formats,
            )
            .await
    }
}

// ============================================================================
// Dynamic field types
// ============================================================================

/// The type of a dynamic field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DynamicFieldType {
    /// A regular dynamic field (value is wrapped, not accessible by ID).
    Field,
    /// A dynamic object field (child object remains accessible by ID).
    Object,
}

/// The value of a dynamic field, dispatched by `__typename`.
#[derive(Debug, Clone, Response)]
#[response(root_type = "DynamicFieldValue")]
pub enum DynamicFieldValue {
    MoveValue(MoveValue),
    MoveObject(MoveObject),
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
    /// Create a request builder for listing dynamic fields on an object.
    pub fn dynamic_fields(&self, parent: Address) -> DynamicFieldsRequest<'_> {
        DynamicFieldsRequest {
            client: self,
            parent,
            formats: vec![],
        }
    }

    /// Create a request builder for fetching a single dynamic field by name.
    pub fn dynamic_field<N: Serialize>(
        &self,
        parent: Address,
        name_type: TypeTag,
        name: Bcs<N>,
    ) -> DynamicFieldRequest<'_, N> {
        DynamicFieldRequest {
            client: self,
            parent,
            name_type,
            name,
            field_type: DynamicFieldType::Field,
            formats: vec![],
        }
    }

    /// Create a request builder for fetching a single dynamic object field by name.
    pub fn dynamic_object_field<N: Serialize>(
        &self,
        parent: Address,
        name_type: TypeTag,
        name: Bcs<N>,
    ) -> DynamicFieldRequest<'_, N> {
        DynamicFieldRequest {
            client: self,
            parent,
            name_type,
            name,
            field_type: DynamicFieldType::Object,
            formats: vec![],
        }
    }

    /// Fetch a page of dynamic fields with format selection.
    async fn fetch_dynamic_fields_page_with_formats(
        &self,
        parent: Address,
        cursor: Option<&str>,
        formats: &[Format],
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
                json @include(if: $withJson)
                bcs @include(if: $withBcs)
            }
            query($parent: SuiAddress!, $cursor: String, $withJson: Boolean!, $withBcs: Boolean!) {
                object(address: $parent) {
                    dynamicFields(after: $cursor) {
                        nodes {
                            name { ...MoveValueFields }
                            value {
                                __typename
                                ... on MoveValue { ...MoveValueFields }
                                ... on MoveObject {
                                    address
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

        let with_json = formats.contains(&Format::Json);
        let with_bcs = formats.is_empty() || formats.contains(&Format::Bcs);
        let variables = serde_json::json!({
            "parent": parent,
            "cursor": cursor,
            "withJson": with_json,
            "withBcs": with_bcs,
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

        Ok(Page {
            items: data.nodes.unwrap_or_default(),
            has_next_page: page_info.has_next_page,
            end_cursor: page_info.end_cursor,
        })
    }

    /// Fetch a single dynamic field with format selection.
    async fn fetch_single_dynamic_field<N: Serialize>(
        &self,
        parent: Address,
        name_type: TypeTag,
        name: Bcs<N>,
        field_type: DynamicFieldType,
        formats: &[Format],
    ) -> Result<Option<DynamicField>, Error> {
        #[derive(Response)]
        struct DynamicFieldResponse {
            #[field(path = "object.dynamicField")]
            field: Option<DynamicField>,
        }

        #[derive(Response)]
        struct DynamicObjectFieldResponse {
            #[field(path = "object.dynamicObjectField")]
            field: Option<DynamicField>,
        }

        const DYNAMIC_FIELD_QUERY: &str = r#"
            fragment MoveValueFields on MoveValue {
                type { repr }
                json @include(if: $withJson)
                bcs @include(if: $withBcs)
            }
            query($parent: SuiAddress!, $name: DynamicFieldName!, $withJson: Boolean!, $withBcs: Boolean!) {
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

        const DYNAMIC_OBJECT_FIELD_QUERY: &str = r#"
            fragment MoveValueFields on MoveValue {
                type { repr }
                json @include(if: $withJson)
                bcs @include(if: $withBcs)
            }
            query($parent: SuiAddress!, $name: DynamicFieldName!, $withJson: Boolean!, $withBcs: Boolean!) {
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

        let with_json = formats.contains(&Format::Json);
        let with_bcs = formats.is_empty() || formats.contains(&Format::Bcs);
        let variables = serde_json::json!({
            "parent": parent,
            "name": {
                "type": name_type.to_string(),
                "bcs": name,
            },
            "withJson": with_json,
            "withBcs": with_bcs,
        });

        match field_type {
            DynamicFieldType::Field => {
                let response = self
                    .query::<DynamicFieldResponse>(DYNAMIC_FIELD_QUERY, variables)
                    .await?;
                Ok(response.into_data().and_then(|d| d.field))
            }
            DynamicFieldType::Object => {
                let response = self
                    .query::<DynamicObjectFieldResponse>(DYNAMIC_OBJECT_FIELD_QUERY, variables)
                    .await?;
                Ok(response.into_data().and_then(|d| d.field))
            }
        }
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
    async fn test_dynamic_fields_empty() {
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
        let mut stream = pin!(client.dynamic_fields(parent).stream());
        let result = stream.next().await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_dynamic_fields_with_json_format() {
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
                                        "address": "0x0000000000000000000000000000000000000000000000000000000000000def",
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
        let mut stream = pin!(client.dynamic_fields(parent).format(Format::Json).stream());

        // First field - MoveValue
        let field1 = stream.next().await.unwrap().unwrap();
        assert_eq!(field1.name.type_tag, TypeTag::U64);
        assert!(field1.name.json.is_some());
        assert!(field1.name.bcs.is_none()); // BCS not requested
        assert!(matches!(field1.value, DynamicFieldValue::MoveValue(_)));

        // Second field - MoveObject
        let field2 = stream.next().await.unwrap().unwrap();
        assert_eq!(
            field2.name.type_tag,
            "0x2::kiosk::Listing".parse::<TypeTag>().unwrap()
        );
        assert!(matches!(field2.value, DynamicFieldValue::MoveObject(_)));

        // No more fields
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_dynamic_fields_with_default_bcs() {
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
                                        "bcs": "ewAAAAAAAAA="
                                    },
                                    "value": {
                                        "__typename": "MoveValue",
                                        "type": { "repr": "bool" },
                                        "bcs": "AQ=="
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
        // Default - no format specified
        let mut stream = pin!(client.dynamic_fields(parent).stream());

        let field = stream.next().await.unwrap().unwrap();
        assert_eq!(field.name.type_tag, TypeTag::U64);
        assert!(field.name.bcs.is_some());
        assert!(field.name.json.is_none()); // JSON not requested
    }

    #[tokio::test]
    async fn test_dynamic_fields_object_not_found() {
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
        let mut stream = pin!(client.dynamic_fields(parent).stream());
        let result = stream.next().await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_dynamic_field_fetch() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "object": {
                        "dynamicField": {
                            "name": {
                                "type": { "repr": "u64" },
                                "json": "123",
                                "bcs": "ewAAAAAAAAA="
                            },
                            "value": {
                                "__typename": "MoveValue",
                                "type": { "repr": "bool" },
                                "json": true,
                                "bcs": "AQ=="
                            }
                        }
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();
        let parent: Address = "0x123".parse().unwrap();
        let name_type: TypeTag = "u64".parse().unwrap();

        let field = client
            .dynamic_field(parent, name_type, Bcs(123u64))
            .format(Format::Json)
            .format(Format::Bcs)
            .fetch()
            .await
            .unwrap();

        assert!(field.is_some());
        let field = field.unwrap();
        assert_eq!(field.name.type_tag, TypeTag::U64);
        assert!(field.name.json.is_some());
        assert!(field.name.bcs.is_some());
        assert!(matches!(field.value, DynamicFieldValue::MoveValue(_)));
    }

    #[tokio::test]
    async fn test_dynamic_field_object_type() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "object": {
                        "dynamicObjectField": {
                            "name": {
                                "type": { "repr": "0x1::string::String" },
                                "json": "my_key"
                            },
                            "value": {
                                "__typename": "MoveObject",
                                "address": "0x0000000000000000000000000000000000000000000000000000000000000abc",
                                "contents": {
                                    "type": { "repr": "0x2::coin::Coin<0x2::sui::SUI>" },
                                    "json": { "balance": "1000" }
                                }
                            }
                        }
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();
        let parent: Address = "0x123".parse().unwrap();
        let name_type: TypeTag = "0x1::string::String".parse().unwrap();

        let field = client
            .dynamic_object_field(parent, name_type, Bcs("my_key"))
            .format(Format::Json)
            .fetch()
            .await
            .unwrap();

        assert!(field.is_some());
        let field = field.unwrap();
        let DynamicFieldValue::MoveObject(ref obj) = field.value else {
            panic!("expected MoveObject variant");
        };
        assert_eq!(
            obj.contents.type_tag,
            "0x2::coin::Coin<0x2::sui::SUI>".parse::<TypeTag>().unwrap()
        );
    }

    #[tokio::test]
    async fn test_dynamic_field_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "object": {
                        "dynamicField": null
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();
        let parent: Address = "0x123".parse().unwrap();
        let name_type: TypeTag = "u64".parse().unwrap();

        let field = client
            .dynamic_field(parent, name_type, Bcs(999u64))
            .fetch()
            .await
            .unwrap();

        assert!(field.is_none());
    }
}
