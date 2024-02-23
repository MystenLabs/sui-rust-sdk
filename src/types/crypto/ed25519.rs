//! Implementation of ed25519 public-key cryptogrophy.

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Ed25519PrivateKey(
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array32>>")
    )]
    [u8; Self::LENGTH],
);

impl Ed25519PrivateKey {
    /// The length of an ed25519 private key in bytes.
    pub const LENGTH: usize = 32;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Ed25519PublicKey(
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array32>>")
    )]
    [u8; Self::LENGTH],
);

impl Ed25519PublicKey {
    /// The length of an ed25519 public key in bytes.
    pub const LENGTH: usize = 32;
}

impl std::fmt::Display for Ed25519PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&super::Base64Display32(&self.0), f)
    }
}

impl std::fmt::Debug for Ed25519PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Ed25519PublicKey")
            .field(&format_args!("\"{}\"", self))
            .finish()
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Ed25519Signature(
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array64, [::serde_with::Same; 64]>>"
        )
    )]
    [u8; Self::LENGTH],
);

impl Ed25519Signature {
    /// The length of an ed25519 signature key in bytes.
    pub const LENGTH: usize = 64;
}

impl std::fmt::Display for Ed25519Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&super::Base64Display64(&self.0), f)
    }
}

impl std::fmt::Debug for Ed25519Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Ed25519Signature")
            .field(&format_args!("\"{}\"", self))
            .finish()
    }
}
