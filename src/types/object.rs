use std::collections::BTreeMap;

use super::Address;
use super::Identifier;
use super::ObjectDigest;
use super::ObjectId;
use super::StructTag;
use super::TransactionDigest;

pub type Version = u64;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct ObjectReference {
    object_id: ObjectId,
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
    version: Version,
    digest: ObjectDigest,
}

impl ObjectReference {
    pub fn new(object_id: ObjectId, version: Version, digest: ObjectDigest) -> Self {
        Self {
            object_id,
            version,
            digest,
        }
    }

    pub fn object_id(&self) -> &ObjectId {
        &self.object_id
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn digest(&self) -> &ObjectDigest {
        &self.digest
    }

    pub fn into_parts(self) -> (ObjectId, Version, ObjectDigest) {
        let Self {
            object_id,
            version,
            digest,
        } = self;

        (object_id, version, digest)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "lowercase")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub enum Owner {
    /// # Address Owned
    /// Object is exclusively owned by a single address, and is mutable.
    Address(Address),
    /// # Object Owned
    /// Object is exclusively owned by a single object, and is mutable.
    Object(ObjectId),
    /// # Shared Object
    /// Object is shared, can be used by any address, and is mutable.
    Shared(
        /// The version at which the object became shared
        #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        Version,
    ),
    /// # Immutable
    /// Object is immutable, and hence ownership doesn't matter.
    Immutable,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[allow(clippy::large_enum_variant)]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
//TODO think about hiding this type and not exposing it
pub enum ObjectData {
    /// An object whose governing logic lives in a published Move module
    Struct(MoveStruct),
    /// Map from each module name to raw serialized Move module bytes
    Package(MovePackage),
    // ... Sui "native" types go here
}

// serde_bytes::ByteBuf is an analog of Vec<u8> with built-in fast serialization.
// #[serde_as]
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct MovePackage {
    id: ObjectId,
    /// Most move packages are uniquely identified by their ID (i.e. there is only one version per
    /// ID), but the version is still stored because one package may be an upgrade of another (at a
    /// different ID), in which case its version will be one greater than the version of the
    /// upgraded package.
    ///
    /// Framework packages are an exception to this rule -- all versions of the framework packages
    /// exist at the same ID, at increasing versions.
    ///
    /// In all cases, packages are referred to by move calls using just their ID, and they are
    /// always loaded at their latest version.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    version: Version,

    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<BTreeMap<::serde_with::Same, ::serde_with::Bytes>>")
    )]
    #[cfg_attr(
        test,
        strategy(
            proptest::collection::btree_map(proptest::arbitrary::any::<Identifier>(), proptest::collection::vec(proptest::arbitrary::any::<u8>(), 0..=1024), 0..=5)
        )
    )]
    modules: BTreeMap<Identifier, Vec<u8>>,

    /// Maps struct/module to a package version where it was first defined, stored as a vector for
    /// simple serialization and deserialization.
    type_origin_table: Vec<TypeOrigin>,

    // For each dependency, maps original package ID to the info about the (upgraded) dependency
    // version that this package is using
    #[cfg_attr(
        test,
        strategy(
            proptest::collection::btree_map(proptest::arbitrary::any::<ObjectId>(), proptest::arbitrary::any::<UpgradeInfo>(), 0..=5)
        )
    )]
    linkage_table: BTreeMap<ObjectId, UpgradeInfo>,
}

/// Identifies a struct and the module it was defined in
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct TypeOrigin {
    pub module_name: Identifier,
    pub struct_name: Identifier,
    pub package: ObjectId,
}

/// Upgraded package info for the linkage table
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct UpgradeInfo {
    /// Id of the upgraded packages
    pub upgraded_id: ObjectId,
    /// Version of the upgraded package
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
    pub upgraded_version: Version,
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct MoveStruct {
    /// The type of this object. Immutable
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<serialization::BinaryMoveStructType>")
    )]
    pub(crate) type_: StructTag,
    /// DEPRECATED this field is no longer used to determine whether a tx can transfer this
    /// object. Instead, it is always calculated from the objects type when loaded in execution
    has_public_transfer: bool,
    /// Number that increases each time a tx takes this object as a mutable input
    /// This is a lamport timestamp, not a sequentially increasing version
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    version: Version,
    /// BCS bytes of a Move struct value
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<::serde_with::Bytes>")
    )]
    #[cfg_attr(test, any(proptest::collection::size_range(32..=1024).lift()))]
    pub(crate) contents: Vec<u8>,
}

/// Type of a Sui object
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ObjectType {
    /// Move package containing one or more bytecode modules
    Package,
    /// A Move struct of the given type
    Struct(StructTag),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct Object {
    /// The meat of the object
    pub(crate) data: ObjectData,
    /// The owner that unlocks this object
    owner: Owner,
    /// The digest of the transaction that created or last mutated this object
    previous_transaction: TransactionDigest,
    /// The amount of SUI we would rebate if this object gets deleted.
    /// This number is re-calculated each time the object is mutated based on
    /// the present storage gas price.
    storage_rebate: u64,
}

impl Object {
    pub fn object_id(&self) -> ObjectId {
        match &self.data {
            ObjectData::Struct(struct_) => id_opt(&struct_.contents).unwrap(),
            ObjectData::Package(package) => package.id,
        }
    }

    pub fn version(&self) -> Version {
        match &self.data {
            ObjectData::Struct(struct_) => struct_.version,
            ObjectData::Package(package) => package.version,
        }
    }

    pub fn object_type(&self) -> ObjectType {
        match &self.data {
            ObjectData::Struct(struct_) => ObjectType::Struct(struct_.type_.clone()),
            ObjectData::Package(_) => ObjectType::Package,
        }
    }

    pub fn owner(&self) -> &Owner {
        &self.owner
    }
}

fn id_opt(contents: &[u8]) -> Option<ObjectId> {
    if ObjectId::LENGTH > contents.len() {
        return None;
    }

    Some(ObjectId::from(
        Address::from_bytes(&contents[..ObjectId::LENGTH]).unwrap(),
    ))
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct GenesisObject {
    data: ObjectData,
    owner: Owner,
}

impl GenesisObject {
    pub fn object_id(&self) -> ObjectId {
        match &self.data {
            ObjectData::Struct(struct_) => id_opt(&struct_.contents).unwrap(),
            ObjectData::Package(package) => package.id,
        }
    }

    pub fn version(&self) -> Version {
        match &self.data {
            ObjectData::Struct(struct_) => struct_.version,
            ObjectData::Package(package) => package.version,
        }
    }

    pub fn object_type(&self) -> ObjectType {
        match &self.data {
            ObjectData::Struct(struct_) => ObjectType::Struct(struct_.type_.clone()),
            ObjectData::Package(_) => ObjectType::Package,
        }
    }
}

//TODO improve ser/de to do borrowing to avoid clones where possible
#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use std::borrow::Cow;
    use std::str::FromStr;

    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;
    use serde_with::DeserializeAs;
    use serde_with::SerializeAs;

    use super::*;
    use crate::types::TypeTag;

    #[test]
    fn obj() {
        let o = Object {
            data: ObjectData::Struct(MoveStruct {
                type_: StructTag {
                    address: Address::TWO,
                    module: Identifier::new("bar").unwrap(),
                    name: Identifier::new("foo").unwrap(),
                    type_params: Vec::new(),
                },
                has_public_transfer: true,
                version: 12,
                contents: ObjectId::ZERO.into(),
            }),
            // owner: Owner::Address(Address::ZERO),
            owner: Owner::Object(ObjectId::ZERO),
            // owner: Owner::Immutable,
            // owner: Owner::Shared {
            //     initial_shared_version: 14,
            // },
            previous_transaction: TransactionDigest::ZERO,
            storage_rebate: 100,
        };

        println!("{}", serde_json::to_string_pretty(&o).unwrap());
        println!(
            "{}",
            serde_json::to_string_pretty(&ObjectReference {
                object_id: ObjectId::ZERO,
                version: 1,
                digest: ObjectDigest::ZERO,
            })
            .unwrap()
        );
    }

    /// Wrapper around StructTag with a space-efficient representation for common types like coins
    /// The StructTag for a gas coin is 84 bytes, so using 1 byte instead is a win.
    /// The inner representation is private to prevent incorrectly constructing an `Other` instead of
    /// one of the specialized variants, e.g. `Other(GasCoin::type_())` instead of `GasCoin`
    #[derive(serde_derive::Deserialize)]
    enum MoveStructType {
        /// A type that is not `0x2::coin::Coin<T>`
        Other(StructTag),
        /// A SUI coin (i.e., `0x2::coin::Coin<0x2::sui::SUI>`)
        GasCoin,
        /// A record of a staked SUI coin (i.e., `0x3::staking_pool::StakedSui`)
        StakedSui,
        /// A non-SUI coin type (i.e., `0x2::coin::Coin<T> where T != 0x2::sui::SUI`)
        Coin(TypeTag),
        // NOTE: if adding a new type here, and there are existing on-chain objects of that
        // type with Other(_), that is ok, but you must hand-roll PartialEq/Eq/Ord/maybe Hash
        // to make sure the new type and Other(_) are interpreted consistently.
    }

    /// See `MoveStructType`
    #[derive(serde_derive::Serialize)]
    enum MoveStructTypeRef<'a> {
        /// A type that is not `0x2::coin::Coin<T>`
        Other(&'a StructTag),
        /// A SUI coin (i.e., `0x2::coin::Coin<0x2::sui::SUI>`)
        GasCoin,
        /// A record of a staked SUI coin (i.e., `0x3::staking_pool::StakedSui`)
        StakedSui,
        /// A non-SUI coin type (i.e., `0x2::coin::Coin<T> where T != 0x2::sui::SUI`)
        Coin(&'a TypeTag),
        // NOTE: if adding a new type here, and there are existing on-chain objects of that
        // type with Other(_), that is ok, but you must hand-roll PartialEq/Eq/Ord/maybe Hash
        // to make sure the new type and Other(_) are interpreted consistently.
    }

    impl MoveStructType {
        fn into_struct_tag(self) -> StructTag {
            match self {
                MoveStructType::Other(tag) => tag,
                MoveStructType::GasCoin => StructTag::gas_coin(),
                MoveStructType::StakedSui => StructTag::staked_sui(),
                MoveStructType::Coin(type_tag) => StructTag::coin(type_tag),
            }
        }
    }

    impl<'a> MoveStructTypeRef<'a> {
        fn from_struct_tag(s: &'a StructTag) -> Self {
            let StructTag {
                address,
                module,
                name,
                type_params,
            } = s;

            if let Some(coin_type) = s.is_coin() {
                if let TypeTag::Struct(s_inner) = coin_type {
                    let StructTag {
                        address,
                        module,
                        name,
                        type_params,
                    } = s_inner.as_ref();

                    if address == &Address::TWO
                        && module == "sui"
                        && name == "SUI"
                        && type_params.is_empty()
                    {
                        return Self::GasCoin;
                    }
                }

                Self::Coin(coin_type)
            } else if address == &Address::THREE
                && module == "staking_pool"
                && name == "StakedSui"
                && type_params.is_empty()
            {
                Self::StakedSui
            } else {
                Self::Other(s)
            }
        }
    }

    pub(super) struct BinaryMoveStructType;

    impl SerializeAs<StructTag> for BinaryMoveStructType {
        fn serialize_as<S>(source: &StructTag, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let move_object_type = MoveStructTypeRef::from_struct_tag(source);
            move_object_type.serialize(serializer)
        }
    }

    impl<'de> DeserializeAs<'de, StructTag> for BinaryMoveStructType {
        fn deserialize_as<D>(deserializer: D) -> Result<StructTag, D::Error>
        where
            D: Deserializer<'de>,
        {
            let struct_type = MoveStructType::deserialize(deserializer)?;
            Ok(struct_type.into_struct_tag())
        }
    }

    struct ReadableObjectType;

    impl SerializeAs<ObjectType> for ReadableObjectType {
        fn serialize_as<S>(source: &ObjectType, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match source {
                ObjectType::Package => "package".serialize(serializer),
                ObjectType::Struct(s) => s.serialize(serializer),
            }
        }
    }

    impl<'de> DeserializeAs<'de, ObjectType> for ReadableObjectType {
        fn deserialize_as<D>(deserializer: D) -> Result<ObjectType, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
            if s == "package" {
                Ok(ObjectType::Package)
            } else {
                let struct_tag = StructTag::from_str(&s)
                    .map_err(|_| serde::de::Error::custom("invalid object type"))?;
                Ok(ObjectType::Struct(struct_tag))
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(rename = "Object")]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    struct ReadableObject {
        object_id: ObjectId,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        version: Version,
        //TODO include digest in output
        // digest: ObjectDigest,
        owner: Owner,

        #[serde(with = "::serde_with::As::<ReadableObjectType>")]
        #[serde(rename = "type")]
        #[cfg_attr(feature = "schemars", schemars(with = "String"))]
        type_: ObjectType,

        #[serde(flatten)]
        data: ReadableObjectData,

        previous_transaction: TransactionDigest,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        storage_rebate: u64,
    }

    #[cfg(feature = "schemars")]
    impl schemars::JsonSchema for Object {
        fn schema_name() -> String {
            ReadableObject::schema_name()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            ReadableObject::json_schema(gen)
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(untagged)]
    #[cfg_attr(
        feature = "schemars",
        derive(schemars::JsonSchema),
        schemars(rename = "ObjectData")
    )]
    enum ReadableObjectData {
        Move(ReadableMoveStruct),
        Package(ReadablePackage),
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[cfg_attr(
        feature = "schemars",
        derive(schemars::JsonSchema),
        schemars(rename = "Package")
    )]
    struct ReadablePackage {
        #[serde(
            with = "::serde_with::As::<BTreeMap<::serde_with::Same, crate::_serde::Base64Encoded>>"
        )]
        #[cfg_attr(
            feature = "schemars",
            schemars(with = "BTreeMap<Identifier, crate::_schemars::Base64>")
        )]
        modules: BTreeMap<Identifier, Vec<u8>>,
        type_origin_table: Vec<TypeOrigin>,
        linkage_table: BTreeMap<ObjectId, UpgradeInfo>,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[cfg_attr(
        feature = "schemars",
        derive(schemars::JsonSchema),
        schemars(rename = "MoveStruct")
    )]
    struct ReadableMoveStruct {
        has_public_transfer: bool,
        #[serde(with = "::serde_with::As::<crate::_serde::Base64Encoded>")]
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::Base64"))]
        contents: Vec<u8>,
    }

    impl Object {
        fn readable_object_data(&self) -> ReadableObjectData {
            match &self.data {
                ObjectData::Struct(struct_) => ReadableObjectData::Move(ReadableMoveStruct {
                    has_public_transfer: struct_.has_public_transfer,
                    contents: struct_.contents.clone(),
                }),
                ObjectData::Package(package) => ReadableObjectData::Package(ReadablePackage {
                    modules: package.modules.clone(),
                    type_origin_table: package.type_origin_table.clone(),
                    linkage_table: package.linkage_table.clone(),
                }),
            }
        }
    }

    impl Serialize for Object {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = ReadableObject {
                    object_id: self.object_id(),
                    version: self.version(),
                    // digest: todo!(),
                    owner: self.owner,
                    previous_transaction: self.previous_transaction,
                    storage_rebate: self.storage_rebate,
                    type_: self.object_type(),
                    data: self.readable_object_data(),
                };
                readable.serialize(serializer)
            } else {
                let binary = BinaryObject {
                    data: self.data.clone(),
                    owner: self.owner,
                    previous_transaction: self.previous_transaction,
                    storage_rebate: self.storage_rebate,
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for Object {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let ReadableObject {
                    object_id,
                    version,
                    owner,
                    previous_transaction,
                    storage_rebate,
                    type_,
                    data,
                } = Deserialize::deserialize(deserializer)?;

                // check if package or struct
                let data = match (type_, data) {
                    (
                        ObjectType::Package,
                        ReadableObjectData::Package(ReadablePackage {
                            modules,
                            type_origin_table,
                            linkage_table,
                        }),
                    ) => ObjectData::Package(MovePackage {
                        id: object_id,
                        version,
                        modules,
                        type_origin_table,
                        linkage_table,
                    }),
                    (
                        ObjectType::Struct(type_),
                        ReadableObjectData::Move(ReadableMoveStruct {
                            has_public_transfer,
                            contents,
                        }),
                    ) => {
                        // check id matches in contents
                        if !id_opt(&contents).is_some_and(|id| id == object_id) {
                            return Err(serde::de::Error::custom("id from contents doesn't match"));
                        }

                        ObjectData::Struct(MoveStruct {
                            type_,
                            has_public_transfer,
                            version,
                            contents,
                        })
                    }
                    _ => return Err(serde::de::Error::custom("type and data don't match")),
                };

                Ok(Object {
                    data,
                    owner,
                    previous_transaction,
                    storage_rebate,
                })
            } else {
                let BinaryObject {
                    data,
                    owner,
                    previous_transaction,
                    storage_rebate,
                } = Deserialize::deserialize(deserializer)?;

                Ok(Object {
                    data,
                    owner,
                    previous_transaction,
                    storage_rebate,
                })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    struct BinaryObject {
        data: ObjectData,
        owner: Owner,
        previous_transaction: TransactionDigest,
        storage_rebate: u64,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(rename = "GenesisObject")]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    struct ReadableGenesisObject {
        object_id: ObjectId,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
        version: Version,
        owner: Owner,

        #[serde(with = "::serde_with::As::<ReadableObjectType>")]
        #[serde(rename = "type")]
        #[cfg_attr(feature = "schemars", schemars(with = "String"))]
        type_: ObjectType,

        #[serde(flatten)]
        data: ReadableObjectData,
    }

    #[cfg(feature = "schemars")]
    impl schemars::JsonSchema for GenesisObject {
        fn schema_name() -> String {
            ReadableGenesisObject::schema_name()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            ReadableGenesisObject::json_schema(gen)
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum BinaryGenesisObject {
        RawObject { data: ObjectData, owner: Owner },
    }

    impl GenesisObject {
        fn readable_object_data(&self) -> ReadableObjectData {
            match &self.data {
                ObjectData::Struct(struct_) => ReadableObjectData::Move(ReadableMoveStruct {
                    has_public_transfer: struct_.has_public_transfer,
                    contents: struct_.contents.clone(),
                }),
                ObjectData::Package(package) => ReadableObjectData::Package(ReadablePackage {
                    modules: package.modules.clone(),
                    type_origin_table: package.type_origin_table.clone(),
                    linkage_table: package.linkage_table.clone(),
                }),
            }
        }
    }

    impl Serialize for GenesisObject {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = ReadableGenesisObject {
                    object_id: self.object_id(),
                    version: self.version(),
                    owner: self.owner,
                    type_: self.object_type(),
                    data: self.readable_object_data(),
                };
                readable.serialize(serializer)
            } else {
                let binary = BinaryGenesisObject::RawObject {
                    data: self.data.clone(),
                    owner: self.owner,
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for GenesisObject {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let ReadableGenesisObject {
                    object_id,
                    version,
                    owner,
                    type_,
                    data,
                } = Deserialize::deserialize(deserializer)?;

                // check if package or struct
                let data = match (type_, data) {
                    (
                        ObjectType::Package,
                        ReadableObjectData::Package(ReadablePackage {
                            modules,
                            type_origin_table,
                            linkage_table,
                        }),
                    ) => ObjectData::Package(MovePackage {
                        id: object_id,
                        version,
                        modules,
                        type_origin_table,
                        linkage_table,
                    }),
                    (
                        ObjectType::Struct(type_),
                        ReadableObjectData::Move(ReadableMoveStruct {
                            has_public_transfer,
                            contents,
                        }),
                    ) => {
                        // check id matches in contents
                        if !id_opt(&contents).is_some_and(|id| id == object_id) {
                            return Err(serde::de::Error::custom("id from contents doesn't match"));
                        }

                        ObjectData::Struct(MoveStruct {
                            type_,
                            has_public_transfer,
                            version,
                            contents,
                        })
                    }
                    _ => return Err(serde::de::Error::custom("type and data don't match")),
                };

                Ok(GenesisObject { data, owner })
            } else {
                let BinaryGenesisObject::RawObject { data, owner } =
                    Deserialize::deserialize(deserializer)?;

                Ok(GenesisObject { data, owner })
            }
        }
    }

    #[cfg(test)]
    mod test {
        use crate::types::object::Object;

        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen_test::wasm_bindgen_test as test;

        #[test]
        fn object_fixture() {
            const SUI_COIN: &[u8] = &[
                0, 1, 1, 32, 79, 43, 0, 0, 0, 0, 0, 40, 35, 95, 175, 213, 151, 87, 206, 190, 35,
                131, 79, 35, 254, 22, 15, 181, 40, 108, 28, 77, 68, 229, 107, 254, 191, 160, 196,
                186, 42, 2, 122, 53, 52, 133, 199, 58, 0, 0, 0, 0, 0, 79, 255, 208, 0, 85, 34, 190,
                75, 192, 41, 114, 76, 127, 15, 110, 215, 9, 58, 107, 243, 160, 155, 144, 230, 47,
                97, 220, 21, 24, 30, 26, 62, 32, 17, 197, 192, 38, 64, 173, 142, 143, 49, 111, 15,
                211, 92, 84, 48, 160, 243, 102, 229, 253, 251, 137, 210, 101, 119, 173, 228, 51,
                141, 20, 15, 85, 96, 19, 15, 0, 0, 0, 0, 0,
            ];

            const SUI_STAKE: &[u8] = &[
                0, 2, 1, 154, 1, 52, 5, 0, 0, 0, 0, 80, 3, 112, 71, 231, 166, 234, 205, 164, 99,
                237, 29, 56, 97, 170, 21, 96, 105, 158, 227, 122, 22, 251, 60, 162, 12, 97, 151,
                218, 71, 253, 231, 239, 116, 138, 12, 233, 128, 195, 128, 77, 33, 38, 122, 77, 53,
                154, 197, 198, 75, 212, 12, 182, 163, 224, 42, 82, 123, 69, 248, 40, 207, 143, 211,
                13, 106, 1, 0, 0, 0, 0, 0, 0, 59, 81, 183, 246, 112, 0, 0, 0, 0, 79, 255, 208, 0,
                85, 34, 190, 75, 192, 41, 114, 76, 127, 15, 110, 215, 9, 58, 107, 243, 160, 155,
                144, 230, 47, 97, 220, 21, 24, 30, 26, 62, 32, 247, 239, 248, 71, 247, 102, 190,
                149, 232, 153, 138, 67, 169, 209, 203, 29, 255, 215, 223, 57, 159, 44, 40, 218,
                166, 13, 80, 71, 14, 188, 232, 68, 0, 0, 0, 0, 0, 0, 0, 0,
            ];

            const NFT: &[u8] = &[
                0, 0, 97, 201, 195, 159, 216, 97, 133, 173, 96, 215, 56, 212, 229, 43, 208, 139,
                218, 7, 29, 54, 106, 205, 224, 126, 7, 195, 145, 106, 45, 117, 168, 22, 12, 100,
                105, 115, 116, 114, 105, 98, 117, 116, 105, 111, 110, 11, 68, 69, 69, 80, 87, 114,
                97, 112, 112, 101, 114, 0, 0, 124, 24, 223, 4, 0, 0, 0, 0, 40, 31, 8, 18, 84, 38,
                164, 252, 84, 115, 250, 246, 137, 132, 128, 186, 156, 36, 62, 18, 140, 21, 4, 90,
                209, 105, 85, 84, 92, 214, 97, 81, 207, 64, 194, 198, 208, 21, 0, 0, 0, 0, 79, 255,
                208, 0, 85, 34, 190, 75, 192, 41, 114, 76, 127, 15, 110, 215, 9, 58, 107, 243, 160,
                155, 144, 230, 47, 97, 220, 21, 24, 30, 26, 62, 32, 170, 4, 94, 114, 207, 155, 31,
                80, 62, 254, 220, 206, 240, 218, 83, 54, 204, 197, 255, 239, 41, 66, 199, 150, 56,
                189, 86, 217, 166, 216, 128, 241, 64, 205, 21, 0, 0, 0, 0, 0,
            ];

            const FUD_COIN: &[u8] = &[
                0, 3, 7, 118, 203, 129, 155, 1, 171, 237, 80, 43, 238, 138, 112, 43, 76, 45, 84,
                117, 50, 193, 47, 37, 0, 28, 157, 234, 121, 90, 94, 99, 28, 38, 241, 3, 102, 117,
                100, 3, 70, 85, 68, 0, 1, 193, 89, 252, 3, 0, 0, 0, 0, 40, 33, 214, 90, 11, 56,
                243, 115, 10, 250, 121, 250, 28, 34, 237, 104, 130, 148, 40, 130, 29, 248, 137,
                244, 27, 138, 94, 150, 28, 182, 104, 162, 185, 0, 152, 247, 62, 93, 1, 0, 0, 0, 42,
                95, 32, 226, 13, 31, 128, 91, 188, 127, 235, 12, 75, 73, 116, 112, 3, 227, 244,
                126, 59, 81, 214, 118, 144, 243, 195, 17, 82, 216, 119, 170, 32, 239, 247, 71, 249,
                241, 98, 133, 53, 46, 37, 100, 242, 94, 231, 241, 184, 8, 69, 192, 69, 67, 1, 116,
                251, 229, 226, 99, 119, 79, 255, 71, 43, 64, 242, 19, 0, 0, 0, 0, 0,
            ];

            const BULLSHARK_PACKAGE: &[u8] = &[
                1, 135, 35, 29, 28, 138, 126, 114, 145, 204, 122, 145, 8, 244, 199, 188, 26, 10,
                28, 14, 182, 55, 91, 91, 97, 10, 245, 202, 35, 223, 14, 140, 86, 1, 0, 0, 0, 0, 0,
                0, 0, 1, 9, 98, 117, 108, 108, 115, 104, 97, 114, 107, 162, 6, 161, 28, 235, 11, 6,
                0, 0, 0, 10, 1, 0, 12, 2, 12, 36, 3, 48, 61, 4, 109, 12, 5, 121, 137, 1, 7, 130, 2,
                239, 1, 8, 241, 3, 96, 6, 209, 4, 82, 10, 163, 5, 5, 12, 168, 5, 75, 0, 7, 1, 16,
                2, 9, 2, 21, 2, 22, 2, 23, 0, 0, 2, 0, 1, 3, 7, 1, 0, 0, 2, 1, 12, 1, 0, 1, 2, 2,
                12, 1, 0, 1, 2, 4, 12, 1, 0, 1, 4, 5, 2, 0, 5, 6, 7, 0, 0, 12, 0, 1, 0, 0, 13, 2,
                1, 0, 0, 8, 3, 1, 0, 1, 20, 7, 8, 1, 0, 2, 8, 18, 19, 1, 0, 2, 10, 10, 11, 1, 2, 2,
                14, 17, 1, 1, 0, 3, 17, 7, 1, 1, 12, 3, 18, 16, 1, 1, 12, 4, 19, 13, 14, 0, 5, 15,
                5, 6, 0, 3, 6, 5, 9, 7, 12, 8, 15, 6, 9, 4, 9, 2, 8, 0, 7, 8, 5, 0, 4, 7, 11, 4, 1,
                8, 0, 3, 5, 7, 8, 5, 2, 7, 11, 4, 1, 8, 0, 11, 2, 1, 8, 0, 2, 11, 3, 1, 8, 0, 11,
                4, 1, 8, 0, 1, 10, 2, 1, 8, 6, 1, 9, 0, 1, 11, 1, 1, 9, 0, 1, 8, 0, 7, 9, 0, 2, 10,
                2, 10, 2, 10, 2, 11, 1, 1, 8, 6, 7, 8, 5, 2, 11, 4, 1, 9, 0, 11, 3, 1, 9, 0, 1, 11,
                3, 1, 8, 0, 1, 6, 8, 5, 1, 5, 1, 11, 4, 1, 8, 0, 2, 9, 0, 5, 4, 7, 11, 4, 1, 9, 0,
                3, 5, 7, 8, 5, 2, 7, 11, 4, 1, 9, 0, 11, 2, 1, 9, 0, 1, 3, 9, 66, 85, 76, 76, 83,
                72, 65, 82, 75, 4, 67, 111, 105, 110, 12, 67, 111, 105, 110, 77, 101, 116, 97, 100,
                97, 116, 97, 6, 79, 112, 116, 105, 111, 110, 11, 84, 114, 101, 97, 115, 117, 114,
                121, 67, 97, 112, 9, 84, 120, 67, 111, 110, 116, 101, 120, 116, 3, 85, 114, 108, 9,
                98, 117, 108, 108, 115, 104, 97, 114, 107, 4, 98, 117, 114, 110, 4, 99, 111, 105,
                110, 15, 99, 114, 101, 97, 116, 101, 95, 99, 117, 114, 114, 101, 110, 99, 121, 11,
                100, 117, 109, 109, 121, 95, 102, 105, 101, 108, 100, 4, 105, 110, 105, 116, 4,
                109, 105, 110, 116, 17, 109, 105, 110, 116, 95, 97, 110, 100, 95, 116, 114, 97,
                110, 115, 102, 101, 114, 21, 110, 101, 119, 95, 117, 110, 115, 97, 102, 101, 95,
                102, 114, 111, 109, 95, 98, 121, 116, 101, 115, 6, 111, 112, 116, 105, 111, 110,
                20, 112, 117, 98, 108, 105, 99, 95, 102, 114, 101, 101, 122, 101, 95, 111, 98, 106,
                101, 99, 116, 15, 112, 117, 98, 108, 105, 99, 95, 116, 114, 97, 110, 115, 102, 101,
                114, 6, 115, 101, 110, 100, 101, 114, 4, 115, 111, 109, 101, 8, 116, 114, 97, 110,
                115, 102, 101, 114, 10, 116, 120, 95, 99, 111, 110, 116, 101, 120, 116, 3, 117,
                114, 108, 135, 35, 29, 28, 138, 126, 114, 145, 204, 122, 145, 8, 244, 199, 188, 26,
                10, 28, 14, 182, 55, 91, 91, 97, 10, 245, 202, 35, 223, 14, 140, 86, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 10, 2, 10, 9, 66, 85, 76, 76, 83, 72, 65, 82, 75, 10, 2, 20, 19, 66, 117,
                108, 108, 32, 83, 104, 97, 114, 107, 32, 83, 117, 105, 70, 114, 101, 110, 115, 10,
                2, 1, 0, 10, 2, 39, 38, 104, 116, 116, 112, 115, 58, 47, 47, 105, 46, 105, 98, 98,
                46, 99, 111, 47, 104, 87, 89, 50, 87, 53, 120, 47, 98, 117, 108, 108, 115, 104, 97,
                114, 107, 46, 112, 110, 103, 0, 2, 1, 11, 1, 0, 0, 0, 0, 4, 20, 11, 0, 49, 6, 7, 0,
                7, 1, 7, 2, 7, 3, 17, 10, 56, 0, 10, 1, 56, 1, 12, 2, 12, 3, 11, 2, 56, 2, 11, 3,
                11, 1, 46, 17, 9, 56, 3, 2, 1, 1, 4, 0, 1, 6, 11, 0, 11, 1, 11, 2, 11, 3, 56, 4, 2,
                2, 1, 4, 0, 1, 5, 11, 0, 11, 1, 56, 5, 1, 2, 0, 1, 9, 98, 117, 108, 108, 115, 104,
                97, 114, 107, 9, 66, 85, 76, 76, 83, 72, 65, 82, 75, 135, 35, 29, 28, 138, 126,
                114, 145, 204, 122, 145, 8, 244, 199, 188, 26, 10, 28, 14, 182, 55, 91, 91, 97, 10,
                245, 202, 35, 223, 14, 140, 86, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 2, 4, 0, 0, 0, 0, 0, 0, 0, 3, 32, 87, 145, 191, 231, 147, 185,
                46, 159, 240, 181, 95, 126, 236, 65, 154, 55, 16, 196, 229, 218, 47, 59, 99, 197,
                13, 89, 18, 159, 205, 129, 112, 131, 112, 192, 126, 0, 0, 0, 0, 0,
            ];

            for fixture in [SUI_COIN, SUI_STAKE, NFT, FUD_COIN, BULLSHARK_PACKAGE] {
                let object: Object = bcs::from_bytes(fixture).unwrap();
                assert_eq!(bcs::to_bytes(&object).unwrap(), fixture);

                let json = serde_json::to_string_pretty(&object).unwrap();
                println!("{json}");
                assert_eq!(object, serde_json::from_str(&json).unwrap());
            }
        }
    }
}
