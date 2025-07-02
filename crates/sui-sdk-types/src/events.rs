use super::Address;
use super::Identifier;
use super::ObjectId;
use super::StructTag;
use super::TypeTag;

/// Events emitted during the successful execution of a transaction
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// transaction-events = vector event
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct TransactionEvents(
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=1).lift()))]
    pub  Vec<Event>,
);

/// An event
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// event = object-id identifier address struct-tag bytes
/// ```
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct Event {
    /// Package id of the top-level function invoked by a MoveCall command which triggered this
    /// event to be emitted.
    pub package_id: ObjectId,

    /// Module name of the top-level function invoked by a MoveCall command which triggered this
    /// event to be emitted.
    pub module: Identifier,

    /// Address of the account that sent the transaction where this event was emitted.
    pub sender: Address,

    /// The type of the event emitted
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: StructTag,

    /// BCS serialized bytes of the event
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::_serde::ReadableBase64Encoded")
    )]
    pub contents: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct BalanceChange {
    /// Owner of the balance change
    pub address: Address,

    /// Type of the Coin
    pub coin_type: TypeTag,

    /// The amount indicate the balance value changes.
    ///
    /// A negative amount means spending coin value and positive means receiving coin value.
    pub amount: i128,
}
