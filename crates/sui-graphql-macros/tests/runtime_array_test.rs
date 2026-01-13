use sui_graphql_macros::QueryResponse;

#[test]
fn test_single_array_iteration() {
    #[derive(QueryResponse)]
    struct NodeNames {
        #[field(path = "nodes[].name")]
        names: Vec<String>,
    }

    let json = serde_json::json!({
        "nodes": [
            { "name": "alice" },
            { "name": "bob" },
            { "name": "charlie" }
        ]
    });
    let data = NodeNames::from_value(json).unwrap();
    assert_eq!(data.names, vec!["alice", "bob", "charlie"]);
}

#[test]
fn test_nested_array_iteration() {
    #[derive(QueryResponse)]
    struct NestedData {
        #[field(path = "nodes[].edges[].id")]
        edge_ids: Vec<Vec<u64>>,
    }

    let json = serde_json::json!({
        "nodes": [
            { "edges": [{ "id": 1 }, { "id": 2 }] },
            { "edges": [{ "id": 3 }] },
            { "edges": [] }
        ]
    });
    let data = NestedData::from_value(json).unwrap();
    assert_eq!(data.edge_ids, vec![vec![1, 2], vec![3], vec![]]);
}

#[test]
fn test_array_at_end_of_path() {
    #[derive(QueryResponse)]
    struct ItemList {
        #[field(path = "items[]")]
        items: Vec<u64>,
    }

    let json = serde_json::json!({
        "items": [10, 20, 30]
    });
    let data = ItemList::from_value(json).unwrap();
    assert_eq!(data.items, vec![10, 20, 30]);
}

#[test]
fn test_array_with_prefix_path() {
    #[derive(QueryResponse)]
    struct DeepArray {
        #[field(path = "data.response.nodes[].value")]
        values: Vec<i32>,
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
    assert_eq!(data.values, vec![100, 200]);
}

#[test]
fn test_empty_array() {
    #[derive(QueryResponse)]
    struct NodeNames {
        #[field(path = "nodes[].name")]
        names: Vec<String>,
    }

    let json = serde_json::json!({
        "nodes": []
    });
    let data = NodeNames::from_value(json).unwrap();
    assert_eq!(data.names, Vec::<String>::new());
}
