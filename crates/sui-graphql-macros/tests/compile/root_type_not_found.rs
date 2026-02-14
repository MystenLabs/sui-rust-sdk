use sui_graphql_macros::Response;

/// Test that an invalid root_type that doesn't match any type produces an error.
#[derive(Response)]
#[response(root_type = "XyzNonExistent")]
struct BadRootType {
    #[field(path = "someField")]
    field: String,
}

fn main() {}
