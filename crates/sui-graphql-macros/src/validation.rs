//! Field path validation against the GraphQL schema and Rust types.

use crate::path::ParsedPath;
use crate::schema::Schema;

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

/// Validate that the path structure matches the type structure.
///
/// Ensures the number of `[]` in the path matches the number of `Vec` wrappers in the type.
pub fn validate_path_type_match(
    path: &ParsedPath,
    ty: &syn::Type,
    field_ident: &syn::Ident,
) -> Result<(), syn::Error> {
    let array_count = path.segments.iter().filter(|s| s.is_array).count();
    let type_structure = analyze_type(ty);
    let vec_count = count_vec_depth(&type_structure);

    if array_count != vec_count {
        return Err(syn::Error::new_spanned(
            field_ident,
            format!(
                "path '{}' has {} array segment(s) but type has {} Vec wrapper(s)",
                path.raw, array_count, vec_count
            ),
        ));
    }

    Ok(())
}

/// Validate a parsed field path against the schema, starting from the Query type.
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
pub fn validate_path_against_schema(
    schema: &Schema,
    path: &ParsedPath,
    span: proc_macro2::Span,
) -> Result<String, syn::Error> {
    let mut current_type: &str = "Query";

    for segment in &path.segments {
        // Look up the field
        let field = schema
            .get_field(current_type, segment.field)
            .ok_or_else(|| field_not_found_error(schema, current_type, segment.field, span))?;

        // If marked as array, verify it's actually a list type
        if segment.is_array && !field.is_list {
            return Err(syn::Error::new(
                span,
                format!(
                    "Cannot use '[]' on non-list field '{}' (type '{}')",
                    segment.field, field.type_name
                ),
            ));
        }

        current_type = &field.type_name;
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
