//! The `graphql_query!` function-style macro: compile-time validation and
//! canonical formatting of GraphQL queries and mutations against the embedded
//! Sui schema.
//!
//! Validation is delegated to `apollo-compiler`, which performs full GraphQL
//! spec validation (selection set against schema, argument types, fragment
//! shapes, variable usage, etc.). On success, the macro emits a `&'static str`
//! literal containing the canonically formatted query. On failure, every
//! diagnostic becomes a `syn::Error` anchored at the input literal's span.
//!
//! The macro is intentionally decoupled from any consumer crate: it produces
//! a plain string literal so callers can wrap it in their own type. The
//! `sui-graphql` crate ships a `macro_rules!` wrapper that nests this macro
//! inside a `ValidatedQuery` constructor, but this proc macro itself has no
//! dependency on or knowledge of that type.

use std::sync::LazyLock;

use apollo_compiler::ExecutableDocument;
use apollo_compiler::Schema;
use apollo_compiler::validation::Valid;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::LitStr;

/// Diagnostic label for the embedded SDL. apollo-compiler embeds this string
/// into its error messages (e.g. `error at <sui schema>:42:5`); it does not
/// open any file. The actual SDL bytes come from [`crate::schema::SCHEMA_SDL`].
const SCHEMA_DIAGNOSTIC_LABEL: &str = "<sui schema>";

/// Diagnostic label for the user's query string passed to `graphql!`. Same
/// idea as [`SCHEMA_DIAGNOSTIC_LABEL`]: a string that appears in apollo's
/// error messages, not a file path.
const QUERY_DIAGNOSTIC_LABEL: &str = "<graphql! input>";

/// `SCHEMA_SDL` parsed and validated into a form that can type-check incoming
/// queries.
///
/// Cached for the lifetime of a single `cargo build`: parsing the full SDL is
/// non-trivial, and proc macros are loaded once per build, so we parse on
/// first use and reuse the result for every later `graphql!` call.
///
/// `Valid<Schema>` is apollo-compiler's marker that the schema itself is
/// internally consistent; `ExecutableDocument::parse_and_validate` requires
/// that proof. The `Result<_, String>` shape is what `LazyLock` needs (the
/// cell value must be `Sync` and shareable across reads); if SDL parsing ever
/// fails the cached message is reported at every call site.
static VALIDATED_SCHEMA: LazyLock<Result<Valid<Schema>, String>> = LazyLock::new(|| {
    Schema::parse_and_validate(crate::schema::SCHEMA_SDL, SCHEMA_DIAGNOSTIC_LABEL)
        .map_err(|e| format!("Failed to parse Sui GraphQL schema: {e}"))
});

pub fn expand(input: TokenStream) -> TokenStream {
    match expand_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            // Block-wrap with a `&str` tail; `compile_error!{...}` doesn't
            // parse on its own in expression position.
            let compile_error = err.to_compile_error();
            quote!({ #compile_error "" }).into()
        }
    }
}

fn expand_impl(input: TokenStream) -> Result<TokenStream2, syn::Error> {
    let lit: LitStr = syn::parse(input)?;
    let source = lit.value();

    let schema = VALIDATED_SCHEMA
        .as_ref()
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e.clone()))?;

    match ExecutableDocument::parse_and_validate(schema, source.as_str(), QUERY_DIAGNOSTIC_LABEL) {
        Ok(valid) => {
            let formatted = valid.to_string();
            Ok(quote!(#formatted))
        }
        Err(with_errors) => {
            let mut combined: Option<syn::Error> = None;
            for diag in with_errors.errors.iter() {
                let msg = match diag.line_column_range() {
                    Some(range) => format!(
                        "GraphQL [line {}, col {}]: {}",
                        range.start.line, range.start.column, diag.error
                    ),
                    None => format!("GraphQL: {}", diag.error),
                };
                let err = syn::Error::new(proc_macro2::Span::call_site(), msg);
                match &mut combined {
                    Some(existing) => existing.combine(err),
                    None => combined = Some(err),
                }
            }
            Err(combined.unwrap_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "GraphQL validation failed with no diagnostics",
                )
            }))
        }
    }
}
