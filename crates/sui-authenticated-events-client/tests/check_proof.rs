// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_authenticated_events_client::proof::base::Proof;
use sui_authenticated_events_client::proof::base::ProofBuilder;
use sui_authenticated_events_client::proof::base::ProofContents;
use sui_authenticated_events_client::proof::base::ProofTarget;
use sui_authenticated_events_client::proof::base::ProofVerifier;
use sui_authenticated_events_client::proof::committee::CommitteeProof;
use sui_authenticated_events_client::proof::committee::extract_new_committee_info;
use sui_authenticated_events_client::proof::objects::ObjectsTarget;
use sui_sdk_types::CheckpointData;
use sui_sdk_types::EventId;
use sui_sdk_types::ValidatorCommittee;

use std::path::PathBuf;

fn read_full_checkpoint(checkpoint_path: &PathBuf) -> CheckpointData {
    let buffer = std::fs::read(checkpoint_path).unwrap();
    let (_version, data): (u8, CheckpointData) =
        bcs::from_bytes(&buffer).expect("Unable to parse checkpoint file");
    data
}

fn read_data(committee_seq: u64, seq: u64) -> (ValidatorCommittee, CheckpointData) {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(format!("tests/test_files/{}.chk", committee_seq));
    let committee_checkpoint = read_full_checkpoint(&d);
    let committee =
        extract_new_committee_info(&committee_checkpoint.checkpoint_summary.checkpoint).unwrap();

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(format!("tests/test_files/{}.chk", seq));
    let full_checkpoint = read_full_checkpoint(&d);

    (committee, full_checkpoint)
}

#[test]
fn check_can_read_test_data() {
    let (_committee, full_checkpoint) = read_data(15918264, 16005062);
    assert!(
        full_checkpoint
            .checkpoint_summary
            .checkpoint
            .end_of_epoch_data
            .is_some()
    );
}

#[test]
fn test_new_committee() {
    let (committee, full_checkpoint) = read_data(15918264, 16005062);

    let new_committee =
        extract_new_committee_info(&full_checkpoint.checkpoint_summary.checkpoint).unwrap();

    let target = ProofTarget::new_committee(new_committee);
    let committee_proof = target.construct(&full_checkpoint).unwrap();

    assert!(committee_proof.verify(&committee).is_ok());
}

#[test]
fn test_incorrect_new_committee() {
    let (committee, full_checkpoint) = read_data(15918264, 16005062);

    let committee_proof = Proof {
        checkpoint_summary: full_checkpoint.checkpoint_summary.clone(),
        proof_contents: ProofContents::CommitteeProof(CommitteeProof {}),
        targets: ProofTarget::new_committee(committee.clone()), // WRONG
    };

    assert!(committee_proof.verify(&committee).is_err());
}

#[test]
fn test_fail_incorrect_cert() {
    let (_committee, full_checkpoint) = read_data(15918264, 16005062);

    let new_committee =
        extract_new_committee_info(&full_checkpoint.checkpoint_summary.checkpoint).unwrap();

    let target = ProofTarget::new_committee(new_committee.clone());
    let committee_proof = target.construct(&full_checkpoint).unwrap();

    assert!(committee_proof.verify(&new_committee).is_err());
}

#[test]
fn test_object_target_fail_no_data() {
    let (committee, full_checkpoint) = read_data(15918264, 16005062);

    let sample_object = full_checkpoint.transactions[0].output_objects[0].clone();
    let sample_ref = sui_sdk_types::ObjectReference::new(
        sample_object.object_id(),
        sample_object.version(),
        sample_object.digest(),
    );

    let bad_proof = Proof {
        checkpoint_summary: full_checkpoint.checkpoint_summary.clone(),
        proof_contents: ProofContents::CommitteeProof(CommitteeProof {}), // WRONG
        targets: ProofTarget::Objects(ObjectsTarget {
            objects: vec![(sample_ref, sample_object)],
        }),
    };

    assert!(bad_proof.verify(&committee).is_err());
}

#[test]
fn test_object_target_success() {
    let (committee, full_checkpoint) = read_data(15918264, 16005062);

    let sample_object = full_checkpoint.transactions[0].output_objects[0].clone();
    let sample_ref = sui_sdk_types::ObjectReference::new(
        sample_object.object_id(),
        sample_object.version(),
        sample_object.digest(),
    );

    let target = ProofTarget::Objects(ObjectsTarget {
        objects: vec![(sample_ref, sample_object)],
    });
    let object_proof = target.construct(&full_checkpoint).unwrap();

    assert!(object_proof.verify(&committee).is_ok());
}

#[test]
fn test_object_target_fail_wrong_object() {
    let (committee, full_checkpoint) = read_data(15918264, 16005062);

    let sample_object = full_checkpoint.transactions[0].output_objects[0].clone();
    let wrong_object = full_checkpoint.transactions[1].output_objects[1].clone();
    let wrong_ref = sui_sdk_types::ObjectReference::new(
        wrong_object.object_id(),
        wrong_object.version(),
        wrong_object.digest(),
    );

    let target = ProofTarget::new_objects(vec![(wrong_ref, sample_object.clone())]); // WRONG
    let object_proof = target.construct(&full_checkpoint).unwrap();
    assert!(object_proof.verify(&committee).is_err());

    let bad_ref = sui_sdk_types::ObjectReference::new(
        sample_object.object_id(),
        sample_object.version() + 1, // WRONG
        sample_object.digest(),
    );

    let target = ProofTarget::new_objects(vec![(bad_ref, sample_object)]);
    let object_proof = target.construct(&full_checkpoint).unwrap();
    assert!(object_proof.verify(&committee).is_err());
}

#[test]
fn test_event_target_fail_no_data() {
    let (committee, full_checkpoint) = read_data(15918264, 16005062);

    let sample_event = full_checkpoint.transactions[1].events.as_ref().unwrap().0[0].clone();
    let sample_eid = EventId {
        tx_digest: *full_checkpoint.transactions[1].effects.transaction_digest(),
        event_seq: 0,
    };

    let bad_proof = Proof {
        checkpoint_summary: full_checkpoint.checkpoint_summary.clone(),
        proof_contents: ProofContents::CommitteeProof(CommitteeProof {}), // WRONG
        targets: ProofTarget::new_events(vec![(sample_eid, sample_event)]),
    };

    assert!(bad_proof.verify(&committee).is_err());
}

#[test]
fn test_event_target_success() {
    let (committee, full_checkpoint) = read_data(15918264, 16005062);

    let sample_event = full_checkpoint.transactions[1].events.as_ref().unwrap().0[0].clone();
    let sample_eid = EventId {
        tx_digest: *full_checkpoint.transactions[1].effects.transaction_digest(),
        event_seq: 0,
    };

    let target = ProofTarget::new_events(vec![(sample_eid, sample_event)]);
    let event_proof = target.construct(&full_checkpoint).unwrap();

    assert!(event_proof.verify(&committee).is_ok());
}

#[test]
fn test_event_target_fail_bad_event() {
    let (committee, full_checkpoint) = read_data(15918264, 16005062);

    let sample_event = full_checkpoint.transactions[1].events.as_ref().unwrap().0[0].clone();
    let sample_eid = EventId {
        tx_digest: *full_checkpoint.transactions[1].effects.transaction_digest(),
        event_seq: 1, // WRONG
    };

    let target = ProofTarget::new_events(vec![(sample_eid, sample_event)]);
    let event_proof = target.construct(&full_checkpoint).unwrap();

    assert!(event_proof.verify(&committee).is_err());
}
