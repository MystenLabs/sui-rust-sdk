use sui_graphql_macros::Response;

/// Test that root_type = "DynamicField" works correctly.
/// This demonstrates using a non-root GraphQL type as the starting point.
#[derive(Response)]
#[response(root_type = "DynamicField")]
struct DynamicFieldData {
    #[field(path = "address")]
    address: String,
}

fn main() {
    let json = serde_json::json!({
        "address": "0x123abc"
    });

    let data = DynamicFieldData::from_value(json).unwrap();
    assert_eq!(data.address, "0x123abc");
}
