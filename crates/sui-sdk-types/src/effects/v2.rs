use crate::execution_status::ExecutionStatus;
use crate::object::Owner;
use crate::object::Version;
use crate::Address;
use crate::Digest;
use crate::EpochId;
use crate::GasCostSummary;

/// Version 2 of TransactionEffects
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// effects-v2 = execution-status
///              u64                                ; epoch
///              gas-cost-summary
///              digest                             ; transaction digest
///              (option u32)                       ; gas object index
///              (option digest)                    ; events digest
///              (vector digest)                    ; list of transaction dependencies
///              u64                                ; lamport version
///              (vector changed-object)
///              (vector unchanged-shared-object)
///              (option digest)                    ; auxiliary data digest
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct TransactionEffectsV2 {
    /// The status of the execution
    pub status: ExecutionStatus,

    /// The epoch when this transaction was executed.
    pub epoch: EpochId,

    /// The gas used by this transaction
    pub gas_used: GasCostSummary,

    /// The transaction digest
    pub transaction_digest: Digest,

    /// The updated gas object reference, as an index into the `changed_objects` vector.
    /// Having a dedicated field for convenient access.
    /// System transaction that don't require gas will leave this as None.
    pub gas_object_index: Option<u32>,

    /// The digest of the events emitted during execution,
    /// can be None if the transaction does not emit any event.
    pub events_digest: Option<Digest>,

    /// The set of transaction digests this transaction depends on.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub dependencies: Vec<Digest>,

    /// The version number of all the written Move objects by this transaction.
    pub lamport_version: Version,

    /// Objects whose state are changed in the object store.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub changed_objects: Vec<ChangedObject>,

    /// Shared objects that are not mutated in this transaction. Unlike owned objects,
    /// read-only shared objects' version are not committed in the transaction,
    /// and in order for a node to catch up and execute it without consensus sequencing,
    /// the version needs to be committed in the effects.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub unchanged_shared_objects: Vec<UnchangedSharedObject>,

    /// Auxiliary data that are not protocol-critical, generated as part of the effects but are stored separately.
    /// Storing it separately allows us to avoid bloating the effects with data that are not critical.
    /// It also provides more flexibility on the format and type of the data.
    pub auxiliary_data_digest: Option<Digest>,
}

/// Input/output state of an object that was changed during execution
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// changed-object = address object-in object-out id-operation
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ChangedObject {
    /// Id of the object
    pub object_id: Address,

    /// State of the object in the store prior to this transaction.
    pub input_state: ObjectIn,

    /// State of the object in the store after this transaction.
    pub output_state: ObjectOut,

    /// Whether this object ID is created or deleted in this transaction.
    /// This information isn't required by the protocol but is useful for providing more detailed
    /// semantics on object changes.
    pub id_operation: IdOperation,
}

/// A shared object that wasn't changed during execution
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// unchanged-shared-object = address unchanged-shared-object-kind
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct UnchangedSharedObject {
    pub object_id: Address,
    pub kind: UnchangedSharedKind,
}

/// Type of unchanged shared object
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// unchanged-shared-object-kind =  read-only-root
///                              =/ mutate-deleted
///                              =/ read-deleted
///                              =/ canceled
///                              =/ per-epoch-config
///
/// read-only-root                           = %x00 u64 digest
/// mutate-deleted                           = %x01 u64
/// read-deleted                             = %x02 u64
/// canceled                                 = %x03 u64
/// per-epoch-config                         = %x04
/// per-epoch-config-with-sequence-number    = %x05 u64
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum UnchangedSharedKind {
    /// Read-only shared objects from the input. We don't really need ObjectDigest
    /// for protocol correctness, but it will make it easier to verify untrusted read.
    ReadOnlyRoot { version: Version, digest: Digest },

    /// Deleted shared objects that appear mutably/owned in the input.
    MutateDeleted { version: Version },

    /// Deleted shared objects that appear as read-only in the input.
    ReadDeleted { version: Version },

    /// Shared objects in canceled transaction. The sequence number embed cancellation reason.
    Canceled { version: Version },

    /// Read of a per-epoch config object that should remain the same during an epoch.
    /// NOTE: Will be deprecated in the near future in favor of `PerEpochConfigWithSequenceNumber`.
    PerEpochConfig,

    /// Read of a per-epoch config and it's starting sequence number in the epoch.
    PerEpochConfigWithSequenceNumber { version: Version },
}

/// State of an object prior to execution
///
/// If an object exists (at root-level) in the store prior to this transaction,
/// it should be Exist, otherwise it's NonExist, e.g. wrapped objects should be
/// NonExist.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// object-in = object-in-not-exist / object-in-exist
///
/// object-in-not-exist = %x00
/// object-in-exist     = %x01 u64 digest owner
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum ObjectIn {
    NotExist,

    /// The old version, digest and owner.
    Exist {
        version: Version,
        digest: Digest,
        owner: Owner,
    },
}

/// State of an object after execution
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// object-out  =  object-out-not-exist
///             =/ object-out-object-write
///             =/ object-out-package-write
///
///
/// object-out-not-exist        = %x00
/// object-out-object-write     = %x01 digest owner
/// object-out-package-write    = %x02 version digest
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum ObjectOut {
    /// Same definition as in ObjectIn.
    NotExist,

    /// Any written object, including all of mutated, created, unwrapped today.
    ObjectWrite { digest: Digest, owner: Owner },

    /// Packages writes need to be tracked separately with version because
    /// we don't use lamport version for package publish and upgrades.
    PackageWrite { version: Version, digest: Digest },
}

/// Defines what happened to an ObjectId during execution
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// id-operation =  id-operation-none
///              =/ id-operation-created
///              =/ id-operation-deleted
///
/// id-operation-none       = %x00
/// id-operation-created    = %x01
/// id-operation-deleted    = %x02
/// ```
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum IdOperation {
    None,
    Created,
    Deleted,
}

impl TransactionEffectsV2 {
    /// The status of the execution
    pub fn status(&self) -> &ExecutionStatus {
        &self.status
    }

    /// The epoch when this transaction was executed.
    pub fn epoch(&self) -> EpochId {
        self.epoch
    }

    /// The gas used in this transaction.
    pub fn gas_summary(&self) -> &GasCostSummary {
        &self.gas_used
    }
}
