//! A type that does not derive `Response` declares no root type, so it is treated as
//! `Query` and cannot be flattened into a response rooted elsewhere.

use sui_graphql_macros::Response;

struct HandWritten {
    digest: String,
}

impl HandWritten {
    fn extract(value: &serde_json::Value) -> Result<Self, String> {
        let digest = value
            .get("digest")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "missing digest".to_string())?;
        Ok(Self {
            digest: digest.to_string(),
        })
    }
}

#[derive(Response)]
#[response(root_type = "Object")]
struct ObjectResponse {
    #[field(flatten)]
    metadata: HandWritten,
}

fn main() {}
