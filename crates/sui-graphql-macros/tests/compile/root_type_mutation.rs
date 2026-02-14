use sui_graphql_macros::Response;

/// Test that root_type = "Mutation" works correctly.
#[derive(Response)]
#[response(root_type = "Mutation")]
struct ExecuteResult {
    #[field(path = "executeTransaction.effects.effectsBcs")]
    effects_bcs: Option<String>,
}

fn main() {
    let json = serde_json::json!({
        "executeTransaction": {
            "effects": {
                "effectsBcs": "base64encodedeffects"
            }
        }
    });

    let result = ExecuteResult::from_value(json).unwrap();
    assert_eq!(result.effects_bcs, Some("base64encodedeffects".to_string()));
}
