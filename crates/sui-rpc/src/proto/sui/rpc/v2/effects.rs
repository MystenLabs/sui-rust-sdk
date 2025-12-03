use super::*;
use crate::field::FieldMaskTree;
use crate::merge::Merge;
use crate::proto::TryFromProtoError;
use tap::Pipe;

//
// TransactionEffects
//

impl From<sui_sdk_types::TransactionEffects> for TransactionEffects {
    fn from(value: sui_sdk_types::TransactionEffects) -> Self {
        Self::merge_from(&value, &FieldMaskTree::new_wildcard())
    }
}

impl Merge<&sui_sdk_types::TransactionEffects> for TransactionEffects {
    fn merge(&mut self, source: &sui_sdk_types::TransactionEffects, mask: &FieldMaskTree) {
        if mask.contains(Self::BCS_FIELD.name) {
            let mut bcs = Bcs::serialize(&source).unwrap();
            bcs.name = Some("TransactionEffects".to_owned());
            self.bcs = Some(bcs);
        }

        if mask.contains(Self::DIGEST_FIELD.name) {
            self.digest = Some(source.digest().to_string());
        }

        match source {
            sui_sdk_types::TransactionEffects::V1(v1) => self.merge(v1.as_ref(), mask),
            sui_sdk_types::TransactionEffects::V2(v2) => self.merge(v2.as_ref(), mask),
        }
    }
}

impl Merge<&TransactionEffects> for TransactionEffects {
    fn merge(
        &mut self,
        TransactionEffects {
            bcs,
            digest,
            version,
            status,
            epoch,
            gas_used,
            transaction_digest,
            gas_object,
            events_digest,
            dependencies,
            lamport_version,
            changed_objects,
            unchanged_consensus_objects,
            auxiliary_data_digest,
            unchanged_loaded_runtime_objects,
        }: &TransactionEffects,
        mask: &FieldMaskTree,
    ) {
        if mask.contains(Self::BCS_FIELD.name) {
            self.bcs = bcs.clone();
        }

        if mask.contains(Self::DIGEST_FIELD.name) {
            self.digest = digest.clone();
        }
        if mask.contains(Self::VERSION_FIELD.name) {
            self.version = *version;
        }

        if mask.contains(Self::STATUS_FIELD.name) {
            self.status = status.clone();
        }

        if mask.contains(Self::EPOCH_FIELD.name) {
            self.epoch = *epoch;
        }

        if mask.contains(Self::GAS_USED_FIELD.name) {
            self.gas_used = *gas_used;
        }

        if mask.contains(Self::TRANSACTION_DIGEST_FIELD.name) {
            self.transaction_digest = transaction_digest.clone();
        }

        if mask.contains(Self::GAS_OBJECT_FIELD.name) {
            self.gas_object = gas_object.clone();
        }

        if mask.contains(Self::EVENTS_DIGEST_FIELD.name) {
            self.events_digest = events_digest.clone();
        }

        if mask.contains(Self::DEPENDENCIES_FIELD.name) {
            self.dependencies = dependencies.clone();
        }

        if mask.contains(Self::LAMPORT_VERSION_FIELD.name) {
            self.lamport_version = *lamport_version;
        }

        if mask.contains(Self::CHANGED_OBJECTS_FIELD.name) {
            self.changed_objects = changed_objects.clone();
        }

        if mask.contains(Self::UNCHANGED_CONSENSUS_OBJECTS_FIELD.name) {
            self.unchanged_consensus_objects = unchanged_consensus_objects.clone();
        }

        if mask.contains(Self::AUXILIARY_DATA_DIGEST_FIELD.name) {
            self.auxiliary_data_digest = auxiliary_data_digest.clone();
        }

        if mask.contains(Self::UNCHANGED_LOADED_RUNTIME_OBJECTS_FIELD.name) {
            self.unchanged_loaded_runtime_objects = unchanged_loaded_runtime_objects.clone();
        }
    }
}

impl TryFrom<&TransactionEffects> for sui_sdk_types::TransactionEffects {
    type Error = TryFromProtoError;

    fn try_from(value: &TransactionEffects) -> Result<Self, Self::Error> {
        value
            .bcs
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("bcs"))?
            .deserialize()
            .map_err(|e| TryFromProtoError::invalid(TransactionEffects::BCS_FIELD, e))
    }
}

//
// TransactionEffectsV1
//

impl Merge<&sui_sdk_types::TransactionEffectsV1> for TransactionEffects {
    fn merge(
        &mut self,
        sui_sdk_types::TransactionEffectsV1 {
            status,
            epoch,
            gas_used,
            modified_at_versions,
            consensus_objects,
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
        }: &sui_sdk_types::TransactionEffectsV1,
        mask: &FieldMaskTree,
    ) {
        if mask.contains(Self::VERSION_FIELD.name) {
            self.version = Some(1);
        }

        if mask.contains(Self::STATUS_FIELD.name) {
            self.status = Some(status.clone().into());
        }

        if mask.contains(Self::EPOCH_FIELD.name) {
            self.epoch = Some(*epoch);
        }

        if mask.contains(Self::GAS_USED_FIELD.name) {
            self.gas_used = Some(gas_used.clone().into());
        }

        if mask.contains(Self::TRANSACTION_DIGEST_FIELD.name) {
            self.transaction_digest = Some(transaction_digest.to_string());
        }

        if mask.contains(Self::EVENTS_DIGEST_FIELD.name) {
            self.events_digest = events_digest.map(|d| d.to_string());
        }

        if mask.contains(Self::DEPENDENCIES_FIELD.name) {
            self.dependencies = dependencies.iter().map(ToString::to_string).collect();
        }

        if mask.contains(Self::CHANGED_OBJECTS_FIELD.name)
            || mask.contains(Self::UNCHANGED_CONSENSUS_OBJECTS_FIELD.name)
            || mask.contains(Self::GAS_OBJECT_FIELD.name)
        {
            let mut changed_objects = Vec::new();
            let mut unchanged_consensus_objects = Vec::new();

            for object in created {
                let change = ChangedObject {
                    object_id: Some(object.reference.object_id().to_string()),
                    input_state: Some(changed_object::InputObjectState::DoesNotExist.into()),
                    input_version: None,
                    input_digest: None,
                    input_owner: None,
                    output_state: Some(changed_object::OutputObjectState::ObjectWrite.into()),
                    output_version: Some(object.reference.version()),
                    output_digest: Some(object.reference.digest().to_string()),
                    output_owner: Some(object.owner.into()),
                    id_operation: Some(changed_object::IdOperation::Created.into()),
                    object_type: None,
                    accumulator_write: None,
                };

                changed_objects.push(change);
            }

            for object in mutated {
                let change = ChangedObject {
                    object_id: Some(object.reference.object_id().to_string()),
                    input_state: Some(changed_object::InputObjectState::Exists.into()),
                    input_version: None,
                    input_digest: None,
                    input_owner: None,
                    output_state: Some(changed_object::OutputObjectState::ObjectWrite.into()),
                    output_version: Some(object.reference.version()),
                    output_digest: Some(object.reference.digest().to_string()),
                    output_owner: Some(object.owner.into()),
                    id_operation: Some(changed_object::IdOperation::None.into()),
                    object_type: None,
                    accumulator_write: None,
                };

                changed_objects.push(change);
            }

            for object in unwrapped {
                let change = ChangedObject {
                    object_id: Some(object.reference.object_id().to_string()),
                    input_state: Some(changed_object::InputObjectState::DoesNotExist.into()),
                    input_version: None,
                    input_digest: None,
                    input_owner: None,
                    output_state: Some(changed_object::OutputObjectState::ObjectWrite.into()),
                    output_version: Some(object.reference.version()),
                    output_digest: Some(object.reference.digest().to_string()),
                    output_owner: Some(object.owner.into()),
                    id_operation: Some(changed_object::IdOperation::None.into()),
                    object_type: None,
                    accumulator_write: None,
                };

                changed_objects.push(change);
            }

            for object in deleted {
                let change = ChangedObject {
                    object_id: Some(object.object_id().to_string()),
                    input_state: Some(changed_object::InputObjectState::Exists.into()),
                    input_version: None,
                    input_digest: None,
                    input_owner: None,
                    output_state: Some(changed_object::OutputObjectState::DoesNotExist.into()),
                    output_version: Some(object.version()),
                    output_digest: Some(object.digest().to_string()),
                    output_owner: None,
                    id_operation: Some(changed_object::IdOperation::Deleted.into()),
                    object_type: None,
                    accumulator_write: None,
                };

                changed_objects.push(change);
            }

            for object in unwrapped_then_deleted {
                let change = ChangedObject {
                    object_id: Some(object.object_id().to_string()),
                    input_state: Some(changed_object::InputObjectState::DoesNotExist.into()),
                    input_version: None,
                    input_digest: None,
                    input_owner: None,
                    output_state: Some(changed_object::OutputObjectState::DoesNotExist.into()),
                    output_version: Some(object.version()),
                    output_digest: Some(object.digest().to_string()),
                    output_owner: None,
                    id_operation: Some(changed_object::IdOperation::Deleted.into()),
                    object_type: None,
                    accumulator_write: None,
                };

                changed_objects.push(change);
            }

            for object in wrapped {
                let change = ChangedObject {
                    object_id: Some(object.object_id().to_string()),
                    input_state: Some(changed_object::InputObjectState::Exists.into()),
                    input_version: None,
                    input_digest: None,
                    input_owner: None,
                    output_state: Some(changed_object::OutputObjectState::DoesNotExist.into()),
                    output_version: Some(object.version()),
                    output_digest: Some(object.digest().to_string()),
                    output_owner: None,
                    id_operation: Some(changed_object::IdOperation::Deleted.into()),
                    object_type: None,
                    accumulator_write: None,
                };

                changed_objects.push(change);
            }

            for modified_at_version in modified_at_versions {
                let object_id = modified_at_version.object_id.to_string();
                let version = modified_at_version.version;
                if let Some(changed_object) = changed_objects
                    .iter_mut()
                    .find(|object| object.object_id() == object_id)
                {
                    changed_object.input_version = Some(version);
                }
            }

            for object in consensus_objects {
                let object_id = object.object_id().to_string();
                let version = object.version();
                let digest = object.digest().to_string();

                if let Some(changed_object) = changed_objects
                    .iter_mut()
                    .find(|object| object.object_id() == object_id)
                {
                    changed_object.input_version = Some(version);
                    changed_object.input_digest = Some(digest);
                } else {
                    let unchanged_consensus_object = UnchangedConsensusObject {
                        kind: Some(
                            unchanged_consensus_object::UnchangedConsensusObjectKind::ReadOnlyRoot
                                .into(),
                        ),
                        object_id: Some(object_id),
                        version: Some(version),
                        digest: Some(digest),
                        object_type: None,
                    };

                    unchanged_consensus_objects.push(unchanged_consensus_object);
                }
            }

            if mask.contains(Self::GAS_OBJECT_FIELD.name) {
                let gas_object_id = gas_object.reference.object_id().to_string();
                self.gas_object = changed_objects
                    .iter()
                    .find(|object| object.object_id() == gas_object_id)
                    .cloned();
            }

            if mask.contains(Self::CHANGED_OBJECTS_FIELD.name) {
                self.changed_objects = changed_objects;
            }

            if mask.contains(Self::UNCHANGED_CONSENSUS_OBJECTS_FIELD.name) {
                self.unchanged_consensus_objects = unchanged_consensus_objects;
            }
        }
    }
}

//
// TransactionEffectsV2
//

impl Merge<&sui_sdk_types::TransactionEffectsV2> for TransactionEffects {
    fn merge(
        &mut self,
        sui_sdk_types::TransactionEffectsV2 {
            status,
            epoch,
            gas_used,
            transaction_digest,
            gas_object_index,
            events_digest,
            dependencies,
            lamport_version,
            changed_objects,
            unchanged_consensus_objects,
            auxiliary_data_digest,
        }: &sui_sdk_types::TransactionEffectsV2,
        mask: &FieldMaskTree,
    ) {
        if mask.contains(Self::VERSION_FIELD.name) {
            self.version = Some(2);
        }

        if mask.contains(Self::STATUS_FIELD.name) {
            self.status = Some(status.clone().into());
        }

        if mask.contains(Self::EPOCH_FIELD.name) {
            self.epoch = Some(*epoch);
        }

        if mask.contains(Self::GAS_USED_FIELD.name) {
            self.gas_used = Some(gas_used.clone().into());
        }

        if mask.contains(Self::TRANSACTION_DIGEST_FIELD.name) {
            self.transaction_digest = Some(transaction_digest.to_string());
        }

        if mask.contains(Self::GAS_OBJECT_FIELD.name) {
            self.gas_object = gas_object_index
                .map(|index| changed_objects.get(index as usize).cloned().map(Into::into))
                .flatten();
        }

        if mask.contains(Self::EVENTS_DIGEST_FIELD.name) {
            self.events_digest = events_digest.map(|d| d.to_string());
        }

        if mask.contains(Self::DEPENDENCIES_FIELD.name) {
            self.dependencies = dependencies.iter().map(ToString::to_string).collect();
        }

        if mask.contains(Self::LAMPORT_VERSION_FIELD.name) {
            self.lamport_version = Some(*lamport_version);
        }

        if mask.contains(Self::CHANGED_OBJECTS_FIELD.name) {
            self.changed_objects = changed_objects
                .clone()
                .into_iter()
                .map(Into::into)
                .collect();
        }

        for object in self.changed_objects.iter_mut().chain(&mut self.gas_object) {
            if object.output_digest.is_some() && object.output_version.is_none() {
                object.output_version = Some(*lamport_version);
            }
        }

        if mask.contains(Self::UNCHANGED_CONSENSUS_OBJECTS_FIELD.name) {
            self.unchanged_consensus_objects = unchanged_consensus_objects
                .clone()
                .into_iter()
                .map(Into::into)
                .collect();
        }

        if mask.contains(Self::AUXILIARY_DATA_DIGEST_FIELD.name) {
            self.auxiliary_data_digest = auxiliary_data_digest.map(|d| d.to_string());
        }
    }
}

//
// ChangedObject
//

impl From<sui_sdk_types::ChangedObject> for ChangedObject {
    fn from(value: sui_sdk_types::ChangedObject) -> Self {
        use changed_object::InputObjectState;
        use changed_object::OutputObjectState;

        let mut message = Self {
            object_id: Some(value.object_id.to_string()),
            ..Default::default()
        };

        // Input State
        let input_state = match value.input_state {
            sui_sdk_types::ObjectIn::NotExist => InputObjectState::DoesNotExist,
            sui_sdk_types::ObjectIn::Exist {
                version,
                digest,
                owner,
            } => {
                message.input_version = Some(version);
                message.input_digest = Some(digest.to_string());
                message.input_owner = Some(owner.into());
                InputObjectState::Exists
            }
            _ => InputObjectState::Unknown,
        };
        message.set_input_state(input_state);

        // Output State
        let output_state = match value.output_state {
            sui_sdk_types::ObjectOut::NotExist => OutputObjectState::DoesNotExist,
            sui_sdk_types::ObjectOut::ObjectWrite { digest, owner } => {
                message.output_digest = Some(digest.to_string());
                message.output_owner = Some(owner.into());
                OutputObjectState::ObjectWrite
            }
            sui_sdk_types::ObjectOut::PackageWrite { version, digest } => {
                message.output_version = Some(version);
                message.output_digest = Some(digest.to_string());
                OutputObjectState::PackageWrite
            }
            sui_sdk_types::ObjectOut::AccumulatorWrite(accumulator_write) => {
                message.set_accumulator_write(accumulator_write);
                OutputObjectState::AccumulatorWrite
            }
            _ => OutputObjectState::Unknown,
        };
        message.set_output_state(output_state);

        message.set_id_operation(value.id_operation.into());
        message
    }
}

impl TryFrom<&ChangedObject> for sui_sdk_types::ChangedObject {
    type Error = TryFromProtoError;

    fn try_from(value: &ChangedObject) -> Result<Self, Self::Error> {
        use changed_object::InputObjectState;
        use changed_object::OutputObjectState;

        let object_id = value
            .object_id
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("object_id"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(ChangedObject::OBJECT_ID_FIELD, e))?;

        let input_state = match value.input_state() {
            InputObjectState::Unknown => {
                return Err(TryFromProtoError::invalid(
                    ChangedObject::INPUT_STATE_FIELD,
                    "unknown InputObjectState",
                ));
            }
            InputObjectState::DoesNotExist => sui_sdk_types::ObjectIn::NotExist,
            InputObjectState::Exists => sui_sdk_types::ObjectIn::Exist {
                version: value
                    .input_version
                    .ok_or_else(|| TryFromProtoError::missing("version"))?,
                digest: value
                    .input_digest
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("digest"))?
                    .parse()
                    .map_err(|e| {
                        TryFromProtoError::invalid(ChangedObject::INPUT_DIGEST_FIELD, e)
                    })?,
                owner: value
                    .input_owner
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("owner"))?
                    .try_into()?,
            },
        };

        let output_state = match value.output_state() {
            OutputObjectState::Unknown => {
                return Err(TryFromProtoError::invalid(
                    ChangedObject::OUTPUT_STATE_FIELD,
                    "unknown OutputObjectState",
                ));
            }
            OutputObjectState::DoesNotExist => sui_sdk_types::ObjectOut::NotExist,
            OutputObjectState::ObjectWrite => sui_sdk_types::ObjectOut::ObjectWrite {
                digest: value
                    .output_digest
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("digest"))?
                    .parse()
                    .map_err(|e| {
                        TryFromProtoError::invalid(ChangedObject::OUTPUT_DIGEST_FIELD, e)
                    })?,

                owner: value
                    .output_owner
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("owner"))?
                    .try_into()?,
            },
            OutputObjectState::PackageWrite => sui_sdk_types::ObjectOut::PackageWrite {
                version: value
                    .output_version
                    .ok_or_else(|| TryFromProtoError::missing("version"))?,
                digest: value
                    .output_digest
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("digest"))?
                    .parse()
                    .map_err(|e| {
                        TryFromProtoError::invalid(ChangedObject::OUTPUT_DIGEST_FIELD, e)
                    })?,
            },
            OutputObjectState::AccumulatorWrite => sui_sdk_types::ObjectOut::AccumulatorWrite(
                value
                    .accumulator_write_opt()
                    .ok_or_else(|| TryFromProtoError::missing("accumulator_write"))?
                    .try_into()?,
            ),
        };

        let id_operation = value.id_operation().try_into()?;

        Ok(Self {
            object_id,
            input_state,
            output_state,
            id_operation,
        })
    }
}

//
// IdOperation
//

impl From<sui_sdk_types::IdOperation> for changed_object::IdOperation {
    fn from(value: sui_sdk_types::IdOperation) -> Self {
        use sui_sdk_types::IdOperation::*;

        match value {
            None => Self::None,
            Created => Self::Created,
            Deleted => Self::Deleted,
            _ => Self::Unknown,
        }
    }
}

impl TryFrom<changed_object::IdOperation> for sui_sdk_types::IdOperation {
    type Error = TryFromProtoError;

    fn try_from(value: changed_object::IdOperation) -> Result<Self, Self::Error> {
        use changed_object::IdOperation;

        match value {
            IdOperation::Unknown => {
                return Err(TryFromProtoError::invalid(
                    "id_operation",
                    "unknown IdOperation",
                ));
            }
            IdOperation::None => Self::None,
            IdOperation::Created => Self::Created,
            IdOperation::Deleted => Self::Deleted,
        }
        .pipe(Ok)
    }
}

//
// IdOperation
//

impl From<sui_sdk_types::AccumulatorWrite> for AccumulatorWrite {
    fn from(value: sui_sdk_types::AccumulatorWrite) -> Self {
        use accumulator_write::AccumulatorOperation;

        let mut message = Self::default();

        message.set_address(value.address());
        message.set_accumulator_type(value.accumulator_type());

        message.set_operation(match value.operation() {
            sui_sdk_types::AccumulatorOperation::Merge => AccumulatorOperation::Merge,
            sui_sdk_types::AccumulatorOperation::Split => AccumulatorOperation::Split,
            _ => AccumulatorOperation::Unknown,
        });

        message.set_value(value.value());

        message
    }
}

impl TryFrom<&AccumulatorWrite> for sui_sdk_types::AccumulatorWrite {
    type Error = TryFromProtoError;

    fn try_from(value: &AccumulatorWrite) -> Result<Self, Self::Error> {
        let address = value
            .address_opt()
            .ok_or_else(|| TryFromProtoError::missing("address"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid("address", e))?;
        let accumulator_type = value
            .accumulator_type_opt()
            .ok_or_else(|| TryFromProtoError::missing("accumulator_type"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid("accumulator_type", e))?;

        let operation = match value.operation() {
            accumulator_write::AccumulatorOperation::Unknown => {
                return Err(TryFromProtoError::invalid("operation", "unknown operation"));
            }
            accumulator_write::AccumulatorOperation::Merge => {
                sui_sdk_types::AccumulatorOperation::Merge
            }
            accumulator_write::AccumulatorOperation::Split => {
                sui_sdk_types::AccumulatorOperation::Split
            }
        };

        let value = value
            .value_opt()
            .ok_or_else(|| TryFromProtoError::missing("value"))?;

        Ok(Self::new(address, accumulator_type, operation, value))
    }
}

//
// UnchangedConsensusObject
//

impl From<sui_sdk_types::UnchangedConsensusObject> for UnchangedConsensusObject {
    fn from(value: sui_sdk_types::UnchangedConsensusObject) -> Self {
        use sui_sdk_types::UnchangedConsensusKind::*;
        use unchanged_consensus_object::UnchangedConsensusObjectKind;

        let mut message = Self {
            object_id: Some(value.object_id.to_string()),
            ..Default::default()
        };

        let kind = match value.kind {
            ReadOnlyRoot { version, digest } => {
                message.version = Some(version);
                message.digest = Some(digest.to_string());
                UnchangedConsensusObjectKind::ReadOnlyRoot
            }
            MutateDeleted { version } => {
                message.version = Some(version);
                UnchangedConsensusObjectKind::MutateConsensusStreamEnded
            }
            ReadDeleted { version } => {
                message.version = Some(version);
                UnchangedConsensusObjectKind::ReadConsensusStreamEnded
            }
            Canceled { version } => {
                message.version = Some(version);
                UnchangedConsensusObjectKind::Canceled
            }
            PerEpochConfig => UnchangedConsensusObjectKind::PerEpochConfig,
            _ => UnchangedConsensusObjectKind::Unknown,
        };

        message.set_kind(kind);
        message
    }
}

impl TryFrom<&UnchangedConsensusObject> for sui_sdk_types::UnchangedConsensusObject {
    type Error = TryFromProtoError;

    fn try_from(value: &UnchangedConsensusObject) -> Result<Self, Self::Error> {
        use sui_sdk_types::UnchangedConsensusKind;
        use unchanged_consensus_object::UnchangedConsensusObjectKind;

        let object_id = value
            .object_id
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("object_id"))?
            .parse()
            .map_err(|e| {
                TryFromProtoError::invalid(UnchangedConsensusObject::OBJECT_ID_FIELD, e)
            })?;

        let kind = match value.kind() {
            UnchangedConsensusObjectKind::Unknown => {
                return Err(TryFromProtoError::invalid(
                    UnchangedConsensusObject::KIND_FIELD,
                    "unknown InputKind",
                ));
            }

            UnchangedConsensusObjectKind::ReadOnlyRoot => UnchangedConsensusKind::ReadOnlyRoot {
                version: value
                    .version
                    .ok_or_else(|| TryFromProtoError::missing("version"))?,

                digest: value
                    .digest
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("digest"))?
                    .parse()
                    .map_err(|e| {
                        TryFromProtoError::invalid(UnchangedConsensusObject::DIGEST_FIELD, e)
                    })?,
            },
            UnchangedConsensusObjectKind::MutateConsensusStreamEnded => {
                UnchangedConsensusKind::MutateDeleted {
                    version: value
                        .version
                        .ok_or_else(|| TryFromProtoError::missing("version"))?,
                }
            }
            UnchangedConsensusObjectKind::ReadConsensusStreamEnded => {
                UnchangedConsensusKind::ReadDeleted {
                    version: value
                        .version
                        .ok_or_else(|| TryFromProtoError::missing("version"))?,
                }
            }
            UnchangedConsensusObjectKind::Canceled => UnchangedConsensusKind::Canceled {
                version: value
                    .version
                    .ok_or_else(|| TryFromProtoError::missing("version"))?,
            },
            UnchangedConsensusObjectKind::PerEpochConfig => UnchangedConsensusKind::PerEpochConfig,
        };

        Ok(Self { object_id, kind })
    }
}
