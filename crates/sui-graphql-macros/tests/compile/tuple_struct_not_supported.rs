use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct TupleStruct(String, u64);

fn main() {}
