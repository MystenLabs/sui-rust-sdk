use crate::Address;
use crate::Digest;
use crate::EpochId;
use crate::GasCostSummary;
use crate::ObjectReference;
use crate::execution_status::ExecutionStatus;
use crate::object::Owner;
use crate::object::Version;

/// Version 1 of TransactionEffects
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// effects-v1 = execution-status
///              u64                            ; epoch
///              gas-cost-summary
///              (vector modified-at-version)
///              (vector object-ref)            ; consensus object references
///              digest                         ; transaction digest
///              (vector object-ref-with-owner) ; created objects
///              (vector object-ref-with-owner) ; mutated objects
///              (vector object-ref-with-owner) ; unwrapped objects
///              (vector object-ref)            ; deleted objects
///              (vector object-ref)            ; unwrapped then deleted objects
///              (vector object-ref)            ; wrapped objects
///              object-ref-with-owner          ; gas object
///              (option digest)                ; events digest
///              (vector digest)                ; list of transaction dependencies
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct TransactionEffectsV1 {
    /// The status of the execution
    pub status: ExecutionStatus,

    /// The epoch when this transaction was executed.
    pub epoch: EpochId,

    /// The gas used by this transaction
    pub gas_used: GasCostSummary,

    /// The version that every modified (mutated or deleted) object had before it was modified by
    /// this transaction.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub modified_at_versions: Vec<ModifiedAtVersion>,

    /// The object references of the consensus objects used in this transaction. Empty if no consensus objects were used.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub consensus_objects: Vec<ObjectReference>,

    /// The transaction digest
    pub transaction_digest: Digest,

    /// ObjectReference and owner of new objects created.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub created: Vec<ObjectReferenceWithOwner>,

    /// ObjectReference and owner of mutated objects, including gas object.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub mutated: Vec<ObjectReferenceWithOwner>,

    /// ObjectReference and owner of objects that are unwrapped in this transaction.
    /// Unwrapped objects are objects that were wrapped into other objects in the past,
    /// and just got extracted out.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub unwrapped: Vec<ObjectReferenceWithOwner>,

    /// Object Refs of objects now deleted (the new refs).
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub deleted: Vec<ObjectReference>,

    /// Object refs of objects previously wrapped in other objects but now deleted.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub unwrapped_then_deleted: Vec<ObjectReference>,

    /// Object refs of objects now wrapped in other objects.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub wrapped: Vec<ObjectReference>,

    /// The updated gas object reference. Have a dedicated field for convenient access.
    /// It's also included in mutated.
    pub gas_object: ObjectReferenceWithOwner,

    /// The digest of the events emitted during execution,
    /// can be None if the transaction does not emit any event.
    pub events_digest: Option<Digest>,

    /// The set of transaction digests this transaction depends on.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub dependencies: Vec<Digest>,
}

/// Indicates that an Object was modified at a specific version
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// modified-at-version = address u64
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ModifiedAtVersion {
    pub object_id: Address,
    pub version: Version,
}

/// An object reference with owner information
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// object-ref-with-owner = object-ref owner
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ObjectReferenceWithOwner {
    pub reference: ObjectReference,
    pub owner: Owner,
}

impl TransactionEffectsV1 {
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
