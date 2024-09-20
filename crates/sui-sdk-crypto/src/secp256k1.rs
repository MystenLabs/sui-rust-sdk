#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Secp256k1PrivateKey([u8; Self::LENGTH]);

impl Secp256k1PrivateKey {
    /// The length of an secp256k1 private key in bytes.
    pub const LENGTH: usize = 32;
}
