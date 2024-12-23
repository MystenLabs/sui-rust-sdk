use super::*;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde_with::DeserializeAs;
use serde_with::SerializeAs;

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serde_with::DisplayFromStr::serialize_as(self, serializer)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_with::DisplayFromStr::deserialize_as(deserializer)
    }
}

#[repr(u32)]
enum SerializedTypeTagVariant {
    Bool = 0,
    U8 = 1,
    U64 = 2,
    U128 = 3,
    Address = 4,
    Signer = 5,
    Vector = 6,
    Struct = 7,
    U16 = 8,
    U32 = 9,
    U256 = 10,
}

impl SerializedTypeTagVariant {
    fn new(variant: u32) -> Result<Self, u32> {
        Ok(match variant {
            0 => Self::Bool,
            1 => Self::U8,
            2 => Self::U64,
            3 => Self::U128,
            4 => Self::Address,
            5 => Self::Signer,
            6 => Self::Vector,
            7 => Self::Struct,
            8 => Self::U16,
            9 => Self::U32,
            10 => Self::U256,
            unknown => return Err(unknown),
        })
    }
}

impl Serialize for TypeTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serde_with::DisplayFromStr::serialize_as(self, serializer)
        } else {
            match self {
                TypeTag::U8 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U8 as u32,
                    "u8",
                ),
                TypeTag::U16 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U16 as u32,
                    "u16",
                ),
                TypeTag::U32 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U32 as u32,
                    "u32",
                ),
                TypeTag::U64 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U64 as u32,
                    "u64",
                ),
                TypeTag::U128 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U128 as u32,
                    "u128",
                ),
                TypeTag::U256 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U256 as u32,
                    "u256",
                ),
                TypeTag::Bool => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::Bool as u32,
                    "bool",
                ),
                TypeTag::Address => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::Address as u32,
                    "address",
                ),
                TypeTag::Signer => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::Signer as u32,
                    "signer",
                ),
                TypeTag::Vector(v) => serializer.serialize_newtype_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::Vector as u32,
                    "vector",
                    v,
                ),
                TypeTag::Struct(s) => serializer.serialize_newtype_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::Struct as u32,
                    "struct",
                    s,
                ),
            }
        }
    }
}

impl<'de> Deserialize<'de> for TypeTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            serde_with::DisplayFromStr::deserialize_as(deserializer)
        } else {
            let variants = &[
                "bool", "u8", "u64", "u128", "address", "signer", "vector", "struct", "u16", "u32",
                "u256",
            ];
            deserializer.deserialize_enum("TypeTag", variants, TypeTagVisitor)
        }
    }
}

struct TypeTagVisitor;

impl<'de> Visitor<'de> for TypeTagVisitor {
    type Value = TypeTag;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("TypeTag")
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        use serde::de::VariantAccess;

        let (variant, deserializer) = data.variant::<u32>()?;
        match SerializedTypeTagVariant::new(variant)
            .map_err(|e| serde::de::Error::custom(format!("unknown variant {e}")))?
        {
            SerializedTypeTagVariant::Bool => deserializer.unit_variant().map(|_| TypeTag::Bool),
            SerializedTypeTagVariant::U8 => deserializer.unit_variant().map(|_| TypeTag::U8),
            SerializedTypeTagVariant::U64 => deserializer.unit_variant().map(|_| TypeTag::U64),
            SerializedTypeTagVariant::U128 => deserializer.unit_variant().map(|_| TypeTag::U128),
            SerializedTypeTagVariant::Address => {
                deserializer.unit_variant().map(|_| TypeTag::Address)
            }
            SerializedTypeTagVariant::Signer => {
                deserializer.unit_variant().map(|_| TypeTag::Signer)
            }
            SerializedTypeTagVariant::Vector => deserializer.newtype_variant().map(TypeTag::Vector),
            SerializedTypeTagVariant::Struct => deserializer.newtype_variant().map(TypeTag::Struct),
            SerializedTypeTagVariant::U16 => deserializer.unit_variant().map(|_| TypeTag::U16),
            SerializedTypeTagVariant::U32 => deserializer.unit_variant().map(|_| TypeTag::U32),
            SerializedTypeTagVariant::U256 => deserializer.unit_variant().map(|_| TypeTag::U256),
        }
    }
}

#[derive(serde_derive::Serialize)]
struct BinaryStructTagRef<'a> {
    address: &'a Address,
    module: &'a Identifier,
    name: &'a Identifier,
    type_params: &'a [TypeTag],
}

impl Serialize for StructTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serde_with::DisplayFromStr::serialize_as(self, serializer)
        } else {
            BinaryStructTagRef {
                address: &self.address,
                module: &self.module,
                name: &self.name,
                type_params: &self.type_params,
            }
            .serialize(serializer)
        }
    }
}

#[derive(serde_derive::Deserialize)]
struct BinaryStructTag {
    address: Address,
    module: Identifier,
    name: Identifier,
    type_params: Vec<TypeTag>,
}

impl<'de> Deserialize<'de> for StructTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            serde_with::DisplayFromStr::deserialize_as(deserializer)
        } else {
            let BinaryStructTag {
                address,
                module,
                name,
                type_params,
            } = Deserialize::deserialize(deserializer)?;
            Ok(Self {
                address,
                module,
                name,
                type_params,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::str::FromStr;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn type_tag_fixture() {
        let expected = TypeTag::Struct(Box::new(StructTag {
            address: Address::from_str("0x1").unwrap(),
            module: Identifier("Foo".into()),
            name: Identifier("Bar".into()),
            type_params: vec![
                TypeTag::Bool,
                TypeTag::U8,
                TypeTag::U64,
                TypeTag::U128,
                TypeTag::Address,
                TypeTag::Signer,
                TypeTag::U16,
                TypeTag::U32,
                TypeTag::U256,
                TypeTag::Vector(Box::new(TypeTag::Address)),
            ],
        }));

        let display = "0x0000000000000000000000000000000000000000000000000000000000000001::Foo::Bar<bool,u8,u64,u128,address,signer,u16,u32,u256,vector<address>>";
        let bcs_fixture: &[u8] = &[
            7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 3, 70, 111, 111, 3, 66, 97, 114, 10, 0, 1, 2, 3, 4, 5, 8, 9, 10, 6, 4,
        ];

        let type_from_string = TypeTag::from_str(display).unwrap();
        assert_eq!(type_from_string, expected);
        let type_from_bcs: TypeTag = bcs::from_bytes(bcs_fixture).unwrap();
        assert_eq!(type_from_bcs, expected);

        assert_eq!(type_from_string, type_from_bcs);
        assert_eq!(bcs_fixture, bcs::to_bytes(&type_from_bcs).unwrap());
        assert_eq!(bcs_fixture, bcs::to_bytes(&type_from_string).unwrap());
        assert_eq!(display, type_from_string.to_string().replace(' ', ""));
    }
}
