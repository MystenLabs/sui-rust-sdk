use super::Bls12381PublicKey;
use super::Bls12381Signature;
use crate::checkpoint::EpochId;
use crate::checkpoint::StakeUnit;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ValidatorCommittee {
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub epoch: EpochId,
    pub members: Vec<ValidatorCommitteeMember>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ValidatorCommitteeMember {
    #[cfg_attr(feature = "serde", serde(with = "ValidatorPublicKeySerialization"))]
    pub public_key: Bls12381PublicKey,
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub stake: StakeUnit,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ValidatorAggregatedSignature {
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub epoch: EpochId,
    pub signature: Bls12381Signature,
    #[cfg_attr(feature = "serde", serde(with = "RoaringBitMapSerialization"))]
    #[cfg_attr(
        feature = "proptest",
        strategy(proptest::strategy::Just(roaring::RoaringBitmap::default()))
    )]
    pub bitmap: roaring::RoaringBitmap,
}

#[cfg(feature = "serde")]
type RoaringBitMapSerialization = ::serde_with::As<
    ::serde_with::IfIsHumanReadable<
        crate::_serde::Base64RoaringBitmap,
        crate::_serde::BinaryRoaringBitmap,
    >,
>;

// Similar to Digest...unfortunately validator's public key material is serialized with the length
// (96) prefixed
#[cfg(feature = "serde")]
type ValidatorPublicKeySerialization = ::serde_with::As<
    ::serde_with::IfIsHumanReadable<::serde_with::DisplayFromStr, BinaryValidatorPublicKey>,
>;

#[cfg(feature = "serde")]
struct BinaryValidatorPublicKey;

#[cfg(feature = "serde")]
impl serde_with::SerializeAs<Bls12381PublicKey> for BinaryValidatorPublicKey {
    fn serialize_as<S>(source: &Bls12381PublicKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ::serde_with::Bytes::serialize_as(source.inner(), serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde_with::DeserializeAs<'de, Bls12381PublicKey> for BinaryValidatorPublicKey {
    fn deserialize_as<D>(deserializer: D) -> Result<Bls12381PublicKey, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: [u8; Bls12381PublicKey::LENGTH] =
            ::serde_with::Bytes::deserialize_as(deserializer)?;
        Ok(Bls12381PublicKey::new(bytes))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ValidatorSignature {
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub epoch: EpochId,
    #[cfg_attr(feature = "serde", serde(with = "ValidatorPublicKeySerialization"))]
    pub public_key: Bls12381PublicKey,
    pub signature: Bls12381Signature,
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[cfg(feature = "serde")]
    #[test]
    fn aggregated_signature_fixture() {
        use base64ct::Base64;
        use base64ct::Encoding;

        const FIXTURE: &str = "CgAAAAAAAACZrBcXiqa0ttztfwrBxKzQRzIRnZhbmsQV7tqNXwiZQrRC+dVDbdua1Ety9uy2pCUSOjAAAAEAAAAAAAAAEAAAAAAA";
        let bcs = Base64::decode_vec(FIXTURE).unwrap();

        let signature: ValidatorAggregatedSignature = bcs::from_bytes(&bcs).unwrap();
        let bytes = bcs::to_bytes(&signature).unwrap();
        assert_eq!(bcs, bytes);
    }
}
