use sui_graphql_macros::Response;

// With skip_schema_validation, the Vec excess check is skipped — the user takes
// full responsibility for the type matching the actual JSON structure.

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct ExtraVecs {
    #[field(path = "items[].name", skip_schema_validation = true)]
    names: Vec<Vec<Vec<String>>>,
}

fn main() {
    // Just verify it compiles
    let _ = ExtraVecs::from_value;
}
