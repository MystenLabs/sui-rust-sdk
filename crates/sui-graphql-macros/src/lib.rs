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
use darling::FromVariant;
use darling::util::SpannedValue;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;
use syn::parse_macro_input;

// ---------------------------------------------------------------------------
// Darling input structures — define the "schema" for macro input.
// Darling generates parsing code automatically, including error messages.
// ---------------------------------------------------------------------------

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(response), supports(struct_named, enum_newtype))]
struct ResponseInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<ResponseVariant, ResponseField>,
    #[darling(default)]
    schema: Option<String>,
    #[darling(default)]
    root_type: Option<SpannedValue<String>>,
}

/// A struct field (requires `#[field(path = "...")]`).
#[derive(Debug, FromField)]
#[darling(attributes(field))]
struct ResponseField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    path: SpannedValue<String>,
    #[darling(default)]
    skip_schema_validation: bool,
}

/// The inner type of a newtype enum variant.
#[derive(Debug, FromField)]
struct VariantInner {
    ty: syn::Type,
}

/// An enum variant mapping to a GraphQL union member.
#[derive(Debug, FromVariant)]
#[darling(attributes(response))]
struct ResponseVariant {
    ident: syn::Ident,
    fields: darling::ast::Fields<VariantInner>,
    /// The GraphQL type name this variant maps to (e.g., `#[response(on = "MoveValue")]`).
    /// Defaults to the variant ident if not specified.
    #[darling(default)]
    on: Option<SpannedValue<String>>,
}

/// Derive macro for GraphQL response types with nested field extraction.
///
/// Use `#[field(path = "...")]` to specify the JSON path to extract each field.
/// Paths are dot-separated (e.g., `"object.address"` extracts `json["object"]["address"]`).
///
/// # Root Type
///
/// By default, field paths are validated against the `Query` type. Use
/// `#[response(root_type = "...")]` to validate against a different type instead.
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
/// // Query response (default)
/// #[derive(Response)]
/// struct ChainInfo {
///     #[field(path = "chainIdentifier")]
///     chain_id: String,
///
///     #[field(path = "epoch.epochId")]
///     epoch_id: Option<u64>,
/// }
///
/// // Mutation response
/// #[derive(Response)]
/// #[response(root_type = "Mutation")]
/// struct ExecuteResult {
///     #[field(path = "executeTransaction.effects.effectsBcs")]
///     effects_bcs: Option<String>,
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

    // Determine root type: use specified root_type or default to "Query"
    let root_type = parsed
        .root_type
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Query");

    // Validate that the root type exists in the schema
    if !schema.has_type(root_type) {
        use std::fmt::Write;

        let type_names = schema.type_names();
        let suggestion = validation::find_similar(&type_names, root_type);

        let mut msg = format!("Type '{}' not found in GraphQL schema", root_type);
        if let Some(suggested) = suggestion {
            write!(msg, ". Did you mean '{}'?", suggested).unwrap();
        }

        // We only enter this block if root_type was explicitly specified (and invalid),
        // since "Query" (the default) always exists in a valid schema.
        let span = parsed.root_type.as_ref().unwrap().span();

        return Err(syn::Error::new(span, msg));
    }

    match parsed.data {
        darling::ast::Data::Struct(ref fields) => {
            generate_struct_impl(&parsed, &fields.fields, schema, root_type)
        }
        darling::ast::Data::Enum(ref variants) => {
            generate_enum_impl(&parsed, variants, schema, root_type)
        }
    }
}

/// Generate `from_value` and `Deserialize` for a struct.
fn generate_struct_impl(
    input: &ResponseInput,
    fields: &[ResponseField],
    schema: &schema::Schema,
    root_type: &str,
) -> Result<TokenStream2, syn::Error> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Generate extraction code for each field
    let mut field_extractions = Vec::new();
    let mut field_names = Vec::new();

    for field in fields {
        let field_ident = field
            .ident
            .as_ref()
            .expect("darling ensures named fields only");

        let spanned_path = &field.path;
        let parsed_path = path::ParsedPath::parse(spanned_path.as_str())
            .map_err(|e| syn::Error::new(spanned_path.span(), e.to_string()))?;

        let terminal_type = if !field.skip_schema_validation {
            Some(validation::validate_path_against_schema(
                schema,
                root_type,
                &parsed_path,
                spanned_path.span(),
            )?)
        } else {
            None
        };

        // Skip Vec excess check when schema validation is skipped (user takes full
        // responsibility) or when the terminal type is an object-like scalar (e.g., JSON)
        // whose value can be an array.
        let skip_vec_excess_check = field.skip_schema_validation
            || terminal_type.is_some_and(validation::is_object_like_scalar);
        validation::validate_type_matches_path(&parsed_path, &field.ty, skip_vec_excess_check)?;

        // Generate extraction code using the same parsed path
        let type_structure = validation::analyze_type(&field.ty);
        let extraction = generate_field_extraction(&parsed_path, &type_structure, field_ident);
        field_extractions.push(extraction);
        field_names.push(field_ident);
    }

    // Generate both `from_value` and `Deserialize` impl:
    //
    // - `from_value`: Core extraction logic, parses from serde_json::Value
    // - `Deserialize`: Allows direct use with serde (e.g., `serde_json::from_str::<MyStruct>(...)`)
    //   and with the GraphQL client's `query::<T>()` which requires `T: DeserializeOwned`
    Ok(quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn from_value(value: serde_json::Value) -> Result<Self, String> {
                #(#field_extractions)*

                Ok(Self {
                    #(#field_names),*
                })
            }
        }

        // TODO: Implement efficient deserialization that only extracts the fields we need.
        impl<'de> serde::Deserialize<'de> for #ident #ty_generics #where_clause {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = serde_json::Value::deserialize(deserializer)?;
                Self::from_value(value).map_err(serde::de::Error::custom)
            }
        }
    })
}

/// Generate `from_value` and `Deserialize` for an enum (GraphQL union).
///
/// Each variant wraps a type that implements `from_value`. Dispatches on `__typename`.
fn generate_enum_impl(
    input: &ResponseInput,
    variants: &[ResponseVariant],
    schema: &schema::Schema,
    root_type: &str,
) -> Result<TokenStream2, syn::Error> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let root_type_span = input
        .root_type
        .as_ref()
        .map(|s| s.span())
        .unwrap_or_else(|| ident.span());

    if !schema.is_union(root_type) {
        return Err(syn::Error::new(
            root_type_span,
            format!(
                "'{}' is not a union type. \
                 Enum Response requires root_type to be a GraphQL union",
                root_type
            ),
        ));
    }

    let mut match_arms = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;

        // Resolve the GraphQL typename: explicit `on` or variant ident
        let graphql_typename = variant
            .on
            .as_ref()
            .map(|s| s.as_str().to_string())
            .unwrap_or_else(|| variant_ident.to_string());

        let span = variant
            .on
            .as_ref()
            .map(|s| s.span())
            .unwrap_or_else(|| variant_ident.span());

        if let Err(mut err) =
            validation::validate_union_member(schema, root_type, &graphql_typename, span)
        {
            if variant.on.is_none() {
                err.combine(syn::Error::new(
                    span,
                    "hint: use #[response(on = \"...\")] to specify a GraphQL type name different from the variant name",
                ));
            }
            return Err(err);
        }

        // Newtype variant: delegate to inner type's from_value
        let inner_ty = &variant.fields.fields[0].ty;
        match_arms.push(quote! {
            #graphql_typename => {
                Ok(Self::#variant_ident(
                    <#inner_ty>::from_value(value)?
                ))
            }
        });
    }

    let root_type_str = root_type;
    let enum_name_str = ident.to_string();

    Ok(quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn from_value(value: serde_json::Value) -> Result<Self, String> {
                let typename = value.get("__typename")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| format!(
                        "union '{}' requires '__typename' in the response to distinguish variants. \
                         Make sure your query requests '__typename' on this field ({})",
                        #root_type_str, #enum_name_str
                    ))?;

                match typename {
                    #(#match_arms)*
                    other => Err(format!(
                        "unknown __typename '{}' for union '{}' ({})",
                        other, #root_type_str, #enum_name_str
                    )),
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for #ident #ty_generics #where_clause {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = serde_json::Value::deserialize(deserializer)?;
                Self::from_value(value).map_err(serde::de::Error::custom)
            }
        }
    })
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
/// ## Example: `"data.nodes[].edges[].id"` with `Option<Vec<Vec<String>>>`
///
/// Each `[]` in the path corresponds to one `Vec<_>` wrapper in the type.
///
/// For `Option<_>` types, null at the outer level returns `Ok(None)`. This is achieved
/// by wrapping the extraction in a closure to capture early returns. However, once
/// inside an array iteration, the element type (`Vec<String>`) is not Optional, so
/// null values there return errors instead.
///
/// ```ignore
/// (|| {
///     // "data" (non-list) - missing/null returns None (outer Optional)
///     let current = current.get("data").unwrap_or(&serde_json::Value::Null);
///     if current.is_null() { return Ok(None); }
///
///     // "nodes[]" (list) - missing/null returns None (outer Optional)
///     let field_value = current.get("nodes").unwrap_or(&serde_json::Value::Null);
///     if field_value.is_null() { return Ok(None); }
///     let array = field_value.as_array().ok_or_else(|| "expected array")?;
///     array.iter().map(|current| {
///         // Element type: Vec<String> (not Optional, so null = error)
///
///         // "edges[]" (list) - missing/null returns Err
///         let field_value = current.get("edges").unwrap_or(&serde_json::Value::Null);
///         if field_value.is_null() { return Err("null at 'edges'"); }
///         let array = field_value.as_array().ok_or_else(|| "expected array")?;
///         array.iter().map(|current| {
///             // Element type: String (not Optional, so null = error)
///
///             // "id" (scalar) - missing/null returns Err
///             let current = current.get("id").unwrap_or(&serde_json::Value::Null);
///             if current.is_null() { return Err("null at 'id'"); }
///             serde_json::from_value(current.clone())
///         }).collect::<Result<Vec<_>, _>>()
///     }).collect::<Result<Vec<_>, _>>()
///     .map(Some)  // Wrap in Some for Option
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

    if segment.is_list {
        // For list segments, unwrap Vector to get element type
        let element_type = match type_structure {
            validation::TypeStructure::Vector(inner) => inner.as_ref(),
            _ => unreachable!("validated: list segment requires Vec type"),
        };

        // Each array element is processed independently with its own type structure.
        // Use generate_from_segments (not _core) to handle element-level Optional.
        let rest_code = generate_from_segments(full_path, rest, element_type);

        quote! {
            // Treat missing fields as null (allows Option<T> to deserialize as None)
            let field_value = current.get(#json_key).unwrap_or(&serde_json::Value::Null);
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
            // Treat missing fields as null (allows Option<T> to deserialize as None)
            let current = current.get(#json_key).unwrap_or(&serde_json::Value::Null);
            if current.is_null() {
                #on_null
            }
            #rest_code
        }
    }
}
