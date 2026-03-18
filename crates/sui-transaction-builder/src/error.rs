// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_sdk_types::Address;

/// Errors that can occur when building or resolving a transaction.
#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    #[error("Conversion error due to input issue: {0}")]
    Input(String),
    #[error("Gas object should be an immutable or owned object")]
    WrongGasObject,
    #[error("Missing object id")]
    MissingObjectId,
    #[error("Missing version for object {0}")]
    MissingVersion(Address),
    #[error("Missing digest for object {0}")]
    MissingDigest(Address),
    #[error("Missing sender")]
    MissingSender,
    #[error("Missing gas objects")]
    MissingGasObjects,
    #[error("Missing gas budget")]
    MissingGasBudget,
    #[error("Missing gas price")]
    MissingGasPrice,
    #[error("Missing object kind for object {0}")]
    MissingObjectKind(Address),
    #[error("Unknown shared object mutability for object {0}")]
    SharedObjectMutability(Address),
    #[error("{0}")]
    #[cfg(feature = "intents")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "intents")))]
    SimulationFailure(Box<SimulationFailure>),
}

/// Rich error information from a failed transaction simulation.
#[derive(Debug, Clone)]
#[cfg(feature = "intents")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "intents")))]
pub struct SimulationFailure {
    /// The execution error returned by the simulate response.
    error: sui_rpc::proto::sui::rpc::v2::ExecutionError,
}

#[cfg(feature = "intents")]
impl SimulationFailure {
    pub(crate) fn new(error: sui_rpc::proto::sui::rpc::v2::ExecutionError) -> Self {
        Self { error }
    }

    /// Returns the underlying execution error.
    pub fn execution_error(&self) -> &sui_rpc::proto::sui::rpc::v2::ExecutionError {
        &self.error
    }

    /// Returns the human-readable error description, if available.
    pub fn description(&self) -> Option<&str> {
        self.error.description_opt()
    }

    /// Returns the command index that failed, if available.
    pub fn command(&self) -> Option<u64> {
        self.error.command_opt()
    }

    /// Returns the error kind, if available.
    pub fn kind(
        &self,
    ) -> Option<sui_rpc::proto::sui::rpc::v2::execution_error::ExecutionErrorKind> {
        self.error.kind.and_then(|k| {
            sui_rpc::proto::sui::rpc::v2::execution_error::ExecutionErrorKind::try_from(k).ok()
        })
    }

    /// Returns the Move abort details, if this was a `MoveAbort` error.
    pub fn move_abort(&self) -> Option<&sui_rpc::proto::sui::rpc::v2::MoveAbort> {
        self.error.abort_opt()
    }

    /// Returns the clever error details, if available.
    ///
    /// Clever errors provide structured information about Move abort
    /// codes, including the error constant name, type, and value.
    pub fn clever_error(&self) -> Option<&sui_rpc::proto::sui::rpc::v2::CleverError> {
        self.error.abort_opt()?.clever_error.as_ref()
    }
}

#[cfg(feature = "intents")]
impl std::fmt::Display for SimulationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "transaction simulation failed")?;

        if let Some(cmd) = self.command() {
            write!(f, " in command {cmd}")?;
        }

        if let Some(kind) = self.kind() {
            write!(f, " ({})", kind.as_str_name())?;
        }

        if let Some(desc) = self.description()
            && !desc.is_empty()
        {
            write!(f, ": {desc}")?;
        }

        if let Some(abort) = self.move_abort() {
            if let Some(loc) = &abort.location
                && let (Some(pkg), Some(module)) = (loc.package.as_deref(), loc.module.as_deref())
            {
                write!(f, " at {pkg}::{module}")?;
                if let Some(func) = loc.function_name.as_deref() {
                    write!(f, "::{func}")?;
                }
            }

            if let Some(clever) = &abort.clever_error
                && let Some(name) = clever.constant_name.as_deref()
            {
                write!(f, " [{name}")?;
                if let Some(sui_rpc::proto::sui::rpc::v2::clever_error::Value::Rendered(v)) =
                    &clever.value
                {
                    write!(f, " = {v}")?;
                }
                write!(f, "]")?;
            }
        }

        Ok(())
    }
}
