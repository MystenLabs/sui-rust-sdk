use sui_graphql_macros::QueryResponse;

#[derive(QueryResponse)]
struct InvalidFieldAfterList {
    // 'nonExistent' does not exist on Checkpoint type
    #[field(path = "checkpoints.nodes[].nonExistent")]
    values: Vec<String>,
}

fn main() {}
