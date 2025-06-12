use crate::digest::EffectsAuxiliaryDataDigest;
use crate::execution_status::ExecutionStatus;
use crate::object::Owner;
use crate::object::Version;
use crate::EpochId;
use crate::GasCostSummary;
use crate::ObjectDigest;
use crate::ObjectId;
use crate::TransactionDigest;
use crate::TransactionEventsDigest;

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
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct TransactionEffectsV2 {
    /// The status of the execution
    pub status: ExecutionStatus,

    /// The epoch when this transaction was executed.
    pub epoch: EpochId,

    /// The gas used by this transaction
    pub gas_used: GasCostSummary,

    /// The transaction digest
    pub transaction_digest: TransactionDigest,

    /// The updated gas object reference, as an index into the `changed_objects` vector.
    /// Having a dedicated field for convenient access.
    /// System transaction that don't require gas will leave this as None.
    pub gas_object_index: Option<u32>,

    /// The digest of the events emitted during execution,
    /// can be None if the transaction does not emit any event.
    pub events_digest: Option<TransactionEventsDigest>,

    /// The set of transaction digests this transaction depends on.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=5).lift()))]
    pub dependencies: Vec<TransactionDigest>,

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
    pub auxiliary_data_digest: Option<EffectsAuxiliaryDataDigest>,
}

/// Input/output state of an object that was changed during execution
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// changed-object = object-id object-in object-out id-operation
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ChangedObject {
    /// Id of the object
    pub object_id: ObjectId,

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
/// unchanged-shared-object = object-id unchanged-shared-object-kind
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct UnchangedSharedObject {
    pub object_id: ObjectId,
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
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
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
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum ObjectIn {
    NotExist,

    /// The old version, digest and owner.
    Exist {
        version: Version,
        digest: ObjectDigest,
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
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
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
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "lowercase")
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

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;

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
    #[serde(tag = "kind", rename_all = "snake_case")]
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
        Canceled {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
        },
        PerEpochConfig,
        PerEpochConfigWithSequenceNumber {
            #[serde(with = "crate::_serde::ReadableDisplay")]
            version: Version,
        },
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum BinaryUnchangedSharedKind {
        ReadOnlyRoot {
            version: Version,
            digest: ObjectDigest,
        },
        MutateDeleted {
            version: Version,
        },
        ReadDeleted {
            version: Version,
        },
        Canceled {
            version: Version,
        },
        PerEpochConfig,
        PerEpochConfigWithSequenceNumber {
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
                    UnchangedSharedKind::Canceled { version } => {
                        ReadableUnchangedSharedKind::Canceled { version }
                    }
                    UnchangedSharedKind::PerEpochConfig => {
                        ReadableUnchangedSharedKind::PerEpochConfig
                    }
                    UnchangedSharedKind::PerEpochConfigWithSequenceNumber { version } => {
                        ReadableUnchangedSharedKind::PerEpochConfigWithSequenceNumber { version }
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
                    UnchangedSharedKind::Canceled { version } => {
                        BinaryUnchangedSharedKind::Canceled { version }
                    }
                    UnchangedSharedKind::PerEpochConfig => {
                        BinaryUnchangedSharedKind::PerEpochConfig
                    }
                    UnchangedSharedKind::PerEpochConfigWithSequenceNumber { version } => {
                        BinaryUnchangedSharedKind::PerEpochConfigWithSequenceNumber { version }
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
                        ReadableUnchangedSharedKind::Canceled { version } => {
                            Self::Canceled { version }
                        }
                        ReadableUnchangedSharedKind::PerEpochConfig => Self::PerEpochConfig,
                        ReadableUnchangedSharedKind::PerEpochConfigWithSequenceNumber {
                            version,
                        } => Self::PerEpochConfigWithSequenceNumber { version },
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
                    BinaryUnchangedSharedKind::Canceled { version } => Self::Canceled { version },
                    BinaryUnchangedSharedKind::PerEpochConfig => Self::PerEpochConfig,
                    BinaryUnchangedSharedKind::PerEpochConfigWithSequenceNumber { version } => {
                        Self::PerEpochConfigWithSequenceNumber { version }
                    }
                })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "state", rename_all = "snake_case")]
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
    #[serde(tag = "state", rename_all = "snake_case")]
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
