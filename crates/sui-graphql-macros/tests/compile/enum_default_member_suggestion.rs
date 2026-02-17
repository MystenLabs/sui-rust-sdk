use sui_graphql_macros::Response;

// Variant ident "MoveValeu" is a typo — should suggest "MoveValue".
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql", root_type = "DynamicFieldValue")]
enum BadEnum {
    MoveValeu(String),
}

fn main() {}
