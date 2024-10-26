// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// ===========================================================================
// SuiNS Queries
// ===========================================================================

use crate::query_types::schema;
use crate::query_types::Address as SdkAddress;

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
