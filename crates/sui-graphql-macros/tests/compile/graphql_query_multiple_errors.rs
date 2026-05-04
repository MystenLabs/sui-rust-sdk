//! trybuild compile-fail: a mix of one valid field and two invalid ones
//! should produce two separate rustc errors (one per apollo diagnostic), not
//! a single combined message.

use sui_graphql_macros::graphql_query;

const Q: &str = graphql_query!(
    "query {
        chainIdentifier
        foo
        bar
    }"
);

fn main() {
    let _ = Q;
}
