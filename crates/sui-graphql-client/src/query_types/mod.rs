// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use std::str::FromStr;

mod active_validators;
mod balance;
mod chain;
mod checkpoint;
mod coin;
mod epoch;
mod events;
mod execute_tx;
mod object;
mod protocol_config;
mod service_config;
mod transaction;

pub use active_validators::{
    ActiveValidatorsArgs, ActiveValidatorsQuery, EpochValidator, Validator, ValidatorConnection,
    ValidatorSet,
};
use anyhow::{anyhow, Error};
pub use balance::{Balance, BalanceArgs, BalanceQuery, Owner};
pub use chain::ChainIdentifierQuery;
pub use checkpoint::{CheckpointArgs, CheckpointId, CheckpointQuery};
pub use coin::{CoinMetadata, CoinMetadataArgs, CoinMetadataQuery};
pub use epoch::{Epoch, EpochSummaryArgs, EpochSummaryQuery};
pub use events::{Event, EventConnection, EventFilter, EventsQuery, EventsQueryArgs};
pub use execute_tx::{ExecuteTransactionArgs, ExecuteTransactionQuery, ExecutionResult};
pub use object::{
    ObjectFilter, ObjectKey, ObjectQuery, ObjectQueryArgs, ObjectsQuery, ObjectsQueryArgs,
};
pub use protocol_config::{ProtocolConfigQuery, ProtocolConfigs, ProtocolVersionArgs};
pub use service_config::{Feature, ServiceConfig, ServiceConfigQuery};
use sui_types::types::Address as NativeAddress;
pub use transaction::{
    TransactionBlockArgs, TransactionBlockQuery, TransactionBlocksQuery,
    TransactionBlocksQueryArgs, TransactionsFilter,
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

#[derive(cynic::Scalar, Debug)]
pub struct SuiAddress(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "UInt53")]
pub struct Uint53(pub u64);

// ===========================================================================
// Types used in several queries
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct Address {
    pub address: SuiAddress,
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

impl From<NativeAddress> for SuiAddress {
    fn from(value: NativeAddress) -> Self {
        SuiAddress(value.to_string())
    }
}

impl TryFrom<SuiAddress> for NativeAddress {
    type Error = anyhow::Error;

    fn try_from(value: SuiAddress) -> Result<Self, Self::Error> {
        NativeAddress::from_str(&value.0)
            .map_err(|e| Error::msg(format!("Cannot convert SuiAddress into Address: {e}")))
    }
}
