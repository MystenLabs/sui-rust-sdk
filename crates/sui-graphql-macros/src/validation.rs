//! Field path validation against the GraphQL schema and Rust types.

use std::fmt::Write;

use crate::schema::Schema;
use darling::util::SpannedValue;

/// Represents the nesting structure of `Option` and `Vec` in a field type.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeStructure {
    /// A type that is neither `Option` nor `Vec` (e.g., `String`, `u64`)
    Plain,
    /// `Option<T>` wrapping an inner structure
    Optional(Box<TypeStructure>),
    /// `Vec<T>` wrapping an inner structure
    Vector(Box<TypeStructure>),
}

/// Analyze a `syn::Type` into a `TypeStructure`.
pub fn analyze_type(ty: &syn::Type) -> TypeStructure {
    if let syn::Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        let ident_str = segment.ident.to_string();

        if ident_str == "Option" {
            let inner =
                extract_first_generic_arg(segment).expect("Option must have a type argument");
            return TypeStructure::Optional(Box::new(analyze_type(inner)));
        }

        if ident_str == "Vec" {
            let inner = extract_first_generic_arg(segment).expect("Vec must have a type argument");
            return TypeStructure::Vector(Box::new(analyze_type(inner)));
        }
    }
    TypeStructure::Plain
}

/// Extract the first generic type argument from a path segment.
fn extract_first_generic_arg(segment: &syn::PathSegment) -> Option<&syn::Type> {
    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        for arg in &args.args {
            if let syn::GenericArgument::Type(inner) = arg {
                return Some(inner);
            }
        }
    }
    None
}

/// Count the number of `Vec` wrappers in a type structure.
fn count_vec_depth(ts: &TypeStructure) -> usize {
    match ts {
        TypeStructure::Plain => 0,
        TypeStructure::Optional(inner) => count_vec_depth(inner),
        TypeStructure::Vector(inner) => 1 + count_vec_depth(inner),
    }
}

/// Count the number of array segments (`[]`) in a path.
fn count_array_segments(path: &str) -> usize {
    path.split('.').filter(|s| s.ends_with("[]")).count()
}

/// Validate a struct field's path attribute and type.
///
/// This performs validation in order:
/// 1. Checks that the `path` attribute is provided
/// 2. Checks that the path is not empty
/// 3. Validates that the path structure matches the type structure ([] count matches Vec count)
/// 4. Validates the path against the GraphQL schema (unless skipped)
///
/// Errors point to the path attribute when available, otherwise to the field identifier.
pub fn validate_field(
    schema: &Schema,
    path: Option<&SpannedValue<String>>,
    ty: &syn::Type,
    field_ident: &syn::Ident,
    skip_schema_validation: bool,
) -> Result<(), syn::Error> {
    // Check that path attribute is provided (point to field since there's no path to point to)
    let path = path.ok_or_else(|| {
        syn::Error::new_spanned(field_ident, "missing #[field(path = \"...\")] attribute")
    })?;

    // Check that path is not empty (point to the path attribute)
    if path.is_empty() {
        return Err(syn::Error::new(path.span(), "Field path cannot be empty"));
    }

    // Ensure the number of `[]` in the path matches the number of `Vec` in the type
    validate_path_type_match(path.as_str(), ty, field_ident)?;

    // Validate against GraphQL schema (point to the path attribute)
    if !skip_schema_validation {
        validate_path_against_schema(schema, path.as_str(), path.span())?;
    }

    Ok(())
}

/// Validate that the path structure matches the type structure.
fn validate_path_type_match(
    path: &str,
    ty: &syn::Type,
    field_ident: &syn::Ident,
) -> Result<(), syn::Error> {
    let array_count = count_array_segments(path);
    let type_structure = analyze_type(ty);
    let vec_count = count_vec_depth(&type_structure);

    if array_count != vec_count {
        return Err(syn::Error::new_spanned(
            field_ident,
            format!(
                "path '{}' has {} array segment(s) but type has {} Vec wrapper(s)",
                path, array_count, vec_count
            ),
        ));
    }

    Ok(())
}

/// Validate a field path against the GraphQL schema, starting from the Query type.
///
/// A path like `"object.address"` validates that:
/// - Query type has a field named `object`
/// - The type returned by `object` has a field named `address`
///
/// For array paths like `"objects[].address"`:
/// - Validates that `objects` is a list type
/// - Validates fields after `[]` against the list element type
///
/// Returns the GraphQL type name of the final field.
fn validate_path_against_schema(
    schema: &Schema,
    path: &str,
    span: proc_macro2::Span,
) -> Result<String, syn::Error> {
    let segments: Vec<&str> = path.split('.').collect();

    let mut current_type: &str = "Query";

    for segment in &segments {
        // Handle array iteration: "field[]"
        if let Some(field_name) = segment.strip_suffix("[]") {
            // Look up the field
            let field = schema
                .field(current_type, field_name)
                .ok_or_else(|| field_not_found_error(schema, current_type, field_name, span))?;

            // Verify it's a list type
            if !field.is_list {
                return Err(syn::Error::new(
                    span,
                    format!(
                        "Cannot use '[]' on non-list field '{}' (type '{}')",
                        field_name, field.type_name
                    ),
                ));
            }

            // Continue with the element type
            current_type = &field.type_name;
        } else {
            // Regular field access
            let field = schema
                .field(current_type, segment)
                .ok_or_else(|| field_not_found_error(schema, current_type, segment, span))?;

            current_type = &field.type_name;
        }
    }

    Ok(current_type.to_string())
}

/// Generate an error for a field not found, with "Did you mean?" suggestion.
fn field_not_found_error(
    schema: &Schema,
    type_name: &str,
    field_name: &str,
    span: proc_macro2::Span,
) -> syn::Error {
    let available = schema.field_names(type_name);
    let suggestion = find_similar(&available, field_name);

    let mut msg = format!("Field '{field_name}' not found on type '{type_name}'");

    if let Some(suggested) = suggestion {
        write!(msg, ". Did you mean '{suggested}'?").unwrap();
    } else if !available.is_empty() {
        // Sort for deterministic error messages (HashMap iteration order is random)
        let mut fields: Vec<_> = available;
        fields.sort();
        write!(msg, ". Available fields: {}", fields.join(", ")).unwrap();
    }

    syn::Error::new(span, msg)
}

/// Find a similar string using Levenshtein distance.
fn find_similar<'a>(candidates: &[&'a str], target: &str) -> Option<&'a str> {
    candidates
        .iter()
        .filter_map(|&candidate| {
            let distance = edit_distance::edit_distance(candidate, target);
            // Suggest if within 3 edits
            if distance <= 3 {
                Some((candidate, distance))
            } else {
                None
            }
        })
        .min_by_key(|(_, d)| *d)
        .map(|(c, _)| c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_similar() {
        let candidates = vec!["address", "version", "digest", "owner"];
        assert_eq!(find_similar(&candidates, "addrss"), Some("address"));
        assert_eq!(find_similar(&candidates, "vesion"), Some("version"));
        assert_eq!(find_similar(&candidates, "xyz"), None);
    }
}
