use crate::execution_status::ExecutionStatus;
use crate::object::Owner;
use crate::object::Version;
use crate::EpochId;
use crate::GasCostSummary;
use crate::ObjectId;
use crate::ObjectReference;
use crate::TransactionDigest;
use crate::TransactionEventsDigest;

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
///              (vector object-ref)            ; shared object references
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

    /// The object references of the shared objects used in this transaction. Empty if no shared objects were used.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub shared_objects: Vec<ObjectReference>,

    /// The transaction digest
    pub transaction_digest: TransactionDigest,

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
    pub events_digest: Option<TransactionEventsDigest>,

    /// The set of transaction digests this transaction depends on.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub dependencies: Vec<TransactionDigest>,
}

/// Indicates that an Object was modified at a specific version
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// modified-at-version = object-id u64
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ModifiedAtVersion {
    pub object_id: ObjectId,
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
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

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;

    use super::*;

    #[derive(serde_derive::Serialize)]
    struct ReadableTransactionEffectsV1Ref<'a> {
        #[serde(flatten)]
        status: &'a ExecutionStatus,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        epoch: &'a EpochId,
        gas_used: &'a GasCostSummary,
        modified_at_versions: &'a Vec<ModifiedAtVersion>,
        shared_objects: &'a Vec<ObjectReference>,
        transaction_digest: &'a TransactionDigest,
        created: &'a Vec<ObjectReferenceWithOwner>,
        mutated: &'a Vec<ObjectReferenceWithOwner>,
        unwrapped: &'a Vec<ObjectReferenceWithOwner>,
        deleted: &'a Vec<ObjectReference>,
        unwrapped_then_deleted: &'a Vec<ObjectReference>,
        wrapped: &'a Vec<ObjectReference>,
        gas_object: &'a ObjectReferenceWithOwner,
        events_digest: &'a Option<TransactionEventsDigest>,
        dependencies: &'a Vec<TransactionDigest>,
    }

    #[derive(serde_derive::Deserialize)]
    struct ReadableTransactionEffectsV1 {
        #[serde(flatten)]
        status: ExecutionStatus,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        epoch: EpochId,
        gas_used: GasCostSummary,
        modified_at_versions: Vec<ModifiedAtVersion>,
        shared_objects: Vec<ObjectReference>,
        transaction_digest: TransactionDigest,
        created: Vec<ObjectReferenceWithOwner>,
        mutated: Vec<ObjectReferenceWithOwner>,
        unwrapped: Vec<ObjectReferenceWithOwner>,
        deleted: Vec<ObjectReference>,
        unwrapped_then_deleted: Vec<ObjectReference>,
        wrapped: Vec<ObjectReference>,
        gas_object: ObjectReferenceWithOwner,
        events_digest: Option<TransactionEventsDigest>,
        dependencies: Vec<TransactionDigest>,
    }

    #[derive(serde_derive::Serialize)]
    struct BinaryTransactionEffectsV1Ref<'a> {
        status: &'a ExecutionStatus,
        epoch: &'a EpochId,
        gas_used: &'a GasCostSummary,
        modified_at_versions: &'a Vec<ModifiedAtVersion>,
        shared_objects: &'a Vec<ObjectReference>,
        transaction_digest: &'a TransactionDigest,
        created: &'a Vec<ObjectReferenceWithOwner>,
        mutated: &'a Vec<ObjectReferenceWithOwner>,
        unwrapped: &'a Vec<ObjectReferenceWithOwner>,
        deleted: &'a Vec<ObjectReference>,
        unwrapped_then_deleted: &'a Vec<ObjectReference>,
        wrapped: &'a Vec<ObjectReference>,
        gas_object: &'a ObjectReferenceWithOwner,
        events_digest: &'a Option<TransactionEventsDigest>,
        dependencies: &'a Vec<TransactionDigest>,
    }
    #[derive(serde_derive::Deserialize)]
    struct BinaryTransactionEffectsV1 {
        status: ExecutionStatus,
        epoch: EpochId,
        gas_used: GasCostSummary,
        modified_at_versions: Vec<ModifiedAtVersion>,
        shared_objects: Vec<ObjectReference>,
        transaction_digest: TransactionDigest,
        created: Vec<ObjectReferenceWithOwner>,
        mutated: Vec<ObjectReferenceWithOwner>,
        unwrapped: Vec<ObjectReferenceWithOwner>,
        deleted: Vec<ObjectReference>,
        unwrapped_then_deleted: Vec<ObjectReference>,
        wrapped: Vec<ObjectReference>,
        gas_object: ObjectReferenceWithOwner,
        events_digest: Option<TransactionEventsDigest>,
        dependencies: Vec<TransactionDigest>,
    }

    impl Serialize for TransactionEffectsV1 {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let Self {
                status,
                epoch,
                gas_used,
                modified_at_versions,
                shared_objects,
                transaction_digest,
                created,
                mutated,
                unwrapped,
                deleted,
                unwrapped_then_deleted,
                wrapped,
                gas_object,
                events_digest,
                dependencies,
            } = self;
            if serializer.is_human_readable() {
                let readable = ReadableTransactionEffectsV1Ref {
                    status,
                    epoch,
                    gas_used,
                    modified_at_versions,
                    shared_objects,
                    transaction_digest,
                    created,
                    mutated,
                    unwrapped,
                    deleted,
                    unwrapped_then_deleted,
                    wrapped,
                    gas_object,
                    events_digest,
                    dependencies,
                };
                readable.serialize(serializer)
            } else {
                let binary = BinaryTransactionEffectsV1Ref {
                    status,
                    epoch,
                    gas_used,
                    modified_at_versions,
                    shared_objects,
                    transaction_digest,
                    created,
                    mutated,
                    unwrapped,
                    deleted,
                    unwrapped_then_deleted,
                    wrapped,
                    gas_object,
                    events_digest,
                    dependencies,
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for TransactionEffectsV1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let ReadableTransactionEffectsV1 {
                    status,
                    epoch,
                    gas_used,
                    modified_at_versions,
                    shared_objects,
                    transaction_digest,
                    created,
                    mutated,
                    unwrapped,
                    deleted,
                    unwrapped_then_deleted,
                    wrapped,
                    gas_object,
                    events_digest,
                    dependencies,
                } = Deserialize::deserialize(deserializer)?;
                Ok(Self {
                    status,
                    epoch,
                    gas_used,
                    modified_at_versions,
                    shared_objects,
                    transaction_digest,
                    created,
                    mutated,
                    unwrapped,
                    deleted,
                    unwrapped_then_deleted,
                    wrapped,
                    gas_object,
                    events_digest,
                    dependencies,
                })
            } else {
                let BinaryTransactionEffectsV1 {
                    status,
                    epoch,
                    gas_used,
                    modified_at_versions,
                    shared_objects,
                    transaction_digest,
                    created,
                    mutated,
                    unwrapped,
                    deleted,
                    unwrapped_then_deleted,
                    wrapped,
                    gas_object,
                    events_digest,
                    dependencies,
                } = Deserialize::deserialize(deserializer)?;
                Ok(Self {
                    status,
                    epoch,
                    gas_used,
                    modified_at_versions,
                    shared_objects,
                    transaction_digest,
                    created,
                    mutated,
                    unwrapped,
                    deleted,
                    unwrapped_then_deleted,
                    wrapped,
                    gas_object,
                    events_digest,
                    dependencies,
                })
            }
        }
    }
}
