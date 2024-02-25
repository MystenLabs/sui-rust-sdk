use super::Ed25519PublicKey;
use super::Ed25519Signature;
use super::MultisigAggregatedSignature;
use super::Secp256k1PublicKey;
use super::Secp256k1Signature;
use super::Secp256r1PublicKey;
use super::Secp256r1Signature;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
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
    pub fn scheme(&self) -> SignatureScheme {
        match self {
            SimpleSignature::Ed25519 { .. } => SignatureScheme::Ed25519,
            SimpleSignature::Secp256k1 { .. } => SignatureScheme::Secp256k1,
            SimpleSignature::Secp256r1 { .. } => SignatureScheme::Secp256r1,
        }
    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum UserSignature {
    Simple(SimpleSignature),
    Multisig(MultisigAggregatedSignature),
    // ZkLoginAuthenticator,
}

impl UserSignature {
    pub fn scheme(&self) -> SignatureScheme {
        match self {
            UserSignature::Simple(simple) => simple.scheme(),
            UserSignature::Multisig(_) => SignatureScheme::Multisig,
        }
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use super::*;

    #[derive(serde_derive::Serialize)]
    #[serde(tag = "scheme", rename_all = "lowercase")]
    enum ReadableUserSignatureRef<'a> {
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
        Multisig(&'a MultisigAggregatedSignature),
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(tag = "scheme", rename_all = "lowercase")]
    enum ReadableUserSignature {
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
        Multisig(MultisigAggregatedSignature),
    }

    impl serde::Serialize for UserSignature {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self {
                    UserSignature::Simple(SimpleSignature::Ed25519 {
                        signature,
                        public_key,
                    }) => ReadableUserSignatureRef::Ed25519 {
                        signature,
                        public_key,
                    },
                    UserSignature::Simple(SimpleSignature::Secp256k1 {
                        signature,
                        public_key,
                    }) => ReadableUserSignatureRef::Secp256k1 {
                        signature,
                        public_key,
                    },
                    UserSignature::Simple(SimpleSignature::Secp256r1 {
                        signature,
                        public_key,
                    }) => ReadableUserSignatureRef::Secp256r1 {
                        signature,
                        public_key,
                    },
                    UserSignature::Multisig(multisig) => {
                        ReadableUserSignatureRef::Multisig(multisig)
                    }
                };
                readable.serialize(serializer)
            } else {
                match self {
                    UserSignature::Simple(simple) => simple.serialize(serializer),
                    UserSignature::Multisig(multisig) => multisig.serialize(serializer),
                }
            }
        }
    }

    impl<'de> serde::Deserialize<'de> for UserSignature {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let readable = ReadableUserSignature::deserialize(deserializer)?;
                Ok(match readable {
                    ReadableUserSignature::Ed25519 {
                        signature,
                        public_key,
                    } => Self::Simple(SimpleSignature::Ed25519 {
                        signature,
                        public_key,
                    }),
                    ReadableUserSignature::Secp256k1 {
                        signature,
                        public_key,
                    } => Self::Simple(SimpleSignature::Secp256k1 {
                        signature,
                        public_key,
                    }),
                    ReadableUserSignature::Secp256r1 {
                        signature,
                        public_key,
                    } => Self::Simple(SimpleSignature::Secp256r1 {
                        signature,
                        public_key,
                    }),
                    ReadableUserSignature::Multisig(multisig) => Self::Multisig(multisig),
                })
            } else {
                use serde_with::DeserializeAs;

                let bytes: std::borrow::Cow<'de, [u8]> =
                    serde_with::Bytes::deserialize_as(deserializer)?;
                let flag =
                    SignatureScheme::from_byte(bytes[0]).map_err(serde::de::Error::custom)?;
                match flag {
                    SignatureScheme::Ed25519
                    | SignatureScheme::Secp256k1
                    | SignatureScheme::Secp256r1 => {
                        let simple = SimpleSignature::from_serialized_bytes(bytes)?;
                        Ok(Self::Simple(simple))
                    }
                    SignatureScheme::Multisig => {
                        let multisig = MultisigAggregatedSignature::from_serialized_bytes(bytes)?;
                        Ok(Self::Multisig(multisig))
                    }
                    SignatureScheme::BLS12381 | SignatureScheme::ZkLoginAuthenticator => {
                        Err(serde::de::Error::custom("invalid signature scheme"))
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen_test::wasm_bindgen_test as test;

        proptest::proptest! {
            #[test]
            #[cfg(feature = "serde")]
            fn roundtrip_bcs(signature: UserSignature) {
                let b = bcs::to_bytes(&signature).unwrap();
                let s = bcs::from_bytes(&b).unwrap();
                assert_eq!(signature, s);
            }

            #[test]
            #[cfg(feature = "serde")]
            fn roundtrip_json(signature: UserSignature) {
                let s = serde_json::to_string(&signature).unwrap();
                let sig = serde_json::from_str(&s).unwrap();
                assert_eq!(signature, sig);
            }
        }

        #[test]
        fn simple_fixtures() {
            const FIXTURES: &[(SignatureScheme, &str)]  = &[
                (SignatureScheme::Ed25519, "YQDaeO4w2ULMy5eqHBzP0oalr1YhDX/9uJS9MntKnW3d55q4aqZYYnoEloaBmXKc6FoD5bTwONdwS9CwdMQGhIcPDX2rNYyNrapO+gBJp1sHQ2VVsQo2ghm7aA9wVxNJ13U="),
                (SignatureScheme::Secp256k1, "YgErcT6WUSQXGD1DaIwls5rWq648akDMlvL41ugUUhyIPWnqURl+daQLG+ILNemARKHYVNOikKJJ8jqu+HzlRa5rAg4XzVk55GsZZkGWjNdZkQuiV34n+nP944dtub7FvOsr"),
                (SignatureScheme::Secp256r1, "YgLp1p4K9dSQTt2AeR05yK1MkXmtLm6Sieb9yfkpW1gOBiqnO9ZKZiWUrLJQav2Mxw64zM37g3IVdsB/To6qfl8IA0f7ryPwOKvEwwiicRF6Kkz/rt28X/gcdRe8bHSn7bQw"),
            ];

            for (scheme, fixture) in FIXTURES {
                let bcs = <base64ct::Base64 as base64ct::Encoding>::decode_vec(fixture).unwrap();

                let sig: UserSignature = bcs::from_bytes(&bcs).unwrap();
                assert_eq!(*scheme, sig.scheme());
                let bytes = bcs::to_bytes(&sig).unwrap();
                assert_eq!(bcs, bytes);

                let json = serde_json::to_string_pretty(&sig).unwrap();
                println!("{json}");
                assert_eq!(sig, serde_json::from_str(&json).unwrap());
            }
        }

        #[test]
        fn legacy_multisig_fixtures() {
            const FIXTURE1: &str = "rgIDAgAnwUSyrALP8m0eEPZE6aPggBELk72n1u3LU+i4nx5kqzhahcICbskEYzHJrbarvFr/RQITgDMoorqpDhN8dgsKATyrN3CD8g37D60dYiGW6sOBqIcf3E1mdMsKvX2pbOZsYQv8VNL+2Jz3vnMXcwEZF32PplKjcnmyUGRhV11M7n4UOjAAAAEAAAAAAAEAEAAAAAAAAQADLEFBMTlxeldNamEycVR2b0FTYWRiQjBObFZiRUtOb0ladTJnUGNGY1RTZGQxATBBUUlPRjgxWk9lUnJHV1pCbG96WFdaRUxvbGQrSi9wei9lT0hiYm0reGJ6ckt3PT0BMEFnTkgrNjhqOERpcnhNTUlvbkVSZWlwTS82N2R2Ri80SEhVWHZHeDBwKzIwTUE9PQECAA==";

            const FIXTURE2: &str = "8QIDAwDYAAra4KQGp2Oq1TCOgWfH8IxC4UA5wJB/NqOcNmMh54Y5d5pnVQfTlqgq4J17a8+W+y3+jk9h4YMB9LzPDYcLAaJJBH+WLPfPaQ7T3Cv8nqpZ1TbPrT8E61FrSgeIbN4OTJeijjguv1pd3ImvTeo4AMYZczf5OH6+5yBaur7R6YACiooT5J36agjUk0TpVcTKMGwykIwD7NBkZ0gbinHxuVJwdi1tSbqhMpqvNgP+CFO6F7FSTe+xiHh0MDOKyYQItxY6MAAAAQAAAAAAAgAQAAAAAAABAAIAAyxBQTE5cXpXTWphMnFUdm9BU2FkYkIwTmxWYkVLTm9JWnUyZ1BjRmNUU2RkMQEwQVFJT0Y4MVpPZVJyR1daQmxvelhXWkVMb2xkK0ovcHovZU9IYmJtK3hienJLdz09ATBBZ05IKzY4ajhEaXJ4TU1Jb25FUmVpcE0vNjdkdkYvNEhIVVh2R3gwcCsyME1BPT0BAgA=";

            for fixture in [FIXTURE1, FIXTURE2] {
                let bcs = <base64ct::Base64 as base64ct::Encoding>::decode_vec(fixture).unwrap();

                let sig: UserSignature = bcs::from_bytes(&bcs).unwrap();
                assert_eq!(SignatureScheme::Multisig, sig.scheme());
                let bytes = bcs::to_bytes(&sig).unwrap();
                assert_eq!(bcs, bytes);

                let json = serde_json::to_string_pretty(&sig).unwrap();
                println!("{json}");
                assert_eq!(sig, serde_json::from_str(&json).unwrap());
            }
        }

        #[test]
        fn multisig_fixtures() {
            const FIXTURE1: &str = "sgIDAwCTLgVngjC4yeuvpAGKVkgcvIKVFUJnL1r6oFZScQVE5DNIz6kfxAGDRcVUczE9CUb7/sN/EuFJ8ot86Sdb8pAFASoQ91stRHXdW5dLy0BQ6v+7XWptawy2ItMyPk508p+PHdtZcm2aKl3lZGIvXe6MPY73E+1Hakv/xJbTYsw5SPMC5dx3gBwxds2GV12c7VUSqkyXamliSF1W/QBMufqrlmdIOZ1ox9gbsvIPtXYahfvKm8ozA7rsZWwRv8atsnyfYgcAAwANfas1jI2tqk76AEmnWwdDZVWxCjaCGbtoD3BXE0nXdQEBAg4XzVk55GsZZkGWjNdZkQuiV34n+nP944dtub7FvOsrAQIDR/uvI/A4q8TDCKJxEXoqTP+u3bxf+Bx1F7xsdKfttDABAgA=";

            const FIXTURE2: &str = "8QEDAgBMW4Oq7XMjO5c6HLgTBJrWDZsCEcZF2EPOf68fdf1aY3e3pvA3cmk0tjMmXFB9+A6J2NohCpTFb/CsXEBjtCcMAfraaMMOMzG815145jlrY44Rbp0d1JQJOJ3hjgEe2xVBFP3QR94IVZk6ssyYxsecpBA+re5eqVRacvZGSobNPkMDAAMADX2rNYyNrapO+gBJp1sHQ2VVsQo2ghm7aA9wVxNJ13UBAQIOF81ZOeRrGWZBlozXWZELold+J/pz/eOHbbm+xbzrKwECA0f7ryPwOKvEwwiicRF6Kkz/rt28X/gcdRe8bHSn7bQwAQIA";

            for fixture in [FIXTURE1, FIXTURE2] {
                let bcs = <base64ct::Base64 as base64ct::Encoding>::decode_vec(fixture).unwrap();

                let sig: UserSignature = bcs::from_bytes(&bcs).unwrap();
                assert_eq!(SignatureScheme::Multisig, sig.scheme());
                let bytes = bcs::to_bytes(&sig).unwrap();
                assert_eq!(bcs, bytes);

                let json = serde_json::to_string_pretty(&sig).unwrap();
                println!("{json}");
                assert_eq!(sig, serde_json::from_str(&json).unwrap());
            }
        }
    }
}
