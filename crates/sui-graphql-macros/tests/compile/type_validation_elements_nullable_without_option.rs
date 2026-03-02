use sui_graphql_macros::Response;

#[derive(Response)]
struct R {
    #[field(path = "checkpoints.nodes[]?.digest")]
    digests: Vec<String>,
}

fn main() {}
