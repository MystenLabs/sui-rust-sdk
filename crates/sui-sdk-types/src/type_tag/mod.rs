mod parse;

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization;

use super::Address;

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
    Box<str>,
);

impl Identifier {
    pub fn new<T: AsRef<str>>(identifier: T) -> Result<Self, TypeParseError> {
        parse::parse_identifier(identifier.as_ref())
            .map(|ident| Self(ident.into()))
            .map_err(|_| TypeParseError {
                source: identifier.as_ref().into(),
            })
    }

    pub fn into_inner(self) -> Box<str> {
        self.0
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
        self.0.as_ref() == other
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct StructTag {
    pub address: Address,
    pub module: Identifier,
    pub name: Identifier,
    #[cfg_attr(feature = "proptest", strategy(proptest::strategy::Just(Vec::new())))]
    pub type_params: Vec<TypeTag>,
}

impl StructTag {
    pub fn gas_coin() -> Self {
        let sui = Self {
            address: Address::TWO,
            module: Identifier::new("sui").unwrap(),
            name: Identifier::new("SUI").unwrap(),
            type_params: vec![],
        };

        Self::coin(TypeTag::Struct(Box::new(sui)))
    }

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

impl std::str::FromStr for StructTag {
    type Err = TypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::parse_struct_tag(s).map_err(|_| TypeParseError { source: s.into() })
    }
}
