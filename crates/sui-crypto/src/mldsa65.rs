//! ML-DSA-65 (FIPS 204) signing and verification.
//!
//! Pure ML-DSA-65 with an empty context over the 32-byte signing digest,
//! hedged with 32 bytes of caller-supplied randomness.
//!
//! Caller-supplied randomness keeps `getrandom` out of the crate and wasm
//! builds working. [`SuiRandomizedSigner`](crate::SuiRandomizedSigner) is
//! the randomized counterpart of [`SuiSigner`](crate::SuiSigner).

use crate::SignatureError;
use mysten_mldsa_native_rs as mldsa;
use signature::RandomizedSigner;
use signature::Verifier;
use sui_sdk_types::MlDsa65PublicKey;
use sui_sdk_types::MlDsa65Signature;
use sui_sdk_types::SignatureScheme;
use sui_sdk_types::SimpleSignature;
use sui_sdk_types::UserSignature;

/// The 32-byte FIPS 204 seed plus the key expanded from it.
///
/// Only the seed is serialized (keystore, `suiprivkey`); the expanded key is
/// kept so signing does not re-expand. Both zeroize on drop.
pub struct MlDsa65PrivateKey {
    seed: mldsa::SigningKeySeed,
    signing_key: mldsa::SigningKey,
    // Boxed to keep the enums carrying this type small.
    verifying_key: Box<mldsa::VerifyingKey>,
}

impl Clone for MlDsa65PrivateKey {
    fn clone(&self) -> Self {
        Self::new(*self.seed.as_bytes())
    }
}

impl std::fmt::Debug for MlDsa65PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MlDsa65PrivateKey")
            .field(&"__elided__")
            .finish()
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for MlDsa65PrivateKey {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        proptest::arbitrary::any::<[u8; Self::LENGTH]>()
            .prop_map(Self::new)
            .boxed()
    }
}

impl MlDsa65PrivateKey {
    /// The length of the seed in bytes.
    pub const LENGTH: usize = mldsa::SEED_LENGTH;

    pub fn new(seed: [u8; Self::LENGTH]) -> Self {
        let seed = mldsa::SigningKeySeed::from(seed);
        let (signing_key, verifying_key) = seed.expand();
        Self {
            seed,
            signing_key,
            verifying_key: Box::new(verifying_key),
        }
    }

    pub fn scheme(&self) -> SignatureScheme {
        SignatureScheme::MlDsa65
    }

    pub fn verifying_key(&self) -> MlDsa65VerifyingKey {
        MlDsa65VerifyingKey {
            key: self.verifying_key.clone(),
        }
    }

    pub fn public_key(&self) -> MlDsa65PublicKey {
        MlDsa65PublicKey::new(*self.verifying_key.as_bytes())
    }

    pub fn seed(&self) -> &[u8; Self::LENGTH] {
        self.seed.as_bytes()
    }

    pub fn generate<R>(mut rng: R) -> Self
    where
        R: rand_core::RngCore + rand_core::CryptoRng,
    {
        let mut seed = [0; Self::LENGTH];
        rng.fill_bytes(&mut seed);
        Self::new(seed)
    }

    fn from_flagged_key_bytes(
        scheme: SignatureScheme,
        key: Vec<u8>,
    ) -> Result<Self, SignatureError> {
        if scheme != SignatureScheme::MlDsa65 {
            return Err(SignatureError::from_source(format!(
                "expected mldsa65 private key, found scheme `{}`",
                scheme.name(),
            )));
        }
        let bytes: [u8; Self::LENGTH] = key.try_into().map_err(|_: Vec<u8>| {
            SignatureError::from_source("private key has invalid length for mldsa65")
        })?;
        Ok(Self::new(bytes))
    }

    #[cfg(feature = "bech32")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "bech32")))]
    /// Decode a Bech32 `suiprivkey` string produced by the Sui CLI; the
    /// payload is `0x08 || seed`.
    pub fn from_suiprivkey(s: &str) -> Result<Self, SignatureError> {
        let (scheme, key) = crate::suipriv::decode(s)?;
        Self::from_flagged_key_bytes(scheme, key)
    }

    #[cfg(feature = "bech32")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "bech32")))]
    /// Encode this private key as a Bech32 `suiprivkey` string.
    pub fn to_suiprivkey(&self) -> Result<String, SignatureError> {
        crate::suipriv::encode(SignatureScheme::MlDsa65, self.seed())
    }

    /// Decode a Base64 `flag || seed` string, the legacy keystore format used
    /// for entries of the Sui CLI's `sui.keystore` file.
    pub fn from_base64(s: &str) -> Result<Self, SignatureError> {
        let (scheme, key) = crate::suipriv::decode_base64(s)?;
        Self::from_flagged_key_bytes(scheme, key)
    }

    /// Encode this private key as a Base64 `flag || seed` string, the legacy
    /// keystore format used for entries of the Sui CLI's `sui.keystore` file.
    pub fn to_base64(&self) -> String {
        crate::suipriv::encode_base64(SignatureScheme::MlDsa65, self.seed())
    }
}

impl RandomizedSigner<MlDsa65Signature> for MlDsa65PrivateKey {
    /// Hedged per FIPS 204: 32 bytes from `rng` enter the commitment, so
    /// repeated signatures differ. A weak `rng` only degrades to the
    /// deterministic variant's security.
    fn try_sign_with_rng(
        &self,
        rng: &mut impl rand_core::CryptoRngCore,
        msg: &[u8],
    ) -> Result<MlDsa65Signature, SignatureError> {
        let mut rnd = [0; mldsa::RND_LENGTH];
        rng.fill_bytes(&mut rnd);
        self.signing_key
            .sign(msg, b"", &rnd)
            .map(|signature| MlDsa65Signature::new(*signature.as_bytes()))
            .map_err(SignatureError::from_source)
    }
}

impl RandomizedSigner<SimpleSignature> for MlDsa65PrivateKey {
    fn try_sign_with_rng(
        &self,
        rng: &mut impl rand_core::CryptoRngCore,
        msg: &[u8],
    ) -> Result<SimpleSignature, SignatureError> {
        <Self as RandomizedSigner<MlDsa65Signature>>::try_sign_with_rng(self, rng, msg).map(
            |signature| SimpleSignature::MlDsa65 {
                signature: Box::new(signature),
                public_key: Box::new(self.public_key()),
            },
        )
    }
}

impl RandomizedSigner<UserSignature> for MlDsa65PrivateKey {
    fn try_sign_with_rng(
        &self,
        rng: &mut impl rand_core::CryptoRngCore,
        msg: &[u8],
    ) -> Result<UserSignature, SignatureError> {
        <Self as RandomizedSigner<SimpleSignature>>::try_sign_with_rng(self, rng, msg)
            .map(UserSignature::Simple)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MlDsa65VerifyingKey {
    // Boxed to keep the enums carrying this type small.
    key: Box<mldsa::VerifyingKey>,
}

impl MlDsa65VerifyingKey {
    pub fn new(public_key: &MlDsa65PublicKey) -> Result<Self, SignatureError> {
        mldsa::VerifyingKey::from_bytes(public_key.inner())
            .map(|key| Self { key: Box::new(key) })
            .map_err(SignatureError::from_source)
    }

    pub fn public_key(&self) -> MlDsa65PublicKey {
        MlDsa65PublicKey::new(*self.key.as_bytes())
    }
}

impl Verifier<MlDsa65Signature> for MlDsa65VerifyingKey {
    fn verify(&self, message: &[u8], signature: &MlDsa65Signature) -> Result<(), SignatureError> {
        let signature =
            mldsa::Signature::from_bytes(signature.inner()).map_err(SignatureError::from_source)?;
        self.key
            .verify(message, b"", &signature)
            .map_err(|_| SignatureError::new())
    }
}

impl Verifier<SimpleSignature> for MlDsa65VerifyingKey {
    fn verify(&self, message: &[u8], signature: &SimpleSignature) -> Result<(), SignatureError> {
        let SimpleSignature::MlDsa65 {
            signature,
            public_key,
        } = signature
        else {
            return Err(SignatureError::from_source("not an mldsa65 signature"));
        };
        if public_key.inner() != self.key.as_bytes() {
            return Err(SignatureError::from_source("public_key mismatch"));
        }
        <Self as Verifier<MlDsa65Signature>>::verify(self, message, signature)
    }
}

impl Verifier<UserSignature> for MlDsa65VerifyingKey {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::Simple(signature) = signature else {
            return Err(SignatureError::from_source("not a simple signature"));
        };
        <Self as Verifier<SimpleSignature>>::verify(self, message, signature)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::SuiRandomizedSigner;
    use crate::SuiVerifier;
    use sui_sdk_types::PersonalMessage;
    use test_strategy::proptest;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    /// Produced by Sui's signer (fastcrypto-pq, wrapping mldsa-native) over the
    /// personal message "hello": seed `[2; 32]`, hedging randomness `[7; 32]`.
    const SUI_SIGNATURE: &str = include_str!("fixtures/mldsa65-personal-message-signature");
    const SUI_ADDRESS: &str = "0x687afa13b5510548e8ab9c57b34544c8ade5507559cfb944db0453fae2a68d4c";

    fn sui_fixture() -> (MlDsa65Signature, MlDsa65PublicKey) {
        let UserSignature::Simple(SimpleSignature::MlDsa65 {
            signature,
            public_key,
        }) = UserSignature::from_base64(SUI_SIGNATURE.trim()).unwrap()
        else {
            panic!("expected an mldsa65 signature");
        };
        (*signature, *public_key)
    }

    /// Fixed-stream rng: hedging only needs per-signature variation, and
    /// determinism keeps failures reproducible.
    struct TestRng(u64);

    impl rand_core::RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }

        // SplitMix64.
        fn next_u64(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for chunk in dst.chunks_mut(8) {
                let bytes = self.next_u64().to_le_bytes();
                chunk.copy_from_slice(&bytes[..chunk.len()]);
            }
        }

        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dst);
            Ok(())
        }
    }

    impl rand_core::CryptoRng for TestRng {}

    /// A signature from Sui's signer verifies here under the same address.
    /// The reverse direction is covered by Sui's e2e tests.
    #[test]
    fn verifies_sui_signature() {
        let (sui_signature, sui_public_key) = sui_fixture();
        let key = MlDsa65PrivateKey::new([2; 32]);
        assert_eq!(key.public_key(), sui_public_key);
        assert_eq!(key.public_key().derive_address().to_string(), SUI_ADDRESS);

        let digest = PersonalMessage(b"hello".as_slice().into()).signing_digest();
        key.verifying_key().verify(&digest, &sui_signature).unwrap();
    }

    /// Same key and message, different rng: two distinct valid signatures.
    #[test]
    fn hedging_varies_signatures() {
        let key = MlDsa65PrivateKey::new([3; 32]);
        let digest = PersonalMessage(b"hello".as_slice().into()).signing_digest();
        let first: MlDsa65Signature = key.try_sign_with_rng(&mut TestRng(1), &digest).unwrap();
        let second: MlDsa65Signature = key.try_sign_with_rng(&mut TestRng(2), &digest).unwrap();

        assert_ne!(first, second);
        key.verifying_key().verify(&digest, &first).unwrap();
        key.verifying_key().verify(&digest, &second).unwrap();
    }

    #[test]
    fn personal_message_roundtrip() {
        let key = MlDsa65PrivateKey::new([2; 32]);
        let message = PersonalMessage(b"hello".as_slice().into());
        let signature = key
            .sign_personal_message_with_rng(&mut TestRng(1), &message)
            .unwrap();
        assert_eq!(signature.derive_address().to_string(), SUI_ADDRESS);
        key.verifying_key()
            .verify_personal_message(&message, &signature)
            .unwrap();
        crate::simple::SimpleVerifier
            .verify_personal_message(&message, &signature)
            .unwrap();

        let other = PersonalMessage(b"bye".as_slice().into());
        key.verifying_key()
            .verify_personal_message(&other, &signature)
            .unwrap_err();
    }

    #[test]
    fn tampered_signature_fails() {
        let (sui_signature, sui_public_key) = sui_fixture();
        let digest = PersonalMessage(b"hello".as_slice().into()).signing_digest();
        let mut bytes = *sui_signature.inner();
        bytes[100] ^= 1;
        MlDsa65VerifyingKey::new(&sui_public_key)
            .unwrap()
            .verify(&digest, &MlDsa65Signature::new(bytes))
            .unwrap_err();
    }

    #[cfg(feature = "bech32")]
    #[test]
    fn suiprivkey_matches_sui_keytool() {
        // `sui keytool` encoding of the seed [2; 32], from Sui's keytool tests.
        const SUIPRIVKEY: &str =
            "suiprivkey1pqpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyrndku7";
        let key = MlDsa65PrivateKey::from_suiprivkey(SUIPRIVKEY).unwrap();
        assert_eq!(key.seed(), &[2; 32]);
        assert_eq!(key.public_key().derive_address().to_string(), SUI_ADDRESS);
        assert_eq!(key.to_suiprivkey().unwrap(), SUIPRIVKEY);
        assert_eq!(
            MlDsa65PrivateKey::from_base64(&key.to_base64())
                .unwrap()
                .seed(),
            key.seed()
        );
    }

    #[cfg(feature = "ed25519")]
    #[test]
    fn hybrid_multisig() {
        use crate::SuiSigner;
        use crate::multisig::MultisigAggregator;
        use crate::multisig::UserSignatureVerifier;
        use sui_sdk_types::MultisigCommittee;
        use sui_sdk_types::MultisigMember;
        use sui_sdk_types::MultisigMemberPublicKey;

        let ed_key = crate::ed25519::Ed25519PrivateKey::new([1; 32]);
        let key = MlDsa65PrivateKey::new([2; 32]);
        // Weight 1 each, threshold 2: both schemes must sign.
        let committee = MultisigCommittee::new(
            vec![
                MultisigMember::new(MultisigMemberPublicKey::Ed25519(ed_key.public_key()), 1),
                MultisigMember::new(
                    MultisigMemberPublicKey::MlDsa65(Box::new(key.public_key())),
                    1,
                ),
            ],
            2,
        );
        let message = PersonalMessage(b"hello".as_slice().into());

        let mut aggregator = MultisigAggregator::new_with_message(committee, &message);
        aggregator
            .add_signature(ed_key.sign_personal_message(&message).unwrap())
            .unwrap();
        aggregator.finish().unwrap_err();
        aggregator
            .add_signature(
                key.sign_personal_message_with_rng(&mut TestRng(1), &message)
                    .unwrap(),
            )
            .unwrap();
        let multisig = UserSignature::Multisig(aggregator.finish().unwrap());

        UserSignatureVerifier::new()
            .verify_personal_message(&message, &multisig)
            .unwrap();
        let bytes = multisig.to_bytes();
        assert_eq!(multisig, UserSignature::from_bytes(&bytes).unwrap());
    }

    #[proptest]
    fn roundtrip(key: MlDsa65PrivateKey, message: Vec<u8>) {
        let signature: MlDsa65Signature = key.try_sign_with_rng(&mut TestRng(7), &message).unwrap();
        key.verifying_key().verify(&message, &signature).unwrap();
        let recovered = MlDsa65PrivateKey::new(*key.seed());
        assert_eq!(recovered.public_key(), key.public_key());
    }
}
