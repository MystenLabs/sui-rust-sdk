mod parse;

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization;

use super::Address;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Hash)]
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
        parse::parse_type_tag(s).map_err(|_| TypeParseError)
    }
}

//TODO flesh out this error type
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeParseError;

impl std::fmt::Display for TypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for TypeParseError {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(Box<str>);

impl Identifier {
    pub fn new<T: AsRef<str>>(identifier: T) -> Result<Self, TypeParseError> {
        parse::parse_identifier(identifier.as_ref())
            .map(|ident| Self(ident.into()))
            .map_err(|_| TypeParseError)
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
            .map_err(|_| TypeParseError)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructTag {
    pub address: Address,
    pub module: Identifier,
    pub name: Identifier,
    pub type_params: Vec<TypeTag>,
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
        parse::parse_struct_tag(s).map_err(|_| TypeParseError)
    }
}
