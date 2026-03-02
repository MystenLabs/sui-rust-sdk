use sui_graphql_macros::Response;

#[derive(Response)]
struct R {
    #[field(path = "chainIdentifier?")]
    id: String,
}

fn main() {}
