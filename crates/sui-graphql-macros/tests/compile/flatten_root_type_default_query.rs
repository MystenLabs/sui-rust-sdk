//! A projection that derives `Response` without a `root_type` is rooted at `Query`.
//! Flattening it elsewhere is a root type mismatch, not a missing derive.

use sui_graphql_macros::Response;

#[derive(Response)]
struct QueryRooted {
    #[field(path = "chainIdentifier")]
    chain_id: String,
}

#[derive(Response)]
#[response(root_type = "Object")]
struct ObjectResponse {
    #[field(flatten)]
    metadata: QueryRooted,
}

fn main() {}
