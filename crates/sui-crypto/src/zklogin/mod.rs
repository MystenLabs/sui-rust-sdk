use std::borrow::Cow;
use std::collections::HashMap;

use crate::SignatureError;
use poseidon::POSEIDON;
use signature::Verifier;
use sui_sdk_types::Jwk;
use sui_sdk_types::JwkId;
use sui_sdk_types::UserSignature;
use sui_sdk_types::VersionedZkLoginInputs;
use sui_sdk_types::ZkLoginAuthenticator;
use sui_sdk_types::ZkLoginAuthenticatorV2;
use verify::CircuitVersion;

mod poseidon;
mod verify;

#[cfg(test)]
mod tests;

/// Which zkLogin authenticator versions to accept during proof verification.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum ZkLoginCircuitMode {
    /// Accept only the v1 authenticator with a v1 circuit proof.
    #[default]
    V1Only = 0,
    /// Accept both v1 and v2 authenticators.
    Both = 1,
    /// Accept only the v2 authenticator with a v2 circuit proof.
    V2Only = 2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZkloginVerifier {
    proof_verifying_keys: HashMap<CircuitVersion, verify::VerifyingKey>,
    circuit_mode: ZkLoginCircuitMode,
    jwks: HashMap<JwkId, Jwk>,
}

impl ZkloginVerifier {
    fn new(proof_verifying_keys: HashMap<CircuitVersion, verify::VerifyingKey>) -> Self {
        Self {
            proof_verifying_keys,
            circuit_mode: Default::default(),
            jwks: Default::default(),
        }
    }

    pub fn new_mainnet() -> Self {
        // The production v2 verifying key is pending.
        Self::new(HashMap::from([(
            CircuitVersion::V1,
            verify::VerifyingKey::new_mainnet(),
        )]))
    }

    pub fn new_dev() -> Self {
        Self::new(
            [CircuitVersion::V1, CircuitVersion::V2]
                .into_iter()
                .map(|version| (version, verify::VerifyingKey::new_dev_for(version)))
                .collect(),
        )
    }

    fn verifying_key(
        &self,
        version: CircuitVersion,
    ) -> Result<&verify::VerifyingKey, SignatureError> {
        self.proof_verifying_keys
            .get(&version)
            .ok_or_else(|| SignatureError::from_source("Groth16 proof verify failed"))
    }

    pub fn jwks(&self) -> &HashMap<JwkId, Jwk> {
        &self.jwks
    }

    pub fn jwks_mut(&mut self) -> &mut HashMap<JwkId, Jwk> {
        &mut self.jwks
    }

    fn jwk(&self, jwk_id: &JwkId) -> Result<&Jwk, SignatureError> {
        self.jwks.get(jwk_id).ok_or_else(|| {
            SignatureError::from_source(format!(
                "unable to find corrisponding jwk with id '{jwk_id:?}' for provided authenticator",
            ))
        })
    }

    pub fn circuit_mode(&self) -> ZkLoginCircuitMode {
        self.circuit_mode
    }

    /// Set which circuit mode version from sui.
    pub fn set_circuit_mode(&mut self, circuit_mode: ZkLoginCircuitMode) {
        self.circuit_mode = circuit_mode;
    }
}

impl Verifier<ZkLoginAuthenticator> for ZkloginVerifier {
    fn verify(
        &self,
        message: &[u8],
        signature: &ZkLoginAuthenticator,
    ) -> Result<(), SignatureError> {
        if self.circuit_mode == ZkLoginCircuitMode::V2Only {
            return Err(SignatureError::from_source("zkLogin v1 is not enabled"));
        }

        let verifying_key = self.verifying_key(CircuitVersion::V1)?;
        let inputs = &signature.inputs;
        let user_signature = &signature.signature;
        let jwk = self.jwk(inputs.jwk_id())?;

        crate::simple::SimpleVerifier.verify(message, user_signature)?;
        verifying_key.verify_zklogin_v1(jwk, inputs, user_signature, signature.max_epoch)
    }
}

impl Verifier<ZkLoginAuthenticatorV2> for ZkloginVerifier {
    fn verify(
        &self,
        message: &[u8],
        signature: &ZkLoginAuthenticatorV2,
    ) -> Result<(), SignatureError> {
        if self.circuit_mode == ZkLoginCircuitMode::V1Only {
            return Err(SignatureError::from_source("zkLogin v2 is not enabled"));
        }

        let verifying_key = self.verifying_key(CircuitVersion::V2)?;
        let VersionedZkLoginInputs::V2(inputs) = &signature.inputs else {
            return Err(SignatureError::from_source("unknown zkLogin v2 inputs"));
        };
        let user_signature = &signature.user_signature;
        let mut jwk_id = Cow::Borrowed(inputs.jwk_id());
        // FastCrypto looks up tenant-specific Microsoft issuers using the common issuer.
        if jwk_id.iss.starts_with("https://login.microsoftonline.com/")
            && jwk_id.iss.ends_with("/v2.0")
        {
            jwk_id.to_mut().iss = "https://login.microsoftonline.com/v2.0".to_owned();
        }
        let jwk = self.jwk(&jwk_id)?;

        crate::simple::SimpleVerifier.verify(message, user_signature)?;
        verifying_key.verify_zklogin_v2(jwk, inputs, user_signature, signature.max_epoch)
    }
}

impl Verifier<UserSignature> for ZkloginVerifier {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        match signature {
            UserSignature::ZkLogin(authenticator) => self.verify(message, authenticator.as_ref()),
            UserSignature::ZkLoginV2(authenticator) => self.verify(message, authenticator.as_ref()),
            _ => Err(SignatureError::from_source("not a zklogin signature")),
        }
    }
}
