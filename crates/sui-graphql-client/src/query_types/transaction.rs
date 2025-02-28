// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use base64ct::Encoding;
use sui_types::SignedTransaction;
use sui_types::Transaction;
use sui_types::TransactionEffects;
use sui_types::UserSignature;

use crate::error;
use crate::error::Error;
use crate::error::Kind;
use crate::query_types::schema;
use crate::query_types::Address;
use crate::query_types::Base64;
use crate::query_types::PageInfo;

// ===========================================================================
// Transaction Block(s) Queries
// ===========================================================================

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
    variables = "TransactionBlockArgs"
)]
pub struct TransactionBlockWithEffectsQuery {
    #[arguments(digest: $digest)]
    pub transaction_block: Option<TransactionBlockWithEffects>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "TransactionBlockArgs"
)]
pub struct TransactionBlockEffectsQuery {
    #[arguments(digest: $digest)]
    pub transaction_block: Option<TxBlockEffects>,
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
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "TransactionBlocksQueryArgs"
)]
pub struct TransactionBlocksWithEffectsQuery {
    #[arguments(first: $first, after: $after, last: $last, before: $before, filter: $filter)]
    pub transaction_blocks: TransactionBlockWithEffectsConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "TransactionBlocksQueryArgs"
)]
pub struct TransactionBlocksEffectsQuery {
    #[arguments(first: $first, after: $after, last: $last, before: $before, filter: $filter)]
    pub transaction_blocks: TransactionBlockEffectsConnection,
}
// ===========================================================================
// Transaction Block(s) Query Args
// ===========================================================================

#[derive(cynic::QueryVariables, Debug)]
pub struct TransactionBlockArgs {
    pub digest: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct TransactionBlocksQueryArgs<'a> {
    pub first: Option<i32>,
    pub after: Option<&'a str>,
    pub last: Option<i32>,
    pub before: Option<&'a str>,
    pub filter: Option<TransactionsFilter<'a>>,
}

// ===========================================================================
// Transaction Block(s) Types
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlock")]
pub struct TransactionBlock {
    pub bcs: Option<Base64>,
    pub signatures: Option<Vec<Base64>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlock")]
pub struct TransactionBlockWithEffects {
    pub bcs: Option<Base64>,
    pub effects: Option<TransactionBlockEffects>,
    pub signatures: Option<Vec<Base64>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlock")]
pub struct TransactionBlockDigest {
    pub digest: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlock")]
pub struct TxBlockEffects {
    pub effects: Option<TransactionBlockEffects>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlockEffects")]
pub struct TransactionBlockEffects {
    pub bcs: Option<Base64>,
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

#[derive(Clone, cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlockFilter")]
pub struct TransactionsFilter<'a> {
    pub function: Option<String>,
    pub kind: Option<TransactionBlockKindInput>,
    pub after_checkpoint: Option<u64>,
    pub at_checkpoint: Option<u64>,
    pub before_checkpoint: Option<u64>,
    pub affected_address: Option<Address>,
    pub sent_address: Option<Address>,
    pub input_object: Option<Address>,
    pub changed_object: Option<Address>,
    pub transaction_ids: Option<Vec<&'a str>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlockConnection")]
pub struct TransactionBlockConnection {
    pub nodes: Vec<TransactionBlock>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlockConnection")]
pub struct TransactionBlockWithEffectsConnection {
    pub nodes: Vec<TransactionBlockWithEffects>,
    pub page_info: PageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "TransactionBlockConnection")]
pub struct TransactionBlockEffectsConnection {
    pub nodes: Vec<TxBlockEffects>,
    pub page_info: PageInfo,
}

impl TryFrom<TransactionBlock> for SignedTransaction {
    type Error = error::Error;

    fn try_from(value: TransactionBlock) -> Result<Self, Self::Error> {
        let transaction = value
            .bcs
            .map(|tx| base64ct::Base64::decode_vec(tx.0.as_str()))
            .transpose()?
            .map(|bcs| bcs::from_bytes::<Transaction>(&bcs))
            .transpose()?;

        let signatures = if let Some(sigs) = value.signatures {
            sigs.iter()
                .map(|s| UserSignature::from_base64(&s.0))
                .collect::<Result<Vec<_>, _>>()?
        } else {
            vec![]
        };

        if let Some(transaction) = transaction {
            Ok(SignedTransaction {
                transaction,
                signatures,
            })
        } else {
            Err(Error::from_error(
                Kind::Other,
                "Expected a deserialized transaction but got None",
            ))
        }
    }
}

impl TryFrom<TxBlockEffects> for TransactionEffects {
    type Error = error::Error;

    fn try_from(value: TxBlockEffects) -> Result<Self, Self::Error> {
        let effects = value
            .effects
            .map(|fx| base64ct::Base64::decode_vec(fx.bcs.unwrap().0.as_str()))
            .transpose()?
            .map(|bcs| bcs::from_bytes::<TransactionEffects>(&bcs))
            .transpose()?;
        effects.ok_or_else(|| {
            Error::from_error(
                Kind::Other,
                "Cannot convert GraphQL TxBlockEffects into TransactionEffects",
            )
        })
    }
}
