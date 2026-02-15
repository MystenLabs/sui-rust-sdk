//! Field path validation against the GraphQL schema and Rust types.
//!
//! Validates paths and determines iteration by matching:
//! - Schema: which fields are lists
//! - Type: how many Vec wrappers
//!
//! Iteration happens automatically when schema says list AND type has Vec.

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
///
/// Note: Type detection uses simple name matching (e.g., `ident == "Option"`),
/// the same approach used by serde_derive. This works for standard library types
/// but won't distinguish custom types with the same name.
/// See: https://github.com/serde-rs/serde/blob/master/serde_derive/src/internals/attr.rs
pub fn analyze_type(ty: &syn::Type) -> TypeStructure {
    if let Some(inner) = unwrap_option(ty) {
        TypeStructure::Optional(Box::new(analyze_type(inner)))
    } else if let Some(inner) = unwrap_vec(ty) {
        TypeStructure::Vector(Box::new(analyze_type(inner)))
    } else {
        TypeStructure::Plain
    }
}

/// Returns the inner type if `ty` is `Option<T>`, otherwise `None`.
fn unwrap_option(ty: &syn::Type) -> Option<&syn::Type> {
    unwrap_type(ty, "Option")
}

/// Returns the inner type if `ty` is `Vec<T>`, otherwise `None`.
fn unwrap_vec(ty: &syn::Type) -> Option<&syn::Type> {
    unwrap_type(ty, "Vec")
}

/// Returns the inner type if `ty` matches `TypeName<T>`, otherwise `None`.
fn unwrap_type<'a>(ty: &'a syn::Type, type_name: &str) -> Option<&'a syn::Type> {
    let syn::Type::Path(type_path) = ungroup(ty) else {
        return None;
    };
    let seg = type_path.path.segments.last()?;
    let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };

    if seg.ident == type_name
        && args.args.len() == 1
        && let syn::GenericArgument::Type(inner) = &args.args[0]
    {
        return Some(inner);
    }
    None
}

/// Unwrap `syn::Type::Group` nodes which may appear in macro-generated code.
///
/// When a macro captures a type with `$t:ty` and substitutes it, the type may be
/// wrapped in an invisible `Group` node. For example, `Option<String>` might appear
/// as `Group(Path("Option<String>"))` instead of just `Path("Option<String>")`.
///
/// Credit: serde_derive (https://github.com/serde-rs/serde)
fn ungroup(mut ty: &syn::Type) -> &syn::Type {
    while let syn::Type::Group(group) = ty {
        ty = &group.elem;
    }
    ty
}

/// Count the number of `Vec` wrappers in a type structure.
pub fn count_vec_depth(ts: &TypeStructure) -> usize {
    match ts {
        TypeStructure::Plain => 0,
        TypeStructure::Optional(inner) => count_vec_depth(inner),
        TypeStructure::Vector(inner) => 1 + count_vec_depth(inner),
    }
}

/// Scalars whose values can be objects or arrays in JSON.
///
/// These scalars represent structured data (not simple strings or numbers), so the macro
/// cannot validate Vec count — the Rust type is the user's responsibility.
const OBJECT_LIKE_SCALARS: &[&str] = &[
    "JSON",
    "MoveTypeLayout",
    "MoveTypeSignature",
    "OpenMoveTypeSignature",
];

/// Validate a parsed field path against the schema.
///
/// Checks that all fields in the path exist in the schema and that `[]` markers
/// match the schema's list types. List fields must always use `[]` explicitly.
///
/// Returns the terminal type name (the type of the last field in the path).
pub fn validate_path_against_schema<'a>(
    schema: &'a Schema,
    root_type: &'a str,
    path: &ParsedPath,
    span: proc_macro2::Span,
) -> Result<&'a str, syn::Error> {
    let mut current_type: &str = root_type;

    for segment in &path.segments {
        let field = schema
            .get_field(current_type, segment.field)
            .ok_or_else(|| field_not_found_error(schema, current_type, segment.field, span))?;

        if segment.is_list && !field.is_list {
            return Err(syn::Error::new(
                span,
                format!(
                    "Cannot use '[]' on non-list field '{}' (type '{}')",
                    segment.field, field.type_name
                ),
            ));
        }

        if !segment.is_list && field.is_list {
            return Err(syn::Error::new(
                span,
                format!(
                    "Field '{}' is a list type, use '{}[]' to iterate over it",
                    segment.field, segment.field
                ),
            ));
        }

        current_type = &field.type_name;
    }

    Ok(current_type)
}

/// Validate that a type name is a member of a union.
pub fn validate_union_member(
    schema: &Schema,
    union_type: &str,
    member_name: &str,
    span: proc_macro2::Span,
) -> Result<(), syn::Error> {
    let union_types = schema.union_types(union_type);
    if !union_types.contains(&member_name) {
        let suggestion = find_similar(&union_types, member_name);

        let mut msg = format!(
            "'{}' is not a member of union '{}'. Members: {}",
            member_name,
            union_type,
            union_types.join(", ")
        );
        if let Some(s) = suggestion {
            msg.push_str(&format!(". Did you mean '{}'?", s));
        }
        return Err(syn::Error::new(span, msg));
    }
    Ok(())
}

/// Returns true if the type name is a scalar that can represent objects or arrays.
pub fn is_object_like_scalar(type_name: &str) -> bool {
    OBJECT_LIKE_SCALARS.contains(&type_name)
}

/// Validate that the type's Vec count matches the number of list fields in the path.
///
/// When `skip_vec_excess_check` is true, extra Vec wrappers beyond the list count are
/// allowed. This is used for object-like scalars (e.g., JSON) whose values can be arrays.
///
/// # Errors
///
/// - If Vec count < list count: points to the specific list field missing a Vec
/// - If Vec count > list count (and `skip_vec_excess_check` is false): too many Vec wrappers
pub fn validate_type_matches_path(
    path: &ParsedPath<'_>,
    ty: &syn::Type,
    skip_vec_excess_check: bool,
) -> Result<(), syn::Error> {
    let type_structure = analyze_type(ty);
    let vec_count = count_vec_depth(&type_structure);
    let list_count = path.list_fields().len();

    if vec_count < list_count {
        let list_fields = path.list_fields();
        let mismatched_field = list_fields[vec_count];
        return Err(syn::Error::new_spanned(
            ty,
            format!(
                "field '{}' is a list but type has no Vec wrapper for it",
                mismatched_field
            ),
        ));
    }

    if !skip_vec_excess_check && vec_count > list_count {
        return Err(syn::Error::new_spanned(
            ty,
            format!(
                "type has {} Vec wrapper(s) but path '{}' has {} list field(s)",
                vec_count, path.raw, list_count
            ),
        ));
    }

    Ok(())
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
pub fn find_similar<'a>(candidates: &[&'a str], target: &str) -> Option<&'a str> {
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
