use sui_graphql_macros::Response;

// Path has [] but type is not Vec
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct TooManyArraysInPath {
    #[field(path = "items[].name")]
    names: String,
}

// Path has no [] but type is Vec
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct TooFewArraysInPath {
    #[field(path = "chainIdentifier")]
    ids: Vec<String>,
}

fn main() {}
