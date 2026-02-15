use sui_graphql_macros::Response;

// Without root_type, defaults to "Query" which is not a union type.
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
enum BadEnum {
    #[response(on = "MoveValue")]
    Value(String),
}

fn main() {}
