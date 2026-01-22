//! Response type for GraphQL queries.

use crate::error::GraphQLError;

/// A GraphQL response containing data and/or errors.
///
/// GraphQL responses can have three states:
/// - Success: `data` is present, `errors` is empty
/// - Partial success: `data` is present AND `errors` is non-empty
/// - Failure: `data` is None, `errors` is non-empty
#[derive(Debug)]
pub struct Response<T> {
    data: Option<T>,
    errors: Vec<GraphQLError>,
}

impl<T> Response<T> {
    /// Create a new response with data and errors.
    pub(crate) fn new(data: Option<T>, errors: Vec<GraphQLError>) -> Self {
        Self { data, errors }
    }

    /// The deserialized data from the response, if present.
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// Consumes the response and returns the data, if present.
    pub fn into_data(self) -> Option<T> {
        self.data
    }

    /// Returns true if the response has any errors.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns all errors from the response.
    pub fn errors(&self) -> &[GraphQLError] {
        &self.errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_error() -> GraphQLError {
        serde_json::from_value(serde_json::json!({
            "message": "test error",
            "path": ["field"]
        }))
        .unwrap()
    }

    #[test]
    fn test_response_no_errors() {
        let response: Response<String> = Response::new(Some("data".to_string()), vec![]);
        assert!(!response.has_errors());
        assert!(response.errors().is_empty());
    }

    #[test]
    fn test_response_with_errors() {
        let response: Response<String> =
            Response::new(Some("data".to_string()), vec![make_error()]);
        assert!(response.has_errors());
        assert_eq!(response.errors().len(), 1);
    }
}
