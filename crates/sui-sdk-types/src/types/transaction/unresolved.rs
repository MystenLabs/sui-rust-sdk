use crate::types::object::Version;
use crate::types::Address;
use crate::types::ObjectDigest;
use crate::types::ObjectId;
use crate::types::ObjectReference;

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
    pub objects: Vec<UnresolvedObjectReference>,
    pub owner: Address,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::_serde::OptionReadableDisplay")
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<crate::_schemars::U64>"))]
    pub price: Option<u64>,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::_serde::OptionReadableDisplay")
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
        serde(with = "crate::_serde::OptionReadableDisplay")
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<crate::_schemars::U64>"))]
    pub version: Option<Version>,
    pub digest: Option<ObjectDigest>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub enum UnresolvedInputArgument {
    // contains no structs or objects
    Pure {
        #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::Base64"))]
        value: Vec<u8>,
    },
    // // A Move object, either immutable, or owned mutable.
    // ImmutableOrOwned(ObjectReference),
    // // A Move object that's shared.
    // // SharedObject::mutable controls whether caller asks for a mutable reference to shared object.
    // Shared {
    //     object_id: ObjectId,
    //     #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::U64"))]
    //     initial_shared_version: u64,
    //     mutable: bool,
    // },
    // // A Move object that can be received in this transaction.
    // Receiving(ObjectReference),
    UnresolvedObject {
        object_id: ObjectId,
        #[cfg_attr(
            feature = "serde",
            serde(with = "crate::_serde::OptionReadableDisplay")
        )]
        #[cfg_attr(feature = "schemars", schemars(with = "Option<crate::_schemars::U64>"))]
        version: Option<Version>, // Error case for for owned vs shared?
        digest: Option<ObjectDigest>,
        mutable: Option<bool>,
        // #[cfg_attr(
        //     feature = "serde",
        //     serde(with = "crate::_serde::OptionReadableDisplay")
        // )]
        // #[cfg_attr(feature = "schemars", schemars(with = "Option<crate::_schemars::U64>"))]
        // initial_shared_version: Option<u64>,
    },

    // unresolve Pure
    // value: JsonValue,
}

{
    type_: "pure" | "revieving" | "shared" | "owned" ...
    object_id: Null | ObjectId,
    version:
    digest:
    mutable:
    value:
}
