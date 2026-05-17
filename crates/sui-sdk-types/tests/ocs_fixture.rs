//! Known-answer tests for OCS proof verification against a real Sui
//! checkpoint.
//!
//! The fixture in `tests/fixtures/ocs/1137.chk` is checkpoint 1137 of an
//! upstream OCS-bringup network, captured byte-for-byte from the network's
//! checkpoint store. Its summary carries a
//! [`CheckpointCommitment::CheckpointArtifacts`] commitment to the
//! modified-objects Merkle tree root, which is exactly what
//! [`OcsInclusionProof`] and [`OcsNonInclusionProof`] verify against.
//!
//! These tests pin the end-to-end pipeline against real chain data: a
//! regression in [`TransactionEffects::object_changes`], the
//! modified-objects filter applied below, [`MerkleTree`] construction,
//! leaf BCS layout, the artifacts-digest derivation, or either proof
//! envelope will break one of these assertions.
//!
//! The fixture was lifted from PR #223 of MystenLabs/sui-rust-sdk
//! (`crates/sui-authenticated-events-client/tests/test_files/ocs/1137.chk`
//! at commit 25a17590), where it was originally captured for the
//! relocated authenticated-events client.
//!
//! [`TransactionEffects::object_changes`]: sui_sdk_types::TransactionEffects::object_changes

#![cfg(feature = "unstable")]

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use sui_sdk_types::Address;
use sui_sdk_types::CheckpointCommitment;
use sui_sdk_types::CheckpointData;
use sui_sdk_types::CheckpointSummary;
use sui_sdk_types::Digest;
use sui_sdk_types::ObjectChange;
use sui_sdk_types::ObjectReference;
use sui_sdk_types::merkle::MerkleTree;
use sui_sdk_types::proof::OcsInclusionProof;
use sui_sdk_types::proof::OcsNonInclusionProof;

/// Load the checkpoint fixture.
///
/// The file is a bare `bcs(CheckpointData)` blob — no version prefix.
fn load_checkpoint() -> CheckpointData {
    let bytes = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/ocs/1137.chk",
    ))
    .expect("fixture file must be readable");
    bcs::from_bytes(&bytes).expect("fixture must deserialize as CheckpointData")
}

/// Build the sorted modified-objects leaf set for the checkpoint.
///
/// This is the OCS tree contract: each leaf is the latest
/// [`ObjectReference`] (after the checkpoint's transactions are applied)
/// for every object that was written — created, mutated, or unwrapped.
/// Tombstone changes (deleted, wrapped, unwrapped-then-deleted) do not
/// appear in the OCS tree.
///
/// When the same object is touched more than once in a checkpoint, the
/// later (higher-version) state wins. Leaves are emitted in ascending
/// `ObjectID` order, which is what the non-inclusion proof scheme relies on.
fn build_modified_objects(checkpoint: &CheckpointData) -> Vec<ObjectReference> {
    let mut latest: BTreeMap<Address, (u64, Digest)> = BTreeMap::new();
    for tx in &checkpoint.transactions {
        for change in tx.effects.object_changes() {
            let (object_id, version, digest) = match change {
                ObjectChange::Created {
                    object_id,
                    output_version,
                    output_digest,
                    ..
                }
                | ObjectChange::Mutated {
                    object_id,
                    output_version,
                    output_digest,
                    ..
                }
                | ObjectChange::Unwrapped {
                    object_id,
                    output_version,
                    output_digest,
                    ..
                } => (*object_id, output_version, *output_digest),
                ObjectChange::Deleted { .. }
                | ObjectChange::Wrapped { .. }
                | ObjectChange::UnwrappedThenDeleted { .. } => continue,
                _ => continue,
            };
            // Keep the highest-version write per object id: V1 effects
            // drain `created` before `mutated`, so out-of-order versions
            // can appear in the iteration, and the OCS tree commits to
            // the final state.
            latest
                .entry(object_id)
                .and_modify(|(v, d)| {
                    if version > *v {
                        *v = version;
                        *d = digest;
                    }
                })
                .or_insert((version, digest));
        }
    }
    latest
        .into_iter()
        .map(|(id, (v, d))| ObjectReference::new(id, v, d))
        .collect()
}

/// Extract the `CheckpointArtifacts` digest committed to by `summary`.
fn artifacts_commitment(summary: &CheckpointSummary) -> Digest {
    summary
        .checkpoint_commitments
        .iter()
        .find_map(|c| match c {
            CheckpointCommitment::CheckpointArtifacts { digest } => Some(*digest),
            _ => None,
        })
        .expect("fixture checkpoint must carry a CheckpointArtifacts commitment")
}

/// Recompute the artifacts digest the same way the chain does:
/// `BLAKE2b-256(bcs(Vec<Digest>))` over a single-element vector
/// containing the tree root. Mirrors `proof::compute_checkpoint_artifacts_digest`
/// (module-private) so the test does not need to expose it.
fn recompute_artifacts_digest(tree_root: Digest) -> Digest {
    let bytes = bcs::to_bytes(&vec![tree_root]).expect("BCS of Vec<Digest> cannot fail");
    sui_sdk_types::hash::Hasher::digest(bytes)
}

#[test]
fn fixture_artifacts_digest_matches_on_chain_commitment() {
    let checkpoint = load_checkpoint();

    // The OCS commitment scheme commits to exactly one artifact (the
    // modified-objects tree root). A future protocol change that adds a
    // second artifact would invalidate the single-element assumption in
    // `recompute_artifacts_digest`.
    assert_eq!(
        checkpoint
            .checkpoint_summary
            .checkpoint
            .checkpoint_commitments
            .len(),
        1,
        "fixture checkpoint should carry exactly one commitment",
    );

    let leaves = build_modified_objects(&checkpoint);
    assert!(!leaves.is_empty(), "fixture should have modified objects");

    let tree = MerkleTree::build_from_unserialized(leaves.iter()).unwrap();
    let tree_root = Digest::new(tree.root().bytes());

    let on_chain = artifacts_commitment(&checkpoint.checkpoint_summary.checkpoint);
    let recomputed = recompute_artifacts_digest(tree_root);
    assert_eq!(
        recomputed, on_chain,
        "recomputed artifacts digest must match the chain's commitment",
    );
}

#[test]
fn fixture_inclusion_proof_verifies_for_every_modified_object() {
    let checkpoint = load_checkpoint();
    let summary = &checkpoint.checkpoint_summary.checkpoint;

    let leaves = build_modified_objects(&checkpoint);
    let tree = MerkleTree::build_from_unserialized(leaves.iter()).unwrap();
    let tree_root = Digest::new(tree.root().bytes());

    for (index, object_ref) in leaves.iter().enumerate() {
        let proof = OcsInclusionProof {
            merkle_proof: tree.get_proof(index).unwrap(),
            leaf_index: index as u64,
            tree_root,
        };
        proof
            .verify(summary, object_ref)
            .unwrap_or_else(|e| panic!("leaf {index} should verify but got {e:?}"));
    }
}

#[test]
fn fixture_inclusion_proof_rejects_wrong_leaf() {
    let checkpoint = load_checkpoint();
    let summary = &checkpoint.checkpoint_summary.checkpoint;

    let leaves = build_modified_objects(&checkpoint);
    assert!(
        leaves.len() >= 2,
        "negative inclusion test needs a second leaf to swap in",
    );
    let tree = MerkleTree::build_from_unserialized(leaves.iter()).unwrap();
    let tree_root = Digest::new(tree.root().bytes());

    let proof = OcsInclusionProof {
        merkle_proof: tree.get_proof(0).unwrap(),
        leaf_index: 0,
        tree_root,
    };
    // Pointing the proof at the wrong leaf should fail at the Merkle step,
    // before the artifacts-digest comparison ever runs.
    let err = proof.verify(summary, &leaves[1]).unwrap_err();
    assert_eq!(
        format!("{err}"),
        "invalid merkle proof",
        "wrong-leaf verification should fail at the merkle step",
    );
}

#[test]
fn fixture_non_inclusion_proof_verifies_for_absent_object() {
    let checkpoint = load_checkpoint();
    let summary = &checkpoint.checkpoint_summary.checkpoint;

    let leaves = build_modified_objects(&checkpoint);
    let tree = MerkleTree::build_from_unserialized(leaves.iter()).unwrap();
    let tree_root = Digest::new(tree.root().bytes());

    // Candidate ids picked to land in different positions of the sorted
    // leaf list: low (before the first leaf), high (after the last leaf),
    // and a mid-range value. Version and digest are dummies — the
    // non-inclusion proof only depends on the id-level ordering.
    let candidate_ids: &[&str] = &[
        "0x0000000000000000000000000000000000000000000000000000000000000001",
        "0x7f00000000000000000000000000000000000000000000000000000000000000",
        "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    ];
    let present: BTreeSet<Address> = leaves.iter().map(|r| *r.object_id()).collect();

    let mut verified = 0usize;
    for raw in candidate_ids {
        let id: Address = raw.parse().unwrap();
        if present.contains(&id) {
            // Vanishingly unlikely for the synthetic ids above; skip
            // rather than asserting on a fixture-specific id absence.
            continue;
        }
        let target = ObjectReference::new(id, 0, Digest::ZERO);
        let non_inclusion = tree.compute_non_inclusion_proof(&leaves, &target).unwrap();
        let proof = OcsNonInclusionProof {
            non_inclusion_proof: non_inclusion,
            tree_root,
        };
        proof
            .verify(summary, target.object_id())
            .unwrap_or_else(|e| panic!("absent id {raw} should verify but got {e:?}"));
        verified += 1;
    }
    assert!(
        verified > 0,
        "at least one synthetic candidate id must have been absent",
    );
}

#[test]
fn fixture_non_inclusion_proof_rejects_present_leaf() {
    let checkpoint = load_checkpoint();
    let summary = &checkpoint.checkpoint_summary.checkpoint;

    let leaves = build_modified_objects(&checkpoint);
    let tree = MerkleTree::build_from_unserialized(leaves.iter()).unwrap();
    let tree_root = Digest::new(tree.root().bytes());

    // Pick a target that is absent but neighbours a present leaf, then
    // try to reuse the resulting non-inclusion proof against that present
    // leaf. The proof's neighbour comparison should reject it.
    let present_id = *leaves[1].object_id();
    let mut absent_id_bytes = *present_id.inner();
    absent_id_bytes[31] = absent_id_bytes[31].wrapping_sub(1);
    let absent_id = Address::new(absent_id_bytes);
    if leaves.iter().any(|r| r.object_id() == &absent_id) {
        // If the synthetic neighbour collides with a real id, skip rather
        // than asserting on fixture-specific structure.
        return;
    }

    let absent_target = ObjectReference::new(absent_id, 0, Digest::ZERO);
    let non_inclusion = tree
        .compute_non_inclusion_proof(&leaves, &absent_target)
        .unwrap();
    let proof = OcsNonInclusionProof {
        non_inclusion_proof: non_inclusion,
        tree_root,
    };
    let err = proof.verify(summary, leaves[1].object_id()).unwrap_err();
    assert_eq!(
        format!("{err}"),
        "invalid merkle proof",
        "reusing a non-inclusion proof against a present leaf should be rejected",
    );
}
