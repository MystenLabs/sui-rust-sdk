use sui_sdk_types::proof::ProofError;

use crate::proto::TryFromProtoError;

/// Errors returned by the [`crate::light_client`] APIs.
#[derive(Debug)]
#[non_exhaustive]
pub enum LightClientError {
    /// A gRPC call returned a non-success status.
    Rpc(tonic::Status),

    /// A protobuf message returned by the server was malformed.
    Proto(TryFromProtoError),

    /// BCS decoding of a wire-format payload (e.g. the response's
    /// `checkpoint_summary` bytes) failed.
    Bcs(bcs::Error),

    /// The BLS aggregate signature on a checkpoint summary failed to
    /// verify against the cached validator committee.
    InvalidSignature(sui_crypto::SignatureError),

    /// The OCS proof failed to verify against the trusted checkpoint
    /// summary.
    InvalidProof(ProofError),

    /// The cache was asked to apply a ratchet update with a new committee
    /// whose `epoch` was not exactly one greater than the current epoch.
    InvalidEpochAdvance {
        /// The current epoch held by the cache.
        current: u64,
        /// The advertised epoch of the new committee.
        provided: u64,
    },

    /// The cache was asked to apply a ratchet update whose end-of-epoch
    /// checkpoint sits before the start of the current epoch.
    InvalidCheckpointRange {
        /// The first checkpoint of the cache's current epoch.
        current_epoch_start: u64,
        /// The end-of-epoch checkpoint sequence number that was supplied.
        end_of_epoch_checkpoint: u64,
    },

    /// A checkpoint summary advertised as the last of an epoch had no
    /// `end_of_epoch_data` payload — the next epoch's committee cannot
    /// be extracted.
    MissingEndOfEpochData {
        /// The checkpoint sequence number whose summary was missing the
        /// payload.
        checkpoint: u64,
    },
}

impl std::fmt::Display for LightClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rpc(status) => write!(f, "gRPC error: {status}"),
            Self::Proto(e) => write!(f, "malformed protobuf message: {e}"),
            Self::Bcs(e) => write!(f, "BCS decoding failed: {e}"),
            Self::InvalidSignature(e) => {
                write!(f, "checkpoint summary signature did not verify: {e}")
            }
            Self::InvalidProof(e) => write!(f, "OCS proof did not verify: {e}"),
            Self::InvalidEpochAdvance { current, provided } => write!(
                f,
                "cannot advance epoch cache: current epoch is {current}, but provided committee is for epoch {provided}"
            ),
            Self::InvalidCheckpointRange {
                current_epoch_start,
                end_of_epoch_checkpoint,
            } => write!(
                f,
                "end-of-epoch checkpoint {end_of_epoch_checkpoint} is before the current epoch's start checkpoint {current_epoch_start}"
            ),
            Self::MissingEndOfEpochData { checkpoint } => write!(
                f,
                "checkpoint {checkpoint} was advertised as end-of-epoch but its summary has no `end_of_epoch_data` payload"
            ),
        }
    }
}

impl std::error::Error for LightClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Rpc(e) => Some(e),
            Self::Proto(e) => Some(e),
            Self::Bcs(e) => Some(e),
            Self::InvalidSignature(e) => Some(e),
            Self::InvalidProof(e) => Some(e),
            _ => None,
        }
    }
}

impl From<tonic::Status> for LightClientError {
    fn from(value: tonic::Status) -> Self {
        Self::Rpc(value)
    }
}

impl From<TryFromProtoError> for LightClientError {
    fn from(value: TryFromProtoError) -> Self {
        Self::Proto(value)
    }
}

impl From<bcs::Error> for LightClientError {
    fn from(value: bcs::Error) -> Self {
        Self::Bcs(value)
    }
}

impl From<sui_crypto::SignatureError> for LightClientError {
    fn from(value: sui_crypto::SignatureError) -> Self {
        Self::InvalidSignature(value)
    }
}

impl From<ProofError> for LightClientError {
    fn from(value: ProofError) -> Self {
        Self::InvalidProof(value)
    }
}
