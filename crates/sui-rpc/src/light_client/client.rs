//! The end-to-end `LightClient` facade.

use sui_crypto::bls12381::ValidatorCommitteeSignatureVerifier;
use sui_sdk_types::Address;
use sui_sdk_types::Object;
use sui_sdk_types::ObjectReference;
use sui_sdk_types::SignedCheckpointSummary;
use sui_sdk_types::ValidatorCommittee;
use sui_sdk_types::proof::OcsInclusionProof;
use sui_sdk_types::proof::OcsNonInclusionProof;

use crate::Client;
use crate::field::FieldMask;
use crate::field::FieldMaskUtil;
use crate::proto::TryFromProtoError;
use crate::proto::sui::rpc::v2::GetCheckpointRequest;
use crate::proto::sui::rpc::v2alpha::GetCheckpointObjectProofRequest;
use crate::proto::sui::rpc::v2alpha::get_checkpoint_object_proof_response;

use super::EpochCache;
use super::RatchetConfig;
use super::error::LightClientError;
use super::error::ObjectDataMismatch;
use super::ratchet::ratchet_to_checkpoint_with_config;

/// The authenticated outcome of [`LightClient::prove_object_at_checkpoint`].
///
/// [`Inclusion`] means the checkpoint modified the object id: the
/// reference is authenticated by the OCS Merkle proof against the
/// checkpoint's BLS-verified summary. `object` is `Some` if the
/// modification left the object live (created, mutated, or unwrapped)
/// and BCS-decodes to a value whose `(id, version, digest)` matches
/// `object_ref`. `object` is `None` if the modification was a deletion
/// or wrap; in that case `object_ref.digest()` carries the framework's
/// sentinel digest for that operation, and the server omits the
/// `object_data` bytes (there is no live object state to authenticate).
///
/// [`NonInclusion`] means the checkpoint did **not** modify this
/// object id — cryptographically attested via a sorted-leaf
/// id-strict-bracketing non-inclusion proof.
///
/// **`NonInclusion` does not prove the object doesn't exist on chain.**
/// An object that was last modified in an earlier checkpoint and was
/// untouched at the requested checkpoint will produce a
/// `NonInclusion` result. Callers that need the object's state at a
/// checkpoint when it wasn't modified there must use a separate query
/// — for example, ratchet back to the most recent modification, or use
/// an unauthenticated read via
/// [`LedgerService.GetObject`](crate::proto::sui::rpc::v2::ledger_service_client::LedgerServiceClient).
/// An authenticated "current state at this checkpoint when unmodified"
/// query is future work tied to a different commitment scheme.
///
/// [`Inclusion`]: CheckpointObjectProof::Inclusion
/// [`NonInclusion`]: CheckpointObjectProof::NonInclusion
#[derive(Debug)]
#[non_exhaustive]
pub enum CheckpointObjectProof {
    /// The checkpoint modified the requested object id.
    Inclusion {
        /// The authenticated reference for the modified object.
        object_ref: ObjectReference,
        /// The decoded object at the version this checkpoint
        /// committed to, or `None` if the modification was a deletion
        /// or wrap.
        //
        // Boxed because `Object` is several hundred bytes and the
        // `NonInclusion` variant carries no data; keeping `Object`
        // inline would inflate every result by the full
        // `Inclusion`-payload size for callers that get a
        // `NonInclusion`.
        object: Option<Box<Object>>,
    },
    /// The checkpoint did not modify the requested object id. See the
    /// type-level doc for what this does and does not attest.
    NonInclusion,
}

/// A light client that authenticates state against a trusted validator
/// committee.
///
/// `LightClient` owns a [`Client`] for talking to a Sui gRPC endpoint and
/// an [`EpochCache`] seeded with a starting committee. Each verification
/// call advances the cache forward as far as needed (BLS-verifying each
/// epoch transition along the way), verifies the response's checkpoint
/// summary against the now-cached committee, and finally verifies the
/// returned proof against the trusted summary.
pub struct LightClient {
    rpc: Client,
    archive: Option<Client>,
    cache: EpochCache,
    ratchet_config: RatchetConfig,
}

impl LightClient {
    /// Build a new `LightClient` seeded with `starting_committee`.
    ///
    /// The starting committee must be obtained out of band (e.g. baked
    /// into the application, or read from a trusted source). It need
    /// not be the genesis committee — a client that only cares about
    /// recent state can seed the cache with a known-trusted committee
    /// at a later epoch (see the `bundled-trust-anchors` feature) and
    /// skip ratcheting through every prior epoch.
    pub fn new(rpc: Client, starting_committee: ValidatorCommittee) -> Self {
        Self {
            rpc,
            archive: None,
            cache: EpochCache::new(starting_committee),
            ratchet_config: RatchetConfig::default(),
        }
    }

    /// Override the [`RatchetConfig`] used when this client advances its
    /// epoch cache. Defaults to [`RatchetConfig::default`].
    pub fn with_ratchet_config(mut self, config: RatchetConfig) -> Self {
        self.ratchet_config = config;
        self
    }

    /// Attach an archive endpoint. The ratchet driver will prefer the
    /// archive for historical reads (`GetEpoch` during discovery and
    /// `GetCheckpoint` for end-of-epoch summaries) and fall back to
    /// the fullnode on any archive miss or error.
    ///
    /// Archives serve historical data with higher availability and
    /// tighter latency than typical fullnodes; misses (newer
    /// checkpoints not yet archived) transparently fall back to the
    /// fullnode. Both clients are used read-only by the ratchet —
    /// nothing in the cache becomes trustworthy solely because the
    /// archive served it; every end-of-epoch summary is still
    /// BLS-verified against the cache's current committee.
    pub fn with_archive(mut self, archive: Client) -> Self {
        self.archive = Some(archive);
        self
    }

    /// Read-only access to the client's epoch cache, for inspection.
    pub fn epoch_cache(&self) -> &EpochCache {
        &self.cache
    }

    /// Mutable access to the underlying RPC client, for callers that
    /// want to issue additional gRPC requests through the same channel.
    pub fn rpc(&mut self) -> &mut Client {
        &mut self.rpc
    }

    /// Return the sequence number of the network's most recent
    /// checkpoint, as reported by `LedgerService.GetCheckpoint(latest)`.
    ///
    /// The result is **not** trust-anchored — it's a single read from
    /// the server with no signature verification. Streaming clients use
    /// it as a starting cursor (e.g., "begin reading events from the
    /// checkpoint after this one"); any subsequent claim about an
    /// object's state at this checkpoint must still flow through
    /// [`Self::prove_object_at_checkpoint`] for cryptographic
    /// authentication.
    pub async fn latest_checkpoint_seq(&mut self) -> Result<u64, LightClientError> {
        let request = GetCheckpointRequest::latest()
            .with_read_mask(FieldMask::from_paths(["sequence_number"]));
        let response = self
            .rpc
            .ledger_client()
            .get_checkpoint(request)
            .await?
            .into_inner();
        response
            .checkpoint
            .and_then(|c| c.sequence_number)
            .ok_or_else(|| TryFromProtoError::missing("checkpoint.sequence_number").into())
    }

    /// Prove what (if anything) the checkpoint at `checkpoint_seq` did
    /// to `object_id`, returning a [`CheckpointObjectProof`] that
    /// distinguishes the four cases (created/mutated/unwrapped,
    /// deleted/wrapped, or not modified at all).
    ///
    /// The full chain of trust:
    ///
    /// 1. Call the alpha
    ///    [`ProofService.GetCheckpointObjectProof`](crate::proto::sui::rpc::v2alpha::proof_service_client)
    ///    RPC with `(object_id, checkpoint_seq)`.
    /// 2. Ratchet the epoch cache forward to cover the checkpoint
    ///    whose summary the server returned, BLS-verifying each
    ///    intervening end-of-epoch summary along the way.
    /// 3. BCS-decode the returned `SignedCheckpointSummary`, sanity-check
    ///    that its `sequence_number` matches `checkpoint_seq`, and
    ///    BLS-verify its aggregate signature against the now-trusted
    ///    committee for that epoch.
    /// 4. Dispatch on the response's `proof` oneof:
    ///    - **Inclusion**: sanity-check the returned `ObjectReference`'s
    ///      object id matches the request, verify the OCS inclusion
    ///      proof against the trusted summary, and (when `object_data`
    ///      is present) BCS-decode it and confirm its
    ///      `(id, version, digest)` match the authenticated leaf.
    ///      Absent `object_data` signals a deletion or wrap, in which
    ///      case `object` is returned as `None`.
    ///    - **NonInclusion**: verify the OCS non-inclusion proof
    ///      against the trusted summary for `object_id`. The proof's
    ///      id-strict-bracketing rules out any leaf with `object_id`,
    ///      proving the checkpoint did not modify it.
    ///
    /// **This method does not answer "what is the state of `object_id`
    /// at checkpoint `checkpoint_seq`?"** — that question requires
    /// walking back to the most recent modification or an entirely
    /// different commitment scheme. See [`CheckpointObjectProof`]'s
    /// type-level documentation for details.
    pub async fn prove_object_at_checkpoint(
        &mut self,
        object_id: &Address,
        checkpoint_seq: u64,
    ) -> Result<CheckpointObjectProof, LightClientError> {
        // 1. Fetch the proof from the alpha ProofService.
        let request = GetCheckpointObjectProofRequest::default()
            .with_object_id(object_id.to_string())
            .with_checkpoint(checkpoint_seq);
        let response = self
            .rpc
            .proof_client()
            .get_checkpoint_object_proof(request)
            .await?
            .into_inner();

        let summary_bytes = response
            .checkpoint_summary
            .ok_or_else(|| TryFromProtoError::missing("checkpoint_summary"))?;
        let proof = response
            .proof
            .ok_or_else(|| TryFromProtoError::missing("proof"))?;

        // 2. BCS-decode the signed checkpoint summary, sanity-check
        //    sequence number, ratchet, then BLS-verify.
        let signed_summary: SignedCheckpointSummary = bcs::from_bytes(&summary_bytes)?;
        let summary_seq = signed_summary.checkpoint.sequence_number;
        if summary_seq != checkpoint_seq {
            return Err(LightClientError::CheckpointMismatch {
                requested: checkpoint_seq,
                returned: summary_seq,
            });
        }

        ratchet_to_checkpoint_with_config(
            &mut self.rpc,
            self.archive.as_mut(),
            &mut self.cache,
            summary_seq,
            &self.ratchet_config,
        )
        .await?;

        let summary_epoch = signed_summary.checkpoint.epoch;
        let committee = self.cache.committee_for_epoch(summary_epoch).ok_or(
            LightClientError::NoCommitteeForEpoch {
                epoch: summary_epoch,
            },
        )?;
        let verifier = ValidatorCommitteeSignatureVerifier::new((*committee).clone())?;
        verifier
            .verify_checkpoint_summary(&signed_summary.checkpoint, &signed_summary.signature)?;

        // 3. Dispatch on the proof variant.
        match proof {
            get_checkpoint_object_proof_response::Proof::Inclusion(inclusion_proto) => {
                verify_inclusion(&signed_summary, object_id, inclusion_proto)
            }
            get_checkpoint_object_proof_response::Proof::NonInclusion(non_inclusion_proto) => {
                verify_non_inclusion(&signed_summary, object_id, non_inclusion_proto)
            }
        }
    }
}

/// Authenticate an `Inclusion` response: check the object id matches,
/// verify the OCS inclusion proof against the trusted summary, and
/// (when present) BCS-decode and cross-check `object_data` against the
/// authenticated leaf.
fn verify_inclusion(
    signed_summary: &SignedCheckpointSummary,
    object_id: &Address,
    inclusion_proto: crate::proto::sui::rpc::v2alpha::OcsInclusionProof,
) -> Result<CheckpointObjectProof, LightClientError> {
    let object_ref_proto = inclusion_proto
        .object_ref
        .as_ref()
        .ok_or_else(|| TryFromProtoError::missing("proof.inclusion.object_ref"))?;
    let object_ref: ObjectReference = object_ref_proto.try_into()?;

    // The server could otherwise authenticate a different object id —
    // the inclusion proof would still verify cryptographically, but
    // wouldn't answer the caller's question.
    if object_ref.object_id() != object_id {
        return Err(LightClientError::ObjectIdMismatch {
            requested: *object_id,
            returned: *object_ref.object_id(),
        });
    }

    let inclusion_proof: OcsInclusionProof = (&inclusion_proto).try_into()?;
    inclusion_proof.verify(&signed_summary.checkpoint, &object_ref)?;

    // BCS-decode the object data when present and cross-check.
    // Absence is the in-band signal for a deletion or wrap, in which
    // case the leaf's digest is a framework sentinel and there is no
    // live object state to authenticate.
    let object = inclusion_proto
        .object_data
        .as_ref()
        .map(|bytes| -> Result<Box<Object>, LightClientError> {
            let object: Object = bcs::from_bytes(bytes)?;
            let returned_ref =
                ObjectReference::new(object.object_id(), object.version(), object.digest());
            if returned_ref != object_ref {
                return Err(LightClientError::ObjectDataMismatch(Box::new(
                    ObjectDataMismatch {
                        expected: object_ref.clone(),
                        returned: returned_ref,
                    },
                )));
            }
            Ok(Box::new(object))
        })
        .transpose()?;

    Ok(CheckpointObjectProof::Inclusion { object_ref, object })
}

/// Authenticate a `NonInclusion` response: verify the OCS non-inclusion
/// proof against the trusted summary for `object_id`.
fn verify_non_inclusion(
    signed_summary: &SignedCheckpointSummary,
    object_id: &Address,
    non_inclusion_proto: crate::proto::sui::rpc::v2alpha::OcsNonInclusionProof,
) -> Result<CheckpointObjectProof, LightClientError> {
    let non_inclusion_proof: OcsNonInclusionProof = (&non_inclusion_proto).try_into()?;
    non_inclusion_proof.verify(&signed_summary.checkpoint, object_id)?;
    Ok(CheckpointObjectProof::NonInclusion)
}
