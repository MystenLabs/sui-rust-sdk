//! Move value types for JSON and BCS representations.
//!
//! These types provide a unified interface for working with Move values
//! returned from GraphQL queries, supporting both JSON and BCS formats.

use sui_graphql_macros::Response;
use sui_sdk_types::Address;
use sui_sdk_types::TypeTag;

use crate::bcs::BcsBytes;

/// A Move value with type information and optional JSON/BCS representations.
///
/// This type can be used in Response structs to extract Move values from GraphQL.
/// The JSON and BCS fields are optional - which ones are populated depends on
/// what fields were requested in the GraphQL query.
#[derive(Debug, Clone, Response)]
#[response(root_type = "MoveValue")]
pub struct MoveValue {
    /// The Move type of this value.
    #[field(path = "type.repr")]
    pub type_tag: TypeTag,

    /// JSON representation (if fetched).
    #[field(path = "json")]
    pub json: Option<serde_json::Value>,

    /// BCS representation (if fetched).
    #[field(path = "bcs")]
    pub bcs: Option<BcsBytes>,
}

/// A Move object with its address and contents.
#[derive(Debug, Clone, Response)]
#[response(root_type = "MoveObject")]
pub struct MoveObject {
    /// The object's address.
    #[field(path = "address")]
    pub address: Address,
    /// The object's contents as a MoveValue.
    #[field(path = "contents")]
    pub contents: MoveValue,
}
