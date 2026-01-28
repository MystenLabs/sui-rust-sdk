use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
enum BadEnum {
    Variant1,
    Variant2,
}

fn main() {}
