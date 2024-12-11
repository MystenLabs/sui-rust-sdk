// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::query_types::schema;
use crate::query_types::transaction::TransactionBlockDigest;
use crate::query_types::Address;
use crate::query_types::Base64;
use crate::query_types::PageInfo;

// ===========================================================================
// Events Queries
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query", variables = "EventsQueryArgs")]
pub struct EventsQuery {
    #[arguments(after: $after, before: $before, filter: $filter, first: $first, last: $last)]
    pub events: EventConnection,
}

// ===========================================================================
// Events Query Args
// ===========================================================================

#[derive(cynic::QueryVariables, Debug)]
pub struct EventsQueryArgs<'a> {
    pub filter: Option<EventFilter>,
    pub after: Option<&'a str>,
    pub before: Option<&'a str>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

// ===========================================================================
// Events Types
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "EventConnection")]
pub struct EventConnection {
    pub page_info: PageInfo,
    pub nodes: Vec<Event>,
}

#[derive(Clone, cynic::InputObject, Debug)]
#[cynic(schema = "rpc", graphql_type = "EventFilter")]
pub struct EventFilter {
    pub emitting_module: Option<String>,
    pub event_type: Option<String>,
    pub sender: Option<Address>,
    pub transaction_digest: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Event")]
pub struct Event {
    pub bcs: Base64,
    pub transaction_block: Option<TransactionBlockDigest>,
}
