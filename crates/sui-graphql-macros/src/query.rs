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

use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use bluejay_parser::Error as BluejayError;
use bluejay_parser::ast::Parse as _;
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
use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;

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

/// A piece of GraphQL source supplied inline or loaded from a file.
enum Source {
    Inline(LitStr),
    File(LitStr),
}

impl Parse for Source {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            input.parse().map(Self::File)
        } else {
            input.parse().map(Self::Inline)
        }
    }
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

fn expand_impl(input: TokenStream) -> Result<TokenStream2, syn::Error> {
    let sources = Punctuated::<Source, Token![,]>::parse_terminated.parse(input)?;
    if sources.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected at least one GraphQL source",
        ));
    }

    // Concatenate exactly like Rust's `concat!`: callers can split a document
    // wherever they like, and fragments can live in separate sources. The
    // complete document is still parsed, validated, and formatted as one unit.
    let mut source = String::new();
    let mut dependencies = Vec::new();
    for input in sources {
        match input {
            Source::Inline(literal) => source.push_str(&literal.value()),
            Source::File(literal) => {
                let path = source_relative(&literal)?;
                let contents = std::fs::read_to_string(&path).map_err(|error| {
                    syn::Error::new(
                        literal.span(),
                        format!(
                            "failed to read GraphQL source from '{}': {error}",
                            path.display()
                        ),
                    )
                })?;

                source.push_str(&contents);
                dependencies.push(literal);
            }
        }
    }

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
    // Keep an `include_str!` for every loaded source in the expansion so
    // rustc and Cargo rebuild when one of them changes.
    Ok(quote!({
        #(const _: &str = ::core::include_str!(#dependencies);)*
        #formatted
    }))
}

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

/// Interpret `path` as a path relative to the source file containing the macro invocation.
fn source_relative(literal: &LitStr) -> Result<PathBuf, syn::Error> {
    let value = literal.value();
    let path = Path::new(&value);
    if path.is_absolute() {
        return Ok(path.to_owned());
    }

    let source = literal.span().local_file();
    let Some(source_dir) = source.as_ref().and_then(|file| file.parent()) else {
        return Err(syn::Error::new(
            literal.span(),
            "cannot resolve GraphQL path relative to source file",
        ));
    };

    Ok(source_dir.join(path))
}
