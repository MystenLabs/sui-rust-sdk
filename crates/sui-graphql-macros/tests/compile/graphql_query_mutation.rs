//! trybuild compile-pass: a mutation validates and formats just like a query.

use sui_graphql_macros::graphql_query;

const M: &str = graphql_query!(
    "mutation($txDataBcs: Base64!, $signatures: [Base64!]!) {
        executeTransaction(transactionDataBcs: $txDataBcs, signatures: $signatures) {
            effects {
                effectsBcs
            }
        }
    }"
);

fn main() {
    assert!(!M.is_empty());
}
