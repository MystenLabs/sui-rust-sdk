use sui_graphql_macros::Response;

// This should compile even though "nonexistentField" is not in the schema
#[derive(Response)]
struct SkipValidationResponse {
    #[field(path = "object.nonexistentField", skip_validation)]
    value: Option<String>,
}

fn main() {}
