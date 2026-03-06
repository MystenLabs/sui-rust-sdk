# sui-graphql-macros

[![sui-graphql-macros on crates.io](https://img.shields.io/crates/v/sui-graphql-macros)](https://crates.io/crates/sui-graphql-macros)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/sui-graphql-macros)
[![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/sui-rust-sdk/sui_graphql_macros/)

A `#[derive(Response)]` macro that generates deserialization code for Sui GraphQL
JSON responses, with field paths validated against the schema at compile time.
