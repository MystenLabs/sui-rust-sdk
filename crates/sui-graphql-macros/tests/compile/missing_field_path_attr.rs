use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct WrongFormat {
    // Missing `path = ` key
    #[field("object.address")]
    address: String,
}

fn main() {}
