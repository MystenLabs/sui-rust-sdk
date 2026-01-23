//! GraphQL client for the Sui blockchain.
//!
//! This crate provides a simple GraphQL client for interacting with
//! Sui's GraphQL API, with support for partial error handling.
//!
//! See [`Client`] for usage examples.

mod bcs;
mod client;
mod error;
mod pagination;
mod response;
pub mod scalars;

pub use bcs::Bcs;
pub use client::Client;
pub use client::chain::Epoch;
pub use client::coins::Balance;
pub use client::transactions::TransactionResponse;
pub use error::Error;
pub use error::GraphQLError;
pub use error::Location;
pub use error::PathFragment;
pub use pagination::Page;
pub use pagination::PageInfo;
pub use response::Response;
