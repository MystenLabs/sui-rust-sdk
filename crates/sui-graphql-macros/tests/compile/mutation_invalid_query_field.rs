use sui_graphql_macros::Response;

// This should fail: "epoch" is a Query field, not a Mutation field
#[derive(Response)]
#[response(mutation)]
struct InvalidResponse {
    #[field(path = "epoch.epochId")]
    epoch_id: u64,
}

fn main() {}
