use super::{Address, Identifier, ObjectId, StructTag, TypeTag};

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct TransactionEvents(Vec<Event>);

/// Specific type of event
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Event {
    pub package_id: ObjectId,
    pub transaction_module: Identifier,
    pub sender: Address,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: StructTag,
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<::serde_with::Bytes>")
    )]
    pub contents: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct BalanceChange {
    /// Owner of the balance change
    pub address: Address,
    /// Type of the Coin
    pub coin_type: TypeTag,
    /// The amount indicate the balance value changes.
    ///
    /// A negative amount means spending coin value and positive means receiving coin value.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub amount: i128,
}
