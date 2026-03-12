// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;
use sui_sdk_types::Address;
use sui_sdk_types::CheckpointCommitment;
use sui_sdk_types::CheckpointData;
use sui_sdk_types::CheckpointSummary;
use sui_sdk_types::Digest;
use sui_sdk_types::hash::Hasher;
use sui_sdk_types::ObjectReference;

use crate::proof::base::Proof;
use crate::proof::base::ProofContents;
use crate::proof::base::ProofTarget;
use crate::proof::error::ProofError;
use crate::proof::error::ProofResult;

type MerkleTree = sui_crypto::merkle::MerkleTree;
type MerkleProof = sui_crypto::merkle::MerkleProof;
type MerkleNonInclusionProof = sui_crypto::merkle::MerkleNonInclusionProof<ObjectReference>;
type Node = sui_crypto::merkle::Node;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCSTarget {
    pub object_ref: ObjectReference,
    pub target_type: OCSTargetType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OCSTargetType {
    Inclusion,
    NonInclusion,
}

impl OCSTarget {
    pub fn new_non_inclusion_target(id: Address) -> Self {
        Self {
            object_ref: ObjectReference::new(id, 0, Digest::ZERO),
            target_type: OCSTargetType::NonInclusion,
        }
    }

    pub fn new_inclusion_target(object_ref: ObjectReference) -> Self {
        Self {
            object_ref,
            target_type: OCSTargetType::Inclusion,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OCSInclusionProof {
    pub merkle_proof: MerkleProof,
    pub leaf_index: usize,
    pub tree_root: Digest,
}

impl OCSInclusionProof {
    pub fn verify(&self, target: &OCSTarget) -> ProofResult<()> {
        if target.target_type != OCSTargetType::Inclusion {
            return Err(ProofError::MismatchedTargetAndProofType);
        }

        self.merkle_proof
            .verify_proof_with_unserialized_leaf(
                &Node::from(self.tree_root.into_inner()),
                &target.object_ref,
                self.leaf_index,
            )
            .map_err(|_| ProofError::InvalidProof)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OCSNonInclusionProof {
    pub non_inclusion_proof: MerkleNonInclusionProof,
    pub tree_root: Digest,
}

impl OCSNonInclusionProof {
    pub fn verify(&self, target: &OCSTarget) -> ProofResult<()> {
        if target.target_type != OCSTargetType::NonInclusion {
            return Err(ProofError::MismatchedTargetAndProofType);
        }

        self.non_inclusion_proof
            .verify_proof(&Node::from(self.tree_root.into_inner()), &target.object_ref)
            .map_err(|_| ProofError::InvalidProof)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OCSProof {
    Inclusion(OCSInclusionProof),
    NonInclusion(OCSNonInclusionProof),
}

impl OCSProof {
    pub fn get_artifact_digest(&self) -> Digest {
        match self {
            OCSProof::Inclusion(proof) => proof.tree_root,
            OCSProof::NonInclusion(proof) => proof.tree_root,
        }
    }
}

fn checkpoint_artifacts_digest(summary: &CheckpointSummary) -> ProofResult<&Digest> {
    summary
        .checkpoint_commitments
        .iter()
        .find_map(|c| match c {
            CheckpointCommitment::CheckpointArtifacts { digest } => Some(digest),
            _ => None,
        })
        .ok_or_else(|| {
            ProofError::GeneralError(
                "Checkpoint artifacts digest not found in checkpoint commitments".to_string(),
            )
        })
}

fn compute_artifacts_digest(digests: Vec<Digest>) -> ProofResult<Digest> {
    let bytes = bcs::to_bytes(&digests)
        .map_err(|e| ProofError::GeneralError(format!("BCS error: {}", e)))?;
    Ok(Hasher::digest(&bytes))
}

#[derive(Debug)]
pub struct ModifiedObjectTree {
    pub leaves: Vec<ObjectReference>,
    pub tree: MerkleTree,
    pub tree_root: Digest,
    pub object_pos_map: BTreeMap<Address, usize>,
}

impl ModifiedObjectTree {
    pub fn from_checkpoint(checkpoint: &CheckpointData) -> ProofResult<Self> {
        let mut latest_object_states: BTreeMap<Address, (u64, Digest)> = BTreeMap::new();
        for tx in &checkpoint.transactions {
            for obj_ref in tx.effects.changed_objects() {
                let id = *obj_ref.object_id();
                let version = obj_ref.version();
                let digest = *obj_ref.digest();
                if let Some((old_version, _)) = latest_object_states.get(&id) {
                    if *old_version >= version {
                        return Err(ProofError::GeneralError(
                            "Object states should be monotonically increasing".to_string(),
                        ));
                    }
                }
                latest_object_states.insert(id, (version, digest));
            }
        }

        let mut object_pos_map = BTreeMap::new();
        let leaves: Vec<ObjectReference> = latest_object_states
            .iter()
            .enumerate()
            .map(|(i, (id, (version, digest)))| {
                object_pos_map.insert(*id, i);
                ObjectReference::new(*id, *version, *digest)
            })
            .collect();

        let tree = MerkleTree::build_from_unserialized(leaves.clone())
            .map_err(|e| ProofError::GeneralError(e.to_string()))?;
        let tree_root = Digest::new(tree.root().bytes());

        Ok(ModifiedObjectTree {
            leaves,
            object_pos_map,
            tree,
            tree_root,
        })
    }

    pub fn get_inclusion_proof(
        &self,
        object_ref: &ObjectReference,
    ) -> ProofResult<OCSInclusionProof> {
        let id = *object_ref.object_id();
        let index = self
            .object_pos_map
            .get(&id)
            .ok_or_else(|| ProofError::GeneralError(format!("Object ID {} not found", id)))?;

        if self.leaves[*index] != *object_ref {
            return Err(ProofError::GeneralError(format!(
                "Input object ref {:?} does not match the actual ref {:?}",
                object_ref, self.leaves[*index]
            )));
        }

        let proof = self
            .tree
            .get_proof(*index)
            .map_err(|e| ProofError::GeneralError(e.to_string()))?;
        Ok(OCSInclusionProof {
            merkle_proof: proof,
            leaf_index: *index,
            tree_root: self.tree_root,
        })
    }

    pub fn get_non_inclusion_proof(
        &self,
        object_ref: &ObjectReference,
    ) -> ProofResult<OCSNonInclusionProof> {
        let id = *object_ref.object_id();
        if self.object_pos_map.contains_key(&id) {
            return Err(ProofError::GeneralError(format!(
                "Object ID {} is in checkpoint",
                id
            )));
        }

        let non_inclusion_proof = self
            .tree
            .compute_non_inclusion_proof(&self.leaves, object_ref)
            .map_err(|e| ProofError::GeneralError(e.to_string()))?;
        Ok(OCSNonInclusionProof {
            non_inclusion_proof,
            tree_root: self.tree_root,
        })
    }
}

impl OCSTarget {
    pub fn construct(self, checkpoint: &CheckpointData) -> ProofResult<Proof> {
        let modified_object_tree = ModifiedObjectTree::from_checkpoint(checkpoint)?;
        match self.target_type {
            OCSTargetType::Inclusion => {
                let proof = modified_object_tree.get_inclusion_proof(&self.object_ref)?;
                Ok(Proof {
                    targets: ProofTarget::ObjectCheckpointState(self),
                    checkpoint_summary: checkpoint.checkpoint_summary.clone(),
                    proof_contents: ProofContents::ObjectCheckpointStateProof(OCSProof::Inclusion(
                        proof,
                    )),
                })
            }
            OCSTargetType::NonInclusion => {
                let proof = modified_object_tree.get_non_inclusion_proof(&self.object_ref)?;
                Ok(Proof {
                    targets: ProofTarget::ObjectCheckpointState(self),
                    checkpoint_summary: checkpoint.checkpoint_summary.clone(),
                    proof_contents: ProofContents::ObjectCheckpointStateProof(
                        OCSProof::NonInclusion(proof),
                    ),
                })
            }
        }
    }
}

impl OCSProof {
    pub fn verify(self, target: &ProofTarget, summary: &CheckpointSummary) -> ProofResult<()> {
        match target {
            ProofTarget::ObjectCheckpointState(target) => {
                let actual_artifacts_digest = checkpoint_artifacts_digest(summary)?;

                let expected_artifacts_digest =
                    compute_artifacts_digest(vec![self.get_artifact_digest()])?;

                if expected_artifacts_digest != *actual_artifacts_digest {
                    return Err(ProofError::ArtifactDigestMismatch);
                }

                match self {
                    OCSProof::Inclusion(proof) => proof.verify(target),
                    OCSProof::NonInclusion(proof) => proof.verify(target),
                }
            }
            _ => Err(ProofError::MismatchedTargetAndProofType),
        }
    }
}
