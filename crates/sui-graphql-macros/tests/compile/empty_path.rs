use sui_graphql_macros::Response;

#[derive(Response)]
struct EmptyPath {
    #[field(path = "")]
    address: String,
}

fn main() {}
