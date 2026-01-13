use sui_graphql_macros::QueryResponse;

#[derive(QueryResponse)]
struct TypoPath {
    // "chainIdentifer" is a typo of "chainIdentifier"
    #[field(path = "chainIdentifer")]
    chain_id: String,
}

fn main() {}
