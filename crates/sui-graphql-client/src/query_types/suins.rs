// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// ===========================================================================
// Suins Queries
// ===========================================================================

use crate::query_types::schema;
use crate::query_types::Address as SdkAddress;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "ResolveSuinsQueryArgs"
)]
pub struct ResolveSuinsQuery {
    #[arguments(domain: $name)]
    pub resolve_suins_address: Option<DomainAddress>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ResolveSuinsQueryArgs<'a> {
    pub name: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct DomainAddress {
    pub address: SdkAddress,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "DefaultSuinsNameQueryArgs"
)]
pub struct DefaultSuinsNameQuery {
    #[arguments(address: $address)]
    pub address: Option<AddressDefaultSuins>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DefaultSuinsNameQueryArgs {
    pub address: SdkAddress,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct AddressDefaultSuins {
    pub default_suins_name: Option<String>,
}
