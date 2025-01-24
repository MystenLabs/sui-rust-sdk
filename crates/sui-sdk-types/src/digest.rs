/// A representation of a 32 byte digest.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
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
        let mut buf = [0; Self::LENGTH];

        bs58::decode(base58)
            .onto(&mut buf)
            //TODO fix error to contain bs58 parse error
            .map_err(|_| DigestParseError)?;

        Ok(Self(buf))
    }

    /// Returns a Base58 encoded string representation of this digest.
    pub fn to_base58(&self) -> String {
        self.to_string()
    }

    /// Generates a digest from bytes.
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, DigestParseError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| DigestParseError)
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
            .field(&format_args!("\"{}\"", self))
            .finish()
    }
}

impl std::fmt::LowerHex for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
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
pub struct DigestParseError;

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

//
// Implement Various Digest wrappers
//

macro_rules! impl_digest {
    ($t:ident) => {
        #[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Serialize, serde_derive::Deserialize)
        )]
        #[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
        pub struct $t(Digest);

        impl $t {
            pub const LENGTH: usize = Digest::LENGTH;
            pub const ZERO: Self = Self::new([0; Self::LENGTH]);

            pub const fn new(digest: [u8; Self::LENGTH]) -> Self {
                Self(Digest::new(digest))
            }

            #[cfg(feature = "rand")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "rand")))]
            pub fn generate<R>(rng: R) -> Self
            where
                R: rand_core::RngCore + rand_core::CryptoRng,
            {
                Self(Digest::generate(rng))
            }

            pub const fn inner(&self) -> &[u8; Self::LENGTH] {
                self.0.inner()
            }

            pub const fn into_inner(self) -> [u8; Self::LENGTH] {
                self.0.into_inner()
            }

            pub const fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }

            pub fn from_base58<T: AsRef<[u8]>>(base58: T) -> Result<Self, DigestParseError> {
                Digest::from_base58(base58).map(Self)
            }

            #[allow(clippy::wrong_self_convention)]
            pub fn to_base58(&self) -> String {
                self.to_string()
            }

            pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, DigestParseError> {
                Digest::from_bytes(bytes).map(Self)
            }
        }

        impl std::str::FromStr for $t {
            type Err = DigestParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::from_base58(s)
            }
        }

        impl AsRef<[u8]> for $t {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }

        impl AsRef<[u8; Self::LENGTH]> for $t {
            fn as_ref(&self) -> &[u8; Self::LENGTH] {
                self.0.as_ref()
            }
        }

        impl From<$t> for [u8; $t::LENGTH] {
            fn from(digest: $t) -> Self {
                digest.into_inner()
            }
        }

        impl From<[u8; Self::LENGTH]> for $t {
            fn from(digest: [u8; Self::LENGTH]) -> Self {
                Self::new(digest)
            }
        }

        impl From<Digest> for $t {
            fn from(digest: Digest) -> Self {
                Self(digest)
            }
        }

        impl From<$t> for Digest {
            fn from(digest: $t) -> Self {
                digest.0
            }
        }

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($t))
                    .field(&format_args!("\"{}\"", self))
                    .finish()
            }
        }

        impl std::fmt::LowerHex for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::LowerHex::fmt(&self.0, f)
            }
        }
    };
}

impl_digest!(CheckpointDigest);
impl_digest!(CheckpointContentsDigest);
impl_digest!(TransactionDigest);
impl_digest!(TransactionEffectsDigest);
impl_digest!(TransactionEventsDigest);
impl_digest!(ObjectDigest);
impl_digest!(ConsensusCommitDigest);
impl_digest!(EffectsAuxiliaryDataDigest);

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
