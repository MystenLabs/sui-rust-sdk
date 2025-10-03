#![cfg_attr(doc_cfg, feature(doc_cfg))]

pub mod client;
pub mod field;
pub mod headers;
pub mod merge;
pub mod proto;

#[cfg(feature = "faucet")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "faucet")))]
pub mod faucet;

pub use client::Client;

#[doc(hidden)]
mod _serde;
