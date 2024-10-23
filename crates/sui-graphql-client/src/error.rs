// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use cynic::GraphQlError;
use sui_types::types::DigestParseError;
use sui_types::types::TypeParseError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ClientError {
    #[error("Invalid URL: {url}")]
    InvalidURL { url: String },
    #[error("GraphQL error: {message}")]
    GraphQLError { message: String },
    #[error("Error when serializing to bcs: {message}")]
    SerializeToBcsError { message: String },
    #[error("Decoding Base64 error: {message}")]
    DecodingBase64Error { message: String },
    #[error("Deserializing bcs error: {message}")]
    DeserializingBcsError { message: String },
    #[error("Request error: {error}")]
    ReqwestError { error: reqwest::Error },
    #[error("Empty response from server")]
    EmptyResponse,
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Custom parse error: {message}")]
    CustomParseError { message: String },
    #[error("Either digest or sequence number must be provided")]
    DigestOrSequenceError,
    #[error("Checkpoint parse error: {message}")]
    CheckpointParseError { message: String },
    #[error("DateTime parse error: {message}")]
    DateTimeParseError { message: String },
    #[error("FromStr error: {0}")]
    FromStrError(#[from] std::string::ParseError),
    #[error("Gas cost summary parse error: {message}")]
    GasCostSummaryParseError { message: String },
    #[error("Transaction parse error: {message}")]
    TransactionParseError { message: String },
    #[error("Invalid type tag: {message}")]
    InvalidTypeTag { message: String },
    #[error("Dynamic field has no value field")]
    NoValueDynamicField,
}

impl From<GraphQlError> for ClientError {
    fn from(error: GraphQlError) -> Self {
        ClientError::GraphQLError {
            message: error.message,
        }
    }
}

impl From<Vec<GraphQlError>> for ClientError {
    fn from(errors: Vec<GraphQlError>) -> Self {
        ClientError::GraphQLError {
            message: errors
                .into_iter()
                .map(|e| e.message)
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}

impl From<DigestParseError> for ClientError {
    fn from(error: DigestParseError) -> Self {
        ClientError::CheckpointParseError {
            message: error.to_string(),
        }
    }
}

impl From<TypeParseError> for ClientError {
    fn from(error: TypeParseError) -> Self {
        ClientError::InvalidTypeTag {
            message: error.to_string(),
        }
    }
}
