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
        object_size: u64,
        max_object_size: u64,
    },
    /// Package is larger than the maximum allowed size
    PackageTooBig {
        object_size: u64,
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
    MoveAbort { location: MoveLocation, code: u64 },
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
    EffectsTooLarge { current_size: u64, max_size: u64 },

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
        object_size: u64,
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
    derive(serde_derive::Serialize, serde_derive::Deserialize)
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
    struct ReadableExecutionStatus {
        success: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<FailureStatus>,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
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
    #[serde(tag = "error")]
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
                    ExecutionError::InsufficientGas => ReadableExecutionError::InsufficientGas,
                    ExecutionError::InvalidGasObject => ReadableExecutionError::InvalidGasObject,
                    ExecutionError::InvariantViolation => {
                        ReadableExecutionError::InvariantViolation
                    }
                    ExecutionError::FeatureNotYetSupported => {
                        ReadableExecutionError::FeatureNotYetSupported
                    }
                    ExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    } => ReadableExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    },
                    ExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    } => ReadableExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    },
                    ExecutionError::CircularObjectOwnership { object } => {
                        ReadableExecutionError::CircularObjectOwnership { object }
                    }
                    ExecutionError::InsufficientCoinBalance => {
                        ReadableExecutionError::InsufficientCoinBalance
                    }
                    ExecutionError::CoinBalanceOverflow => {
                        ReadableExecutionError::CoinBalanceOverflow
                    }
                    ExecutionError::PublishErrorNonZeroAddress => {
                        ReadableExecutionError::PublishErrorNonZeroAddress
                    }
                    ExecutionError::SuiMoveVerificationError => {
                        ReadableExecutionError::SuiMoveVerificationError
                    }
                    ExecutionError::MovePrimitiveRuntimeError { location } => {
                        ReadableExecutionError::MovePrimitiveRuntimeError { location }
                    }
                    ExecutionError::MoveAbort { location, code } => {
                        ReadableExecutionError::MoveAbort { location, code }
                    }
                    ExecutionError::VmVerificationOrDeserializationError => {
                        ReadableExecutionError::VmVerificationOrDeserializationError
                    }
                    ExecutionError::VmInvariantViolation => {
                        ReadableExecutionError::VmInvariantViolation
                    }
                    ExecutionError::FunctionNotFound => ReadableExecutionError::FunctionNotFound,
                    ExecutionError::ArityMismatch => ReadableExecutionError::ArityMismatch,
                    ExecutionError::TypeArityMismatch => ReadableExecutionError::TypeArityMismatch,
                    ExecutionError::NonEntryFunctionInvoked => {
                        ReadableExecutionError::NonEntryFunctionInvoked
                    }
                    ExecutionError::CommandArgumentError { argument, kind } => {
                        ReadableExecutionError::CommandArgumentError { argument, kind }
                    }
                    ExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    } => ReadableExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    },
                    ExecutionError::UnusedValueWithoutDrop { result, subresult } => {
                        ReadableExecutionError::UnusedValueWithoutDrop { result, subresult }
                    }
                    ExecutionError::InvalidPublicFunctionReturnType { index } => {
                        ReadableExecutionError::InvalidPublicFunctionReturnType { index }
                    }
                    ExecutionError::InvalidTransferObject => {
                        ReadableExecutionError::InvalidTransferObject
                    }
                    ExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    } => ReadableExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    },
                    ExecutionError::PublishUpgradeMissingDependency => {
                        ReadableExecutionError::PublishUpgradeMissingDependency
                    }
                    ExecutionError::PublishUpgradeDependencyDowngrade => {
                        ReadableExecutionError::PublishUpgradeDependencyDowngrade
                    }
                    ExecutionError::PackageUpgradeError(err) => {
                        ReadableExecutionError::PackageUpgradeError(err)
                    }
                    ExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    } => ReadableExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    },
                    ExecutionError::CertificateDenied => ReadableExecutionError::CertificateDenied,
                    ExecutionError::SuiMoveVerificationTimedout => {
                        ReadableExecutionError::SuiMoveVerificationTimedout
                    }
                    ExecutionError::SharedObjectOperationNotAllowed => {
                        ReadableExecutionError::SharedObjectOperationNotAllowed
                    }
                    ExecutionError::InputObjectDeleted => {
                        ReadableExecutionError::InputObjectDeleted
                    }
                };
                readable.serialize(serializer)
            } else {
                let binary = match self.clone() {
                    ExecutionError::InsufficientGas => BinaryExecutionError::InsufficientGas,
                    ExecutionError::InvalidGasObject => BinaryExecutionError::InvalidGasObject,
                    ExecutionError::InvariantViolation => BinaryExecutionError::InvariantViolation,
                    ExecutionError::FeatureNotYetSupported => {
                        BinaryExecutionError::FeatureNotYetSupported
                    }
                    ExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    } => BinaryExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    },
                    ExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    } => BinaryExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    },
                    ExecutionError::CircularObjectOwnership { object } => {
                        BinaryExecutionError::CircularObjectOwnership { object }
                    }
                    ExecutionError::InsufficientCoinBalance => {
                        BinaryExecutionError::InsufficientCoinBalance
                    }
                    ExecutionError::CoinBalanceOverflow => {
                        BinaryExecutionError::CoinBalanceOverflow
                    }
                    ExecutionError::PublishErrorNonZeroAddress => {
                        BinaryExecutionError::PublishErrorNonZeroAddress
                    }
                    ExecutionError::SuiMoveVerificationError => {
                        BinaryExecutionError::SuiMoveVerificationError
                    }
                    ExecutionError::MovePrimitiveRuntimeError { location } => {
                        BinaryExecutionError::MovePrimitiveRuntimeError { location }
                    }
                    ExecutionError::MoveAbort { location, code } => {
                        BinaryExecutionError::MoveAbort { location, code }
                    }
                    ExecutionError::VmVerificationOrDeserializationError => {
                        BinaryExecutionError::VmVerificationOrDeserializationError
                    }
                    ExecutionError::VmInvariantViolation => {
                        BinaryExecutionError::VmInvariantViolation
                    }
                    ExecutionError::FunctionNotFound => BinaryExecutionError::FunctionNotFound,
                    ExecutionError::ArityMismatch => BinaryExecutionError::ArityMismatch,
                    ExecutionError::TypeArityMismatch => BinaryExecutionError::TypeArityMismatch,
                    ExecutionError::NonEntryFunctionInvoked => {
                        BinaryExecutionError::NonEntryFunctionInvoked
                    }
                    ExecutionError::CommandArgumentError { argument, kind } => {
                        BinaryExecutionError::CommandArgumentError { argument, kind }
                    }
                    ExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    } => BinaryExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    },
                    ExecutionError::UnusedValueWithoutDrop { result, subresult } => {
                        BinaryExecutionError::UnusedValueWithoutDrop { result, subresult }
                    }
                    ExecutionError::InvalidPublicFunctionReturnType { index } => {
                        BinaryExecutionError::InvalidPublicFunctionReturnType { index }
                    }
                    ExecutionError::InvalidTransferObject => {
                        BinaryExecutionError::InvalidTransferObject
                    }
                    ExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    } => BinaryExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    },
                    ExecutionError::PublishUpgradeMissingDependency => {
                        BinaryExecutionError::PublishUpgradeMissingDependency
                    }
                    ExecutionError::PublishUpgradeDependencyDowngrade => {
                        BinaryExecutionError::PublishUpgradeDependencyDowngrade
                    }
                    ExecutionError::PackageUpgradeError(err) => {
                        BinaryExecutionError::PackageUpgradeError(err)
                    }
                    ExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    } => BinaryExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    },
                    ExecutionError::CertificateDenied => BinaryExecutionError::CertificateDenied,
                    ExecutionError::SuiMoveVerificationTimedout => {
                        BinaryExecutionError::SuiMoveVerificationTimedout
                    }
                    ExecutionError::SharedObjectOperationNotAllowed => {
                        BinaryExecutionError::SharedObjectOperationNotAllowed
                    }
                    ExecutionError::InputObjectDeleted => BinaryExecutionError::InputObjectDeleted,
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
                    ReadableExecutionError::InsufficientGas => ExecutionError::InsufficientGas,
                    ReadableExecutionError::InvalidGasObject => ExecutionError::InvalidGasObject,
                    ReadableExecutionError::InvariantViolation => {
                        ExecutionError::InvariantViolation
                    }
                    ReadableExecutionError::FeatureNotYetSupported => {
                        ExecutionError::FeatureNotYetSupported
                    }
                    ReadableExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    } => ExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    },
                    ReadableExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    } => ExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    },
                    ReadableExecutionError::CircularObjectOwnership { object } => {
                        ExecutionError::CircularObjectOwnership { object }
                    }
                    ReadableExecutionError::InsufficientCoinBalance => {
                        ExecutionError::InsufficientCoinBalance
                    }
                    ReadableExecutionError::CoinBalanceOverflow => {
                        ExecutionError::CoinBalanceOverflow
                    }
                    ReadableExecutionError::PublishErrorNonZeroAddress => {
                        ExecutionError::PublishErrorNonZeroAddress
                    }
                    ReadableExecutionError::SuiMoveVerificationError => {
                        ExecutionError::SuiMoveVerificationError
                    }
                    ReadableExecutionError::MovePrimitiveRuntimeError { location } => {
                        ExecutionError::MovePrimitiveRuntimeError { location }
                    }
                    ReadableExecutionError::MoveAbort { location, code } => {
                        ExecutionError::MoveAbort { location, code }
                    }
                    ReadableExecutionError::VmVerificationOrDeserializationError => {
                        ExecutionError::VmVerificationOrDeserializationError
                    }
                    ReadableExecutionError::VmInvariantViolation => {
                        ExecutionError::VmInvariantViolation
                    }
                    ReadableExecutionError::FunctionNotFound => ExecutionError::FunctionNotFound,
                    ReadableExecutionError::ArityMismatch => ExecutionError::ArityMismatch,
                    ReadableExecutionError::TypeArityMismatch => ExecutionError::TypeArityMismatch,
                    ReadableExecutionError::NonEntryFunctionInvoked => {
                        ExecutionError::NonEntryFunctionInvoked
                    }
                    ReadableExecutionError::CommandArgumentError { argument, kind } => {
                        ExecutionError::CommandArgumentError { argument, kind }
                    }
                    ReadableExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    } => ExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    },
                    ReadableExecutionError::UnusedValueWithoutDrop { result, subresult } => {
                        ExecutionError::UnusedValueWithoutDrop { result, subresult }
                    }
                    ReadableExecutionError::InvalidPublicFunctionReturnType { index } => {
                        ExecutionError::InvalidPublicFunctionReturnType { index }
                    }
                    ReadableExecutionError::InvalidTransferObject => {
                        ExecutionError::InvalidTransferObject
                    }
                    ReadableExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    } => ExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    },
                    ReadableExecutionError::PublishUpgradeMissingDependency => {
                        ExecutionError::PublishUpgradeMissingDependency
                    }
                    ReadableExecutionError::PublishUpgradeDependencyDowngrade => {
                        ExecutionError::PublishUpgradeDependencyDowngrade
                    }
                    ReadableExecutionError::PackageUpgradeError(err) => {
                        ExecutionError::PackageUpgradeError(err)
                    }
                    ReadableExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    } => ExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    },
                    ReadableExecutionError::CertificateDenied => ExecutionError::CertificateDenied,
                    ReadableExecutionError::SuiMoveVerificationTimedout => {
                        ExecutionError::SuiMoveVerificationTimedout
                    }
                    ReadableExecutionError::SharedObjectOperationNotAllowed => {
                        ExecutionError::SharedObjectOperationNotAllowed
                    }
                    ReadableExecutionError::InputObjectDeleted => {
                        ExecutionError::InputObjectDeleted
                    }
                })
            } else {
                BinaryExecutionError::deserialize(deserializer).map(|binary| match binary {
                    BinaryExecutionError::InsufficientGas => ExecutionError::InsufficientGas,
                    BinaryExecutionError::InvalidGasObject => ExecutionError::InvalidGasObject,
                    BinaryExecutionError::InvariantViolation => ExecutionError::InvariantViolation,
                    BinaryExecutionError::FeatureNotYetSupported => {
                        ExecutionError::FeatureNotYetSupported
                    }
                    BinaryExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    } => ExecutionError::ObjectTooBig {
                        object_size,
                        max_object_size,
                    },
                    BinaryExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    } => ExecutionError::PackageTooBig {
                        object_size,
                        max_object_size,
                    },
                    BinaryExecutionError::CircularObjectOwnership { object } => {
                        ExecutionError::CircularObjectOwnership { object }
                    }
                    BinaryExecutionError::InsufficientCoinBalance => {
                        ExecutionError::InsufficientCoinBalance
                    }
                    BinaryExecutionError::CoinBalanceOverflow => {
                        ExecutionError::CoinBalanceOverflow
                    }
                    BinaryExecutionError::PublishErrorNonZeroAddress => {
                        ExecutionError::PublishErrorNonZeroAddress
                    }
                    BinaryExecutionError::SuiMoveVerificationError => {
                        ExecutionError::SuiMoveVerificationError
                    }
                    BinaryExecutionError::MovePrimitiveRuntimeError { location } => {
                        ExecutionError::MovePrimitiveRuntimeError { location }
                    }
                    BinaryExecutionError::MoveAbort { location, code } => {
                        ExecutionError::MoveAbort { location, code }
                    }
                    BinaryExecutionError::VmVerificationOrDeserializationError => {
                        ExecutionError::VmVerificationOrDeserializationError
                    }
                    BinaryExecutionError::VmInvariantViolation => {
                        ExecutionError::VmInvariantViolation
                    }
                    BinaryExecutionError::FunctionNotFound => ExecutionError::FunctionNotFound,
                    BinaryExecutionError::ArityMismatch => ExecutionError::ArityMismatch,
                    BinaryExecutionError::TypeArityMismatch => ExecutionError::TypeArityMismatch,
                    BinaryExecutionError::NonEntryFunctionInvoked => {
                        ExecutionError::NonEntryFunctionInvoked
                    }
                    BinaryExecutionError::CommandArgumentError { argument, kind } => {
                        ExecutionError::CommandArgumentError { argument, kind }
                    }
                    BinaryExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    } => ExecutionError::TypeArgumentError {
                        type_argument,
                        kind,
                    },
                    BinaryExecutionError::UnusedValueWithoutDrop { result, subresult } => {
                        ExecutionError::UnusedValueWithoutDrop { result, subresult }
                    }
                    BinaryExecutionError::InvalidPublicFunctionReturnType { index } => {
                        ExecutionError::InvalidPublicFunctionReturnType { index }
                    }
                    BinaryExecutionError::InvalidTransferObject => {
                        ExecutionError::InvalidTransferObject
                    }
                    BinaryExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    } => ExecutionError::EffectsTooLarge {
                        current_size,
                        max_size,
                    },
                    BinaryExecutionError::PublishUpgradeMissingDependency => {
                        ExecutionError::PublishUpgradeMissingDependency
                    }
                    BinaryExecutionError::PublishUpgradeDependencyDowngrade => {
                        ExecutionError::PublishUpgradeDependencyDowngrade
                    }
                    BinaryExecutionError::PackageUpgradeError(err) => {
                        ExecutionError::PackageUpgradeError(err)
                    }
                    BinaryExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    } => ExecutionError::WrittenObjectsTooLarge {
                        object_size,
                        max_object_size,
                    },
                    BinaryExecutionError::CertificateDenied => ExecutionError::CertificateDenied,
                    BinaryExecutionError::SuiMoveVerificationTimedout => {
                        ExecutionError::SuiMoveVerificationTimedout
                    }
                    BinaryExecutionError::SharedObjectOperationNotAllowed => {
                        ExecutionError::SharedObjectOperationNotAllowed
                    }
                    BinaryExecutionError::InputObjectDeleted => ExecutionError::InputObjectDeleted,
                })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "kind")]
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
    #[serde(tag = "kind")]
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
