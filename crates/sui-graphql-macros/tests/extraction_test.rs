use sui_graphql_macros::Response;

#[test]
fn test_simple_path() {
    #[derive(Response)]
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
    #[derive(Response)]
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
    #[derive(Response)]
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
    assert!(data.active);
}

#[test]
fn test_single_array_iteration() {
    #[derive(Response)]
    struct NodeNames {
        #[field(path = "nodes[].name")]
        names: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "nodes": [
            { "name": "alice" },
            { "name": "bob" },
            { "name": "charlie" }
        ]
    });
    let data = NodeNames::from_value(json).unwrap();
    assert_eq!(
        data.names,
        Some(vec![
            "alice".to_string(),
            "bob".to_string(),
            "charlie".to_string()
        ])
    );
}

#[test]
fn test_nested_array_iteration() {
    #[derive(Response)]
    struct NestedData {
        #[field(path = "nodes[].edges[].id")]
        edge_ids: Option<Vec<Option<Vec<u64>>>>,
    }

    let json = serde_json::json!({
        "nodes": [
            { "edges": [{ "id": 1 }, { "id": 2 }] },
            { "edges": [{ "id": 3 }] },
            { "edges": [] }
        ]
    });
    let data = NestedData::from_value(json).unwrap();
    assert_eq!(
        data.edge_ids,
        Some(vec![Some(vec![1, 2]), Some(vec![3]), Some(vec![])])
    );
}

#[test]
fn test_array_at_end_of_path() {
    #[derive(Response)]
    struct ItemList {
        #[field(path = "items[]")]
        items: Option<Vec<u64>>,
    }

    let json = serde_json::json!({
        "items": [10, 20, 30]
    });
    let data = ItemList::from_value(json).unwrap();
    assert_eq!(data.items, Some(vec![10, 20, 30]));
}

#[test]
fn test_array_with_prefix_path() {
    #[derive(Response)]
    struct DeepArray {
        #[field(path = "data.response.nodes[].value")]
        values: Option<Vec<i32>>,
    }

    let json = serde_json::json!({
        "data": {
            "response": {
                "nodes": [
                    { "value": 100 },
                    { "value": 200 }
                ]
            }
        }
    });
    let data = DeepArray::from_value(json).unwrap();
    assert_eq!(data.values, Some(vec![100, 200]));
}

#[test]
fn test_empty_array() {
    #[derive(Response)]
    struct NodeNames {
        #[field(path = "nodes[].name")]
        names: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "nodes": []
    });
    let data = NodeNames::from_value(json).unwrap();
    assert_eq!(data.names, Some(vec![]));
}

#[test]
fn test_null_array_returns_none() {
    #[derive(Response)]
    struct NodeNames {
        #[field(path = "nodes[].name")]
        names: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "nodes": null
    });
    let data = NodeNames::from_value(json).unwrap();
    assert_eq!(data.names, None);
}

#[test]
fn test_null_nested_array() {
    #[derive(Response)]
    struct NestedData {
        #[field(path = "data.nodes[].name")]
        names: Option<Vec<String>>,
    }

    // When parent field (data) is null, the array should be None
    let json = serde_json::json!({
        "data": null
    });
    let data = NestedData::from_value(json).unwrap();
    assert_eq!(data.names, None);
}
