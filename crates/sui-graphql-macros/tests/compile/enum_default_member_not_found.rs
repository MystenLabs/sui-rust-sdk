use sui_graphql_macros::Response;

// Variant ident "Value" doesn't match any member of DynamicFieldValue.
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql", root_type = "DynamicFieldValue")]
enum BadEnum {
    Value(String),
}

fn main() {}
