// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
//
use crate::query_types::schema;
use crate::query_types::Address;
use crate::query_types::BigInt;
use crate::query_types::DateTime;

// ===========================================================================
// Epoch Queries
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "EpochSummaryArgs")]
pub struct EpochSummaryQuery {
    #[arguments(id: $id)]
    pub epoch: Option<EpochSummary>,
}

// ===========================================================================
// Epoch Summary Args
// ===========================================================================

#[derive(cynic::QueryVariables, Debug)]
pub struct EpochSummaryArgs {
    pub id: Option<u64>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Epoch")]
pub struct EpochSummary {
    pub epoch_id: u64,
    pub reference_gas_price: Option<BigInt>,
    pub total_checkpoints: Option<u64>,
    pub total_transactions: Option<u64>,
}

// ===========================================================================
// Epoch Types
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Epoch")]
pub struct Epoch {
    pub end_timestamp: Option<DateTime>,
    pub epoch_id: u64,
    pub fund_inflow: Option<BigInt>,
    pub fund_outflow: Option<BigInt>,
    pub fund_size: Option<BigInt>,
    pub live_object_set_digest: Option<String>,
    pub net_inflow: Option<BigInt>,
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

#[derive(cynic::QueryFragment, Debug)]
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
