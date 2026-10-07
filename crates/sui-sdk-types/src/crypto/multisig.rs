use super::Ed25519PublicKey;
use super::Ed25519Signature;
use super::MlDsa65PublicKey;
use super::MlDsa65Signature;
use super::PasskeyAuthenticator;
use super::PasskeyPublicKey;
use super::Secp256k1PublicKey;
use super::Secp256k1Signature;
use super::Secp256r1PublicKey;
use super::Secp256r1Signature;
use super::SignatureScheme;
use super::zklogin::ZkLoginAuthenticator;
use super::zklogin::ZkLoginPublicIdentifier;

pub type WeightUnit = u8;
pub type ThresholdUnit = u16;
pub type BitmapUnit = u16;

const MAX_COMMITTEE_SIZE: usize = 10;
// TODO validate sigs
// const MAX_BITMAP_VALUE: BitmapUnit = 0b1111111111;

/// Enum of valid public keys for multisig committee members
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// multisig-member-public-key = ed25519-multisig-member-public-key /
///                              secp256k1-multisig-member-public-key /
///                              secp256r1-multisig-member-public-key /
///                              zklogin-multisig-member-public-key
///
/// ed25519-multisig-member-public-key   = %x00 ed25519-public-key
/// secp256k1-multisig-member-public-key = %x01 secp256k1-public-key
/// secp256r1-multisig-member-public-key = %x02 secp256r1-public-key
/// zklogin-multisig-member-public-key   = %x03 zklogin-public-identifier
/// ```
///
/// There is also a legacy encoding for this type defined as:
///
/// ```text
/// legacy-multisig-member-public-key = string ; which is valid base64 encoded
///                                            ; and the decoded bytes are defined
///                                            ; by legacy-public-key
/// legacy-public-key = (ed25519-flag ed25519-public-key) /
///                     (secp256k1-flag secp256k1-public-key) /
///                     (secp256r1-flag secp256r1-public-key)
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
#[non_exhaustive]
pub enum MultisigMemberPublicKey {
    Ed25519(Ed25519PublicKey),
    Secp256k1(Secp256k1PublicKey),
    Secp256r1(Secp256r1PublicKey),
    ZkLogin(ZkLoginPublicIdentifier),
    Passkey(PasskeyPublicKey),
    MlDsa65(Box<MlDsa65PublicKey>),
}

/// A member in a multisig committee
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// multisig-member = multisig-member-public-key
///                   u8    ; weight
/// ```
///
/// There is also a legacy encoding for this type defined as:
///
/// ```text
/// legacy-multisig-member = legacy-multisig-member-public-key
///                          u8     ; weight
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct MultisigMember {
    public_key: MultisigMemberPublicKey,
    weight: WeightUnit,
}

impl MultisigMember {
    /// Construct a new member from a `MultisigMemberPublicKey` and a `weight`.
    pub fn new(public_key: MultisigMemberPublicKey, weight: WeightUnit) -> Self {
        Self { public_key, weight }
    }

    /// This member's public key.
    pub fn public_key(&self) -> &MultisigMemberPublicKey {
        &self.public_key
    }

    /// Weight of this member's signature.
    pub fn weight(&self) -> WeightUnit {
        self.weight
    }
}

/// A multisig committee
///
/// A `MultisigCommittee` is a set of members who collectively control a single `Address` on the
/// Sui blockchain. The number of required signautres to authorize the execution of a transaction
/// is determined by `(signature_0_weight + signature_1_weight ..) >= threshold`.
///
/// # Validity
///
/// Deserialization (BCS, JSON, or `from_serialized_bytes` on a containing
/// `MultisigAggregatedSignature`) does **not** enforce structural validity:
/// the resulting committee may have zero members, zero threshold, threshold
/// greater than the sum of weights, duplicate members, or more than the
/// `MAX_COMMITTEE_SIZE` limit. Validity is checked downstream by the
/// verifier in `sui-crypto` before any signature is verified.
///
/// Consumers who inspect a deserialized committee — counting members,
/// summing weights, indexing by bitmap, etc. — without first running
/// signature verification **must** call [`MultisigCommittee::is_valid`]
/// and reject the committee if it returns `false`. Skipping this check
/// can cause downstream code to operate on attacker-supplied,
/// well-formed-looking but malformed committees.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// multisig-committee = (vector multisig-member)
///                      u16    ; threshold
/// ```
///
/// There is also a legacy encoding for this type defined as:
///
/// ```text
/// legacy-multisig-committee = (vector legacy-multisig-member)
///                             u16     ; threshold
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct MultisigCommittee {
    /// A list of committee members and their corresponding weight.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=10).lift()))]
    members: Vec<MultisigMember>,

    /// If the total weight of the public keys corresponding to verified signatures is larger than
    /// threshold, the Multisig is verified.
    threshold: ThresholdUnit,
}

impl MultisigCommittee {
    /// Construct a new committee from a list of `MultisigMember`s and a `threshold`.
    ///
    /// Note that the order of the members is significant towards deriving the `Address` governed
    /// by this committee.
    pub fn new(members: Vec<MultisigMember>, threshold: ThresholdUnit) -> Self {
        Self { members, threshold }
    }

    /// The members of the committee
    pub fn members(&self) -> &[MultisigMember] {
        &self.members
    }

    /// The total signature weight required to authorize a transaction for the address
    /// corresponding to this `MultisigCommittee`.
    pub fn threshold(&self) -> ThresholdUnit {
        self.threshold
    }

    /// Return the flag for this signature scheme
    pub fn scheme(&self) -> SignatureScheme {
        SignatureScheme::Multisig
    }

    /// Checks if the Committee is valid.
    ///
    /// A valid committee is one that:
    ///  - Has a nonzero threshold
    ///  - Has at least one member
    ///  - Has at most ten members
    ///  - No member has weight 0
    ///  - the sum of the weights of all members must be larger than the threshold
    ///  - contains no duplicate members
    pub fn is_valid(&self) -> bool {
        self.threshold != 0
            && !self.members.is_empty()
            && self.members.len() <= MAX_COMMITTEE_SIZE
            && !self.members.iter().any(|member| member.weight == 0)
            && self
                .members
                .iter()
                .map(|member| member.weight as ThresholdUnit)
                .sum::<ThresholdUnit>()
                >= self.threshold
            && !self.members.iter().enumerate().any(|(i, member)| {
                self.members
                    .iter()
                    .skip(i + 1)
                    .any(|m| member.public_key == m.public_key)
            })
    }
}

/// Aggregated signature from members of a multisig committee.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// multisig-aggregated-signature = (vector multisig-member-signature)
///                                 u16     ; bitmap
///                                 multisig-committee
/// ```
///
/// There is also a legacy encoding for this type defined as:
///
/// ```text
/// legacy-multisig-aggregated-signature = (vector multisig-member-signature)
///                                        roaring-bitmap   ; bitmap
///                                        legacy-multisig-committee
/// roaring-bitmap = bytes  ; where the contents of the bytes are valid
///                         ; according to the serialized spec for
///                         ; roaring bitmaps
/// ```
///
/// See [here](https://github.com/RoaringBitmap/RoaringFormatSpec) for the specification for the
/// serialized format of RoaringBitmaps.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct MultisigAggregatedSignature {
    /// The plain signature encoded with signature scheme.
    ///
    /// The signatures must be in the same order as they are listed in the committee.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=10).lift()))]
    signatures: Vec<MultisigMemberSignature>,
    /// A bitmap that indicates the position of which public key the signature should be
    /// authenticated with.
    bitmap: BitmapUnit,
    /// Legacy encoding for the bitmap.
    //TODO implement a strategy for legacy bitmap
    #[cfg_attr(feature = "proptest", strategy(proptest::strategy::Just(None)))]
    legacy_bitmap: Option<crate::Bitmap>,
    /// The public key encoded with each public key with its signature scheme used along with the
    /// corresponding weight.
    committee: MultisigCommittee,
}

impl MultisigAggregatedSignature {
    /// Construct a new aggregated multisig signature.
    ///
    /// Since the list of signatures doesn't contain sufficient information to identify which
    /// committee member provided the signature, it is up to the caller to ensure that the provided
    /// signature list is in the same order as it's corresponding member in the provided committee
    /// and that it's position in the provided bitmap is set.
    pub fn new(
        committee: MultisigCommittee,
        signatures: Vec<MultisigMemberSignature>,
        bitmap: BitmapUnit,
    ) -> Self {
        Self {
            signatures,
            bitmap,
            legacy_bitmap: None,
            committee,
        }
    }

    /// The list of signatures from committee members
    pub fn signatures(&self) -> &[MultisigMemberSignature] {
        &self.signatures
    }

    /// The bitmap that indicates which committee members provided their signature.
    pub fn bitmap(&self) -> BitmapUnit {
        self.bitmap
    }

    /// The legacy roaring bitmap, if this is a legacy formatted signature
    pub fn legacy_bitmap(&self) -> Option<&crate::Bitmap> {
        self.legacy_bitmap.as_ref()
    }

    /// Configure with a legacy roaring bitmap
    #[deprecated(note = "use `try_with_legacy_bitmap`, which rejects unserializable bitmaps")]
    pub fn with_legacy_bitmap(&mut self, legacy_bitmap: crate::Bitmap) {
        self.legacy_bitmap = Some(legacy_bitmap);
    }

    /// Configure with a legacy roaring bitmap.
    ///
    /// When a legacy bitmap is present, this signature serializes in the
    /// legacy format, which carries the signer set only as the roaring
    /// bitmap and can only encode Ed25519, Secp256k1, and Secp256r1
    /// committee members. Returns an error, leaving `self` unchanged, if
    /// `legacy_bitmap` does not encode the same signer set as
    /// [`Self::bitmap`] or if the committee has a member that the legacy
    /// format cannot encode.
    pub fn try_with_legacy_bitmap(
        &mut self,
        legacy_bitmap: crate::Bitmap,
    ) -> Result<(), InvalidLegacyBitmapError> {
        validate_legacy_bitmap(&self.committee, self.bitmap, &legacy_bitmap)?;
        self.legacy_bitmap = Some(legacy_bitmap);
        Ok(())
    }

    /// The committee for this aggregated signature
    pub fn committee(&self) -> &MultisigCommittee {
        &self.committee
    }
}

impl PartialEq for MultisigAggregatedSignature {
    fn eq(&self, other: &Self) -> bool {
        // Compare every field, including `legacy_bitmap`. Although the
        // legacy bitmap is logically redundant with `bitmap` (they encode
        // the same information in different formats), `to_bytes` prefers
        // the legacy form whenever it is `Some`, so two signatures that
        // differ only by `legacy_bitmap` will serialize to different byte
        // strings. Excluding `legacy_bitmap` from `==` would let downstream
        // consumers observe values where `a == b` but
        // `bcs::to_bytes(a) != bcs::to_bytes(b)`, breaking standard
        // Eq/Serialize expectations.
        self.bitmap == other.bitmap
            && self.legacy_bitmap == other.legacy_bitmap
            && self.signatures == other.signatures
            && self.committee == other.committee
    }
}

impl Eq for MultisigAggregatedSignature {}

/// Convert a roaring bitmap to plain bitmap.
fn roaring_bitmap_to_u16(roaring: &crate::Bitmap) -> Result<BitmapUnit, &'static str> {
    let mut val = 0;
    for i in roaring.iter() {
        if i >= MAX_COMMITTEE_SIZE as u32 {
            return Err("invalid bitmap");
        }
        val |= 1 << i as u8;
    }
    Ok(val)
}

/// Check that `legacy_bitmap` can be attached to a multisig with the given
/// `committee` and `bitmap`.
///
/// The BCS legacy decoder always derives `bitmap` from the roaring bitmap, so
/// requiring the same here keeps `bitmap()` and `to_bytes()` from reporting
/// two different signer sets for one signature. The legacy member encoding
/// (`Base64MultisigMemberPublicKey`) only supports Ed25519, Secp256k1, and
/// Secp256r1, so any other member would make `to_bytes()` fail to serialize
/// the legacy form and panic.
fn validate_legacy_bitmap(
    committee: &MultisigCommittee,
    bitmap: BitmapUnit,
    legacy_bitmap: &crate::Bitmap,
) -> Result<(), InvalidLegacyBitmapError> {
    let derived = roaring_bitmap_to_u16(legacy_bitmap).map_err(InvalidLegacyBitmapError)?;
    if derived != bitmap {
        return Err(InvalidLegacyBitmapError(
            "bitmap does not match legacy_bitmap",
        ));
    }

    for member in &committee.members {
        let reason = match member.public_key {
            MultisigMemberPublicKey::Ed25519(_)
            | MultisigMemberPublicKey::Secp256k1(_)
            | MultisigMemberPublicKey::Secp256r1(_) => continue,
            MultisigMemberPublicKey::ZkLogin(_) => {
                "zklogin member is not representable in legacy multisig"
            }
            MultisigMemberPublicKey::Passkey(_) => {
                "passkey member is not representable in legacy multisig"
            }
            MultisigMemberPublicKey::MlDsa65(_) => {
                "mldsa65 member is not representable in legacy multisig"
            }
        };
        return Err(InvalidLegacyBitmapError(reason));
    }

    Ok(())
}

/// Error returned when a legacy roaring bitmap cannot be attached to a
/// [`MultisigAggregatedSignature`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidLegacyBitmapError(&'static str);

impl std::fmt::Display for InvalidLegacyBitmapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl std::error::Error for InvalidLegacyBitmapError {}

/// A signature from a member of a multisig committee.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// multisig-member-signature = ed25519-multisig-member-signature /
///                             secp256k1-multisig-member-signature /
///                             secp256r1-multisig-member-signature /
///                             zklogin-multisig-member-signature
///
/// ed25519-multisig-member-signature   = %x00 ed25519-signature
/// secp256k1-multisig-member-signature = %x01 secp256k1-signature
/// secp256r1-multisig-member-signature = %x02 secp256r1-signature
/// zklogin-multisig-member-signature   = %x03 zklogin-authenticator
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
#[non_exhaustive]
pub enum MultisigMemberSignature {
    Ed25519(Ed25519Signature),
    Secp256k1(Secp256k1Signature),
    Secp256r1(Secp256r1Signature),
    ZkLogin(Box<ZkLoginAuthenticator>),
    Passkey(PasskeyAuthenticator),
    MlDsa65(Box<MlDsa65Signature>),
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use super::*;
    use crate::Ed25519PublicKey;
    use crate::PasskeyPublicKey;
    use crate::Secp256k1PublicKey;
    use crate::Secp256r1PublicKey;
    use crate::SignatureScheme;
    use crate::crypto::Base64Array33;
    use crate::crypto::Base64Array34;
    use base64ct::Base64;
    use base64ct::Encoding;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;
    use serde_with::Bytes;
    use serde_with::DeserializeAs;
    use serde_with::SerializeAs;
    use std::borrow::Cow;

    pub struct Base64MultisigMemberPublicKey;

    impl SerializeAs<MultisigMemberPublicKey> for Base64MultisigMemberPublicKey {
        fn serialize_as<S>(
            source: &MultisigMemberPublicKey,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match source {
                MultisigMemberPublicKey::Ed25519(public_key) => {
                    let mut buf = [0; 1 + Ed25519PublicKey::LENGTH];
                    buf[0] = SignatureScheme::Ed25519 as u8;
                    buf[1..].copy_from_slice(public_key.as_ref());
                    Base64Array33::serialize_as(&buf, serializer)
                }
                MultisigMemberPublicKey::Secp256k1(public_key) => {
                    let mut buf = [0; 1 + Secp256k1PublicKey::LENGTH];
                    buf[0] = SignatureScheme::Secp256k1 as u8;
                    buf[1..].copy_from_slice(public_key.as_ref());
                    Base64Array34::serialize_as(&buf, serializer)
                }
                MultisigMemberPublicKey::Secp256r1(public_key) => {
                    let mut buf = [0; 1 + Secp256r1PublicKey::LENGTH];
                    buf[0] = SignatureScheme::Secp256r1 as u8;
                    buf[1..].copy_from_slice(public_key.as_ref());
                    Base64Array34::serialize_as(&buf, serializer)
                }
                MultisigMemberPublicKey::ZkLogin(_) => Err(serde::ser::Error::custom(
                    "zklogin not supported in legacy multisig",
                )),
                MultisigMemberPublicKey::Passkey(_) => Err(serde::ser::Error::custom(
                    "passkey not supported in legacy multisig",
                )),
                MultisigMemberPublicKey::MlDsa65(_) => Err(serde::ser::Error::custom(
                    "mldsa65 not supported in legacy multisig",
                )),
            }
        }
    }

    impl<'de> DeserializeAs<'de, MultisigMemberPublicKey> for Base64MultisigMemberPublicKey {
        fn deserialize_as<D>(deserializer: D) -> Result<MultisigMemberPublicKey, D::Error>
        where
            D: Deserializer<'de>,
        {
            let b64: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
            let bytes = Base64::decode_vec(&b64).map_err(serde::de::Error::custom)?;
            let flag = SignatureScheme::from_byte(
                *bytes
                    .first()
                    .ok_or_else(|| serde::de::Error::custom("missing signature scheme flag"))?,
            )
            .map_err(serde::de::Error::custom)?;
            let public_key_bytes = &bytes[1..];
            match flag {
                SignatureScheme::Ed25519 => {
                    let public_key = Ed25519PublicKey::from_bytes(public_key_bytes)
                        .map_err(serde::de::Error::custom)?;
                    Ok(MultisigMemberPublicKey::Ed25519(public_key))
                }
                SignatureScheme::Secp256k1 => {
                    let public_key = Secp256k1PublicKey::from_bytes(public_key_bytes)
                        .map_err(serde::de::Error::custom)?;
                    Ok(MultisigMemberPublicKey::Secp256k1(public_key))
                }
                SignatureScheme::Secp256r1 => {
                    let public_key = Secp256r1PublicKey::from_bytes(public_key_bytes)
                        .map_err(serde::de::Error::custom)?;
                    Ok(MultisigMemberPublicKey::Secp256r1(public_key))
                }
                SignatureScheme::Multisig
                | SignatureScheme::Bls12381
                | SignatureScheme::ZkLogin
                | SignatureScheme::Passkey
                | SignatureScheme::MlDsa65 => {
                    Err(serde::de::Error::custom("invalid public key type"))
                }
            }
        }
    }

    pub struct LegacyMultisigMember;

    impl SerializeAs<MultisigMember> for LegacyMultisigMember {
        fn serialize_as<S>(source: &MultisigMember, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            #[derive(serde_derive::Serialize)]
            struct LegacyMember<'a> {
                #[serde(with = "::serde_with::As::<Base64MultisigMemberPublicKey>")]
                public_key: &'a MultisigMemberPublicKey,
                weight: WeightUnit,
            }

            let legacy = LegacyMember {
                public_key: &source.public_key,
                weight: source.weight,
            };

            legacy.serialize(serializer)
        }
    }

    impl<'de> DeserializeAs<'de, MultisigMember> for LegacyMultisigMember {
        fn deserialize_as<D>(deserializer: D) -> Result<MultisigMember, D::Error>
        where
            D: Deserializer<'de>,
        {
            #[derive(serde_derive::Deserialize)]
            struct LegacyMember {
                #[serde(with = "::serde_with::As::<Base64MultisigMemberPublicKey>")]
                public_key: MultisigMemberPublicKey,
                weight: WeightUnit,
            }

            let legacy = LegacyMember::deserialize(deserializer)?;

            Ok(MultisigMember {
                public_key: legacy.public_key,
                weight: legacy.weight,
            })
        }
    }

    #[derive(serde_derive::Deserialize)]
    pub struct Multisig {
        signatures: Vec<MultisigMemberSignature>,
        bitmap: BitmapUnit,
        committee: MultisigCommittee,
    }

    #[derive(serde_derive::Serialize)]
    pub struct MultisigRef<'a> {
        signatures: &'a [MultisigMemberSignature],
        bitmap: BitmapUnit,
        committee: &'a MultisigCommittee,
    }

    #[derive(serde_derive::Deserialize)]
    pub struct LegacyMultisig {
        signatures: Vec<MultisigMemberSignature>,
        bitmap: crate::Bitmap,
        committee: LegacyMultisigCommittee,
    }

    #[derive(serde_derive::Serialize)]
    pub struct LegacyMultisigRef<'a> {
        signatures: &'a [MultisigMemberSignature],
        bitmap: &'a crate::Bitmap,
        committee: LegacyMultisigCommitteeRef<'a>,
    }

    #[derive(serde_derive::Deserialize)]
    struct LegacyMultisigCommittee {
        #[serde(with = "::serde_with::As::<Vec<LegacyMultisigMember>>")]
        members: Vec<MultisigMember>,
        threshold: ThresholdUnit,
    }

    #[derive(serde_derive::Serialize)]
    struct LegacyMultisigCommitteeRef<'a> {
        #[serde(with = "::serde_with::As::<&[LegacyMultisigMember]>")]
        members: &'a [MultisigMember],
        threshold: ThresholdUnit,
    }

    #[derive(serde_derive::Deserialize)]
    struct ReadableMultisigAggregatedSignature {
        signatures: Vec<MultisigMemberSignature>,
        bitmap: BitmapUnit,
        legacy_bitmap: Option<crate::Bitmap>,
        committee: MultisigCommittee,
    }

    #[derive(serde_derive::Serialize)]
    struct ReadableMultisigAggregatedSignatureRef<'a> {
        signatures: &'a [MultisigMemberSignature],
        bitmap: BitmapUnit,
        #[serde(skip_serializing_if = "Option::is_none")]
        legacy_bitmap: &'a Option<crate::Bitmap>,
        committee: &'a MultisigCommittee,
    }

    impl Serialize for MultisigAggregatedSignature {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = ReadableMultisigAggregatedSignatureRef {
                    signatures: &self.signatures,
                    bitmap: self.bitmap,
                    legacy_bitmap: &self.legacy_bitmap,
                    committee: &self.committee,
                };
                readable.serialize(serializer)
            } else {
                let bytes = self.to_bytes();
                serializer.serialize_bytes(&bytes)
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
                // An untrusted JSON payload must not be able to produce a
                // value whose `bitmap()` and `to_bytes()` disagree on the
                // signer set, or whose `to_bytes()` panics.
                if let Some(legacy_bitmap) = &readable.legacy_bitmap {
                    validate_legacy_bitmap(&readable.committee, readable.bitmap, legacy_bitmap)
                        .map_err(serde::de::Error::custom)?;
                }
                Ok(Self {
                    signatures: readable.signatures,
                    bitmap: readable.bitmap,
                    legacy_bitmap: readable.legacy_bitmap,
                    committee: readable.committee,
                })
            } else {
                let bytes: Cow<'de, [u8]> = Bytes::deserialize_as(deserializer)?;
                Self::from_serialized_bytes(bytes)
            }
        }
    }

    impl MultisigAggregatedSignature {
        pub(crate) fn to_bytes(&self) -> Vec<u8> {
            let mut buf = Vec::new();
            buf.push(SignatureScheme::Multisig as u8);

            if let Some(bitmap) = &self.legacy_bitmap {
                let legacy = LegacyMultisigRef {
                    signatures: &self.signatures,
                    bitmap,
                    committee: LegacyMultisigCommitteeRef {
                        members: &self.committee.members,
                        threshold: self.committee.threshold,
                    },
                };

                bcs::serialize_into(&mut buf, &legacy).expect("serialization cannot fail");
            } else {
                let multisig = MultisigRef {
                    signatures: &self.signatures,
                    bitmap: self.bitmap,
                    committee: &self.committee,
                };
                bcs::serialize_into(&mut buf, &multisig).expect("serialization cannot fail");
            }
            buf
        }

        pub(crate) fn from_serialized_bytes<T: AsRef<[u8]>, E: serde::de::Error>(
            bytes: T,
        ) -> Result<Self, E> {
            let bytes = bytes.as_ref();
            let flag = SignatureScheme::from_byte(
                *bytes
                    .first()
                    .ok_or_else(|| serde::de::Error::custom("missing signature scheme flag"))?,
            )
            .map_err(serde::de::Error::custom)?;
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

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum MemberPublicKey {
        Ed25519(Ed25519PublicKey),
        Secp256k1(Secp256k1PublicKey),
        Secp256r1(Secp256r1PublicKey),
        ZkLogin(ZkLoginPublicIdentifier),
        Passkey(PasskeyPublicKey),
        MlDsa65(Box<MlDsa65PublicKey>),
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "scheme", rename_all = "lowercase")]
    #[serde(rename = "MultisigMemberPublicKey")]
    enum ReadableMemberPublicKey {
        Ed25519 { public_key: Ed25519PublicKey },
        Secp256k1 { public_key: Secp256k1PublicKey },
        Secp256r1 { public_key: Secp256r1PublicKey },
        ZkLogin(ZkLoginPublicIdentifier),
        Passkey { public_key: PasskeyPublicKey },
        MlDsa65 { public_key: Box<MlDsa65PublicKey> },
    }

    impl Serialize for MultisigMemberPublicKey {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self {
                    MultisigMemberPublicKey::Ed25519(public_key) => {
                        ReadableMemberPublicKey::Ed25519 {
                            public_key: *public_key,
                        }
                    }
                    MultisigMemberPublicKey::Secp256k1(public_key) => {
                        ReadableMemberPublicKey::Secp256k1 {
                            public_key: *public_key,
                        }
                    }
                    MultisigMemberPublicKey::Secp256r1(public_key) => {
                        ReadableMemberPublicKey::Secp256r1 {
                            public_key: *public_key,
                        }
                    }
                    MultisigMemberPublicKey::ZkLogin(public_id) => {
                        ReadableMemberPublicKey::ZkLogin(public_id.clone())
                    }
                    MultisigMemberPublicKey::Passkey(public_key) => {
                        ReadableMemberPublicKey::Passkey {
                            public_key: *public_key,
                        }
                    }
                    MultisigMemberPublicKey::MlDsa65(public_key) => {
                        ReadableMemberPublicKey::MlDsa65 {
                            public_key: public_key.clone(),
                        }
                    }
                };
                readable.serialize(serializer)
            } else {
                let binary = match self {
                    MultisigMemberPublicKey::Ed25519(public_key) => {
                        MemberPublicKey::Ed25519(*public_key)
                    }
                    MultisigMemberPublicKey::Secp256k1(public_key) => {
                        MemberPublicKey::Secp256k1(*public_key)
                    }
                    MultisigMemberPublicKey::Secp256r1(public_key) => {
                        MemberPublicKey::Secp256r1(*public_key)
                    }
                    MultisigMemberPublicKey::ZkLogin(public_id) => {
                        MemberPublicKey::ZkLogin(public_id.clone())
                    }
                    MultisigMemberPublicKey::Passkey(public_key) => {
                        MemberPublicKey::Passkey(*public_key)
                    }
                    MultisigMemberPublicKey::MlDsa65(public_key) => {
                        MemberPublicKey::MlDsa65(public_key.clone())
                    }
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for MultisigMemberPublicKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let readable = ReadableMemberPublicKey::deserialize(deserializer)?;
                Ok(match readable {
                    ReadableMemberPublicKey::Ed25519 { public_key } => Self::Ed25519(public_key),
                    ReadableMemberPublicKey::Secp256k1 { public_key } => {
                        Self::Secp256k1(public_key)
                    }
                    ReadableMemberPublicKey::Secp256r1 { public_key } => {
                        Self::Secp256r1(public_key)
                    }
                    ReadableMemberPublicKey::ZkLogin(public_id) => Self::ZkLogin(public_id),
                    ReadableMemberPublicKey::Passkey { public_key } => Self::Passkey(public_key),
                    ReadableMemberPublicKey::MlDsa65 { public_key } => Self::MlDsa65(public_key),
                })
            } else {
                let binary = MemberPublicKey::deserialize(deserializer)?;
                Ok(match binary {
                    MemberPublicKey::Ed25519(public_key) => Self::Ed25519(public_key),
                    MemberPublicKey::Secp256k1(public_key) => Self::Secp256k1(public_key),
                    MemberPublicKey::Secp256r1(public_key) => Self::Secp256r1(public_key),
                    MemberPublicKey::ZkLogin(public_id) => Self::ZkLogin(public_id),
                    MemberPublicKey::Passkey(public_key) => Self::Passkey(public_key),
                    MemberPublicKey::MlDsa65(public_key) => Self::MlDsa65(public_key),
                })
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum MemberSignature {
        Ed25519(Ed25519Signature),
        Secp256k1(Secp256k1Signature),
        Secp256r1(Secp256r1Signature),
        ZkLogin(Box<ZkLoginAuthenticator>),
        Passkey(PasskeyAuthenticator),
        MlDsa65(Box<MlDsa65Signature>),
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "scheme", rename_all = "lowercase")]
    #[serde(rename = "MultisigMemberSignature")]
    enum ReadableMemberSignature {
        Ed25519 { signature: Ed25519Signature },
        Secp256k1 { signature: Secp256k1Signature },
        Secp256r1 { signature: Secp256r1Signature },
        ZkLogin(Box<ZkLoginAuthenticator>),
        Passkey(PasskeyAuthenticator),
        MlDsa65 { signature: Box<MlDsa65Signature> },
    }

    impl Serialize for MultisigMemberSignature {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self {
                    MultisigMemberSignature::Ed25519(signature) => {
                        ReadableMemberSignature::Ed25519 {
                            signature: *signature,
                        }
                    }
                    MultisigMemberSignature::Secp256k1(signature) => {
                        ReadableMemberSignature::Secp256k1 {
                            signature: *signature,
                        }
                    }
                    MultisigMemberSignature::Secp256r1(signature) => {
                        ReadableMemberSignature::Secp256r1 {
                            signature: *signature,
                        }
                    }
                    MultisigMemberSignature::ZkLogin(authenticator) => {
                        ReadableMemberSignature::ZkLogin(authenticator.clone())
                    }
                    MultisigMemberSignature::Passkey(authenticator) => {
                        ReadableMemberSignature::Passkey(authenticator.clone())
                    }
                    MultisigMemberSignature::MlDsa65(signature) => {
                        ReadableMemberSignature::MlDsa65 {
                            signature: signature.clone(),
                        }
                    }
                };
                readable.serialize(serializer)
            } else {
                let binary = match self {
                    MultisigMemberSignature::Ed25519(signature) => {
                        MemberSignature::Ed25519(*signature)
                    }
                    MultisigMemberSignature::Secp256k1(signature) => {
                        MemberSignature::Secp256k1(*signature)
                    }
                    MultisigMemberSignature::Secp256r1(signature) => {
                        MemberSignature::Secp256r1(*signature)
                    }
                    MultisigMemberSignature::ZkLogin(authenticator) => {
                        MemberSignature::ZkLogin(authenticator.clone())
                    }
                    MultisigMemberSignature::Passkey(authenticator) => {
                        MemberSignature::Passkey(authenticator.clone())
                    }
                    MultisigMemberSignature::MlDsa65(signature) => {
                        MemberSignature::MlDsa65(signature.clone())
                    }
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for MultisigMemberSignature {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let readable = ReadableMemberSignature::deserialize(deserializer)?;
                Ok(match readable {
                    ReadableMemberSignature::Ed25519 { signature } => Self::Ed25519(signature),
                    ReadableMemberSignature::Secp256k1 { signature } => Self::Secp256k1(signature),
                    ReadableMemberSignature::Secp256r1 { signature } => Self::Secp256r1(signature),
                    ReadableMemberSignature::ZkLogin(authenticator) => Self::ZkLogin(authenticator),
                    ReadableMemberSignature::Passkey(authenticator) => Self::Passkey(authenticator),
                    ReadableMemberSignature::MlDsa65 { signature } => Self::MlDsa65(signature),
                })
            } else {
                let binary = MemberSignature::deserialize(deserializer)?;
                Ok(match binary {
                    MemberSignature::Ed25519(signature) => Self::Ed25519(signature),
                    MemberSignature::Secp256k1(signature) => Self::Secp256k1(signature),
                    MemberSignature::Secp256r1(signature) => Self::Secp256r1(signature),
                    MemberSignature::ZkLogin(authenticator) => Self::ZkLogin(authenticator),
                    MemberSignature::Passkey(authenticator) => Self::Passkey(authenticator),
                    MemberSignature::MlDsa65(signature) => Self::MlDsa65(signature),
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    // Regression test: `legacy_bitmap` used to be excluded from
    // `PartialEq`, so two signatures whose `to_bytes` output differed
    // (because the legacy form is preferred when present) could compare
    // equal. Equality must now imply byte equality.
    #[test]
    fn partial_eq_includes_legacy_bitmap() {
        let committee = MultisigCommittee::new(Vec::new(), 0);
        let a = MultisigAggregatedSignature::new(committee.clone(), Vec::new(), 0);
        let mut b = MultisigAggregatedSignature::new(committee, Vec::new(), 0);
        assert_eq!(a, b);

        b.try_with_legacy_bitmap(crate::Bitmap::new()).unwrap();
        assert_ne!(a, b);
    }

    fn ed25519_member(byte: u8) -> MultisigMember {
        MultisigMember::new(
            MultisigMemberPublicKey::Ed25519(Ed25519PublicKey::new(
                [byte; Ed25519PublicKey::LENGTH],
            )),
            1,
        )
    }

    #[test]
    fn try_with_legacy_bitmap_rejects_mismatched_bitmap() {
        let committee = MultisigCommittee::new(vec![ed25519_member(1), ed25519_member(2)], 1);
        let mut signature = MultisigAggregatedSignature::new(committee, Vec::new(), 0b10);

        let err = signature
            .try_with_legacy_bitmap([0].into_iter().collect())
            .unwrap_err();
        assert_eq!(err.to_string(), "bitmap does not match legacy_bitmap");

        let err = signature
            .try_with_legacy_bitmap([MAX_COMMITTEE_SIZE as u32].into_iter().collect())
            .unwrap_err();
        assert_eq!(err.to_string(), "invalid bitmap");

        assert_eq!(signature.legacy_bitmap(), None);

        let legacy_bitmap: crate::Bitmap = [1].into_iter().collect();
        signature
            .try_with_legacy_bitmap(legacy_bitmap.clone())
            .unwrap();
        assert_eq!(signature.legacy_bitmap(), Some(&legacy_bitmap));
    }

    // Regression test: attaching a legacy bitmap to a committee with a
    // member that has no legacy encoding made `to_bytes()` panic.
    #[test]
    fn try_with_legacy_bitmap_rejects_unrepresentable_members() {
        let zklogin = MultisigMemberPublicKey::ZkLogin(
            ZkLoginPublicIdentifier::new(
                "https://accounts.google.com".to_owned(),
                "7".parse().unwrap(),
            )
            .unwrap(),
        );
        let passkey = MultisigMemberPublicKey::Passkey(PasskeyPublicKey::new(
            Secp256r1PublicKey::new([2; Secp256r1PublicKey::LENGTH]),
        ));
        let mldsa65 = MultisigMemberPublicKey::MlDsa65(Box::new(MlDsa65PublicKey::new(
            [0; MlDsa65PublicKey::LENGTH],
        )));

        for (public_key, scheme) in [
            (zklogin, "zklogin"),
            (passkey, "passkey"),
            (mldsa65, "mldsa65"),
        ] {
            let committee = MultisigCommittee::new(
                vec![ed25519_member(1), MultisigMember::new(public_key, 1)],
                1,
            );
            let mut signature = MultisigAggregatedSignature::new(committee, Vec::new(), 0b1);

            let err = signature
                .try_with_legacy_bitmap([0].into_iter().collect())
                .unwrap_err();
            assert_eq!(
                err.to_string(),
                format!("{scheme} member is not representable in legacy multisig")
            );
            assert_eq!(signature.legacy_bitmap(), None);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn try_with_legacy_bitmap_round_trips_through_bcs() {
        let committee = MultisigCommittee::new(vec![ed25519_member(1), ed25519_member(2)], 1);
        let mut signature = MultisigAggregatedSignature::new(
            committee,
            vec![MultisigMemberSignature::Ed25519(Ed25519Signature::new(
                [3; Ed25519Signature::LENGTH],
            ))],
            0b10,
        );
        signature
            .try_with_legacy_bitmap([1].into_iter().collect())
            .unwrap();

        let signature = crate::UserSignature::Multisig(signature);
        let bytes = signature.to_bytes();
        assert_eq!(crate::UserSignature::from_bytes(&bytes).unwrap(), signature);
    }

    // Regression test: the JSON deserializer used to copy `bitmap` and
    // `legacy_bitmap` straight onto the value without checking that
    // they encoded the same signer set, letting a single payload carry
    // one signer set observed by `bitmap()` and a different one emitted
    // by `to_bytes()` (which prefers the legacy form when present). The
    // BCS legacy branch always derives `bitmap` from `legacy_bitmap`, so
    // the JSON path must reject inputs where those two fields disagree.
    #[cfg(feature = "serde")]
    #[test]
    fn json_dual_bitmap_must_be_consistent() {
        let mut roaring = crate::Bitmap::new();
        roaring.insert(5);
        let legacy_b64 = {
            use base64ct::Encoding;
            let mut buf = Vec::new();
            roaring.serialize_into(&mut buf).unwrap();
            base64ct::Base64::encode_string(&buf)
        };

        // `bitmap` claims signer 0, `legacy_bitmap` claims signer 5.
        let inconsistent = format!(
            r#"{{"signatures":[],"bitmap":1,"legacy_bitmap":"{legacy_b64}",
                "committee":{{"members":[],"threshold":0}}}}"#
        );
        let err = serde_json::from_str::<MultisigAggregatedSignature>(&inconsistent)
            .expect_err("inconsistent dual bitmap must be rejected");
        assert!(
            err.to_string().contains("legacy_bitmap"),
            "unexpected error: {err}"
        );

        // The canonical form (bitmap derived from legacy_bitmap) is
        // accepted.
        let consistent = format!(
            r#"{{"signatures":[],"bitmap":{},"legacy_bitmap":"{legacy_b64}",
                "committee":{{"members":[],"threshold":0}}}}"#,
            1u16 << 5,
        );
        serde_json::from_str::<MultisigAggregatedSignature>(&consistent)
            .expect("consistent dual bitmap must be accepted");
    }

    // Regression test: `to_bytes()` used to panic via
    // `.expect("serialization cannot fail")` when `legacy_bitmap` was
    // present alongside a ZkLogin or Passkey committee member, because
    // the legacy member encoding (`Base64MultisigMemberPublicKey`)
    // explicitly returns `Err` for those variants. The JSON
    // deserializer must reject the combination so an attacker cannot
    // craft a payload that crashes a consumer on its first `to_bytes()`.
    #[cfg(feature = "serde")]
    #[test]
    fn json_legacy_bitmap_with_zklogin_member_is_rejected() {
        let legacy_b64 = {
            use base64ct::Encoding;
            let mut buf = Vec::new();
            crate::Bitmap::new().serialize_into(&mut buf).unwrap();
            base64ct::Base64::encode_string(&buf)
        };

        let payload = format!(
            r#"{{
                "signatures":[],
                "bitmap":0,
                "legacy_bitmap":"{legacy_b64}",
                "committee":{{
                    "members":[{{
                        "public_key":{{
                            "scheme":"zklogin",
                            "iss":"https://accounts.google.com",
                            "address_seed":"7"
                        }},
                        "weight":1
                    }}],
                    "threshold":1
                }}
            }}"#
        );
        let err = serde_json::from_str::<MultisigAggregatedSignature>(&payload)
            .expect_err("zklogin member with legacy bitmap must be rejected");
        assert!(
            err.to_string().contains("zklogin"),
            "unexpected error: {err}"
        );
    }
}
