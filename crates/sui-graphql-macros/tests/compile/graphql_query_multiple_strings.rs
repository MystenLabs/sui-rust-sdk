//! trybuild compile-pass: operations and fragments can be supplied as separate literals.

use sui_graphql_macros::graphql_query;

const Q: &str = graphql_query!(
    "query { ...ChainIdentifier }",
    "fragment ChainIdentifier on Query { chainIdentifier }",
);
const SPLIT_TOKEN: &str = graphql_query!("query { chainIdent", "ifier }");

fn main() {
    assert!(Q.contains("chainIdentifier"));
    assert!(SPLIT_TOKEN.contains("chainIdentifier"));
}
