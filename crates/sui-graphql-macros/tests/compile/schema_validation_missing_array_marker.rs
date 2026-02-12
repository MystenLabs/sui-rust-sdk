use sui_graphql_macros::Response;

// 'nodes' is a list type but missing [] in a non-last position
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct MissingArrayMarker {
    #[field(path = "checkpoints.nodes.digest")]
    digests: Vec<String>,
}

fn main() {}
