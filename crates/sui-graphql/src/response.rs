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
    /// The deserialized data from the response.
    pub data: Option<T>,

    /// Any errors returned by the server.
    pub errors: Vec<GraphQLError>,
}

impl<T> Response<T> {
    /// Create a new response with data and errors.
    pub(crate) fn new(data: Option<T>, errors: Vec<GraphQLError>) -> Self {
        Self { data, errors }
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
    use crate::error::PathFragment;

    fn make_error(path: Vec<PathFragment>) -> GraphQLError {
        GraphQLError {
            message: "test error".to_string(),
            locations: None,
            path: Some(path),
            extensions: None,
        }
    }

    #[test]
    fn test_response_no_errors() {
        let response: Response<String> = Response::new(Some("data".to_string()), vec![]);
        assert!(!response.has_errors());
        assert!(response.errors().is_empty());
    }

    #[test]
    fn test_response_with_errors() {
        let response: Response<String> = Response::new(
            Some("data".to_string()),
            vec![make_error(vec![PathFragment::Key("field".to_string())])],
        );
        assert!(response.has_errors());
        assert_eq!(response.errors().len(), 1);
    }
}
