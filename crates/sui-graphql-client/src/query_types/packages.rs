// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_types::types::Address;

use crate::query_types::schema;
use crate::query_types::Base64;
use crate::query_types::PageInfo;

// ===========================================================================
// Latest Package
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "LatestPackageArgs"
)]
pub struct LatestPackageQuery {
    #[arguments(address: $address)]
    pub latest_package: Option<MovePackage>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct LatestPackageArgs {
    pub address: Address,
}

// ===========================================================================
// Package By Name
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "PackageByNameArgs"
)]
pub struct PackageByNameQuery {
    #[arguments(name: "")]
    pub package_by_name: Option<MovePackage>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct PackageByNameArgs<'a> {
    pub name: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "MovePackage")]
pub struct MovePackage {
    pub bcs: Option<Base64>,
}

/// ===========================================================================
/// Packages
/// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "PackagesQueryArgs"
)]
pub struct PackagesQuery {
    #[arguments(after: $after, before: $before, filter: $filter, first: $first, last: $last)]
    pub packages: MovePackageConnection,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct PackagesQueryArgs<'a> {
    pub after: Option<&'a str>,
    pub before: Option<&'a str>,
    pub filter: Option<PackageCheckpointFilter>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "MovePackageCheckpointFilter")]
pub struct PackageCheckpointFilter {
    pub after_checkpoint: Option<u64>,
    pub before_checkpoint: Option<u64>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "MovePackageConnection")]
pub struct MovePackageConnection {
    pub nodes: Vec<MovePackage>,
    pub page_info: PageInfo,
}
