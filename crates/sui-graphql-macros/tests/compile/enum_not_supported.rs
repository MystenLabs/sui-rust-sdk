use sui_graphql_macros::QueryResponse;

#[derive(QueryResponse)]
enum BadEnum {
    Variant1,
    Variant2,
}

fn main() {}
