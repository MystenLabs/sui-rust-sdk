# Sui Rust Sdk

A rust sdk for integrating with the [Sui blockchain](https://docs.sui.io/).

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
* [`sui-rpc`](crates/sui-rpc)
    [![sui-rpc on crates.io](https://img.shields.io/crates/v/sui-rpc)](https://crates.io/crates/sui-rpc)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/sui-rpc)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/sui-rust-sdk/sui_rpc/)
* [`sui-transaction-builder`](crates/sui-transaction-builder)
    [![sui-transaction-builder on crates.io](https://img.shields.io/crates/v/sui-transaction-builder)](https://crates.io/crates/sui-transaction-builder)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/sui-transaction-builder)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/sui-rust-sdk/sui_transaction_builder/)

## License

This project is available under the terms of the [Apache 2.0 license](LICENSE).
