use sui_graphql_macros::Response;

#[derive(Response)]
struct BadStruct {
    // Missing #[field(path = "...")] attribute
    address: String,
}

fn main() {}
