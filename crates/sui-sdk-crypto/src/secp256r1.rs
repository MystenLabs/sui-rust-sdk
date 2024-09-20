#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Secp256r1PrivateKey([u8; Self::LENGTH]);

impl Secp256r1PrivateKey {
    /// The length of an secp256r1 private key in bytes.
    pub const LENGTH: usize = 32;
}
