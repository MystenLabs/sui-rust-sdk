//! A projection rooted at one member of a union cannot be flattened into a response
//! rooted at that union: the union resolves to any of its members at runtime, and the
//! projection only covers one of them.

use sui_graphql_macros::Response;

// `contents` exists on `MoveObject`, but not on `MoveValue`.
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql", root_type = "MoveObject")]
struct MO {
    #[field(path = "contents.type.repr")]
    content: String,
}

// union DynamicFieldValue = MoveObject | MoveValue
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql", root_type = "DynamicFieldValue")]
struct DF {
    #[field(flatten)]
    obj: MO,
}

fn main() {}
