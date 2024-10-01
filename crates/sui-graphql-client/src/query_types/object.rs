// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::query_types::schema;
use crate::query_types::Address;
use crate::query_types::Base64;
use crate::query_types::PageInfo;
use crate::query_types::Uint53;

// ===========================================================================
// Object(s) Queries
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "ObjectQueryArgs")]
pub struct ObjectQuery {
    #[arguments(address: $address, version: $version)]
    pub object: Option<Object>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "ObjectsQueryArgs")]
pub struct ObjectsQuery {
    #[arguments(after: $after, before: $before, filter: $filter, first: $first, last: $last)]
    pub objects: ObjectConnection,
}

// ===========================================================================
// Object(s) Query Args
// ===========================================================================

#[derive(cynic::QueryVariables, Debug)]
pub struct ObjectQueryArgs {
    pub address: Address,
    pub version: Option<Uint53>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ObjectsQueryArgs<'a> {
    pub after: Option<&'a str>,
    pub before: Option<&'a str>,
    pub filter: Option<ObjectFilter<'a>>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

// ===========================================================================
// Object(s) Types
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Object")]
pub struct Object {
    pub bcs: Option<Base64>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "ObjectFilter")]
pub struct ObjectFilter<'a> {
    #[cynic(rename = "type")]
    pub type_: Option<&'a str>,
    pub owner: Option<Address>,
    pub object_ids: Option<Vec<Address>>,
    pub object_keys: Option<Vec<ObjectKey>>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "ObjectKey")]
pub struct ObjectKey {
    pub object_id: Address,
    pub version: Uint53,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "ObjectConnection")]
pub struct ObjectConnection {
    pub page_info: PageInfo,
    pub nodes: Vec<Object>,
}
