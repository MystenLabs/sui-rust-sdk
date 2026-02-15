use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql", root_type = "Object")]
enum BadEnum {
    #[response(on = "MoveValue")]
    Value(String),
}

fn main() {}
