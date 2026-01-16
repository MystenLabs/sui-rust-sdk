use sui_graphql_macros::Response;

/// Test that alias syntax validates the real field name (before @) against the schema.
/// "nonExistentField" doesn't exist on Epoch type, so this should fail.
#[derive(Response)]
struct InvalidAlias {
    #[field(path = "epoch.nonExistentField@myAlias.nodes[].sequenceNumber")]
    data: Option<Vec<u64>>,
}

fn main() {}
