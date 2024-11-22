// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use base64ct::Error as Base64Error;
use sui_types::types::ObjectId;

/// A boxed error type to be used for wrapping specific errors.
type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum Kind {
    #[error("Conversion error due to input issue: {0}")]
    Input(String),
    #[error("Gas object should be an immutable or owned object")]
    WrongGasObjectKind,
    #[error("Decoding error: {0}")]
    DecodingError(Base64Error),
    /// Missing object id
    #[error("Missing object id")]
    ObjectId,
    /// Missing version for the respective object
    #[error("Missing version for object {0}")]
    Version(ObjectId),
    /// Missing digest for the respective object
    #[error("Missing digest for object {0}")]
    Digest(ObjectId),
    /// Missing the sender for this transaction. A sender is always required when building a
    /// transaction.
    #[error("Missing sender")]
    Sender,
    /// Missing the gas objects for this transaction
    #[error("Missing gas objects")]
    GasObjects,
    /// Missing the gas budget for this transaction
    #[error("Missing gas budget")]
    GasBudget,
    /// Missing the gas price for this transaction
    #[error("Missing gas price")]
    GasPrice,
    /// Missing the object kind for this object
    #[error("Missing object kind for object {0}")]
    ObjectKind(ObjectId),
    /// Missing the initial shared version for this object
    #[error("Missing initial shared version for object {0}")]
    InitialSharedVersion(ObjectId),
    /// Missing the value data for this input that is of type Pure
    #[error("Missing pure value")]
    PureValue,
    /// Cannot determine the shared object mutability for this object
    #[error("Unknown shared object mutability for object {0}")]
    SharedObjectMutability(ObjectId),
    /// Literals are not yet supported.
    #[error("Unsupported literal")]
    UnsupportedLiteral,
}

/// General error type for the transaction builder.
#[derive(Debug)]
pub struct Error {
    inner: Box<BuilderError>,
}

/// Specific error type for the transaction builder.
#[derive(Debug)]
pub struct BuilderError {
    /// Indicates that the error is due to missing data.
    missing_data_error: bool,
    /// Indicates that the error is due to a conversion error
    conversion_error: bool,
    /// The original source of the error.
    source: Option<BoxError>,
}

impl Error {
    /// Create an empty error with no source.
    fn empty() -> Self {
        Self {
            inner: Box::new(BuilderError {
                missing_data_error: false,
                conversion_error: false,
                source: None,
            }),
        }
    }

    /// Check if the error is due to missing data.
    pub fn is_missing_data_error(&self) -> bool {
        self.inner.missing_data_error
    }

    /// Check if the error is due to a conversion error.
    pub fn is_conversion_error(&self) -> bool {
        self.inner.conversion_error
    }

    /// Convert the given error into a generic error.
    fn from_error<E: Into<BoxError>>(error: E) -> Self {
        let boxed_error = error.into();
        let result = Self::empty();
        result.with_error(boxed_error)
    }

    /// Set the source to the given error.
    fn with_error(mut self, error: BoxError) -> Self {
        self.inner.source.replace(error);
        self
    }

    /// Set the missing data variable to true and set the source to the given error.
    pub(crate) fn with_missing_data(error: Kind) -> Self {
        Self {
            inner: Box::new(BuilderError {
                missing_data_error: true,
                conversion_error: false,
                source: Some(Box::new(error)),
            }),
        }
    }

    /// Set the conversion error to true and set the source to the given error.
    pub(crate) fn with_conversion_error(error: Kind) -> Self {
        Self {
            inner: Box::new(BuilderError {
                missing_data_error: false,
                conversion_error: true,
                source: Some(Box::new(error)),
            }),
        }
    }
}

impl From<Kind> for Error {
    fn from(value: Kind) -> Self {
        Self::from_error(value)
    }
}

impl From<Base64Error> for Kind {
    fn from(value: Base64Error) -> Self {
        Kind::DecodingError(value)
    }
}
