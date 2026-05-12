//! trybuild compile-pass: a minimal valid query against the embedded schema.

use sui_graphql_macros::graphql_query;

const Q: &str = graphql_query!("query { chainIdentifier }");

fn main() {
    assert!(!Q.is_empty());
}
