// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use base64ct::Error as Base64Error;
use sui_types::types::ObjectId;

/// A boxed error type to be used for wrapping specific errors.
type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Conversion error type for the transaction builder.
#[derive(thiserror::Error, Debug, Clone)]
pub enum ConversionError {
    #[error("Conversion error due to input issue: {0}")]
    Input(String),
    #[error("Gas object should be an immutable or owned object")]
    WrongGasObjectKind,
    #[error("Decoding error: {0}")]
    DecodingError(Base64Error),
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum MissingDataError {
    /// Missing object id
    #[error("missing object id")]
    ObjectId,
    /// Missing version for the respective object
    #[error("missing version for object {0}")]
    Version(ObjectId),
    /// Missing digest for the respective object
    #[error("missing digest for object {0}")]
    Digest(ObjectId),
    /// Missing the sender for this transaction. A sender is always required when building a
    /// transaction.
    #[error("missing sender")]
    Sender,
    /// Missing the gas objects for this transaction
    #[error("missing gas objects")]
    GasObjects,
    /// Missing the gas budget for this transaction
    #[error("missing gas budget")]
    GasBudget,
    /// Missing the gas price for this transaction
    #[error("missing gas price")]
    GasPrice,
    /// Missing the object kind for this object
    #[error("missing object kind for object {0}")]
    ObjectKind(ObjectId),
    /// Missing the initial shared version for this object
    #[error("missing initial shared version for object {0}")]
    InitialSharedVersion(ObjectId),
    /// Missing the value data for this input that is of type Pure
    #[error("missing pure value")]
    PureValue,
    /// Cannot determine the shared object mutability for this object
    #[error("Unknown shared object mutability for object {0}")]
    SharedObjectMutability(ObjectId),
}

/// Literals are not supported yet.
#[derive(thiserror::Error, Debug, Clone)]
#[error("Unsupported literal")]
pub struct UnsupportedLiteralError;

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

    /// Convert the given error into a generic error.
    fn from_error<E: Into<BoxError>>(error: E) -> Self {
        let boxed_error = error.into();
        let result = Self::empty();

        if let Some(conversion_error) = boxed_error.downcast_ref::<ConversionError>() {
            result.with_conversion_error(conversion_error.clone())
        } else if let Some(missing_data_error) = boxed_error.downcast_ref::<MissingDataError>() {
            result.with_missing_data(missing_data_error.clone())
        } else {
            result.with_error(boxed_error)
        }
    }

    /// Set the source to the given error.
    fn with_error(mut self, error: BoxError) -> Self {
        self.inner.source.replace(error);
        self
    }

    /// Set the missing data variable to true and set the source to the given error.
    fn with_missing_data(mut self, error: MissingDataError) -> Self {
        self.inner.missing_data_error = true;
        self.inner.source.replace(Box::new(error));
        self
    }

    /// Set the conversion error to true and set the source to the given error.
    fn with_conversion_error(mut self, error: ConversionError) -> Self {
        self.inner.conversion_error = true;
        self.inner.source.replace(Box::new(error));
        self
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction builder error.")?;

        if self.inner.missing_data_error {
            write!(f, " Missing data: ")?;
        }

        if self.inner.conversion_error {
            write!(f, " Conversion error: ")?;
        }

        if let Some(source) = &self.inner.source {
            write!(f, "{source}")?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source.as_deref().map(|e| e as _)
    }
}

impl From<ConversionError> for Error {
    fn from(value: ConversionError) -> Self {
        Self::from_error(value)
    }
}

impl From<UnsupportedLiteralError> for Error {
    fn from(value: UnsupportedLiteralError) -> Self {
        Self::from_error(value)
    }
}

impl From<MissingDataError> for Error {
    fn from(value: MissingDataError) -> Self {
        Self::from_error(value)
    }
}

impl From<Base64Error> for ConversionError {
    fn from(value: Base64Error) -> Self {
        ConversionError::DecodingError(value)
    }
}
