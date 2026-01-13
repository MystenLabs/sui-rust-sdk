//! Error types for the GraphQL client.

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Error type for GraphQL client operations.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP or network error.
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    /// Base64 decoding error.
    #[error("Base64 decode error: {0}")]
    Base64(#[from] base64ct::Error),

    /// BCS deserialization error.
    #[error("BCS decode error: {0}")]
    Bcs(#[from] bcs::Error),
}

// =============================================================================
// GraphQL Response Types (per GraphQL spec)
// https://spec.graphql.org/October2021/#sec-Errors
// =============================================================================

/// A single GraphQL error.
#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    /// A description of the error.
    pub message: String,

    /// Locations in the query where the error occurred.
    pub locations: Option<Vec<Location>>,

    /// Path to the field that caused the error (e.g., `["user", "name"]`).
    pub path: Option<Vec<PathFragment>>,

    /// Additional error metadata (e.g., error code).
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

impl fmt::Display for GraphQLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl GraphQLError {
    /// Attempt to extract an error code from the extensions, if present.
    pub fn code(&self) -> Option<&str> {
        self.extensions.as_ref()?.get("code")?.as_str()
    }
}

/// A segment in an error path - either a field name or array index.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PathFragment {
    /// Field name in the response.
    Key(String),
    /// Array index in the response.
    Index(i32),
}

/// Location in the GraphQL query where an error occurred.
#[derive(Debug, Deserialize)]
pub struct Location {
    /// Line number (1-indexed).
    pub line: i32,
    /// Column number (1-indexed).
    pub column: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        // Use a relative URL (no scheme) to trigger a builder error
        let err = Error::Request(
            reqwest::Client::new()
                .get("not a valid url")
                .build()
                .unwrap_err(),
        );
        assert!(err.to_string().contains("Request error"));
    }

    #[test]
    fn test_graphql_error_display() {
        let err = GraphQLError {
            message: "Field not found".to_string(),
            locations: None,
            path: None,
            extensions: None,
        };
        assert_eq!(err.to_string(), "Field not found");
    }

    #[test]
    fn test_graphql_error_code() {
        let mut extensions = HashMap::new();
        extensions.insert(
            "code".to_string(),
            serde_json::json!("GRAPHQL_VALIDATION_FAILED"),
        );

        let err = GraphQLError {
            message: "Unknown field \"foo\" on type \"Query\".".to_string(),
            locations: None,
            path: None,
            extensions: Some(extensions),
        };
        assert_eq!(err.code(), Some("GRAPHQL_VALIDATION_FAILED"));
    }

    #[test]
    fn test_graphql_error_no_code() {
        let err = GraphQLError {
            message: "Error".to_string(),
            locations: None,
            path: None,
            extensions: None,
        };
        assert_eq!(err.code(), None);
    }

    #[test]
    fn test_path_fragment_deserialization() {
        let json = r#"["object", 0, "field"]"#;
        let path: Vec<PathFragment> = serde_json::from_str(json).unwrap();

        assert!(matches!(&path[0], PathFragment::Key(k) if k == "object"));
        assert!(matches!(&path[1], PathFragment::Index(0)));
        assert!(matches!(&path[2], PathFragment::Key(k) if k == "field"));
    }
}
