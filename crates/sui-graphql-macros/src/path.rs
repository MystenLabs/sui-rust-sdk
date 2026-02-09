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
///         PathSegment { field: "data", alias: None, is_list: Some(false) },
///         PathSegment { field: "nodes", alias: None, is_list: Some(true) },
///         PathSegment { field: "name", alias: None, is_list: None },
///     ]
/// }
/// ```
///
/// Paths with aliases like `"epoch.firstCheckpoint:checkpoints.nodes[]"`:
/// ```text
/// ParsedPath {
///     segments: [
///         PathSegment { field: "epoch", alias: None, is_list: Some(false) },
///         PathSegment { field: "checkpoints", alias: Some("firstCheckpoint"), is_list: Some(false) },
///         PathSegment { field: "nodes", alias: None, is_list: Some(true) },
///     ]
/// }
/// ```
///
/// The alias syntax `alias:field` matches GraphQL alias responses where:
/// - `alias` (before `:`) is the JSON key in the response
/// - `field` (after `:`) is the real field name for schema validation
///
/// After parsing:
/// - Segments with `[]` suffix: `is_list = Some(true)`
/// - Non-last segments without `[]`: `is_list = Some(false)`
/// - Last segment without `[]`: `is_list = None` (may be inferred as trailing array)
#[derive(Debug, Clone)]
pub struct ParsedPath<'a> {
    /// The original path string (for error messages)
    pub raw: &'a str,
    /// Parsed segments of the path
    pub segments: Vec<PathSegment<'a>>,
}

/// A single segment in a field path.
///
/// Segments can include an alias using `:` syntax for GraphQL aliases:
/// - The alias (before `:`) is used for JSON extraction
/// - The field name (after `:`) is used for schema validation
#[derive(Debug, Clone)]
pub struct PathSegment<'a> {
    /// The field name (used for schema validation)
    pub field: &'a str,
    /// Optional alias (used for JSON extraction instead of field name)
    pub alias: Option<&'a str>,
    /// Whether this field is a list type.
    /// - `Some(true)` = field is a list
    /// - `Some(false)` = field is not a list
    /// - `None` = not yet determined (last segment without [], may be trailing array)
    pub is_list: Option<bool>,
}

impl<'a> PathSegment<'a> {
    /// Get the key to use for JSON extraction (alias if present, otherwise field name)
    pub fn json_key(&self) -> &str {
        self.alias.unwrap_or(self.field)
    }
}

impl<'a> ParsedPath<'a> {
    /// Parse a path string into a structured representation.
    ///
    /// Returns `Err` if the path is empty or has invalid syntax.
    ///
    /// # Alias Syntax
    ///
    /// Use `alias:field` to handle GraphQL aliases where the JSON response
    /// uses a different key than the schema field name.
    ///
    /// After parsing:
    /// - Segments with `[]` suffix: `is_list = Some(true)`
    /// - Non-last segments without `[]`: `is_list = Some(false)`
    /// - Last segment without `[]`: `is_list = None` (may be inferred as trailing array)
    pub fn parse(path: &'a str) -> Result<Self, PathParseError<'a>> {
        if path.is_empty() {
            return Err(PathParseError::Empty);
        }

        let raw_segments: Vec<&str> = path.split('.').collect();
        let last_idx = raw_segments.len().saturating_sub(1);

        let mut segments = Vec::with_capacity(raw_segments.len());
        for (idx, segment) in raw_segments.into_iter().enumerate() {
            if segment.is_empty() {
                return Err(PathParseError::EmptySegment { path });
            }

            // Check for array suffix first
            let (segment, has_array_suffix) = if let Some(stripped) = segment.strip_suffix("[]") {
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
                return Err(PathParseError::EmptySegment { path });
            }

            // Determine is_list based on [] suffix and position
            let is_list = if has_array_suffix {
                Some(true)
            } else if idx == last_idx {
                // Last segment without []: could be a trailing array (e.g., `items[].tags` where
                // `tags: [String]`). Unlike middle segments, the last segment doesn't need []
                // to access array elements - the whole array becomes the final value.
                //
                // How is_list gets determined later:
                // - With schema validation (default): set from schema's field type
                // - Without schema validation (skip_schema_validation = true): inferred as list
                //   if Rust type has one more Vec than the number of [] in path
                None
            } else {
                // Non-last segment without []: must be a scalar. If it were a list, user would
                // need [] to iterate and access fields from each element.
                Some(false)
            };

            segments.push(PathSegment {
                field,
                alias,
                is_list,
            });
        }

        Ok(ParsedPath {
            raw: path,
            segments,
        })
    }

    /// Returns the field names that are lists in the schema.
    ///
    /// Only valid after `validation::validate_path_against_schema` has been called.
    pub fn list_fields(&self) -> Vec<&'a str> {
        self.segments
            .iter()
            .filter(|s| s.is_list == Some(true))
            .map(|s| s.field)
            .collect()
    }
}

/// Errors that can occur when parsing a path.
#[derive(Debug, Clone)]
pub enum PathParseError<'a> {
    /// The path string is empty
    Empty,
    /// A segment in the path is empty (e.g., "foo..bar" or ".foo")
    EmptySegment { path: &'a str },
}

impl<'a> std::fmt::Display for PathParseError<'a> {
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
        assert_eq!(path.segments[0].is_list, Some(false)); // non-last, no []
        assert_eq!(path.segments[1].field, "address");
        assert_eq!(path.segments[1].is_list, None); // last, no []
    }

    #[test]
    fn test_parse_nested_path() {
        let path = ParsedPath::parse("data.nodes.name").unwrap();
        assert_eq!(path.segments.len(), 3);
        assert_eq!(path.segments[0].field, "data");
        assert_eq!(path.segments[0].is_list, Some(false)); // non-last, no []
        assert_eq!(path.segments[1].field, "nodes");
        assert_eq!(path.segments[1].is_list, Some(false)); // non-last, no []
        assert_eq!(path.segments[2].field, "name");
        assert_eq!(path.segments[2].is_list, None); // last, no []
    }

    #[test]
    fn test_parse_single_field() {
        let path = ParsedPath::parse("chainIdentifier").unwrap();
        assert_eq!(path.segments.len(), 1);
        assert_eq!(path.segments[0].field, "chainIdentifier");
        assert_eq!(path.segments[0].is_list, None); // single field is last, no []
    }

    #[test]
    fn test_parse_with_alias() {
        let path = ParsedPath::parse("epoch.firstCheckpoint:checkpoints.nodes[]").unwrap();
        assert_eq!(path.segments.len(), 3);
        assert_eq!(path.segments[0].field, "epoch");
        assert!(path.segments[0].alias.is_none());
        assert_eq!(path.segments[0].is_list, Some(false));
        assert_eq!(path.segments[1].field, "checkpoints");
        assert_eq!(path.segments[1].alias, Some("firstCheckpoint"));
        assert_eq!(path.segments[1].is_list, Some(false));
        assert_eq!(path.segments[2].field, "nodes");
        assert_eq!(path.segments[2].is_list, Some(true));
    }

    #[test]
    fn test_parse_array_with_alias() {
        let path = ParsedPath::parse("myObjects:objects[]").unwrap();
        assert_eq!(path.segments.len(), 1);
        assert_eq!(path.segments[0].field, "objects");
        assert_eq!(path.segments[0].alias, Some("myObjects"));
        assert_eq!(path.segments[0].is_list, Some(true));
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

    #[test]
    fn test_parse_array_syntax() {
        let path = ParsedPath::parse("items[].name").unwrap();
        assert_eq!(path.segments.len(), 2);
        assert_eq!(path.segments[0].field, "items");
        assert_eq!(path.segments[0].is_list, Some(true)); // has []
        assert_eq!(path.segments[1].field, "name");
        assert_eq!(path.segments[1].is_list, None); // last, no []
    }

    #[test]
    fn test_parse_nested_arrays() {
        let path = ParsedPath::parse("groups[].members[].name").unwrap();
        assert_eq!(path.segments.len(), 3);
        assert_eq!(path.segments[0].field, "groups");
        assert_eq!(path.segments[0].is_list, Some(true)); // has []
        assert_eq!(path.segments[1].field, "members");
        assert_eq!(path.segments[1].is_list, Some(true)); // has []
        assert_eq!(path.segments[2].field, "name");
        assert_eq!(path.segments[2].is_list, None); // last, no []
    }

    #[test]
    fn test_parse_trailing_array() {
        let path = ParsedPath::parse("items[].tags[]").unwrap();
        assert_eq!(path.segments.len(), 2);
        assert_eq!(path.segments[0].field, "items");
        assert_eq!(path.segments[0].is_list, Some(true));
        assert_eq!(path.segments[1].field, "tags");
        assert_eq!(path.segments[1].is_list, Some(true));
    }

    #[test]
    fn test_parse_empty_array_field_error() {
        let err = ParsedPath::parse("[].name").unwrap_err();
        assert!(matches!(err, PathParseError::EmptySegment { .. }));
    }
}
