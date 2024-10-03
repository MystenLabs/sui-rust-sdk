// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_types::types::ObjectReference;

use crate::query_types::schema;
use crate::query_types::Address;
use crate::query_types::Base64;
use crate::query_types::Uint53;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Result")]
pub struct GqlResult {
    pub ix: Option<i32>,
    pub cmd: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "DryRunArgs")]
pub struct DryRunQuery {
    #[arguments(txBytes: $tx_bytes, skipChecks: $skip_checks, txMeta: $tx_meta)]
    pub dry_run_transaction_block: DryRunResult,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Input")]
pub struct Input {
    pub __typename: String,
    pub ix: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "GasCoin")]
pub struct GasCoin {
    #[cynic(rename = "_")]
    pub __underscore: Option<bool>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "DryRunResult")]
pub struct DryRunResult {
    pub error: Option<String>,
    pub results: Option<Vec<DryRunEffect>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "DryRunEffect")]
pub struct DryRunEffect {
    pub mutated_references: Option<Vec<DryRunMutation>>,
    pub return_values: Option<Vec<DryRunReturn>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "DryRunReturn")]
pub struct DryRunReturn {
    pub bcs: Base64,
    #[cynic(rename = "type")]
    pub type_: MoveType,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "DryRunMutation")]
pub struct DryRunMutation {
    pub bcs: Base64,
    pub input: TransactionArgument,
    #[cynic(rename = "type")]
    pub type_: MoveType,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "MoveType")]
pub struct MoveType {
    pub repr: String,
}

#[derive(cynic::InlineFragments, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionArgument")]
pub enum TransactionArgument {
    GasCoin(GasCoin),
    Input(Input),
    Result(GqlResult),
    #[cynic(fallback)]
    Unknown,
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
    pub address: Address,
    pub digest: String,
    pub version: Uint53,
}

impl TryFrom<ObjectReference> for ObjectRef {
    type Error = anyhow::Error;

    fn try_from(value: ObjectReference) -> Result<Self, Self::Error> {
        let address: Address = (*value.object_id()).into();
        Ok(ObjectRef {
            address,
            version: Uint53(value.version()),
            digest: value.digest().to_string(),
        })
    }
}
