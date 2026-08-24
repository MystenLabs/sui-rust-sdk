//! A type that declares `RESPONSE_ROOT_TYPE` and `extract` itself is a valid flattened
//! field type, at whatever root it names.

use sui_graphql_macros::Response;

struct HandWritten {
    digest: String,
}

impl HandWritten {
    const RESPONSE_ROOT_TYPE: &'static str = "IObject";

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

// type Object implements Node & IAddressable & IObject
#[derive(Response)]
#[response(root_type = "Object")]
struct ObjectResponse {
    #[field(flatten)]
    metadata: HandWritten,
}

fn main() {
    let value = serde_json::json!({ "digest": "abc" });
    let response = ObjectResponse::from_value(value).unwrap();
    assert_eq!(response.metadata.digest, "abc");
}
