use crate::{SignatureError, SuiVerifier};
use signature::Verifier;
use sui_sdk::types::{SimpleSignature, UserSignature};

pub struct SimpleVerifier;

impl Verifier<SimpleSignature> for SimpleVerifier {
    #[allow(unused_variables)]
    fn verify(&self, message: &[u8], signature: &SimpleSignature) -> Result<(), SignatureError> {
        match signature {
            #[cfg(feature = "ed25519")]
            SimpleSignature::Ed25519 {
                signature,
                public_key,
            } => {
                let verifying_key = crate::ed25519::Ed25519VerifyingKey::new(public_key)?;
                verifying_key.verify(message, signature)
            }
            #[cfg(not(feature = "ed25519"))]
            SimpleSignature::Ed25519 { .. } => Err(SignatureError::from_source(
                "support for ed25519 is not enabled",
            )),

            #[cfg(feature = "secp256k1")]
            SimpleSignature::Secp256k1 {
                signature,
                public_key,
            } => {
                let verifying_key = crate::secp256k1::Secp256k1VerifyingKey::new(public_key)?;
                verifying_key.verify(message, signature)
            }
            #[cfg(not(feature = "secp256k1"))]
            SimpleSignature::Secp256k1 { .. } => Err(SignatureError::from_source(
                "support for secp256k1 is not enabled",
            )),

            #[cfg(feature = "secp256r1")]
            SimpleSignature::Secp256r1 {
                signature,
                public_key,
            } => {
                let verifying_key = crate::secp256r1::Secp256r1VerifyingKey::new(public_key)?;
                verifying_key.verify(message, signature)
            }
            #[cfg(not(feature = "secp256r1"))]
            SimpleSignature::Secp256r1 { .. } => Err(SignatureError::from_source(
                "support for secp256r1 is not enabled",
            )),
        }
    }
}

impl Verifier<UserSignature> for SimpleVerifier {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::Simple(signature) = signature else {
            return Err(SignatureError::from_source("not a simple signature"));
        };

        <Self as Verifier<SimpleSignature>>::verify(self, message, signature)
    }
}

impl SuiVerifier for SimpleVerifier {
    fn verify_transaction(
        &self,
        transaction: &sui_sdk::types::Transaction,
        signature: &UserSignature,
    ) -> Result<(), SignatureError> {
        let message = transaction.signing_digest();
        self.verify(&message, signature)
    }

    fn verify_personal_message(
        &self,
        message: &sui_sdk::types::PersonalMessage<'_>,
        signature: &UserSignature,
    ) -> Result<(), SignatureError> {
        let message = message.signing_digest();
        self.verify(&message, signature)
    }
}
