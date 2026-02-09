use sui_graphql_macros::Response;

// When skip_schema_validation is true, trailing array is inferred from type.
// If Vec count = list count + 1, the last field is inferred as a list.

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct TrailingArrayInferred {
    // Path has 1 explicit [], type has 2 Vec wrappers
    // So the last field (tags) is inferred as a list
    #[field(path = "items[].tags", skip_schema_validation = true)]
    all_tags: Vec<Vec<String>>,
}

fn main() {
    // Just verify it compiles
    let _ = TrailingArrayInferred::from_value;
}
