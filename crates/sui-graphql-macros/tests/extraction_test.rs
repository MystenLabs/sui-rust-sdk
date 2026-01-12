use sui_graphql_macros::QueryResponse;

// === Simple Field Extraction ===

#[test]
fn test_simple_path() {
    #[derive(QueryResponse)]
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
    #[derive(QueryResponse)]
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
    #[derive(QueryResponse)]
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

#[test]
fn test_single_array_iteration() {
    #[derive(QueryResponse)]
    struct CheckpointDigests {
        #[field(path = "checkpoints.nodes[].digest")]
        digests: Vec<String>,
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
    assert_eq!(data.digests, vec!["abc123", "def456", "ghi789"]);
}

#[test]
fn test_nested_array_iteration() {
    #[derive(QueryResponse)]
    struct EpochCheckpointDigests {
        #[field(path = "epochs.nodes[].checkpoints.nodes[].digest")]
        checkpoint_digests: Vec<Vec<String>>,
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
        vec![vec!["a1", "a2"], vec!["b1"], vec![]]
    );
}

#[test]
fn test_array_with_prefix_path() {
    #[derive(QueryResponse)]
    struct NestedCheckpoints {
        #[field(path = "epoch.checkpoints.nodes[].sequenceNumber")]
        sequence_numbers: Vec<u64>,
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
    assert_eq!(data.sequence_numbers, vec![100, 200]);
}

#[test]
fn test_empty_array() {
    #[derive(QueryResponse)]
    struct CheckpointDigests {
        #[field(path = "checkpoints.nodes[].digest")]
        digests: Vec<String>,
    }

    let json = serde_json::json!({
        "checkpoints": {
            "nodes": []
        }
    });
    let data = CheckpointDigests::from_value(json).unwrap();
    assert_eq!(data.digests, Vec::<String>::new());
}

#[test]
fn test_null_intermediate_with_option() {
    #[derive(QueryResponse)]
    struct Response {
        #[field(path = "object.address")]
        address: Option<String>,
    }

    // When intermediate "object" is null, Option<T> should get None
    let json = serde_json::json!({
        "object": null
    });
    let data = Response::from_value(json).unwrap();
    assert_eq!(data.address, None);
}

#[test]
fn test_null_intermediate_with_required_field() {
    #[derive(QueryResponse)]
    struct Response {
        #[field(path = "object.address")]
        address: String,
    }

    // When intermediate "object" is null and field is required, should error
    let json = serde_json::json!({
        "object": null
    });
    let result = Response::from_value(json);
    assert!(result.is_err());
}

#[test]
fn test_null_final_value_with_option() {
    #[derive(QueryResponse)]
    struct Response {
        #[field(path = "object.address")]
        address: Option<String>,
    }

    // When final value is null, Option<T> should get None
    let json = serde_json::json!({
        "object": {
            "address": null
        }
    });
    let data = Response::from_value(json).unwrap();
    assert_eq!(data.address, None);
}
