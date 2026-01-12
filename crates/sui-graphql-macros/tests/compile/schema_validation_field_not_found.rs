use sui_graphql_macros::QueryResponse;

#[derive(QueryResponse)]
struct InvalidField {
    #[field(path = "object.nonExistentField")]
    address: String,
}

fn main() {}
