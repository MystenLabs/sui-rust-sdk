#![cfg_attr(doc_cfg, feature(doc_cfg))]

use sui_sdk::types::PersonalMessage;
use sui_sdk::types::Transaction;
use sui_sdk::types::UserSignature;

pub use signature::Error as SignatureError;
pub use signature::Signer;
pub use signature::Verifier;

#[cfg(feature = "ed25519")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "ed25519")))]
pub mod ed25519;

#[allow(unused)]
mod bls12381;

#[cfg(feature = "secp256k1")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "secp256k1")))]
pub mod secp256k1;

#[cfg(feature = "secp256r1")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "secp256r1")))]
pub mod secp256r1;

pub trait SuiSigner {
    fn sign_transaction(&self, transaction: &Transaction) -> Result<UserSignature, SignatureError>;
    fn sign_personal_message(
        &self,
        message: &PersonalMessage<'_>,
    ) -> Result<UserSignature, SignatureError>;
}

pub trait SuiVerifier {
    fn verify_transaction(
        &self,
        transaction: &Transaction,
        signature: &UserSignature,
    ) -> Result<(), SignatureError>;
    fn verify_personal_message(
        &self,
        message: &PersonalMessage<'_>,
        signature: &UserSignature,
    ) -> Result<(), SignatureError>;
}
