// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::query_types::{schema, Address, Base64, Uint53};

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "DryRunArgs")]
pub struct DryRunQuery {
    #[arguments(txBytes: $tx_bytes, skipChecks: $skip_checks, txMeta: $tx_meta)]
    pub dry_run_transaction_block: DryRunResult,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DryRunArgs {
    pub tx_bytes: String,
    pub skip_checks: bool,
    pub tx_meta: Option<TransactionMetadata>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionMetadata")]
pub struct TransactionMetadata {
    pub gas_budget: Option<Uint53>,
    pub gas_objects: Option<Vec<ObjectRef>>,
    pub gas_price: Option<Uint53>,
    pub gas_sponsor: Option<Address>,
    pub sender: Option<Address>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "ObjectRef")]
pub struct ObjectRef {
    address: Address,
    digest: String,
    version: Uint53,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "DryRunResult")]
pub struct DryRunResult {
    pub error: Option<String>,
    pub transaction: Option<TransactionBlock>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlock")]
pub struct TransactionBlock {
    pub bcs: Option<Base64>,
}
