// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use sui_authenticated_events_client::proof::base::Proof;
use sui_authenticated_events_client::proof::base::ProofBuilder;
use sui_authenticated_events_client::proof::base::ProofTarget;
use sui_authenticated_events_client::proof::base::ProofVerifier;
use sui_authenticated_events_client::proof::ocs::ModifiedObjectTree;
use sui_sdk_types::Address;
use sui_sdk_types::CheckpointCommitment;
use sui_sdk_types::CheckpointData;
use sui_sdk_types::Digest;
use sui_sdk_types::ValidatorCommittee;

fn load_genesis_committee() -> ValidatorCommittee {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/test_files/ocs/committee.bcs");
    let bytes = std::fs::read(&path).unwrap();
    bcs::from_bytes(&bytes).expect("Failed to deserialize committee")
}

fn load_checkpoint() -> CheckpointData {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/test_files/ocs/1137.chk");
    let bytes = std::fs::read(&path).unwrap();
    bcs::from_bytes(&bytes).unwrap()
}

fn load_test_data() -> (CheckpointData, ValidatorCommittee) {
    let committee = load_genesis_committee();
    let checkpoint = load_checkpoint();
    (checkpoint, committee)
}

#[test]
fn test_derive_artifacts() {
    let (checkpoint, _) = load_test_data();

    let tree = ModifiedObjectTree::from_checkpoint(&checkpoint).unwrap();

    assert_eq!(
        checkpoint
            .checkpoint_summary
            .checkpoint
            .checkpoint_commitments
            .len(),
        1
    );

    let expected_artifacts_digest = match &checkpoint
        .checkpoint_summary
        .checkpoint
        .checkpoint_commitments[0]
    {
        CheckpointCommitment::CheckpointArtifacts { digest } => digest,
        _ => panic!("Expected CheckpointArtifacts commitment"),
    };

    let computed_artifacts_digest = {
        use blake2::digest::Digest as _;
        use blake2::digest::consts::U32;
        let bytes = bcs::to_bytes(&vec![tree.tree_root]).unwrap();
        Digest::new(blake2::Blake2b::<U32>::digest(&bytes).into())
    };
    assert_eq!(computed_artifacts_digest, *expected_artifacts_digest);
}

#[test]
fn test_get_object_inclusion_proof() {
    let (checkpoint, committee) = load_test_data();
    let tree = ModifiedObjectTree::from_checkpoint(&checkpoint).unwrap();

    let leaf = &tree.leaves[0];
    let object_ref = sui_sdk_types::ObjectReference::new(leaf.0, leaf.1, leaf.2);

    let target = ProofTarget::new_ocs_inclusion(object_ref);
    let proof = target.construct(&checkpoint).unwrap();

    assert!(proof.verify(&committee).is_ok());
}

#[test]
fn test_get_object_non_inclusion_proof() {
    let (checkpoint, committee) = load_test_data();
    let tree = ModifiedObjectTree::from_checkpoint(&checkpoint).unwrap();

    let test_ids: Vec<Address> = [
        "0x0000000000000000000000000000000000000000000000000000000000000001",
        "0x0000000000000000000000000000000000000000000000000000000000000456",
        "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        "0x0000000000000000000000000000000000000000000000000000000000000007",
    ]
    .iter()
    .map(|id| id.parse().unwrap())
    .collect();

    for id in &test_ids {
        let target = ProofTarget::new_ocs_non_inclusion(*id);
        let proof = target.construct(&checkpoint);
        if tree.object_pos_map.contains_key(id) {
            assert!(
                proof.is_err(),
                "Should not be able to get non-inclusion proof for included object"
            );
        } else {
            let proof = proof.expect("Should be able to get non-inclusion proof");
            assert!(proof.verify(&committee).is_ok());
        }
    }
}

#[test]
fn test_modified_object_tree_properties() {
    let (checkpoint, _) = load_test_data();
    let tree = ModifiedObjectTree::from_checkpoint(&checkpoint).unwrap();

    assert_eq!(tree.object_pos_map.len(), tree.leaves.len());

    for (i, leaf) in tree.leaves.iter().enumerate() {
        assert_eq!(tree.object_pos_map.get(&leaf.0), Some(&i));
    }

    for window in tree.leaves.windows(2) {
        assert!(window[0].0 < window[1].0, "Objects should be sorted by ID");
    }
}

#[test]
fn test_serialization_roundtrip() {
    let (checkpoint, committee) = load_test_data();
    let tree = ModifiedObjectTree::from_checkpoint(&checkpoint).unwrap();

    let leaf = &tree.leaves[0];
    let object_ref = sui_sdk_types::ObjectReference::new(leaf.0, leaf.1, leaf.2);

    let target = ProofTarget::new_ocs_inclusion(object_ref);
    let proof = target.construct(&checkpoint).unwrap();

    let json_serialized = serde_json::to_string(&proof).expect("Should serialize to JSON");
    let json_deserialized: Proof =
        serde_json::from_str(&json_serialized).expect("Should deserialize from JSON");
    assert!(json_deserialized.verify(&committee).is_ok());

    let bcs_serialized = bcs::to_bytes(&proof).expect("Should serialize to BCS");
    let bcs_deserialized: Proof =
        bcs::from_bytes(&bcs_serialized).expect("Should deserialize from BCS");
    assert!(bcs_deserialized.verify(&committee).is_ok());

    // Non-inclusion proof roundtrip
    let non_existent_id: Address =
        "0x0000000000000000000000000000000000000000000000000000000000999999"
            .parse()
            .unwrap();
    if !tree.object_pos_map.contains_key(&non_existent_id) {
        let target = ProofTarget::new_ocs_non_inclusion(non_existent_id);
        let proof = target.construct(&checkpoint).unwrap();

        let json_serialized = serde_json::to_string(&proof).unwrap();
        let json_deserialized: Proof = serde_json::from_str(&json_serialized).unwrap();
        assert!(json_deserialized.verify(&committee).is_ok());

        let bcs_serialized = bcs::to_bytes(&proof).unwrap();
        let bcs_deserialized: Proof = bcs::from_bytes(&bcs_serialized).unwrap();
        assert!(bcs_deserialized.verify(&committee).is_ok());
    }
}
