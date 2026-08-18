//! A projection rooted at an unrelated type cannot be flattened.

use sui_graphql_macros::Response;

#[derive(Response)]
#[response(root_type = "Epoch")]
struct EpochMetadata {
    #[field(path = "epochId")]
    epoch_id: u64,
}

#[derive(Response)]
#[response(root_type = "Object")]
struct ObjectResponse {
    #[field(flatten)]
    metadata: EpochMetadata,
}

fn main() {}
