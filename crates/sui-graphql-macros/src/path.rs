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
///         PathSegment { field: "data",  is_nullable: false, list: None },
///         PathSegment { field: "nodes", is_nullable: false, list: Some(List { elements_nullable: false }) },
///         PathSegment { field: "name",  is_nullable: false, list: None },
///     ]
/// }
/// ```
///
/// The alias syntax `alias:field` matches GraphQL alias responses where:
/// - `alias` (before `:`) is the JSON key in the response
/// - `field` (after `:`) is the real field name for schema validation
///
/// List fields must always use `[]` suffix explicitly (e.g., `nodes[]`, `tags[]`).
///
/// Null markers use `?` suffix:
/// - `field?` — field value is nullable (null → `Ok(None)`)
/// - `field?[]` — array itself is nullable
/// - `field[]?` — array elements are nullable
/// - `field?[]?` — both array and elements are nullable
#[derive(Debug, Clone)]
pub struct ParsedPath<'a> {
    /// The original path string (for error messages)
    pub raw: &'a str,
    /// Parsed segments of the path
    pub segments: Vec<PathSegment<'a>>,
}

/// List properties for a path segment that has `[]`.
///
/// The presence of this struct indicates the segment is a list field.
#[derive(Debug, Clone, PartialEq)]
pub struct List {
    /// Whether elements of the list are nullable (`[]?` suffix).
    /// When true, null elements within the array return `Ok(None)` per element.
    pub elements_nullable: bool,
}

/// A single segment in a field path.
///
/// Segments can include an alias using `:` syntax for GraphQL aliases:
/// - The alias (before `:`) is used for JSON extraction
/// - The field name (after `:`) is used for schema validation
///
/// Null markers (`?`) control per-segment null tolerance:
/// - `is_nullable`: the field value itself is nullable
/// - `list.elements_nullable`: array elements are nullable
#[derive(Debug, Clone)]
pub struct PathSegment<'a> {
    /// The field name (used for schema validation)
    pub field: &'a str,
    /// Optional alias (used for JSON extraction instead of field name)
    pub alias: Option<&'a str>,
    /// Whether this field value is nullable (`?` before `[]` or on non-list field).
    /// When true, null at this field returns `Ok(None)` instead of an error.
    pub is_nullable: bool,
    /// List properties if this field has `[]` suffix. `None` means not a list.
    pub list: Option<List>,
}

impl<'a> PathSegment<'a> {
    /// Get the key to use for JSON extraction (alias if present, otherwise field name)
    pub fn json_key(&self) -> &str {
        self.alias.unwrap_or(self.field)
    }

    /// Whether this segment is a list field (has `[]` suffix).
    pub fn is_list(&self) -> bool {
        self.list.is_some()
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
    /// List fields must use `[]` suffix explicitly (e.g., `nodes[]`, `tags[]`).
    ///
    /// # Null Marker Syntax
    ///
    /// Use `?` to mark nullable segments:
    /// - `field?` — field value is nullable
    /// - `field?[]` — array is nullable
    /// - `field[]?` — elements are nullable
    /// - `field?[]?` — both array and elements are nullable
    pub fn parse(path: &'a str) -> Result<Self, PathParseError<'a>> {
        if path.is_empty() {
            return Err(PathParseError::Empty);
        }

        let raw_segments: Vec<&str> = path.split('.').collect();

        let mut segments = Vec::with_capacity(raw_segments.len());
        for segment in raw_segments {
            if segment.is_empty() {
                return Err(PathParseError::EmptySegment { path });
            }

            // Split into field part and suffix (?, [], ?[], []?, ?[]?)
            let suffix_start = segment.find(['?', '[']).unwrap_or(segment.len());
            let (field_part, suffix) = segment.split_at(suffix_start);

            let (is_nullable, list) = match suffix {
                "" => (false, None),
                "?" => (true, None),
                "[]" => (
                    false,
                    Some(List {
                        elements_nullable: false,
                    }),
                ),
                "?[]" => (
                    true,
                    Some(List {
                        elements_nullable: false,
                    }),
                ),
                "[]?" => (
                    false,
                    Some(List {
                        elements_nullable: true,
                    }),
                ),
                "?[]?" => (
                    true,
                    Some(List {
                        elements_nullable: true,
                    }),
                ),
                _ => return Err(PathParseError::InvalidSuffix { path, suffix }),
            };

            // Check for alias syntax: alias:field
            let (field, alias) = if let Some(colon_pos) = field_part.find(':') {
                let alias = &field_part[..colon_pos];
                let field = &field_part[colon_pos + 1..];
                (field, Some(alias))
            } else {
                (field_part, None)
            };

            if field.is_empty() {
                return Err(PathParseError::EmptySegment { path });
            }

            segments.push(PathSegment {
                field,
                alias,
                is_nullable,
                list,
            });
        }

        Ok(ParsedPath {
            raw: path,
            segments,
        })
    }
}

/// Errors that can occur when parsing a path.
#[derive(Debug, Clone)]
pub enum PathParseError<'a> {
    /// The path string is empty
    Empty,
    /// A segment in the path is empty (e.g., "foo..bar" or ".foo")
    EmptySegment { path: &'a str },
    /// Invalid suffix on a segment (e.g., "foo[?]", "foo??")
    InvalidSuffix { path: &'a str, suffix: &'a str },
}

impl<'a> std::fmt::Display for PathParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathParseError::Empty => write!(f, "Field path cannot be empty"),
            PathParseError::EmptySegment { path } => {
                write!(f, "Empty segment in path '{}'", path)
            }
            PathParseError::InvalidSuffix { path, suffix } => {
                write!(
                    f,
                    "Invalid suffix '{}' in path '{}'. Valid suffixes: ?, [], ?[], []?, ?[]?",
                    suffix, path
                )
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
        assert!(!path.segments[0].is_nullable);
        assert!(!path.segments[0].is_list());
        assert_eq!(path.segments[1].field, "address");
        assert!(!path.segments[1].is_list());
    }

    #[test]
    fn test_parse_nested_path() {
        let path = ParsedPath::parse("data.nodes.name").unwrap();
        assert_eq!(path.segments.len(), 3);
        assert_eq!(path.segments[0].field, "data");
        assert!(!path.segments[0].is_list());
        assert_eq!(path.segments[1].field, "nodes");
        assert!(!path.segments[1].is_list());
        assert_eq!(path.segments[2].field, "name");
        assert!(!path.segments[2].is_list());
    }

    #[test]
    fn test_parse_single_field() {
        let path = ParsedPath::parse("chainIdentifier").unwrap();
        assert_eq!(path.segments.len(), 1);
        assert_eq!(path.segments[0].field, "chainIdentifier");
        assert!(!path.segments[0].is_list());
    }

    #[test]
    fn test_parse_with_alias() {
        let path = ParsedPath::parse("epoch.firstCheckpoint:checkpoints.nodes[]").unwrap();
        assert_eq!(path.segments.len(), 3);
        assert_eq!(path.segments[0].field, "epoch");
        assert!(path.segments[0].alias.is_none());
        assert!(!path.segments[0].is_list());
        assert_eq!(path.segments[1].field, "checkpoints");
        assert_eq!(path.segments[1].alias, Some("firstCheckpoint"));
        assert!(!path.segments[1].is_list());
        assert_eq!(path.segments[2].field, "nodes");
        assert!(path.segments[2].is_list());
    }

    #[test]
    fn test_parse_array_with_alias() {
        let path = ParsedPath::parse("myObjects:objects[]").unwrap();
        assert_eq!(path.segments.len(), 1);
        assert_eq!(path.segments[0].field, "objects");
        assert_eq!(path.segments[0].alias, Some("myObjects"));
        assert!(path.segments[0].is_list());
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
        assert!(path.segments[0].is_list());
        assert_eq!(path.segments[1].field, "name");
        assert!(!path.segments[1].is_list());
    }

    #[test]
    fn test_parse_nested_arrays() {
        let path = ParsedPath::parse("groups[].members[].name").unwrap();
        assert_eq!(path.segments.len(), 3);
        assert_eq!(path.segments[0].field, "groups");
        assert!(path.segments[0].is_list());
        assert_eq!(path.segments[1].field, "members");
        assert!(path.segments[1].is_list());
        assert_eq!(path.segments[2].field, "name");
        assert!(!path.segments[2].is_list());
    }

    #[test]
    fn test_parse_trailing_array() {
        let path = ParsedPath::parse("items[].tags[]").unwrap();
        assert_eq!(path.segments.len(), 2);
        assert_eq!(path.segments[0].field, "items");
        assert!(path.segments[0].is_list());
        assert_eq!(path.segments[1].field, "tags");
        assert!(path.segments[1].is_list());
    }

    #[test]
    fn test_parse_empty_array_field_error() {
        let err = ParsedPath::parse("[].name").unwrap_err();
        assert!(matches!(err, PathParseError::EmptySegment { .. }));
    }

    // === Null marker parsing tests ===

    #[test]
    fn test_parse_nullable_field() {
        let path = ParsedPath::parse("object?.address?").unwrap();
        assert_eq!(path.segments.len(), 2);
        assert!(path.segments[0].is_nullable);
        assert!(!path.segments[0].is_list());
        assert!(path.segments[1].is_nullable);
        assert!(!path.segments[1].is_list());
    }

    #[test]
    fn test_parse_nullable_array() {
        // ?[] → array is nullable, elements are not
        let path = ParsedPath::parse("items?[].name").unwrap();
        assert!(path.segments[0].is_nullable);
        assert!(path.segments[0].is_list());
        assert!(!path.segments[0].list.as_ref().unwrap().elements_nullable);
    }

    #[test]
    fn test_parse_elements_nullable() {
        // []? → array is required, elements are nullable
        let path = ParsedPath::parse("items[]?.name").unwrap();
        assert!(!path.segments[0].is_nullable);
        assert!(path.segments[0].is_list());
        assert!(path.segments[0].list.as_ref().unwrap().elements_nullable);
    }

    #[test]
    fn test_parse_both_nullable() {
        // ?[]? → array is nullable, elements are nullable
        let path = ParsedPath::parse("items?[]?.name").unwrap();
        assert!(path.segments[0].is_nullable);
        assert!(path.segments[0].is_list());
        assert!(path.segments[0].list.as_ref().unwrap().elements_nullable);
    }

    #[test]
    fn test_parse_nullable_with_alias() {
        let path = ParsedPath::parse("myAddr:address?").unwrap();
        assert_eq!(path.segments[0].field, "address");
        assert_eq!(path.segments[0].alias, Some("myAddr"));
        assert!(path.segments[0].is_nullable);
    }

    #[test]
    fn test_parse_nullable_array_with_alias() {
        let path = ParsedPath::parse("myItems:items?[]?.name").unwrap();
        assert_eq!(path.segments[0].field, "items");
        assert_eq!(path.segments[0].alias, Some("myItems"));
        assert!(path.segments[0].is_nullable);
        assert!(path.segments[0].list.as_ref().unwrap().elements_nullable);
    }

    #[test]
    fn test_parse_mixed_nullable_path() {
        let path = ParsedPath::parse("epoch?.checkpoints?.nodes?[].digest").unwrap();
        assert_eq!(path.segments.len(), 4);
        assert!(path.segments[0].is_nullable);
        assert!(!path.segments[0].is_list());
        assert!(path.segments[1].is_nullable);
        assert!(!path.segments[1].is_list());
        assert!(path.segments[2].is_nullable);
        assert!(path.segments[2].is_list());
        assert!(!path.segments[3].is_nullable);
        assert!(!path.segments[3].is_list());
    }

    #[test]
    fn test_parse_invalid_suffix_error() {
        let err = ParsedPath::parse("foo[?].name").unwrap_err();
        assert!(matches!(err, PathParseError::InvalidSuffix { .. }));
    }
}
