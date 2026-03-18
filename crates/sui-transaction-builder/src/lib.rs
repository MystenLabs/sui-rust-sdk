// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Transaction building API for the [Sui] blockchain.
//!
//! This crate provides [`TransactionBuilder`], for constructing user transactions or PTBs
//! (programmable transaction blocks). Inputs and commands are added incrementally, and the builder
//! resolves them into a finalized [`Transaction`](sui_sdk_types::Transaction).
//!
//! [Sui]: https://sui.io
//!
//! # Feature flags
//!
//! - `intents` *(enabled by default)*: Enables high-level transaction intents (e.g.,
//!   [`CoinWithBalance`](intent::CoinWithBalance)) and the async [`TransactionBuilder::build`]
//!   method that resolves intents and gas via an RPC client.
//!
//! # Example
//!
//! Build a simple SUI transfer transaction offline using [`TransactionBuilder::try_build`]:
//!
//! ```
//! use sui_sdk_types::Address;
//! use sui_sdk_types::Digest;
//! use sui_transaction_builder::ObjectInput;
//! use sui_transaction_builder::TransactionBuilder;
//!
//! let mut tx = TransactionBuilder::new();
//!
//! // Split 1 SUI from the gas coin
//! let amount = tx.pure(&1_000_000_000u64);
//! let gas = tx.gas();
//! let coins = tx.split_coins(gas, vec![amount]);
//!
//! // Transfer to recipient
//! let recipient = tx.pure(&Address::ZERO);
//! tx.transfer_objects(coins, recipient);
//!
//! // Set required metadata
//! tx.set_sender(Address::ZERO);
//! tx.set_gas_budget(500_000_000);
//! tx.set_gas_price(1000);
//! tx.add_gas_objects([ObjectInput::owned(Address::ZERO, 1, Digest::ZERO)]);
//!
//! let transaction = tx.try_build().expect("build should succeed");
//! ```

#![cfg_attr(doc_cfg, feature(doc_cfg))]

mod builder;
mod error;
#[cfg(feature = "intents")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "intents")))]
pub mod intent;

pub use builder::Argument;
pub use builder::Function;
pub use builder::ObjectInput;
pub use builder::TransactionBuilder;
pub use error::Error;
#[cfg(feature = "intents")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "intents")))]
pub use error::SimulationFailure;
