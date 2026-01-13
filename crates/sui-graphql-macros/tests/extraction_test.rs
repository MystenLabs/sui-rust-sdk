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
