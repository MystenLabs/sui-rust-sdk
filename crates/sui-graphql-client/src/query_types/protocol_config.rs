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
/// Constants that control how the chain operates.
/// These can only change during protocol upgrades which happen on epoch boundaries.
#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema = "rpc", graphql_type = "ProtocolConfigs")]
pub struct ProtocolConfigs {
    /// The protocol is not required to change on every epoch boundary, so the protocol version
    /// tracks which change to the protocol these configs are from.
    pub protocol_version: u64,
    /// List all available feature flags and their values. Feature flags are a form of boolean
    /// configuration that are usually used to gate features while they are in development. Once a
    /// flag has been enabled, it is rare for it to be disabled.
    pub feature_flags: Vec<ProtocolConfigFeatureFlag>,
    /// List all available configurations and their values. These configurations can take any value
    /// (but they will all be represented in string form), and do not include feature flags.
    pub configs: Vec<ProtocolConfigAttr>,
}

/// Feature flags are a form of boolean configuration that are usually used to gate features while
/// they are in development. Once a lag has been enabled, it is rare for it to be disabled.
#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema = "rpc", graphql_type = "ProtocolConfigFeatureFlag")]
pub struct ProtocolConfigFeatureFlag {
    pub key: String,
    pub value: bool,
}

/// A key-value protocol configuration attribute.
#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema = "rpc", graphql_type = "ProtocolConfigAttr")]
pub struct ProtocolConfigAttr {
    pub key: String,
    pub value: Option<String>,
}
