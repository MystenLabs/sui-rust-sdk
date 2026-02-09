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

mod path;
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
    #[darling(attributes(field))]
    struct ResponseField {
        ident: Option<syn::Ident>,
        ty: syn::Type,
        path: SpannedValue<String>, // Required - darling will error if missing
        #[darling(default)]
        skip_schema_validation: bool,
    }

    let parsed = ResponseInput::from_derive_input(&input)?;

    // Load the GraphQL schema for validation.
    // If a custom schema path is provided, load it; otherwise use the embedded Sui schema.
    let loaded_schema = if let Some(path) = &parsed.schema {
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
        Some(schema::Schema::from_sdl(&sdl)?)
    } else {
        None
    };
    let schema = if let Some(schema) = &loaded_schema {
        schema
    } else {
        schema::Schema::load()?
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

        // Parse path once - used for both validation and code generation
        let spanned_path = &field.path;
        let mut parsed_path = path::ParsedPath::parse(spanned_path.as_str())
            .map_err(|e| syn::Error::new(spanned_path.span(), e.to_string()))?;

        // Validate path against GraphQL schema
        if !field.skip_schema_validation {
            validation::validate_path_against_schema(
                schema,
                &mut parsed_path,
                spanned_path.span(),
            )?;
        }

        // Validate type matches path (handles trailing array inference when schema validation is skipped)
        validation::validate_type_matches_path(&mut parsed_path, &field.ty)?;

        // Generate extraction code using the same parsed path
        let type_structure = validation::analyze_type(&field.ty);
        let extraction = generate_field_extraction(&parsed_path, &type_structure, field_ident);
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

/// Generate code to extract a single field from JSON using its path.
///
/// Supports multiple path formats:
/// - Simple: `"object.address"` - navigates to nested field
/// - Array: `"nodes[].name"` - iterates over array, extracts field from each element
/// - Nested arrays: `"nodes[].edges[].id"` - nested iteration, returns `Vec<Vec<T>>`
/// - Aliased: `"alias:field"` - uses alias for JSON extraction, field for validation
fn generate_field_extraction(
    path: &path::ParsedPath,
    type_structure: &validation::TypeStructure,
    field_ident: &syn::Ident,
) -> TokenStream2 {
    let full_path = &path.raw;
    let inner = generate_from_segments(full_path, &path.segments, type_structure);
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
/// ## Design: Separation of Optional and Array handling
///
/// Optional handling and array handling are separate concerns:
/// 1. First, check if the outer type is Optional and unwrap it
/// 2. Then, handle arrays by unwrapping Vector
///
/// For Optional types, null at ANY point in the path returns `Ok(None)`.
/// This is handled by wrapping the core logic in a closure for early return.
///
/// ## Example: Simple path `"object.address"` with `Option<String>`
///
/// ```ignore
/// (|| {
///     let current = current.get("object").ok_or_else(|| ...)?;
///     if current.is_null() { return Ok(None); }
///     let current = current.get("address").ok_or_else(|| ...)?;
///     if current.is_null() { return Ok(None); }
///     serde_json::from_value(current.clone()).map_err(...).map(Some)
/// })()
/// ```
///
/// ## Example: Array path `"nodes[].name"` with `Option<Vec<String>>`
///
/// ```ignore
/// (|| {
///     let field_value = current.get("nodes").ok_or_else(|| ...)?;
///     if field_value.is_null() { return Ok(None); }
///     let array = field_value.as_array().ok_or_else(|| ...)?;
///     array.iter()
///         .map(|current| { /* element extraction */ })
///         .collect::<Result<Vec<_>, String>>()
///         .map(Some)
/// })()
/// ```
fn generate_from_segments(
    full_path: &str,
    segments: &[path::PathSegment],
    type_structure: &validation::TypeStructure,
) -> TokenStream2 {
    // Step 1: Check if outer type is Optional and unwrap it
    let (is_optional, inner_type) = match type_structure {
        validation::TypeStructure::Optional(inner) => (true, inner.as_ref()),
        other => (false, other),
    };

    // Step 2: Generate core extraction code
    let core = generate_from_segments_core(full_path, segments, inner_type, is_optional);

    // Step 3: Wrap Optional types in a closure to capture `return Ok(None)` from null handling.
    // Without closure, the return would escape to `from_value`. With closure, field gets `None`.
    if is_optional {
        quote! {
            (|| {
                #core.map(Some)
            })()
        }
    } else {
        core
    }
}

/// Core extraction logic that handles both list and non-list segments.
///
/// `null_returns_none`: If true, null values cause early return with `Ok(None)`.
/// This is set when the outermost type is Optional.
fn generate_from_segments_core(
    full_path: &str,
    segments: &[path::PathSegment],
    type_structure: &validation::TypeStructure,
    null_returns_none: bool,
) -> TokenStream2 {
    // Base case: no more segments, deserialize the current value
    let Some((segment, rest)) = segments.split_first() else {
        return quote! {
            serde_json::from_value(current.clone())
                .map_err(|e| format!("failed to deserialize '{}': {}", #full_path, e))
        };
    };

    let name = segment.field;
    // Use alias for JSON extraction if present, otherwise use field name
    let json_key = segment.json_key();

    // Generate null handling based on whether outer type is Optional
    let on_null = if null_returns_none {
        quote! { return Ok(None) }
    } else {
        quote! {
            return Err(format!("null value at '{}' in path '{}'", #name, #full_path))
        }
    };

    if segment
        .is_list
        .expect("is_list must be set after validation")
    {
        // For list segments, unwrap Vector to get element type
        let element_type = match type_structure {
            validation::TypeStructure::Vector(inner) => inner.as_ref(),
            _ => unreachable!("validated: list segment requires Vec type"),
        };

        // Each array element is processed independently with its own type structure.
        // Use generate_from_segments (not _core) to handle element-level Optional.
        let rest_code = generate_from_segments(full_path, rest, element_type);

        quote! {
            let field_value = current.get(#json_key)
                .ok_or_else(|| format!("missing field '{}' in path '{}'", #json_key, #full_path))?;
            if field_value.is_null() {
                #on_null
            }
            let array = field_value.as_array()
                .ok_or_else(|| format!("expected array at '{}' in path '{}'", #json_key, #full_path))?;
            array.iter()
                .map(|current| { #rest_code })
                .collect::<Result<Vec<_>, String>>()
        }
    } else {
        // For non-list segments, pass type unchanged to handle nested structures
        let rest_code =
            generate_from_segments_core(full_path, rest, type_structure, null_returns_none);

        quote! {
            let current = current.get(#json_key)
                .ok_or_else(|| format!("missing field '{}' in path '{}'", #json_key, #full_path))?;
            if current.is_null() {
                #on_null
            }
            #rest_code
        }
    }
}
