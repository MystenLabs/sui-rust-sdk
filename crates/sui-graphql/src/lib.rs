//! GraphQL client for the Sui blockchain.
//!
//! This crate provides a simple GraphQL client for interacting with
//! Sui's GraphQL API, with support for partial error handling.
//!
//! # Example
//!
//! ```ignore
//! use sui_graphql::Client;
//! use serde::Deserialize;
//! use serde_json::json;
//!
//! #[derive(Debug, Deserialize)]
//! struct ChainResponse {
//!     #[serde(rename = "chainIdentifier")]
//!     chain_identifier: String,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), sui_graphql::Error> {
//!     let client = Client::new("https://sui-testnet.mystenlabs.com/graphql");
//!
//!     let query = "query { chainIdentifier }";
//!     let response = client.query::<ChainResponse>(query, json!({})).await?;
//!
//!     // Check for partial errors
//!     if response.has_errors() {
//!         for err in response.errors() {
//!             eprintln!("Error: {}", err.message);
//!         }
//!     }
//!
//!     // Access the data
//!     if let Some(data) = response.data {
//!         println!("Chain: {}", data.chain_identifier);
//!     }
//!
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod response;

pub use client::Client;
pub use error::{Error, GraphQLError, Location, PathFragment};
pub use response::Response;
