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
///         PathSegment { field: "data", alias: None, is_array: false },
///         PathSegment { field: "nodes", alias: None, is_array: true },
///         PathSegment { field: "name", alias: None, is_array: false },
///     ]
/// }
/// ```
///
/// Paths with aliases like `"epoch.firstCheckpoint:checkpoints.nodes[]"`:
/// ```text
/// ParsedPath {
///     segments: [
///         PathSegment { field: "epoch", alias: None, is_array: false },
///         PathSegment { field: "checkpoints", alias: Some("firstCheckpoint"), is_array: false },
///         PathSegment { field: "nodes", alias: None, is_array: true },
///     ]
/// }
/// ```
///
/// The alias syntax `alias:field` matches GraphQL alias responses where:
/// - `alias` (before `:`) is the JSON key in the response
/// - `field` (after `:`) is the real field name for schema validation
#[derive(Debug, Clone)]
pub struct ParsedPath {
    /// The original path string (for error messages)
    pub raw: String,
    /// Parsed segments of the path
    pub segments: Vec<PathSegment>,
}

/// A single segment in a field path.
///
/// Segments can include an alias using `:` syntax for GraphQL aliases:
/// - The alias (before `:`) is used for JSON extraction
/// - The field name (after `:`) is used for schema validation
#[derive(Debug, Clone)]
pub struct PathSegment {
    /// The field name (used for schema validation)
    pub field: String,
    /// Optional alias (used for JSON extraction instead of field name)
    pub alias: Option<String>,
    /// Whether this is an array field (ends with `[]` in the path)
    pub is_array: bool,
}

impl PathSegment {
    /// Get the key to use for JSON extraction (alias if present, otherwise field name)
    pub fn json_key(&self) -> &str {
        self.alias.as_deref().unwrap_or(&self.field)
    }
}

impl ParsedPath {
    /// Parse a path string into a structured representation.
    ///
    /// Returns `Err` if the path is empty or has invalid syntax.
    ///
    /// # Alias Syntax
    ///
    /// Use `alias:field` to handle GraphQL aliases where the JSON response
    /// uses a different key than the schema field name:
    ///
    /// ```ignore
    /// // GraphQL query with alias:
    /// // epoch { firstCheckpoint: checkpoints(first: 1) { nodes { sequenceNumber } } }
    /// //
    /// // Path uses alias:field syntax:
    /// let path = ParsedPath::parse("epoch.firstCheckpoint:checkpoints.nodes[]")?;
    /// assert_eq!(path.segments[1].field, "checkpoints");  // for schema validation
    /// assert_eq!(path.segments[1].alias, Some("firstCheckpoint".to_string()));  // for JSON
    /// ```
    pub fn parse(path: &str) -> Result<Self, PathParseError> {
        if path.is_empty() {
            return Err(PathParseError::Empty);
        }

        let segments: Vec<PathSegment> = path
            .split('.')
            .map(|segment| {
                // Check for array suffix first
                let (segment, is_array) = if let Some(stripped) = segment.strip_suffix("[]") {
                    (stripped, true)
                } else {
                    (segment, false)
                };

                // Check for alias syntax: alias:field
                let (field, alias) = if let Some(colon_pos) = segment.find(':') {
                    let alias = &segment[..colon_pos];
                    let field = &segment[colon_pos + 1..];
                    (field, Some(alias))
                } else {
                    (segment, None)
                };

                if field.is_empty() {
                    return Err(PathParseError::EmptySegment {
                        path: path.to_string(),
                    });
                }

                Ok(PathSegment {
                    field: field.to_string(),
                    alias: alias.map(|s| s.to_string()),
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
        assert!(path.segments[0].alias.is_none());
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
    fn test_parse_with_alias() {
        let path = ParsedPath::parse("epoch.firstCheckpoint:checkpoints.nodes[]").unwrap();
        assert_eq!(path.segments.len(), 3);
        assert_eq!(path.segments[0].field, "epoch");
        assert!(path.segments[0].alias.is_none());
        assert_eq!(path.segments[1].field, "checkpoints");
        assert_eq!(
            path.segments[1].alias,
            Some("firstCheckpoint".to_string())
        );
        assert!(!path.segments[1].is_array);
        assert_eq!(path.segments[2].field, "nodes");
        assert!(path.segments[2].is_array);
    }

    #[test]
    fn test_parse_array_with_alias() {
        let path = ParsedPath::parse("myObjects:objects[]").unwrap();
        assert_eq!(path.segments.len(), 1);
        assert_eq!(path.segments[0].field, "objects");
        assert_eq!(path.segments[0].alias, Some("myObjects".to_string()));
        assert!(path.segments[0].is_array);
    }

    #[test]
    fn test_json_key() {
        let path = ParsedPath::parse("alias:field.normal").unwrap();
        assert_eq!(path.segments[0].json_key(), "alias");
        assert_eq!(path.segments[1].json_key(), "normal");
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
