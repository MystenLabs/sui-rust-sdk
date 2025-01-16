// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
//
use super::PageInfo;
use crate::query_types::schema;
use crate::query_types::Address;
use crate::query_types::BigInt;
use crate::query_types::DateTime;
use crate::query_types::ProtocolConfigs;

// ===========================================================================
// Epoch Queries
// ===========================================================================
#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "EpochArgs")]
pub struct EpochQuery {
    #[arguments(id: $id)]
    pub epoch: Option<Epoch>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "EpochsArgs")]
pub struct EpochsQuery {
    #[arguments(first: $first, after: $after, last: $last, before: $before)]
    pub epochs: EpochConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "EpochArgs")]
pub struct EpochSummaryQuery {
    #[arguments(id: $id)]
    pub epoch: Option<EpochSummary>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "EpochConnection")]
pub struct EpochConnection {
    pub nodes: Vec<Epoch>,
    pub page_info: PageInfo,
}
// ===========================================================================
// Epoch Summary Args
// ===========================================================================

#[derive(cynic::QueryVariables, Debug)]
pub struct EpochArgs {
    pub id: Option<u64>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct EpochsArgs<'a> {
    pub first: Option<i32>,
    pub after: Option<&'a str>,
    pub last: Option<i32>,
    pub before: Option<&'a str>,
}

/// A summary of the epoch.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Epoch")]
pub struct EpochSummary {
    /// The epoch number.
    pub epoch_id: u64,
    /// The reference gas price throughout this epoch.
    pub reference_gas_price: Option<BigInt>,
    /// The total number of checkpoints in this epoch.
    pub total_checkpoints: Option<u64>,
    /// The total number of transactions in this epoch.
    pub total_transactions: Option<u64>,
}

// ===========================================================================
// Epoch Types
// ===========================================================================

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema = "rpc", graphql_type = "Epoch")]
pub struct Epoch {
    pub end_timestamp: Option<DateTime>,
    pub epoch_id: u64,
    pub fund_inflow: Option<BigInt>,
    pub fund_outflow: Option<BigInt>,
    pub fund_size: Option<BigInt>,
    pub live_object_set_digest: Option<String>,
    pub net_inflow: Option<BigInt>,
    pub protocol_configs: Option<ProtocolConfigs>,
    pub reference_gas_price: Option<BigInt>,
    pub start_timestamp: DateTime,
    pub system_state_version: Option<u64>,
    pub total_checkpoints: Option<u64>,
    pub total_gas_fees: Option<BigInt>,
    pub total_stake_rewards: Option<BigInt>,
    pub total_stake_subsidies: Option<BigInt>,
    pub total_transactions: Option<u64>,
    pub validator_set: Option<ValidatorSet>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema = "rpc", graphql_type = "ValidatorSet")]
pub struct ValidatorSet {
    pub inactive_pools_id: Option<Address>,
    pub inactive_pools_size: Option<i32>,
    pub pending_active_validators_id: Option<Address>,
    pub pending_active_validators_size: Option<i32>,
    pub pending_removals: Option<Vec<i32>>,
    pub staking_pool_mappings_id: Option<Address>,
    pub staking_pool_mappings_size: Option<i32>,
    pub total_stake: Option<BigInt>,
    pub validator_candidates_size: Option<i32>,
    pub validator_candidates_id: Option<Address>,
}
