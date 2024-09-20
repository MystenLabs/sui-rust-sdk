use crate::SignatureError;
use crate::Signer;
use crate::SuiSigner;
use crate::SuiVerifier;
use crate::Verifier;
use sui_sdk::types::Ed25519PublicKey;
use sui_sdk::types::Ed25519Signature;
use sui_sdk::types::PersonalMessage;
use sui_sdk::types::SimpleSignature;
use sui_sdk::types::Transaction;
use sui_sdk::types::UserSignature;

pub struct Ed25519PrivateKey(ed25519_dalek::SigningKey);

impl std::fmt::Debug for Ed25519PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Ed25519PrivateKey")
            .field(&"__elided__")
            .finish()
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Ed25519PrivateKey {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        proptest::arbitrary::any::<[u8; Self::LENGTH]>()
            .prop_map(Self::new)
            .boxed()
    }
}

impl Ed25519PrivateKey {
    /// The length of an ed25519 private key in bytes.
    pub const LENGTH: usize = 32;

    pub fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes.into())
    }

    pub fn verifying_key(&self) -> Ed25519VerifyingKey {
        let verifying_key = self.0.verifying_key();
        Ed25519VerifyingKey(verifying_key)
    }

    pub fn public_key(&self) -> Ed25519PublicKey {
        self.verifying_key().public_key()
    }

    pub fn generate<R>(mut rng: R) -> Self
    where
        R: rand_core::RngCore + rand_core::CryptoRng,
    {
        let mut buf: [u8; Self::LENGTH] = [0; Self::LENGTH];
        rng.fill_bytes(&mut buf);
        Self(buf.into())
    }
}

impl Signer<Ed25519Signature> for Ed25519PrivateKey {
    fn try_sign(&self, msg: &[u8]) -> Result<Ed25519Signature, SignatureError> {
        self.0
            .try_sign(msg)
            .map(|signature| Ed25519Signature::new(signature.to_bytes()))
    }
}

impl Signer<UserSignature> for Ed25519PrivateKey {
    fn try_sign(&self, msg: &[u8]) -> Result<UserSignature, SignatureError> {
        <Self as Signer<Ed25519Signature>>::try_sign(self, msg).map(|signature| {
            let signature = SimpleSignature::Ed25519 {
                signature,
                public_key: self.public_key(),
            };
            UserSignature::Simple(signature)
        })
    }
}

impl SuiSigner for Ed25519PrivateKey {
    fn sign_transaction(&self, transaction: &Transaction) -> Result<UserSignature, SignatureError> {
        let msg = transaction.signing_digest();
        self.try_sign(&msg)
    }

    fn sign_personal_message(
        &self,
        message: &PersonalMessage<'_>,
    ) -> Result<UserSignature, SignatureError> {
        let msg = message.signing_digest();
        self.try_sign(&msg)
    }
}

pub struct Ed25519VerifyingKey(ed25519_dalek::VerifyingKey);

impl Ed25519VerifyingKey {
    pub fn new(public_key: &Ed25519PublicKey) -> Result<Self, SignatureError> {
        ed25519_dalek::VerifyingKey::from_bytes(public_key.inner()).map(Self)
    }

    pub fn public_key(&self) -> Ed25519PublicKey {
        Ed25519PublicKey::new(self.0.to_bytes())
    }
}

impl Verifier<Ed25519Signature> for Ed25519VerifyingKey {
    fn verify(&self, message: &[u8], signature: &Ed25519Signature) -> Result<(), SignatureError> {
        let signature = ed25519_dalek::Signature::from_bytes(signature.inner());
        self.0.verify_strict(message, &signature)
    }
}

impl Verifier<UserSignature> for Ed25519VerifyingKey {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::Simple(SimpleSignature::Ed25519 {
            signature,
            public_key,
        }) = signature
        else {
            return Err(SignatureError::from_source("not an ed25519 signature"));
        };

        if public_key.inner() != self.0.as_bytes() {
            return Err(SignatureError::from_source(
                "public_key in signature does not match",
            ));
        }

        <Self as Verifier<Ed25519Signature>>::verify(self, message, signature)
    }
}

impl SuiVerifier for Ed25519VerifyingKey {
    fn verify_transaction(
        &self,
        transaction: &Transaction,
        signature: &UserSignature,
    ) -> Result<(), SignatureError> {
        let message = transaction.signing_digest();
        self.verify(&message, signature)
    }

    fn verify_personal_message(
        &self,
        message: &PersonalMessage<'_>,
        signature: &UserSignature,
    ) -> Result<(), SignatureError> {
        let message = message.signing_digest();
        self.verify(&message, signature)
    }
}

#[derive(Default, Clone, Debug)]
pub struct Ed25519Verifier {}

impl Ed25519Verifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Verifier<UserSignature> for Ed25519Verifier {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::Simple(SimpleSignature::Ed25519 {
            signature,
            public_key,
        }) = signature
        else {
            return Err(SignatureError::from_source("not an ed25519 signature"));
        };

        let verifying_key = Ed25519VerifyingKey::new(public_key)?;

        verifying_key.verify(message, signature)
    }
}

impl SuiVerifier for Ed25519Verifier {
    fn verify_transaction(
        &self,
        transaction: &Transaction,
        signature: &UserSignature,
    ) -> Result<(), SignatureError> {
        let message = transaction.signing_digest();
        self.verify(&message, signature)
    }

    fn verify_personal_message(
        &self,
        message: &PersonalMessage<'_>,
        signature: &UserSignature,
    ) -> Result<(), SignatureError> {
        let message = message.signing_digest();
        self.verify(&message, signature)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_strategy::proptest;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    // TODO need to export proptest impl from core crate
    // #[proptest]
    // fn transaction_signing(signer: Ed25519PrivateKey, transaction: Transaction) {
    //     let signature = signer.sign_transaction(&transaction).unwrap();
    //     let verifier = signer.public_key();
    //     verifier
    //         .verify_transaction(&transaction, &signature)
    //         .unwrap();
    // }

    #[proptest]
    fn personal_message_signing(signer: Ed25519PrivateKey, message: Vec<u8>) {
        let message = PersonalMessage(message.into());
        let signature = signer.sign_personal_message(&message).unwrap();
        let verifying_key = signer.verifying_key();
        verifying_key
            .verify_personal_message(&message, &signature)
            .unwrap();

        let verifier = Ed25519Verifier::default();
        verifier
            .verify_personal_message(&message, &signature)
            .unwrap();
    }
}
