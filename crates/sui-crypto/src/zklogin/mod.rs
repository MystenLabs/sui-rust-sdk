use std::collections::HashMap;

use crate::SignatureError;
use poseidon::POSEIDON;
use signature::Verifier;
use sui_sdk_types::Jwk;
use sui_sdk_types::JwkId;
use sui_sdk_types::UserSignature;
use sui_sdk_types::ZkLoginAuthenticator;
use sui_sdk_types::ZkLoginInputs;

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
        let jwt_details = JwtDetails::from_zklogin_inputs(&signature.inputs)?;
        let jwk = self.jwks.get(&jwt_details.id).ok_or_else(|| {
            SignatureError::from_source(format!(
                "unable to find corrisponding jwk with id '{:?}' for provided authenticator",
                jwt_details.id
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

/// A structed of parsed JWT details, consists of kid, header, iss.
#[derive(Debug, Clone, PartialEq, Eq)]
struct JwtDetails {
    header: JwtHeader,
    id: JwkId,
}

impl JwtDetails {
    fn from_zklogin_inputs(inputs: &ZkLoginInputs) -> Result<Self, SignatureError> {
        let header = JwtHeader::from_base64(&inputs.header_base64)?;
        let id = JwkId {
            iss: inputs.iss().map_err(SignatureError::from_source)?,
            kid: header.kid.clone(),
        };
        Ok(JwtDetails { header, id })
    }
}

/// Struct that represents a standard JWT header according to
/// https://openid.net/specs/openid-connect-core-1_0.html
#[derive(Debug, Clone, PartialEq, Eq)]
struct JwtHeader {
    alg: String,
    kid: String,
    typ: Option<String>,
}

impl JwtHeader {
    fn from_base64(s: &str) -> Result<Self, SignatureError> {
        use base64ct::Base64UrlUnpadded;
        use base64ct::Encoding;

        #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
        struct Header {
            alg: String,
            kid: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            typ: Option<String>,
        }

        let header_bytes = Base64UrlUnpadded::decode_vec(s)
            .map_err(|e| SignatureError::from_source(e.to_string()))?;
        let Header { alg, kid, typ } =
            serde_json::from_slice(&header_bytes).map_err(SignatureError::from_source)?;
        if alg != "RS256" {
            return Err(SignatureError::from_source("jwt alg must be RS256"));
        }
        Ok(Self { alg, kid, typ })
    }
}
