//! Implementation of secp256r1 public-key cryptogrophy.

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Secp256r1PrivateKey(
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array32>>")
    )]
    [u8; Self::LENGTH],
);

impl Secp256r1PrivateKey {
    /// The length of an secp256r1 private key in bytes.
    pub const LENGTH: usize = 32;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct Secp256r1PublicKey(
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array33, [::serde_with::Same; 33]>>"
        )
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::Base64"))]
    [u8; Self::LENGTH],
);

impl Secp256r1PublicKey {
    /// The length of an secp256r1 public key in bytes.
    pub const LENGTH: usize = 33;

    pub const fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
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

    /// Return the underlying byte array of an Secp256r1PublicKey.
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

impl std::str::FromStr for Secp256r1PublicKey {
    type Err = base64ct::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        super::Base64FromStr33::from_str(s).map(|a| Self::new(a.0))
    }
}

impl AsRef<[u8]> for Secp256r1PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; Self::LENGTH]> for Secp256r1PublicKey {
    fn as_ref(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }
}

impl From<Secp256r1PublicKey> for [u8; Secp256r1PublicKey::LENGTH] {
    fn from(public_key: Secp256r1PublicKey) -> Self {
        public_key.into_inner()
    }
}

impl From<[u8; Self::LENGTH]> for Secp256r1PublicKey {
    fn from(public_key: [u8; Self::LENGTH]) -> Self {
        Self::new(public_key)
    }
}

impl std::fmt::Display for Secp256r1PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&super::Base64Display33(&self.0), f)
    }
}

impl std::fmt::Debug for Secp256r1PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Secp256r1PublicKey")
            .field(&format_args!("\"{}\"", self))
            .finish()
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(test_strategy::Arbitrary))]
pub struct Secp256r1Signature(
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array64, [::serde_with::Same; 64]>>"
        )
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "crate::_schemars::Base64"))]
    [u8; Self::LENGTH],
);

impl Secp256r1Signature {
    /// The length of an secp256r1 signature key in bytes.
    pub const LENGTH: usize = 64;

    pub const fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
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

    /// Return the underlying byte array of an Secp256r1Signature.
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

impl std::str::FromStr for Secp256r1Signature {
    type Err = base64ct::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        super::Base64FromStr64::from_str(s).map(|a| Self::new(a.0))
    }
}

impl AsRef<[u8]> for Secp256r1Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; Self::LENGTH]> for Secp256r1Signature {
    fn as_ref(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }
}

impl From<Secp256r1Signature> for [u8; Secp256r1Signature::LENGTH] {
    fn from(signature: Secp256r1Signature) -> Self {
        signature.into_inner()
    }
}

impl From<[u8; Self::LENGTH]> for Secp256r1Signature {
    fn from(signature: [u8; Self::LENGTH]) -> Self {
        Self::new(signature)
    }
}

impl std::fmt::Display for Secp256r1Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&super::Base64Display64(&self.0), f)
    }
}

impl std::fmt::Debug for Secp256r1Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Secp256r1Signature")
            .field(&format_args!("\"{}\"", self))
            .finish()
    }
}
