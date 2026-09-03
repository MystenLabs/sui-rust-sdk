//! Providing `extract` is not on its own enough to be a flattened field's type. A type
//! naming no root could be reading anything at all, fields absent from the schema
//! included, so it is rejected rather than assumed to share the outer response's root.

use sui_graphql_macros::Response;

struct NotInTheSchema {
    baz: String,
}

impl NotInTheSchema {
    fn extract(value: &serde_json::Value) -> Result<Self, String> {
        let baz = value
            .get("baz")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "missing baz".to_string())?;
        Ok(Self {
            baz: baz.to_string(),
        })
    }
}

// `root_type` defaults to `Query`, the root a type declaring none would most plausibly
// be assumed to share.
#[derive(Response)]
struct QueryResponse {
    #[field(flatten)]
    metadata: NotInTheSchema,
}

fn main() {}
