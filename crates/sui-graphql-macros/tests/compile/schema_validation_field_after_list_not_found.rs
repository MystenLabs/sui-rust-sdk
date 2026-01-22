use sui_graphql_macros::Response;

#[derive(Response)]
struct InvalidFieldAfterList {
    // 'nonExistent' does not exist on Checkpoint type
    #[field(path = "checkpoints.nodes[].nonExistent")]
    values: Vec<String>,
}

fn main() {}
