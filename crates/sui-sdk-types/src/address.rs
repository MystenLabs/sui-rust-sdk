/// Unique identifier for an Account on the Sui blockchain.
///
/// An `Address` is a 32-byte pseudonymous identifier used to uniquely identify an account and
/// asset-ownership on the Sui blockchain. Often, human-readable addresses are encoded in
/// hexadecimal with a `0x` prefix. For example, this is a valid Sui address:
/// `0x02a212de6a9dfa3a69e22387acfbafbb1a9e591bd9d636e7895dcfc8de05f331`.
///
/// ```
/// use sui_sdk_types::Address;
///
/// let hex = "0x02a212de6a9dfa3a69e22387acfbafbb1a9e591bd9d636e7895dcfc8de05f331";
/// let address = Address::from_hex(hex).unwrap();
/// println!("Address: {}", address);
/// assert_eq!(hex, address.to_string());
/// ```
///
/// # Deriving an account Address
///
/// Account addresses are cryptographically derived from a number of user account authenticators,
/// the simplest of which is an [`Ed25519PublicKey`](crate::Ed25519PublicKey).
///
/// Deriving an address consists of the Blake2b256 hash of the sequence of bytes of its
/// corresponding authenticator, prefixed with a domain-separator. For each authenticator, this
/// domain-separator is the single byte-value of its [`SignatureScheme`](crate::SignatureScheme)
/// flag. E.g. `hash(signature schema flag || authenticator bytes)`.
///
/// Each authenticator includes a convince method for deriving its `Address` as well as
/// documentation for the specifics of how the derivation is done. See
/// [`Ed25519PublicKey::derive_address`] for an example.
///
/// [`Ed25519PublicKey::derive_address`]: crate::Ed25519PublicKey::derive_address
///
/// # Usage as ObjectIds
///
/// `Address`es are also used as a way to uniquely identify an [`Object`]. When an `Address` is
/// used as an object identifierit can also be referred to as an `ObjectId`. `ObjectId`s and
/// account `Address`es share the same 32-byte addressable space but are derived leveraging
/// different domain-separator values to ensure, cryptographically, that there won't be any
/// overlap, e.g. there can't be a valid `Object` whose `ObjectId` is equal to that of the
/// `Address` of a user account.
///
/// [`Object`]: crate::Object
///
/// # BCS
///
/// An `Address`'s BCS serialized form is defined by the following:
///
/// ```text
/// address = 32OCTET
/// ```
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
#[doc(alias = "ObjectId")]
pub struct Address(
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<::serde_with::IfIsHumanReadable<ReadableAddress>>")
    )]
    [u8; Self::LENGTH],
);

impl Address {
    pub const LENGTH: usize = 32;
    pub const ZERO: Self = Self([0u8; Self::LENGTH]);
    pub const TWO: Self = Self::from_u8(2);
    pub const THREE: Self = Self::from_u8(3);

    pub const fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
    }

    const fn from_u8(byte: u8) -> Self {
        let mut address = Self::ZERO;
        address.0[31] = byte;
        address
    }

    #[cfg(feature = "rand")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "rand")))]
    pub fn generate<R>(mut rng: R) -> Self
    where
        R: rand_core::RngCore + rand_core::CryptoRng,
    {
        let mut buf: [u8; Self::LENGTH] = [0; Self::LENGTH];
        rng.fill_bytes(&mut buf);
        Self::new(buf)
    }

    /// Return the underlying byte array of a Address.
    pub const fn into_inner(self) -> [u8; Self::LENGTH] {
        self.0
    }

    pub const fn inner(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }

    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Decodes an address from a hex encoded string.
    pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, AddressParseError> {
        let hex = hex.as_ref();

        hex_address_bytes(hex)
            .map(Self)
            .map_err(|e| AddressParseError { hex_error: Some(e) })
    }

    /// Decodes an address from a hex encoded &'static str.
    ///
    /// Similar to `from_hex` except any errors are unwrapped, turning them into panics.
    pub const fn from_static(hex: &'static str) -> Self {
        match hex_address_bytes(hex.as_bytes()) {
            Ok(address) => Self(address),
            Err(e) => e.const_panic(),
        }
    }

    pub fn to_hex(&self) -> String {
        self.to_string()
    }

    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, AddressParseError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| AddressParseError { hex_error: None })
            .map(Self)
    }
}

impl std::str::FromStr for Address {
    type Err = AddressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for Address {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<Address> for [u8; 32] {
    fn from(address: Address) -> Self {
        address.into_inner()
    }
}

impl From<[u8; 32]> for Address {
    fn from(address: [u8; 32]) -> Self {
        Self::new(address)
    }
}

impl From<Address> for Vec<u8> {
    fn from(value: Address) -> Self {
        value.0.to_vec()
    }
}

impl From<&Address> for Vec<u8> {
    fn from(value: &Address) -> Self {
        value.0.to_vec()
    }
}

impl From<Address> for String {
    fn from(value: Address) -> Self {
        value.to_string()
    }
}

impl From<&Address> for String {
    fn from(value: &Address) -> Self {
        value.to_string()
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Address")
            .field(&format_args!("\"{self}\""))
            .finish()
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
struct ReadableAddress;

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl serde_with::SerializeAs<[u8; Address::LENGTH]> for ReadableAddress {
    fn serialize_as<S>(source: &[u8; Address::LENGTH], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let address = Address::new(*source);
        serde_with::DisplayFromStr::serialize_as(&address, serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<'de> serde_with::DeserializeAs<'de, [u8; Address::LENGTH]> for ReadableAddress {
    fn deserialize_as<D>(deserializer: D) -> Result<[u8; Address::LENGTH], D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let address: Address = serde_with::DisplayFromStr::deserialize_as(deserializer)?;
        Ok(address.into_inner())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddressParseError {
    hex_error: Option<HexDecodeError>,
}

impl std::fmt::Display for AddressParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse Address (must be hex string of length {})",
            Address::LENGTH
        )
    }
}

impl std::error::Error for AddressParseError {}

#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HexDecodeError {
    EmptyInput,
    InputTooLong(usize),
    InvalidHexCharacter(u8),
}

impl HexDecodeError {
    const fn const_panic(&self) -> ! {
        match self {
            HexDecodeError::EmptyInput => panic!("input hex string must be non-empty"),
            HexDecodeError::InputTooLong(_) => panic!("input hex string is too long for address"),
            HexDecodeError::InvalidHexCharacter(_) => {
                panic!("input hex string has wrong character")
            }
        }
    }
}

/// 32-byte address from a hex byte vector, optionally `0x`-prefixed.
const fn hex_address_bytes(bytes: &[u8]) -> Result<[u8; Address::LENGTH], HexDecodeError> {
    if bytes.is_empty() {
        return Err(HexDecodeError::EmptyInput);
    }
    let hex = remove_0x_prefix(bytes);
    if hex.len() > 64 {
        return Err(HexDecodeError::InputTooLong(hex.len()));
    }

    // Decode the hex input from back to front
    let mut buffer = [0; Address::LENGTH];
    let mut i = hex.len();
    let mut j = buffer.len();
    while i >= 2 {
        let lo = HEX_DECODE_LUT[hex[i - 1] as usize];
        let hi = HEX_DECODE_LUT[hex[i - 2] as usize];
        if lo == NIL {
            return Err(HexDecodeError::InvalidHexCharacter(hex[i - 1]));
        }
        if hi == NIL {
            return Err(HexDecodeError::InvalidHexCharacter(hex[i - 2]));
        }
        buffer[j - 1] = (hi << 4) | lo;
        i -= 2;
        j -= 1;
    }
    if i == 1 {
        let lo = HEX_DECODE_LUT[hex[0] as usize];
        if lo == NIL {
            return Err(HexDecodeError::InvalidHexCharacter(hex[0]));
        }
        buffer[j - 1] = lo;
    }
    Ok(buffer)
}

/// Removes initial "0x" prefix if any.
const fn remove_0x_prefix(hex: &[u8]) -> &[u8] {
    if let Some((two, hex2)) = hex.split_first_chunk::<2>()
        && two[0] == b'0'
        && two[1] == b'x'
    {
        return hex2;
    }
    hex
}

/// The lookup table of hex byte to value, used for hex decoding.
///
/// [`NIL`] is used for invalid values.
const HEX_DECODE_LUT: &[u8; 256] = &make_decode_lut();

/// Represents an invalid value in the [`HEX_DECODE_LUT`] table.
const NIL: u8 = u8::MAX;

const fn make_decode_lut() -> [u8; 256] {
    let mut lut = [0; 256];
    let mut i = 0u8;
    loop {
        lut[i as usize] = match i {
            b'0'..=b'9' => i - b'0',
            b'A'..=b'F' => i - b'A' + 10,
            b'a'..=b'f' => i - b'a' + 10,
            // use max value for invalid characters
            _ => NIL,
        };
        if i == NIL {
            break;
        }
        i += 1;
    }
    lut
}

#[cfg(test)]
mod test {
    use super::*;
    use test_strategy::proptest;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn hex_parsing() {
        let actual = Address::from_hex("0x2").unwrap();
        let expected = "0x0000000000000000000000000000000000000000000000000000000000000002";

        assert_eq!(actual.to_string(), expected);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn formats() {
        let actual = Address::from_hex("0x2").unwrap();

        println!("{}", serde_json::to_string(&actual).unwrap());
        println!("{:?}", bcs::to_bytes(&actual).unwrap());
        let a: Address = serde_json::from_str("\"0x2\"").unwrap();
        println!("{a}");
    }

    #[proptest]
    fn roundtrip_display_fromstr(address: Address) {
        let s = address.to_string();
        let a = s.parse::<Address>().unwrap();
        assert_eq!(address, a);
    }
}
