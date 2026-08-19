use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct InvalidFlatten {
    #[field(path = "chainIdentifier", flatten)]
    chain_id: String,
}

fn main() {}
