use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct AfterListTypo {
    // "digst" is a typo of "digest"
    #[field(path = "checkpoints.nodes[].digst")]
    digests: Vec<String>,
}

fn main() {}
