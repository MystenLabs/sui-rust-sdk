use sui_graphql_macros::Response;

#[derive(Response)]
struct InvalidField {
    #[field(path = "object.nonExistentField")]
    address: String,
}

fn main() {}
