use sui_graphql_macros::Response;

/// Test that alias syntax `alias:field` works correctly (matching GraphQL's syntax).
/// The alias (before :) is used for JSON extraction,
/// while the field name (after :) is validated against the schema.
#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct EpochCheckpoints {
    // "firstCheckpoint" is the alias used for JSON extraction
    // "checkpoints" is the real field name validated against schema
    #[field(path = "epoch.firstCheckpoint:checkpoints.nodes[].sequenceNumber")]
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
