// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TxBuilderError {
    #[error("No version provided.")]
    NoVersion,
    #[error("No digest provided.")]
    NoDigest,
    #[error("No client provided for resolving the transaction.")]
    NoClient,
    #[error("No sender provided.")]
    NoSender,
    #[error("No gas objects provided.")]
    NoGasObjects,
    #[error("No gas budget provided.")]
    NoGasBudget,
    #[error("No gas price provided.")]
    NoGasPrice,
    #[error("No commands provided, only inputs")]
    NoCommandsOnlyInputs,
    #[error("No inputs provided, only commands")]
    NoInputsOnlyCommands,
    #[error("Object kind not set.")]
    ObjectKindNotSet,
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Initial shared version not found.")]
    InitialSharedVersionNotFound,
    #[error("Mutable property not found.")]
    MutablePropertyNotFound,
}
