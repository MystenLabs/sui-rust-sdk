use sui_graphql_macros::Response;

#[derive(Response)]
struct ArrayTypo {
    // "checkpointss" is a typo of "checkpoints"
    #[field(path = "checkpointss[].digest")]
    digests: Vec<String>,
}

fn main() {}
