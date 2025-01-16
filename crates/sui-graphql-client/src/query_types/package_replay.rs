// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_types::Address;

use crate::query_types::schema;
use crate::query_types::Base64;
use crate::query_types::PageInfo;
use crate::query_types::TransactionBlock;

// ===========================================================================
// PackagesVersions
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "PackageVersionsArgs"
)]
pub struct PackageVersionsWithEpochDataQuery {
    #[arguments(address: $address, after: $after, first: $first, last: $last, before: $before, filter:$filter)]
    pub package_versions: MovePackageConnection,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct PackageVersionsArgs<'a> {
    pub address: Address,
    pub after: Option<&'a str>,
    pub first: Option<i32>,
    pub last: Option<i32>,
    pub before: Option<&'a str>,
    pub filter: Option<MovePackageVersionFilter>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "MovePackageVersionFilter")]
pub struct MovePackageVersionFilter {
    pub after_version: Option<u64>,
    pub before_version: Option<u64>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "MovePackage")]
pub struct MovePackage {
    pub version: u64,
    pub package_bcs: Option<Base64>,
    pub previous_transaction_block: Option<TransactionBlock>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "MovePackageConnection")]
pub struct MovePackageConnection {
    pub nodes: Vec<MovePackage>,
    pub page_info: PageInfo,
}
