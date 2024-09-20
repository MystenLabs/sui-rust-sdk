use crate::SignatureError;
use crate::SuiSigner;
use crate::SuiVerifier;
use secp256k1::ecdsa::Signature;
use secp256k1::Message;
use secp256k1::PublicKey;
use secp256k1::SecretKey;
use signature::Signer;
use signature::Verifier;
use sui_sdk::types::PersonalMessage;
use sui_sdk::types::Secp256k1PublicKey;
use sui_sdk::types::Secp256k1Signature;
use sui_sdk::types::SimpleSignature;
use sui_sdk::types::Transaction;
use sui_sdk::types::UserSignature;

pub struct Secp256k1PrivateKey(SecretKey);

impl std::fmt::Debug for Secp256k1PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Secp256k1PrivateKey")
            .field(&"__elided__")
            .finish()
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Secp256k1PrivateKey {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        proptest::arbitrary::any::<[u8; Self::LENGTH]>()
            .prop_filter_map("invalid secp256k1 private key", |bytes| {
                Self::new(bytes).ok()
            })
            .boxed()
    }
}

impl Secp256k1PrivateKey {
    /// The length of an secp256k1 private key in bytes.
    pub const LENGTH: usize = 32;

    pub fn new(bytes: [u8; Self::LENGTH]) -> Result<Self, SignatureError> {
        SecretKey::from_slice(&bytes)
            .map_err(SignatureError::from_source)
            .map(Self)
    }

    pub fn verifying_key(&self) -> Secp256k1VerifyingKey {
        Secp256k1VerifyingKey(PublicKey::from_secret_key_global(&self.0))
    }

    pub fn public_key(&self) -> Secp256k1PublicKey {
        self.verifying_key().public_key()
    }

    #[cfg(feature = "rand")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "rand")))]
    pub fn generate<R>(mut rng: R) -> Self
    where
        R: rand_core::RngCore + rand_core::CryptoRng,
    {
        Self(SecretKey::new(&mut rng))
    }
}

impl Signer<Secp256k1Signature> for Secp256k1PrivateKey {
    fn try_sign(&self, message: &[u8]) -> Result<Secp256k1Signature, SignatureError> {
        let message = to_message(message);
        let signature = self.0.sign_ecdsa(message);
        Ok(Secp256k1Signature::new(signature.serialize_compact()))
    }
}

impl Signer<UserSignature> for Secp256k1PrivateKey {
    fn try_sign(&self, msg: &[u8]) -> Result<UserSignature, SignatureError> {
        <Self as Signer<Secp256k1Signature>>::try_sign(self, msg).map(|signature| {
            let signature = SimpleSignature::Secp256k1 {
                signature,
                public_key: self.public_key(),
            };
            UserSignature::Simple(signature)
        })
    }
}

impl SuiSigner for Secp256k1PrivateKey {
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

pub struct Secp256k1VerifyingKey(PublicKey);

impl Secp256k1VerifyingKey {
    pub fn new(public_key: &Secp256k1PublicKey) -> Result<Self, SignatureError> {
        PublicKey::from_slice(public_key.inner().as_ref())
            .map_err(SignatureError::from_source)
            .map(Self)
    }

    pub fn public_key(&self) -> Secp256k1PublicKey {
        Secp256k1PublicKey::new(self.0.serialize())
    }
}

impl Verifier<Secp256k1Signature> for Secp256k1VerifyingKey {
    fn verify(&self, message: &[u8], signature: &Secp256k1Signature) -> Result<(), SignatureError> {
        let signature =
            Signature::from_compact(signature.inner()).map_err(SignatureError::from_source)?;
        let message = to_message(message);
        signature
            .verify(&message, &self.0)
            .map_err(SignatureError::from_source)
    }
}

impl Verifier<UserSignature> for Secp256k1VerifyingKey {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::Simple(SimpleSignature::Secp256k1 {
            signature,
            public_key,
        }) = signature
        else {
            return Err(SignatureError::from_source("not a secp256k1 signature"));
        };

        if public_key.inner() != self.public_key().inner() {
            return Err(SignatureError::from_source(
                "public_key in signature does not match",
            ));
        }

        <Self as Verifier<Secp256k1Signature>>::verify(self, message, signature)
    }
}

impl SuiVerifier for Secp256k1VerifyingKey {
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

fn to_message(message: &[u8]) -> Message {
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();
    hasher.update(message);
    let digest = hasher.finalize();
    Message::from_digest(digest.into())
}

#[derive(Default, Clone, Debug)]
pub struct Secp256k1Verifier {}

impl Secp256k1Verifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Verifier<UserSignature> for Secp256k1Verifier {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::Simple(SimpleSignature::Secp256k1 {
            signature,
            public_key,
        }) = signature
        else {
            return Err(SignatureError::from_source("not a secp256k1 signature"));
        };

        let verifying_key = Secp256k1VerifyingKey::new(public_key)?;

        verifying_key.verify(message, signature)
    }
}

impl SuiVerifier for Secp256k1Verifier {
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
    // fn transaction_signing(signer: Secp256k1PrivateKey, transaction: Transaction) {
    //     let signature = signer.sign_transaction(&transaction).unwrap();
    //     let verifier = signer.public_key();
    //     verifier
    //         .verify_transaction(&transaction, &signature)
    //         .unwrap();
    // }

    #[proptest]
    fn personal_message_signing(signer: Secp256k1PrivateKey, message: Vec<u8>) {
        let message = PersonalMessage(message.into());
        let signature = signer.sign_personal_message(&message).unwrap();
        let verifying_key = signer.verifying_key();
        verifying_key
            .verify_personal_message(&message, &signature)
            .unwrap();

        let verifier = Secp256k1Verifier::default();
        verifier
            .verify_personal_message(&message, &signature)
            .unwrap();
    }
}
