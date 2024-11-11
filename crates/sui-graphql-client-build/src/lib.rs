// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../README.md")]

/// Register the schema to enable building custom queries using cynic derive macros queries. Call
/// this function in a `build.rs` file in your crate if you need to build custom queries.
///
/// Examples
/// ```rust,ignore
/// // build.rs file
/// fn main() {
///     let schema_name = "MYSCHEMA"
///     sui_graphql_client_build::register_schema(schema_name);
/// }
///
/// // Cargo.toml
/// ...
/// [dependencies]
/// cynic = "3.8.0"
/// ...
/// [build-dependencies]
/// sui_graphql_client_build = "VERSION_HERE"
///
/// // lib.rs
/// // Custom query
/// use cynic::QueryBuilder;
/// use sui_graphql_client::{query_types::schema, Client};
///
/// #[derive(cynic::QueryFragment, Debug)]
/// #[cynic(schema = "MYSCHEMA", graphql_type = "Query")]
/// pub struct MyQuery {
///    pub chain_identifier: String,
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new_mainnet();
///     let operation = MyQuery::build(());
///     let q = client.run_query(&operation).await.unwrap();
///     println!("{:?}", q);
/// }
/// ```
pub fn register_schema(schema_name: &str) {
    let sdl = include_str!("../schema.graphql");
    cynic_codegen::register_schema(schema_name)
        .from_sdl(sdl)
        .expect("Failed to find GraphQL Schema")
        .as_default()
        .unwrap();
}
