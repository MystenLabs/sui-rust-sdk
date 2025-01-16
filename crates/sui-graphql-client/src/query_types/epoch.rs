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
    /// The epoch's id as a sequence number that starts at 0 and is incremented by one at every epoch change.
    pub epoch_id: u64,
    /// The storage fees paid for transactions executed during the epoch.
    pub fund_inflow: Option<BigInt>,
    /// The storage fee rebates paid to users who deleted the data associated with past
    /// transactions.
    pub fund_outflow: Option<BigInt>,
    /// The storage fund available in this epoch.
    /// This fund is used to redistribute storage fees from past transactions
    /// to future validators.
    pub fund_size: Option<BigInt>,
    /// A commitment by the committee at the end of epoch on the contents of the live object set at
    /// that time. This can be used to verify state snapshots.
    pub live_object_set_digest: Option<String>,
    /// The difference between the fund inflow and outflow, representing
    /// the net amount of storage fees accumulated in this epoch.
    pub net_inflow: Option<BigInt>,
    /// The epoch's corresponding protocol configuration, including the feature flags and the
    /// configuration options.
    pub protocol_configs: Option<ProtocolConfigs>,
    /// The minimum gas price that a quorum of validators are guaranteed to sign a transaction for.
    pub reference_gas_price: Option<BigInt>,
    /// The epoch's starting timestamp.
    pub start_timestamp: DateTime,
    /// The epoch's ending timestamp. Note that this is available only on epochs that have ended.
    pub end_timestamp: Option<DateTime>,
    /// The value of the `version` field of `0x5`, the `0x3::sui::SuiSystemState` object.  This
    /// version changes whenever the fields contained in the system state object (held in a dynamic
    /// field attached to `0x5`) change.
    pub system_state_version: Option<u64>,
    /// The total number of checkpoints in this epoch.
    pub total_checkpoints: Option<u64>,
    /// The total amount of gas fees (in MIST) that were paid in this epoch.
    pub total_gas_fees: Option<BigInt>,
    /// The total MIST rewarded as stake.
    pub total_stake_rewards: Option<BigInt>,
    /// The amount added to total gas fees to make up the total stake rewards.
    pub total_stake_subsidies: Option<BigInt>,
    /// The total number of transaction in this epoch.
    pub total_transactions: Option<u64>,
    /// Validator related properties. For active validators, see [`active_validators`] API.
    pub validator_set: Option<ValidatorSet>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema = "rpc", graphql_type = "ValidatorSet")]
pub struct ValidatorSet {
    /// Object ID of the `Table` storing the inactive staking pools.
    pub inactive_pools_id: Option<Address>,
    /// Size of the inactive pools `Table`.
    pub inactive_pools_size: Option<i32>,
    /// Object ID of the wrapped object `TableVec` storing the pending active validators.
    pub pending_active_validators_id: Option<Address>,
    /// Size of the pending active validators table.
    pub pending_active_validators_size: Option<i32>,
    /// Validators that are pending removal from the active validator set, expressed as indices in
    /// to `activeValidators`.
    pub pending_removals: Option<Vec<i32>>,
    /// Object ID of the `Table` storing the mapping from staking pool ids to the addresses
    /// of the corresponding validators. This is needed because a validator's address
    /// can potentially change but the object ID of its pool will not.
    pub staking_pool_mappings_id: Option<Address>,
    /// Size of the stake pool mappings `Table`.
    pub staking_pool_mappings_size: Option<i32>,
    /// Total amount of stake for all active validators at the beginning of the epoch.
    pub total_stake: Option<BigInt>,
    /// Size of the validator candidates `Table`.
    pub validator_candidates_size: Option<i32>,
    /// Object ID of the `Table` storing the validator candidates.
    pub validator_candidates_id: Option<Address>,
}
