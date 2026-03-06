# sui-graphql

[![sui-graphql on crates.io](https://img.shields.io/crates/v/sui-graphql)](https://crates.io/crates/sui-graphql)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/sui-graphql)
[![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/sui-rust-sdk/sui_graphql/)

A Rust client for interacting with the Sui blockchain via its GraphQL API.
Provides typed methods for querying chain state, objects, transactions,
checkpoints, epochs, executing transactions, and more. For custom queries,
the companion [`sui-graphql-macros`](https://crates.io/crates/sui-graphql-macros)
crate offers `#[derive(Response)]` for ergonomic, compile-time validated
response deserialization.
