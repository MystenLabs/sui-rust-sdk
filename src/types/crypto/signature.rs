use super::Ed25519PublicKey;
use super::Ed25519Signature;
use super::MultisigAggregatedSignature;
use super::Secp256k1PublicKey;
use super::Secp256k1Signature;
use super::Secp256r1PublicKey;
use super::Secp256r1Signature;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum SimpleSignature {
    Ed25519 {
        signature: Ed25519Signature,
        public_key: Ed25519PublicKey,
    },
    Secp256k1 {
        signature: Secp256k1Signature,
        public_key: Secp256k1PublicKey,
    },
    Secp256r1 {
        signature: Secp256r1Signature,
        public_key: Secp256r1PublicKey,
    },
}

impl SimpleSignature {
    #[cfg(feature = "serde")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    fn from_serialized_bytes<T: AsRef<[u8]>, E: serde::de::Error>(bytes: T) -> Result<Self, E> {
        let bytes = bytes.as_ref();
        let flag = SignatureScheme::from_byte(bytes[0]).map_err(serde::de::Error::custom)?;
        match flag {
            SignatureScheme::Ed25519 => {
                let expected_length = 1 + Ed25519Signature::LENGTH + Ed25519PublicKey::LENGTH;

                if bytes.len() != expected_length {
                    return Err(serde::de::Error::custom("invalid ed25519 signature"));
                }

                let mut signature = [0; Ed25519Signature::LENGTH];
                signature.copy_from_slice(&bytes[1..(1 + Ed25519Signature::LENGTH)]);

                let mut public_key = [0; Ed25519PublicKey::LENGTH];
                public_key.copy_from_slice(&bytes[(1 + Ed25519Signature::LENGTH)..]);

                Ok(SimpleSignature::Ed25519 {
                    signature: Ed25519Signature::new(signature),
                    public_key: Ed25519PublicKey::new(public_key),
                })
            }
            SignatureScheme::Secp256k1 => {
                let expected_length = 1 + Secp256k1Signature::LENGTH + Secp256k1PublicKey::LENGTH;

                if bytes.len() != expected_length {
                    return Err(serde::de::Error::custom("invalid secp25k1 signature"));
                }

                let mut signature = [0; Secp256k1Signature::LENGTH];
                signature.copy_from_slice(&bytes[1..(1 + Secp256k1Signature::LENGTH)]);

                let mut public_key = [0; Secp256k1PublicKey::LENGTH];
                public_key.copy_from_slice(&bytes[(1 + Secp256k1Signature::LENGTH)..]);

                Ok(SimpleSignature::Secp256k1 {
                    signature: Secp256k1Signature::new(signature),
                    public_key: Secp256k1PublicKey::new(public_key),
                })
            }
            SignatureScheme::Secp256r1 => {
                let expected_length = 1 + Secp256r1Signature::LENGTH + Secp256r1PublicKey::LENGTH;

                if bytes.len() != expected_length {
                    return Err(serde::de::Error::custom("invalid secp25r1 signature"));
                }

                let mut signature = [0; Secp256r1Signature::LENGTH];
                signature.copy_from_slice(&bytes[1..(1 + Secp256r1Signature::LENGTH)]);

                let mut public_key = [0; Secp256r1PublicKey::LENGTH];
                public_key.copy_from_slice(&bytes[(1 + Secp256r1Signature::LENGTH)..]);

                Ok(SimpleSignature::Secp256r1 {
                    signature: Secp256r1Signature::new(signature),
                    public_key: Secp256r1PublicKey::new(public_key),
                })
            }
            SignatureScheme::Multisig
            | SignatureScheme::BLS12381
            | SignatureScheme::ZkLoginAuthenticator => {
                Err(serde::de::Error::custom("invalid signature scheme"))
            }
        }
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl serde::Serialize for SimpleSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde_derive::Serialize)]
        #[serde(tag = "scheme")]
        #[serde(rename_all = "lowercase")]
        pub enum Sig<'a> {
            Ed25519 {
                signature: &'a Ed25519Signature,
                public_key: &'a Ed25519PublicKey,
            },
            Secp256k1 {
                signature: &'a Secp256k1Signature,
                public_key: &'a Secp256k1PublicKey,
            },
            Secp256r1 {
                signature: &'a Secp256r1Signature,
                public_key: &'a Secp256r1PublicKey,
            },
        }

        if serializer.is_human_readable() {
            let sig = match self {
                SimpleSignature::Ed25519 {
                    signature,
                    public_key,
                } => Sig::Ed25519 {
                    signature,
                    public_key,
                },
                SimpleSignature::Secp256k1 {
                    signature,
                    public_key,
                } => Sig::Secp256k1 {
                    signature,
                    public_key,
                },
                SimpleSignature::Secp256r1 {
                    signature,
                    public_key,
                } => Sig::Secp256r1 {
                    signature,
                    public_key,
                },
            };

            sig.serialize(serializer)
        } else {
            match self {
                SimpleSignature::Ed25519 {
                    signature,
                    public_key,
                } => {
                    let mut buf = [0; 1 + Ed25519Signature::LENGTH + Ed25519PublicKey::LENGTH];
                    buf[0] = SignatureScheme::Ed25519 as u8;
                    buf[1..(1 + Ed25519Signature::LENGTH)].copy_from_slice(signature.as_ref());
                    buf[(1 + Ed25519Signature::LENGTH)..].copy_from_slice(public_key.as_ref());

                    serializer.serialize_bytes(&buf)
                }
                SimpleSignature::Secp256k1 {
                    signature,
                    public_key,
                } => {
                    let mut buf = [0; 1 + Secp256k1Signature::LENGTH + Secp256k1PublicKey::LENGTH];
                    buf[0] = SignatureScheme::Secp256k1 as u8;
                    buf[1..(1 + Secp256k1Signature::LENGTH)].copy_from_slice(signature.as_ref());
                    buf[(1 + Secp256k1Signature::LENGTH)..].copy_from_slice(public_key.as_ref());

                    serializer.serialize_bytes(&buf)
                }
                SimpleSignature::Secp256r1 {
                    signature,
                    public_key,
                } => {
                    let mut buf = [0; 1 + Secp256r1Signature::LENGTH + Secp256r1PublicKey::LENGTH];
                    buf[0] = SignatureScheme::Secp256r1 as u8;
                    buf[1..(1 + Secp256r1Signature::LENGTH)].copy_from_slice(signature.as_ref());
                    buf[(1 + Secp256r1Signature::LENGTH)..].copy_from_slice(public_key.as_ref());

                    serializer.serialize_bytes(&buf)
                }
            }
        }
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<'de> serde::Deserialize<'de> for SimpleSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde_derive::Deserialize)]
        #[serde(tag = "scheme")]
        #[serde(rename_all = "lowercase")]
        pub enum Sig {
            Ed25519 {
                signature: Ed25519Signature,
                public_key: Ed25519PublicKey,
            },
            Secp256k1 {
                signature: Secp256k1Signature,
                public_key: Secp256k1PublicKey,
            },
            Secp256r1 {
                signature: Secp256r1Signature,
                public_key: Secp256r1PublicKey,
            },
        }

        if deserializer.is_human_readable() {
            let sig = Sig::deserialize(deserializer)?;
            Ok(match sig {
                Sig::Ed25519 {
                    signature,
                    public_key,
                } => SimpleSignature::Ed25519 {
                    signature,
                    public_key,
                },
                Sig::Secp256k1 {
                    signature,
                    public_key,
                } => SimpleSignature::Secp256k1 {
                    signature,
                    public_key,
                },
                Sig::Secp256r1 {
                    signature,
                    public_key,
                } => SimpleSignature::Secp256r1 {
                    signature,
                    public_key,
                },
            })
        } else {
            let bytes: std::borrow::Cow<'de, [u8]> = std::borrow::Cow::deserialize(deserializer)?;
            Self::from_serialized_bytes(bytes)
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SignatureScheme {
    Ed25519 = 0x00,
    Secp256k1 = 0x01,
    Secp256r1 = 0x02,
    Multisig = 0x03,
    BLS12381 = 0x04, // This is currently not supported for user addresses
    ZkLoginAuthenticator = 0x05,
}

impl SignatureScheme {
    pub fn name(self) -> &'static str {
        match self {
            SignatureScheme::Ed25519 => "ed25519",
            SignatureScheme::Secp256k1 => "secp256k1",
            SignatureScheme::Secp256r1 => "secp256r1",
            SignatureScheme::Multisig => "multisig",
            SignatureScheme::BLS12381 => "bls12381",
            SignatureScheme::ZkLoginAuthenticator => "zklogin",
        }
    }

    pub fn from_byte(flag: u8) -> Result<Self, InvalidSignatureScheme> {
        match flag {
            0x00 => Ok(Self::Ed25519),
            0x01 => Ok(Self::Secp256k1),
            0x02 => Ok(Self::Secp256r1),
            0x03 => Ok(Self::Multisig),
            0x04 => Ok(Self::BLS12381),
            0x05 => Ok(Self::ZkLoginAuthenticator),
            invalid => Err(InvalidSignatureScheme(invalid)),
        }
    }
}

pub struct InvalidSignatureScheme(u8);

impl std::fmt::Display for InvalidSignatureScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid signature scheme: {:02x}", self.0)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum UserSignature {
    Simple(SimpleSignature),
    Multisig(MultisigAggregatedSignature),
    // ZkLoginAuthenticator,
}
