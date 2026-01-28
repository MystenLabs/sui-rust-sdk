//! Procedural macros for sui-graphql with compile-time schema validation.
//!
//! This crate provides the `Response` derive macro that:
//! - Validates field paths against the Sui GraphQL schema at compile time
//! - Generates deserialization code for extracting nested fields from JSON responses
//!
//! # Example
//!
//! ```ignore
//! use sui_graphql_macros::Response;
//!
//! #[derive(Response)]
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

use darling::FromDeriveInput;
use darling::FromField;
use darling::util::SpannedValue;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;
use syn::parse_macro_input;

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
/// #[derive(Response)]
/// struct ChainInfo {
///     #[field(path = "chainIdentifier")]
///     chain_id: String,
///
///     #[field(path = "epoch.epochId")]
///     epoch_id: Option<u64>,
/// }
/// ```
#[proc_macro_derive(Response, attributes(response, field))]
pub fn derive_query_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_query_response_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn derive_query_response_impl(input: DeriveInput) -> Result<TokenStream2, syn::Error> {
    // Darling input structures - define the "schema" for macro input.
    // Darling generates parsing code automatically, including error messages for invalid input.

    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(response), supports(struct_named))]
    struct ResponseInput {
        ident: syn::Ident,                           // Struct name
        generics: syn::Generics,                     // Generic parameters
        data: darling::ast::Data<(), ResponseField>, // Struct fields
        #[darling(default)]
        schema: Option<String>, // Custom schema path: #[response(schema = "path/to/schema.graphql")]
    }

    #[derive(Debug, FromField)]
    #[darling(attributes(field))] // Parse #[field(...)] attributes
    struct ResponseField {
        ident: Option<syn::Ident>, // Field name
        ty: syn::Type,             // Field type (auto-populated by darling)
        #[darling(default)]
        path: Option<SpannedValue<String>>, // The path = "..." value with span for error messages
        #[darling(default)]
        skip_schema_validation: bool,
    }

    let parsed = ResponseInput::from_derive_input(&input)?;

    // Load the GraphQL schema for validation.
    // If a custom schema path is provided, load it; otherwise use the embedded Sui schema.
    let schema = match &parsed.schema {
        Some(path) => {
            // Resolve path relative to the crate's directory.
            // SUI_GRAPHQL_SCHEMA_DIR is used by trybuild tests (which run from a temp directory).
            let base_dir = std::env::var("SUI_GRAPHQL_SCHEMA_DIR")
                .or_else(|_| std::env::var("CARGO_MANIFEST_DIR"))
                .unwrap();
            let full_path = std::path::Path::new(&base_dir).join(path);
            let sdl = std::fs::read_to_string(&full_path).map_err(|e| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Failed to read schema from '{}': {}",
                        full_path.display(),
                        e
                    ),
                )
            })?;
            schema::Schema::from_sdl(&sdl)?
        }
        None => schema::Schema::load()?.clone(),
    };

    let fields = parsed
        .data
        .as_ref()
        .take_struct()
        .ok_or_else(|| syn::Error::new_spanned(&input, "Response only supports structs"))?
        .fields;

    // Generate extraction code for each field
    let mut field_extractions = Vec::new();
    let mut field_names = Vec::new();

    for field in &fields {
        let field_ident = field
            .ident
            .as_ref()
            .ok_or_else(|| syn::Error::new_spanned(&input, "Unnamed fields not supported"))?;

        // Validate field: checks path is provided, not empty, type structure matches, and valid against schema
        validation::validate_field(
            &schema,
            field.path.as_ref(),
            &field.ty,
            field_ident,
            field.skip_schema_validation,
        )?;

        // Safe to unwrap as validate_field ensures path is Some
        let path = field.path.as_ref().unwrap().as_str();

        let type_structure = validation::analyze_type(&field.ty);
        let extraction = generate_field_extraction(path, &type_structure, field_ident);
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

        // TODO: Implement efficient deserialization that only extracts the fields we need.
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
/// Paths like `"data.nodes[].name"` are parsed into segments:
/// - `PathSegment { field: "data", is_array: false }`
/// - `PathSegment { field: "nodes", is_array: true }`
/// - `PathSegment { field: "name", is_array: false }`
struct PathSegment<'a> {
    /// The field name to access
    field: &'a str,
    /// Whether this is an array field (ends with `[]`)
    is_array: bool,
}

/// Parse a path string into segments.
///
/// Each dot-separated part becomes a `PathSegment`. If it ends with `[]`, it's an array field.
fn parse_path(path: &str) -> Vec<PathSegment<'_>> {
    path.split('.')
        .map(|segment| {
            let (field, is_array) = if let Some(stripped) = segment.strip_suffix("[]") {
                (stripped, true)
            } else {
                (segment, false)
            };
            PathSegment { field, is_array }
        })
        .collect()
}

/// Generate code to extract a single field from JSON using its path and type structure.
fn generate_field_extraction(
    path: &str,
    type_structure: &validation::TypeStructure,
    field_ident: &syn::Ident,
) -> TokenStream2 {
    let segments = parse_path(path);
    let extraction_code = generate_from_segments(path, &segments, type_structure);

    quote! {
        let #field_ident = {
            let current = &value;
            #extraction_code?
        };
    }
}

/// Recursively generate extraction code by traversing path segments.
///
/// The `type_structure` determines how null values are handled at array boundaries:
/// - `Optional(Vector(...))`: null array -> Ok(None), wrap result in Some
/// - `Vector(...)`: null array -> Err, return vec directly
fn generate_from_segments(
    full_path: &str,
    segments: &[PathSegment<'_>],
    type_structure: &validation::TypeStructure,
) -> TokenStream2 {
    // Base case: no more segments, deserialize the current value
    let Some((
        PathSegment {
            field: name,
            is_array,
        },
        rest_segments,
    )) = segments.split_first()
    else {
        return quote! {
            serde_json::from_value(current.clone())
                .map_err(|e| format!("failed to deserialize '{}': {}", #full_path, e))
        };
    };

    if *is_array {
        let (is_optional, element_type) = match type_structure {
            validation::TypeStructure::Optional(inner) => match inner.as_ref() {
                validation::TypeStructure::Vector(v) => (true, v.as_ref()),
                _ => unreachable!("validated: Option with [] must be Option<Vec<...>>"),
            },
            validation::TypeStructure::Vector(inner) => (false, inner.as_ref()),
            _ => unreachable!("validated: [] segment requires Vec or Option<Vec> type"),
        };

        let rest = generate_from_segments(full_path, rest_segments, element_type);

        let on_null = if is_optional {
            quote! { Ok(None) }
        } else {
            quote! { Err(format!("null value at '{}' in path '{}', expected array", #name, #full_path)) }
        };

        let wrap_result = if is_optional {
            quote! { .map(Some) }
        } else {
            quote! {}
        };

        quote! {
            {
                let field_value = current.get(#name)
                    .ok_or_else(|| format!("missing field '{}' in path '{}'", #name, #full_path))?;
                if field_value.is_null() {
                    #on_null
                } else {
                    field_value.as_array()
                        .ok_or_else(|| format!("expected array at '{}' in path '{}'", #name, #full_path))?
                        .iter()
                        .map(|current| { #rest })
                        .collect::<Result<Vec<_>, String>>()
                        #wrap_result
                }
            }
        }
    } else {
        // Non-array segment: navigate to the field and continue with remaining segments.
        let rest = generate_from_segments(full_path, rest_segments, type_structure);

        quote! {
            {
                let current = current.get(#name)
                    .ok_or_else(|| format!("missing field '{}' in path '{}'", #name, #full_path))?;
                // If null, let serde handle it (returns Ok(None) for Option<T>, error for non-Option)
                if current.is_null() {
                    serde_json::from_value(current.clone())
                        .map_err(|e| format!("failed to deserialize '{}': {}", #full_path, e))
                } else {
                    #rest
                }
            }
        }
    }
}
