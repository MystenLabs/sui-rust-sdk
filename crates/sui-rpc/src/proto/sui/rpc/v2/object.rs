use super::*;
use crate::field::FieldMaskTree;
use crate::merge::Merge;
use crate::proto::TryFromProtoError;
use tap::Pipe;

//
// Object
//

pub const PACKAGE_TYPE: &str = "package";

impl From<sui_sdk_types::Object> for Object {
    fn from(value: sui_sdk_types::Object) -> Self {
        Self::merge_from(value, &FieldMaskTree::new_wildcard())
    }
}

impl Merge<&Object> for Object {
    fn merge(&mut self, source: &Object, mask: &FieldMaskTree) {
        let Object {
            bcs,
            object_id,
            version,
            digest,
            owner,
            object_type,
            has_public_transfer,
            contents,
            package,
            previous_transaction,
            storage_rebate,
            json,
            balance,
            display,
        } = source;

        if mask.contains(Self::BCS_FIELD.name) {
            self.bcs = bcs.clone();
        }

        if mask.contains(Self::DIGEST_FIELD.name) {
            self.digest = digest.clone();
        }

        if mask.contains(Self::OBJECT_ID_FIELD.name) {
            self.object_id = object_id.clone();
        }

        if mask.contains(Self::VERSION_FIELD.name) {
            self.version = *version;
        }

        if mask.contains(Self::OWNER_FIELD.name) {
            self.owner = owner.clone();
        }

        if mask.contains(Self::PREVIOUS_TRANSACTION_FIELD.name) {
            self.previous_transaction = previous_transaction.clone();
        }

        if mask.contains(Self::STORAGE_REBATE_FIELD.name) {
            self.storage_rebate = *storage_rebate;
        }

        if mask.contains(Self::OBJECT_TYPE_FIELD.name) {
            self.object_type = object_type.clone();
        }

        if mask.contains(Self::HAS_PUBLIC_TRANSFER_FIELD.name) {
            self.has_public_transfer = *has_public_transfer;
        }

        if mask.contains(Self::CONTENTS_FIELD.name) {
            self.contents = contents.clone();
        }

        if mask.contains(Self::PACKAGE_FIELD.name) {
            self.package = package.clone();
        }

        if mask.contains(Self::JSON_FIELD.name) {
            self.json = json.clone();
        }

        if mask.contains(Self::BALANCE_FIELD) {
            self.balance = *balance;
        }

        if mask.contains(Self::DIGEST_FIELD) {
            self.display = display.clone();
        }
    }
}

impl Merge<sui_sdk_types::Object> for Object {
    fn merge(&mut self, source: sui_sdk_types::Object, mask: &FieldMaskTree) {
        if mask.contains(Self::BCS_FIELD.name) {
            let mut bcs = Bcs::serialize(&source).unwrap();
            bcs.name = Some("Object".to_owned());
            self.bcs = Some(bcs);
        }

        if mask.contains(Self::DIGEST_FIELD.name) {
            self.digest = Some(source.digest().to_string());
        }

        if mask.contains(Self::OBJECT_ID_FIELD.name) {
            self.object_id = Some(source.object_id().to_string());
        }

        if mask.contains(Self::VERSION_FIELD.name) {
            self.version = Some(source.version());
        }

        if mask.contains(Self::OWNER_FIELD.name) {
            self.owner = Some(source.owner().to_owned().into());
        }

        if mask.contains(Self::PREVIOUS_TRANSACTION_FIELD.name) {
            self.previous_transaction = Some(source.previous_transaction().to_string());
        }

        if mask.contains(Self::STORAGE_REBATE_FIELD.name) {
            self.storage_rebate = Some(source.storage_rebate());
        }

        match source.data() {
            sui_sdk_types::ObjectData::Struct(move_struct) => {
                self.merge(move_struct, mask);
            }
            sui_sdk_types::ObjectData::Package(move_package) => {
                self.merge(move_package, mask);
            }
        }
    }
}

impl Merge<&sui_sdk_types::MoveStruct> for Object {
    fn merge(&mut self, source: &sui_sdk_types::MoveStruct, mask: &FieldMaskTree) {
        if mask.contains(Self::OBJECT_TYPE_FIELD.name) {
            self.object_type = Some(source.object_type().to_string());
        }

        if mask.contains(Self::HAS_PUBLIC_TRANSFER_FIELD.name) {
            self.has_public_transfer = Some(source.has_public_transfer());
        }

        if mask.contains(Self::CONTENTS_FIELD.name) {
            self.contents = Some(Bcs {
                name: Some(source.object_type().to_string()),
                value: Some(source.contents().to_vec().into()),
            });
        }
    }
}

impl Merge<&sui_sdk_types::MovePackage> for Object {
    fn merge(&mut self, source: &sui_sdk_types::MovePackage, mask: &FieldMaskTree) {
        if mask.contains(Self::OBJECT_TYPE_FIELD.name) {
            self.object_type = Some(PACKAGE_TYPE.to_owned());
        }

        if mask.contains(Self::PACKAGE_FIELD.name) {
            self.package = Some(Package {
                modules: source
                    .modules
                    .iter()
                    .map(|(name, contents)| Module {
                        name: Some(name.to_string()),
                        contents: Some(contents.clone().into()),
                        ..Default::default()
                    })
                    .collect(),
                type_origins: source
                    .type_origin_table
                    .clone()
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                linkage: source
                    .linkage_table
                    .iter()
                    .map(
                        |(
                            original_id,
                            sui_sdk_types::UpgradeInfo {
                                upgraded_id,
                                upgraded_version,
                            },
                        )| {
                            Linkage {
                                original_id: Some(original_id.to_string()),
                                upgraded_id: Some(upgraded_id.to_string()),
                                upgraded_version: Some(*upgraded_version),
                            }
                        },
                    )
                    .collect(),

                ..Default::default()
            })
        }
    }
}

#[allow(clippy::result_large_err)]
fn try_extract_struct(value: &Object) -> Result<sui_sdk_types::MoveStruct, TryFromProtoError> {
    let version = value
        .version
        .ok_or_else(|| TryFromProtoError::missing("version"))?;

    let object_type = value
        .object_type()
        .parse()
        .map_err(|e| TryFromProtoError::invalid(Object::OBJECT_TYPE_FIELD, e))?;

    let has_public_transfer = value
        .has_public_transfer
        .ok_or_else(|| TryFromProtoError::missing("has_public_transfer"))?;
    let contents = value
        .contents
        .as_ref()
        .ok_or_else(|| TryFromProtoError::missing("contents"))?
        .value()
        .to_vec();

    sui_sdk_types::MoveStruct::new(object_type, has_public_transfer, version, contents).ok_or_else(
        || TryFromProtoError::invalid(Object::CONTENTS_FIELD, "contents missing object_id"),
    )
}

#[allow(clippy::result_large_err)]
fn try_extract_package(value: &Object) -> Result<sui_sdk_types::MovePackage, TryFromProtoError> {
    if value.object_type() != PACKAGE_TYPE {
        return Err(TryFromProtoError::invalid(
            Object::OBJECT_TYPE_FIELD,
            format!(
                "expected type {}, found {}",
                PACKAGE_TYPE,
                value.object_type()
            ),
        ));
    }

    let version = value
        .version
        .ok_or_else(|| TryFromProtoError::missing("version"))?;
    let id = value
        .object_id
        .as_ref()
        .ok_or_else(|| TryFromProtoError::missing("object_id"))?
        .parse()
        .map_err(|e| TryFromProtoError::invalid(Object::OBJECT_ID_FIELD, e))?;

    let package = value
        .package
        .as_ref()
        .ok_or_else(|| TryFromProtoError::missing("package"))?;
    let modules = package
        .modules
        .iter()
        .map(|module| {
            let name = module
                .name
                .as_ref()
                .ok_or_else(|| TryFromProtoError::missing("name"))?
                .parse()
                .map_err(|e| TryFromProtoError::invalid(Module::NAME_FIELD, e))?;

            let contents = module
                .contents
                .as_ref()
                .ok_or_else(|| TryFromProtoError::missing("contents"))?
                .to_vec();

            Ok((name, contents))
        })
        .collect::<Result<_, TryFromProtoError>>()?;

    let type_origin_table = package
        .type_origins
        .iter()
        .map(TryInto::try_into)
        .collect::<Result<_, _>>()?;

    let linkage_table = package
        .linkage
        .iter()
        .map(|upgrade_info| {
            let original_id = upgrade_info
                .original_id
                .as_ref()
                .ok_or_else(|| TryFromProtoError::missing("original_id"))?
                .parse()
                .map_err(|e| TryFromProtoError::invalid(Linkage::ORIGINAL_ID_FIELD, e))?;

            let upgraded_id = upgrade_info
                .upgraded_id
                .as_ref()
                .ok_or_else(|| TryFromProtoError::missing("upgraded_id"))?
                .parse()
                .map_err(|e| TryFromProtoError::invalid(Linkage::UPGRADED_ID_FIELD, e))?;
            let upgraded_version = upgrade_info
                .upgraded_version
                .ok_or_else(|| TryFromProtoError::missing("upgraded_version"))?;

            Ok((
                original_id,
                sui_sdk_types::UpgradeInfo {
                    upgraded_id,
                    upgraded_version,
                },
            ))
        })
        .collect::<Result<_, TryFromProtoError>>()?;

    Ok(sui_sdk_types::MovePackage {
        id,
        version,
        modules,
        type_origin_table,
        linkage_table,
    })
}

impl TryFrom<&Object> for sui_sdk_types::Object {
    type Error = TryFromProtoError;

    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        let owner = value
            .owner
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("owner"))?
            .try_into()?;

        let previous_transaction = value
            .previous_transaction
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("previous_transaction"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(Object::PREVIOUS_TRANSACTION_FIELD, e))?;
        let storage_rebate = value
            .storage_rebate
            .ok_or_else(|| TryFromProtoError::missing("storage_rebate"))?;

        let object_data = if value.object_type() == PACKAGE_TYPE {
            // Package
            sui_sdk_types::ObjectData::Package(try_extract_package(value)?)
        } else {
            // Struct
            sui_sdk_types::ObjectData::Struct(try_extract_struct(value)?)
        };

        Ok(Self::new(
            object_data,
            owner,
            previous_transaction,
            storage_rebate,
        ))
    }
}

//
// TypeOrigin
//

impl From<sui_sdk_types::TypeOrigin> for TypeOrigin {
    fn from(value: sui_sdk_types::TypeOrigin) -> Self {
        Self {
            module_name: Some(value.module_name.to_string()),
            datatype_name: Some(value.struct_name.to_string()),
            package_id: Some(value.package.to_string()),
        }
    }
}

impl TryFrom<&TypeOrigin> for sui_sdk_types::TypeOrigin {
    type Error = TryFromProtoError;

    fn try_from(value: &TypeOrigin) -> Result<Self, Self::Error> {
        let module_name = value
            .module_name
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("module_name"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(TypeOrigin::MODULE_NAME_FIELD, e))?;

        let struct_name = value
            .datatype_name
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("datatype_name"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(TypeOrigin::DATATYPE_NAME_FIELD, e))?;

        let package = value
            .package_id
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("package_id"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(TypeOrigin::PACKAGE_ID_FIELD, e))?;

        Ok(Self {
            module_name,
            struct_name,
            package,
        })
    }
}

//
// GenesisObject
//

impl From<sui_sdk_types::GenesisObject> for Object {
    fn from(value: sui_sdk_types::GenesisObject) -> Self {
        let mut message = Self {
            object_id: Some(value.object_id().to_string()),
            version: Some(value.version()),
            owner: Some(value.owner().to_owned().into()),
            ..Default::default()
        };

        match value.data() {
            sui_sdk_types::ObjectData::Struct(move_struct) => {
                message.merge(move_struct, &FieldMaskTree::new_wildcard());
            }
            sui_sdk_types::ObjectData::Package(move_package) => {
                message.merge(move_package, &FieldMaskTree::new_wildcard());
            }
        }

        message
    }
}

impl TryFrom<&Object> for sui_sdk_types::GenesisObject {
    type Error = TryFromProtoError;

    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        let object_data = if value.object_type() == PACKAGE_TYPE {
            // Package
            sui_sdk_types::ObjectData::Package(try_extract_package(value)?)
        } else {
            // Struct
            sui_sdk_types::ObjectData::Struct(try_extract_struct(value)?)
        };

        let owner = value
            .owner
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("owner"))?
            .try_into()?;

        Ok(Self::new(object_data, owner))
    }
}

impl Object {
    pub fn object_reference(&self) -> ObjectReference {
        ObjectReference {
            object_id: self.object_id.clone(),
            version: self.version,
            digest: self.digest.clone(),
        }
    }
}

//
// ObjectReference
//

impl From<sui_sdk_types::ObjectReference> for ObjectReference {
    fn from(value: sui_sdk_types::ObjectReference) -> Self {
        let (object_id, version, digest) = value.into_parts();
        Self {
            object_id: Some(object_id.to_string()),
            version: Some(version),
            digest: Some(digest.to_string()),
        }
    }
}

impl TryFrom<&ObjectReference> for sui_sdk_types::ObjectReference {
    type Error = TryFromProtoError;

    fn try_from(value: &ObjectReference) -> Result<Self, Self::Error> {
        let object_id = value
            .object_id
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("object_id"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(ObjectReference::OBJECT_ID_FIELD, e))?;

        let version = value
            .version
            .ok_or_else(|| TryFromProtoError::missing("version"))?;

        let digest = value
            .digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("digest"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(ObjectReference::DIGEST_FIELD, e))?;

        Ok(Self::new(object_id, version, digest))
    }
}

//
// Owner
//

impl From<sui_sdk_types::Owner> for Owner {
    fn from(value: sui_sdk_types::Owner) -> Self {
        use owner::OwnerKind;
        use sui_sdk_types::Owner::*;

        let mut message = Self::default();

        let kind = match value {
            Address(address) => {
                message.address = Some(address.to_string());
                OwnerKind::Address
            }
            Object(object) => {
                message.address = Some(object.to_string());
                OwnerKind::Object
            }
            Shared(version) => {
                message.version = Some(version);
                OwnerKind::Shared
            }
            Immutable => OwnerKind::Immutable,
            ConsensusAddress {
                start_version,
                owner,
            } => {
                message.version = Some(start_version);
                message.address = Some(owner.to_string());
                OwnerKind::ConsensusAddress
            }
            _ => OwnerKind::Unknown,
        };

        message.set_kind(kind);
        message
    }
}

impl TryFrom<&Owner> for sui_sdk_types::Owner {
    type Error = TryFromProtoError;

    fn try_from(value: &Owner) -> Result<Self, Self::Error> {
        use owner::OwnerKind;

        match value.kind() {
            OwnerKind::Unknown => {
                return Err(TryFromProtoError::invalid(
                    Owner::KIND_FIELD,
                    "unknown OwnerKind",
                ));
            }
            OwnerKind::Address => Self::Address(
                value
                    .address()
                    .parse()
                    .map_err(|e| TryFromProtoError::invalid(Owner::ADDRESS_FIELD, e))?,
            ),
            OwnerKind::Object => Self::Object(
                value
                    .address()
                    .parse()
                    .map_err(|e| TryFromProtoError::invalid(Owner::ADDRESS_FIELD, e))?,
            ),
            OwnerKind::Shared => Self::Shared(value.version()),
            OwnerKind::Immutable => Self::Immutable,
            OwnerKind::ConsensusAddress => Self::ConsensusAddress {
                start_version: value.version(),
                owner: value
                    .address()
                    .parse()
                    .map_err(|e| TryFromProtoError::invalid(Owner::ADDRESS_FIELD, e))?,
            },
        }
        .pipe(Ok)
    }
}

impl Merge<&ObjectSet> for ObjectSet {
    fn merge(&mut self, source: &ObjectSet, mask: &FieldMaskTree) {
        if let Some(submask) = mask.subtree(Self::OBJECTS_FIELD) {
            self.objects = source
                .objects()
                .iter()
                .map(|object| Object::merge_from(object, &submask))
                .collect();
        }
    }
}

impl ObjectSet {
    // Sorts the objects in this set by the key `(object_id, version)`
    #[doc(hidden)]
    pub fn sort_objects(&mut self) {
        self.objects_mut().sort_by(|a, b| {
            let a = (a.object_id(), a.version());
            let b = (b.object_id(), b.version());
            a.cmp(&b)
        });
    }

    // Performs a binary search on the contained object set searching for the specified
    // (object_id, version). This function assumes that both the `object_id` and `version` fields
    // are set for all contained objects.
    pub fn binary_search<'a>(
        &'a self,
        object_id: &sui_sdk_types::Address,
        version: u64,
    ) -> Option<&'a Object> {
        let object_id = object_id.to_string();
        let seek = (object_id.as_str(), version);
        self.objects()
            .binary_search_by(|object| {
                let probe = (object.object_id(), object.version());
                probe.cmp(&seek)
            })
            .ok()
            .and_then(|found| self.objects().get(found))
    }
}
