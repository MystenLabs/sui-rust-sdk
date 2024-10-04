// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use base64ct::Encoding;
use sui_types::types::Identifier;

use crate::query_types::schema;
use crate::query_types::Address;
use crate::query_types::Base64;
use crate::query_types::GQLAddress;
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

#[derive(cynic::InputObject, Debug, Clone)]
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
    #[cynic(rename = "type")]
    pub type_: MoveType,
    pub sending_module: Option<MoveModule>,
    pub sender: Option<GQLAddress>,
    pub bcs: Base64,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "MoveModule")]
pub struct MoveModule {
    pub name: String,
    pub package: MovePackage,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "MovePackage")]
pub struct MovePackage {
    pub address: Address,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "MoveType")]
pub struct MoveType {
    pub repr: Option<String>,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct MoveTypeLayout(pub String);

impl TryFrom<Event> for sui_types::types::Event {
    type Error = anyhow::Error;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        let Event {
            type_,
            sending_module,
            sender,
            bcs,
        } = value;

        let type_ = if let Some(t) = type_
            .repr
            .map(|layout| sui_types::types::StructTag::from_str(&layout))
            .transpose()
            .map_err(|e| anyhow::anyhow!("Invalid struct tag in event: {}", e))?
        {
            t
        } else {
            return Err(anyhow::anyhow!("Missing struct tag in event"));
        };

        let (package_id, module) = sending_module
            .map(|module| (module.package.address, module.name))
            .ok_or_else(|| anyhow::anyhow!("Missing sending module in event"))?;
        let package_id = package_id.into();
        let module = Identifier::from_str(&module)?;

        let sender = sender.map(|x| x.address).unwrap_or_else(|| Address::ZERO);

        let contents = base64ct::Base64::decode_vec(&bcs.0)
            .map_err(|_| anyhow::anyhow!("Invalid base64 in event"))?;

        Ok(Self {
            package_id,
            module,
            sender,
            type_,
            contents,
        })
    }
}
