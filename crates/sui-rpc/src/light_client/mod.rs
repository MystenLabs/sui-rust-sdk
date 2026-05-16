//! Light-client helpers for verifying state against a trusted validator
//! committee.
//!
//! This module composes the pieces that the rest of the crate provides
//! into a small end-to-end verifier:
//!
//! - [`EpochCache`] stores the currently-trusted validator committee and
//!   remembers each completed epoch as a closed checkpoint range.
//! - The committee ratchet driver (in a follow-up commit) advances the
//!   cache forward by fetching and BLS-verifying each epoch's last
//!   checkpoint summary.
//! - The high-level `LightClient` (in a follow-up commit) ties together
//!   the alpha `ProofService` (see [`crate::proto::sui::rpc::v2alpha`]),
//!   the ratchet, and the OCS proof verifier in
//!   `sui_sdk_types::proof` to authenticate `ObjectReference`s against
//!   the network's validator committee end-to-end.
//!
//! The whole module is gated on the crate's `unstable` feature.

mod client;
mod epoch_cache;
mod error;
pub mod events;
mod ratchet;

pub use client::LightClient;
pub use epoch_cache::EpochCache;
pub use error::LightClientError;
pub use events::AuthenticatedEvent;
pub use ratchet::ratchet_to_checkpoint;
