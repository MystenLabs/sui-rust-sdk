# Sui Sdk

A rust Sdk for integrating with the [Sui blockchain](https://docs.sui.io/).

> [!NOTE]
> This is project is under development and many features may still be under
> development or missing.

## Overview

This repository contains a collection of libraries for integrating with the Sui blockchain.

A few of the project's high-level goals are as follows:

* **Be modular** - user's should only need to pay the cost (in terms of dependencies/compilation time) for the features that they use.
* **Be light** - strive to have a minimal dependency footprint.
* **Support developers** - provide all needed types, abstractions and APIs to enable developers to build robust applications on Sui.
* **Support wasm** - where possible, libraries should be usable in wasm environments.

## Crates

In an effort to be modular, functionality is split between a number of crates.

* [`sui-sdk-types`](crates/sui-sdk-types)
    [![sui-sdk-types on crates.io](https://img.shields.io/crates/v/sui-sdk-types)](https://crates.io/crates/sui-sdk-types)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/sui-sdk-types)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/sui-rust-sdk/sui_sdk_types/)
* [`sui-crypto`](crates/sui-crypto)
    [![sui-crypto on crates.io](https://img.shields.io/crates/v/sui-crypto)](https://crates.io/crates/sui-crypto)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/sui-crypto)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/sui-rust-sdk/sui_crypto/)
* [`sui-graphql-client`](crates/sui-crypto)
    [![sui-graphql-client on crates.io](https://img.shields.io/crates/v/sui-graphql-client)](https://crates.io/crates/sui-graphql-client)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/sui-graphql-client)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/sui-rust-sdk/sui-graphql-client/)

## Examples

Runnable examples live in the top-level [examples](examples/) crate.

| Example | Focus | What it shows |
|---------|-------|---------------|
| `graphql_basic.rs` | Core GraphQL reads | Connect to public testnet; fetch chain id, SUI balance for address `0x1`, first page of coins, latest checkpoint summary. |
| `objects_read.rs` | Object listing | List objects owned by a demo address (first page) and debug-print the first object. |
| `governance_basic.rs` | Network / governance data | Active validators (subset), reference gas price, protocol version, total transaction blocks. |
| `transaction_build.rs` | Transaction construction | Build (but do not execute) a simple programmable transaction using `sui-transaction-builder`. |

### Run examples

From the repository root (network access required for GraphQL queries):

```shell
cargo run --example graphql_basic
cargo run --example objects_read
cargo run --example governance_basic
cargo run --example transaction_build
```

Each example is small and self-contained; open the corresponding file under `examples/` to explore the code.

## License

This project is available under the terms of the [Apache 2.0 license](LICENSE).
