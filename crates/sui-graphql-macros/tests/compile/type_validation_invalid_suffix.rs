use sui_graphql_macros::Response;

#[derive(Response)]
struct R {
    #[field(path = "object.address[?]")]
    addr: String,
}

fn main() {}
