use sui_graphql_macros::QueryResponse;

#[derive(QueryResponse)]
struct BadStruct {
    // Missing #[field(path = "...")] attribute
    address: String,
}

fn main() {}
