use crate::types::object::Version;
use crate::types::Address;
use crate::types::ObjectDigest;
use crate::types::ObjectId;

use super::Command;
use super::TransactionExpiration;

// A potentially Unresolved user transaction
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct UnresolvedTransaction {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub ptb: UnresolvedProgrammableTransaction,
    pub sender: Address,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub gas_payment: Option<UnresolvedGasPayment>,
    pub expiration: TransactionExpiration,
}

#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct UnresolvedProgrammableTransaction {
    pub inputs: Vec<UnresolvedInputArgument>,
    pub commands: Vec<Command>,
}

#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct UnresolvedGasPayment {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub objects: Vec<UnresolvedObjectReference>,
    pub owner: Address,
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "crate::_serde::OptionReadableDisplay",
            default,
            skip_serializing_if = "Option::is_none",
        )
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<crate::_schemars::U64>"))]
    pub price: Option<u64>,
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "crate::_serde::OptionReadableDisplay",
            default,
            skip_serializing_if = "Option::is_none",
        )
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<crate::_schemars::U64>"))]
    pub budget: Option<u64>,
}

#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct UnresolvedObjectReference {
    pub object_id: ObjectId,
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "crate::_serde::OptionReadableDisplay",
            default,
            skip_serializing_if = "Option::is_none",
        )
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<crate::_schemars::U64>"))]
    pub version: Option<Version>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub digest: Option<ObjectDigest>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum UnresolvedInputArgumentKind {
    Pure,
    Shared,
    Receiving,
    ImmutableOrOwned,
    Immutable,
    Owned,
    Literal,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct UnresolvedInputArgument {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub kind: Option<UnresolvedInputArgumentKind>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub value: Option<UnresolvedValue>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub object_id: Option<ObjectId>,
    /// Either the `initial_shared_version` if object is a shared object, or the `version` if
    /// this is an owned object
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "crate::_serde::OptionReadableDisplay",
            default,
            skip_serializing_if = "Option::is_none",
            alias = "initial_shared_version",
        )
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<crate::_schemars::U64>"))]
    pub version: Option<Version>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub digest: Option<ObjectDigest>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub mutable: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(try_from = "serde_json::Value", into = "serde_json::Value")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(untagged))]
pub enum UnresolvedValue {
    Null,
    Bool(bool),
    Number(u64),
    String(String),
    Array(Vec<UnresolvedValue>),
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl TryFrom<serde_json::Value> for UnresolvedValue {
    type Error = &'static str;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let v = match value {
            serde_json::Value::Null => Self::Null,
            serde_json::Value::Bool(b) => Self::Bool(b),
            serde_json::Value::Number(n) => {
                Self::Number(n.as_u64().ok_or("expected unsigned integer")?)
            }
            serde_json::Value::String(s) => Self::String(s),
            serde_json::Value::Array(a) => Self::Array(
                a.into_iter()
                    .map(Self::try_from)
                    .collect::<Result<_, _>>()?,
            ),
            serde_json::Value::Object(_) => return Err("objects are not supported"),
        };

        Ok(v)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl From<UnresolvedValue> for serde_json::Value {
    fn from(value: UnresolvedValue) -> Self {
        match value {
            UnresolvedValue::Null => Self::Null,
            UnresolvedValue::Bool(b) => Self::Bool(b),
            UnresolvedValue::Number(n) => Self::Number(n.into()),
            UnresolvedValue::String(s) => Self::String(s),
            UnresolvedValue::Array(a) => Self::Array(a.into_iter().map(Into::into).collect()),
        }
    }
}
