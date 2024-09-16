// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::query_types::schema;

// ===========================================================================
// Service Config Query
// ===========================================================================

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Query")]
pub struct ServiceConfigQuery {
    pub service_config: ServiceConfig,
}

// ===========================================================================
// Service Config Types
// ===========================================================================

// Information about the configuration of the GraphQL service.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "ServiceConfig")]
pub struct ServiceConfig {
    /// List the available versions for this GraphQL service.
    pub available_versions: Vec<String>,
    /// Default number of elements allowed on a single page of a connection.
    pub default_page_size: i32,
    /// List of all features that are enabled on this RPC service.
    pub enabled_features: Vec<Feature>,
    // TODO This field is retrieved as a string, instead of i32
    /// Maximum estimated cost of a database query used to serve a GraphQL request.  This is
    /// measured in the same units that the database uses in EXPLAIN queries.
    // pub max_db_query_cost: i32,
    /// Maximum nesting allowed in struct fields when calculating the layout of a single Move Type.
    pub max_move_value_depth: i32,
    /// The maximum number of output nodes in a GraphQL response.
    /// Non-connection nodes have a count of 1, while connection nodes are counted as
    /// the specified 'first' or 'last' number of items, or the default_page_size
    /// as set by the server if those arguments are not set.
    /// Counts accumulate multiplicatively down the query tree. For example, if a query starts
    /// with a connection of first: 10 and has a field to a connection with last: 20, the count
    /// at the second level would be 200 nodes. This is then summed to the count of 10 nodes
    /// at the first level, for a total of 210 nodes.
    pub max_output_nodes: i32,
    /// Maximum number of elements allowed on a single page of a connection.
    pub max_page_size: i32,
    /// The maximum depth a GraphQL query can be to be accepted by this service.
    pub max_query_depth: i32,
    /// The maximum number of nodes (field names) the service will accept in a single query.
    pub max_query_nodes: i32,
    /// Maximum length of a query payload string.
    pub max_query_payload_size: i32,
    /// Maximum nesting allowed in type arguments in Move Types resolved by this service.
    pub max_type_argument_depth: i32,
    /// Maximum number of type arguments passed into a generic instantiation of a Move Type resolved
    /// by this service.
    pub max_type_argument_width: i32,
    /// Maximum number of structs that need to be processed when calculating the layout of a single
    /// Move Type.
    pub max_type_nodes: i32,
    /// Maximum time in milliseconds spent waiting for a response from fullnode after issuing a
    /// a transaction to execute. Note that the transaction may still succeed even in the case of a
    /// timeout. Transactions are idempotent, so a transaction that times out should be resubmitted
    /// until the network returns a definite response (success or failure, not timeout).
    pub mutation_timeout_ms: i32,
    /// Maximum time in milliseconds that will be spent to serve one query request.
    pub request_timeout_ms: i32,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Feature",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum Feature {
    Analytics,
    Coins,
    DynamicFields,
    NameService,
    Subscriptions,
    SystemState,
}
