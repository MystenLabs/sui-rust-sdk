use sui_graphql_macros::Response;

/// Test that alias syntax validates the real field name (after :) against the schema.
/// "nonExistentField" doesn't exist on Epoch type, so this should fail.
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct InvalidAlias {
    #[field(path = "epoch.myAlias:nonExistentField.nodes[].sequenceNumber")]
    data: Option<Vec<u64>>,
}

fn main() {}
