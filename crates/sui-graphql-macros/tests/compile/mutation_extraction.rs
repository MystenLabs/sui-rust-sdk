use sui_graphql_macros::Response;

#[derive(Response)]
#[response(mutation)]
struct ExecuteResponse {
    #[field(path = "executeTransaction.effects.digest")]
    digest: Option<String>,
    #[field(path = "executeTransaction.errors")]
    errors: Option<Vec<String>>,
}

fn main() {
    let json = serde_json::json!({
        "executeTransaction": {
            "effects": {
                "digest": "ABC123"
            },
            "errors": null
        }
    });

    let data = ExecuteResponse::from_value(json).unwrap();
    assert_eq!(data.digest, Some("ABC123".to_string()));
    assert!(data.errors.is_none());
}
