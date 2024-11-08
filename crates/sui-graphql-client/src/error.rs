// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::num::ParseIntError;
use std::num::TryFromIntError;

use cynic::GraphQlError;

use sui_types::types::AddressParseError;
use sui_types::types::DigestParseError;
use sui_types::types::TypeParseError;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// General error type for the client. It is used to wrap all the possible errors that can occur.
#[derive(Debug)]
pub struct Error {
    inner: Box<ClientError>,
}

/// Error type for the client. It is split into multiple fields to allow for more granular error
/// handling. The `source` field is used to store the original error.
#[derive(Debug)]
pub struct ClientError {
    /// Error when the GraphQL server returns an error.
    pub query_response_error: bool,
    /// Error when deserialization a value (mostly bcs or base64).
    pub deserialize_error: bool,
    /// Error when parsing a value.
    pub parse_error: bool,
    /// Graphql server returned an empty response, though it was expected to return a value.
    pub empty_response_error: bool,
    /// Error when converting from a type to another.
    pub conversion_error: bool,
    /// The original error. Use downcasting to get the original error based on the error type from
    /// above.
    pub source: Option<BoxError>,
}

/// An empty response with no data from the GraphQL server. This is used to signal that the API
/// expected some data, but the response was empty.
#[derive(Debug, Clone)]
pub struct EmptyResponse;

/// Error when deserialization a value (mostly bcs or base64).
#[derive(Debug, Clone)]
pub struct DeserializeError(pub String);

/// Error when parsing a value.
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Error when parsing a URL.
    UrlParseError(String),
    /// Error when parsing a BigInt into integer.
    BigIntParsingError,
}

/// Error when converting from a type to another.
#[derive(Debug, Clone)]
pub struct ConversionError(pub String);

/// Error when the GraphQL server returns an error.
#[derive(Debug, Clone)]
pub struct QueryResponseError(pub Vec<GraphQlError>);

#[derive(Debug, Clone)]
pub struct ConflictingArguments(pub String);

impl std::error::Error for ConflictingArguments {}

impl std::fmt::Display for ConflictingArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "conflicting arguments: {}", self.0)
    }
}

impl std::error::Error for ConversionError {}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, " {}", self.0)
    }
}

impl std::error::Error for DeserializeError {}

impl std::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, " {}", self.0)
    }
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UrlParseError(e) => writeln!(f, "{e}"),
            Self::BigIntParsingError => {
                writeln!(f, " cannot parse GraphQL BigInt string into integer")
            }
        }
    }
}

impl std::error::Error for QueryResponseError {}

impl std::fmt::Display for QueryResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for e in self.0.iter() {
            writeln!(f, "  {e}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source.as_deref().map(|e| e as _)
    }
}

impl From<EmptyResponse> for Error {
    fn from(_: EmptyResponse) -> Self {
        Self::empty().with_empty_response_error()
    }
}

impl From<bcs::Error> for Error {
    fn from(error: bcs::Error) -> Self {
        Self::empty().with_deserialize_error(DeserializeError(error.to_string()))
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::from_error(error)
    }
}

impl From<QueryResponseError> for Error {
    fn from(error: QueryResponseError) -> Self {
        Self::empty().with_query_response_error(error)
    }
}

impl From<DeserializeError> for Error {
    fn from(error: DeserializeError) -> Self {
        Self::empty().with_deserialize_error(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Self::empty().with_parse_error(ParseError::UrlParseError(error.to_string()))
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self::empty().with_parse_error(ParseError::BigIntParsingError)
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Self::empty().with_parse_error(error)
    }
}

impl From<AddressParseError> for Error {
    fn from(error: AddressParseError) -> Self {
        Self::empty().with_error(Box::new(error))
    }
}

impl From<base64ct::Error> for Error {
    fn from(error: base64ct::Error) -> Self {
        Self::empty().with_deserialize_error(DeserializeError(error.to_string()))
    }
}

impl From<chrono::ParseError> for Error {
    fn from(error: chrono::ParseError) -> Self {
        Self::empty().with_error(Box::new(error))
    }
}

impl From<DigestParseError> for Error {
    fn from(error: DigestParseError) -> Self {
        Self::empty().with_error(Box::new(error))
    }
}

impl From<TryFromIntError> for Error {
    fn from(error: TryFromIntError) -> Self {
        Self::empty().with_error(Box::new(error))
    }
}

impl From<ConversionError> for Error {
    fn from(error: ConversionError) -> Self {
        Self::empty().with_conversion_error(error)
    }
}

impl From<TypeParseError> for Error {
    fn from(error: TypeParseError) -> Self {
        Self::empty().with_error(Box::new(error))
    }
}

impl From<ConflictingArguments> for Error {
    fn from(error: ConflictingArguments) -> Self {
        Self::empty().with_error(Box::new(error))
    }
}

impl Error {
    /// Create an empty error with no source.
    fn empty() -> Self {
        Self {
            inner: Box::new(ClientError {
                query_response_error: false,
                deserialize_error: false,
                parse_error: false,
                empty_response_error: false,
                conversion_error: false,
                source: None,
            }),
        }
    }

    /// Convert the given error into a generic error.
    fn from_error<E: Into<BoxError>>(error: E) -> Self {
        let boxed_error = error.into();
        let result = Self::empty();

        if let Some(query_response_error) = boxed_error.downcast_ref::<QueryResponseError>() {
            result.with_query_response_error(query_response_error.clone())
        } else if let Some(deserialize_error) = boxed_error.downcast_ref::<DeserializeError>() {
            result.with_deserialize_error(deserialize_error.clone())
        } else if let Some(parse_error) = boxed_error.downcast_ref::<ParseError>() {
            result.with_parse_error(parse_error.clone())
        } else if let Some(conversion_error) = boxed_error.downcast_ref::<ConversionError>() {
            result.with_conversion_error(conversion_error.clone())
        } else {
            result.with_error(boxed_error)
        }
    }

    /// Set the deserialize error to true and set the source to the given error.
    fn with_deserialize_error(mut self, error: DeserializeError) -> Self {
        self.inner.deserialize_error = true;
        self.inner.source.replace(Box::new(error));
        self
    }

    /// Set the empty response error to true.
    fn with_empty_response_error(mut self) -> Self {
        self.inner.empty_response_error = true;
        self
    }

    /// Set the parse error to true and set the source to the given error.
    fn with_parse_error(mut self, error: ParseError) -> Self {
        self.inner.parse_error = true;
        self.inner.source.replace(Box::new(error));
        self
    }

    /// Set the query response error to true and set the source to the given error.
    fn with_query_response_error(mut self, error: QueryResponseError) -> Self {
        self.inner.query_response_error = true;
        self.inner.source.replace(Box::new(error));
        self
    }

    /// Set the conversion error to true and set the source to the given error.
    fn with_conversion_error(mut self, error: ConversionError) -> Self {
        self.inner.conversion_error = true;
        self.inner.source.replace(Box::new(error));
        self
    }

    /// Set the source to the given error.
    fn with_error(mut self, error: BoxError) -> Self {
        self.inner.source.replace(error);
        self
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.inner.query_response_error {
            writeln!(f, "Query response error:")?;
        }
        if self.inner.deserialize_error {
            writeln!(f, "Deserialize error:")?;
        }
        if self.inner.empty_response_error {
            writeln!(f, "Empty response from GraphQL server")?;
            return Ok(());
        }
        if self.inner.conversion_error {
            writeln!(f, "Conversion error:")?;
        }
        if self.inner.parse_error {
            writeln!(f, "Parse error:")?;
        }
        if let Some(source) = &self.inner.source {
            writeln!(f, " {}", source)?;
        }
        Ok(())
    }
}
