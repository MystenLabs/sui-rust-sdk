mod parse;

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization;

use super::Address;

/// Type of a move value
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// type-tag = type-tag-u8 \
///            type-tag-u16 \
///            type-tag-u32 \
///            type-tag-u64 \
///            type-tag-u128 \
///            type-tag-u256 \
///            type-tag-bool \
///            type-tag-address \
///            type-tag-signer \
///            type-tag-vector \
///            type-tag-struct
///
/// type-tag-u8 = %x01
/// type-tag-u16 = %x08
/// type-tag-u32 = %x09
/// type-tag-u64 = %x02
/// type-tag-u128 = %x03
/// type-tag-u256 = %x0a
/// type-tag-bool = %x00
/// type-tag-address = %x04
/// type-tag-signer = %x05
/// type-tag-vector = %x06 type-tag
/// type-tag-struct = %x07 struct-tag
/// ```
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Hash)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum TypeTag {
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Bool,
    Address,
    Signer,
    #[cfg_attr(feature = "proptest", weight(0))]
    Vector(Box<TypeTag>),
    Struct(Box<StructTag>),
}

impl std::fmt::Display for TypeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeTag::U8 => write!(f, "u8"),
            TypeTag::U16 => write!(f, "u16"),
            TypeTag::U32 => write!(f, "u32"),
            TypeTag::U64 => write!(f, "u64"),
            TypeTag::U128 => write!(f, "u128"),
            TypeTag::U256 => write!(f, "u256"),
            TypeTag::Bool => write!(f, "bool"),
            TypeTag::Address => write!(f, "address"),
            TypeTag::Signer => write!(f, "signer"),
            TypeTag::Vector(t) => {
                write!(f, "vector<{t}>")
            }
            TypeTag::Struct(s) => s.fmt(f),
        }
    }
}

impl From<TypeTag> for String {
    fn from(value: TypeTag) -> Self {
        value.to_string()
    }
}

impl From<&TypeTag> for String {
    fn from(value: &TypeTag) -> Self {
        value.to_string()
    }
}

impl std::str::FromStr for TypeTag {
    type Err = TypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::parse_type_tag(s).map_err(|_| TypeParseError { source: s.into() })
    }
}

impl From<StructTag> for TypeTag {
    fn from(value: StructTag) -> Self {
        Self::Struct(Box::new(value))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeParseError {
    source: String,
}

impl std::fmt::Display for TypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for TypeParseError {}

/// A move identifier
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// identifier = %x01-80    ; length of the identifier
///              (ALPHA *127(ALPHA / DIGIT / UNDERSCORE)) /
///              (UNDERSCORE 1*127(ALPHA / DIGIT / UNDERSCORE))
///
/// UNDERSCORE = %x95
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct Identifier(
    #[cfg_attr(
        feature = "proptest",
        strategy(proptest::strategy::Strategy::prop_map(
            "[a-zA-Z][a-zA-Z0-9_]{0,127}",
            Into::into
        ))
    )]
    bytestring::ByteString,
);

impl Identifier {
    pub fn new<T: AsRef<str>>(identifier: T) -> Result<Self, TypeParseError> {
        parse::parse_identifier(identifier.as_ref())
            .map(|ident| Self(ident.into()))
            .map_err(|_| TypeParseError {
                source: identifier.as_ref().into(),
            })
    }

    /// Construct an Identifier from a `&'static str`, panicking if it is invalid.
    pub const fn from_static(identifier: &'static str) -> Self {
        if !parse::is_valid_identifier(identifier) {
            panic!("invalid identifier");
        }

        Self(bytestring::ByteString::from_static(identifier))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.to_string()
    }
}

impl From<&Identifier> for String {
    fn from(value: &Identifier) -> Self {
        value.to_string()
    }
}

impl std::str::FromStr for Identifier {
    type Err = TypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::parse_identifier(s)
            .map(|ident| Self(ident.into()))
            .map_err(|_| TypeParseError { source: s.into() })
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl std::ops::Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Type information for a move struct
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// struct-tag = address            ; address of the package
///              identifier         ; name of the module
///              identifier         ; name of the type
///              (vector type-tag)  ; type parameters
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct StructTag {
    address: Address,
    module: Identifier,
    name: Identifier,
    #[cfg_attr(feature = "proptest", strategy(proptest::strategy::Just(Vec::new())))]
    type_params: Vec<TypeTag>,
}

impl StructTag {
    pub fn new(
        address: Address,
        module: Identifier,
        name: Identifier,
        type_params: Vec<TypeTag>,
    ) -> Self {
        Self {
            address,
            module,
            name,
            type_params,
        }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn module(&self) -> &Identifier {
        &self.module
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn type_params(&self) -> &[TypeTag] {
        &self.type_params
    }

    /// Returns the struct tag for the native SUI token (without the Coin wrapper).
    /// This represents `0x2::sui::SUI`.
    pub fn sui() -> Self {
        Self {
            address: Address::TWO,
            module: Identifier::new("sui").unwrap(),
            name: Identifier::new("SUI").unwrap(),
            type_params: vec![],
        }
    }

    /// Returns the struct tag for SUI wrapped in a Coin.
    /// This represents `0x2::coin::Coin<0x2::sui::SUI>`.
    pub fn gas_coin() -> Self {
        Self::coin(Self::sui().into())
    }

    /// Wraps any type tag in a Coin type.
    /// For example, `coin(TypeTag::sui())` returns `0x2::coin::Coin<0x2::sui::SUI>`.
    pub fn coin(type_tag: TypeTag) -> Self {
        Self {
            address: Address::TWO,
            module: Identifier::new("coin").unwrap(),
            name: Identifier::new("Coin").unwrap(),
            type_params: vec![type_tag],
        }
    }

    pub fn staked_sui() -> Self {
        Self {
            address: Address::THREE,
            module: Identifier::new("staking_pool").unwrap(),
            name: Identifier::new("StakedSui").unwrap(),
            type_params: vec![],
        }
    }

    /// Checks if this is a Coin type
    pub fn is_coin(&self) -> Option<&TypeTag> {
        let Self {
            address,
            module,
            name,
            type_params,
        } = self;

        if address == &Address::TWO && module == "coin" && name == "Coin" && type_params.len() == 1
        {
            type_params.first()
        } else {
            None
        }
    }

    #[cfg(feature = "serde")]
    pub(crate) fn balance_accumulator_field(coin_type: TypeTag) -> Self {
        let u128_type = Self {
            address: Address::TWO,
            module: Identifier::from_static("accumulator"),
            name: Identifier::from_static("U128"),
            type_params: vec![],
        };

        let balance_type = Self {
            address: Address::TWO,
            module: Identifier::from_static("balance"),
            name: Identifier::from_static("Balance"),
            type_params: vec![coin_type],
        };

        let key_type = Self {
            address: Address::TWO,
            module: Identifier::from_static("accumulator"),
            name: Identifier::from_static("Key"),
            type_params: vec![balance_type.into()],
        };

        Self {
            address: Address::TWO,
            module: Identifier::from_static("dynamic_field"),
            name: Identifier::from_static("Field"),
            type_params: vec![key_type.into(), u128_type.into()],
        }
    }

    #[cfg(feature = "serde")]
    pub(crate) fn is_balance_accumulator_field(&self) -> Option<&TypeTag> {
        let (key_type, u128_type) = if self.address() == &Address::TWO
            && self.module() == "dynamic_field"
            && self.name() == "Field"
            && let [TypeTag::Struct(key_type), TypeTag::Struct(u128_type)] = self.type_params()
        {
            (key_type, u128_type)
        } else {
            return None;
        };

        if !(u128_type.address() == &Address::TWO
            && u128_type.module() == "accumulator"
            && u128_type.name() == "U128"
            && u128_type.type_params().is_empty())
        {
            return None;
        }

        let balance_type = if key_type.address() == &Address::TWO
            && key_type.module() == "accumulator"
            && key_type.name() == "Key"
            && let [TypeTag::Struct(balance_type)] = key_type.type_params()
        {
            balance_type
        } else {
            return None;
        };

        if balance_type.address() == &Address::TWO
            && balance_type.module() == "balance"
            && balance_type.name() == "Balance"
            && let [coin_type] = balance_type.type_params()
        {
            Some(coin_type)
        } else {
            None
        }
    }

    #[cfg(feature = "serde")]
    pub(crate) fn is_gas(&self) -> bool {
        self.address() == &Address::TWO
            && self.module() == "sui"
            && self.name() == "SUI"
            && self.type_params().is_empty()
    }
}

impl std::fmt::Display for StructTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}::{}", self.address, self.module, self.name)?;

        if let Some(first_type) = self.type_params.first() {
            write!(f, "<")?;
            write!(f, "{first_type}")?;
            for ty in self.type_params.iter().skip(1) {
                write!(f, ", {ty}")?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

impl From<StructTag> for String {
    fn from(value: StructTag) -> Self {
        value.to_string()
    }
}

impl From<&StructTag> for String {
    fn from(value: &StructTag) -> Self {
        value.to_string()
    }
}

impl std::str::FromStr for StructTag {
    type Err = TypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::parse_struct_tag(s).map_err(|_| TypeParseError { source: s.into() })
    }
}
