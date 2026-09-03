//! A flattened field's type must provide `extract`. This one names no root either, so
//! it is wrong twice over and both are reported.

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
