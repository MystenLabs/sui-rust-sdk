//! trybuild compile-fail: variable is declared but the reference has a typo
//! (declared as `$id`, referenced as `$idd`).

use sui_graphql_macros::graphql_query;

const Q: &str = graphql_query!(
    "query($id: SuiAddress!) {
        object(address: $idd) {
            version
        }
    }"
);

fn main() {
    let _ = Q;
}
