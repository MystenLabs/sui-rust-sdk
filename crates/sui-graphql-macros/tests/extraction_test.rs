use sui_graphql_macros::Response;

// === Simple Field Extraction ===

#[test]
fn test_simple_path() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct ChainInfo {
        #[field(path = "chainIdentifier")]
        chain_id: String,
    }

    let json = serde_json::json!({
        "chainIdentifier": "4c78adac"
    });
    let data = ChainInfo::from_value(json).unwrap();
    assert_eq!(data.chain_id, "4c78adac");
}

#[test]
fn test_nested_path() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct EpochData {
        #[field(path = "epoch.epochId")]
        epoch_id: u64,
    }

    let json = serde_json::json!({
        "epoch": {
            "epochId": 42
        }
    });
    let data = EpochData::from_value(json).unwrap();
    assert_eq!(data.epoch_id, 42);
}

#[test]
fn test_multiple_fields() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct CheckpointData {
        #[field(path = "checkpoint.sequenceNumber")]
        sequence_number: u64,
        #[field(path = "checkpoint.digest")]
        digest: String,
    }

    let json = serde_json::json!({
        "checkpoint": {
            "sequenceNumber": 100,
            "digest": "abc123"
        }
    });
    let data = CheckpointData::from_value(json).unwrap();
    assert_eq!(data.sequence_number, 100);
    assert_eq!(data.digest, "abc123");
}

// === Array Extraction ===
// Array fields return Option<Vec<T>> - None if null, Some(vec) otherwise

#[test]
fn test_single_array_iteration() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct CheckpointDigests {
        #[field(path = "checkpoints.nodes[].digest")]
        digests: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "checkpoints": {
            "nodes": [
                { "digest": "abc123" },
                { "digest": "def456" },
                { "digest": "ghi789" }
            ]
        }
    });
    let data = CheckpointDigests::from_value(json).unwrap();
    assert_eq!(
        data.digests,
        Some(vec![
            "abc123".to_string(),
            "def456".to_string(),
            "ghi789".to_string()
        ])
    );
}

#[test]
fn test_nested_array_iteration() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct EpochCheckpointDigests {
        #[field(path = "epochs.nodes[].checkpoints.nodes[].digest")]
        checkpoint_digests: Option<Vec<Option<Vec<String>>>>,
    }

    let json = serde_json::json!({
        "epochs": {
            "nodes": [
                {
                    "checkpoints": {
                        "nodes": [
                            { "digest": "a1" },
                            { "digest": "a2" }
                        ]
                    }
                },
                {
                    "checkpoints": {
                        "nodes": [
                            { "digest": "b1" }
                        ]
                    }
                },
                {
                    "checkpoints": {
                        "nodes": []
                    }
                }
            ]
        }
    });
    let data = EpochCheckpointDigests::from_value(json).unwrap();
    assert_eq!(
        data.checkpoint_digests,
        Some(vec![
            Some(vec!["a1".to_string(), "a2".to_string()]),
            Some(vec!["b1".to_string()]),
            Some(vec![])
        ])
    );
}

#[test]
fn test_array_with_prefix_path() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct NestedCheckpoints {
        #[field(path = "epoch.checkpoints.nodes[].sequenceNumber")]
        sequence_numbers: Option<Vec<u64>>,
    }

    let json = serde_json::json!({
        "epoch": {
            "checkpoints": {
                "nodes": [
                    { "sequenceNumber": 100 },
                    { "sequenceNumber": 200 }
                ]
            }
        }
    });
    let data = NestedCheckpoints::from_value(json).unwrap();
    assert_eq!(data.sequence_numbers, Some(vec![100, 200]));
}

#[test]
fn test_empty_array() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct CheckpointDigests {
        #[field(path = "checkpoints.nodes[].digest")]
        digests: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "checkpoints": {
            "nodes": []
        }
    });
    let data = CheckpointDigests::from_value(json).unwrap();
    assert_eq!(data.digests, Some(vec![]));
}

#[test]
fn test_null_array_returns_none() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct CheckpointDigests {
        #[field(path = "checkpoints.nodes[].digest")]
        digests: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "checkpoints": {
            "nodes": null
        }
    });
    let data = CheckpointDigests::from_value(json).unwrap();
    assert_eq!(data.digests, None);
}

#[test]
fn test_null_parent_of_array() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct CheckpointDigests {
        #[field(path = "checkpoints.nodes[].digest")]
        digests: Option<Vec<String>>,
    }

    // When parent field (checkpoints) is null, should get None
    let json = serde_json::json!({
        "checkpoints": null
    });
    let data = CheckpointDigests::from_value(json).unwrap();
    assert_eq!(data.digests, None);
}

// === Type-Driven Array Handling ===
// The Rust type determines how null values are handled:
// - Vec<T>: array is required, null array → error
// - Option<Vec<T>>: array is optional, null array → None
// - Vec<Option<T>>: array required, null elements → None per element
// - Option<Vec<Option<T>>>: array optional, null elements → None per element

#[test]
fn test_required_array_with_valid_data() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "items[].name")]
        names: Vec<String>,
    }

    let json = serde_json::json!({
        "items": [
            { "name": "alice" },
            { "name": "bob" }
        ]
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(data.names, vec!["alice", "bob"]);
}

#[test]
fn test_required_array_with_empty_array() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "items[].name")]
        names: Vec<String>,
    }

    let json = serde_json::json!({
        "items": []
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(data.names, Vec::<String>::new());
}

#[test]
fn test_required_array_with_null_array_returns_error() {
    #[allow(dead_code)]
    #[derive(Debug, Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "items[].name")]
        names: Vec<String>,
    }

    let json = serde_json::json!({
        "items": null
    });
    let result = Data::from_value(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("null value"));
}

#[test]
fn test_optional_array_with_valid_data() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "items[].name")]
        names: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "items": [
            { "name": "alice" },
            { "name": "bob" }
        ]
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(
        data.names,
        Some(vec!["alice".to_string(), "bob".to_string()])
    );
}

#[test]
fn test_optional_array_with_null_array_returns_none() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "items[].name")]
        names: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "items": null
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(data.names, None);
}

#[test]
fn test_optional_array_with_empty_array() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "items[].name")]
        names: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "items": []
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(data.names, Some(vec![]));
}

#[test]
fn test_required_array_with_optional_elements() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "items[].nickname")]
        nicknames: Vec<Option<String>>,
    }

    let json = serde_json::json!({
        "items": [
            { "nickname": "ally" },
            { "nickname": null },
            { "nickname": "bobby" }
        ]
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(
        data.nicknames,
        vec![Some("ally".to_string()), None, Some("bobby".to_string())]
    );
}

#[test]
fn test_required_array_with_optional_elements_null_array_returns_error() {
    #[allow(dead_code)]
    #[derive(Debug, Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "items[].nickname")]
        nicknames: Vec<Option<String>>,
    }

    let json = serde_json::json!({
        "items": null
    });
    let result = Data::from_value(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("null value"));
}

#[test]
fn test_optional_array_with_optional_elements() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "items[].nickname")]
        nicknames: Option<Vec<Option<String>>>,
    }

    let json = serde_json::json!({
        "items": [
            { "nickname": "ally" },
            { "nickname": null },
            { "nickname": "bobby" }
        ]
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(
        data.nicknames,
        Some(vec![
            Some("ally".to_string()),
            None,
            Some("bobby".to_string())
        ])
    );
}

#[test]
fn test_optional_array_with_optional_elements_null_array() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "items[].nickname")]
        nicknames: Option<Vec<Option<String>>>,
    }

    let json = serde_json::json!({
        "items": null
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(data.nicknames, None);
}

// === Nested Array Type-Driven Handling ===

#[test]
fn test_nested_required_arrays() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "groups[].members[].name")]
        names: Vec<Vec<String>>,
    }

    let json = serde_json::json!({
        "groups": [
            { "members": [{ "name": "alice" }, { "name": "bob" }] },
            { "members": [{ "name": "charlie" }] }
        ]
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(
        data.names,
        vec![
            vec!["alice".to_string(), "bob".to_string()],
            vec!["charlie".to_string()]
        ]
    );
}

#[test]
fn test_nested_required_arrays_null_outer_returns_error() {
    #[allow(dead_code)]
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "groups[].members[].name")]
        names: Vec<Vec<String>>,
    }

    let json = serde_json::json!({
        "groups": null
    });
    let result = Data::from_value(json);
    assert!(result.is_err());
}

#[test]
fn test_nested_required_arrays_null_inner_returns_error() {
    #[allow(dead_code)]
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "groups[].members[].name")]
        names: Vec<Vec<String>>,
    }

    let json = serde_json::json!({
        "groups": [
            { "members": [{ "name": "alice" }] },
            { "members": null }
        ]
    });
    let result = Data::from_value(json);
    assert!(result.is_err());
}

#[test]
fn test_nested_optional_outer_required_inner() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "groups[].members[].name")]
        names: Option<Vec<Vec<String>>>,
    }

    let json = serde_json::json!({
        "groups": [
            { "members": [{ "name": "alice" }] },
            { "members": [{ "name": "bob" }, { "name": "charlie" }] }
        ]
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(
        data.names,
        Some(vec![
            vec!["alice".to_string()],
            vec!["bob".to_string(), "charlie".to_string()]
        ])
    );
}

#[test]
fn test_nested_optional_outer_required_inner_null_outer() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "groups[].members[].name")]
        names: Option<Vec<Vec<String>>>,
    }

    let json = serde_json::json!({
        "groups": null
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(data.names, None);
}

#[test]
fn test_nested_optional_outer_required_inner_null_inner_returns_error() {
    #[allow(dead_code)]
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "groups[].members[].name")]
        names: Option<Vec<Vec<String>>>,
    }

    let json = serde_json::json!({
        "groups": [
            { "members": [{ "name": "alice" }] },
            { "members": null }
        ]
    });
    let result = Data::from_value(json);
    assert!(result.is_err());
}

#[test]
fn test_nested_required_outer_optional_inner() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "groups[].members[].name")]
        names: Vec<Option<Vec<String>>>,
    }

    let json = serde_json::json!({
        "groups": [
            { "members": [{ "name": "alice" }] },
            { "members": null },
            { "members": [{ "name": "bob" }] }
        ]
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(
        data.names,
        vec![
            Some(vec!["alice".to_string()]),
            None,
            Some(vec!["bob".to_string()])
        ]
    );
}

#[test]
fn test_nested_required_outer_optional_inner_null_outer_returns_error() {
    #[allow(dead_code)]
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "groups[].members[].name")]
        names: Vec<Option<Vec<String>>>,
    }

    let json = serde_json::json!({
        "groups": null
    });
    let result = Data::from_value(json);
    assert!(result.is_err());
}

#[test]
fn test_nested_both_optional() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "groups[].members[].name")]
        names: Option<Vec<Option<Vec<String>>>>,
    }

    let json = serde_json::json!({
        "groups": [
            { "members": [{ "name": "alice" }] },
            { "members": null },
            { "members": [{ "name": "bob" }, { "name": "charlie" }] }
        ]
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(
        data.names,
        Some(vec![
            Some(vec!["alice".to_string()]),
            None,
            Some(vec!["bob".to_string(), "charlie".to_string()])
        ])
    );
}

#[test]
fn test_nested_both_optional_null_outer() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct Data {
        #[field(path = "groups[].members[].name")]
        names: Option<Vec<Option<Vec<String>>>>,
    }

    let json = serde_json::json!({
        "groups": null
    });
    let data = Data::from_value(json).unwrap();
    assert_eq!(data.names, None);
}

// === Null Handling ===

#[test]
fn test_null_intermediate_with_option() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct ObjectResponse {
        #[field(path = "object.address")]
        address: Option<String>,
    }

    // When intermediate "object" is null, Option<T> should get None
    let json = serde_json::json!({
        "object": null
    });
    let data = ObjectResponse::from_value(json).unwrap();
    assert_eq!(data.address, None);
}

#[test]
fn test_null_intermediate_with_required_field() {
    #[allow(dead_code)] // Field never read because parsing fails
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct ObjectResponse {
        #[field(path = "object.address")]
        address: String,
    }

    // When intermediate "object" is null and field is required, should error
    let json = serde_json::json!({
        "object": null
    });
    let result = ObjectResponse::from_value(json);
    assert!(result.is_err());
}

#[test]
fn test_null_final_value_with_option() {
    #[derive(Response)]
    #[response(schema = "tests/test_schema.graphql")]
    struct ObjectResponse {
        #[field(path = "object.address")]
        address: Option<String>,
    }

    // When final value is null, Option<T> should get None
    let json = serde_json::json!({
        "object": {
            "address": null
        }
    });
    let data = ObjectResponse::from_value(json).unwrap();
    assert_eq!(data.address, None);
}
