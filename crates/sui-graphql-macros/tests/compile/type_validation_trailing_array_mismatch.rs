use sui_graphql_macros::Response;

// Path items[].tags[] where tags: [String] requires Vec<Vec<String>>
// but type only has one Vec wrapper
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct MissingInnerVec {
    #[field(path = "items[].tags[]")]
    all_tags: Vec<String>, // Should be Vec<Vec<String>>
}

fn main() {}
