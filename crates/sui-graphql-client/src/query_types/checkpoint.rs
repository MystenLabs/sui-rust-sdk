// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use std::str::FromStr;

use anyhow::Error;
use chrono::DateTime as ChronoDT;
use sui_types::types::GasCostSummary as NativeGasCostSummary;
use sui_types::types::{CheckpointContentsDigest, CheckpointDigest, CheckpointSummary};

use crate::query_types::{schema, Base64, BigInt, DateTime, Epoch, Uint53};

// ===========================================================================
// Checkpoint Queries
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "CheckpointArgs")]
pub struct CheckpointQuery {
    #[arguments(id: $id)]
    pub checkpoint: Option<Checkpoint>,
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
    pub sequence_number: Option<Uint53>,
}
// ===========================================================================
// Checkpoint Types
// ===========================================================================

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
#[cynic(schema = "rpc", graphql_type = "GasCostSummary")]
pub struct GasCostSummary {
    pub computation_cost: Option<BigInt>,
    pub non_refundable_storage_fee: Option<BigInt>,
    pub storage_cost: Option<BigInt>,
    pub storage_rebate: Option<BigInt>,
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
