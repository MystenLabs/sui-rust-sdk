// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::query_types::schema;

// ===========================================================================
// Protocol Config Queries
// ===========================================================================

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

// ===========================================================================
// Protocol Version Args
// ===========================================================================

#[derive(cynic::QueryVariables, Debug)]
pub struct ProtocolVersionArgs {
    pub id: Option<u64>,
}

// ===========================================================================
// Protocol Config Types
// ===========================================================================

/// Information about the configuration of the protocol.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "ProtocolConfigs")]
pub struct ProtocolConfigs {
    pub protocol_version: u64,
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
