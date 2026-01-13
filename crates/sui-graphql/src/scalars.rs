//! GraphQL scalar types for Sui.

/// `SuiAddress` scalar in Sui GraphQL schema. 32-byte hex-encoded address with `0x` prefix.
pub use sui_sdk_types::Address;

/// Useful for digest fields (Base58 string). Not a scalar in Sui GraphQL schema.
pub use sui_sdk_types::Digest;

/// `UInt53` scalar in Sui GraphQL schema. Unsigned integer up to 2^53 - 1.
pub type UInt53 = u64;

/// `BigInt` scalar in Sui GraphQL schema. Arbitrary precision signed integer as string.
pub type BigInt = String;

/// `Base64` scalar in Sui GraphQL schema. Base64-encoded binary data.
pub type Base64 = String;

/// `DateTime` scalar in Sui GraphQL schema. ISO-8601 timestamp (YYYY-MM-DDTHH:MM:SS.mmmZ).
pub type DateTime = String;

/// `JSON` scalar in Sui GraphQL schema. Arbitrary JSON data.
pub type Json = serde_json::Value;

/// `MoveTypeLayout` scalar in Sui GraphQL schema. JSON representation of a Move type's memory layout.
pub type MoveTypeLayout = serde_json::Value;

/// `MoveTypeSignature` scalar in Sui GraphQL schema. JSON representation of a Move type signature.
pub type MoveTypeSignature = serde_json::Value;

/// `OpenMoveTypeSignature` scalar in Sui GraphQL schema. JSON for open Move type signature (may have type params).
pub type OpenMoveTypeSignature = serde_json::Value;
