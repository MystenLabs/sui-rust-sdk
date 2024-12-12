// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use base64ct::Encoding;
use sui_types::types::CheckpointSummary;

use crate::error;
use crate::error::Error;
use crate::error::Kind;
use crate::query_types::schema;
use crate::query_types::Base64;
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
    pub bcs: Option<Base64>,
}

impl TryInto<CheckpointSummary> for Checkpoint {
    type Error = error::Error;

    fn try_into(self) -> Result<CheckpointSummary, Error> {
        let checkpoint = self
            .bcs
            .map(|x| base64ct::Base64::decode_vec(&x.0))
            .transpose()?
            .map(|bcs| {
                bcs::from_bytes::<CheckpointSummary>(&bcs).map_err(|e| {
                    Error::from_error(
                        Kind::Other,
                        format!("Failed to deserialize checkpoint summary: {}", e),
                    )
                })
            })
            .transpose()?;
        checkpoint.ok_or_else(|| Error::from_error(Kind::Other, "Checkpoint summary is missing"))
    }
}
