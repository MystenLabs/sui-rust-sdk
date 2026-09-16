//! Implementation of ML-DSA-65 (FIPS 204) public-key cryptography.

/// An ML-DSA-65 public key.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// mldsa65-public-key = 1952OCTECT
/// ```
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct MlDsa65PublicKey(
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array1952, [::serde_with::Same; 1952]>>"
        )
    )]
    [u8; Self::LENGTH],
);

impl MlDsa65PublicKey {
    /// The length of an ML-DSA-65 public key in bytes.
    pub const LENGTH: usize = 1952;

    pub const fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
    }

    /// Return the underlying byte array of an MlDsa65PublicKey.
    pub const fn into_inner(self) -> [u8; Self::LENGTH] {
        self.0
    }

    pub const fn inner(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }

    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, std::array::TryFromSliceError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref()).map(Self)
    }
}

impl std::str::FromStr for MlDsa65PublicKey {
    type Err = base64ct::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        super::Base64FromStr1952::from_str(s).map(|a| Self::new(a.0))
    }
}

impl AsRef<[u8]> for MlDsa65PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; Self::LENGTH]> for MlDsa65PublicKey {
    fn as_ref(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }
}

impl From<MlDsa65PublicKey> for [u8; MlDsa65PublicKey::LENGTH] {
    fn from(public_key: MlDsa65PublicKey) -> Self {
        public_key.into_inner()
    }
}

impl From<[u8; Self::LENGTH]> for MlDsa65PublicKey {
    fn from(public_key: [u8; Self::LENGTH]) -> Self {
        Self::new(public_key)
    }
}

impl std::fmt::Display for MlDsa65PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&super::Base64Display1952(&self.0), f)
    }
}

impl std::fmt::Debug for MlDsa65PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MlDsa65PublicKey")
            .field(&format_args!("\"{self}\""))
            .finish()
    }
}

/// An ML-DSA-65 signature.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// mldsa65-signature = 3309OCTECT
/// ```
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct MlDsa65Signature(
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array3309, [::serde_with::Same; 3309]>>"
        )
    )]
    [u8; Self::LENGTH],
);

impl MlDsa65Signature {
    /// The length of an ML-DSA-65 signature in bytes.
    pub const LENGTH: usize = 3309;

    pub const fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
    }

    /// Return the underlying byte array of an MlDsa65Signature.
    pub const fn into_inner(self) -> [u8; Self::LENGTH] {
        self.0
    }

    pub const fn inner(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }

    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, std::array::TryFromSliceError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref()).map(Self)
    }
}

impl std::str::FromStr for MlDsa65Signature {
    type Err = base64ct::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        super::Base64FromStr3309::from_str(s).map(|a| Self::new(a.0))
    }
}

impl AsRef<[u8]> for MlDsa65Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; Self::LENGTH]> for MlDsa65Signature {
    fn as_ref(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }
}

impl From<MlDsa65Signature> for [u8; MlDsa65Signature::LENGTH] {
    fn from(signature: MlDsa65Signature) -> Self {
        signature.into_inner()
    }
}

impl From<[u8; Self::LENGTH]> for MlDsa65Signature {
    fn from(signature: [u8; Self::LENGTH]) -> Self {
        Self::new(signature)
    }
}

impl std::fmt::Display for MlDsa65Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&super::Base64Display3309(&self.0), f)
    }
}

impl std::fmt::Debug for MlDsa65Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MlDsa65Signature")
            .field(&format_args!("\"{self}\""))
            .finish()
    }
}
