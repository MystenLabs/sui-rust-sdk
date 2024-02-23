use super::Ed25519PublicKey;
use super::Ed25519Signature;
use super::Secp256k1PublicKey;
use super::Secp256k1Signature;
use super::Secp256r1PublicKey;
use super::Secp256r1Signature;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum SimpleSignature {
    Ed25519 {
        public_key: Ed25519PublicKey,
        signature: Ed25519Signature,
    },
    Secp256k1 {
        public_key: Secp256k1PublicKey,
        signature: Secp256k1Signature,
    },
    Secp256r1 {
        public_key: Secp256r1PublicKey,
        signature: Secp256r1Signature,
    },
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SignatureScheme {
    ED25519 = 0x00,
    Secp256k1 = 0x01,
    Secp256r1 = 0x02,
    MultiSig = 0x03,
    BLS12381 = 0x04, // This is currently not supported for user addresses
    ZkLoginAuthenticator = 0x05,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum UserSignature {
    Simple(SimpleSignature),
    // MultiSigLegacy,
    // MultiSig,
    // ZkLoginAuthenticator,
}
