use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct EmptyPath {
    #[field(path = "")]
    address: String,
}

fn main() {}
