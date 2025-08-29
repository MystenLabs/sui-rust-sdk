use super::Bls12381PublicKey;
use super::Bls12381Signature;
use crate::checkpoint::EpochId;
use crate::checkpoint::StakeUnit;

/// The Validator Set for a particular epoch.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// validator-committee = u64 ; epoch
///                       (vector validator-committee-member)
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ValidatorCommittee {
    pub epoch: EpochId,
    pub members: Vec<ValidatorCommitteeMember>,
}

/// A member of a Validator Committee
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// validator-committee-member = bls-public-key
///                              u64 ; stake
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ValidatorCommitteeMember {
    pub public_key: Bls12381PublicKey,
    pub stake: StakeUnit,
}

/// An aggregated signature from multiple Validators.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// validator-aggregated-signature = u64               ; epoch
///                                  bls-signature
///                                  roaring-bitmap
/// roaring-bitmap = bytes  ; where the contents of the bytes are valid
///                         ; according to the serialized spec for
///                         ; roaring bitmaps
/// ```
///
/// See [here](https://github.com/RoaringBitmap/RoaringFormatSpec) for the specification for the
/// serialized format of RoaringBitmaps.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ValidatorAggregatedSignature {
    pub epoch: EpochId,
    pub signature: Bls12381Signature,
    pub bitmap: crate::Bitmap,
}

/// A signature from a Validator
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// validator-signature = u64               ; epoch
///                       bls-public-key
///                       bls-signature
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct ValidatorSignature {
    pub epoch: EpochId,
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
