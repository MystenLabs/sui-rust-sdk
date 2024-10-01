// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod active_validators;
mod balance;
mod chain;
mod checkpoint;
mod coin;
mod dry_run;
mod epoch;
mod events;
mod execute_tx;
mod object;
mod protocol_config;
mod service_config;
mod transaction;

pub use active_validators::ActiveValidatorsArgs;
pub use active_validators::ActiveValidatorsQuery;
pub use active_validators::EpochValidator;
pub use active_validators::Validator;
pub use active_validators::ValidatorConnection;
pub use active_validators::ValidatorSet;
pub use balance::Balance;
pub use balance::BalanceArgs;
pub use balance::BalanceQuery;
pub use balance::Owner;
pub use chain::ChainIdentifierQuery;
pub use checkpoint::CheckpointArgs;
pub use checkpoint::CheckpointId;
pub use checkpoint::CheckpointQuery;
pub use coin::CoinMetadata;
pub use coin::CoinMetadataArgs;
pub use coin::CoinMetadataQuery;
pub use dry_run::DryRunArgs;
pub use dry_run::DryRunQuery;
pub use dry_run::DryRunResult;
pub use dry_run::TransactionMetadata;
pub use epoch::Epoch;
pub use epoch::EpochSummaryArgs;
pub use epoch::EpochSummaryQuery;
pub use events::Event;
pub use events::EventConnection;
pub use events::EventFilter;
pub use events::EventsQuery;
pub use events::EventsQueryArgs;
pub use execute_tx::ExecuteTransactionArgs;
pub use execute_tx::ExecuteTransactionQuery;
pub use execute_tx::ExecutionResult;
pub use object::ObjectFilter;
pub use object::ObjectKey;
pub use object::ObjectQuery;
pub use object::ObjectQueryArgs;
pub use object::ObjectsQuery;
pub use object::ObjectsQueryArgs;
pub use protocol_config::ProtocolConfigQuery;
pub use protocol_config::ProtocolConfigs;
pub use protocol_config::ProtocolVersionArgs;
pub use service_config::Feature;
pub use service_config::ServiceConfig;
pub use service_config::ServiceConfigQuery;
pub use transaction::TransactionBlockArgs;
pub use transaction::TransactionBlockQuery;
pub use transaction::TransactionBlocksQuery;
pub use transaction::TransactionBlocksQueryArgs;
pub use transaction::TransactionsFilter;

use sui_types::types::Address;

use anyhow::anyhow;
use cynic::impl_scalar;

#[cynic::schema("rpc")]
pub mod schema {}

// ===========================================================================
// Scalars
// ===========================================================================

impl_scalar!(Address, schema::SuiAddress);
impl_scalar!(u64, schema::UInt53);

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "Base64")]
pub struct Base64(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "BigInt")]
pub struct BigInt(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "DateTime")]
pub struct DateTime(pub String);

// ===========================================================================
// Types used in several queries
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct GQLAddress {
    pub address: Address,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "MoveObject")]
pub struct MoveObject {
    pub bcs: Option<Base64>,
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

impl TryFrom<BigInt> for u64 {
    type Error = anyhow::Error;

    fn try_from(value: BigInt) -> Result<Self, Self::Error> {
        value
            .0
            .parse::<u64>()
            .map_err(|e| anyhow!("Cannot convert BigInt into u64: {e}"))
    }
}
