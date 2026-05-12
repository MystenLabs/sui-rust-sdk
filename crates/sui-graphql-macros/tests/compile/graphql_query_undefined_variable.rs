//! trybuild compile-fail: referencing an undeclared variable is rejected.

use sui_graphql_macros::graphql_query;

const Q: &str = graphql_query!(
    "query {
        object(address: $id) {
            version
        }
    }"
);

fn main() {
    let _ = Q;
}
