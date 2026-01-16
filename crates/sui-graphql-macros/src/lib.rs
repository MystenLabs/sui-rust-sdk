//! Procedural macros for sui-graphql with compile-time schema validation.
//!
//! This crate provides the `QueryResponse` derive macro that:
//! - Validates field paths against the Sui GraphQL schema at compile time
//! - Generates deserialization code for extracting nested fields from JSON responses
//!
//! # Example
//!
//! ```ignore
//! use sui_graphql_macros::QueryResponse;
//!
//! #[derive(QueryResponse)]
//! struct ObjectData {
//!     #[field(path = "object.address")]
//!     address: String,
//!     #[field(path = "object.version")]
//!     version: u64,
//! }
//!
//! // The macro validates paths against the schema and generates extraction code.
//! // Invalid paths like "object.nonexistent" will cause a compile error.
//! ```

extern crate proc_macro;

mod schema;
mod validation;

use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for GraphQL response types with nested field extraction.
///
/// Use `#[field(path = "...")]` to specify the JSON path to extract each field.
/// Paths are dot-separated (e.g., `"object.address"` extracts `json["object"]["address"]`).
///
/// # Generated Code
///
/// The macro generates:
/// - `from_value(serde_json::Value) -> Result<Self, String>` method
/// - `Deserialize` implementation that uses `from_value`
///
/// # Example
///
/// ```ignore
/// #[derive(QueryResponse)]
/// struct ChainInfo {
///     #[field(path = "chainIdentifier")]
///     chain_id: String,
///
///     #[field(path = "epoch.epochId")]
///     epoch_id: Option<u64>,
/// }
/// ```
#[proc_macro_derive(QueryResponse, attributes(field))]
pub fn derive_query_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_query_response_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn derive_query_response_impl(input: DeriveInput) -> Result<TokenStream2, syn::Error> {
    let parsed = QueryResponseInput::from_derive_input(&input)?;

    // Load the GraphQL schema for validation
    let schema = schema::Schema::load()?;

    let fields = parsed
        .data
        .as_ref()
        .take_struct()
        .ok_or_else(|| syn::Error::new_spanned(&input, "QueryResponse only supports structs"))?
        .fields;

    // Generate extraction code for each field
    let mut field_extractions = Vec::new();
    let mut field_names = Vec::new();

    for field in &fields {
        let field_ident = field
            .ident
            .as_ref()
            .ok_or_else(|| syn::Error::new_spanned(&input, "Unnamed fields not supported"))?;

        let path = &field.path;

        // Validate the path against the GraphQL schema (unless skip_validation is set)
        if !field.skip_validation {
            validation::validate_path(schema, path, field_ident)?;
        }

        let extraction = generate_field_extraction(path, field_ident);
        field_extractions.push(extraction);
        field_names.push(field_ident);
    }

    let struct_ident = &parsed.ident;
    let (impl_generics, ty_generics, where_clause) = parsed.generics.split_for_impl();

    // Generate both `from_value` and `Deserialize` impl:
    //
    // - `from_value`: Core extraction logic, parses from serde_json::Value
    // - `Deserialize`: Allows direct use with serde (e.g., `serde_json::from_str::<MyStruct>(...)`)
    //   and with the GraphQL client's `query::<T>()` which requires `T: DeserializeOwned`
    let output = quote! {
        impl #impl_generics #struct_ident #ty_generics #where_clause {
            pub fn from_value(value: serde_json::Value) -> Result<Self, String> {
                #(#field_extractions)*

                Ok(Self {
                    #(#field_names),*
                })
            }
        }

        impl<'de> serde::Deserialize<'de> for #struct_ident #ty_generics #where_clause {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = serde_json::Value::deserialize(deserializer)?;
                Self::from_value(value).map_err(serde::de::Error::custom)
            }
        }
    };

    Ok(output)
}

/// A segment in a field path.
///
/// Segments can include an alias using `@` syntax for GraphQL aliases:
/// - The field name (before `@`) is used for schema validation
/// - The alias (after `@`) is used for JSON extraction
///
/// Examples:
/// - `"data.nodes[].name"` parses to:
///   - `{ field: "data", alias: None, is_array: false }`
///   - `{ field: "nodes", alias: None, is_array: true }`
///   - `{ field: "name", alias: None, is_array: false }`
///
/// - `"epoch.checkpoints@firstCheckpoints.nodes[].sequenceNumber"` parses to:
///   - `{ field: "epoch", alias: None, is_array: false }`
///   - `{ field: "checkpoints", alias: Some("firstCheckpoints"), is_array: false }`
///   - `{ field: "nodes", alias: None, is_array: true }`
///   - `{ field: "sequenceNumber", alias: None, is_array: false }`
struct PathSegment<'a> {
    /// The field name (used for schema validation)
    field: &'a str,
    /// Optional alias (used for JSON extraction instead of field name)
    alias: Option<&'a str>,
    /// Whether this is an array field (ends with `[]`)
    is_array: bool,
}

/// Parse a path string into segments.
///
/// Each dot-separated part is parsed for:
/// - Array suffix `[]` (e.g., `nodes[]`)
/// - Alias syntax `@` (e.g., `checkpoints@firstCheckpoints`)
fn parse_path(path: &str) -> Vec<PathSegment<'_>> {
    path.split('.')
        .map(|segment| {
            // Check for array suffix first
            let (segment, is_array) = if let Some(stripped) = segment.strip_suffix("[]") {
                (stripped, true)
            } else {
                (segment, false)
            };

            // Check for alias syntax: field@alias
            let (field, alias) = if let Some(at_pos) = segment.find('@') {
                (&segment[..at_pos], Some(&segment[at_pos + 1..]))
            } else {
                (segment, None)
            };

            PathSegment {
                field,
                alias,
                is_array,
            }
        })
        .collect()
}

/// Generate code to extract a single field from JSON using its path.
///
/// Supports multiple path formats:
/// - Simple: `"object.address"` - navigates to nested field
/// - Array: `"nodes[].name"` - iterates over array, extracts field from each element
/// - Nested arrays: `"nodes[].edges[].id"` - nested iteration, returns `Vec<Vec<T>>`
fn generate_field_extraction(path: &str, field_ident: &syn::Ident) -> TokenStream2 {
    let segments = parse_path(path);
    let inner = generate_from_segments(path, &segments);
    // The inner expression returns Result<T, String>, so we use ? to unwrap
    quote! {
        let #field_ident = {
            let current = &value;
            #inner?
        };
    }
}

/// Recursively generate extraction code by traversing path segments.
///
/// For JSON extraction, uses the alias if present, otherwise uses the field name.
/// Returns code that evaluates to `Result<T, String>` (caller adds `?` to unwrap).
///
/// ## Example: Simple path `"object.address"`
///
/// ```ignore
/// let current = current.get("object").ok_or_else(|| ...)?;
/// if current.is_null() { return serde_json::from_value(null)... }
/// let current = current.get("address").ok_or_else(|| ...)?;
/// serde_json::from_value(current.clone()).map_err(|e| ...)
/// ```
///
/// ## Example: Array path `"nodes[].name"` (returns `Option<Vec<T>>`)
///
/// ```ignore
/// {
///     let field_value = current.get("nodes").ok_or_else(|| ...)?;
///     if field_value.is_null() {
///         Ok(None)
///     } else {
///         let array = field_value.as_array().ok_or_else(|| ...)?;
///         let vec: Vec<_> = array.iter().map(|current| { ... }).collect::<Result<_, _>>()?;
///         Ok(Some(vec))
///     }
/// }
/// ```
fn generate_from_segments(full_path: &str, segments: &[PathSegment<'_>]) -> TokenStream2 {
    // Base case: no more segments, deserialize the current value
    if segments.is_empty() {
        return quote! {
            serde_json::from_value(current.clone())
                .map_err(|e| format!("failed to deserialize '{}': {}", #full_path, e))
        };
    }

    let rest = generate_from_segments(full_path, &segments[1..]);
    let segment = &segments[0];

    // Use alias for JSON extraction if present, otherwise use field name
    let json_key = segment.alias.unwrap_or(segment.field);

    if segment.is_array {
        // Array field: check for null, then iterate and collect into Option<Vec>
        quote! {
            {
                let field_value = current.get(#json_key)
                    .ok_or_else(|| format!("missing field '{}' in path '{}'", #json_key, #full_path))?;
                if field_value.is_null() {
                    Ok(None)
                } else {
                    let array = field_value.as_array()
                        .ok_or_else(|| format!("expected array at '{}' in path '{}'", #json_key, #full_path))?;
                    array.iter()
                        .map(|current| { #rest })
                        .collect::<Result<Vec<_>, String>>()
                        .map(Some)
                }
            }
        }
    } else {
        quote! {
            let current = current.get(#json_key)
                .ok_or_else(|| format!("missing field '{}' in path '{}'", #json_key, #full_path))?;
            // If null, skip remaining navigation and let serde handle it
            // (returns Ok(None) for Option<T>, error for non-Option)
            if current.is_null() {
                serde_json::from_value(current.clone())
                    .map_err(|e| format!("failed to deserialize '{}': {}", #full_path, e))
            } else {
                #rest
            }
        }
    }
}

// Darling input structures - these define the "schema" for our macro input.
// Darling generates parsing code automatically, including error messages for invalid input.

#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_named))] // Only supports structs with named fields
struct QueryResponseInput {
    ident: syn::Ident,                                // Struct name
    generics: syn::Generics,                          // Generic parameters
    data: darling::ast::Data<(), QueryResponseField>, // Struct fields
}

#[derive(Debug, FromField)]
#[darling(attributes(field))] // Parse #[field(...)] attributes
struct QueryResponseField {
    ident: Option<syn::Ident>, // Field name
    path: String,              // The path = "..." value
    #[darling(default)]
    skip_validation: bool, // Skip schema validation if true
}
