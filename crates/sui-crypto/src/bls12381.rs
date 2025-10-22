use std::collections::HashMap;

use crate::SignatureError;
use crate::Signer;
use crate::Verifier;
use blst::min_sig::AggregatePublicKey;
use blst::min_sig::AggregateSignature;
use blst::min_sig::PublicKey;
use blst::min_sig::SecretKey;
use blst::min_sig::Signature;
use sui_sdk_types::Bls12381PublicKey;
use sui_sdk_types::Bls12381Signature;
use sui_sdk_types::SignatureScheme;
use sui_sdk_types::ValidatorAggregatedSignature;
use sui_sdk_types::ValidatorCommittee;
use sui_sdk_types::ValidatorSignature;

const DST_G1: &[u8] = b"BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_NUL_";

#[derive(Debug)]
#[allow(unused)]
struct BlstError(blst::BLST_ERROR);

impl std::fmt::Display for BlstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for BlstError {}

pub struct Bls12381PrivateKey(SecretKey);

impl std::fmt::Debug for Bls12381PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bls12381PrivateKey")
            .field(&"__elided__")
            .finish()
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Bls12381PrivateKey {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        proptest::arbitrary::any::<[u8; Self::LENGTH]>()
            .prop_map(|bytes| {
                let secret_key = SecretKey::key_gen(&bytes, &[]).unwrap();
                Self(secret_key)
            })
            .boxed()
    }
}

impl Bls12381PrivateKey {
    /// The length of an bls12381 private key in bytes.
    pub const LENGTH: usize = 32;

    pub fn new(bytes: [u8; Self::LENGTH]) -> Result<Self, SignatureError> {
        SecretKey::from_bytes(&bytes)
            .map_err(BlstError)
            .map_err(SignatureError::from_source)
            .map(Self)
    }

    pub fn scheme(&self) -> SignatureScheme {
        SignatureScheme::Bls12381
    }

    pub fn verifying_key(&self) -> Bls12381VerifyingKey {
        let verifying_key = self.0.sk_to_pk();
        Bls12381VerifyingKey(verifying_key)
    }

    pub fn public_key(&self) -> Bls12381PublicKey {
        self.verifying_key().public_key()
    }

    pub fn generate<R>(mut rng: R) -> Self
    where
        R: rand_core::RngCore + rand_core::CryptoRng,
    {
        let mut buf: [u8; Self::LENGTH] = [0; Self::LENGTH];
        rng.fill_bytes(&mut buf);
        let secret_key = SecretKey::key_gen(&buf, &[]).unwrap();
        Self(secret_key)
    }

    pub fn sign_checkpoint_summary(
        &self,
        summary: &sui_sdk_types::CheckpointSummary,
    ) -> ValidatorSignature {
        let message = summary.signing_message();
        let signature = self.sign(&message);
        ValidatorSignature {
            epoch: summary.epoch,
            public_key: self.public_key(),
            signature,
        }
    }
}

impl Signer<Bls12381Signature> for Bls12381PrivateKey {
    fn try_sign(&self, msg: &[u8]) -> Result<Bls12381Signature, SignatureError> {
        let signature = self.0.sign(msg, DST_G1, &[]);
        Ok(Bls12381Signature::new(signature.to_bytes()))
    }
}

#[derive(Debug)]
pub struct Bls12381VerifyingKey(PublicKey);

impl Bls12381VerifyingKey {
    pub fn new(public_key: &Bls12381PublicKey) -> Result<Self, SignatureError> {
        PublicKey::key_validate(public_key.inner())
            .map(Self)
            .map_err(BlstError)
            .map_err(SignatureError::from_source)
    }

    pub fn public_key(&self) -> Bls12381PublicKey {
        Bls12381PublicKey::new(self.0.to_bytes())
    }
}

impl Verifier<Bls12381Signature> for Bls12381VerifyingKey {
    fn verify(&self, message: &[u8], signature: &Bls12381Signature) -> Result<(), SignatureError> {
        let signature = Signature::sig_validate(signature.inner(), true)
            .map_err(BlstError)
            .map_err(SignatureError::from_source)?;

        let err = signature.verify(true, message, DST_G1, &[], &self.0, false);
        if err == blst::BLST_ERROR::BLST_SUCCESS {
            Ok(())
        } else {
            Err(SignatureError::from_source(BlstError(err)))
        }
    }
}

#[derive(Debug)]
struct ExtendedValidatorCommittee {
    committee: ValidatorCommittee,
    verifying_keys: Vec<Bls12381VerifyingKey>,
    public_key_to_index: HashMap<Bls12381PublicKey, usize>,
    total_weight: u64,
    quorum_threshold: u64,
}

struct MemberInfo<'a> {
    verifying_key: &'a Bls12381VerifyingKey,
    weight: u64,
    index: usize,
}

impl ExtendedValidatorCommittee {
    fn new(committee: ValidatorCommittee) -> Result<Self, SignatureError> {
        let mut public_key_to_index = HashMap::new();
        let mut verifying_keys = Vec::new();

        let mut total_weight = 0;
        for (idx, member) in committee.members.iter().enumerate() {
            assert_eq!(idx, verifying_keys.len());
            verifying_keys.push(Bls12381VerifyingKey::new(&member.public_key)?);
            public_key_to_index.insert(member.public_key, idx);
            total_weight += member.stake;
        }

        let quorum_threshold = ((total_weight - 1) / 3) * 2 + 1;

        Ok(Self {
            committee,
            verifying_keys,
            public_key_to_index,
            total_weight,
            quorum_threshold,
        })
    }

    fn committee(&self) -> &ValidatorCommittee {
        &self.committee
    }

    #[allow(unused)]
    fn total_weight(&self) -> u64 {
        self.total_weight
    }

    #[allow(unused)]
    fn quorum_threshold(&self) -> u64 {
        self.quorum_threshold
    }

    fn verifying_key(
        &self,
        public_key: &Bls12381PublicKey,
    ) -> Result<&Bls12381VerifyingKey, SignatureError> {
        self.public_key_to_index
            .get(public_key)
            .and_then(|idx| self.verifying_keys.get(*idx))
            .ok_or_else(|| {
                SignatureError::from_source(format!(
                    "signature from public_key {public_key} does not belong to this committee",
                ))
            })
    }

    fn member(&self, public_key: &Bls12381PublicKey) -> Result<MemberInfo<'_>, SignatureError> {
        self.public_key_to_index
            .get(public_key)
            .ok_or_else(|| {
                SignatureError::from_source(format!(
                    "signature from public_key {public_key} does not belong to this committee",
                ))
            })
            .and_then(|idx| self.member_by_idx(*idx))
    }

    fn member_by_idx(&self, idx: usize) -> Result<MemberInfo<'_>, SignatureError> {
        let verifying_key = self.verifying_keys.get(idx).ok_or_else(|| {
            SignatureError::from_source(format!(
                "index {idx} out of bounds; committee has {} members",
                self.committee().members.len(),
            ))
        })?;
        let weight = self
            .committee()
            .members
            .get(idx)
            .ok_or_else(|| {
                SignatureError::from_source(format!(
                    "index {idx} out of bounds; committee has {} members",
                    self.committee().members.len(),
                ))
            })?
            .stake;

        Ok(MemberInfo {
            verifying_key,
            weight,
            index: idx,
        })
    }
}

#[derive(Debug)]
pub struct ValidatorCommitteeSignatureVerifier {
    committee: ExtendedValidatorCommittee,
}

impl ValidatorCommitteeSignatureVerifier {
    pub fn new(committee: ValidatorCommittee) -> Result<Self, SignatureError> {
        ExtendedValidatorCommittee::new(committee).map(|committee| Self { committee })
    }

    pub fn committee(&self) -> &ValidatorCommittee {
        self.committee.committee()
    }

    pub fn verify_checkpoint_summary(
        &self,
        summary: &sui_sdk_types::CheckpointSummary,
        signature: &ValidatorAggregatedSignature,
    ) -> Result<(), SignatureError> {
        let message = summary.signing_message();
        self.verify(&message, signature)
    }
}

impl Verifier<ValidatorSignature> for ValidatorCommitteeSignatureVerifier {
    fn verify(&self, message: &[u8], signature: &ValidatorSignature) -> Result<(), SignatureError> {
        if signature.epoch != self.committee().epoch {
            return Err(SignatureError::from_source(format!(
                "signature epoch {} does not match committee epoch {}",
                signature.epoch,
                self.committee().epoch
            )));
        }

        let verifying_key = self.committee.verifying_key(&signature.public_key)?;
        verifying_key.verify(message, &signature.signature)
    }
}

impl Verifier<ValidatorAggregatedSignature> for ValidatorCommitteeSignatureVerifier {
    fn verify(
        &self,
        message: &[u8],
        signature: &ValidatorAggregatedSignature,
    ) -> Result<(), SignatureError> {
        if signature.epoch != self.committee().epoch {
            return Err(SignatureError::from_source(format!(
                "signature epoch {} does not match committee epoch {}",
                signature.epoch,
                self.committee().epoch
            )));
        }

        let mut signed_weight = 0;
        let mut bitmap = signature.bitmap.iter();

        let mut aggregated_public_key = {
            let idx = bitmap.next().ok_or_else(|| {
                SignatureError::from_source("signature bitmap must have at least one entry")
            })?;

            let member = self.committee.member_by_idx(idx as usize)?;

            signed_weight += member.weight;
            AggregatePublicKey::from_public_key(&member.verifying_key.0)
        };

        for idx in bitmap {
            let member = self.committee.member_by_idx(idx as usize)?;

            signed_weight += member.weight;
            aggregated_public_key
                .add_public_key(&member.verifying_key.0, false) // Keys are already verified
                .map_err(BlstError)
                .map_err(SignatureError::from_source)?;
        }

        Bls12381VerifyingKey(aggregated_public_key.to_public_key())
            .verify(message, &signature.signature)?;

        if signed_weight >= self.committee.quorum_threshold {
            Ok(())
        } else {
            Err(SignatureError::from_source(format!(
                "insufficient signing weight {}; quorum threshold is {}",
                signed_weight, self.committee.quorum_threshold,
            )))
        }
    }
}

#[derive(Debug)]
pub struct ValidatorCommitteeSignatureAggregator {
    verifier: ValidatorCommitteeSignatureVerifier,
    signatures: std::collections::BTreeMap<usize, ValidatorSignature>,
    signed_weight: u64,
    message: Vec<u8>,
}

impl ValidatorCommitteeSignatureAggregator {
    pub fn new_checkpoint_summary(
        committee: ValidatorCommittee,
        summary: &sui_sdk_types::CheckpointSummary,
    ) -> Result<Self, SignatureError> {
        let verifier = ValidatorCommitteeSignatureVerifier::new(committee)?;
        let message = summary.signing_message();

        Ok(Self {
            verifier,
            signatures: Default::default(),
            signed_weight: 0,
            message,
        })
    }

    pub fn committee(&self) -> &ValidatorCommittee {
        self.verifier.committee()
    }

    pub fn add_signature(&mut self, signature: ValidatorSignature) -> Result<(), SignatureError> {
        use std::collections::btree_map::Entry;

        if signature.epoch != self.verifier.committee().epoch {
            return Err(SignatureError::from_source(format!(
                "signature epoch {} does not match committee epoch {}",
                signature.epoch,
                self.committee().epoch
            )));
        }

        let member = self.verifier.committee.member(&signature.public_key)?;

        member
            .verifying_key
            .verify(&self.message, &signature.signature)?;

        match self.signatures.entry(member.index) {
            Entry::Vacant(v) => {
                v.insert(signature);
            }
            Entry::Occupied(_) => {
                return Err(SignatureError::from_source(
                    "duplicate signature from same committee member",
                ));
            }
        }

        self.signed_weight += member.weight;

        Ok(())
    }

    pub fn finish(&self) -> Result<ValidatorAggregatedSignature, SignatureError> {
        if self.signed_weight < self.verifier.committee.quorum_threshold {
            return Err(SignatureError::from_source(format!(
                "signature weight of {} is insufficient to reach quorum threshold of {}",
                self.signed_weight, self.verifier.committee.quorum_threshold
            )));
        }

        let mut iter = self.signatures.iter();
        let (member_idx, signature) = iter.next().ok_or_else(|| {
            SignatureError::from_source("signature map must have at least one entry")
        })?;

        let mut bitmap = sui_sdk_types::Bitmap::new();
        bitmap.insert(*member_idx as u32);
        let agg_sig = AggregateSignature::from_signature(
            &Signature::from_bytes(signature.signature.inner())
                .expect("signature was already verified"),
        );

        let (agg_sig, bitmap) = iter.fold(
            (agg_sig, bitmap),
            |(mut agg_sig, mut bitmap), (member_idx, signature)| {
                bitmap.insert(*member_idx as u32);
                agg_sig
                    .add_signature(
                        &Signature::from_bytes(signature.signature.inner())
                            .expect("signature was already verified"),
                        false,
                    )
                    .expect("signature was already verified");
                (agg_sig, bitmap)
            },
        );

        let aggregated_signature = ValidatorAggregatedSignature {
            epoch: self.verifier.committee().epoch,
            signature: Bls12381Signature::new(agg_sig.to_signature().to_bytes()),
            bitmap,
        };

        // Double check that the aggregated sig still verifies
        self.verifier.verify(&self.message, &aggregated_signature)?;

        Ok(aggregated_signature)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sui_sdk_types::CheckpointSummary;
    use sui_sdk_types::ValidatorCommittee;
    use sui_sdk_types::ValidatorCommitteeMember;
    use test_strategy::proptest;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[proptest]
    fn basic_signing(signer: Bls12381PrivateKey, message: Vec<u8>) {
        let signature = signer.sign(&message);
        signer.verifying_key().verify(&message, &signature).unwrap();
    }

    #[proptest]
    fn basic_aggregation(private_keys: [Bls12381PrivateKey; 4], summary: CheckpointSummary) {
        let committee = ValidatorCommittee {
            epoch: summary.epoch,
            members: private_keys
                .iter()
                .map(|key| ValidatorCommitteeMember {
                    public_key: key.public_key(),
                    stake: 1,
                })
                .collect(),
        };

        let mut aggregator =
            ValidatorCommitteeSignatureAggregator::new_checkpoint_summary(committee, &summary)
                .unwrap();

        // Aggregating with no sigs fails
        aggregator.finish().unwrap_err();

        aggregator
            .add_signature(private_keys[0].sign_checkpoint_summary(&summary))
            .unwrap();

        // Aggregating with a sig from the same committee member more than once fails
        aggregator
            .add_signature(private_keys[0].sign_checkpoint_summary(&summary))
            .unwrap_err();

        // Aggregating with insufficient weight fails
        aggregator.finish().unwrap_err();

        aggregator
            .add_signature(private_keys[1].sign_checkpoint_summary(&summary))
            .unwrap();
        aggregator
            .add_signature(private_keys[2].sign_checkpoint_summary(&summary))
            .unwrap();

        // Aggregating with sufficient weight succeeds and verifies
        let signature = aggregator.finish().unwrap();
        aggregator
            .verifier
            .verify_checkpoint_summary(&summary, &signature)
            .unwrap();

        // We can add the last sig and still be successful
        aggregator
            .add_signature(private_keys[3].sign_checkpoint_summary(&summary))
            .unwrap();
        let signature = aggregator.finish().unwrap();
        aggregator
            .verifier
            .verify_checkpoint_summary(&summary, &signature)
            .unwrap();
    }
}
