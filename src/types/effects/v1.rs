use crate::types::{
    execution_status::ExecutionStatus,
    object::{Owner, Version},
    EpochId, GasCostSummary, ObjectId, ObjectReference, TransactionDigest, TransactionEventsDigest,
};

/// The response from processing a transaction or a certified transaction
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct TransactionEffectsV1 {
    /// The status of the execution
    status: ExecutionStatus,
    /// The epoch when this transaction was executed.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    executed_epoch: EpochId,
    gas_used: GasCostSummary,
    /// The version that every modified (mutated or deleted) object had before it was modified by
    /// this transaction.
    modified_at_versions: Vec<ModifiedAtVersion>,
    /// The object references of the shared objects used in this transaction. Empty if no shared objects were used.
    shared_objects: Vec<ObjectReference>,
    /// The transaction digest
    transaction_digest: TransactionDigest,

    /// ObjectReference and owner of new objects created.
    created: Vec<ObjectReferenceWithOwner>,
    /// ObjectReference and owner of mutated objects, including gas object.
    mutated: Vec<ObjectReferenceWithOwner>,
    /// ObjectReference and owner of objects that are unwrapped in this transaction.
    /// Unwrapped objects are objects that were wrapped into other objects in the past,
    /// and just got extracted out.
    unwrapped: Vec<ObjectReferenceWithOwner>,
    /// Object Refs of objects now deleted (the new refs).
    deleted: Vec<ObjectReference>,
    /// Object refs of objects previously wrapped in other objects but now deleted.
    unwrapped_then_deleted: Vec<ObjectReference>,
    /// Object refs of objects now wrapped in other objects.
    wrapped: Vec<ObjectReference>,
    /// The updated gas object reference. Have a dedicated field for convenient access.
    /// It's also included in mutated.
    gas_object: ObjectReferenceWithOwner,
    /// The digest of the events emitted during execution,
    /// can be None if the transaction does not emit any event.
    events_digest: Option<TransactionEventsDigest>,
    /// The set of transaction digests this transaction depends on.
    dependencies: Vec<TransactionDigest>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[allow(dead_code)]
pub struct ModifiedAtVersion {
    object_id: ObjectId,
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    version: Version,
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub struct ObjectReferenceWithOwner {
    object_ref: ObjectReference,
    owner: Owner,
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::*;

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    struct ReadableObjectReferenceWithOwner {
        #[serde(flatten)]
        object_ref: ObjectReference,
        owner: Owner,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    struct BinaryObjectReferenceWithOwner {
        object_ref: ObjectReference,
        owner: Owner,
    }

    impl Serialize for ObjectReferenceWithOwner {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let Self { object_ref, owner } = self.clone();
            if serializer.is_human_readable() {
                let readable = ReadableObjectReferenceWithOwner { object_ref, owner };
                readable.serialize(serializer)
            } else {
                let binary = BinaryObjectReferenceWithOwner { object_ref, owner };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for ObjectReferenceWithOwner {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let ReadableObjectReferenceWithOwner { object_ref, owner } =
                    Deserialize::deserialize(deserializer)?;
                Ok(Self { object_ref, owner })
            } else {
                let BinaryObjectReferenceWithOwner { object_ref, owner } =
                    Deserialize::deserialize(deserializer)?;
                Ok(Self { object_ref, owner })
            }
        }
    }
}
