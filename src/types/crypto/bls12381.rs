//! Implementation of bls12381 min-sig public-key cryptogrophy.

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Bls12381PrivateKey(
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array32>>")
    )]
    [u8; Self::LENGTH],
);

impl Bls12381PrivateKey {
    /// The length of an bls12381 private key in bytes.
    pub const LENGTH: usize = 32;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Bls12381PublicKey(
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array96, [::serde_with::Same; 96]>>"
        )
    )]
    [u8; Self::LENGTH],
);

impl Bls12381PublicKey {
    /// The length of an bls12381 public key in bytes.
    pub const LENGTH: usize = 96;
}

impl std::fmt::Display for Bls12381PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&super::Base64Display96(&self.0), f)
    }
}

impl std::fmt::Debug for Bls12381PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bls12381PublicKey")
            .field(&format_args!("\"{}\"", self))
            .finish()
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Bls12381Signature(
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array48, [::serde_with::Same; 48]>>"
        )
    )]
    [u8; Self::LENGTH],
);

impl Bls12381Signature {
    /// The length of an bls12381 signature key in bytes.
    pub const LENGTH: usize = 48;
}

impl std::fmt::Display for Bls12381Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&super::Base64Display48(&self.0), f)
    }
}

impl std::fmt::Debug for Bls12381Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bls12381Signature")
            .field(&format_args!("\"{}\"", self))
            .finish()
    }
}
