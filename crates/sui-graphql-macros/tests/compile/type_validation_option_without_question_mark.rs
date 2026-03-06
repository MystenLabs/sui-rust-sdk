use sui_graphql_macros::Response;

#[derive(Response)]
struct R {
    #[field(path = "chainIdentifier")]
    id: Option<String>,
}

fn main() {}
