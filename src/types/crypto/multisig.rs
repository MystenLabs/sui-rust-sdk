use super::Ed25519PublicKey;
use super::Ed25519Signature;
use super::Secp256k1PublicKey;
use super::Secp256k1Signature;
use super::Secp256r1PublicKey;
use super::Secp256r1Signature;

pub type WeightUnit = u8;
pub type ThresholdUnit = u16;
pub type BitmapUnit = u16;

#[cfg(feature = "serde")]
const MAX_COMMITTEE_SIZE: usize = 10;
// TODO validate sigs
// const MAX_BITMAP_VALUE: BitmapUnit = 0b1111111111;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum MultisigMember {
    Ed25519(Ed25519PublicKey),
    Secp256k1(Secp256k1PublicKey),
    Secp256r1(Secp256r1PublicKey),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct MultisigCommittee {
    /// A list of committee members and their corresponding weight.
    members: Vec<(MultisigMember, WeightUnit)>,
    /// If the total weight of the public keys corresponding to verified signatures is larger than threshold, the Multisig is verified.
    threshold: ThresholdUnit,
}

impl MultisigCommittee {
    pub fn members(&self) -> &[(MultisigMember, WeightUnit)] {
        &self.members
    }

    pub fn threshold(&self) -> ThresholdUnit {
        self.threshold
    }
}

/// The struct that contains signatures and public keys necessary for authenticating a Multisig.
#[derive(Debug, Clone)]
pub struct MultisigAggregatedSignature {
    /// The plain signature encoded with signature scheme.
    signatures: Vec<MultisigMemberSignature>,
    /// A bitmap that indicates the position of which public key the signature should be authenticated with.
    bitmap: BitmapUnit,
    /// Legacy encoding for the bitmap.
    // TODO remove the allow(dead_code) attr once the public interface has been fleshed out more
    #[cfg_attr(not(feature = "serde"), allow(dead_code))]
    legacy_bitmap: Option<roaring::RoaringBitmap>,
    /// The public key encoded with each public key with its signature scheme used along with the corresponding weight.
    committee: MultisigCommittee,
}

impl MultisigAggregatedSignature {
    pub fn signatures(&self) -> &[MultisigMemberSignature] {
        &self.signatures
    }

    pub fn bitmap(&self) -> BitmapUnit {
        self.bitmap
    }

    pub fn committee(&self) -> &MultisigCommittee {
        &self.committee
    }
}

impl PartialEq for MultisigAggregatedSignature {
    fn eq(&self, other: &Self) -> bool {
        // Skip comparing the legacy bitmap since we always convert to the new bitmap form
        self.bitmap == other.bitmap
            && self.signatures == other.signatures
            && self.committee == other.committee
    }
}

impl Eq for MultisigAggregatedSignature {}

/// Convert a roaring bitmap to plain bitmap.
#[cfg(feature = "serde")]
fn roaring_bitmap_to_u16(roaring: &roaring::RoaringBitmap) -> Result<BitmapUnit, &'static str> {
    let mut val = 0;
    for i in roaring.iter() {
        if i >= MAX_COMMITTEE_SIZE as u32 {
            return Err("invalid bitmap");
        }
        val |= 1 << i as u8;
    }
    Ok(val)
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum MultisigMemberSignature {
    Ed25519(Ed25519Signature),
    Secp256k1(Secp256k1Signature),
    Secp256r1(Secp256r1Signature),
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use super::*;
    use crate::types::crypto::Base64Array33;
    use crate::types::crypto::Base64Array34;
    use crate::types::Ed25519PublicKey;
    use crate::types::Secp256k1PublicKey;
    use crate::types::Secp256r1PublicKey;
    use crate::types::SignatureScheme;
    use base64ct::{Base64, Encoding};
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;
    use serde_with::Bytes;
    use serde_with::DeserializeAs;
    use serde_with::SerializeAs;
    use std::borrow::Cow;

    /// Serializes a bitmap according to the roaring bitmap on-disk standard.
    /// <https://github.com/RoaringBitmap/RoaringFormatSpec>
    pub struct BinaryRoaringBitmap;

    impl SerializeAs<roaring::RoaringBitmap> for BinaryRoaringBitmap {
        fn serialize_as<S>(
            source: &roaring::RoaringBitmap,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut bytes = vec![];

            source
                .serialize_into(&mut bytes)
                .map_err(serde::ser::Error::custom)?;
            Bytes::serialize_as(&bytes, serializer)
        }
    }

    impl<'de> DeserializeAs<'de, roaring::RoaringBitmap> for BinaryRoaringBitmap {
        fn deserialize_as<D>(deserializer: D) -> Result<roaring::RoaringBitmap, D::Error>
        where
            D: Deserializer<'de>,
        {
            let bytes: Cow<'de, [u8]> = Bytes::deserialize_as(deserializer)?;
            roaring::RoaringBitmap::deserialize_from(&bytes[..]).map_err(serde::de::Error::custom)
        }
    }

    pub struct Base64RoaringBitmap;

    impl SerializeAs<roaring::RoaringBitmap> for Base64RoaringBitmap {
        fn serialize_as<S>(
            source: &roaring::RoaringBitmap,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut bytes = vec![];

            source
                .serialize_into(&mut bytes)
                .map_err(serde::ser::Error::custom)?;
            let b64 = Base64::encode_string(&bytes);
            b64.serialize(serializer)
        }
    }

    impl<'de> DeserializeAs<'de, roaring::RoaringBitmap> for Base64RoaringBitmap {
        fn deserialize_as<D>(deserializer: D) -> Result<roaring::RoaringBitmap, D::Error>
        where
            D: Deserializer<'de>,
        {
            let b64: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
            let bytes = Base64::decode_vec(&b64).map_err(serde::de::Error::custom)?;
            roaring::RoaringBitmap::deserialize_from(&bytes[..]).map_err(serde::de::Error::custom)
        }
    }

    pub struct Base64MultisigMember;

    impl SerializeAs<MultisigMember> for Base64MultisigMember {
        fn serialize_as<S>(source: &MultisigMember, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match source {
                MultisigMember::Ed25519(public_key) => {
                    let mut buf = [0; 1 + Ed25519PublicKey::LENGTH];
                    buf[0] = SignatureScheme::Ed25519 as u8;
                    buf[1..].copy_from_slice(public_key.as_ref());
                    Base64Array33::serialize_as(&buf, serializer)
                }
                MultisigMember::Secp256k1(public_key) => {
                    let mut buf = [0; 1 + Secp256k1PublicKey::LENGTH];
                    buf[0] = SignatureScheme::Secp256k1 as u8;
                    buf[1..].copy_from_slice(public_key.as_ref());
                    Base64Array34::serialize_as(&buf, serializer)
                }
                MultisigMember::Secp256r1(public_key) => {
                    let mut buf = [0; 1 + Secp256r1PublicKey::LENGTH];
                    buf[0] = SignatureScheme::Secp256r1 as u8;
                    buf[1..].copy_from_slice(public_key.as_ref());
                    Base64Array34::serialize_as(&buf, serializer)
                }
            }
        }
    }

    impl<'de> DeserializeAs<'de, MultisigMember> for Base64MultisigMember {
        fn deserialize_as<D>(deserializer: D) -> Result<MultisigMember, D::Error>
        where
            D: Deserializer<'de>,
        {
            let b64: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
            let bytes = Base64::decode_vec(&b64).map_err(serde::de::Error::custom)?;
            let flag = SignatureScheme::from_byte(bytes[0]).map_err(serde::de::Error::custom)?;
            let public_key_bytes = &bytes[1..];
            match flag {
                SignatureScheme::Ed25519 => {
                    let public_key = Ed25519PublicKey::from_bytes(public_key_bytes)
                        .map_err(serde::de::Error::custom)?;
                    Ok(MultisigMember::Ed25519(public_key))
                }
                SignatureScheme::Secp256k1 => {
                    let public_key = Secp256k1PublicKey::from_bytes(public_key_bytes)
                        .map_err(serde::de::Error::custom)?;
                    Ok(MultisigMember::Secp256k1(public_key))
                }
                SignatureScheme::Secp256r1 => {
                    let public_key = Secp256r1PublicKey::from_bytes(public_key_bytes)
                        .map_err(serde::de::Error::custom)?;
                    Ok(MultisigMember::Secp256r1(public_key))
                }
                SignatureScheme::Multisig
                | SignatureScheme::BLS12381
                | SignatureScheme::ZkLoginAuthenticator => {
                    Err(serde::de::Error::custom("invalid public key type"))
                }
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    pub struct Multisig {
        signatures: Vec<MultisigMemberSignature>,
        bitmap: BitmapUnit,
        committee: MultisigCommittee,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    pub struct LegacyMultisig {
        signatures: Vec<MultisigMemberSignature>,
        #[serde(with = "::serde_with::As::<BinaryRoaringBitmap>")]
        bitmap: roaring::RoaringBitmap,
        committee: LegacyMultisigCommittee,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    struct LegacyMultisigCommittee {
        /// A list of committee members and their corresponding weight.
        #[serde(with = "::serde_with::As::<Vec<(Base64MultisigMember, ::serde_with::Same)>>")]
        members: Vec<(MultisigMember, WeightUnit)>,
        /// If the total weight of the public keys corresponding to verified signatures is larger than threshold, the Multisig is verified.
        threshold: ThresholdUnit,
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    struct ReadableMultisigAggregatedSignature {
        signatures: Vec<MultisigMemberSignature>,
        bitmap: BitmapUnit,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "::serde_with::As::<Option<Base64RoaringBitmap>>")]
        legacy_bitmap: Option<roaring::RoaringBitmap>,
        committee: MultisigCommittee,
    }

    impl Serialize for MultisigAggregatedSignature {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = ReadableMultisigAggregatedSignature {
                    signatures: self.signatures.clone(),
                    bitmap: self.bitmap,
                    legacy_bitmap: self.legacy_bitmap.clone(),
                    committee: self.committee.clone(),
                };
                readable.serialize(serializer)
            } else {
                let mut buf = Vec::new();
                buf.push(SignatureScheme::Multisig as u8);

                if let Some(bitmap) = &self.legacy_bitmap {
                    let legacy = LegacyMultisig {
                        signatures: self.signatures.clone(),
                        bitmap: bitmap.clone(),
                        committee: LegacyMultisigCommittee {
                            members: self.committee.members.clone(),
                            threshold: self.committee.threshold,
                        },
                    };

                    bcs::serialize_into(&mut buf, &legacy).expect("serialization cannot fail");
                } else {
                    let multisig = Multisig {
                        signatures: self.signatures.clone(),
                        bitmap: self.bitmap,
                        committee: self.committee.clone(),
                    };
                    bcs::serialize_into(&mut buf, &multisig).expect("serialization cannot fail");
                }
                serializer.serialize_bytes(&buf)
            }
        }
    }

    impl<'de> Deserialize<'de> for MultisigAggregatedSignature {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let readable = ReadableMultisigAggregatedSignature::deserialize(deserializer)?;
                Ok(Self {
                    signatures: readable.signatures,
                    bitmap: readable.bitmap,
                    legacy_bitmap: readable.legacy_bitmap,
                    committee: readable.committee,
                })
            } else {
                let bytes: Cow<'de, [u8]> = Bytes::deserialize_as(deserializer)?;
                let flag =
                    SignatureScheme::from_byte(bytes[0]).map_err(serde::de::Error::custom)?;
                if flag != SignatureScheme::Multisig {
                    return Err(serde::de::Error::custom("invalid multisig flag"));
                }
                let bcs_bytes = &bytes[1..];

                // Unfortunately we have no information in the serialized form of a Multisig to be
                // able to determine if its a Legacy format or the new standard format so we just
                // need to try each.
                //
                // We'll start with the newer format as that should be more prevalent.
                if let Ok(multisig) = bcs::from_bytes::<Multisig>(bcs_bytes) {
                    Ok(Self {
                        signatures: multisig.signatures,
                        bitmap: multisig.bitmap,
                        legacy_bitmap: None,
                        committee: multisig.committee,
                    })
                } else if let Ok(legacy) = bcs::from_bytes::<LegacyMultisig>(bcs_bytes) {
                    Ok(Self {
                        signatures: legacy.signatures,
                        bitmap: roaring_bitmap_to_u16(&legacy.bitmap)
                            .map_err(serde::de::Error::custom)?,
                        legacy_bitmap: Some(legacy.bitmap),
                        committee: MultisigCommittee {
                            members: legacy.committee.members,
                            threshold: legacy.committee.threshold,
                        },
                    })
                } else {
                    Err(serde::de::Error::custom("invalid multisig"))
                }
            }
        }
    }
}
