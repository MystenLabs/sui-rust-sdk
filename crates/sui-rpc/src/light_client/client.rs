//! The end-to-end `LightClient` facade.

use sui_crypto::bls12381::ValidatorCommitteeSignatureVerifier;
use sui_sdk_types::Address;
use sui_sdk_types::Digest;
use sui_sdk_types::Object;
use sui_sdk_types::ObjectReference;
use sui_sdk_types::SignedCheckpointSummary;
use sui_sdk_types::ValidatorCommittee;
use sui_sdk_types::proof::OcsInclusionProof;

use crate::Client;
use crate::field::FieldMask;
use crate::field::FieldMaskUtil;
use crate::proto::TryFromProtoError;
use crate::proto::sui::rpc::v2::GetCheckpointRequest;
use crate::proto::sui::rpc::v2alpha::GetOcsInclusionProofRequest;

use super::EpochCache;
use super::error::LightClientError;
use super::error::ObjectDataMismatch;
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

    /// Return the sequence number of the network's most recent
    /// checkpoint, as reported by `LedgerService.GetCheckpoint(latest)`.
    ///
    /// The result is **not** trust-anchored — it's a single read from
    /// the server with no signature verification. Streaming clients use
    /// it as a starting cursor (e.g., "begin reading events from the
    /// checkpoint after this one"); any subsequent claim about an
    /// object's state at this checkpoint must still flow through
    /// [`Self::verify_object_at_checkpoint`] for cryptographic
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

    /// Verify that `object_id` was written in the checkpoint with
    /// `checkpoint_seq`, returning the authenticated [`ObjectReference`]
    /// and the BCS-decoded [`Object`] data on success.
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
    /// 6. BCS-decode the response's `object_data` into an [`Object`] and
    ///    check that its `(object_id, version, digest)` matches the
    ///    authenticated `ObjectReference`. This pins the object's
    ///    contents to the verified leaf rather than trusting the server
    ///    to send the right bytes.
    ///
    /// On success the returned `ObjectReference` and `Object` are
    /// authenticated end-to-end: the reference's version and digest are
    /// the values the network committed to at the requested checkpoint,
    /// and the object's BCS bytes hash to that same digest.
    pub async fn verify_object_at_checkpoint(
        &mut self,
        object_id: &Address,
        checkpoint_seq: u64,
    ) -> Result<(ObjectReference, Object), LightClientError> {
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
        let object_data_bytes = response
            .object_data
            .ok_or_else(|| TryFromProtoError::missing("object_data"))?;

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

        // 6. BCS-decode the returned object data and confirm it matches
        //    the now-trusted object reference. A misbehaving server
        //    that wanted to lie about the object's contents would have
        //    to either send bytes whose `(id, version, digest)` differ
        //    from the leaf (caught here) or whose `Object::digest()`
        //    no longer matches the bytes themselves (caught by the
        //    digest equality check).
        let object: Object = bcs::from_bytes(&object_data_bytes)?;
        let returned_ref =
            ObjectReference::new(object.object_id(), object.version(), object.digest());
        if returned_ref != object_ref {
            return Err(LightClientError::ObjectDataMismatch(Box::new(
                ObjectDataMismatch {
                    expected: object_ref,
                    returned: returned_ref,
                },
            )));
        }

        Ok((object_ref, object))
    }
}
