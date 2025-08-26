use std::collections::HashMap;

use crate::SignatureError;
use poseidon::POSEIDON;
use signature::Verifier;
use sui_sdk_types::Jwk;
use sui_sdk_types::JwkId;
use sui_sdk_types::UserSignature;
use sui_sdk_types::ZkLoginAuthenticator;

mod poseidon;
mod verify;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ZkloginVerifier {
    proof_verifying_key: verify::VerifyingKey,
    jwks: HashMap<JwkId, Jwk>,
}

impl ZkloginVerifier {
    fn new(proof_verifying_key: verify::VerifyingKey) -> Self {
        Self {
            proof_verifying_key,
            jwks: Default::default(),
        }
    }

    pub fn new_mainnet() -> Self {
        Self::new(verify::VerifyingKey::new_mainnet())
    }

    pub fn new_dev() -> Self {
        Self::new(verify::VerifyingKey::new_dev())
    }

    pub fn jwks(&self) -> &HashMap<JwkId, Jwk> {
        &self.jwks
    }

    pub fn jwks_mut(&mut self) -> &mut HashMap<JwkId, Jwk> {
        &mut self.jwks
    }
}

impl Verifier<ZkLoginAuthenticator> for ZkloginVerifier {
    fn verify(
        &self,
        message: &[u8],
        signature: &ZkLoginAuthenticator,
    ) -> Result<(), SignatureError> {
        // 1. check that we have a valid corrisponding Jwk
        let jwk_id = signature.inputs.jwk_id();
        let jwk = self.jwks.get(jwk_id).ok_or_else(|| {
            SignatureError::from_source(format!(
                "unable to find corrisponding jwk with id '{:?}' for provided authenticator",
                jwk_id,
            ))
        })?;

        // 2. verify that the provided SimpleSignature is valid
        crate::simple::SimpleVerifier.verify(message, &signature.signature)?;

        // 3. verify groth16 proof
        self.proof_verifying_key.verify_zklogin(
            jwk,
            &signature.inputs,
            &signature.signature,
            signature.max_epoch,
        )
    }
}

impl Verifier<UserSignature> for ZkloginVerifier {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::ZkLogin(zklogin_authenticator) = signature else {
            return Err(SignatureError::from_source("not a zklogin signature"));
        };

        self.verify(message, zklogin_authenticator.as_ref())
    }
}
