use super::{Digest, Identifier, ObjectId};

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum ExecutionStatus {
    Success,
    /// Gas used in the failed case, and the error.
    Failure {
        /// The error
        error: ExecutionError,
        /// Which command the error occurred
        #[cfg_attr(
            test,
            proptest(
                strategy = "proptest::strategy::Strategy::prop_map(proptest::arbitrary::any::<Option<u16>>(), |opt| opt.map(Into::into))"
            )
        )]
        command: Option<u64>,
    },
}

/// Type parameters are encoded as indices. This index can also be used to lookup the kind of a
/// type parameter in the `FunctionHandle` and `StructHandle`.
pub type TypeParameterIndex = u16;

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "schemars",
    derive(schemars::JsonSchema),
    schemars(tag = "error", rename_all = "snake_case")
)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum ExecutionError {
    //
    // General transaction errors
    //
    /// Insufficient Gas
    InsufficientGas,
    /// Invalid Gas Object.
    InvalidGasObject,
    /// Invariant Violation
    InvariantViolation,
    /// Attempted to used feature that is not supported yet
    FeatureNotYetSupported,
    /// Move object is larger than the maximum allowed size
    ObjectTooBig {
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        object_size: u64,
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        max_object_size: u64,
    },
    /// Package is larger than the maximum allowed size
    PackageTooBig {
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        object_size: u64,
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        max_object_size: u64,
    },
    /// Circular Object Ownership
    CircularObjectOwnership { object: ObjectId },

    //
    // Coin errors
    //
    /// Insufficient coin balance for requested operation
    InsufficientCoinBalance,
    /// Coin balance overflowed an u64
    CoinBalanceOverflow,

    //
    // Publish/Upgrade errors
    //
    /// Publish Error, Non-zero Address.
    /// The modules in the package must have their self-addresses set to zero.
    PublishErrorNonZeroAddress,

    /// Sui Move Bytecode Verification Error.
    SuiMoveVerificationError,

    //
    // MoveVm Errors
    //
    /// Error from a non-abort instruction.
    /// Possible causes:
    ///     Arithmetic error, stack overflow, max value depth, etc."
    MovePrimitiveRuntimeError { location: Option<MoveLocation> },
    /// Move runtime abort
    MoveAbort {
        location: MoveLocation,
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        code: u64,
    },
    /// Bytecode verification error.
    VmVerificationOrDeserializationError,
    /// MoveVm invariant violation
    VmInvariantViolation,

    //
    // Programmable Transaction Errors
    //
    /// Function not found
    FunctionNotFound,
    /// Arity mismatch for Move function.
    /// The number of arguments does not match the number of parameters
    ArityMismatch,
    /// Type arity mismatch for Move function.
    /// Mismatch between the number of actual versus expected type arguments.
    TypeArityMismatch,
    /// Non Entry Function Invoked. Move Call must start with an entry function.
    NonEntryFunctionInvoked,
    /// Invalid command argument
    CommandArgumentError {
        argument: u16,
        #[cfg_attr(feature = "schemars", schemars(flatten))]
        kind: CommandArgumentError,
    },
    /// Type argument error
    TypeArgumentError {
        type_argument: TypeParameterIndex,
        kind: TypeArgumentError,
    },
    /// Unused result without the drop ability.
    UnusedValueWithoutDrop { result: u16, subresult: u16 },
    /// Invalid public Move function signature.
    /// Unsupported return type for return value
    InvalidPublicFunctionReturnType { index: u16 },
    /// Invalid Transfer Object, object does not have public transfer.
    InvalidTransferObject,

    //
    // Post-execution errors
    //
    /// Effects from the transaction are too large
    EffectsTooLarge {
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        current_size: u64,
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        max_size: u64,
    },

    /// Publish or Upgrade is missing dependency
    PublishUpgradeMissingDependency,

    /// Publish or Upgrade dependency downgrade.
    ///
    /// Indirect (transitive) dependency of published or upgraded package has been assigned an
    /// on-chain version that is less than the version required by one of the package's
    /// transitive dependencies.
    PublishUpgradeDependencyDowngrade,

    /// Invalid package upgrade
    PackageUpgradeError(PackageUpgradeError),

    /// Indicates the transaction tried to write objects too large to storage
    WrittenObjectsTooLarge {
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        object_size: u64,
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        max_object_size: u64,
    },

    /// Certificate is on the deny list
    CertificateDenied,

    /// Sui Move Bytecode verification timed out.
    SuiMoveVerificationTimedout,

    /// The requested shared object operation is not allowed
    SharedObjectOperationNotAllowed,

    /// Requested shared object has been deleted
    InputObjectDeleted,
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct MoveLocation {
    pub package: ObjectId,
    pub module: Identifier,
    pub function: u16,
    /// Index into the code stream for a jump. The offset is relative to the beginning of
    /// the instruction stream.
    pub instruction: u16,
    pub function_name: Option<Identifier>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "schemars",
    derive(schemars::JsonSchema),
    schemars(tag = "kind", rename_all = "snake_case")
)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum CommandArgumentError {
    /// The type of the value does not match the expected type
    TypeMismatch,
    /// The argument cannot be deserialized into a value of the specified type
    InvalidBcsBytes,
    /// The argument cannot be instantiated from raw bytes
    InvalidUsageOfPureArgument,
    /// Invalid argument to private entry function.
    /// Private entry functions cannot take arguments from other Move functions.
    InvalidArgumentToPrivateEntryFunction,
    /// Out of bounds access to input or results
    IndexOutOfBounds { index: u16 },
    /// Out of bounds access to subresult
    SecondaryIndexOutOfBounds { result: u16, subresult: u16 },
    /// Invalid usage of result.
    /// Expected a single result but found either no return value or multiple.
    InvalidResultArity { result: u16 },
    /// Invalid usage of Gas coin.
    /// The Gas coin can only be used by-value with a TransferObjects command.
    InvalidGasCoinUsage,
    /// Invalid usage of move value.
    //     Mutably borrowed values require unique usage.
    //     Immutably borrowed values cannot be taken or borrowed mutably.
    //     Taken values cannot be used again.
    InvalidValueUsage,
    /// Immutable objects cannot be passed by-value.
    InvalidObjectByValue,
    /// Immutable objects cannot be passed by mutable reference, &mut.
    InvalidObjectByMutRef,
    /// Shared object operations such a wrapping, freezing, or converting to owned are not
    /// allowed.
    SharedObjectOperationNotAllowed,
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "schemars",
    derive(schemars::JsonSchema),
    schemars(tag = "kind", rename_all = "snake_case")
)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum PackageUpgradeError {
    /// Unable to fetch package
    UnableToFetchPackage { package_id: ObjectId },
    /// Object is not a package
    NotAPackage { object_id: ObjectId },
    /// Package upgrade is incompatible with previous version
    IncompatibleUpgrade,
    /// Digest in upgrade ticket and computed digest differ
    DigestDoesNotMatch { digest: Digest },
    /// Upgrade policy is not valid
    UnknownUpgradePolicy { policy: u8 },
    /// PackageId does not matach PackageId in upgrade ticket
    PackageIdDoesNotMatch {
        package_id: ObjectId,
        ticket_id: ObjectId,
    },
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(
    feature = "schemars",
    derive(schemars::JsonSchema),
    schemars(rename_all = "snake_case")
)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum TypeArgumentError {
    /// A type was not found in the module specified
    TypeNotFound,
    /// A type provided did not match the specified constraint
    ConstraintNotSatisfied,
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use super::*;

    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(rename = "ExecutionStatus")]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    struct ReadableExecutionStatus {
        success: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<FailureStatus>,
    }

    #[cfg(feature = "schemars")]
    impl schemars::JsonSchema for ExecutionStatus {
        fn schema_name() -> String {
            ReadableExecutionStatus::schema_name()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            ReadableExecutionStatus::json_schema(gen)
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    struct FailureStatus {
        #[serde(flatten)]
        error: ExecutionError,
        #[serde(skip_serializing_if = "Option::is_none")]
        command: Option<u16>,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum BinaryExecutionStatus {
        Success,
        Failure {
            error: ExecutionError,
            command: Option<u64>,
        },
    }

    impl Serialize for ExecutionStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self.clone() {
                    ExecutionStatus::Success => ReadableExecutionStatus {
                        success: true,
                        status: None,
                    },
                    ExecutionStatus::Failure { error, command } => ReadableExecutionStatus {
                        success: false,
                        status: Some(FailureStatus {
                            error,
                            command: command.map(|c| c as u16),
                        }),
                    },
                };
                readable.serialize(serializer)
            } else {
                let binary = match self.clone() {
                    ExecutionStatus::Success => BinaryExecutionStatus::Success,
                    ExecutionStatus::Failure { error, command } => {
                        BinaryExecutionStatus::Failure { error, command }
                    }
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for ExecutionStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let ReadableExecutionStatus { success, status } =
                    Deserialize::deserialize(deserializer)?;
                match (success, status) {
                    (true, None) => Ok(ExecutionStatus::Success),
                    (false, Some(FailureStatus { error, command })) => {
                        Ok(ExecutionStatus::Failure {
                            error,
                            command: command.map(Into::into),
                        })
                    }
                    // invalid cases
                    (true, Some(_)) | (false, None) => {
                        Err(serde::de::Error::custom("invalid execution status"))
                    }
                }
            } else {
                BinaryExecutionStatus::deserialize(deserializer).map(|readable| match readable {
                    BinaryExecutionStatus::Success => Self::Success,
                    BinaryExecutionStatus::Failure { error, command } => {
                        Self::Failure { error, command }
                    }
                })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "error", rename_all = "snake_case")]
    enum ReadableExecutionError {
        InsufficientGas,
        InvalidGasObject,
        InvariantViolation,
        FeatureNotYetSupported,
        ObjectTooBig {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            object_size: u64,
            #[serde(with = "crate::_serde::ReadableDisplay")]
            max_object_size: u64,
        },
        PackageTooBig {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            object_size: u64,
            #[serde(with = "crate::_serde::ReadableDisplay")]
            max_object_size: u64,
        },
        CircularObjectOwnership {
            object: ObjectId,
        },
        InsufficientCoinBalance,
        CoinBalanceOverflow,
        PublishErrorNonZeroAddress,
        SuiMoveVerificationError,
        MovePrimitiveRuntimeError {
            location: Option<MoveLocation>,
        },
        MoveAbort {
            location: MoveLocation,
            #[serde(with = "crate::_serde::ReadableDisplay")]
            code: u64,
        },
        VmVerificationOrDeserializationError,
        VmInvariantViolation,
        FunctionNotFound,
        ArityMismatch,
        TypeArityMismatch,
        NonEntryFunctionInvoked,
        CommandArgumentError {
            argument: u16,
            #[serde(flatten)]
            kind: CommandArgumentError,
        },
        TypeArgumentError {
            type_argument: TypeParameterIndex,
            kind: TypeArgumentError,
        },
        UnusedValueWithoutDrop {
            result: u16,
            subresult: u16,
        },
        InvalidPublicFunctionReturnType {
            index: u16,
        },
        InvalidTransferObject,
        EffectsTooLarge {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            current_size: u64,
            #[serde(with = "crate::_serde::ReadableDisplay")]
            max_size: u64,
        },
        PublishUpgradeMissingDependency,
        PublishUpgradeDependencyDowngrade,
        PackageUpgradeError(PackageUpgradeError),
        WrittenObjectsTooLarge {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            object_size: u64,
            #[serde(with = "crate::_serde::ReadableDisplay")]
            max_object_size: u64,
        },
        CertificateDenied,
        SuiMoveVerificationTimedout,
        SharedObjectOperationNotAllowed,
        InputObjectDeleted,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum BinaryExecutionError {
        InsufficientGas,
        InvalidGasObject,
        InvariantViolation,
        FeatureNotYetSupported,
        ObjectTooBig {
            object_size: u64,
            max_object_size: u64,
        },
        PackageTooBig {
            object_size: u64,
            max_object_size: u64,
        },
        CircularObjectOwnership {
            object: ObjectId,
        },
        InsufficientCoinBalance,
        CoinBalanceOverflow,
        PublishErrorNonZeroAddress,
        SuiMoveVerificationError,
        MovePrimitiveRuntimeError {
            location: Option<MoveLocation>,
        },
        MoveAbort {
            location: MoveLocation,
            code: u64,
        },
        VmVerificationOrDeserializationError,
        VmInvariantViolation,
        FunctionNotFound,
        ArityMismatch,
        TypeArityMismatch,
        NonEntryFunctionInvoked,
        CommandArgumentError {
            argument: u16,
            kind: CommandArgumentError,
        },
        TypeArgumentError {
            type_argument: TypeParameterIndex,
            kind: TypeArgumentError,
        },
        UnusedValueWithoutDrop {
            result: u16,
            subresult: u16,
        },
        InvalidPublicFunctionReturnType {
            index: u16,
        },
        InvalidTransferObject,
        EffectsTooLarge {
            current_size: u64,
            max_size: u64,
        },
        PublishUpgradeMissingDependency,
        PublishUpgradeDependencyDowngrade,
        PackageUpgradeError(PackageUpgradeError),
        WrittenObjectsTooLarge {
            object_size: u64,
            max_object_size: u64,
        },
        CertificateDenied,
        SuiMoveVerificationTimedout,
        SharedObjectOperationNotAllowed,
        InputObjectDeleted,
    }

    impl Serialize for ExecutionError {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self.clone() {
                    Self::InsufficientGas => ReadableExecutionError::InsufficientGas,
                    Self::InvalidGasObject => ReadableExecutionError::InvalidGasObject,
                    Self::InvariantViolation => ReadableExecutionError::InvariantViolation,
                    Self::FeatureNotYetSupported => ReadableExecutionError::FeatureNotYetSupported,
                    Self::ObjectTooBig {
                        object_size,
                        max_object_size,
                    } => ReadableExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    },
                    Self::PackageTooBig {
                        object_size,
                        max_object_size,
                    } => ReadableExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    },
                    Self::CircularObjectOwnership { object } => {
                        ReadableExecutionError::CircularObjectOwnership { object }
                    }
                    Self::InsufficientCoinBalance => {
                        ReadableExecutionError::InsufficientCoinBalance
                    }
                    Self::CoinBalanceOverflow => ReadableExecutionError::CoinBalanceOverflow,
                    Self::PublishErrorNonZeroAddress => {
                        ReadableExecutionError::PublishErrorNonZeroAddress
                    }
                    Self::SuiMoveVerificationError => {
                        ReadableExecutionError::SuiMoveVerificationError
                    }
                    Self::MovePrimitiveRuntimeError { location } => {
                        ReadableExecutionError::MovePrimitiveRuntimeError { location }
                    }
                    Self::MoveAbort { location, code } => {
                        ReadableExecutionError::MoveAbort { location, code }
                    }
                    Self::VmVerificationOrDeserializationError => {
                        ReadableExecutionError::VmVerificationOrDeserializationError
                    }
                    Self::VmInvariantViolation => ReadableExecutionError::VmInvariantViolation,
                    Self::FunctionNotFound => ReadableExecutionError::FunctionNotFound,
                    Self::ArityMismatch => ReadableExecutionError::ArityMismatch,
                    Self::TypeArityMismatch => ReadableExecutionError::TypeArityMismatch,
                    Self::NonEntryFunctionInvoked => {
                        ReadableExecutionError::NonEntryFunctionInvoked
                    }
                    Self::CommandArgumentError { argument, kind } => {
                        ReadableExecutionError::CommandArgumentError { argument, kind }
                    }
                    Self::TypeArgumentError {
                        type_argument,
                        kind,
                    } => ReadableExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    },
                    Self::UnusedValueWithoutDrop { result, subresult } => {
                        ReadableExecutionError::UnusedValueWithoutDrop { result, subresult }
                    }
                    Self::InvalidPublicFunctionReturnType { index } => {
                        ReadableExecutionError::InvalidPublicFunctionReturnType { index }
                    }
                    Self::InvalidTransferObject => ReadableExecutionError::InvalidTransferObject,
                    Self::EffectsTooLarge {
                        current_size,
                        max_size,
                    } => ReadableExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    },
                    Self::PublishUpgradeMissingDependency => {
                        ReadableExecutionError::PublishUpgradeMissingDependency
                    }
                    Self::PublishUpgradeDependencyDowngrade => {
                        ReadableExecutionError::PublishUpgradeDependencyDowngrade
                    }
                    Self::PackageUpgradeError(err) => {
                        ReadableExecutionError::PackageUpgradeError(err)
                    }
                    Self::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    } => ReadableExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    },
                    Self::CertificateDenied => ReadableExecutionError::CertificateDenied,
                    Self::SuiMoveVerificationTimedout => {
                        ReadableExecutionError::SuiMoveVerificationTimedout
                    }
                    Self::SharedObjectOperationNotAllowed => {
                        ReadableExecutionError::SharedObjectOperationNotAllowed
                    }
                    Self::InputObjectDeleted => ReadableExecutionError::InputObjectDeleted,
                };
                readable.serialize(serializer)
            } else {
                let binary = match self.clone() {
                    Self::InsufficientGas => BinaryExecutionError::InsufficientGas,
                    Self::InvalidGasObject => BinaryExecutionError::InvalidGasObject,
                    Self::InvariantViolation => BinaryExecutionError::InvariantViolation,
                    Self::FeatureNotYetSupported => BinaryExecutionError::FeatureNotYetSupported,
                    Self::ObjectTooBig {
                        object_size,
                        max_object_size,
                    } => BinaryExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    },
                    Self::PackageTooBig {
                        object_size,
                        max_object_size,
                    } => BinaryExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    },
                    Self::CircularObjectOwnership { object } => {
                        BinaryExecutionError::CircularObjectOwnership { object }
                    }
                    Self::InsufficientCoinBalance => BinaryExecutionError::InsufficientCoinBalance,
                    Self::CoinBalanceOverflow => BinaryExecutionError::CoinBalanceOverflow,
                    Self::PublishErrorNonZeroAddress => {
                        BinaryExecutionError::PublishErrorNonZeroAddress
                    }
                    Self::SuiMoveVerificationError => {
                        BinaryExecutionError::SuiMoveVerificationError
                    }
                    Self::MovePrimitiveRuntimeError { location } => {
                        BinaryExecutionError::MovePrimitiveRuntimeError { location }
                    }
                    Self::MoveAbort { location, code } => {
                        BinaryExecutionError::MoveAbort { location, code }
                    }
                    Self::VmVerificationOrDeserializationError => {
                        BinaryExecutionError::VmVerificationOrDeserializationError
                    }
                    Self::VmInvariantViolation => BinaryExecutionError::VmInvariantViolation,
                    Self::FunctionNotFound => BinaryExecutionError::FunctionNotFound,
                    Self::ArityMismatch => BinaryExecutionError::ArityMismatch,
                    Self::TypeArityMismatch => BinaryExecutionError::TypeArityMismatch,
                    Self::NonEntryFunctionInvoked => BinaryExecutionError::NonEntryFunctionInvoked,
                    Self::CommandArgumentError { argument, kind } => {
                        BinaryExecutionError::CommandArgumentError { argument, kind }
                    }
                    Self::TypeArgumentError {
                        type_argument,
                        kind,
                    } => BinaryExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    },
                    Self::UnusedValueWithoutDrop { result, subresult } => {
                        BinaryExecutionError::UnusedValueWithoutDrop { result, subresult }
                    }
                    Self::InvalidPublicFunctionReturnType { index } => {
                        BinaryExecutionError::InvalidPublicFunctionReturnType { index }
                    }
                    Self::InvalidTransferObject => BinaryExecutionError::InvalidTransferObject,
                    Self::EffectsTooLarge {
                        current_size,
                        max_size,
                    } => BinaryExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    },
                    Self::PublishUpgradeMissingDependency => {
                        BinaryExecutionError::PublishUpgradeMissingDependency
                    }
                    Self::PublishUpgradeDependencyDowngrade => {
                        BinaryExecutionError::PublishUpgradeDependencyDowngrade
                    }
                    Self::PackageUpgradeError(err) => {
                        BinaryExecutionError::PackageUpgradeError(err)
                    }
                    Self::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    } => BinaryExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    },
                    Self::CertificateDenied => BinaryExecutionError::CertificateDenied,
                    Self::SuiMoveVerificationTimedout => {
                        BinaryExecutionError::SuiMoveVerificationTimedout
                    }
                    Self::SharedObjectOperationNotAllowed => {
                        BinaryExecutionError::SharedObjectOperationNotAllowed
                    }
                    Self::InputObjectDeleted => BinaryExecutionError::InputObjectDeleted,
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for ExecutionError {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableExecutionError::deserialize(deserializer).map(|readable| match readable {
                    ReadableExecutionError::InsufficientGas => Self::InsufficientGas,
                    ReadableExecutionError::InvalidGasObject => Self::InvalidGasObject,
                    ReadableExecutionError::InvariantViolation => Self::InvariantViolation,
                    ReadableExecutionError::FeatureNotYetSupported => Self::FeatureNotYetSupported,
                    ReadableExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    } => Self::ObjectTooBig {
                        object_size,
                        max_object_size,
                    },
                    ReadableExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    } => Self::PackageTooBig {
                        object_size,
                        max_object_size,
                    },
                    ReadableExecutionError::CircularObjectOwnership { object } => {
                        Self::CircularObjectOwnership { object }
                    }
                    ReadableExecutionError::InsufficientCoinBalance => {
                        Self::InsufficientCoinBalance
                    }
                    ReadableExecutionError::CoinBalanceOverflow => Self::CoinBalanceOverflow,
                    ReadableExecutionError::PublishErrorNonZeroAddress => {
                        Self::PublishErrorNonZeroAddress
                    }
                    ReadableExecutionError::SuiMoveVerificationError => {
                        Self::SuiMoveVerificationError
                    }
                    ReadableExecutionError::MovePrimitiveRuntimeError { location } => {
                        Self::MovePrimitiveRuntimeError { location }
                    }
                    ReadableExecutionError::MoveAbort { location, code } => {
                        Self::MoveAbort { location, code }
                    }
                    ReadableExecutionError::VmVerificationOrDeserializationError => {
                        Self::VmVerificationOrDeserializationError
                    }
                    ReadableExecutionError::VmInvariantViolation => Self::VmInvariantViolation,
                    ReadableExecutionError::FunctionNotFound => Self::FunctionNotFound,
                    ReadableExecutionError::ArityMismatch => Self::ArityMismatch,
                    ReadableExecutionError::TypeArityMismatch => Self::TypeArityMismatch,
                    ReadableExecutionError::NonEntryFunctionInvoked => {
                        Self::NonEntryFunctionInvoked
                    }
                    ReadableExecutionError::CommandArgumentError { argument, kind } => {
                        Self::CommandArgumentError { argument, kind }
                    }
                    ReadableExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    } => Self::TypeArgumentError {
                        type_argument,
                        kind,
                    },
                    ReadableExecutionError::UnusedValueWithoutDrop { result, subresult } => {
                        Self::UnusedValueWithoutDrop { result, subresult }
                    }
                    ReadableExecutionError::InvalidPublicFunctionReturnType { index } => {
                        Self::InvalidPublicFunctionReturnType { index }
                    }
                    ReadableExecutionError::InvalidTransferObject => Self::InvalidTransferObject,
                    ReadableExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    } => Self::EffectsTooLarge {
                        current_size,
                        max_size,
                    },
                    ReadableExecutionError::PublishUpgradeMissingDependency => {
                        Self::PublishUpgradeMissingDependency
                    }
                    ReadableExecutionError::PublishUpgradeDependencyDowngrade => {
                        Self::PublishUpgradeDependencyDowngrade
                    }
                    ReadableExecutionError::PackageUpgradeError(err) => {
                        Self::PackageUpgradeError(err)
                    }
                    ReadableExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    } => Self::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    },
                    ReadableExecutionError::CertificateDenied => Self::CertificateDenied,
                    ReadableExecutionError::SuiMoveVerificationTimedout => {
                        Self::SuiMoveVerificationTimedout
                    }
                    ReadableExecutionError::SharedObjectOperationNotAllowed => {
                        Self::SharedObjectOperationNotAllowed
                    }
                    ReadableExecutionError::InputObjectDeleted => Self::InputObjectDeleted,
                })
            } else {
                BinaryExecutionError::deserialize(deserializer).map(|binary| match binary {
                    BinaryExecutionError::InsufficientGas => Self::InsufficientGas,
                    BinaryExecutionError::InvalidGasObject => Self::InvalidGasObject,
                    BinaryExecutionError::InvariantViolation => Self::InvariantViolation,
                    BinaryExecutionError::FeatureNotYetSupported => Self::FeatureNotYetSupported,
                    BinaryExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    } => Self::ObjectTooBig {
                        object_size,
                        max_object_size,
                    },
                    BinaryExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    } => Self::PackageTooBig {
                        object_size,
                        max_object_size,
                    },
                    BinaryExecutionError::CircularObjectOwnership { object } => {
                        Self::CircularObjectOwnership { object }
                    }
                    BinaryExecutionError::InsufficientCoinBalance => Self::InsufficientCoinBalance,
                    BinaryExecutionError::CoinBalanceOverflow => Self::CoinBalanceOverflow,
                    BinaryExecutionError::PublishErrorNonZeroAddress => {
                        Self::PublishErrorNonZeroAddress
                    }
                    BinaryExecutionError::SuiMoveVerificationError => {
                        Self::SuiMoveVerificationError
                    }
                    BinaryExecutionError::MovePrimitiveRuntimeError { location } => {
                        Self::MovePrimitiveRuntimeError { location }
                    }
                    BinaryExecutionError::MoveAbort { location, code } => {
                        Self::MoveAbort { location, code }
                    }
                    BinaryExecutionError::VmVerificationOrDeserializationError => {
                        Self::VmVerificationOrDeserializationError
                    }
                    BinaryExecutionError::VmInvariantViolation => Self::VmInvariantViolation,
                    BinaryExecutionError::FunctionNotFound => Self::FunctionNotFound,
                    BinaryExecutionError::ArityMismatch => Self::ArityMismatch,
                    BinaryExecutionError::TypeArityMismatch => Self::TypeArityMismatch,
                    BinaryExecutionError::NonEntryFunctionInvoked => Self::NonEntryFunctionInvoked,
                    BinaryExecutionError::CommandArgumentError { argument, kind } => {
                        Self::CommandArgumentError { argument, kind }
                    }
                    BinaryExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    } => Self::TypeArgumentError {
                        type_argument,
                        kind,
                    },
                    BinaryExecutionError::UnusedValueWithoutDrop { result, subresult } => {
                        Self::UnusedValueWithoutDrop { result, subresult }
                    }
                    BinaryExecutionError::InvalidPublicFunctionReturnType { index } => {
                        Self::InvalidPublicFunctionReturnType { index }
                    }
                    BinaryExecutionError::InvalidTransferObject => Self::InvalidTransferObject,
                    BinaryExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    } => Self::EffectsTooLarge {
                        current_size,
                        max_size,
                    },
                    BinaryExecutionError::PublishUpgradeMissingDependency => {
                        Self::PublishUpgradeMissingDependency
                    }
                    BinaryExecutionError::PublishUpgradeDependencyDowngrade => {
                        Self::PublishUpgradeDependencyDowngrade
                    }
                    BinaryExecutionError::PackageUpgradeError(err) => {
                        Self::PackageUpgradeError(err)
                    }
                    BinaryExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    } => Self::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    },
                    BinaryExecutionError::CertificateDenied => Self::CertificateDenied,
                    BinaryExecutionError::SuiMoveVerificationTimedout => {
                        Self::SuiMoveVerificationTimedout
                    }
                    BinaryExecutionError::SharedObjectOperationNotAllowed => {
                        Self::SharedObjectOperationNotAllowed
                    }
                    BinaryExecutionError::InputObjectDeleted => Self::InputObjectDeleted,
                })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum ReadableCommandArgumentError {
        TypeMismatch,
        InvalidBcsBytes,
        InvalidUsageOfPureArgument,
        InvalidArgumentToPrivateEntryFunction,
        IndexOutOfBounds { index: u16 },
        SecondaryIndexOutOfBounds { result: u16, subresult: u16 },
        InvalidResultArity { result: u16 },
        InvalidGasCoinUsage,
        InvalidValueUsage,
        InvalidObjectByValue,
        InvalidObjectByMutRef,
        SharedObjectOperationNotAllowed,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum BinaryCommandArgumentError {
        TypeMismatch,
        InvalidBcsBytes,
        InvalidUsageOfPureArgument,
        InvalidArgumentToPrivateEntryFunction,
        IndexOutOfBounds { index: u16 },
        SecondaryIndexOutOfBounds { result: u16, subresult: u16 },
        InvalidResultArity { result: u16 },
        InvalidGasCoinUsage,
        InvalidValueUsage,
        InvalidObjectByValue,
        InvalidObjectByMutRef,
        SharedObjectOperationNotAllowed,
    }

    impl Serialize for CommandArgumentError {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self.clone() {
                    Self::TypeMismatch => ReadableCommandArgumentError::TypeMismatch,
                    Self::InvalidBcsBytes => ReadableCommandArgumentError::InvalidBcsBytes,
                    Self::InvalidUsageOfPureArgument => {
                        ReadableCommandArgumentError::InvalidUsageOfPureArgument
                    }
                    Self::InvalidArgumentToPrivateEntryFunction => {
                        ReadableCommandArgumentError::InvalidArgumentToPrivateEntryFunction
                    }
                    Self::IndexOutOfBounds { index } => {
                        ReadableCommandArgumentError::IndexOutOfBounds { index }
                    }
                    Self::SecondaryIndexOutOfBounds { result, subresult } => {
                        ReadableCommandArgumentError::SecondaryIndexOutOfBounds {
                            result,
                            subresult,
                        }
                    }
                    Self::InvalidResultArity { result } => {
                        ReadableCommandArgumentError::InvalidResultArity { result }
                    }
                    Self::InvalidGasCoinUsage => ReadableCommandArgumentError::InvalidGasCoinUsage,
                    Self::InvalidValueUsage => ReadableCommandArgumentError::InvalidValueUsage,
                    Self::InvalidObjectByValue => {
                        ReadableCommandArgumentError::InvalidObjectByValue
                    }
                    Self::InvalidObjectByMutRef => {
                        ReadableCommandArgumentError::InvalidObjectByMutRef
                    }
                    Self::SharedObjectOperationNotAllowed => {
                        ReadableCommandArgumentError::SharedObjectOperationNotAllowed
                    }
                };
                readable.serialize(serializer)
            } else {
                let binary = match self.clone() {
                    Self::TypeMismatch => BinaryCommandArgumentError::TypeMismatch,
                    Self::InvalidBcsBytes => BinaryCommandArgumentError::InvalidBcsBytes,
                    Self::InvalidUsageOfPureArgument => {
                        BinaryCommandArgumentError::InvalidUsageOfPureArgument
                    }
                    Self::InvalidArgumentToPrivateEntryFunction => {
                        BinaryCommandArgumentError::InvalidArgumentToPrivateEntryFunction
                    }
                    Self::IndexOutOfBounds { index } => {
                        BinaryCommandArgumentError::IndexOutOfBounds { index }
                    }
                    Self::SecondaryIndexOutOfBounds { result, subresult } => {
                        BinaryCommandArgumentError::SecondaryIndexOutOfBounds { result, subresult }
                    }
                    Self::InvalidResultArity { result } => {
                        BinaryCommandArgumentError::InvalidResultArity { result }
                    }
                    Self::InvalidGasCoinUsage => BinaryCommandArgumentError::InvalidGasCoinUsage,
                    Self::InvalidValueUsage => BinaryCommandArgumentError::InvalidValueUsage,
                    Self::InvalidObjectByValue => BinaryCommandArgumentError::InvalidObjectByValue,
                    Self::InvalidObjectByMutRef => {
                        BinaryCommandArgumentError::InvalidObjectByMutRef
                    }
                    Self::SharedObjectOperationNotAllowed => {
                        BinaryCommandArgumentError::SharedObjectOperationNotAllowed
                    }
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for CommandArgumentError {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableCommandArgumentError::deserialize(deserializer).map(|readable| {
                    match readable {
                        ReadableCommandArgumentError::TypeMismatch => Self::TypeMismatch,
                        ReadableCommandArgumentError::InvalidBcsBytes => Self::InvalidBcsBytes,
                        ReadableCommandArgumentError::InvalidUsageOfPureArgument => {
                            Self::InvalidUsageOfPureArgument
                        }
                        ReadableCommandArgumentError::InvalidArgumentToPrivateEntryFunction => {
                            Self::InvalidArgumentToPrivateEntryFunction
                        }
                        ReadableCommandArgumentError::IndexOutOfBounds { index } => {
                            Self::IndexOutOfBounds { index }
                        }
                        ReadableCommandArgumentError::SecondaryIndexOutOfBounds {
                            result,
                            subresult,
                        } => Self::SecondaryIndexOutOfBounds { result, subresult },
                        ReadableCommandArgumentError::InvalidResultArity { result } => {
                            Self::InvalidResultArity { result }
                        }
                        ReadableCommandArgumentError::InvalidGasCoinUsage => {
                            Self::InvalidGasCoinUsage
                        }
                        ReadableCommandArgumentError::InvalidValueUsage => Self::InvalidValueUsage,
                        ReadableCommandArgumentError::InvalidObjectByValue => {
                            Self::InvalidObjectByValue
                        }
                        ReadableCommandArgumentError::InvalidObjectByMutRef => {
                            Self::InvalidObjectByMutRef
                        }
                        ReadableCommandArgumentError::SharedObjectOperationNotAllowed => {
                            Self::SharedObjectOperationNotAllowed
                        }
                    }
                })
            } else {
                BinaryCommandArgumentError::deserialize(deserializer).map(|binary| match binary {
                    BinaryCommandArgumentError::TypeMismatch => Self::TypeMismatch,
                    BinaryCommandArgumentError::InvalidBcsBytes => Self::InvalidBcsBytes,
                    BinaryCommandArgumentError::InvalidUsageOfPureArgument => {
                        Self::InvalidUsageOfPureArgument
                    }
                    BinaryCommandArgumentError::InvalidArgumentToPrivateEntryFunction => {
                        Self::InvalidArgumentToPrivateEntryFunction
                    }
                    BinaryCommandArgumentError::IndexOutOfBounds { index } => {
                        Self::IndexOutOfBounds { index }
                    }
                    BinaryCommandArgumentError::SecondaryIndexOutOfBounds { result, subresult } => {
                        Self::SecondaryIndexOutOfBounds { result, subresult }
                    }
                    BinaryCommandArgumentError::InvalidResultArity { result } => {
                        Self::InvalidResultArity { result }
                    }
                    BinaryCommandArgumentError::InvalidGasCoinUsage => Self::InvalidGasCoinUsage,
                    BinaryCommandArgumentError::InvalidValueUsage => Self::InvalidValueUsage,
                    BinaryCommandArgumentError::InvalidObjectByValue => Self::InvalidObjectByValue,
                    BinaryCommandArgumentError::InvalidObjectByMutRef => {
                        Self::InvalidObjectByMutRef
                    }
                    BinaryCommandArgumentError::SharedObjectOperationNotAllowed => {
                        Self::SharedObjectOperationNotAllowed
                    }
                })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum ReadablePackageUpgradeError {
        UnableToFetchPackage {
            package_id: ObjectId,
        },
        NotAPackage {
            object_id: ObjectId,
        },
        IncompatibleUpgrade,
        DigestDoesNotMatch {
            digest: Digest,
        },
        UnknownUpgradePolicy {
            policy: u8,
        },
        PackageIdDoesNotMatch {
            package_id: ObjectId,
            ticket_id: ObjectId,
        },
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum BinaryPackageUpgradeError {
        UnableToFetchPackage {
            package_id: ObjectId,
        },
        NotAPackage {
            object_id: ObjectId,
        },
        IncompatibleUpgrade,
        DigestDoesNotMatch {
            digest: Digest,
        },
        UnknownUpgradePolicy {
            policy: u8,
        },
        PackageIdDoesNotMatch {
            package_id: ObjectId,
            ticket_id: ObjectId,
        },
    }

    impl Serialize for PackageUpgradeError {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self.clone() {
                    Self::UnableToFetchPackage { package_id } => {
                        ReadablePackageUpgradeError::UnableToFetchPackage { package_id }
                    }
                    Self::NotAPackage { object_id } => {
                        ReadablePackageUpgradeError::NotAPackage { object_id }
                    }
                    Self::IncompatibleUpgrade => ReadablePackageUpgradeError::IncompatibleUpgrade,
                    Self::DigestDoesNotMatch { digest } => {
                        ReadablePackageUpgradeError::DigestDoesNotMatch { digest }
                    }
                    Self::UnknownUpgradePolicy { policy } => {
                        ReadablePackageUpgradeError::UnknownUpgradePolicy { policy }
                    }
                    Self::PackageIdDoesNotMatch {
                        package_id,
                        ticket_id,
                    } => ReadablePackageUpgradeError::PackageIdDoesNotMatch {
                        package_id,
                        ticket_id,
                    },
                };
                readable.serialize(serializer)
            } else {
                let binary = match self.clone() {
                    Self::UnableToFetchPackage { package_id } => {
                        BinaryPackageUpgradeError::UnableToFetchPackage { package_id }
                    }
                    Self::NotAPackage { object_id } => {
                        BinaryPackageUpgradeError::NotAPackage { object_id }
                    }
                    Self::IncompatibleUpgrade => BinaryPackageUpgradeError::IncompatibleUpgrade,
                    Self::DigestDoesNotMatch { digest } => {
                        BinaryPackageUpgradeError::DigestDoesNotMatch { digest }
                    }
                    Self::UnknownUpgradePolicy { policy } => {
                        BinaryPackageUpgradeError::UnknownUpgradePolicy { policy }
                    }
                    Self::PackageIdDoesNotMatch {
                        package_id,
                        ticket_id,
                    } => BinaryPackageUpgradeError::PackageIdDoesNotMatch {
                        package_id,
                        ticket_id,
                    },
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for PackageUpgradeError {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadablePackageUpgradeError::deserialize(deserializer).map(
                    |readable| match readable {
                        ReadablePackageUpgradeError::UnableToFetchPackage { package_id } => {
                            Self::UnableToFetchPackage { package_id }
                        }
                        ReadablePackageUpgradeError::NotAPackage { object_id } => {
                            Self::NotAPackage { object_id }
                        }
                        ReadablePackageUpgradeError::IncompatibleUpgrade => {
                            Self::IncompatibleUpgrade
                        }
                        ReadablePackageUpgradeError::DigestDoesNotMatch { digest } => {
                            Self::DigestDoesNotMatch { digest }
                        }
                        ReadablePackageUpgradeError::UnknownUpgradePolicy { policy } => {
                            Self::UnknownUpgradePolicy { policy }
                        }
                        ReadablePackageUpgradeError::PackageIdDoesNotMatch {
                            package_id,
                            ticket_id,
                        } => Self::PackageIdDoesNotMatch {
                            package_id,
                            ticket_id,
                        },
                    },
                )
            } else {
                BinaryPackageUpgradeError::deserialize(deserializer).map(|binary| match binary {
                    BinaryPackageUpgradeError::UnableToFetchPackage { package_id } => {
                        Self::UnableToFetchPackage { package_id }
                    }
                    BinaryPackageUpgradeError::NotAPackage { object_id } => {
                        Self::NotAPackage { object_id }
                    }
                    BinaryPackageUpgradeError::IncompatibleUpgrade => Self::IncompatibleUpgrade,
                    BinaryPackageUpgradeError::DigestDoesNotMatch { digest } => {
                        Self::DigestDoesNotMatch { digest }
                    }
                    BinaryPackageUpgradeError::UnknownUpgradePolicy { policy } => {
                        Self::UnknownUpgradePolicy { policy }
                    }
                    BinaryPackageUpgradeError::PackageIdDoesNotMatch {
                        package_id,
                        ticket_id,
                    } => Self::PackageIdDoesNotMatch {
                        package_id,
                        ticket_id,
                    },
                })
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use test_strategy::proptest;

        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen_test::wasm_bindgen_test as test;

        #[proptest]
        fn roundtrip_bcs(status: ExecutionStatus) {
            let b = bcs::to_bytes(&status).unwrap();
            let a = bcs::from_bytes(&b).unwrap();
            assert_eq!(status, a);
        }

        #[proptest]
        fn roundtrip_json(status: ExecutionStatus) {
            let s = serde_json::to_string(&status).unwrap();
            println!("{s}");
            let a = serde_json::from_str(&s).unwrap();
            assert_eq!(status, a);
        }
    }
}
