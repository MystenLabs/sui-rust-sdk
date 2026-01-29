use sui_graphql_macros::Response;

// This should compile even though "nonexistentField" is not in the schema
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct SkipValidationResponse {
    #[field(path = "object.nonexistentField", skip_schema_validation)]
    value: Option<String>,
}

fn main() {}
