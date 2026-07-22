use super::Client;
use crate::proto::sui::rpc::v2::Bcs;
use crate::proto::sui::rpc::v2::VerifySignatureRequest;
use sui_sdk_types::Address;
use sui_sdk_types::PersonalMessage;
use sui_sdk_types::UserSignature;

/// Error types that can occur when verifying a signature
#[derive(Debug)]
#[non_exhaustive]
pub enum VerifySignatureError {
    /// RPC Error (actual tonic::Status from the client/server)
    RpcError(tonic::Status),
    /// Failed to serialize the message
    SerializationError(bcs::Error),
    /// The signature is invalid, with the reason reported by the server
    InvalidSignature(String),
}

impl std::fmt::Display for VerifySignatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RpcError(status) => write!(f, "RPC error: {status}"),
            Self::SerializationError(e) => write!(f, "Failed to serialize message: {e}"),
            Self::InvalidSignature(reason) => write!(f, "Invalid signature: {reason}"),
        }
    }
}

impl std::error::Error for VerifySignatureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::RpcError(status) => Some(status),
            Self::SerializationError(e) => Some(e),
            Self::InvalidSignature(_) => None,
        }
    }
}

impl Client {
    /// Verifies a personal message signature using the fullnode's
    /// `SignatureVerificationService`. Supports all signature schemes. 
    /// For zklogin signatures the fullnode validates against the current 
    /// epoch and the set of valid JWKs stored onchain.
    pub async fn verify_personal_message_signature(
        &mut self,
        message: &PersonalMessage<'_>,
        signature: &UserSignature,
        address: Option<Address>,
    ) -> Result<(), VerifySignatureError> {
        let mut message_bcs =
            Bcs::serialize(&message.0).map_err(VerifySignatureError::SerializationError)?;
        message_bcs.name = Some("PersonalMessage".to_owned());

        let mut request = VerifySignatureRequest::default();
        request.message = Some(message_bcs);
        request.signature = Some(signature.clone().into());
        request.address = address.map(|address| address.to_string());

        let response = self
            .signature_verification_client()
            .verify_signature(request)
            .await
            .map_err(VerifySignatureError::RpcError)?
            .into_inner();

        if response.is_valid() {
            Ok(())
        } else {
            Err(VerifySignatureError::InvalidSignature(
                response.reason().to_owned(),
            ))
        }
    }
}
