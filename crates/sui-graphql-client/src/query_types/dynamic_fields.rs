// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use base64ct::Encoding;

use crate::query_types::schema;
use crate::query_types::Address;
use crate::query_types::Base64;
use crate::query_types::JsonValue;
use crate::query_types::MoveObject;
use crate::query_types::MoveValue;
use crate::query_types::PageInfo;
use crate::DynamicFieldOutput;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "DynamicFieldConnectionArgs"
)]
pub struct DynamicFieldsQuery {
    #[arguments(address: $address )]
    pub object: Option<Object>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "DynamicFieldConnectionArgs"
)]
pub struct DynamicFieldsOwnerQuery {
    #[arguments(address: $address)]
    pub owner: Option<ObjectOwner>,
}
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Owner",
    variables = "DynamicFieldConnectionArgs"
)]
pub struct ObjectOwner {
    #[arguments(after: $after, before: $before, first: $first, last: $last)]
    pub dynamic_fields: DynamicFieldConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "DynamicFieldArgs")]
pub struct DynamicFieldQuery {
    #[arguments(address: $address)]
    pub object: Option<ObjectField>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Object",
    variables = "DynamicFieldArgs"
)]
pub struct ObjectField {
    #[arguments(name: $name)]
    pub dynamic_field: Option<DynamicField>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DynamicFieldArgs {
    pub address: Address,
    pub name: DynamicFieldName,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Object",
    variables = "DynamicFieldConnectionArgs"
)]
pub struct Object {
    #[arguments(after: $after, before: $before, first: $first, last: $last)]
    pub dynamic_fields: DynamicFieldConnection,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DynamicFieldsQueryArgs {
    pub address: Address,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DynamicFieldConnectionArgs {
    pub address: Address,
    pub after: Option<String>,
    pub before: Option<String>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "DynamicFieldConnection")]
pub struct DynamicFieldConnection {
    pub nodes: Vec<DynamicField>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "DynamicField")]
pub struct DynamicField {
    pub value: Option<DynamicFieldValue>,
    pub name: Option<MoveValue>,
}

#[derive(cynic::InlineFragments, Debug)]
#[cynic(schema = "rpc", graphql_type = "DynamicFieldValue")]
pub enum DynamicFieldValue {
    MoveObject(MoveObject),
    MoveValue(MoveValue),
    #[cynic(fallback)]
    Unknown,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "DynamicFieldName")]
pub struct DynamicFieldName {
    #[cynic(rename = "type")]
    pub type_: String,
    pub bcs: Base64,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Object",
    variables = "DynamicFieldArgs"
)]
pub struct DynamicObjectField {
    #[arguments(name: $name)]
    pub dynamic_object_field: Option<DynamicField>,
}
#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "DynamicFieldArgs")]
pub struct DynamicObjectFieldQuery {
    #[arguments(address: $address)]
    pub object: Option<DynamicObjectField>,
}

impl DynamicFieldValue {
    /// Returns the JSON representation of the field value, if available.
    pub fn field_value_json(&self) -> Option<JsonValue> {
        match self {
            DynamicFieldValue::MoveObject(mo) => {
                mo.contents.as_ref().and_then(|mv| mv.json.clone())
            }
            DynamicFieldValue::MoveValue(mv) => mv.json.clone(),
            _ => None,
        }
    }
}

impl DynamicField {
    /// Returns the JSON representation of the field value, if available.
    pub fn field_value_json(&self) -> Option<JsonValue> {
        self.value.as_ref().and_then(|v| v.field_value_json())
    }
}

impl From<DynamicField> for DynamicFieldOutput {
    fn from(val: DynamicField) -> DynamicFieldOutput {
        let object = match val.value {
            Some(DynamicFieldValue::MoveObject(ref mo)) => mo
                .bcs
                .as_ref()
                .and_then(|bcs| base64ct::Base64::decode_vec(bcs.0.as_ref()).ok())
                .and_then(|o| bcs::from_bytes::<sui_types::types::Object>(o.as_ref()).ok()),
            _ => None,
        };

        DynamicFieldOutput {
            name: crate::DynamicFieldName {
                type_: val.name.as_ref().unwrap().__typename.clone(),
                bcs: base64ct::Base64::decode_vec(val.name.as_ref().unwrap().bcs.0.as_ref())
                    .unwrap(),
                json: val.name.as_ref().unwrap().json.clone(),
            },
            object,
            json: val.field_value_json(),
        }
    }
}
