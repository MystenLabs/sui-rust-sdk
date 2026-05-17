//! Known-answer test for BLS aggregate-signature verification on a real
//! Sui checkpoint summary.
//!
//! This test is the BLS half of the OCS-fixture pin in
//! `crates/sui-sdk-types/tests/ocs_fixture.rs`. Together the two
//! checkpoints' tests demonstrate that we can independently:
//!
//! 1. Verify a checkpoint summary's BLS aggregate signature against the
//!    epoch's [`ValidatorCommittee`] (here).
//!
//! 2. Recompute the on-chain `CheckpointArtifacts` digest from the
//!    checkpoint's modified-object state and verify OCS inclusion /
//!    non-inclusion proofs against the resulting summary (in the
//!    sui-sdk-types test).
//!
//! The fixtures are shared with the sui-sdk-types test and live at
//! `crates/sui-sdk-types/tests/fixtures/ocs/`. They originate from
//! PR #223 of MystenLabs/sui-rust-sdk at commit 25a17590.

#![cfg(feature = "bls12381")]

use sui_crypto::bls12381::ValidatorCommitteeSignatureVerifier;
use sui_sdk_types::CheckpointData;
use sui_sdk_types::ValidatorCommittee;

/// Path to a fixture file in the sister `sui-sdk-types` crate's tests
/// directory. We avoid duplicating the ~234 KB checkpoint blob across
/// crates by referencing it via `CARGO_MANIFEST_DIR` + a relative hop.
macro_rules! sister_fixture {
    ($rel:literal) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../sui-sdk-types/tests/fixtures/ocs/",
            $rel,
        )
    };
}

fn load_committee() -> ValidatorCommittee {
    let bytes = std::fs::read(sister_fixture!("committee.bcs"))
        .expect("committee fixture must be readable");
    bcs::from_bytes(&bytes).expect("committee fixture must deserialize")
}

fn load_checkpoint() -> CheckpointData {
    let bytes =
        std::fs::read(sister_fixture!("1137.chk")).expect("checkpoint fixture must be readable");
    bcs::from_bytes(&bytes).expect("checkpoint fixture must deserialize")
}

#[test]
fn fixture_committee_verifies_signed_checkpoint_summary() {
    let committee = load_committee();
    let checkpoint = load_checkpoint();

    // The fixture is from epoch 0 (the bringup network's genesis epoch),
    // which matches the committee committed to at genesis. If a future
    // fixture rotation drops in a non-genesis checkpoint, the epoch check
    // in `ValidatorCommitteeSignatureVerifier::verify` will surface the
    // mismatch with a clear error before any cryptographic work happens.
    assert_eq!(
        checkpoint.checkpoint_summary.checkpoint.epoch, committee.epoch,
        "fixture checkpoint and committee must agree on epoch",
    );

    let verifier =
        ValidatorCommitteeSignatureVerifier::new(committee).expect("committee should be valid");
    verifier
        .verify_checkpoint_summary(
            &checkpoint.checkpoint_summary.checkpoint,
            &checkpoint.checkpoint_summary.signature,
        )
        .expect("genesis committee should verify the signed summary");
}

#[test]
fn fixture_summary_signature_rejects_tampered_summary() {
    let committee = load_committee();
    let checkpoint = load_checkpoint();

    let mut summary = checkpoint.checkpoint_summary.checkpoint.clone();
    // Flip a load-bearing field: changing the sequence number alters the
    // BCS-encoded signing message, so the aggregated BLS signature should
    // no longer verify.
    summary.sequence_number = summary.sequence_number.wrapping_add(1);

    let verifier =
        ValidatorCommitteeSignatureVerifier::new(committee).expect("committee should be valid");
    assert!(
        verifier
            .verify_checkpoint_summary(&summary, &checkpoint.checkpoint_summary.signature)
            .is_err(),
        "tampered summary must not verify",
    );
}
