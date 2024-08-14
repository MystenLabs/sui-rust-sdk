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
pub struct UnresolvedProgrammableTransaction {
    pub inputs: Vec<UnresolvedInputArgument>,
    pub commands: Vec<Command>,
}

#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct UnresolvedGasPayment {
    pub objects: Vec<UnresolvedObjectReference>,
    pub owner: Address,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::_serde::OptionReadableDisplay")
    )]
    pub price: Option<u64>,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::_serde::OptionReadableDisplay")
    )]
    pub budget: Option<u64>,
}

#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct UnresolvedObjectReference {
    pub object_id: ObjectId,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::_serde::OptionReadableDisplay")
    )]
    pub version: Option<Version>,
    pub digest: Option<ObjectDigest>,
}

#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum UnresolvedInputArgument {
    // contains no structs or objects
    Pure {
        #[cfg_attr(
            feature = "serde",
            serde(with = "::serde_with::As::<::serde_with::Bytes>")
        )]
        value: Vec<u8>,
    },
    // A Move object, either immutable, or owned mutable.
    ImmutableOrOwned(UnresolvedObjectReference),
    // A Move object that's shared.
    // SharedObject::mutable controls whether caller asks for a mutable reference to shared object.
    Shared {
        object_id: ObjectId,
        #[cfg_attr(
            feature = "serde",
            serde(with = "crate::_serde::OptionReadableDisplay")
        )]
        initial_shared_version: Option<u64>,
        mutable: Option<bool>,
    },
    // A Move object that can be received in this transaction.
    Receiving(UnresolvedObjectReference),
}
