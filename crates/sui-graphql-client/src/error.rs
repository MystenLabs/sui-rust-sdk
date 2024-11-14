// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::num::ParseIntError;
use std::num::TryFromIntError;

use cynic::GraphQlError;

use sui_types::types::AddressParseError;
use sui_types::types::DigestParseError;
use sui_types::types::TypeParseError;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// General error type for the client. It is used to wrap all the possible errors that can occur.
#[derive(Debug)]
pub struct Error {
    inner: Box<InnerError>,
}

/// Error type for the client. It is split into multiple fields to allow for more granular error
/// handling. The `source` field is used to store the original error.
#[derive(Debug)]
struct InnerError {
    /// Error kind.
    kind: Kind,
    /// Errors returned by the GraphQL server.
    query_errors: Option<Vec<GraphQlError>>,
    /// The original error.
    source: Option<BoxError>,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Kind {
    Deserialization,
    Parse,
    Query,
    Other,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source.as_deref().map(|e| e as _)
    }
}

impl Error {
    // Public accessors

    /// Returns the kind of error.
    pub fn kind(&self) -> &Kind {
        &self.inner.kind
    }

    /// Original GraphQL query errors.
    pub fn graphql_errors(&self) -> Option<&[GraphQlError]> {
        self.inner.query_errors.as_deref()
    }

    // Private constructors

    /// Convert the given error into a generic error.
    pub(crate) fn from_error<E: Into<BoxError>>(kind: Kind, error: E) -> Self {
        Self {
            inner: Box::new(InnerError {
                kind,
                source: Some(error.into()),
                query_errors: None,
            }),
        }
    }

    /// Create a Query kind of error with the original graphql errors.
    pub(crate) fn graphql_error(errors: Vec<GraphQlError>) -> Self {
        Self {
            inner: Box::new(InnerError {
                kind: Kind::Query,
                source: None,
                query_errors: Some(errors),
            }),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(source) = &self.inner.source {
            writeln!(f, " {}", source)?;
        }
        Ok(())
    }
}

impl From<bcs::Error> for Error {
    fn from(error: bcs::Error) -> Self {
        Self::from_error(Kind::Deserialization, error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::from_error(Kind::Other, error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Self::from_error(Kind::Parse, error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::from_error(Kind::Parse, error)
    }
}

impl From<AddressParseError> for Error {
    fn from(error: AddressParseError) -> Self {
        Self::from_error(Kind::Parse, error)
    }
}

impl From<base64ct::Error> for Error {
    fn from(error: base64ct::Error) -> Self {
        Self::from_error(Kind::Parse, error)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(error: chrono::ParseError) -> Self {
        Self::from_error(Kind::Parse, error)
    }
}

impl From<DigestParseError> for Error {
    fn from(error: DigestParseError) -> Self {
        Self::from_error(Kind::Parse, error)
    }
}

impl From<TryFromIntError> for Error {
    fn from(error: TryFromIntError) -> Self {
        Self::from_error(Kind::Parse, error)
    }
}

impl From<TypeParseError> for Error {
    fn from(error: TypeParseError) -> Self {
        Self::from_error(Kind::Parse, error)
    }
}
