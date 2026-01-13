use sui_graphql_macros::Response;

#[derive(Response)]
struct InvalidField {
    #[field(path = "object.nonExistent")]
    address: String,
}

fn main() {}
