#![cfg_attr(doc_cfg, feature(doc_cfg))]

pub mod client;
pub mod field;
pub mod headers;
pub mod merge;
pub mod proto;

#[cfg(feature = "faucet")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "faucet")))]
pub mod faucet;

#[cfg(feature = "unstable")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "unstable")))]
pub mod light_client;

mod rt;

pub use client::Client;
pub use rt::MaybeSend;
pub use rt::MaybeSync;

#[doc(hidden)]
pub mod _serde;
