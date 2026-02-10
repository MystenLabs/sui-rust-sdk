//! GraphQL client for the Sui blockchain.
//!
//! This crate provides a simple GraphQL client for interacting with
//! Sui's GraphQL API, with support for partial error handling.
//!
//! See [`Client`] for usage examples.

mod bcs;
mod client;
mod error;
mod json;
mod move_value;
mod pagination;
mod response;
pub mod scalars;

pub use bcs::Bcs;
pub use bcs::BcsBytes;
pub use client::Client;
pub use client::chain::Epoch;
pub use client::checkpoints::CheckpointResponse;
pub use client::coins::Balance;
pub use client::dynamic_fields::DynamicField;
pub use client::dynamic_fields::DynamicFieldType;
pub use client::dynamic_fields::DynamicFieldValue;
pub use client::transactions::TransactionResponse;
pub use error::Error;
pub use error::GraphQLError;
pub use error::Location;
pub use error::PathFragment;
pub use json::JsonValue;
pub use move_value::MoveValue;
pub use pagination::Page;
pub use pagination::PageInfo;
pub use response::Response;
