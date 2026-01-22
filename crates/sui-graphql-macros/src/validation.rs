//! Field path validation against the GraphQL schema.

use crate::schema::Schema;

/// Validate a field path against the schema, starting from the Query type.
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
pub fn validate_path(schema: &Schema, path: &str, span: &syn::Ident) -> Result<String, syn::Error> {
    if path.is_empty() {
        return Err(syn::Error::new_spanned(span, "Field path cannot be empty"));
    }

    let segments: Vec<&str> = path.split('.').collect();

    let mut current_type = "Query".to_string();

    for (i, segment) in segments.iter().enumerate() {
        // Handle array iteration: "field[]"
        if let Some(field_name) = segment.strip_suffix("[]") {
            // Look up the field
            let field = schema
                .get_field(&current_type, field_name)
                .ok_or_else(|| field_not_found_error(&current_type, field_name, path, span))?;

            // Verify it's a list type
            if !field.is_list {
                return Err(syn::Error::new_spanned(
                    span,
                    format!(
                        "Cannot use '[]' on non-list field '{}' (type '{}') in path '{}'",
                        field_name, field.type_name, path
                    ),
                ));
            }

            // Continue with the element type
            current_type = field.type_name.clone();
        } else {
            // Regular field access
            let field = schema
                .get_field(&current_type, segment)
                .ok_or_else(|| field_not_found_error(&current_type, segment, path, span))?;

            // If this is the last segment, return the type
            if i == segments.len() - 1 {
                return Ok(field.type_name.clone());
            }

            current_type = field.type_name.clone();
        }
    }

    // Should not reach here since we return in the loop for the last segment
    Ok(current_type)
}

/// Generate an error for a field not found.
fn field_not_found_error(
    type_name: &str,
    field_name: &str,
    path: &str,
    span: &syn::Ident,
) -> syn::Error {
    syn::Error::new_spanned(
        span,
        format!("Field '{field_name}' not found on type '{type_name}' in path '{path}'"),
    )
}
