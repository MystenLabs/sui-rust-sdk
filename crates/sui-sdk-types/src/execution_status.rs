use super::Address;
use super::Digest;
use super::Identifier;
use super::ObjectId;

/// The status of an executed Transaction
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// execution-status = success / failure
/// success = %x00
/// failure = %x01 execution-error (option u64)
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum ExecutionStatus {
    /// The Transaction successfully executed.
    Success,

    /// The Transaction didn't execute successfully.
    ///
    /// Failed transactions are still committed to the blockchain but any intended effects are
    /// rolled back to prior to this transaction executing with the caveat that gas objects are
    /// still smashed and gas usage is still charged.
    Failure {
        /// The error encountered during execution.
        error: ExecutionError,
        /// The command, if any, during which the error occurred.
        #[cfg_attr(feature = "proptest", map(|x: Option<u16>| x.map(Into::into)))]
        command: Option<u64>,
    },
}

/// An error that can occur during the execution of a transaction
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// execution-error =  insufficient-gas
///                 =/ invalid-gas-object
///                 =/ invariant-violation
///                 =/ feature-not-yet-supported
///                 =/ object-too-big
///                 =/ package-too-big
///                 =/ circular-object-ownership
///                 =/ insufficient-coin-balance
///                 =/ coin-balance-overflow
///                 =/ publish-error-non-zero-address
///                 =/ sui-move-verification-error
///                 =/ move-primitive-runtime-error
///                 =/ move-abort
///                 =/ vm-verification-or-deserialization-error
///                 =/ vm-invariant-violation
///                 =/ function-not-found
///                 =/ arity-mismatch
///                 =/ type-arity-mismatch
///                 =/ non-entry-function-invoked
///                 =/ command-argument-error
///                 =/ type-argument-error
///                 =/ unused-value-without-drop
///                 =/ invalid-public-function-return-type
///                 =/ invalid-transfer-object
///                 =/ effects-too-large
///                 =/ publish-upgrade-missing-dependency
///                 =/ publish-upgrade-dependency-downgrade
///                 =/ package-upgrade-error
///                 =/ written-objects-too-large
///                 =/ certificate-denied
///                 =/ sui-move-verification-timedout
///                 =/ shared-object-operation-not-allowed
///                 =/ input-object-deleted
///                 =/ execution-canceled-due-to-shared-object-congestion
///                 =/ address-denied-for-coin
///                 =/ coin-type-global-pause
///                 =/ execution-canceled-due-to-randomness-unavailable
///
/// insufficient-gas                                    = %x00
/// invalid-gas-object                                  = %x01
/// invariant-violation                                 = %x02
/// feature-not-yet-supported                           = %x03
/// object-too-big                                      = %x04 u64 u64
/// package-too-big                                     = %x05 u64 u64
/// circular-object-ownership                           = %x06 object-id
/// insufficient-coin-balance                           = %x07
/// coin-balance-overflow                               = %x08
/// publish-error-non-zero-address                      = %x09
/// sui-move-verification-error                         = %x0a
/// move-primitive-runtime-error                        = %x0b (option move-location)
/// move-abort                                          = %x0c move-location u64
/// vm-verification-or-deserialization-error            = %x0d
/// vm-invariant-violation                              = %x0e
/// function-not-found                                  = %x0f
/// arity-mismatch                                      = %x10
/// type-arity-mismatch                                 = %x11
/// non-entry-function-invoked                          = %x12
/// command-argument-error                              = %x13 u16 command-argument-error
/// type-argument-error                                 = %x14 u16 type-argument-error
/// unused-value-without-drop                           = %x15 u16 u16
/// invalid-public-function-return-type                 = %x16 u16
/// invalid-transfer-object                             = %x17
/// effects-too-large                                   = %x18 u64 u64
/// publish-upgrade-missing-dependency                  = %x19
/// publish-upgrade-dependency-downgrade                = %x1a
/// package-upgrade-error                               = %x1b package-upgrade-error
/// written-objects-too-large                           = %x1c u64 u64
/// certificate-denied                                  = %x1d
/// sui-move-verification-timedout                      = %x1e
/// shared-object-operation-not-allowed                 = %x1f
/// input-object-deleted                                = %x20
/// execution-canceled-due-to-shared-object-congestion = %x21 (vector object-id)
/// address-denied-for-coin                             = %x22 address string
/// coin-type-global-pause                              = %x23 string
/// execution-canceled-due-to-randomness-unavailable   = %x24
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
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
        /// Index of the problematic type argument
        type_argument: u16,
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
    PackageUpgradeError { kind: PackageUpgradeError },

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

    /// Certificate is canceled due to congestion on shared objects
    ExecutionCanceledDueToSharedObjectCongestion { congested_objects: Vec<ObjectId> },

    /// Address is denied for this coin type
    AddressDeniedForCoin { address: Address, coin_type: String },

    /// Coin type is globally paused for use
    CoinTypeGlobalPause { coin_type: String },

    /// Certificate is canceled because randomness could not be generated this epoch
    ExecutionCanceledDueToRandomnessUnavailable,

    /// Move vector element (passed to MakeMoveVec) with size {value_size} is larger \
    /// than the maximum size {max_scaled_size}. Note that this maximum is scaled based on the \
    /// type of the vector element.
    MoveVectorElemTooBig {
        value_size: u64,
        max_scaled_size: u64,
    },

    /// Move value (possibly an upgrade ticket or a dev-inspect value) with size {value_size} \
    /// is larger than the maximum size  {max_scaled_size}. Note that this maximum is scaled based \
    /// on the type of the value.
    MoveRawValueTooBig {
        value_size: u64,
        max_scaled_size: u64,
    },
}

/// Location in move bytecode where an error occurred
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// move-location = object-id identifier u16 u16 (option identifier)
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct MoveLocation {
    /// The package id
    pub package: ObjectId,

    /// The module name
    pub module: Identifier,

    /// The function index
    pub function: u16,

    /// Index into the code stream for a jump. The offset is relative to the beginning of
    /// the instruction stream.
    pub instruction: u16,

    /// The name of the function if available
    pub function_name: Option<Identifier>,
}

/// An error with an argument to a command
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// command-argument-error =  type-mismatch
///                        =/ invalid-bcs-bytes
///                        =/ invalid-usage-of-pure-argument
///                        =/ invalid-argument-to-private-entry-function
///                        =/ index-out-of-bounds
///                        =/ secondary-index-out-of-bound
///                        =/ invalid-result-arity
///                        =/ invalid-gas-coin-usage
///                        =/ invalid-value-usage
///                        =/ invalid-object-by-value
///                        =/ invalid-object-by-mut-ref
///                        =/ shared-object-operation-not-allowed
///
/// type-mismatch                               = %x00
/// invalid-bcs-bytes                           = %x01
/// invalid-usage-of-pure-argument              = %x02
/// invalid-argument-to-private-entry-function  = %x03
/// index-out-of-bounds                         = %x04 u16
/// secondary-index-out-of-bound                = %x05 u16 u16
/// invalid-result-arity                        = %x06 u16
/// invalid-gas-coin-usage                      = %x07
/// invalid-value-usage                         = %x08
/// invalid-object-by-value                     = %x09
/// invalid-object-by-mut-ref                   = %x0a
/// shared-object-operation-not-allowed         = %x0b
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
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

/// An error with a upgrading a package
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// package-upgrade-error = unable-to-fetch-package /
///                         not-a-package           /
///                         incompatible-upgrade    /
///                         digest-does-not-match   /
///                         unknown-upgrade-policy  /
///                         package-id-does-not-match
///
/// unable-to-fetch-package     = %x00 object-id
/// not-a-package               = %x01 object-id
/// incompatible-upgrade        = %x02
/// digest-does-not-match       = %x03 digest
/// unknown-upgrade-policy      = %x04 u8
/// package-id-does-not-match   = %x05 object-id object-id
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
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

/// An error with a type argument
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// type-argument-error = type-not-found / constraint-not-satisfied
/// type-not-found = %x00
/// constraint-not-satisfied = %x01
/// ```
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
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
    struct ReadableExecutionStatus {
        success: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<FailureStatus>,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    struct FailureStatus {
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
            kind: CommandArgumentError,
        },
        TypeArgumentError {
            type_argument: u16,
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
        PackageUpgradeError {
            kind: PackageUpgradeError,
        },
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
        ExecutionCanceledDueToSharedObjectCongestion {
            congested_objects: Vec<ObjectId>,
        },

        AddressDeniedForCoin {
            address: Address,
            coin_type: String,
        },

        CoinTypeGlobalPause {
            coin_type: String,
        },

        ExecutionCanceledDueToRandomnessUnavailable,

        MoveVectorElemTooBig {
            value_size: u64,
            max_scaled_size: u64,
        },

        MoveRawValueTooBig {
            value_size: u64,
            max_scaled_size: u64,
        },
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
            type_argument: u16,
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
        PackageUpgradeError {
            kind: PackageUpgradeError,
        },
        WrittenObjectsTooLarge {
            object_size: u64,
            max_object_size: u64,
        },
        CertificateDenied,
        SuiMoveVerificationTimedout,
        SharedObjectOperationNotAllowed,
        InputObjectDeleted,
        ExecutionCanceledDueToSharedObjectCongestion {
            congested_objects: Vec<ObjectId>,
        },

        AddressDeniedForCoin {
            address: Address,
            coin_type: String,
        },

        CoinTypeGlobalPause {
            coin_type: String,
        },

        ExecutionCanceledDueToRandomnessUnavailable,

        MoveVectorElemTooBig {
            value_size: u64,
            max_scaled_size: u64,
        },

        MoveRawValueTooBig {
            value_size: u64,
            max_scaled_size: u64,
        },
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
                    Self::PackageUpgradeError { kind } => {
                        ReadableExecutionError::PackageUpgradeError { kind }
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
                    Self::ExecutionCanceledDueToSharedObjectCongestion { congested_objects } => {
                        ReadableExecutionError::ExecutionCanceledDueToSharedObjectCongestion {
                            congested_objects,
                        }
                    }
                    Self::AddressDeniedForCoin { address, coin_type } => {
                        ReadableExecutionError::AddressDeniedForCoin { address, coin_type }
                    }
                    Self::CoinTypeGlobalPause { coin_type } => {
                        ReadableExecutionError::CoinTypeGlobalPause { coin_type }
                    }
                    Self::ExecutionCanceledDueToRandomnessUnavailable => {
                        ReadableExecutionError::ExecutionCanceledDueToRandomnessUnavailable
                    }
                    Self::MoveVectorElemTooBig {
                        value_size,
                        max_scaled_size,
                    } => ReadableExecutionError::MoveVectorElemTooBig {
                        value_size,
                        max_scaled_size,
                    },
                    Self::MoveRawValueTooBig {
                        value_size,
                        max_scaled_size,
                    } => ReadableExecutionError::MoveRawValueTooBig {
                        value_size,
                        max_scaled_size,
                    },
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
                    Self::PackageUpgradeError { kind } => {
                        BinaryExecutionError::PackageUpgradeError { kind }
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
                    Self::ExecutionCanceledDueToSharedObjectCongestion { congested_objects } => {
                        BinaryExecutionError::ExecutionCanceledDueToSharedObjectCongestion {
                            congested_objects,
                        }
                    }
                    Self::AddressDeniedForCoin { address, coin_type } => {
                        BinaryExecutionError::AddressDeniedForCoin { address, coin_type }
                    }
                    Self::CoinTypeGlobalPause { coin_type } => {
                        BinaryExecutionError::CoinTypeGlobalPause { coin_type }
                    }
                    Self::ExecutionCanceledDueToRandomnessUnavailable => {
                        BinaryExecutionError::ExecutionCanceledDueToRandomnessUnavailable
                    }
                    Self::MoveVectorElemTooBig {
                        value_size,
                        max_scaled_size,
                    } => BinaryExecutionError::MoveVectorElemTooBig {
                        value_size,
                        max_scaled_size,
                    },
                    Self::MoveRawValueTooBig {
                        value_size,
                        max_scaled_size,
                    } => BinaryExecutionError::MoveRawValueTooBig {
                        value_size,
                        max_scaled_size,
                    },
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
                    ReadableExecutionError::PackageUpgradeError { kind } => {
                        Self::PackageUpgradeError { kind }
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
                    ReadableExecutionError::ExecutionCanceledDueToSharedObjectCongestion {
                        congested_objects,
                    } => Self::ExecutionCanceledDueToSharedObjectCongestion { congested_objects },
                    ReadableExecutionError::AddressDeniedForCoin { address, coin_type } => {
                        Self::AddressDeniedForCoin { address, coin_type }
                    }
                    ReadableExecutionError::CoinTypeGlobalPause { coin_type } => {
                        Self::CoinTypeGlobalPause { coin_type }
                    }
                    ReadableExecutionError::ExecutionCanceledDueToRandomnessUnavailable => {
                        Self::ExecutionCanceledDueToRandomnessUnavailable
                    }
                    ReadableExecutionError::MoveVectorElemTooBig {
                        value_size,
                        max_scaled_size,
                    } => Self::MoveVectorElemTooBig {
                        value_size,
                        max_scaled_size,
                    },
                    ReadableExecutionError::MoveRawValueTooBig {
                        value_size,
                        max_scaled_size,
                    } => Self::MoveRawValueTooBig {
                        value_size,
                        max_scaled_size,
                    },
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
                    BinaryExecutionError::PackageUpgradeError { kind } => {
                        Self::PackageUpgradeError { kind }
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
                    BinaryExecutionError::ExecutionCanceledDueToSharedObjectCongestion {
                        congested_objects,
                    } => Self::ExecutionCanceledDueToSharedObjectCongestion { congested_objects },
                    BinaryExecutionError::AddressDeniedForCoin { address, coin_type } => {
                        Self::AddressDeniedForCoin { address, coin_type }
                    }
                    BinaryExecutionError::CoinTypeGlobalPause { coin_type } => {
                        Self::CoinTypeGlobalPause { coin_type }
                    }
                    BinaryExecutionError::ExecutionCanceledDueToRandomnessUnavailable => {
                        Self::ExecutionCanceledDueToRandomnessUnavailable
                    }
                    BinaryExecutionError::MoveVectorElemTooBig {
                        value_size,
                        max_scaled_size,
                    } => Self::MoveVectorElemTooBig {
                        value_size,
                        max_scaled_size,
                    },
                    BinaryExecutionError::MoveRawValueTooBig {
                        value_size,
                        max_scaled_size,
                    } => Self::MoveRawValueTooBig {
                        value_size,
                        max_scaled_size,
                    },
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
}
