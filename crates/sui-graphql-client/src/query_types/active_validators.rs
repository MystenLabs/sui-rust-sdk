// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::query_types::schema;
use crate::query_types::Address;
use crate::query_types::Base64;
use crate::query_types::BigInt;
use crate::query_types::MoveObject;
use crate::query_types::PageInfo;
use crate::query_types::SuiAddress;
use crate::query_types::Uint53;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "ActiveValidatorsArgs"
)]
pub struct ActiveValidatorsQuery {
    #[arguments(id: $id)]
    pub epoch: Option<EpochValidator>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ActiveValidatorsArgs {
    pub id: Option<Uint53>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Epoch",
    variables = "ActiveValidatorsArgs"
)]
pub struct EpochValidator {
    pub validator_set: Option<ValidatorSet>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "ValidatorSet",
    variables = "ActiveValidatorsArgs"
)]
pub struct ValidatorSet {
    #[arguments(after: $after, before: $before, first: $first, last: $last)]
    pub active_validators: ValidatorConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "ValidatorConnection")]
pub struct ValidatorConnection {
    pub page_info: PageInfo,
    pub nodes: Vec<Validator>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Validator")]
pub struct Validator {
    /// The APY of this validator in basis points.
    /// To get the APY in percentage, divide by 100.
    pub apy: Option<i32>,
    /// The validator's address.
    pub address: Address,
    /// The fee charged by the validator for staking services.
    pub commission_rate: Option<i32>,
    /// Validator's credentials.
    pub credentials: Option<ValidatorCredentials>,
    /// Validator's description.
    pub description: Option<String>,
    /// Number of exchange rates in the table.
    pub exchange_rates_size: Option<Uint53>,
    /// The reference gas price for this epoch.
    pub gas_price: Option<BigInt>,
    /// Validator's name.
    pub name: Option<String>,
    /// Validator's url containing their custom image.
    pub image_url: Option<String>,
    /// The proposed next epoch fee for the validator's staking services.
    pub next_epoch_commission_rate: Option<i32>,
    /// Validator's credentials for the next epoch.
    pub next_epoch_credentials: Option<ValidatorCredentials>,
    /// The validator's gas price quote for the next epoch.
    pub next_epoch_gas_price: Option<BigInt>,
    /// The total number of SUI tokens in this pool plus
    /// the pending stake amount for this epoch.
    pub next_epoch_stake: Option<BigInt>,
    /// The validator's current valid `Cap` object. Validators can delegate
    /// the operation ability to another address. The address holding this `Cap` object
    /// can then update the reference gas price and tallying rule on behalf of the validator.
    pub operation_cap: Option<MoveObject>,
    /// Pending pool token withdrawn during the current epoch, emptied at epoch boundaries.
    pub pending_pool_token_withdraw: Option<BigInt>,
    /// Pending stake amount for this epoch.
    pub pending_stake: Option<BigInt>,
    /// Pending stake withdrawn during the current epoch, emptied at epoch boundaries.
    pub pending_total_sui_withdraw: Option<BigInt>,
    /// Total number of pool tokens issued by the pool.
    pub pool_token_balance: Option<BigInt>,
    /// Validator's homepage URL.
    pub project_url: Option<String>,
    /// The epoch stake rewards will be added here at the end of each epoch.
    pub rewards_pool: Option<BigInt>,
    /// The epoch at which this pool became active.
    pub staking_pool_activation_epoch: Option<Uint53>,
    /// The ID of this validator's `0x3::staking_pool::StakingPool`.
    pub staking_pool_id: SuiAddress,
    /// The total number of SUI tokens in this pool.
    pub staking_pool_sui_balance: Option<BigInt>,
    /// The voting power of this validator in basis points (e.g., 100 = 1% voting power).
    pub voting_power: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "ValidatorCredentials")]
#[allow(non_snake_case)]
/// The credentials related fields associated with a validator.
pub struct ValidatorCredentials {
    pub protocol_pub_key: Option<Base64>,
    pub network_pub_key: Option<Base64>,
    pub worker_pub_key: Option<Base64>,
    pub proof_of_possession: Option<Base64>,
    pub net_address: Option<String>,
    // TODO need to fix this in the graphQL schema ugh. p2P -> p2p
    // pub p2P_address: Option<String>,
    pub primary_address: Option<String>,
    pub worker_address: Option<String>,
}
