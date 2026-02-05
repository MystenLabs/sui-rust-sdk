use sui_graphql_macros::Response;

#[derive(Response)]
#[response(schema = "tests/test_schema.graphql")]
struct ObjectData {
    #[field(path = "object.address")]
    address: String,
    #[field(path = "object.version")]
    version: u64,
}

fn main() {
    let json = serde_json::json!({
        "object": {
            "address": "0x123",
            "version": 42
        }
    });

    let data = ObjectData::from_value(json).unwrap();
    assert_eq!(data.address, "0x123");
    assert_eq!(data.version, 42);
}
