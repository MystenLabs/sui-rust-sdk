use super::Address;

/// An `ObjectId` is a 32-byte identifier used to uniquely identify an object on the Sui
/// blockchain.
///
/// ## Relationship to Address
///
/// [`Address`]es and `ObjectId`s share the same 32-byte addressable space but are derived
/// leveraging different domain-separator values to ensure, cryptographically, that there won't be
/// any overlap, e.g. there can't be a valid `Object` whose `ObjectId` is equal to that of the
/// `Address` of a user account.
///
/// # BCS
///
/// An `ObjectId`'s BCS serialized form is defined by the following:
///
/// ```text
/// object-id = 32*OCTET
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ObjectId(Address);

impl ObjectId {
    pub const LENGTH: usize = Address::LENGTH;
    pub const ZERO: Self = Self(Address::ZERO);

    /// Generates a new ObjectId from the provided byte array.
    pub const fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(Address::new(bytes))
    }

    /// Generates a new ObjectId from single byte.
    pub const fn from_u8(byte: u8) -> Self {
        Self(Address::from_u8(byte))
    }

    /// Returns the underlying byte array of an ObjectId.
    pub const fn into_inner(self) -> [u8; Self::LENGTH] {
        self.0.into_inner()
    }

    /// Returns a reference to the underlying byte array of an ObjectId.
    pub const fn inner(&self) -> &[u8; Self::LENGTH] {
        self.0.inner()
    }

    /// Returns a slice of bytes of an ObjectId.
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns the underlying Address of an ObjectId.
    pub const fn as_address(&self) -> &Address {
        &self.0
    }
}

impl AsRef<[u8]> for ObjectId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<[u8; 32]> for ObjectId {
    fn as_ref(&self) -> &[u8; 32] {
        self.0.as_ref()
    }
}

impl From<ObjectId> for [u8; 32] {
    fn from(object_id: ObjectId) -> Self {
        object_id.into_inner()
    }
}

impl From<[u8; 32]> for ObjectId {
    fn from(object_id: [u8; 32]) -> Self {
        Self::new(object_id)
    }
}

impl From<Address> for ObjectId {
    fn from(value: Address) -> Self {
        Self(value)
    }
}

impl From<ObjectId> for Vec<u8> {
    fn from(value: ObjectId) -> Self {
        value.0.into()
    }
}

impl std::str::FromStr for ObjectId {
    type Err = super::address::AddressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Address::from_str(s).map(Self)
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
