use sui_graphql_macros::QueryResponse;

// This should compile even though "nonexistentField" is not in the schema
#[derive(QueryResponse)]
struct Response {
    #[field(path = "object.nonexistentField", skip_validation)]
    value: Option<String>,
}

fn main() {}
