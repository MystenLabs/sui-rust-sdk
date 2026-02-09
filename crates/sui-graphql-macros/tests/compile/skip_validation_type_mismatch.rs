use sui_graphql_macros::Response;

// Even with skip_schema_validation, type validation still catches mismatches.
// Path has 1 explicit [], but type has 3 Vec wrappers - too many!

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct TooManyVecs {
    #[field(path = "items[].name", skip_schema_validation = true)]
    names: Vec<Vec<Vec<String>>>, // Path can have at most 2 list fields (items[] + trailing)
}

fn main() {}
