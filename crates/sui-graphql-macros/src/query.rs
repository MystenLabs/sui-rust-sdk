//! The `graphql_query!` function-style macro: compile-time validation and
//! canonical formatting of GraphQL queries and mutations against the embedded
//! Sui schema.
//!
//! Parsing, validation, and printing are delegated to Bluejay. On success, the
//! macro emits a `&'static str` literal containing the canonically formatted
//! query. On failure, every diagnostic becomes a `syn::Error` anchored at the
//! input literal's span.
//!
//! The macro is intentionally decoupled from any consumer crate: it produces
//! a plain string literal so callers can wrap it in their own type. The
//! `sui-graphql` crate ships a `macro_rules!` wrapper that nests this macro
//! inside a `ValidatedQuery` constructor, but this proc macro itself has no
//! dependency on or knowledge of that type.

use std::sync::LazyLock;

use bluejay_parser::Error as BluejayError;
use bluejay_parser::ast::Parse;
use bluejay_parser::ast::definition::DefinitionDocument;
use bluejay_parser::ast::definition::SchemaDefinition;
use bluejay_parser::ast::executable::ExecutableDocument;
use bluejay_parser::error::Location;
use bluejay_printer::executable::ExecutableDocumentPrinter;
use bluejay_validator::definition::BuiltinRulesValidator as SchemaValidator;
use bluejay_validator::executable::Cache;
use bluejay_validator::executable::document::BuiltinRulesValidator as ExecutableValidator;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::LitStr;

/// Diagnostic label for the embedded SDL. Bluejay embeds this string into
/// formatted schema errors; it does not open any file. The actual SDL bytes
/// come from [`crate::schema::SCHEMA_SDL`].
const SCHEMA_DIAGNOSTIC_LABEL: &str = "<sui schema>";

/// The parsed `SCHEMA_SDL` document.
///
/// Bluejay's schema representation borrows its parsed definition document, so
/// the two values are cached separately. The SDL itself is embedded with a
/// `'static` lifetime, and proc macros are loaded once per build, allowing both
/// caches to be reused for every `graphql_query!` call.
static SCHEMA_DOCUMENT: LazyLock<Result<DefinitionDocument<'static>, String>> =
    LazyLock::new(|| {
        DefinitionDocument::parse(crate::schema::SCHEMA_SDL)
            .map_err(|errors| format_schema_errors("parse", errors))
    });

/// The resolved and validated Sui schema used to type-check incoming queries.
static VALIDATED_SCHEMA: LazyLock<Result<SchemaDefinition<'static>, String>> =
    LazyLock::new(|| {
        let document = SCHEMA_DOCUMENT.as_ref().map_err(Clone::clone)?;
        let schema = SchemaDefinition::try_from(document)
            .map_err(|errors| format_schema_errors("resolve", errors))?;
        let errors = SchemaValidator::validate(&schema).collect::<Vec<_>>();

        if errors.is_empty() {
            Ok(schema)
        } else {
            Err(format_schema_errors("validate", errors))
        }
    });

fn format_schema_errors<E>(action: &str, errors: impl IntoIterator<Item = E>) -> String
where
    E: Into<BluejayError>,
{
    let formatted = BluejayError::format_errors(
        crate::schema::SCHEMA_SDL,
        Some(SCHEMA_DIAGNOSTIC_LABEL),
        errors,
    );

    format!("Failed to {action} Sui GraphQL schema:\n{formatted}")
}

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
        .map_err(|error| syn::Error::new(proc_macro2::Span::call_site(), error.clone()))?;

    let document = ExecutableDocument::parse(source.as_str()).map_err(|errors| {
        combine_query_errors(source.as_str(), errors).unwrap_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "GraphQL parsing failed with no diagnostics",
            )
        })
    })?;

    let cache = Cache::new(&document, schema);
    if let Some(error) = combine_query_errors(
        source.as_str(),
        ExecutableValidator::validate(&document, schema, &cache),
    ) {
        return Err(error);
    }

    let formatted = ExecutableDocumentPrinter::to_string(&document);
    Ok(quote!(#formatted))
}

fn combine_query_errors<E>(source: &str, errors: impl IntoIterator<Item = E>) -> Option<syn::Error>
where
    E: Into<BluejayError>,
{
    let mut combined: Option<syn::Error> = None;

    for error in errors {
        let error = error.into();
        let message = error.message().to_owned();
        let graph_errors = BluejayError::into_graphql_errors(source, [error]);
        let graph_error = graph_errors.first();

        // Parser errors use a generic top-level message and put the useful
        // detail in their primary annotation. Validation errors have a useful
        // top-level message already, so retain it.
        let message = match graph_error {
            Some(error) if message == "Parse error" => error.message.to_string(),
            _ => message,
        };

        let message = if let Some(Location { line, col }) =
            graph_error.and_then(|error| error.locations.first())
        {
            format!("GraphQL [line {}, col {}]: {message}", line, col)
        } else {
            format!("GraphQL: {message}")
        };

        let error = syn::Error::new(proc_macro2::Span::call_site(), message);
        if let Some(existing) = &mut combined {
            existing.combine(error);
        } else {
            combined = Some(error);
        }
    }

    combined
}
