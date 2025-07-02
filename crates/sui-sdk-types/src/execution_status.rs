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
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
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
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
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
    ExecutionCanceledDueToSharedObjectCongestion {
        #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=1).lift()))]
        congested_objects: Vec<ObjectId>,
    },

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

    /// A valid linkage was unable to be determined for the transaction or one of its commands.
    InvalidLinkage,
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
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
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

    /// Invalid argument arity. Expected a single argument but found a result that expanded to
    /// multiple arguments.
    InvalidArgumentArity,
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
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
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
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum TypeArgumentError {
    /// A type was not found in the module specified
    TypeNotFound,

    /// A type provided did not match the specified constraint
    ConstraintNotSatisfied,
}
