use crate::types::{
    digest::EffectsAuxiliaryDataDigest,
    execution_status::ExecutionStatus,
    object::{Owner, Version},
    EpochId, GasCostSummary, ObjectDigest, ObjectId, TransactionDigest, TransactionEventsDigest,
};

/// The response from processing a transaction or a certified transaction
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TransactionEffectsV2 {
    /// The status of the execution
    status: ExecutionStatus,
    /// The epoch when this transaction was executed.
    epoch: EpochId,
    gas_used: GasCostSummary,
    /// The transaction digest
    transaction_digest: TransactionDigest,
    /// The updated gas object reference, as an index into the `changed_objects` vector.
    /// Having a dedicated field for convenient access.
    /// System transaction that don't require gas will leave this as None.
    gas_object_index: Option<u32>,
    /// The digest of the events emitted during execution,
    /// can be None if the transaction does not emit any event.
    events_digest: Option<TransactionEventsDigest>,
    /// The set of transaction digests this transaction depends on.
    dependencies: Vec<TransactionDigest>,

    /// The version number of all the written Move objects by this transaction.
    lamport_version: Version,
    /// Objects whose state are changed in the object store.
    changed_objects: Vec<ChangedObject>,
    /// Shared objects that are not mutated in this transaction. Unlike owned objects,
    /// read-only shared objects' version are not committed in the transaction,
    /// and in order for a node to catch up and execute it without consensus sequencing,
    /// the version needs to be committed in the effects.
    unchanged_shared_objects: Vec<UnchangedSharedObject>,
    /// Auxiliary data that are not protocol-critical, generated as part of the effects but are stored separately.
    /// Storing it separately allows us to avoid bloating the effects with data that are not critical.
    /// It also provides more flexibility on the format and type of the data.
    auxiliary_data_digest: Option<EffectsAuxiliaryDataDigest>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub struct ChangedObject {
    object_id: ObjectId,
    change: EffectsObjectChange,
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub struct UnchangedSharedObject {
    object_id: ObjectId,
    kind: UnchangedSharedKind,
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum UnchangedSharedKind {
    /// Read-only shared objects from the input. We don't really need ObjectDigest
    /// for protocol correctness, but it will make it easier to verify untrusted read.
    ReadOnlyRoot {
        version: Version,
        digest: ObjectDigest,
    },
    /// Deleted shared objects that appear mutably/owned in the input.
    MutateDeleted { version: Version },
    /// Deleted shared objects that appear as read-only in the input.
    ReadDeleted { version: Version },
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[allow(dead_code)]
pub struct EffectsObjectChange {
    // input_state and output_state are the core fields that's required by
    // the protocol as it tells how an object changes on-chain.
    /// State of the object in the store prior to this transaction.
    pub(crate) input_state: ObjectIn,
    /// State of the object in the store after this transaction.
    pub(crate) output_state: ObjectOut,

    /// Whether this object ID is created or deleted in this transaction.
    /// This information isn't required by the protocol but is useful for providing more detailed
    /// semantics on object changes.
    pub(crate) id_operation: IdOperation,
}

/// If an object exists (at root-level) in the store prior to this transaction,
/// it should be Exist, otherwise it's NonExist, e.g. wrapped objects should be
/// NonExist.
#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum ObjectIn {
    NotExist,
    /// The old version, digest and owner.
    Exist {
        version: Version,
        digest: ObjectDigest,
        owner: Owner,
    },
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum ObjectOut {
    /// Same definition as in ObjectIn.
    NotExist,
    /// Any written object, including all of mutated, created, unwrapped today.
    ObjectWrite { digest: ObjectDigest, owner: Owner },
    /// Packages writes need to be tracked separately with version because
    /// we don't use lamport version for package publish and upgrades.
    PackageWrite {
        version: Version,
        digest: ObjectDigest,
    },
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "lowercase")
)]
#[allow(dead_code)]
pub enum IdOperation {
    None,
    Created,
    Deleted,
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::*;

    #[derive(serde_derive::Serialize)]
    struct ReadableTransactionEffectsV2Ref<'a> {
        #[serde(flatten)]
        status: &'a ExecutionStatus,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        epoch: &'a EpochId,
        gas_used: &'a GasCostSummary,
        transaction_digest: &'a TransactionDigest,
        gas_object_index: &'a Option<u32>,
        events_digest: &'a Option<TransactionEventsDigest>,
        dependencies: &'a Vec<TransactionDigest>,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        lamport_version: &'a Version,
        changed_objects: &'a Vec<ChangedObject>,
        unchanged_shared_objects: &'a Vec<UnchangedSharedObject>,
        auxiliary_data_digest: &'a Option<EffectsAuxiliaryDataDigest>,
    }

    #[derive(serde_derive::Deserialize)]
    struct ReadableTransactionEffectsV2 {
        #[serde(flatten)]
        status: ExecutionStatus,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        epoch: EpochId,
        gas_used: GasCostSummary,
        transaction_digest: TransactionDigest,
        gas_object_index: Option<u32>,
        events_digest: Option<TransactionEventsDigest>,
        dependencies: Vec<TransactionDigest>,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        lamport_version: Version,
        changed_objects: Vec<ChangedObject>,
        unchanged_shared_objects: Vec<UnchangedSharedObject>,
        auxiliary_data_digest: Option<EffectsAuxiliaryDataDigest>,
    }

    #[derive(serde_derive::Serialize)]
    struct BinaryTransactionEffectsV2Ref<'a> {
        status: &'a ExecutionStatus,
        epoch: &'a EpochId,
        gas_used: &'a GasCostSummary,
        transaction_digest: &'a TransactionDigest,
        gas_object_index: &'a Option<u32>,
        events_digest: &'a Option<TransactionEventsDigest>,
        dependencies: &'a Vec<TransactionDigest>,
        lamport_version: &'a Version,
        changed_objects: &'a Vec<ChangedObject>,
        unchanged_shared_objects: &'a Vec<UnchangedSharedObject>,
        auxiliary_data_digest: &'a Option<EffectsAuxiliaryDataDigest>,
    }

    #[derive(serde_derive::Deserialize)]
    struct BinaryTransactionEffectsV2 {
        status: ExecutionStatus,
        epoch: EpochId,
        gas_used: GasCostSummary,
        transaction_digest: TransactionDigest,
        gas_object_index: Option<u32>,
        events_digest: Option<TransactionEventsDigest>,
        dependencies: Vec<TransactionDigest>,
        lamport_version: Version,
        changed_objects: Vec<ChangedObject>,
        unchanged_shared_objects: Vec<UnchangedSharedObject>,
        auxiliary_data_digest: Option<EffectsAuxiliaryDataDigest>,
    }

    impl Serialize for TransactionEffectsV2 {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let Self {
                status,
                epoch,
                gas_used,
                transaction_digest,
                gas_object_index,
                events_digest,
                dependencies,
                lamport_version,
                changed_objects,
                unchanged_shared_objects,
                auxiliary_data_digest,
            } = self;
            if serializer.is_human_readable() {
                let readable = ReadableTransactionEffectsV2Ref {
                    status,
                    epoch,
                    gas_used,
                    transaction_digest,
                    gas_object_index,
                    events_digest,
                    dependencies,
                    lamport_version,
                    changed_objects,
                    unchanged_shared_objects,
                    auxiliary_data_digest,
                };
                readable.serialize(serializer)
            } else {
                let binary = BinaryTransactionEffectsV2Ref {
                    status,
                    epoch,
                    gas_used,
                    transaction_digest,
                    gas_object_index,
                    events_digest,
                    dependencies,
                    lamport_version,
                    changed_objects,
                    unchanged_shared_objects,
                    auxiliary_data_digest,
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for TransactionEffectsV2 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let ReadableTransactionEffectsV2 {
                    status,
                    epoch,
                    gas_used,
                    transaction_digest,
                    gas_object_index,
                    events_digest,
                    dependencies,
                    lamport_version,
                    changed_objects,
                    unchanged_shared_objects,
                    auxiliary_data_digest,
                } = Deserialize::deserialize(deserializer)?;
                Ok(Self {
                    status,
                    epoch,
                    gas_used,
                    transaction_digest,
                    gas_object_index,
                    events_digest,
                    dependencies,
                    lamport_version,
                    changed_objects,
                    unchanged_shared_objects,
                    auxiliary_data_digest,
                })
            } else {
                let BinaryTransactionEffectsV2 {
                    status,
                    epoch,
                    gas_used,
                    transaction_digest,
                    gas_object_index,
                    events_digest,
                    dependencies,
                    lamport_version,
                    changed_objects,
                    unchanged_shared_objects,
                    auxiliary_data_digest,
                } = Deserialize::deserialize(deserializer)?;
                Ok(Self {
                    status,
                    epoch,
                    gas_used,
                    transaction_digest,
                    gas_object_index,
                    events_digest,
                    dependencies,
                    lamport_version,
                    changed_objects,
                    unchanged_shared_objects,
                    auxiliary_data_digest,
                })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    struct ReadableChangedObject {
        object_id: ObjectId,
        #[serde(flatten)]
        change: EffectsObjectChange,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    struct BinaryChangedObject {
        object_id: ObjectId,
        change: EffectsObjectChange,
    }

    impl Serialize for ChangedObject {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let Self { object_id, change } = self.clone();
            if serializer.is_human_readable() {
                let readable = ReadableChangedObject { object_id, change };
                readable.serialize(serializer)
            } else {
                let binary = BinaryChangedObject { object_id, change };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for ChangedObject {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let ReadableChangedObject { object_id, change } =
                    Deserialize::deserialize(deserializer)?;
                Ok(Self { object_id, change })
            } else {
                let BinaryChangedObject { object_id, change } =
                    Deserialize::deserialize(deserializer)?;
                Ok(Self { object_id, change })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    struct ReadableUnchangedSharedObject {
        object_id: ObjectId,
        #[serde(flatten)]
        kind: UnchangedSharedKind,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    struct BinaryUnchangedSharedObject {
        object_id: ObjectId,
        kind: UnchangedSharedKind,
    }

    impl Serialize for UnchangedSharedObject {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let Self { object_id, kind } = self.clone();
            if serializer.is_human_readable() {
                let readable = ReadableUnchangedSharedObject { object_id, kind };
                readable.serialize(serializer)
            } else {
                let binary = BinaryUnchangedSharedObject { object_id, kind };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for UnchangedSharedObject {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let ReadableUnchangedSharedObject { object_id, kind } =
                    Deserialize::deserialize(deserializer)?;
                Ok(Self { object_id, kind })
            } else {
                let BinaryUnchangedSharedObject { object_id, kind } =
                    Deserialize::deserialize(deserializer)?;
                Ok(Self { object_id, kind })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "kind", rename_all = "kebab-case")]
    enum ReadableUnchangedSharedKind {
        ReadOnlyRoot {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
            digest: ObjectDigest,
        },
        MutateDeleted {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
        },
        ReadDeleted {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
        },
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum BinaryUnchangedSharedKind {
        ReadOnlyRoot {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
            digest: ObjectDigest,
        },
        MutateDeleted {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
        },
        ReadDeleted {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
        },
    }

    impl Serialize for UnchangedSharedKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self.clone() {
                    UnchangedSharedKind::ReadOnlyRoot { version, digest } => {
                        ReadableUnchangedSharedKind::ReadOnlyRoot { version, digest }
                    }
                    UnchangedSharedKind::MutateDeleted { version } => {
                        ReadableUnchangedSharedKind::MutateDeleted { version }
                    }
                    UnchangedSharedKind::ReadDeleted { version } => {
                        ReadableUnchangedSharedKind::ReadDeleted { version }
                    }
                };
                readable.serialize(serializer)
            } else {
                let binary = match self.clone() {
                    UnchangedSharedKind::ReadOnlyRoot { version, digest } => {
                        BinaryUnchangedSharedKind::ReadOnlyRoot { version, digest }
                    }
                    UnchangedSharedKind::MutateDeleted { version } => {
                        BinaryUnchangedSharedKind::MutateDeleted { version }
                    }
                    UnchangedSharedKind::ReadDeleted { version } => {
                        BinaryUnchangedSharedKind::ReadDeleted { version }
                    }
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for UnchangedSharedKind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableUnchangedSharedKind::deserialize(deserializer).map(
                    |readable| match readable {
                        ReadableUnchangedSharedKind::ReadOnlyRoot { version, digest } => {
                            Self::ReadOnlyRoot { version, digest }
                        }
                        ReadableUnchangedSharedKind::MutateDeleted { version } => {
                            Self::MutateDeleted { version }
                        }
                        ReadableUnchangedSharedKind::ReadDeleted { version } => {
                            Self::ReadDeleted { version }
                        }
                    },
                )
            } else {
                BinaryUnchangedSharedKind::deserialize(deserializer).map(|binary| match binary {
                    BinaryUnchangedSharedKind::ReadOnlyRoot { version, digest } => {
                        Self::ReadOnlyRoot { version, digest }
                    }
                    BinaryUnchangedSharedKind::MutateDeleted { version } => {
                        Self::MutateDeleted { version }
                    }
                    BinaryUnchangedSharedKind::ReadDeleted { version } => {
                        Self::ReadDeleted { version }
                    }
                })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "state", rename_all = "kebab-case")]
    enum ReadableObjectIn {
        NotExist,
        Exist {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
            digest: ObjectDigest,
            owner: Owner,
        },
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum BinaryObjectIn {
        NotExist,
        Exist {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
            digest: ObjectDigest,
            owner: Owner,
        },
    }

    impl Serialize for ObjectIn {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self.clone() {
                    ObjectIn::NotExist => ReadableObjectIn::NotExist,
                    ObjectIn::Exist {
                        version,
                        digest,
                        owner,
                    } => ReadableObjectIn::Exist {
                        version,
                        digest,
                        owner,
                    },
                };
                readable.serialize(serializer)
            } else {
                let binary = match self.clone() {
                    ObjectIn::NotExist => BinaryObjectIn::NotExist,
                    ObjectIn::Exist {
                        version,
                        digest,
                        owner,
                    } => BinaryObjectIn::Exist {
                        version,
                        digest,
                        owner,
                    },
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for ObjectIn {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableObjectIn::deserialize(deserializer).map(|readable| match readable {
                    ReadableObjectIn::NotExist => Self::NotExist,
                    ReadableObjectIn::Exist {
                        version,
                        digest,
                        owner,
                    } => Self::Exist {
                        version,
                        digest,
                        owner,
                    },
                })
            } else {
                BinaryObjectIn::deserialize(deserializer).map(|binary| match binary {
                    BinaryObjectIn::NotExist => Self::NotExist,
                    BinaryObjectIn::Exist {
                        version,
                        digest,
                        owner,
                    } => Self::Exist {
                        version,
                        digest,
                        owner,
                    },
                })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "state", rename_all = "kebab-case")]
    enum ReadableObjectOut {
        NotExist,
        ObjectWrite {
            digest: ObjectDigest,
            owner: Owner,
        },
        PackageWrite {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
            digest: ObjectDigest,
        },
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum BinaryObjectOut {
        NotExist,
        ObjectWrite {
            digest: ObjectDigest,
            owner: Owner,
        },
        PackageWrite {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
            digest: ObjectDigest,
        },
    }

    impl Serialize for ObjectOut {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self.clone() {
                    ObjectOut::NotExist => ReadableObjectOut::NotExist,
                    ObjectOut::ObjectWrite { digest, owner } => {
                        ReadableObjectOut::ObjectWrite { digest, owner }
                    }
                    ObjectOut::PackageWrite { version, digest } => {
                        ReadableObjectOut::PackageWrite { version, digest }
                    }
                };
                readable.serialize(serializer)
            } else {
                let binary = match self.clone() {
                    ObjectOut::NotExist => BinaryObjectOut::NotExist,
                    ObjectOut::ObjectWrite { digest, owner } => {
                        BinaryObjectOut::ObjectWrite { digest, owner }
                    }
                    ObjectOut::PackageWrite { version, digest } => {
                        BinaryObjectOut::PackageWrite { version, digest }
                    }
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for ObjectOut {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableObjectOut::deserialize(deserializer).map(|readable| match readable {
                    ReadableObjectOut::NotExist => Self::NotExist,
                    ReadableObjectOut::ObjectWrite { digest, owner } => {
                        Self::ObjectWrite { digest, owner }
                    }
                    ReadableObjectOut::PackageWrite { version, digest } => {
                        Self::PackageWrite { version, digest }
                    }
                })
            } else {
                BinaryObjectOut::deserialize(deserializer).map(|binary| match binary {
                    BinaryObjectOut::NotExist => Self::NotExist,
                    BinaryObjectOut::ObjectWrite { digest, owner } => {
                        Self::ObjectWrite { digest, owner }
                    }
                    BinaryObjectOut::PackageWrite { version, digest } => {
                        Self::PackageWrite { version, digest }
                    }
                })
            }
        }
    }
}
