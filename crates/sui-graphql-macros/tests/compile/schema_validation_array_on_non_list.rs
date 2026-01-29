use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct InvalidArrayAccess {
    // 'address' is not a list type, cannot use []
    #[field(path = "object.address[]")]
    values: Vec<String>,
}

fn main() {}
