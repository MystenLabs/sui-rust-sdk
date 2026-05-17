use sui_sdk_types::Address;
use sui_sdk_types::ObjectReference;
use sui_sdk_types::framework::ApplyStreamError;
use sui_sdk_types::framework::EventStreamHead;
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

    /// A checkpoint summary advertised as the last of an epoch had no
    /// `end_of_epoch_data` payload — the next epoch's committee cannot
    /// be extracted.
    MissingEndOfEpochData {
        /// The checkpoint sequence number whose summary was missing the
        /// payload.
        checkpoint: u64,
    },

    /// The server returned a proof anchored at a different checkpoint
    /// than the one the caller requested.
    CheckpointMismatch {
        /// The checkpoint sequence number the caller asked for.
        requested: u64,
        /// The checkpoint sequence number on the returned summary.
        returned: u64,
    },

    /// After ratcheting forward, the cache still did not have a
    /// committee on file for the relevant epoch. This should not
    /// happen in practice and indicates either a bug or a malicious
    /// server response.
    NoCommitteeForEpoch {
        /// The epoch number that has no committee on file.
        epoch: u64,
    },

    /// The server returned an inclusion proof for a different object
    /// id than the caller asked about.
    ObjectIdMismatch {
        /// The object id the caller asked about.
        requested: Address,
        /// The object id on the returned object reference.
        returned: Address,
    },

    /// The `object_data` bytes the server returned did not BCS-decode to
    /// an [`Object`] whose `(id, version, digest)` match the
    /// authenticated `ObjectReference`. Without this check, a server
    /// could pass the proof verification step while still returning
    /// arbitrary bytes for the object's contents.
    ///
    /// Boxed because `ObjectReference` carries a 32-byte digest and a
    /// 32-byte address; keeping two of them inline would inflate every
    /// `Result<_, LightClientError>` slot by ~144 bytes for a case
    /// that's exceedingly rare in practice.
    ///
    /// [`Object`]: sui_sdk_types::Object
    ObjectDataMismatch(Box<ObjectDataMismatch>),

    /// The locally-replayed [`EventStreamHead`] disagreed with the
    /// authenticated head fetched from chain at the given checkpoint.
    /// This is the streaming client's terminal failure mode — once the
    /// MMRs diverge, every subsequent event would compound the
    /// divergence, so the stream is aborted with this error.
    ///
    /// Boxed because `EventStreamHead` carries a `Vec<U256>` (each
    /// `U256` is 32 bytes) and grows unbounded with checkpoint count;
    /// keeping two of them inline would inflate every
    /// `Result<_, LightClientError>` slot returned from the streaming
    /// task.
    MmrMismatch(Box<MmrMismatch>),

    /// Folding a batch of received events into the local MMR violated
    /// the [`apply_stream_updates`] contract — empty batch, mismatched
    /// `checkpoint_seq` within a batch, or non-monotonic batch
    /// ordering. Surfaces a malformed server response that slipped
    /// past the per-event decode.
    ///
    /// [`apply_stream_updates`]: sui_sdk_types::framework::apply_stream_updates
    InvalidEventBatch(ApplyStreamError),

    /// An object returned by the OCS inclusion proof flow did not
    /// match the shape the caller expected — for example, an
    /// `EventStreamHead` fetch returned a package instead of a Move
    /// struct, or the dynamic-field contents were too short to
    /// contain the expected value.
    UnexpectedObjectShape {
        /// A short, human-readable description of what the caller was
        /// expecting versus what the object actually was.
        reason: &'static str,
    },
}

/// Payload for [`LightClientError::ObjectDataMismatch`].
#[derive(Debug)]
pub struct ObjectDataMismatch {
    /// The authenticated reference from the verified inclusion proof.
    pub expected: ObjectReference,
    /// The reference reconstructed from the returned `object_data`
    /// bytes.
    pub returned: ObjectReference,
}

/// Payload for [`LightClientError::MmrMismatch`].
#[derive(Debug)]
pub struct MmrMismatch {
    /// The checkpoint at which the reconciliation was attempted.
    pub checkpoint: u64,
    /// The head fetched from chain via an OCS inclusion proof — the
    /// truth against which the local replay was compared.
    pub expected: EventStreamHead,
    /// The head the streaming client computed by folding received
    /// events into its local MMR.
    pub actual: EventStreamHead,
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
            Self::MissingEndOfEpochData { checkpoint } => write!(
                f,
                "checkpoint {checkpoint} was advertised as end-of-epoch but its summary has no `end_of_epoch_data` payload"
            ),
            Self::CheckpointMismatch {
                requested,
                returned,
            } => write!(
                f,
                "proof was returned for checkpoint {returned} but caller requested checkpoint {requested}"
            ),
            Self::NoCommitteeForEpoch { epoch } => write!(
                f,
                "no validator committee on file for epoch {epoch} after ratchet"
            ),
            Self::ObjectIdMismatch {
                requested,
                returned,
            } => write!(
                f,
                "proof was returned for object {returned} but caller requested object {requested}"
            ),
            Self::ObjectDataMismatch(boxed) => {
                let ObjectDataMismatch { expected, returned } = boxed.as_ref();
                write!(
                    f,
                    "object_data bytes hash to {} version {} but the verified leaf attests to {} version {}",
                    returned.digest(),
                    returned.version(),
                    expected.digest(),
                    expected.version(),
                )
            }
            Self::MmrMismatch(boxed) => {
                let MmrMismatch {
                    checkpoint,
                    expected,
                    actual,
                } = boxed.as_ref();
                write!(
                    f,
                    "local MMR diverged from on-chain EventStreamHead at checkpoint {checkpoint}: \
                     locally replayed {} events to checkpoint {}, chain attests to {} events at \
                     checkpoint {}",
                    actual.num_events,
                    actual.checkpoint_seq,
                    expected.num_events,
                    expected.checkpoint_seq,
                )
            }
            Self::InvalidEventBatch(e) => write!(f, "invalid event batch: {e}"),
            Self::UnexpectedObjectShape { reason } => {
                write!(f, "unexpected object shape: {reason}")
            }
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
            Self::InvalidEventBatch(e) => Some(e),
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

impl From<ApplyStreamError> for LightClientError {
    fn from(value: ApplyStreamError) -> Self {
        Self::InvalidEventBatch(value)
    }
}
