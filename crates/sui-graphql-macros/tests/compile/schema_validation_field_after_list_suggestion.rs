use sui_graphql_macros::Response;

#[derive(Response)]
struct AfterListTypo {
    // "digst" is a typo of "digest"
    #[field(path = "checkpoints.nodes[].digst")]
    digests: Vec<String>,
}

fn main() {}
