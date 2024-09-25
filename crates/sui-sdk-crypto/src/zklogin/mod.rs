use std::collections::HashMap;

use crate::{SignatureError, SuiVerifier};
use poseidon::POSEIDON;
use signature::Verifier;
use sui_sdk_types::types::{Claim, Jwk, JwkId, UserSignature, ZkLoginAuthenticator, ZkLoginInputs};

mod poseidon;
mod verify;

#[cfg(test)]
mod tests;

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

impl SuiVerifier for ZkloginVerifier {
    fn verify_transaction(
        &self,
        transaction: &sui_sdk_types::types::Transaction,
        signature: &UserSignature,
    ) -> Result<(), SignatureError> {
        let message = transaction.signing_digest();
        self.verify(&message, signature)
    }

    fn verify_personal_message(
        &self,
        message: &sui_sdk_types::types::PersonalMessage<'_>,
        signature: &UserSignature,
    ) -> Result<(), SignatureError> {
        let message = message.signing_digest();
        self.verify(&message, signature)
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
        const ISS: &str = "iss";

        let header = JwtHeader::from_base64(&inputs.header_base64)?;
        let id = JwkId {
            iss: verify_extended_claim(&inputs.iss_base64_details, ISS)?,
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
        use base64ct::{Base64UrlUnpadded, Encoding};

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

/// Parse the extended claim json value to its claim value, using the expected claim key.
fn verify_extended_claim(claim: &Claim, expected_key: &str) -> Result<String, SignatureError> {
    /// Map a base64 string to a bit array by taking each char's index and convert it to binary form with one bit per u8
    /// element in the output. Returns SignatureError if one of the characters is not in the base64 charset.
    fn base64_to_bitarray(input: &str) -> Result<Vec<u8>, SignatureError> {
        use itertools::Itertools;

        const BASE64_URL_CHARSET: &str =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

        input
            .chars()
            .map(|c| {
                BASE64_URL_CHARSET
                    .find(c)
                    .map(|index| index as u8)
                    .map(|index| (0..6).rev().map(move |i| index >> i & 1))
                    .ok_or_else(|| SignatureError::from_source("base64_to_bitarry invalid input"))
            })
            .flatten_ok()
            .collect()
    }

    /// Convert a bitarray (each bit is represented by a u8) to a byte array by taking each 8 bits as a
    /// byte in big-endian format.
    fn bitarray_to_bytearray(bits: &[u8]) -> Result<Vec<u8>, SignatureError> {
        if bits.len() % 8 != 0 {
            return Err(SignatureError::from_source(
                "bitarray_to_bytearray invalid input",
            ));
        }
        Ok(bits
            .chunks(8)
            .map(|chunk| {
                let mut byte = 0u8;
                for (i, bit) in chunk.iter().rev().enumerate() {
                    byte |= bit << i;
                }
                byte
            })
            .collect())
    }

    /// Parse the base64 string, add paddings based on offset, and convert to a bytearray.
    fn decode_base64_url(s: &str, index_mod_4: &u8) -> Result<String, SignatureError> {
        if s.len() < 2 {
            return Err(SignatureError::from_source("Base64 string smaller than 2"));
        }
        let mut bits = base64_to_bitarray(s)?;
        match index_mod_4 {
            0 => {}
            1 => {
                bits.drain(..2);
            }
            2 => {
                bits.drain(..4);
            }
            _ => {
                return Err(SignatureError::from_source("Invalid first_char_offset"));
            }
        }

        let last_char_offset = (index_mod_4 + s.len() as u8 - 1) % 4;
        match last_char_offset {
            3 => {}
            2 => {
                bits.drain(bits.len() - 2..);
            }
            1 => {
                bits.drain(bits.len() - 4..);
            }
            _ => {
                return Err(SignatureError::from_source("Invalid last_char_offset"));
            }
        }

        if bits.len() % 8 != 0 {
            return Err(SignatureError::from_source("Invalid bits length"));
        }

        Ok(std::str::from_utf8(&bitarray_to_bytearray(&bits)?)
            .map_err(|_| SignatureError::from_source("Invalid UTF8 string"))?
            .to_owned())
    }

    let extended_claim = decode_base64_url(&claim.value, &claim.index_mod_4)?;

    // Last character of each extracted_claim must be '}' or ','
    if !(extended_claim.ends_with('}') || extended_claim.ends_with(',')) {
        return Err(SignatureError::from_source("Invalid extended claim"));
    }

    let json_str = format!("{{{}}}", &extended_claim[..extended_claim.len() - 1]);

    serde_json::from_str::<serde_json::Value>(&json_str)
        .map_err(SignatureError::from_source)?
        .as_object_mut()
        .and_then(|o| o.get_mut(expected_key))
        .map(serde_json::Value::take)
        .and_then(|v| match v {
            serde_json::Value::String(s) => Some(s),
            _ => None,
        })
        .ok_or_else(|| SignatureError::from_source("invalid extended claim"))
}
