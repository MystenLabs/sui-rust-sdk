//! The end-to-end `LightClient` facade.

use sui_crypto::bls12381::ValidatorCommitteeSignatureVerifier;
use sui_sdk_types::Address;
use sui_sdk_types::Digest;
use sui_sdk_types::ObjectReference;
use sui_sdk_types::SignedCheckpointSummary;
use sui_sdk_types::ValidatorCommittee;
use sui_sdk_types::proof::OcsInclusionProof;

use crate::Client;
use crate::proto::TryFromProtoError;
use crate::proto::sui::rpc::v2alpha::GetOcsInclusionProofRequest;

use super::EpochCache;
use super::error::LightClientError;
use super::ratchet::ratchet_to_checkpoint;

/// A light client that authenticates state against a trusted validator
/// committee.
///
/// `LightClient` owns a [`Client`] for talking to a Sui gRPC endpoint and
/// an [`EpochCache`] seeded with the genesis committee. Each verification
/// call advances the cache forward as far as needed (BLS-verifying each
/// epoch transition along the way), verifies the response's checkpoint
/// summary against the now-cached committee, and finally verifies the
/// returned proof against the trusted summary.
pub struct LightClient {
    rpc: Client,
    cache: EpochCache,
}

impl LightClient {
    /// Build a new `LightClient` seeded with `genesis_committee`.
    ///
    /// The genesis committee must be obtained out of band (e.g. baked
    /// into the application, or read from a trusted source). The cache
    /// will advance forward from there as verification calls are made.
    pub fn new(rpc: Client, genesis_committee: ValidatorCommittee) -> Self {
        Self {
            rpc,
            cache: EpochCache::new(genesis_committee),
        }
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

    /// Verify that `object_id` was written in the checkpoint with
    /// `checkpoint_seq`, returning the authenticated [`ObjectReference`]
    /// on success.
    ///
    /// The full chain of trust:
    ///
    /// 1. Call the alpha `ProofService.GetOcsInclusionProof` RPC with
    ///    `(object_id, checkpoint_seq)`.
    /// 2. Ratchet the epoch cache forward to cover the checkpoint
    ///    whose summary the server returned, BLS-verifying each
    ///    intervening end-of-epoch summary along the way.
    /// 3. BCS-decode the returned `SignedCheckpointSummary`, sanity-check
    ///    that its `sequence_number` matches `checkpoint_seq`, and
    ///    BLS-verify its aggregate signature against the now-trusted
    ///    committee for that epoch.
    /// 4. Sanity-check that the returned `ObjectReference`'s object id
    ///    matches the requested `object_id`.
    /// 5. Verify the OCS inclusion proof against the trusted summary
    ///    using the proof verifier in `sui_sdk_types::proof`.
    ///
    /// On success the returned `ObjectReference` is authenticated
    /// end-to-end: its version and digest are the values the network
    /// committed to at the requested checkpoint.
    pub async fn verify_object_at_checkpoint(
        &mut self,
        object_id: &Address,
        checkpoint_seq: u64,
    ) -> Result<ObjectReference, LightClientError> {
        // 1. Fetch the proof from the alpha ProofService.
        let request = GetOcsInclusionProofRequest::default()
            .with_object_id(object_id.to_string())
            .with_checkpoint(checkpoint_seq);
        let response = self
            .rpc
            .proof_client()
            .get_ocs_inclusion_proof(request)
            .await?
            .into_inner();

        let object_ref_proto = response
            .object_ref
            .ok_or_else(|| TryFromProtoError::missing("object_ref"))?;
        let inclusion_proof_proto = response
            .inclusion_proof
            .ok_or_else(|| TryFromProtoError::missing("inclusion_proof"))?;
        let summary_bytes = response
            .checkpoint_summary
            .ok_or_else(|| TryFromProtoError::missing("checkpoint_summary"))?;

        // 2. Convert proto types into the SDK types.
        let object_ref: ObjectReference = (&object_ref_proto).try_into()?;

        let merkle_proof_proto = inclusion_proof_proto
            .merkle_proof
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("inclusion_proof.merkle_proof"))?;
        let merkle_proof = sui_sdk_types::merkle::MerkleProof::try_from(merkle_proof_proto)?;
        let leaf_index = inclusion_proof_proto
            .leaf_index
            .ok_or_else(|| TryFromProtoError::missing("inclusion_proof.leaf_index"))?;
        let tree_root_bytes = inclusion_proof_proto
            .tree_root
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("inclusion_proof.tree_root"))?;
        let tree_root_arr: [u8; 32] = tree_root_bytes.as_ref().try_into().map_err(|_| {
            TryFromProtoError::invalid(
                "inclusion_proof.tree_root",
                format!("expected 32 bytes, got {}", tree_root_bytes.len()),
            )
        })?;
        let tree_root = Digest::new(tree_root_arr);

        // 3. BCS-decode the signed checkpoint summary, sanity-check
        //    sequence number, ratchet, then BLS-verify.
        let signed_summary: SignedCheckpointSummary = bcs::from_bytes(&summary_bytes)?;
        let summary_seq = signed_summary.checkpoint.sequence_number;
        if summary_seq != checkpoint_seq {
            return Err(LightClientError::CheckpointMismatch {
                requested: checkpoint_seq,
                returned: summary_seq,
            });
        }

        ratchet_to_checkpoint(&mut self.rpc, &mut self.cache, summary_seq).await?;

        let committee = self.cache.committee_for_checkpoint(summary_seq).ok_or(
            LightClientError::NoCommitteeForCheckpoint {
                checkpoint: summary_seq,
            },
        )?;
        let verifier = ValidatorCommitteeSignatureVerifier::new(committee.clone())?;
        verifier
            .verify_checkpoint_summary(&signed_summary.checkpoint, &signed_summary.signature)?;

        // 4. Sanity-check that the response's object id matches what
        //    was asked for. The proof would otherwise authenticate a
        //    different object, which would still pass cryptographic
        //    verification but isn't what the caller requested.
        if object_ref.object_id() != object_id {
            return Err(LightClientError::ObjectIdMismatch {
                requested: *object_id,
                returned: *object_ref.object_id(),
            });
        }

        // 5. Verify the OCS proof against the now-trusted summary.
        let inclusion_proof = OcsInclusionProof {
            merkle_proof,
            leaf_index,
            tree_root,
        };
        inclusion_proof.verify(&signed_summary.checkpoint, &object_ref)?;

        Ok(object_ref)
    }
}
