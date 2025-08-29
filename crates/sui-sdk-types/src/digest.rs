/// A 32-byte Blake2b256 hash output.
///
/// # BCS
///
/// A `Digest`'s BCS serialized form is defined by the following:
///
/// ```text
/// digest = %x20 32OCTET
/// ```
///
/// Due to historical reasons, even though a `Digest` has a fixed-length of 32, Sui's binary
/// representation of a `Digest` is prefixed with its length meaning its serialized binary form (in
/// bcs) is 33 bytes long vs a more compact 32 bytes.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
#[doc(alias = "CheckpointDigest")]
#[doc(alias = "TransactionDigest")]
pub struct Digest(
    #[cfg_attr(feature = "serde", serde(with = "DigestSerialization"))] [u8; Self::LENGTH],
);

impl Digest {
    /// A constant representing the length of a digest in bytes.
    pub const LENGTH: usize = 32;
    /// A constant representing a zero digest.
    pub const ZERO: Self = Self([0; Self::LENGTH]);

    /// Generates a new digest from the provided 32 byte array containing [`u8`] values.
    pub const fn new(digest: [u8; Self::LENGTH]) -> Self {
        Self(digest)
    }

    /// Generates a new digest from the provided random number generator.
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

    /// Returns a slice to the inner array representation of this digest.
    pub const fn inner(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }

    /// Returns the inner array representation of this digest.
    pub const fn into_inner(self) -> [u8; Self::LENGTH] {
        self.0
    }

    /// Returns a slice of bytes representing the digest.
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Decodes a digest from a Base58 encoded string.
    pub fn from_base58<T: AsRef<[u8]>>(base58: T) -> Result<Self, DigestParseError> {
        let buf = bs58::decode(base58.as_ref())
            .into_array_const::<{ Self::LENGTH }>()
            .map_err(|e| DigestParseError {
                bs58_error: Some(e),
            })?;

        Ok(Self(buf))
    }

    /// Decodes a digest from a Base58 encoded string.
    ///
    /// Similar to `from_base58` except any errors are unwrapped, turning them into panics.
    pub const fn from_base58_unwrap(base58: &[u8]) -> Self {
        let buf = bs58::decode(base58).into_array_const_unwrap::<{ Self::LENGTH }>();
        Self(buf)
    }

    /// Returns a Base58 encoded string representation of this digest.
    pub fn to_base58(&self) -> String {
        self.to_string()
    }

    /// Generates a digest from bytes.
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, DigestParseError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| DigestParseError { bs58_error: None })
            .map(Self)
    }
}

impl std::str::FromStr for Digest {
    type Err = DigestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_base58(s)
    }
}

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; Self::LENGTH]> for Digest {
    fn as_ref(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }
}

impl From<Digest> for [u8; Digest::LENGTH] {
    fn from(digest: Digest) -> Self {
        digest.into_inner()
    }
}

impl From<[u8; Self::LENGTH]> for Digest {
    fn from(digest: [u8; Self::LENGTH]) -> Self {
        Self::new(digest)
    }
}

impl std::fmt::Display for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // output size is determined via the following formula:
        //      N * log(256) / log(58) + 1 (round up)
        // where N = 32 this results in a value of 45
        let mut buf = [0; 45];

        let len = bs58::encode(&self.0).onto(&mut buf[..]).unwrap();
        let encoded = std::str::from_utf8(&buf[..len]).unwrap();

        f.write_str(encoded)
    }
}

impl std::fmt::Debug for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Digest")
            .field(&format_args!("\"{self}\""))
            .finish()
    }
}

impl std::fmt::LowerHex for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }

        Ok(())
    }
}

// Unfortunately sui's binary representation of digests is prefixed with its length meaning its
// serialized binary form is 33 bytes long (in bcs) vs a more compact 32 bytes.
#[cfg(feature = "serde")]
type DigestSerialization =
    ::serde_with::As<::serde_with::IfIsHumanReadable<ReadableDigest, ::serde_with::Bytes>>;

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
struct ReadableDigest;

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl serde_with::SerializeAs<[u8; Digest::LENGTH]> for ReadableDigest {
    fn serialize_as<S>(source: &[u8; Digest::LENGTH], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let digest = Digest::new(*source);
        serde_with::DisplayFromStr::serialize_as(&digest, serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<'de> serde_with::DeserializeAs<'de, [u8; Digest::LENGTH]> for ReadableDigest {
    fn deserialize_as<D>(deserializer: D) -> Result<[u8; Digest::LENGTH], D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let digest: Digest = serde_with::DisplayFromStr::deserialize_as(deserializer)?;
        Ok(digest.into_inner())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DigestParseError {
    bs58_error: Option<bs58::decode::Error>,
}

impl std::fmt::Display for DigestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse Digest (must be Base58 string of length {})",
            Digest::LENGTH
        )
    }
}

impl std::error::Error for DigestParseError {}

// Don't implement like the other digest types since this isn't intended to be serialized
pub type SigningDigest = [u8; Digest::LENGTH];

#[cfg(test)]
mod test {
    use super::*;
    use test_strategy::proptest;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[proptest]
    fn roundtrip_display_fromstr(digest: Digest) {
        let s = digest.to_string();
        let d = s.parse::<Digest>().unwrap();
        assert_eq!(digest, d);
    }
}
