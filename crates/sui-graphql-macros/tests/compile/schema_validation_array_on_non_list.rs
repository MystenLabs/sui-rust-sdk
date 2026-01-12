use sui_graphql_macros::QueryResponse;

#[derive(QueryResponse)]
struct InvalidArrayAccess {
    // 'address' is not a list type, cannot use []
    #[field(path = "object.address[]")]
    values: Vec<String>,
}

fn main() {}
