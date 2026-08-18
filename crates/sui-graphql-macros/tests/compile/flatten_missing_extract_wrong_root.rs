//! A type that provides neither `extract` nor a root type is wrong twice over, so both
//! diagnostics are reported.

use sui_graphql_macros::Response;

struct NotAResponse {
    digest: String,
}

#[derive(Response)]
#[response(root_type = "Object")]
struct ObjectResponse {
    #[field(flatten)]
    metadata: NotAResponse,
}

fn main() {}
