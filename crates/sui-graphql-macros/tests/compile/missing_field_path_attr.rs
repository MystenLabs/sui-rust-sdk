use sui_graphql_macros::QueryResponse;

#[derive(QueryResponse)]
struct WrongFormat {
    // Missing `path = ` key
    #[field("object.address")]
    address: String,
}

fn main() {}
