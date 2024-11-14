// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use chrono::DateTime as ChronoDT;
use sui_types::types::CheckpointContentsDigest;
use sui_types::types::CheckpointDigest;
use sui_types::types::CheckpointSummary;
use sui_types::types::GasCostSummary as NativeGasCostSummary;

use crate::error;
use crate::error::Error;
use crate::error::Kind;
use crate::query_types::schema;
use crate::query_types::Base64;
use crate::query_types::BigInt;
use crate::query_types::DateTime;
use crate::query_types::Epoch;
use crate::query_types::PageInfo;

// ===========================================================================
// Checkpoint Queries
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "CheckpointArgs")]
pub struct CheckpointQuery {
    #[arguments(id: $id)]
    pub checkpoint: Option<Checkpoint>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "CheckpointArgs")]
pub struct CheckpointTotalTxQuery {
    #[arguments(id: $id)]
    pub checkpoint: Option<CheckpointTotalTx>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Checkpoint")]
pub struct CheckpointTotalTx {
    pub network_total_transactions: Option<u64>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "CheckpointsArgs")]
pub struct CheckpointsQuery {
    pub checkpoints: CheckpointConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "CheckpointConnection")]
pub struct CheckpointConnection {
    pub nodes: Vec<Checkpoint>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct CheckpointsArgs<'a> {
    pub first: Option<i32>,
    pub after: Option<&'a str>,
    pub last: Option<i32>,
    pub before: Option<&'a str>,
}

// ===========================================================================
// Checkpoint Query Args
// ===========================================================================

#[derive(cynic::QueryVariables, Debug)]
pub struct CheckpointArgs {
    pub id: CheckpointId,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "CheckpointId")]
pub struct CheckpointId {
    pub digest: Option<String>,
    pub sequence_number: Option<u64>,
}
// ===========================================================================
// Checkpoint Types
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Checkpoint")]
pub struct Checkpoint {
    pub epoch: Option<Epoch>,
    pub digest: String,
    pub network_total_transactions: Option<u64>,
    pub previous_checkpoint_digest: Option<String>,
    pub sequence_number: u64,
    pub timestamp: DateTime,
    pub validator_signatures: Base64,
    pub rolling_gas_summary: Option<GasCostSummary>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "GasCostSummary")]
pub struct GasCostSummary {
    pub computation_cost: Option<BigInt>,
    pub non_refundable_storage_fee: Option<BigInt>,
    pub storage_cost: Option<BigInt>,
    pub storage_rebate: Option<BigInt>,
}

// TODO need bcs in GraphQL Checkpoint to avoid this conversion
impl TryInto<CheckpointSummary> for Checkpoint {
    type Error = error::Error;

    fn try_into(self) -> Result<CheckpointSummary, Self::Error> {
        let epoch = self
            .epoch
            .ok_or_else(|| {
                Error::from_error(Kind::Other, "Epoch is checkpoint summary is missing")
            })?
            .epoch_id;
        let network_total_transactions = self.network_total_transactions.ok_or_else(|| {
            Error::from_error(
                Kind::Other,
                "Network total transactions in checkpoint summary is missing",
            )
        })?;
        let sequence_number = self.sequence_number;
        let timestamp_ms = ChronoDT::parse_from_rfc3339(&self.timestamp.0)?
            .timestamp_millis()
            .try_into()?;
        let content_digest = CheckpointContentsDigest::from_base58(&self.digest)?;
        let previous_digest = self
            .previous_checkpoint_digest
            .map(|d| CheckpointDigest::from_base58(&d))
            .transpose()?;
        let epoch_rolling_gas_cost_summary = self
            .rolling_gas_summary
            .ok_or_else(|| {
                Error::from_error(
                    Kind::Other,
                    "Gas cost summary in checkpoint summary is missing",
                )
            })?
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
    type Error = error::Error;
    fn try_into(self) -> Result<NativeGasCostSummary, Self::Error> {
        let computation_cost = self
            .computation_cost
            .ok_or_else(|| Error::from_error(Kind::Other, "Computation cost is missing"))?
            .try_into()?;
        let non_refundable_storage_fee = self
            .non_refundable_storage_fee
            .ok_or_else(|| Error::from_error(Kind::Other, "Non-refundable storage fee is missing"))?
            .try_into()?;
        let storage_cost = self
            .storage_cost
            .ok_or_else(|| Error::from_error(Kind::Other, "Storage cost is missing"))?
            .try_into()?;
        let storage_rebate = self
            .storage_rebate
            .ok_or_else(|| Error::from_error(Kind::Other, "Storage rebate is missing"))?
            .try_into()?;
        Ok(NativeGasCostSummary {
            computation_cost,
            non_refundable_storage_fee,
            storage_cost,
            storage_rebate,
        })
    }
}
