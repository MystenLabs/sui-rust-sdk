use sui_graphql_macros::Response;

#[derive(Response)]
struct WrongFormat {
    // Missing `path = ` key
    #[field("object.address")]
    address: String,
}

fn main() {}
