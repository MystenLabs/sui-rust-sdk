//! Field path parsing for GraphQL response extraction.
//!
//! This module provides a single parser for field paths used in the Response macro.
//! The parsed representation is used for both schema validation and code generation,
//! ensuring consistency and avoiding duplicate parsing logic.

/// A parsed field path.
///
/// Paths like `"data.nodes[].name"` are parsed into segments:
/// ```text
/// ParsedPath {
///     segments: [
///         PathSegment { field: "data", is_array: false },
///         PathSegment { field: "nodes", is_array: true },
///         PathSegment { field: "name", is_array: false },
///     ]
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ParsedPath {
    /// The original path string (for error messages)
    pub raw: String,
    /// Parsed segments of the path
    pub segments: Vec<PathSegment>,
}

/// A single segment in a field path.
#[derive(Debug, Clone)]
pub struct PathSegment {
    /// The field name (used for both schema validation and JSON extraction)
    pub field: String,
    /// Whether this is an array field (ends with `[]` in the path)
    pub is_array: bool,
}

impl ParsedPath {
    /// Parse a path string into a structured representation.
    ///
    /// Returns `Err` if the path is empty or has invalid syntax.
    pub fn parse(path: &str) -> Result<Self, PathParseError> {
        if path.is_empty() {
            return Err(PathParseError::Empty);
        }

        let segments: Vec<PathSegment> = path
            .split('.')
            .map(|segment| {
                let (field, is_array) = if let Some(stripped) = segment.strip_suffix("[]") {
                    (stripped, true)
                } else {
                    (segment, false)
                };

                if field.is_empty() {
                    return Err(PathParseError::EmptySegment {
                        path: path.to_string(),
                    });
                }

                Ok(PathSegment {
                    field: field.to_string(),
                    is_array,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ParsedPath {
            raw: path.to_string(),
            segments,
        })
    }
}

/// Errors that can occur when parsing a path.
#[derive(Debug, Clone)]
pub enum PathParseError {
    /// The path string is empty
    Empty,
    /// A segment in the path is empty (e.g., "foo..bar" or ".foo")
    EmptySegment { path: String },
}

impl std::fmt::Display for PathParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathParseError::Empty => write!(f, "Field path cannot be empty"),
            PathParseError::EmptySegment { path } => {
                write!(f, "Empty segment in path '{}'", path)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_path() {
        let path = ParsedPath::parse("object.address").unwrap();
        assert_eq!(path.segments.len(), 2);
        assert_eq!(path.segments[0].field, "object");
        assert!(!path.segments[0].is_array);
        assert_eq!(path.segments[1].field, "address");
        assert!(!path.segments[1].is_array);
    }

    #[test]
    fn test_parse_array_path() {
        let path = ParsedPath::parse("nodes[].name").unwrap();
        assert_eq!(path.segments.len(), 2);
        assert_eq!(path.segments[0].field, "nodes");
        assert!(path.segments[0].is_array);
        assert_eq!(path.segments[1].field, "name");
        assert!(!path.segments[1].is_array);
    }

    #[test]
    fn test_parse_nested_arrays() {
        let path = ParsedPath::parse("data[].items[].id").unwrap();
        assert_eq!(path.segments.len(), 3);
        assert!(path.segments[0].is_array);
        assert!(path.segments[1].is_array);
        assert!(!path.segments[2].is_array);
    }

    #[test]
    fn test_parse_single_field() {
        let path = ParsedPath::parse("chainIdentifier").unwrap();
        assert_eq!(path.segments.len(), 1);
        assert_eq!(path.segments[0].field, "chainIdentifier");
    }

    #[test]
    fn test_parse_empty_error() {
        let err = ParsedPath::parse("").unwrap_err();
        assert!(matches!(err, PathParseError::Empty));
    }

    #[test]
    fn test_parse_empty_segment_error() {
        let err = ParsedPath::parse("foo..bar").unwrap_err();
        assert!(matches!(err, PathParseError::EmptySegment { .. }));

        let err = ParsedPath::parse(".foo").unwrap_err();
        assert!(matches!(err, PathParseError::EmptySegment { .. }));
    }
}
