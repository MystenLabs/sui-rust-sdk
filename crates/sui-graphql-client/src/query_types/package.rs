// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::query_types::schema;
use crate::query_types::Address;
use crate::query_types::Base64;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "PackageQueryArgs")]
pub struct PackageQuery {
    #[arguments(address: $address, version: $version)]
    pub package: Option<MovePackage>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "MovePackage")]
pub struct MovePackage {
    pub bcs: Option<Base64>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct PackageQueryArgs {
    pub address: Address,
    pub version: Option<u64>,
}
