//! trybuild compile-pass: query with typed variables resolves against the schema.

use sui_graphql_macros::graphql_query;

const Q: &str = graphql_query!(
    "query($id: SuiAddress!) {
        object(address: $id) {
            address
            version
        }
    }"
);

fn main() {
    assert!(!Q.is_empty());
}
