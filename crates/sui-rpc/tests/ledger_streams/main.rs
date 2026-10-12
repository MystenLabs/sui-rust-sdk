// These tests host a mock gRPC server on tokio; `tests/transport.rs` covers the client on wasm32.
#![cfg(not(target_arch = "wasm32"))]

mod list;
mod malformed_protocol;
mod observability;
mod polling;
mod reconnect;
mod startup;
mod support;
