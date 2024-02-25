use super::SimpleSignature;
use crate::types::checkpoint::EpochId;

/// An zk login authenticator with all the necessary fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZkLoginAuthenticator {
    inputs: ZkLoginInputs,
    max_epoch: EpochId,
    signature: SimpleSignature,
}

/// All inputs required for the zk login proof verification and other public inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct ZkLoginInputs {
    proof_points: ZkLoginProof,
    iss_base64_details: Claim,
    header_base64: String,
    address_seed: String,
    // #[serde(skip)]
    // jwt_details: JWTDetails,
}

/// A claim consists of value and index_mod_4.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Claim {
    value: String,
    index_mod_4: u8,
}

/// A structed of parsed JWT details, consists of kid, header, iss.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct JWTDetails {
    kid: String,
    header: String,
    iss: String,
}

/// The struct for zk login proof.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct ZkLoginProof {
    a: CircomG1,
    b: CircomG2,
    c: CircomG1,
}

/// A G1 point in BN254 serialized as a vector of three strings which is the canonical decimal
/// representation of the projective coordinates in Fq.
pub type CircomG1 = Vec<String>;

/// A G2 point in BN254 serialized as a vector of three vectors each being a vector of two strings
/// which are the canonical decimal representation of the coefficients of the projective coordinates
/// in Fq2.
pub type CircomG2 = Vec<Vec<String>>;

/// A wrapper struct to retrofit in [enum PublicKey] for zkLogin.
/// Useful to construct [struct MultiSigPublicKey].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZkLoginPublicIdentifier {
    iss: String,
    //TODO bigint support
    address_seed: [u8; 32],
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use crate::types::SignatureScheme;

    use super::*;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;
    use serde_with::Bytes;
    use serde_with::DeserializeAs;
    use std::borrow::Cow;

    // Serialized format is: iss_bytes_len || iss_bytes || padded_32_byte_address_seed.
    impl Serialize for ZkLoginPublicIdentifier {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                #[derive(serde_derive::Serialize)]
                struct Readable<'a> {
                    iss: &'a str,
                    //TODO this needs to be encoded as a Decimal u256 instead of in base64
                    #[cfg_attr(
                        feature = "serde",
                        serde(with = "::serde_with::As::<crate::types::crypto::Base64Array32>")
                    )]
                    address_seed: [u8; 32],
                }
                let readable = Readable {
                    iss: &self.iss,
                    address_seed: self.address_seed,
                };
                readable.serialize(serializer)
            } else {
                let mut buf = Vec::new();
                let iss_bytes = self.iss.as_bytes();
                buf.push(iss_bytes.len() as u8);
                buf.extend(iss_bytes);

                buf.extend(&self.address_seed);

                serializer.serialize_bytes(&buf)
            }
        }
    }

    impl<'de> Deserialize<'de> for ZkLoginPublicIdentifier {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                #[derive(serde_derive::Deserialize)]
                struct Readable {
                    iss: String,
                    //TODO this needs to be encoded as a Decimal u256 instead of in base64
                    #[cfg_attr(
                        feature = "serde",
                        serde(with = "::serde_with::As::<crate::types::crypto::Base64Array32>")
                    )]
                    address_seed: [u8; 32],
                }

                let Readable { iss, address_seed } = Deserialize::deserialize(deserializer)?;
                Ok(Self { iss, address_seed })
            } else {
                let bytes: Cow<'de, [u8]> = Bytes::deserialize_as(deserializer)?;
                let iss_len = bytes[0];
                let iss_bytes = bytes
                    .get(1..(1 + iss_len as usize))
                    .ok_or_else(|| serde::de::Error::custom("invalid zklogin public identifier"))?;
                let iss = std::str::from_utf8(iss_bytes).map_err(serde::de::Error::custom)?;
                let address_seed_bytes = bytes
                    .get((1 + iss_len as usize)..)
                    .ok_or_else(|| serde::de::Error::custom("invalid zklogin public identifier"))?;

                let address_seed =
                    <[u8; 32]>::try_from(address_seed_bytes).map_err(serde::de::Error::custom)?;

                Ok(Self {
                    iss: iss.into(),
                    address_seed,
                })
            }
        }
    }

    #[derive(serde_derive::Serialize)]
    struct AuthenticatorRef<'a> {
        inputs: &'a ZkLoginInputs,
        max_epoch: EpochId,
        signature: &'a SimpleSignature,
    }

    #[derive(serde_derive::Deserialize)]
    struct Authenticator {
        inputs: ZkLoginInputs,
        max_epoch: EpochId,
        signature: SimpleSignature,
    }

    impl Serialize for ZkLoginAuthenticator {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let authenticator_ref = AuthenticatorRef {
                inputs: &self.inputs,
                max_epoch: self.max_epoch,
                signature: &self.signature,
            };
            if serializer.is_human_readable() {
                authenticator_ref.serialize(serializer)
            } else {
                let mut buf = Vec::new();
                buf.push(SignatureScheme::ZkLogin as u8);

                bcs::serialize_into(&mut buf, &authenticator_ref)
                    .expect("serialization cannot fail");
                serializer.serialize_bytes(&buf)
            }
        }
    }

    impl<'de> Deserialize<'de> for ZkLoginAuthenticator {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let Authenticator {
                    inputs,
                    max_epoch,
                    signature,
                } = Authenticator::deserialize(deserializer)?;
                Ok(Self {
                    inputs,
                    max_epoch,
                    signature,
                })
            } else {
                let bytes: Cow<'de, [u8]> = Bytes::deserialize_as(deserializer)?;
                Self::from_serialized_bytes(bytes)
            }
        }
    }

    impl ZkLoginAuthenticator {
        pub(crate) fn from_serialized_bytes<T: AsRef<[u8]>, E: serde::de::Error>(
            bytes: T,
        ) -> Result<Self, E> {
            let bytes = bytes.as_ref();
            let flag = SignatureScheme::from_byte(bytes[0]).map_err(serde::de::Error::custom)?;
            if flag != SignatureScheme::ZkLogin {
                return Err(serde::de::Error::custom("invalid zklogin flag"));
            }
            let bcs_bytes = &bytes[1..];

            let Authenticator {
                inputs,
                max_epoch,
                signature,
            } = bcs::from_bytes(bcs_bytes).map_err(serde::de::Error::custom)?;
            Ok(Self {
                inputs,
                max_epoch,
                signature,
            })
        }
    }
}
