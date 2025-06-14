use sui_types::Address;
use sui_types::Command;
use sui_types::ObjectDigest;
use sui_types::ObjectId;
use sui_types::TransactionExpiration;
use sui_types::Version;

// A potentially unresolved user transaction. Note that one can construct a fully resolved
// transaction using this type by providing all the required data.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "UnresolvedTransaction")]
pub struct Transaction {
    #[serde(flatten)]
    pub ptb: ProgrammableTransaction,
    pub sender: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_payment: Option<GasPayment>,
    pub expiration: TransactionExpiration,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "UnresolvedProgrammableTransaction")]
pub struct ProgrammableTransaction {
    pub inputs: Vec<Input>,
    pub commands: Vec<Command>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "UnresolvedGasPayment")]
pub struct GasPayment {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub objects: Vec<ObjectReference>,
    pub owner: Address,
    #[serde(
        with = "OptionReadableDisplay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<u64>,
    #[serde(
        with = "OptionReadableDisplay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub budget: Option<u64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename = "UnresolvedObjectReference")]
pub struct ObjectReference {
    pub object_id: ObjectId,
    #[serde(
        with = "OptionReadableDisplay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version: Option<Version>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<ObjectDigest>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "UnresolvedInputKind")]
#[serde(rename_all = "snake_case")]
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
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "UnresolvedInput")]
pub struct Input {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<InputKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for this object.
    pub object_id: Option<ObjectId>,
    /// Either the `initial_shared_version` if object is a shared object, or the `version` if
    /// this is an owned object.
    /// The semantics of version can change depending on whether the object is shared or not.
    /// For shared objects, this is the initial version the object was shared at. For all other
    /// objects, this is the version of the object.
    #[serde(
        with = "OptionReadableDisplay",
        default,
        skip_serializing_if = "Option::is_none",
        alias = "initial_shared_version"
    )]
    pub version: Option<Version>,
    /// The digest of this object. This field is only relevant for owned/immutable/receiving
    /// inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<ObjectDigest>,
    /// Whether this object is mutable. This field is only relevant for shared objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutable: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "UnresolvedValue")]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
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
    pub fn with_immutable_kind(self) -> Self {
        Self {
            kind: Some(InputKind::ImmutableOrOwned),
            ..self
        }
    }

    /// Set the object kind to owned.
    pub fn with_owned_kind(self) -> Self {
        Self {
            kind: Some(InputKind::ImmutableOrOwned),
            ..self
        }
    }

    /// Set the object kind to receiving.
    pub fn with_receiving_kind(self) -> Self {
        Self {
            kind: Some(InputKind::Receiving),
            ..self
        }
    }

    /// Set the object kind to shared.
    pub fn with_shared_kind(self) -> Self {
        Self {
            kind: Some(InputKind::Shared),
            ..self
        }
    }

    /// Set the specified version.
    pub fn with_version(self, version: u64) -> Self {
        Self {
            version: Some(version),
            ..self
        }
    }

    /// Set the specified digest.
    pub fn with_digest(self, digest: ObjectDigest) -> Self {
        Self {
            digest: Some(digest),
            ..self
        }
    }

    // Shared fields

    /// Set the initial shared version.
    pub fn with_initial_shared_version(self, initial_version: u64) -> Self {
        Self {
            kind: Some(InputKind::Shared),
            version: Some(initial_version),
            ..self
        }
    }

    /// Make the object shared and set `mutable` to true when the input is used by value.
    pub fn by_val(self) -> Self {
        Self {
            kind: Some(InputKind::Shared),
            mutable: Some(true),
            ..self
        }
    }

    /// Make the object shared and set `mutable` to false when the input is used by
    /// reference.
    pub fn by_ref(self) -> Self {
        Self {
            kind: Some(InputKind::Shared),
            mutable: Some(false),
            ..self
        }
    }

    /// Make the object shared and set `mutable` to true when the input is used by mutable
    /// reference.
    pub fn by_mut(self) -> Self {
        Self {
            kind: Some(InputKind::Shared),
            mutable: Some(true),
            ..self
        }
    }
}

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

impl From<&sui_types::Object> for Input {
    fn from(object: &sui_types::Object) -> Self {
        use sui_types::Owner;

        let input = Input::by_id(object.object_id())
            .with_digest(object.digest())
            .with_version(object.version());
        match object.owner() {
            Owner::Address(_) => input,
            Owner::Object(_) => input,
            Owner::Shared(at_version) => input.with_initial_shared_version(*at_version),
            Owner::Immutable => input.with_immutable_kind(),
            Owner::ConsensusAddress { start_version, .. } => {
                input.with_initial_shared_version(*start_version)
            }
        }
    }
}

impl From<ObjectId> for Input {
    fn from(object_id: ObjectId) -> Self {
        Input::by_id(object_id)
    }
}

pub(crate) type OptionReadableDisplay =
    ::serde_with::As<Option<::serde_with::IfIsHumanReadable<::serde_with::DisplayFromStr>>>;
