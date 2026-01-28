use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct InvalidField {
    #[field(path = "object.nonExistent")]
    address: String,
}

fn main() {}
