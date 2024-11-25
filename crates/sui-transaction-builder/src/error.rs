// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use base64ct::Error as Base64Error;
use sui_types::types::ObjectId;

#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    #[error("Conversion error due to input issue: {0}")]
    Input(String),
    #[error("Gas object should be an immutable or owned object")]
    WrongGasObject,
    #[error("Decoding error: {0}")]
    DecodingError(#[from] Base64Error),
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
    ObjectKindMissing(ObjectId),
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
