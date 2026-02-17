use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql", root_type = "DynamicFieldValue")]
enum BadEnum {
    #[response(on = "NonExistentType")]
    Value(String),
}

fn main() {}
