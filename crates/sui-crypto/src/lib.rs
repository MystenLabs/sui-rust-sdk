#![cfg_attr(doc_cfg, feature(doc_cfg))]

use sui_sdk_types::PersonalMessage;
use sui_sdk_types::Transaction;
use sui_sdk_types::UserSignature;

pub use signature::Error as SignatureError;
pub use signature::Signer;
pub use signature::Verifier;

#[cfg(feature = "bls12381")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "bls12381")))]
pub mod bls12381;

#[cfg(feature = "ed25519")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "ed25519")))]
pub mod ed25519;

#[cfg(feature = "secp256k1")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "secp256k1")))]
pub mod secp256k1;

#[cfg(feature = "secp256r1")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "secp256r1")))]
pub mod secp256r1;

#[cfg(feature = "passkey")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "passkey")))]
pub mod passkey;

#[cfg(feature = "mldsa65")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "mldsa65")))]
pub mod mldsa65;

#[cfg(feature = "zklogin")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "zklogin")))]
pub mod zklogin;

#[cfg(any(
    feature = "ed25519",
    feature = "secp256r1",
    feature = "secp256k1",
    feature = "mldsa65",
    feature = "zklogin"
))]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(
        feature = "ed25519",
        feature = "secp256r1",
        feature = "secp256k1",
        feature = "mldsa65",
        feature = "zklogin"
    )))
)]
pub mod simple;

#[cfg(any(
    feature = "ed25519",
    feature = "secp256r1",
    feature = "secp256k1",
    feature = "mldsa65"
))]
mod suipriv;

#[cfg(any(
    feature = "ed25519",
    feature = "secp256r1",
    feature = "secp256k1",
    feature = "mldsa65",
    feature = "zklogin"
))]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(
        feature = "ed25519",
        feature = "secp256r1",
        feature = "secp256k1",
        feature = "mldsa65",
        feature = "zklogin"
    )))
)]
pub mod multisig;

#[cfg(any(
    feature = "ed25519",
    feature = "secp256r1",
    feature = "secp256k1",
    feature = "mldsa65",
    feature = "zklogin"
))]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(
        feature = "ed25519",
        feature = "secp256r1",
        feature = "secp256k1",
        feature = "mldsa65",
        feature = "zklogin"
    )))
)]
#[doc(inline)]
pub use multisig::UserSignatureVerifier;

/// Interface for signing user transactions and messages in Sui
///
/// # Note
///
/// There is a blanket implementation of `SuiSigner` for all `T` where `T:
/// `[`Signer`]`<`[`UserSignature`]`>` so it is generally recommended for a signer to implement
/// `Signer<UserSignature>` and rely on the blanket implementation which handles the proper
/// construction of the signing message.
pub trait SuiSigner {
    fn sign_transaction(&self, transaction: &Transaction) -> Result<UserSignature, SignatureError>;
    fn sign_personal_message(
        &self,
        message: &PersonalMessage<'_>,
    ) -> Result<UserSignature, SignatureError>;
}

impl<T: Signer<UserSignature>> SuiSigner for T {
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

/// Interface for signing user transactions and messages with a scheme that
/// takes fresh randomness per signature
///
/// Randomized counterpart of [`SuiSigner`]. Today that is ML-DSA-65 only. The
/// caller supplies the rng, so the crate needs no `getrandom` backend.
///
/// # Note
///
/// There is a blanket implementation of `SuiRandomizedSigner` for all `T`
/// where `T: `[`signature::RandomizedSigner`]`<`[`UserSignature`]`>`, which handles the
/// proper construction of the signing message.
#[cfg(feature = "mldsa65")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "mldsa65")))]
pub trait SuiRandomizedSigner {
    fn sign_transaction_with_rng(
        &self,
        rng: &mut impl rand_core::CryptoRngCore,
        transaction: &Transaction,
    ) -> Result<UserSignature, SignatureError>;
    fn sign_personal_message_with_rng(
        &self,
        rng: &mut impl rand_core::CryptoRngCore,
        message: &PersonalMessage<'_>,
    ) -> Result<UserSignature, SignatureError>;
}

#[cfg(feature = "mldsa65")]
impl<T: signature::RandomizedSigner<UserSignature>> SuiRandomizedSigner for T {
    fn sign_transaction_with_rng(
        &self,
        rng: &mut impl rand_core::CryptoRngCore,
        transaction: &Transaction,
    ) -> Result<UserSignature, SignatureError> {
        let msg = transaction.signing_digest();
        self.try_sign_with_rng(rng, &msg)
    }

    fn sign_personal_message_with_rng(
        &self,
        rng: &mut impl rand_core::CryptoRngCore,
        message: &PersonalMessage<'_>,
    ) -> Result<UserSignature, SignatureError> {
        let msg = message.signing_digest();
        self.try_sign_with_rng(rng, &msg)
    }
}

/// Interface for verifying user transactions and messages in Sui
///
/// # Note
///
/// There is a blanket implementation of `SuiVerifier` for all `T` where `T:
/// `[`Verifier`]`<`[`UserSignature`]`>` so it is generally recommended for a signer to implement
/// `Verifier<UserSignature>` and rely on the blanket implementation which handles the proper
/// construction of the signing message.
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

impl<T: Verifier<UserSignature>> SuiVerifier for T {
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
