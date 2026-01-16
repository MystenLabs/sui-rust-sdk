//! GraphQL client for the Sui blockchain.
//!
//! This crate provides a simple GraphQL client for interacting with
//! Sui's GraphQL API, with support for partial error handling.
//!
//! See [`Client`] for usage examples.

mod client;
mod error;
mod response;

pub use client::Client;
pub use error::{Error, GraphQLError, Location, PathFragment};
pub use response::Response;
