use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct InvalidFieldAfterList {
    // 'nonExistent' does not exist on Checkpoint type
    #[field(path = "checkpoints.nodes[].nonExistent")]
    values: Vec<String>,
}

fn main() {}
