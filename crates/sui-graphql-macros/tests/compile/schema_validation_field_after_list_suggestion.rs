use sui_graphql_macros::QueryResponse;

#[derive(QueryResponse)]
struct AfterListTypo {
    // "digst" is a typo of "digest"
    #[field(path = "checkpoints.nodes[].digst")]
    digests: Vec<String>,
}

fn main() {}
