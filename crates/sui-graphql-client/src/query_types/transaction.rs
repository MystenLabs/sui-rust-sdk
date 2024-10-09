// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::query_types::schema;
use crate::query_types::Address;
use crate::query_types::Base64;
use crate::query_types::PageInfo;

// ===========================================================================
// Transaction Block(s) Queries
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "TransactionBlockArgs"
)]
pub struct TransactionBlockQuery {
    #[arguments(digest: $digest)]
    pub transaction_block: Option<TransactionBlock>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "TransactionBlocksQueryArgs"
)]
pub struct TransactionBlocksQuery {
    #[arguments(first: $first, after: $after, last: $last, before: $before, filter: $filter)]
    pub transaction_blocks: TransactionBlockConnection,
}

// ===========================================================================
// Transaction Block(s) Query Args
// ===========================================================================

#[derive(cynic::QueryVariables, Debug)]
pub struct TransactionBlockArgs {
    pub digest: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct TransactionBlocksQueryArgs {
    pub first: Option<i32>,
    pub after: Option<String>,
    pub last: Option<i32>,
    pub before: Option<String>,
    pub filter: Option<TransactionsFilter>,
}

// ===========================================================================
// Transaction Block(s) Types
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlock")]
pub struct TransactionBlock {
    pub bcs: Option<Base64>,
    pub effects: Option<TransactionBlockEffects>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlockEffects")]
pub struct TransactionBlockEffects {
    pub bcs: Option<Base64>,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "TransactionBlockKindInput",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum TransactionBlockKindInput {
    SystemTx,
    ProgrammableTx,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlockFilter")]
pub struct TransactionsFilter {
    pub function: Option<String>,
    pub kind: Option<TransactionBlockKindInput>,
    pub at_checkpoint: Option<u64>,
    pub before_checkpoint: Option<u64>,
    pub changed_object: Option<Address>,
    pub input_object: Option<Address>,
    pub recv_address: Option<Address>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlockConnection")]
pub struct TransactionBlockConnection {
    pub nodes: Vec<TransactionBlock>,
    pub page_info: PageInfo,
}
