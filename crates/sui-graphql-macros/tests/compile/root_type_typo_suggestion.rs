use sui_graphql_macros::Response;

/// Test that an invalid root_type with a typo produces a "Did you mean?" suggestion.
#[derive(Response)]
#[response(root_type = "Mutaton")]
struct TypoRootType {
    #[field(path = "executeTransaction")]
    tx: Option<String>,
}

fn main() {}
