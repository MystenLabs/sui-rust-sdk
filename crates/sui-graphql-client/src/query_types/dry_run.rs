// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_types::types::ObjectReference;

use crate::query_types::schema;
use crate::query_types::Address;

use super::transaction::TxBlockEffects;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "DryRunArgs")]
pub struct DryRunQuery {
    #[arguments(txBytes: $tx_bytes, skipChecks: $skip_checks, txMeta: $tx_meta)]
    pub dry_run_transaction_block: DryRunResult,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "DryRunResult")]
pub struct DryRunResult {
    pub error: Option<String>,
    pub transaction: Option<TxBlockEffects>,
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
    pub gas_budget: Option<u64>,
    pub gas_objects: Option<Vec<ObjectRef>>,
    pub gas_price: Option<u64>,
    pub gas_sponsor: Option<Address>,
    pub sender: Option<Address>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "ObjectRef")]
pub struct ObjectRef {
    pub address: Address,
    pub digest: String,
    pub version: u64,
}

impl From<ObjectReference> for ObjectRef {
    fn from(value: ObjectReference) -> Self {
        let address: Address = (*value.object_id()).into();
        ObjectRef {
            address,
            version: value.version(),
            digest: value.digest().to_string(),
        }
    }
}
