//! Field path validation against the GraphQL schema.

use crate::schema::Schema;
use darling::util::SpannedValue;

/// Validate a struct field's path attribute.
///
/// This performs validation in order:
/// 1. Checks that the `path` attribute is provided
/// 2. Checks that the path is not empty
/// 3. Validates the path against the GraphQL schema (unless skipped)
///
/// Errors point to the path attribute when available, otherwise to the field identifier.
pub fn validate_field(
    schema: &Schema,
    path: Option<&SpannedValue<String>>,
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

    // Validate against GraphQL schema (point to the path attribute)
    if !skip_schema_validation {
        validate_path_against_schema(schema, path.as_str(), path.span())?;
    }

    Ok(())
}

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
                .get_field(current_type, field_name)
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
                .get_field(current_type, segment)
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
        msg.push_str(&format!(". Did you mean '{suggested}'?"));
    } else if !available.is_empty() {
        // Sort for deterministic error messages (HashMap iteration order is random)
        let mut fields: Vec<_> = available;
        fields.sort();
        let fields_str = fields.join(", ");
        msg.push_str(&format!(". Available fields: {fields_str}"));
    }

    syn::Error::new(span, msg)
}

/// Find a similar string using Levenshtein distance.
fn find_similar<'a>(candidates: &[&'a str], target: &str) -> Option<&'a str> {
    candidates
        .iter()
        .filter_map(|&candidate| {
            let distance = levenshtein_distance(candidate, target);
            // Only suggest if distance is reasonable (less than half the target length + 1)
            if distance <= target.len() / 2 + 1 {
                Some((candidate, distance))
            } else {
                None
            }
        })
        .min_by_key(|(_, d)| *d)
        .map(|(c, _)| c)
}

/// Simple Levenshtein distance implementation.
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let m = a_chars.len();
    let n = b_chars.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut dp = vec![vec![0; n + 1]; m + 1];

    #[allow(clippy::needless_range_loop)]
    for i in 0..=m {
        dp[i][0] = i;
    }
    #[allow(clippy::needless_range_loop)]
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein_distance("address", "addrss"), 1);
        assert_eq!(levenshtein_distance("address", "address"), 0);
        assert_eq!(levenshtein_distance("version", "vesion"), 1);
    }

    #[test]
    fn test_find_similar() {
        let candidates = vec!["address", "version", "digest", "owner"];
        assert_eq!(find_similar(&candidates, "addrss"), Some("address"));
        assert_eq!(find_similar(&candidates, "vesion"), Some("version"));
        assert_eq!(find_similar(&candidates, "xyz"), None);
    }
}
