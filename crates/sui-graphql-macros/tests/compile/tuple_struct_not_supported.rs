use sui_graphql_macros::Response;

#[derive(Response)]
struct TupleStruct(String, u64);

fn main() {}
