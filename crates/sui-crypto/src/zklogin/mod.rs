use std::collections::HashMap;

use crate::SignatureError;
use poseidon::POSEIDON;
use signature::Verifier;
use sui_sdk_types::Jwk;
use sui_sdk_types::JwkId;
use sui_sdk_types::UserSignature;
use sui_sdk_types::ZkLoginAuthenticator;
use verify::CircuitVersion;

mod poseidon;
mod verify;

#[cfg(test)]
mod tests;

/// Which zkLogin circuit versions to accept during proof verification.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum ZkLoginCircuitMode {
    /// Accept proofs against the v1 circuit only; v2 proofs are rejected.
    #[default]
    V1Only,
    /// Try the v2 circuit first, then fall back to v1. Migration phase.
    Both,
    /// Accept proofs against the v2 circuit only; v1 proofs are rejected.
    V2Only,
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
        Self::new(
            [(CircuitVersion::V1, verify::VerifyingKey::new_mainnet())]
                .into_iter()
                .collect(),
        )
    }

    pub fn new_dev() -> Self {
        Self::new(
            [CircuitVersion::V1, CircuitVersion::V2]
                .into_iter()
                .map(|version| (version, verify::VerifyingKey::new_dev_for(version)))
                .collect(),
        )
    }

    pub fn jwks(&self) -> &HashMap<JwkId, Jwk> {
        &self.jwks
    }

    pub fn jwks_mut(&mut self) -> &mut HashMap<JwkId, Jwk> {
        &mut self.jwks
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

        // 3. verify groth16 proof against the circuit versions allowed by the configured mode
        let verify_proof = |version: CircuitVersion| {
            self.proof_verifying_keys
                .get(&version)
                .ok_or_else(|| {
                    SignatureError::from_source(format!("no verifying key for circuit {version:?}"))
                })?
                .verify_zklogin(
                    jwk,
                    &signature.inputs,
                    &signature.signature,
                    signature.max_epoch,
                    version,
                )
        };
        match self.circuit_mode {
            ZkLoginCircuitMode::V1Only => verify_proof(CircuitVersion::V1),
            ZkLoginCircuitMode::V2Only => verify_proof(CircuitVersion::V2),
            ZkLoginCircuitMode::Both => {
                verify_proof(CircuitVersion::V2).or_else(|_| verify_proof(CircuitVersion::V1))
            }
        }
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
