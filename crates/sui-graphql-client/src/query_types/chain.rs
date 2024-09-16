// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use crate::query_types::schema;

// ===========================================================================
// Chain Identifier Query
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query")]
pub struct ChainIdentifierQuery {
    pub chain_identifier: String,
}
