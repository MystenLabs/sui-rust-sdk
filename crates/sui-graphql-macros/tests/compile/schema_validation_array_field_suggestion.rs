use sui_graphql_macros::QueryResponse;

#[derive(QueryResponse)]
struct ArrayTypo {
    // "checkpointss" is a typo of "checkpoints"
    #[field(path = "checkpointss[].digest")]
    digests: Vec<String>,
}

fn main() {}
