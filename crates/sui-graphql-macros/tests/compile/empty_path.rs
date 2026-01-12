use sui_graphql_macros::QueryResponse;

#[derive(QueryResponse)]
struct EmptyPath {
    #[field(path = "")]
    address: String,
}

fn main() {}
