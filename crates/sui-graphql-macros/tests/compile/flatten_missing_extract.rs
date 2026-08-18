//! A flattened field's type must provide `extract`. Rooted at `Query`, so the root type
//! check passes and only the missing method is reported.

use sui_graphql_macros::Response;

struct NotAResponse {
    chain_id: String,
}

#[derive(Response)]
struct QueryResponse {
    #[field(flatten)]
    metadata: NotAResponse,
}

fn main() {}
