// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::SuiAddress;
use crate::query_types::{schema, BigInt};

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "BalanceArgs")]
pub struct BalanceQuery {
    #[arguments(address: $address)]
    pub owner: Option<Owner>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Owner", variables = "BalanceArgs")]
pub struct Owner {
    #[arguments(type: $coin_type)]
    pub balance: Option<Balance>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Balance")]
pub struct Balance {
    pub total_balance: Option<BigInt>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct BalanceArgs {
    pub address: SuiAddress,
    pub coin_type: Option<String>,
}
