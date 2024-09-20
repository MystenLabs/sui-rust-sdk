use crate::SignatureError;
use crate::SuiSigner;
use crate::SuiVerifier;
use p256::ecdsa::SigningKey;
use p256::ecdsa::VerifyingKey;
use p256::elliptic_curve::group::GroupEncoding;
use signature::Signer;
use signature::Verifier;
use sui_sdk::types::PersonalMessage;
use sui_sdk::types::Secp256r1PublicKey;
use sui_sdk::types::Secp256r1Signature;
use sui_sdk::types::SimpleSignature;
use sui_sdk::types::Transaction;
use sui_sdk::types::UserSignature;

pub struct Secp256r1PrivateKey(SigningKey);

impl std::fmt::Debug for Secp256r1PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Secp256r1PrivateKey")
            .field(&"__elided__")
            .finish()
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Secp256r1PrivateKey {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        proptest::arbitrary::any::<[u8; Self::LENGTH]>()
            .prop_map(Self::new)
            .boxed()
    }
}

impl Secp256r1PrivateKey {
    /// The length of an secp256r1 private key in bytes.
    pub const LENGTH: usize = 32;

    pub fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(SigningKey::from_bytes(&bytes.into()).unwrap())
    }

    pub fn verifying_key(&self) -> Secp256r1VerifyingKey {
        let verifying_key = self.0.verifying_key();
        Secp256r1VerifyingKey(*verifying_key)
    }

    pub fn public_key(&self) -> Secp256r1PublicKey {
        Secp256r1PublicKey::new(self.0.verifying_key().as_ref().to_bytes().into())
    }

    pub fn generate<R>(mut rng: R) -> Self
    where
        R: rand_core::RngCore + rand_core::CryptoRng,
    {
        let mut buf: [u8; Self::LENGTH] = [0; Self::LENGTH];
        rng.fill_bytes(&mut buf);
        Self::new(buf)
    }
}

impl Signer<Secp256r1Signature> for Secp256r1PrivateKey {
    fn try_sign(&self, message: &[u8]) -> Result<Secp256r1Signature, SignatureError> {
        let signature: p256::ecdsa::Signature = self.0.try_sign(message)?;
        Ok(Secp256r1Signature::new(signature.to_bytes().into()))
    }
}

impl Signer<UserSignature> for Secp256r1PrivateKey {
    fn try_sign(&self, msg: &[u8]) -> Result<UserSignature, SignatureError> {
        <Self as Signer<Secp256r1Signature>>::try_sign(self, msg).map(|signature| {
            let signature = SimpleSignature::Secp256r1 {
                signature,
                public_key: self.public_key(),
            };
            UserSignature::Simple(signature)
        })
    }
}

impl SuiSigner for Secp256r1PrivateKey {
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

pub struct Secp256r1VerifyingKey(VerifyingKey);

impl Secp256r1VerifyingKey {
    pub fn new(public_key: &Secp256r1PublicKey) -> Result<Self, SignatureError> {
        VerifyingKey::try_from(public_key.inner().as_ref()).map(Self)
    }

    pub fn public_key(&self) -> Secp256r1PublicKey {
        Secp256r1PublicKey::new(self.0.as_ref().to_bytes().into())
    }
}

impl Verifier<Secp256r1Signature> for Secp256r1VerifyingKey {
    fn verify(&self, message: &[u8], signature: &Secp256r1Signature) -> Result<(), SignatureError> {
        let signature = p256::ecdsa::Signature::from_bytes(signature.inner().into())?;
        self.0.verify(message, &signature)
    }
}

impl Verifier<UserSignature> for Secp256r1VerifyingKey {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::Simple(SimpleSignature::Secp256r1 {
            signature,
            public_key,
        }) = signature
        else {
            return Err(SignatureError::from_source("not a secp256r1 signature"));
        };

        if public_key.inner() != self.public_key().inner() {
            return Err(SignatureError::from_source(
                "public_key in signature does not match",
            ));
        }

        <Self as Verifier<Secp256r1Signature>>::verify(self, message, signature)
    }
}

impl SuiVerifier for Secp256r1VerifyingKey {
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
pub struct Secp256r1Verifier {}

impl Secp256r1Verifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Verifier<UserSignature> for Secp256r1Verifier {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::Simple(SimpleSignature::Secp256r1 {
            signature,
            public_key,
        }) = signature
        else {
            return Err(SignatureError::from_source("not a secp256r1 signature"));
        };

        let verifying_key = Secp256r1VerifyingKey::new(public_key)?;

        verifying_key.verify(message, signature)
    }
}

impl SuiVerifier for Secp256r1Verifier {
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
    // fn transaction_signing(signer: Secp256r1PrivateKey, transaction: Transaction) {
    //     let signature = signer.sign_transaction(&transaction).unwrap();
    //     let verifier = signer.public_key();
    //     verifier
    //         .verify_transaction(&transaction, &signature)
    //         .unwrap();
    // }

    #[proptest]
    fn personal_message_signing(signer: Secp256r1PrivateKey, message: Vec<u8>) {
        let message = PersonalMessage(message.into());
        let signature = signer.sign_personal_message(&message).unwrap();
        let verifying_key = signer.verifying_key();
        verifying_key
            .verify_personal_message(&message, &signature)
            .unwrap();

        let verifier = Secp256r1Verifier::default();
        verifier
            .verify_personal_message(&message, &signature)
            .unwrap();
    }
}
