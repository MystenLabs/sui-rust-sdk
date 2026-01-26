//! GraphQL scalar types for Sui.

/// `SuiAddress` scalar in Sui GraphQL schema. 32-byte hex-encoded address with `0x` prefix.
pub use sui_sdk_types::Address;

/// Useful for digest fields (Base58 string). Not a scalar in Sui GraphQL schema.
pub use sui_sdk_types::Digest;
