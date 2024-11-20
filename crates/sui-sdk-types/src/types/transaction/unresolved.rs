use crate::types::object::Version;
use crate::types::Address;
use crate::types::ObjectDigest;
use crate::types::ObjectId;

use super::Command;
use super::TransactionExpiration;

// A potentially unresolved user transaction. Note that one can construct a fully resolved
// transaction using this type by providing all the required data.
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename = "UnresolvedTransaction")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Transaction {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub ptb: ProgrammableTransaction,
    pub sender: Address,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub gas_payment: Option<GasPayment>,
    pub expiration: TransactionExpiration,
}

#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename = "UnresolvedProgrammableTransaction")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ProgrammableTransaction {
    pub inputs: Vec<Input>,
    pub commands: Vec<Command>,
}

#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename = "UnresolvedGasPayment")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct GasPayment {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub objects: Vec<ObjectReference>,
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

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename = "UnresolvedObjectReference")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ObjectReference {
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
    serde(rename = "UnresolvedInputKind"),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum InputKind {
    Pure,
    Shared,
    Receiving,
    ImmutableOrOwned,
    Literal,
}

/// A potentially unresolved transaction input. Note that one can construct a fully resolved input
/// using the provided constructors, but this struct is also useful when the input data is
/// not complete.
///
/// If used in the context of transaction builder, make sure to call `tx.resolve` function on the
/// transaction builder to resolve all unresolved inputs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename = "UnresolvedInput"),
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Input {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub kind: Option<InputKind>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub value: Option<Value>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    /// Unique identifier for this object.
    pub object_id: Option<ObjectId>,
    /// Either the `initial_shared_version` if object is a shared object, or the `version` if
    /// this is an owned object.
    /// The semantics of version can change depending on whether the object is shared or not.
    /// For shared objects, this is the initial version the object was shared at. For all other
    /// objects, this is the version of the object.
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
    /// The digest of this object. This field is only relevant for owned/immutable/receiving
    /// inputs.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub digest: Option<ObjectDigest>,
    /// Whether this object is mutable. This field is only relevant for shared objects.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none",))]
    pub mutable: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(rename = "UnresolvedValue"),
    serde(try_from = "serde_json::Value", into = "serde_json::Value")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(untagged))]
pub enum Value {
    Null,
    Bool(bool),
    Number(u64),
    String(String),
    Array(Vec<Value>),
}

impl Input {
    /// Return an owned kind of object with all required fields.
    pub fn owned(object_id: ObjectId, version: u64, digest: ObjectDigest) -> Self {
        Self {
            kind: Some(InputKind::ImmutableOrOwned),
            object_id: Some(object_id),
            version: Some(version),
            digest: Some(digest),
            ..Default::default()
        }
    }

    /// Return an immutable kind of object with all required fields.
    pub fn immutable(object_id: ObjectId, version: u64, digest: ObjectDigest) -> Self {
        Self {
            kind: Some(InputKind::ImmutableOrOwned),
            object_id: Some(object_id),
            version: Some(version),
            digest: Some(digest),
            ..Default::default()
        }
    }

    /// Return a receiving kind of object with all required fields.
    pub fn receiving(object_id: ObjectId, version: u64, digest: ObjectDigest) -> Self {
        Self {
            kind: Some(InputKind::Receiving),
            object_id: Some(object_id),
            version: Some(version),
            digest: Some(digest),
            ..Default::default()
        }
    }

    /// Return a shared object.
    /// - `mutable` controls whether a command can accept the object by value or mutable reference.
    /// - `initial_shared_version` is the first version the object was shared at.
    pub fn shared(object_id: ObjectId, initial_shared_version: u64, mutable: bool) -> Self {
        Self {
            kind: Some(InputKind::Shared),
            object_id: Some(object_id),
            version: Some(initial_shared_version),
            mutable: Some(mutable),
            ..Default::default()
        }
    }

    /// Return an object with only its unique identifier.
    pub fn by_id(object_id: ObjectId) -> Self {
        Self {
            object_id: Some(object_id),
            ..Default::default()
        }
    }

    /// Set the object kind to immutable.
    pub fn with_immutable_kind(&mut self) {
        self.kind = Some(InputKind::ImmutableOrOwned);
    }

    /// Set the object kind to owned.
    pub fn with_owned_kind(&mut self) {
        self.kind = Some(InputKind::ImmutableOrOwned);
    }

    /// Set the object kind to receiving.
    pub fn with_receiving_kind(&mut self) {
        self.kind = Some(InputKind::Receiving);
    }

    /// Set the object kind to shared.
    pub fn with_shared_kind(&mut self) {
        self.kind = Some(InputKind::Shared);
    }

    /// Set the specified version.
    pub fn with_version(&mut self, version: u64) {
        self.version = Some(version);
    }

    /// Set the specified digest.
    pub fn with_digest(&mut self, digest: ObjectDigest) {
        self.digest = Some(digest);
    }

    // Shared fields

    /// Set the initial shared version.
    pub fn with_initial_shared_version(&mut self, initial: u64) {
        self.version = Some(initial);
    }

    /// Make the object shared and set `mutable` to true when the input is used by value.
    pub fn by_val(&mut self) {
        self.kind = Some(InputKind::Shared);
        self.mutable = Some(true);
    }

    /// Make the object shared and set `mutable` to false when the input is used by
    /// reference.
    pub fn by_ref(&mut self) {
        self.kind = Some(InputKind::Shared);
        self.mutable = Some(false);
    }

    /// Make the object shared and set `mutable` to true when the input is used by mutable
    /// reference.
    pub fn by_mut(&mut self) {
        self.kind = Some(InputKind::Shared);
        self.mutable = Some(true);
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl TryFrom<serde_json::Value> for Value {
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
impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Bool(b) => Self::Bool(b),
            Value::Number(n) => Self::Number(n.into()),
            Value::String(s) => Self::String(s),
            Value::Array(a) => Self::Array(a.into_iter().map(Into::into).collect()),
        }
    }
}
