// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// ===========================================================================
// SuiNS Queries
// ===========================================================================

use crate::query_types::schema;
use crate::query_types::Address as SdkAddress;
use crate::query_types::Base64;
use crate::query_types::PageInfo;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "ResolveSuiNSQueryArgs"
)]
pub struct ResolveSuiNSQuery {
    #[arguments(domain: $name)]
    pub resolve_suins_address: Option<Address>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ResolveSuiNSQueryArgs<'a> {
    pub name: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct Address {
    pub address: SdkAddress,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "SuinsRegistrationsQueryArgs"
)]
pub struct SuinsRegistrationsQuery {
    #[arguments(address: $address)]
    pub owner: Option<Owner>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct SuinsRegistrationsQueryArgs<'a> {
    pub after: Option<&'a str>,
    pub before: Option<&'a str>,
    pub address: SdkAddress,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Owner")]
pub struct Owner {
    #[arguments(after: "", before: "", first: 10, last: 10)]
    pub suins_registrations: SuinsRegistrationConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "SuinsRegistrationConnection")]
pub struct SuinsRegistrationConnection {
    pub page_info: PageInfo,
    pub nodes: Vec<SuinsRegistration>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "SuinsRegistration")]
pub struct SuinsRegistration {
    pub bcs: Option<Base64>,
    pub domain: String,
}
