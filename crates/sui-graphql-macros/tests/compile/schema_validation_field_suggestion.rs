use sui_graphql_macros::Response;

#[derive(Response)]
struct TypoPath {
    // "chainIdentifer" is a typo of "chainIdentifier"
    #[field(path = "chainIdentifer")]
    chain_id: String,
}

fn main() {}
