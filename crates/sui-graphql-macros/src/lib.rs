//! Procedural macros for sui-graphql with compile-time validation.
//!
//! This crate provides the `Response` derive macro that generates
//! deserialization code for extracting nested fields from GraphQL JSON responses.
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
//! // The macro generates code to extract nested fields from JSON:
//! // {
//! //   "object": {
//! //     "address": "0x123...",
//! //     "version": 42
//! //   }
//! // }
//! ```

extern crate proc_macro;

use darling::FromDeriveInput;
use darling::FromField;
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
#[proc_macro_derive(Response, attributes(field))]
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
    #[darling(supports(struct_named))] // Only supports structs with named fields
    struct ResponseInput {
        ident: syn::Ident,                           // Struct name
        generics: syn::Generics,                     // Generic parameters
        data: darling::ast::Data<(), ResponseField>, // Struct fields
    }

    #[derive(Debug, FromField)]
    #[darling(attributes(field))] // Parse #[field(...)] attributes
    struct ResponseField {
        ident: Option<syn::Ident>, // Field name
        path: String,              // The path = "..." value
    }

    let parsed = ResponseInput::from_derive_input(&input)?;

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

        let path = &field.path;

        if path.is_empty() {
            return Err(syn::Error::new_spanned(
                field_ident,
                "Field path cannot be empty",
            ));
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
/// For a field like:
/// ```ignore
/// #[field(path = "object.address")]
/// address: String,
/// ```
///
/// This generates:
/// ```ignore
/// let address = {
///     let mut current = &value;
///     current = current.get("object")
///         .ok_or_else(|| format!("missing field '{}' in path '{}'", "object", "object.address"))?;
///     current = current.get("address")
///         .ok_or_else(|| format!("missing field '{}' in path '{}'", "address", "object.address"))?;
///     serde_json::from_value(current.clone())
///         .map_err(|e| format!("failed to deserialize '{}': {}", "object.address", e))?
/// };
/// ```
fn generate_field_extraction(path: &str, field_ident: &syn::Ident) -> TokenStream2 {
    let segments: Vec<&str> = path.split('.').filter(|s| !s.is_empty()).collect();

    quote! {
        let #field_ident = {
            let mut current = &value;
            #(
                current = current.get(#segments)
                    .ok_or_else(|| format!("missing field '{}' in path '{}'", #segments, #path))?;
            )*
            serde_json::from_value(current.clone())
                .map_err(|e| format!("failed to deserialize '{}': {}", #path, e))?
        };
    }
}
