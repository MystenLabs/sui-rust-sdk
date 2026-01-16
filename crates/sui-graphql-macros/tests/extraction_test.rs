use sui_graphql_macros::QueryResponse;

#[test]
fn test_simple_path() {
    #[derive(QueryResponse)]
    struct ObjectData {
        #[field(path = "object.address")]
        address: String,
        #[field(path = "object.digest")]
        digest: String,
    }

    let json = serde_json::json!({
        "object": {
            "address": "0x123",
            "digest": "abc123"
        }
    });
    let data = ObjectData::from_value(json).unwrap();
    assert_eq!(data.address, "0x123");
    assert_eq!(data.digest, "abc123");
}

#[test]
fn test_nested_path() {
    #[derive(QueryResponse)]
    struct NestedData {
        #[field(path = "data.response.value")]
        value: u64,
    }

    let json = serde_json::json!({
        "data": {
            "response": {
                "value": 42
            }
        }
    });
    let data = NestedData::from_value(json).unwrap();
    assert_eq!(data.value, 42);
}

#[test]
fn test_multiple_fields() {
    #[derive(QueryResponse)]
    struct MultiField {
        #[field(path = "user.name")]
        name: String,
        #[field(path = "user.age")]
        age: u64,
        #[field(path = "user.active")]
        active: bool,
    }

    let json = serde_json::json!({
        "user": {
            "name": "alice",
            "age": 30,
            "active": true
        }
    });
    let data = MultiField::from_value(json).unwrap();
    assert_eq!(data.name, "alice");
    assert_eq!(data.age, 30);
    assert_eq!(data.active, true);
}
