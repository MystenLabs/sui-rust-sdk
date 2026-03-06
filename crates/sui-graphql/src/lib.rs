//! GraphQL client for the [Sui] blockchain.
//!
//! [Sui]: https://sui.io
//!
//! This crate provides a typed GraphQL client for Sui's GraphQL API with
//! automatic BCS deserialization and pagination support.
//!
//! # Quick Start
//!
//! ```no_run
//! use sui_graphql::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(Client::MAINNET)?;
//!
//!     // Chain info
//!     let chain_id = client.chain_identifier().await?;
//!     println!("Chain: {chain_id}");
//!
//!     // Fetch objects, transactions, checkpoints
//!     let obj = client.get_object("0x5".parse()?).await?;
//!     let tx = client.get_transaction("digest...").await?;
//!     let cp = client.get_checkpoint(None).await?; // latest
//!
//!     Ok(())
//! }
//! ```
//!
//! # Streaming
//!
//! List methods return async streams with automatic pagination:
//!
//! ```no_run
//! use futures::StreamExt;
//! use std::pin::pin;
//! use sui_graphql::Client;
//! use sui_sdk_types::Address;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(Client::MAINNET)?;
//!     let owner: Address = "0x123...".parse()?;
//!
//!     let mut stream = pin!(client.list_objects(owner));
//!     while let Some(obj) = stream.next().await {
//!         let obj = obj?;
//!         println!("Object version: {}", obj.version());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! # Custom Queries
//!
//! For queries beyond the built-in methods, use [`Client::query`] with a
//! response type that implements [`serde::de::DeserializeOwned`]. The
//! [`sui-graphql-macros`] crate provides `#[derive(Response)]` which generates
//! the deserialization code from declarative field paths, with compile-time
//! validation against the Sui GraphQL schema.
//!
//! [`sui-graphql-macros`]: https://docs.rs/sui-graphql-macros
//!
//! ```no_run
//! use sui_graphql::Client;
//! use sui_graphql_macros::Response;
//!
//! // Define a response type with field paths into the GraphQL response JSON.
//! // Paths are validated against the schema at compile time — typos like
//! // "epoch.epochIdd" will produce a compile error with a "Did you mean?" suggestion.
//! #[derive(Response)]
//! struct MyResponse {
//!     #[field(path = "epoch.epochId")]
//!     epoch_id: u64,
//!     // Use `[]` to extract items from a list field
//!     #[field(path = "epoch.checkpoints.nodes[].digest")]
//!     checkpoint_digests: Vec<String>,
//!     // Use `?` to mark nullable fields — null returns Ok(None) instead of an error.
//!     // Without `?`, a null value at that segment is a runtime error.
//!     #[field(path = "epoch.referenceGasPrice?")]
//!     gas_price: Option<u64>,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), sui_graphql::Error> {
//!     let client = Client::new(Client::MAINNET)?;
//!
//!     let query = r#"
//!         query($epoch_id: UInt53) {
//!             epoch(id: $epoch_id) {
//!                 epochId
//!                 checkpoints { nodes { digest } }
//!                 referenceGasPrice
//!             }
//!         }
//!     "#;
//!     let variables = serde_json::json!({ "epoch_id": 100 });
//!
//!     let response = client.query::<MyResponse>(query, variables).await?;
//!
//!     // GraphQL supports partial success — data and errors can coexist
//!     for err in response.errors() {
//!         eprintln!("GraphQL error: {}", err.message());
//!     }
//!     if let Some(data) = response.data() {
//!         println!("Epoch: {}", data.epoch_id);
//!         println!("Checkpoints: {:?}", data.checkpoint_digests);
//!         println!("Gas price: {:?}", data.gas_price);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! For the full path syntax reference (`?`, `[]`, aliases, enums), see the
//! [`sui-graphql-macros` documentation](https://docs.rs/sui-graphql-macros).
//!
//! See [`Client`] for the full list of available methods.

mod bcs;
mod client;
mod error;
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
pub use client::dynamic_fields::DynamicFieldRequest;
pub use client::dynamic_fields::DynamicFieldValue;
pub use client::dynamic_fields::DynamicFieldsRequest;
pub use client::dynamic_fields::Format;
pub use client::execution::ExecutionResult;
pub use client::transactions::TransactionResponse;
pub use error::Error;
pub use error::GraphQLError;
pub use error::Location;
pub use error::PathFragment;
pub use move_value::MoveObject;
pub use move_value::MoveValue;
pub use pagination::Page;
pub use pagination::PageInfo;
pub use pagination::paginate;
pub use pagination::paginate_backward;
pub use response::Response;
