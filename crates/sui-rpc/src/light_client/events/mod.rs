//! Authenticated event stream verification.
//!
//! [`AuthenticatedEventsClient`] follows the ledger event stream
//! ([`crate::Client::stream_events_with_config`] over the v2
//! `LedgerService.ListEvents` RPC filtered by `EventStreamHeadFilter`),
//! replays each received event into a local Merkle Mountain Range, and
//! reconciles that MMR against the on-chain [`EventStreamHead`] fetched
//! via an OCS inclusion proof. A checkpoint's events are folded into the
//! MMR only after the event stream claims that checkpoint complete, and
//! events are only yielded to the consumer after the reconciliation that
//! covers them succeeds, so every emitted event is cryptographically
//! authenticated end-to-end.
//!
//! [`AuthenticatedEvent`] is the in-memory envelope used both as the
//! decoded form of a ledger event and as the unit of work flowing
//! through the verifier.
//!
//! [`EventStreamHead`]: sui_sdk_types::framework::EventStreamHead

mod client;
mod config;
mod envelope;
mod state;

pub use client::AuthenticatedEventsClient;
pub use config::AuthenticatedEventsConfig;
pub use envelope::AuthenticatedEvent;
