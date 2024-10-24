// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use base64ct::Encoding;
use sui_types::types::TypeTag;

use crate::query_types::schema;
use crate::query_types::Address;
use crate::query_types::Base64;
use crate::query_types::JsonValue;
use crate::query_types::MoveObjectContents;
use crate::query_types::MoveValue;
use crate::query_types::PageInfo;
use crate::DynamicFieldOutput;

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

#[derive(cynic::QueryVariables, Debug)]
pub struct DynamicFieldsQueryArgs {
    pub address: Address,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DynamicFieldConnectionArgs<'a> {
    pub address: Address,
    pub after: Option<&'a str>,
    pub before: Option<&'a str>,
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
    MoveObject(MoveObjectContents),
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

    /// Return the typename and bcs of this dynamic field value.
    pub fn type_bcs(&self) -> Option<(String, Vec<u8>)> {
        match self {
            DynamicFieldValue::MoveObject(mo) => mo
                .contents
                .as_ref()
                .map(|o| (o.type_.repr.clone(), o.bcs.0.clone().into())),
            DynamicFieldValue::MoveValue(mv) => {
                Some((mv.type_.repr.clone(), mv.bcs.0.clone().into()))
            }
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

impl TryFrom<DynamicField> for DynamicFieldOutput {
    type Error = anyhow::Error;

    fn try_from(val: DynamicField) -> Result<Self, Self::Error> {
        let typetag = TypeTag::from_str(
            val.name
                .as_ref()
                .expect("There should be a name in this dynamic field")
                .type_
                .repr
                .as_str(),
        )
        .map_err(|_| anyhow::anyhow!("Invalid TypeTag"))?;
        Ok(DynamicFieldOutput {
            name: crate::DynamicFieldName {
                type_: typetag,
                bcs: base64ct::Base64::decode_vec(val.name.as_ref().unwrap().bcs.0.as_ref())
                    .unwrap(),
                json: val.name.as_ref().unwrap().json.clone(),
            },
            value_as_json: val.field_value_json(),
            value: val.value.and_then(|x| x.type_bcs()),
        })
    }
}
