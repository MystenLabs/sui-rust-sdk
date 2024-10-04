// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// ===========================================================================
// Coin(s) Queries
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "CoinMetadataArgs")]
pub struct CoinMetadataQuery {
    #[arguments(coinType: $coin_type)]
    pub coin_metadata: Option<CoinMetadata>,
}

// ===========================================================================
// Coin(s) Query Args
// ===========================================================================

#[derive(cynic::QueryVariables, Debug)]
pub struct CoinMetadataArgs<'a> {
    pub coin_type: &'a str,
}

// ===========================================================================
// Types
// ===========================================================================

use crate::query_types::schema;
use crate::query_types::BigInt;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "CoinMetadata")]
pub struct CoinMetadata {
    pub decimals: Option<i32>,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub supply: Option<BigInt>,
    pub version: u64,
}
