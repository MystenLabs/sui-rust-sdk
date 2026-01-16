use sui_graphql_macros::Response;

/// Test that alias syntax `field@alias` works correctly.
/// The field name (before @) is validated against the schema,
/// while the alias (after @) is used for JSON extraction.
#[derive(Response)]
struct EpochCheckpoints {
    // "checkpoints" is the real field name validated against schema
    // "firstCheckpoint" is the alias used for JSON extraction
    #[field(path = "epoch.checkpoints@firstCheckpoint.nodes[].sequenceNumber")]
    first_checkpoint_seq: Option<Vec<u64>>,
}

fn main() {
    let json = serde_json::json!({
        "epoch": {
            "firstCheckpoint": {
                "nodes": [
                    { "sequenceNumber": 1000 },
                    { "sequenceNumber": 1001 }
                ]
            }
        }
    });

    let data = EpochCheckpoints::from_value(json).unwrap();
    assert_eq!(data.first_checkpoint_seq, Some(vec![1000, 1001]));
}
