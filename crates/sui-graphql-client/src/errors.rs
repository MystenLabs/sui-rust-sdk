// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use cynic::GraphQlError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ClientError {
    #[error("Invalid URL: {url}")]
    InvalidURL { url: String },
    #[error("GraphQL error: {message}")]
    GraphQLError { message: String },
    #[error("Decoding Base64 error: {message}")]
    DecodingBase64Error { message: String },
    #[error("Decoding BCS error: {message}")]
    DecodingBCSError { message: String },
    #[error("Request error: {error}")]
    ReqwestError { error: reqwest::Error },
    #[error("Empty response from server")]
    EmptyResponse,
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Custom parse error: {message}")]
    CustomParseError { message: String },
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
                .join(", "),
        }
    }
}
