//! A concrete type's projection cannot be flattened into an interface-rooted response:
//! the interface has other implementors that would not carry `Object`'s fields.

use sui_graphql_macros::Response;

#[derive(Response)]
#[response(root_type = "Object")]
struct ObjectMetadata {
    #[field(path = "digest")]
    digest: String,
}

#[derive(Response)]
#[response(root_type = "IObject")]
struct IObjectResponse {
    #[field(flatten)]
    metadata: ObjectMetadata,
}

fn main() {}
