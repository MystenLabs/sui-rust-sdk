// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use anyhow::{anyhow, Error};
use chrono::DateTime as ChronoDT;

use sui_types::types::{
    Address, CheckpointContentsDigest, CheckpointDigest, CheckpointSummary,
    GasCostSummary as NativeGasCostSummary,
};

#[cynic::schema("rpc")]
pub mod schema {}

// ===========================================================================
// Scalars
// ===========================================================================

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "Base64")]
pub struct Base64(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "BigInt")]
pub struct BigInt(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "DateTime")]
pub struct DateTime(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct SuiAddress(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "UInt53")]
pub struct Uint53(pub u64);

// ===========================================================================
// Queries
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query")]
pub struct ChainIdentifierQuery {
    pub chain_identifier: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "CoinMetadataArgs")]
pub struct CoinMetadataQuery {
    #[arguments(coinType: $coin_type)]
    pub coin_metadata: Option<CoinMetadata>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "CheckpointArgs")]
pub struct CheckpointQuery {
    #[arguments(id: $id)]
    pub checkpoint: Option<Checkpoint>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "EpochSummaryArgs")]
pub struct EpochSummaryQuery {
    #[arguments(id: $id)]
    pub epoch: Option<EpochSummary>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "EventsQueryArgs")]
pub struct EventsQuery {
    #[arguments(after: $after, before: $before, filter: $filter, first: $first, last: $last)]
    pub events: EventConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "ObjectQueryArgs")]
pub struct ObjectQuery {
    #[arguments(address: $address, version: $version)]
    pub object: Option<Object>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "ProtocolVersionArgs"
)]
pub struct ProtocolConfigQuery {
    #[arguments(protocolVersion: $id)]
    pub protocol_config: ProtocolConfigs,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query")]
pub struct ServiceConfigQuery {
    pub service_config: ServiceConfig,
}

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

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "ObjectsQueryArgs")]
pub struct ObjectsQuery {
    #[arguments(after: $after, before: $before, filter: $filter, first: $first, last: $last)]
    pub objects: ObjectConnection,
}

// ===========================================================================
// Variables
// ===========================================================================

#[derive(cynic::QueryVariables, Debug)]
pub struct CheckpointArgs {
    pub id: CheckpointId,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct EpochSummaryArgs {
    pub id: Option<Uint53>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ProtocolVersionArgs {
    pub id: Option<Uint53>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct TransactionBlockArgs {
    pub digest: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct CoinMetadataArgs<'a> {
    pub coin_type: &'a str,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ObjectQueryArgs {
    pub address: SuiAddress,
    pub version: Option<Uint53>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ObjectsQueryArgs<'a> {
    pub after: Option<&'a str>,
    pub before: Option<&'a str>,
    pub filter: Option<ObjectFilter<'a>>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct EventsQueryArgs {
    pub after: Option<String>,
    pub before: Option<String>,
    pub filter: Option<EventFilter>,
    pub first: Option<i32>,
    pub last: Option<i32>,
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
// Types
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "ServiceConfig")]
pub struct ServiceConfig {
    /// List the available versions for this GraphQL service.
    pub available_versions: Vec<String>,
    /// Default number of elements allowed on a single page of a connection.
    pub default_page_size: i32,
    /// List of all features that are enabled on this RPC service.
    pub enabled_features: Vec<Feature>,
    // TODO This field is retrieved as a string, instead of i32
    /// Maximum estimated cost of a database query used to serve a GraphQL request.  This is
    /// measured in the same units that the database uses in EXPLAIN queries.
    // pub max_db_query_cost: i32,
    /// Maximum nesting allowed in struct fields when calculating the layout of a single Move Type.
    pub max_move_value_depth: i32,
    /// The maximum number of output nodes in a GraphQL response.
    /// Non-connection nodes have a count of 1, while connection nodes are counted as
    /// the specified 'first' or 'last' number of items, or the default_page_size
    /// as set by the server if those arguments are not set.
    /// Counts accumulate multiplicatively down the query tree. For example, if a query starts
    /// with a connection of first: 10 and has a field to a connection with last: 20, the count
    /// at the second level would be 200 nodes. This is then summed to the count of 10 nodes
    /// at the first level, for a total of 210 nodes.
    pub max_output_nodes: i32,
    /// Maximum number of elements allowed on a single page of a connection.
    pub max_page_size: i32,
    /// The maximum depth a GraphQL query can be to be accepted by this service.
    pub max_query_depth: i32,
    /// The maximum number of nodes (field names) the service will accept in a single query.
    pub max_query_nodes: i32,
    /// Maximum length of a query payload string.
    pub max_query_payload_size: i32,
    /// Maximum nesting allowed in type arguments in Move Types resolved by this service.
    pub max_type_argument_depth: i32,
    /// Maximum number of type arguments passed into a generic instantiation of a Move Type resolved
    /// by this service.
    pub max_type_argument_width: i32,
    /// Maximum number of structs that need to be processed when calculating the layout of a single
    /// Move Type.
    pub max_type_nodes: i32,
    /// Maximum time in milliseconds spent waiting for a response from fullnode after issuing a
    /// a transaction to execute. Note that the transaction may still succeed even in the case of a
    /// timeout. Transactions are idempotent, so a transaction that times out should be resubmitted
    /// until the network returns a definite response (success or failure, not timeout).
    pub mutation_timeout_ms: i32,
    /// Maximum time in milliseconds that will be spent to serve one query request.
    pub request_timeout_ms: i32,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Feature",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum Feature {
    Analytics,
    Coins,
    DynamicFields,
    NameService,
    Subscriptions,
    SystemState,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "CoinMetadata")]
pub struct CoinMetadata {
    pub decimals: Option<i32>,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub supply: Option<BigInt>,
    pub version: Uint53,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "CheckpointId")]
pub struct CheckpointId {
    pub digest: Option<String>,
    pub sequence_number: Option<Uint53>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Checkpoint")]
pub struct Checkpoint {
    pub epoch: Option<Epoch>,
    pub digest: String,
    pub network_total_transactions: Option<Uint53>,
    pub previous_checkpoint_digest: Option<String>,
    pub sequence_number: Uint53,
    pub timestamp: DateTime,
    pub validator_signatures: Base64,
    pub rolling_gas_summary: Option<GasCostSummary>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Epoch")]
pub struct EpochSummary {
    pub epoch_id: Uint53,
    pub reference_gas_price: Option<BigInt>,
    pub total_checkpoints: Option<Uint53>,
    pub total_transactions: Option<Uint53>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Epoch")]
pub struct Epoch {
    pub end_timestamp: Option<DateTime>,
    pub epoch_id: Uint53,
    pub fund_inflow: Option<BigInt>,
    pub fund_outflow: Option<BigInt>,
    pub fund_size: Option<BigInt>,
    pub live_object_set_digest: Option<String>,
    pub net_inflow: Option<BigInt>,
    pub reference_gas_price: Option<BigInt>,
    pub start_timestamp: DateTime,
    pub system_state_version: Option<Uint53>,
    pub total_checkpoints: Option<Uint53>,
    pub total_gas_fees: Option<BigInt>,
    pub total_stake_rewards: Option<BigInt>,
    pub total_stake_subsidies: Option<BigInt>,
    pub total_transactions: Option<Uint53>,
    pub validator_set: Option<ValidatorSet>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "EventConnection")]
pub struct EventConnection {
    pub page_info: PageInfo,
    pub nodes: Vec<Event>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Event")]
pub struct Event {
    pub bcs: Base64,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "ObjectConnection")]
pub struct ObjectConnection {
    pub page_info: PageInfo,
    pub nodes: Vec<Object>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Object")]
pub struct Object {
    pub bcs: Option<Base64>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "ObjectFilter")]
pub struct ObjectFilter<'a> {
    #[cynic(rename = "type")]
    pub type_: Option<&'a str>,
    pub owner: Option<SuiAddress>,
    pub object_ids: Option<Vec<SuiAddress>>,
    pub object_keys: Option<Vec<ObjectKey>>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "ObjectKey")]
pub struct ObjectKey {
    pub object_id: SuiAddress,
    pub version: Uint53,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "ValidatorSet")]
pub struct ValidatorSet {
    pub inactive_pools_id: Option<SuiAddress>,
    pub inactive_pools_size: Option<i32>,
    pub pending_active_validators_id: Option<SuiAddress>,
    pub pending_active_validators_size: Option<i32>,
    pub pending_removals: Option<Vec<i32>>,
    pub staking_pool_mappings_id: Option<SuiAddress>,
    pub staking_pool_mappings_size: Option<i32>,
    pub total_stake: Option<BigInt>,
    pub validator_candidates_size: Option<i32>,
    pub validator_candidates_id: Option<SuiAddress>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "GasCostSummary")]
pub struct GasCostSummary {
    pub computation_cost: Option<BigInt>,
    pub non_refundable_storage_fee: Option<BigInt>,
    pub storage_cost: Option<BigInt>,
    pub storage_rebate: Option<BigInt>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "ProtocolConfigs")]
pub struct ProtocolConfigs {
    pub protocol_version: Uint53,
    pub feature_flags: Vec<ProtocolConfigFeatureFlag>,
    pub configs: Vec<ProtocolConfigAttr>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "ProtocolConfigFeatureFlag")]
pub struct ProtocolConfigFeatureFlag {
    pub key: String,
    pub value: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "ProtocolConfigAttr")]
pub struct ProtocolConfigAttr {
    pub key: String,
    pub value: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlock")]
pub struct TransactionBlock {
    pub bcs: Option<Base64>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlockConnection")]
pub struct TransactionBlockConnection {
    pub nodes: Vec<TransactionBlock>,
    pub page_info: PageInfo,
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
    pub at_checkpoint: Option<Uint53>,
    pub before_checkpoint: Option<Uint53>,
    pub changed_object: Option<SuiAddress>,
    pub input_object: Option<SuiAddress>,
    pub recv_address: Option<SuiAddress>,
}
#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "EventFilter")]
pub struct EventFilter {
    pub emitting_module: Option<String>,
    pub event_type: Option<String>,
    pub sender: Option<SuiAddress>,
    pub transaction_digest: Option<String>,
}

// ===========================================================================
// Utility Types
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "PageInfo")]
/// Information about pagination in a connection.
pub struct PageInfo {
    /// When paginating backwards, are there more items?
    pub has_previous_page: bool,
    /// Are there more items when paginating forwards?
    pub has_next_page: bool,
    /// When paginating backwards, the cursor to continue.
    pub start_cursor: Option<String>,
    /// When paginating forwards, the cursor to continue.
    pub end_cursor: Option<String>,
}

// ===========================================================================
// Type Conversions
// ===========================================================================

impl From<Uint53> for u64 {
    fn from(value: Uint53) -> Self {
        value.0
    }
}

impl TryFrom<BigInt> for u64 {
    type Error = anyhow::Error;

    fn try_from(value: BigInt) -> Result<Self, Self::Error> {
        value
            .0
            .parse::<u64>()
            .map_err(|e| anyhow!("Cannot convert BigInt into u64: {e}"))
    }
}

impl TryInto<CheckpointSummary> for Checkpoint {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<CheckpointSummary, Self::Error> {
        let epoch = self
            .epoch
            .ok_or_else(|| Error::msg("Epoch is missing"))?
            .epoch_id
            .into();
        let network_total_transactions = self
            .network_total_transactions
            .ok_or_else(|| Error::msg("Network total transactions is missing"))?
            .into();
        let sequence_number = self.sequence_number.into();
        let timestamp_ms = ChronoDT::parse_from_rfc3339(&self.timestamp.0)
            .map_err(|e| Error::msg(format!("Cannot parse DateTime: {e}")))?
            .timestamp_millis()
            .try_into()?;
        let content_digest = CheckpointContentsDigest::from_str(&self.digest)?;
        let previous_digest = self
            .previous_checkpoint_digest
            .map(|d| CheckpointDigest::from_str(&d))
            .transpose()?;
        let epoch_rolling_gas_cost_summary = self
            .rolling_gas_summary
            .ok_or_else(|| Error::msg("Rolling gas summary is missing"))?
            .try_into()?;
        Ok(CheckpointSummary {
            epoch,
            sequence_number,
            network_total_transactions,
            timestamp_ms,
            content_digest,
            previous_digest,
            epoch_rolling_gas_cost_summary,
            checkpoint_commitments: vec![],
            end_of_epoch_data: None,
            version_specific_data: vec![],
        })
    }
}

impl TryInto<NativeGasCostSummary> for GasCostSummary {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<NativeGasCostSummary, Self::Error> {
        let computation_cost = self
            .computation_cost
            .ok_or_else(|| Error::msg("Computation cost is missing"))?
            .try_into()?;
        let non_refundable_storage_fee = self
            .non_refundable_storage_fee
            .ok_or_else(|| Error::msg("Non-refundable storage fee is missing"))?
            .try_into()?;
        let storage_cost = self
            .storage_cost
            .ok_or_else(|| Error::msg("Storage cost is missing"))?
            .try_into()?;
        let storage_rebate = self
            .storage_rebate
            .ok_or_else(|| Error::msg("Storage rebate is missing"))?
            .try_into()?;
        Ok(NativeGasCostSummary {
            computation_cost,
            non_refundable_storage_fee,
            storage_cost,
            storage_rebate,
        })
    }
}

impl From<Address> for SuiAddress {
    fn from(value: Address) -> Self {
        SuiAddress(value.to_string())
    }
}

impl TryFrom<SuiAddress> for Address {
    type Error = anyhow::Error;

    fn try_from(value: SuiAddress) -> Result<Self, Self::Error> {
        Address::from_str(&value.0)
            .map_err(|e| Error::msg(format!("Cannot convert SuiAddress into Address: {e}")))
    }
}
