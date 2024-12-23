use super::Address;
use super::CheckpointTimestamp;
use super::ConsensusCommitDigest;
use super::EpochId;
use super::GenesisObject;
use super::Identifier;
use super::Jwk;
use super::JwkId;
use super::ObjectId;
use super::ObjectReference;
use super::ProtocolVersion;
use super::TransactionDigest;
use super::TypeTag;
use super::UserSignature;
use super::Version;

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization;
#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
pub(crate) use serialization::SignedTransactionWithIntentMessage;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct Transaction {
    pub kind: TransactionKind,
    pub sender: Address,
    pub gas_payment: GasPayment,
    pub expiration: TransactionExpiration,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signatures: Vec<UserSignature>,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum TransactionExpiration {
    /// The transaction has no expiration
    #[default]
    None,
    /// Validators wont sign a transaction unless the expiration Epoch
    /// is greater than or equal to the current epoch
    Epoch(EpochId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct GasPayment {
    pub objects: Vec<ObjectReference>,
    pub owner: Address,
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub price: u64,
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub budget: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct RandomnessStateUpdate {
    /// Epoch of the randomness state update transaction
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub epoch: u64,
    /// Randomness round of the update
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub randomness_round: u64,
    /// Updated random bytes
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::_serde::ReadableBase64Encoded")
    )]
    pub random_bytes: Vec<u8>,
    /// The initial version of the randomness object that it was shared at.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub randomness_obj_initial_shared_version: u64,
    // to version this struct, do not add new fields. Instead, add a RandomnessStateUpdateV2 to
    // TransactionKind.
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum TransactionKind {
    /// A transaction that allows the interleaving of native commands and Move calls
    ProgrammableTransaction(ProgrammableTransaction),
    /// A system transaction that will update epoch information on-chain.
    /// It will only ever be executed once in an epoch.
    /// The argument is the next epoch number, which is critical
    /// because it ensures that this transaction has a unique digest.
    /// This will eventually be translated to a Move call during execution.
    /// It also doesn't require/use a gas object.
    /// A validator will not sign a transaction of this kind from outside. It only
    /// signs internally during epoch changes.
    ///
    /// The ChangeEpoch enumerant is now deprecated (but the ChangeEpoch struct is still used by
    /// EndOfEpochTransaction below).
    ChangeEpoch(ChangeEpoch),
    Genesis(GenesisTransaction),
    ConsensusCommitPrologue(ConsensusCommitPrologue),
    AuthenticatorStateUpdate(AuthenticatorStateUpdate),

    /// EndOfEpochTransaction replaces ChangeEpoch with a list of transactions that are allowed to
    /// run at the end of the epoch.
    EndOfEpoch(Vec<EndOfEpochTransactionKind>),

    RandomnessStateUpdate(RandomnessStateUpdate),
    // V2 ConsensusCommitPrologue also includes the digest of the current consensus output.
    ConsensusCommitPrologueV2(ConsensusCommitPrologueV2),

    ConsensusCommitPrologueV3(ConsensusCommitPrologueV3),
    // .. more transaction types go here
}

/// EndOfEpochTransactionKind
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum EndOfEpochTransactionKind {
    ChangeEpoch(ChangeEpoch),
    AuthenticatorStateCreate,
    AuthenticatorStateExpire(AuthenticatorStateExpire),
    RandomnessStateCreate,
    DenyListStateCreate,
    BridgeStateCreate {
        chain_id: super::CheckpointDigest,
    },
    BridgeCommitteeInit {
        bridge_object_version: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct AuthenticatorStateExpire {
    /// expire JWKs that have a lower epoch than this
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub min_epoch: u64,
    /// The initial version of the authenticator object that it was shared at.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub authenticator_object_initial_shared_version: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct AuthenticatorStateUpdate {
    /// Epoch of the authenticator state update transaction
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub epoch: u64,
    /// Consensus round of the authenticator state update
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub round: u64,
    /// newly active jwks
    pub new_active_jwks: Vec<ActiveJwk>,
    /// The initial version of the authenticator object that it was shared at.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub authenticator_obj_initial_shared_version: u64,
    // to version this struct, do not add new fields. Instead, add a AuthenticatorStateUpdateV2 to
    // TransactionKind.
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ActiveJwk {
    pub jwk_id: JwkId,
    pub jwk: Jwk,
    // the most recent epoch in which the jwk was validated
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub epoch: u64,
}

/// Only commit_timestamp_ms is passed to the move call currently.
/// However we include epoch and round to make sure each ConsensusCommitPrologue has a unique tx digest.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ConsensusCommitPrologue {
    /// Epoch of the commit prologue transaction
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub epoch: u64,
    /// Consensus round of the commit
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub round: u64,
    /// Unix timestamp from consensus
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub commit_timestamp_ms: CheckpointTimestamp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ConsensusCommitPrologueV2 {
    /// Epoch of the commit prologue transaction
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub epoch: u64,
    /// Consensus round of the commit
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub round: u64,
    /// Unix timestamp from consensus
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub commit_timestamp_ms: CheckpointTimestamp,
    /// Digest of consensus output
    pub consensus_commit_digest: ConsensusCommitDigest,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum ConsensusDeterminedVersionAssignments {
    /// Cancelled transaction version assignment.
    CancelledTransactions {
        #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
        cancelled_transactions: Vec<CancelledTransaction>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct CancelledTransaction {
    pub digest: TransactionDigest,
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub version_assignments: Vec<VersionAssignment>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct VersionAssignment {
    pub object_id: ObjectId,
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub version: Version,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ConsensusCommitPrologueV3 {
    /// Epoch of the commit prologue transaction
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub epoch: u64,
    /// Consensus round of the commit
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub round: u64,
    /// The sub DAG index of the consensus commit. This field will be populated if there
    /// are multiple consensus commits per round.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::_serde::OptionReadableDisplay")
    )]
    pub sub_dag_index: Option<u64>,
    /// Unix timestamp from consensus
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub commit_timestamp_ms: CheckpointTimestamp,
    /// Digest of consensus output
    pub consensus_commit_digest: ConsensusCommitDigest,
    /// Stores consensus handler determined shared object version assignments.
    pub consensus_determined_version_assignments: ConsensusDeterminedVersionAssignments,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ChangeEpoch {
    /// The next (to become) epoch ID.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub epoch: EpochId,
    /// The protocol version in effect in the new epoch.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub protocol_version: ProtocolVersion,
    /// The total amount of gas charged for storage during the epoch.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub storage_charge: u64,
    /// The total amount of gas charged for computation during the epoch.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub computation_charge: u64,
    /// The amount of storage rebate refunded to the txn senders.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub storage_rebate: u64,
    /// The non-refundable storage fee.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub non_refundable_storage_fee: u64,
    /// Unix timestamp when epoch started
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub epoch_start_timestamp_ms: u64,
    /// System packages (specifically framework and move stdlib) that are written before the new
    /// epoch starts. This tracks framework upgrades on chain. When executing the ChangeEpoch txn,
    /// the validator must write out the modules below.  Modules are provided with the version they
    /// will be upgraded to, their modules in serialized form (which include their package ID), and
    /// a list of their transitive dependencies.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub system_packages: Vec<SystemPackage>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct SystemPackage {
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub version: Version,
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<Vec<::serde_with::IfIsHumanReadable<crate::_serde::Base64Encoded, ::serde_with::Bytes>>>"
        )
    )]
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub modules: Vec<Vec<u8>>,
    pub dependencies: Vec<ObjectId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct GenesisTransaction {
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub objects: Vec<GenesisObject>,
}

/// A series of commands where the results of one command can be used in future
/// commands
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ProgrammableTransaction {
    /// Input objects or primitive values
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=10).lift()))]
    pub inputs: Vec<Input>,
    /// The commands to be executed sequentially. A failure in any command will
    /// result in the failure of the entire transaction.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=10).lift()))]
    pub commands: Vec<Command>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum Input {
    // contains no structs or objects
    Pure {
        value: Vec<u8>,
    },
    // A Move object, either immutable, or owned mutable.
    ImmutableOrOwned(ObjectReference),
    // A Move object that's shared.
    // SharedObject::mutable controls whether caller asks for a mutable reference to shared object.
    Shared {
        object_id: ObjectId,
        initial_shared_version: u64,
        mutable: bool,
    },
    // A Move object that can be received in this transaction.
    Receiving(ObjectReference),
}

/// A single command in a programmable transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum Command {
    /// A call to either an entry or a public Move function
    MoveCall(MoveCall),
    /// `(Vec<forall T:key+store. T>, address)`
    /// It sends n-objects to the specified address. These objects must have store
    /// (public transfer) and either the previous owner must be an address or the object must
    /// be newly created.
    TransferObjects(TransferObjects),
    /// `(&mut Coin<T>, Vec<u64>)` -> `Vec<Coin<T>>`
    /// It splits off some amounts into a new coins with those amounts
    SplitCoins(SplitCoins),
    /// `(&mut Coin<T>, Vec<Coin<T>>)`
    /// It merges n-coins into the first coin
    MergeCoins(MergeCoins),
    /// Publishes a Move package. It takes the package bytes and a list of the package's transitive
    /// dependencies to link against on-chain.
    Publish(Publish),
    /// `forall T: Vec<T> -> vector<T>`
    /// Given n-values of the same type, it constructs a vector. For non objects or an empty vector,
    /// the type tag must be specified.
    MakeMoveVector(MakeMoveVector),
    /// Upgrades a Move package
    /// Takes (in order):
    /// 1. A vector of serialized modules for the package.
    /// 2. A vector of object ids for the transitive dependencies of the new package.
    /// 3. The object ID of the package being upgraded.
    /// 4. An argument holding the `UpgradeTicket` that must have been produced from an earlier command in the same
    ///    programmable transaction.
    Upgrade(Upgrade),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct TransferObjects {
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub objects: Vec<Argument>,
    pub address: Argument,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct SplitCoins {
    pub coin: Argument,
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub amounts: Vec<Argument>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct MergeCoins {
    pub coin: Argument,
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub coins_to_merge: Vec<Argument>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct Publish {
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<Vec<::serde_with::IfIsHumanReadable<crate::_serde::Base64Encoded, ::serde_with::Bytes>>>"
        )
    )]
    pub modules: Vec<Vec<u8>>,
    pub dependencies: Vec<ObjectId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct MakeMoveVector {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: Option<TypeTag>,
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub elements: Vec<Argument>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct Upgrade {
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<Vec<::serde_with::IfIsHumanReadable<crate::_serde::Base64Encoded, ::serde_with::Bytes>>>"
        )
    )]
    pub modules: Vec<Vec<u8>>,
    pub dependencies: Vec<ObjectId>,
    pub package: ObjectId,
    pub ticket: Argument,
}

/// An argument to a programmable transaction command
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum Argument {
    /// The gas coin. The gas coin can only be used by-ref, except for with
    /// `TransferObjects`, which can use it by-value.
    Gas,
    /// One of the input objects or primitive values (from
    /// `ProgrammableTransaction` inputs)
    Input(u16),
    /// The result of another command (from `ProgrammableTransaction` commands)
    Result(u16),
    /// Like a `Result` but it accesses a nested result. Currently, the only usage
    /// of this is to access a value from a Move call with multiple return values.
    // (command index, subresult index)
    NestedResult(u16, u16),
}

impl Argument {
    /// Turn a Result into a NestedResult. If the argument is not a Result, returns None.
    pub fn nested(&self, ix: u16) -> Option<Argument> {
        match self {
            Argument::Result(i) => Some(Argument::NestedResult(*i, ix)),
            _ => None,
        }
    }
}

/// The command for calling a Move function, either an entry function or a public
/// function (which cannot return references).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct MoveCall {
    /// The package containing the module and function.
    pub package: ObjectId,
    /// The specific module in the package containing the function.
    pub module: Identifier,
    /// The function to be called.
    pub function: Identifier,
    /// The type arguments to the function.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub type_arguments: Vec<TypeTag>,
    /// The arguments to the function.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub arguments: Vec<Argument>,
}
