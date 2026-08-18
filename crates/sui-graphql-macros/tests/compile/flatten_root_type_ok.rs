//! An interface-rooted projection flattens into any type implementing it.

use sui_graphql_macros::Response;

#[derive(Response)]
#[response(root_type = "IObject")]
struct Metadata {
    #[field(path = "digest")]
    digest: String,
    #[field(path = "previousTransaction.digest")]
    previous_transaction: String,
}

// type Object implements Node & IAddressable & IObject
#[derive(Response)]
#[response(root_type = "Object")]
struct ObjectResponse {
    #[field(flatten)]
    metadata: Metadata,
}

// type DynamicField implements Node & IAddressable & IMoveObject & IObject
#[derive(Response)]
#[response(root_type = "DynamicField")]
struct DynamicFieldResponse {
    #[field(flatten)]
    metadata: Metadata,
}

fn main() {}
