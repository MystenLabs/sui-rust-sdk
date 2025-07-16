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
use crate::query_types::Address;
use crate::query_types::BigInt;

/// The coin metadata associated with the given coin type.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "CoinMetadata")]
pub struct CoinMetadata {
    /// The CoinMetadata object ID.
    pub address: Address,
    /// The number of decimal places used to represent the token.
    pub decimals: Option<i32>,
    /// Optional description of the token, provided by the creator of the token.
    pub description: Option<String>,
    /// Icon URL of the coin.
    pub icon_url: Option<String>,
    /// Full, official name of the token.
    pub name: Option<String>,
    /// The token's identifying abbreviation.
    pub symbol: Option<String>,
    /// The overall quantity of tokens that will be issued.
    pub supply: Option<BigInt>,
    /// Version of the token.
    pub version: u64,
}
