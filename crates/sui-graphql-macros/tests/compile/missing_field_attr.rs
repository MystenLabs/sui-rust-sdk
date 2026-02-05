use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct BadStruct {
    // Missing #[field(path = "...")] attribute
    address: String,
}

fn main() {}
