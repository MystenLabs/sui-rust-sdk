use sui_graphql_macros::QueryResponse;

#[derive(QueryResponse)]
struct InvalidField {
    #[field(path = "object.nonExistent")]
    address: String,
}

fn main() {}
