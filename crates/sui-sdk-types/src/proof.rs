//! OCS (Object Checkpoint State) proof verification.
//!
//! The Object Checkpoint State is the Blake2b256 Merkle tree built by each
//! checkpoint over the set of object references it modified, with leaves
//! arranged in ascending `ObjectID` order (see
//! [`crate::merkle`] for the underlying tree primitive). The tree's root
//! is committed to by the containing [`CheckpointSummary`] via the
//! [`CheckpointCommitment::CheckpointArtifacts`] variant of its
//! `checkpoint_commitments`.
//!
//! This module defines the proof envelopes that an SDK consumer verifies
//! against a *trusted* checkpoint summary:
//!
//! - [`OcsInclusionProof`] proves that a specific [`ObjectReference`]
//!   appears in the tree.
//! - [`OcsNonInclusionProof`] proves that no leaf with a given object
//!   id appears in the tree (the OCS is keyed by object id, so this is
//!   the natural notion of "the checkpoint did not modify this object").
//! - [`OcsProof`] tags one of the two.
//!
//! Verification only checks the data-relation half of the proof: it
//! reconstructs the `CheckpointArtifactsDigest` from the proof's `tree_root`
//! and asserts it matches the digest committed to by the summary's
//! `CheckpointArtifacts` commitment. Authenticating the checkpoint summary
//! itself (verifying its BLS aggregate signature against the epoch's
//! validator committee) is a separate step performed by the caller, e.g.
//! via `sui-crypto`'s `ValidatorCommitteeSignatureVerifier`.

use crate::Address;
use crate::CheckpointCommitment;
use crate::CheckpointSummary;
use crate::Digest;
use crate::ObjectReference;
use crate::hash::Hasher;
use crate::merkle::MerkleError;
use crate::merkle::MerkleNonInclusionProof;
use crate::merkle::MerkleProof;
use crate::merkle::Node;

/// An error returned by OCS proof verification.
#[derive(Debug, PartialEq, Eq)]
pub enum ProofError {
    /// The Merkle proof did not authenticate the leaf at the given index
    /// against the proof's claimed `tree_root`.
    InvalidMerkleProof,
    /// The checkpoint summary's `checkpoint_commitments` did not contain a
    /// `CheckpointArtifacts` entry — the summary cannot be used to anchor
    /// an OCS proof.
    MissingArtifactsDigest,
    /// The reconstructed `CheckpointArtifactsDigest` (computed from the
    /// proof's `tree_root`) did not match the digest committed to by the
    /// summary's `CheckpointArtifacts` commitment.
    ArtifactsDigestMismatch,
}

impl std::fmt::Display for ProofError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMerkleProof => f.write_str("invalid merkle proof"),
            Self::MissingArtifactsDigest => f.write_str(
                "checkpoint summary has no `CheckpointArtifacts` commitment to anchor the proof against",
            ),
            Self::ArtifactsDigestMismatch => f.write_str(
                "the checkpoint's `CheckpointArtifacts` digest does not match the proof's `tree_root`",
            ),
        }
    }
}

impl std::error::Error for ProofError {}

impl From<MerkleError> for ProofError {
    fn from(_: MerkleError) -> Self {
        Self::InvalidMerkleProof
    }
}

/// An OCS inclusion proof.
///
/// Proves that a specific [`ObjectReference`] appears at `leaf_index` in
/// the modified-objects Merkle tree whose root is `tree_root`, and that
/// `tree_root` is committed to by a [`CheckpointSummary`]'s
/// `CheckpointArtifacts` commitment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OcsInclusionProof {
    /// The Merkle inclusion proof for the leaf.
    pub merkle_proof: MerkleProof,
    /// The position of the leaf in the modified-objects tree.
    pub leaf_index: u64,
    /// The 32-byte Merkle root of the modified-objects tree.
    pub tree_root: Digest,
}

impl OcsInclusionProof {
    /// Verify that `object_ref` was written in the checkpoint described by
    /// `summary`.
    ///
    /// The caller is responsible for ensuring that `summary` itself is
    /// trusted (i.e. its BLS aggregate signature has been verified against
    /// the epoch's validator committee). This method only checks that the
    /// proof and the summary are consistent.
    pub fn verify(
        &self,
        summary: &CheckpointSummary,
        object_ref: &ObjectReference,
    ) -> Result<(), ProofError> {
        let tree_root_node = Node::Digest(*self.tree_root.inner());
        self.merkle_proof
            .verify_proof(&tree_root_node, object_ref, self.leaf_index as usize)?;
        check_summary_commits_to_tree_root(summary, &self.tree_root)?;
        Ok(())
    }
}

/// An OCS non-inclusion proof.
///
/// Proves that no leaf with a given object id appears in the
/// modified-objects Merkle tree whose root is `tree_root` — for a tree
/// built over `ObjectReference`s in sorted order — and that `tree_root`
/// is committed to by a [`CheckpointSummary`]'s `CheckpointArtifacts`
/// commitment.
///
/// Object-id non-inclusion is strictly stronger than reference
/// non-inclusion: the OCS keys leaves by `(object_id, version, digest)`
/// triples, and verifying that one specific triple is absent leaves
/// open the possibility that a different triple with the same object id
/// is in the tree. The bracketing check enforces that the left and
/// right neighbour leaves have object ids that strictly flank the
/// target id, which combined with the neighbours being at adjacent
/// indices in the sorted tree proves that no leaf under the target id
/// can be in the tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OcsNonInclusionProof {
    /// The Merkle non-inclusion proof, holding inclusion proofs for
    /// the bracketing neighbours of the target object id.
    pub non_inclusion_proof: MerkleNonInclusionProof<ObjectReference>,
    /// The 32-byte Merkle root of the modified-objects tree.
    pub tree_root: Digest,
}

impl OcsNonInclusionProof {
    /// Verify that no leaf with `object_id` appears in the OCS Merkle
    /// tree committed to by `summary`.
    ///
    /// As with [`OcsInclusionProof::verify`], the caller is responsible
    /// for ensuring `summary` itself is trusted.
    ///
    /// Stronger than verifying that a single `(object_id, version,
    /// digest)` triple is absent: the bracketing neighbours' object ids
    /// must strictly flank `object_id`, which combined with the
    /// neighbours being at adjacent indices in the sorted tree proves
    /// that no leaf with any version or digest under `object_id` is in
    /// the tree.
    pub fn verify(
        &self,
        summary: &CheckpointSummary,
        object_id: &Address,
    ) -> Result<(), ProofError> {
        let tree_root_node = Node::Digest(*self.tree_root.inner());
        self.non_inclusion_proof.verify_proof_by_key(
            &tree_root_node,
            object_id,
            ObjectReference::object_id,
        )?;
        check_summary_commits_to_tree_root(summary, &self.tree_root)?;
        Ok(())
    }
}

/// An OCS proof — either an inclusion proof or a non-inclusion proof.
///
/// The two variants have different verification inputs: inclusion
/// authenticates a specific [`ObjectReference`], while non-inclusion
/// authenticates an [`Address`] (object id). Callers that need to
/// dispatch on the variant should match on it directly.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum OcsProof {
    Inclusion(OcsInclusionProof),
    NonInclusion(OcsNonInclusionProof),
}

/// Locate the `CheckpointArtifacts` commitment on `summary`, reconstruct
/// the expected `CheckpointArtifactsDigest` from `tree_root`, and assert
/// equality.
fn check_summary_commits_to_tree_root(
    summary: &CheckpointSummary,
    tree_root: &Digest,
) -> Result<(), ProofError> {
    let artifacts_digest = summary
        .checkpoint_commitments
        .iter()
        .find_map(|c| match c {
            CheckpointCommitment::CheckpointArtifacts { digest } => Some(digest),
            _ => None,
        })
        .ok_or(ProofError::MissingArtifactsDigest)?;

    let expected = compute_checkpoint_artifacts_digest(std::slice::from_ref(tree_root));

    if &expected != artifacts_digest {
        return Err(ProofError::ArtifactsDigestMismatch);
    }
    Ok(())
}

/// Reconstruct the `CheckpointArtifactsDigest` from a slice of artifact
/// digests.
///
/// This mirrors upstream's
/// `sui_types::digests::CheckpointArtifactsDigest::from_artifact_digests`,
/// which is `BLAKE2b-256(bcs(Vec<Digest>))`. Both upstream's `Digest` and
/// this crate's [`Digest`] BCS-encode as a one-byte length prefix (`0x20`)
/// followed by 32 raw bytes, so a `Vec<Digest>` round-trips through BCS
/// byte-for-byte between the two crates.
///
/// For the current single-artifact OCS commitment scheme `artifact_digests`
/// is always a one-element slice containing the modified-objects
/// `tree_root`.
fn compute_checkpoint_artifacts_digest(artifact_digests: &[Digest]) -> Digest {
    let bytes =
        bcs::to_bytes(artifact_digests).expect("BCS of `&[Digest]` cannot fail for in-memory data");
    Hasher::digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Address;
    use crate::CheckpointSummary;
    use crate::GasCostSummary;
    use crate::merkle::MerkleTree;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    /// Construct a synthetic checkpoint summary that commits to the given
    /// `CheckpointArtifactsDigest`.
    fn summary_committing_to(artifacts_digest: Digest) -> CheckpointSummary {
        CheckpointSummary {
            epoch: 0,
            sequence_number: 0,
            network_total_transactions: 0,
            content_digest: Digest::ZERO,
            previous_digest: None,
            epoch_rolling_gas_cost_summary: GasCostSummary::default(),
            timestamp_ms: 0,
            checkpoint_commitments: vec![CheckpointCommitment::CheckpointArtifacts {
                digest: artifacts_digest,
            }],
            end_of_epoch_data: None,
            version_specific_data: vec![],
        }
    }

    /// Synthesize a sorted set of object references for testing.
    fn synthetic_refs(count: u8) -> Vec<ObjectReference> {
        (0..count)
            .map(|i| {
                let mut addr = [0u8; 32];
                addr[31] = i;
                let mut digest = [0u8; 32];
                digest[0] = i ^ 0x42;
                ObjectReference::new(Address::new(addr), u64::from(i) + 1, Digest::new(digest))
            })
            .collect()
    }

    /// End-to-end happy path: build a tree, get a proof, wrap it in
    /// `OcsInclusionProof`, construct a matching summary, verify.
    #[test]
    fn inclusion_proof_verifies_against_consistent_summary() {
        let refs = synthetic_refs(5);
        let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
        let tree_root = Digest::new(tree.root().bytes());
        let artifacts_digest = compute_checkpoint_artifacts_digest(&[tree_root]);
        let summary = summary_committing_to(artifacts_digest);

        for (index, object_ref) in refs.iter().enumerate() {
            let inclusion = OcsInclusionProof {
                merkle_proof: tree.get_proof(index).unwrap(),
                leaf_index: index as u64,
                tree_root,
            };
            inclusion.verify(&summary, object_ref).unwrap();
        }
    }

    /// Verifying against a leaf the proof was *not* generated for fails at
    /// the Merkle step.
    #[test]
    fn inclusion_proof_rejects_wrong_leaf() {
        let refs = synthetic_refs(5);
        let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
        let tree_root = Digest::new(tree.root().bytes());
        let artifacts_digest = compute_checkpoint_artifacts_digest(&[tree_root]);
        let summary = summary_committing_to(artifacts_digest);

        let inclusion = OcsInclusionProof {
            merkle_proof: tree.get_proof(0).unwrap(),
            leaf_index: 0,
            tree_root,
        };
        assert_eq!(
            inclusion.verify(&summary, &refs[1]),
            Err(ProofError::InvalidMerkleProof),
        );
    }

    /// A summary committing to the wrong `CheckpointArtifacts` digest is
    /// rejected at the digest-comparison step.
    #[test]
    fn inclusion_proof_rejects_summary_with_wrong_artifacts_digest() {
        let refs = synthetic_refs(3);
        let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
        let tree_root = Digest::new(tree.root().bytes());

        let bogus_artifacts_digest = Digest::new([0xff; 32]);
        let summary = summary_committing_to(bogus_artifacts_digest);

        let inclusion = OcsInclusionProof {
            merkle_proof: tree.get_proof(0).unwrap(),
            leaf_index: 0,
            tree_root,
        };
        assert_eq!(
            inclusion.verify(&summary, &refs[0]),
            Err(ProofError::ArtifactsDigestMismatch),
        );
    }

    /// A summary that has no `CheckpointArtifacts` commitment at all is
    /// rejected even before the digest comparison.
    #[test]
    fn inclusion_proof_rejects_summary_without_artifacts_commitment() {
        let refs = synthetic_refs(3);
        let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
        let tree_root = Digest::new(tree.root().bytes());

        let summary = CheckpointSummary {
            epoch: 0,
            sequence_number: 0,
            network_total_transactions: 0,
            content_digest: Digest::ZERO,
            previous_digest: None,
            epoch_rolling_gas_cost_summary: GasCostSummary::default(),
            timestamp_ms: 0,
            checkpoint_commitments: vec![],
            end_of_epoch_data: None,
            version_specific_data: vec![],
        };

        let inclusion = OcsInclusionProof {
            merkle_proof: tree.get_proof(0).unwrap(),
            leaf_index: 0,
            tree_root,
        };
        assert_eq!(
            inclusion.verify(&summary, &refs[0]),
            Err(ProofError::MissingArtifactsDigest),
        );
    }

    /// Synthesize an address whose 32nd byte is `byte`, mirroring the
    /// id layout used by [`synthetic_refs`].
    fn id(byte: u8) -> Address {
        let mut addr = [0u8; 32];
        addr[31] = byte;
        Address::new(addr)
    }

    /// Non-inclusion happy path: ask for a proof of an id that is not
    /// in the (sorted) tree, verify it.
    #[test]
    fn non_inclusion_proof_verifies_against_consistent_summary() {
        // refs have ids 0x00..0x04; pick a target between two of them.
        let refs = synthetic_refs(5);
        let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
        let tree_root = Digest::new(tree.root().bytes());
        let artifacts_digest = compute_checkpoint_artifacts_digest(&[tree_root]);
        let summary = summary_committing_to(artifacts_digest);

        // An `ObjectReference` carrying the missing id, used to drive
        // the underlying sorted-leaf bracketing computation. The
        // verifier only inspects the neighbours' ids, so the version
        // and digest on this synthetic ref don't matter.
        let missing_id = {
            let mut addr = [0u8; 32];
            addr[31] = 0x80;
            Address::new(addr)
        };
        let probe = ObjectReference::new(missing_id, 0, Digest::new([0u8; 32]));
        assert!(refs.iter().all(|r| r.object_id() != &missing_id));

        let non_inclusion_proof = tree.compute_non_inclusion_proof(&refs, &probe).unwrap();
        let proof = OcsNonInclusionProof {
            non_inclusion_proof,
            tree_root,
        };
        proof.verify(&summary, &missing_id).unwrap();
    }

    /// Non-inclusion fails when applied to an id that *is* in the
    /// tree: the bracketing neighbours can't strictly flank it.
    #[test]
    fn non_inclusion_proof_rejects_present_id() {
        let refs = synthetic_refs(5);
        let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
        let tree_root = Digest::new(tree.root().bytes());
        let artifacts_digest = compute_checkpoint_artifacts_digest(&[tree_root]);
        let summary = summary_committing_to(artifacts_digest);

        // Build a non-inclusion proof against a target adjacent (in
        // full-reference sort order) to refs[1] but sharing no id with
        // any leaf, then attempt to reuse the proof against refs[1]'s
        // id itself.
        let neighbour_probe = ObjectReference::new(id(0x80), 0, Digest::new([0u8; 32]));
        let non_inclusion_proof = tree
            .compute_non_inclusion_proof(&refs, &neighbour_probe)
            .unwrap();
        let proof = OcsNonInclusionProof {
            non_inclusion_proof,
            tree_root,
        };
        assert_eq!(
            proof.verify(&summary, refs[1].object_id()),
            Err(ProofError::InvalidMerkleProof),
        );
    }

    /// A non-inclusion proof whose bracketing neighbours admit the
    /// target's id (even though they admit some other version/digest
    /// with the same id) is rejected. This is the strictly-stronger
    /// guarantee that id-level non-inclusion provides over
    /// reference-level non-inclusion: a tree containing
    /// `(target_id, v, d)` cannot produce a non-inclusion proof for
    /// `target_id`, even if a synthetic reference with that id and a
    /// smaller version would sort-bracket cleanly.
    #[test]
    fn non_inclusion_rejects_id_strict_bracketing_violation() {
        // Construct a tree whose three leaves include one with the
        // target id at a specific version. Sorted by (id, version,
        // digest), that leaf is the only leaf with the target id.
        let target_id = id(0x42);
        let mut refs = vec![
            ObjectReference::new(id(0x00), 1, Digest::new([0x11; 32])),
            ObjectReference::new(target_id, 5, Digest::new([0x22; 32])),
            ObjectReference::new(id(0x80), 9, Digest::new([0x33; 32])),
        ];
        refs.sort();
        let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
        let tree_root = Digest::new(tree.root().bytes());
        let artifacts_digest = compute_checkpoint_artifacts_digest(&[tree_root]);
        let summary = summary_committing_to(artifacts_digest);

        // A reference that shares `target_id` but with a smaller
        // version would sort *before* the present `(target_id, 5, _)`
        // leaf, so the underlying merkle bracketing would succeed.
        // The id-level check must catch this.
        let synthetic_lower = ObjectReference::new(target_id, 0, Digest::new([0u8; 32]));
        let non_inclusion_proof = tree
            .compute_non_inclusion_proof(&refs, &synthetic_lower)
            .unwrap();

        let proof = OcsNonInclusionProof {
            non_inclusion_proof,
            tree_root,
        };
        assert_eq!(
            proof.verify(&summary, &target_id),
            Err(ProofError::InvalidMerkleProof),
        );
    }

    /// Pin the BCS shape of `compute_checkpoint_artifacts_digest` for the
    /// single-artifact case: `BLAKE2b-256(ULEB128(1) || 0x20 || 32 bytes)`.
    /// The `0x20` is the length prefix on the embedded `Digest`; upstream
    /// emits the same bytes because its `Digest` is also a 33-byte
    /// length-prefixed BCS value.
    #[test]
    fn checkpoint_artifacts_digest_single_artifact_shape() {
        let tree_root = Digest::new([0u8; 32]);
        let mut expected_input = vec![0x01u8, 0x20u8];
        expected_input.extend_from_slice(tree_root.inner());
        let expected = Hasher::digest(&expected_input);

        let actual = compute_checkpoint_artifacts_digest(&[tree_root]);
        assert_eq!(actual, expected);
    }

    /// Cross-implementation pin: the artifacts digest for a one-element
    /// `Vec<Digest>` containing the all-zero digest must match the value
    /// produced by upstream
    /// `sui_types::digests::CheckpointArtifactsDigest::from_artifact_digests`,
    /// captured from a local run of upstream as base58
    /// `Hu1Kq6yF9jGgTd5o9Tav3saEFSzTg7ZKehYqa8QvQXGE`.
    #[test]
    fn checkpoint_artifacts_digest_matches_upstream_for_zero_input() {
        let actual = compute_checkpoint_artifacts_digest(&[Digest::ZERO]);
        let expected = Digest::from_base58("Hu1Kq6yF9jGgTd5o9Tav3saEFSzTg7ZKehYqa8QvQXGE").unwrap();
        assert_eq!(actual, expected);
    }

    /// Property-based tests for OCS proof verification. These cover the
    /// composition of [`MerkleTree`] construction,
    /// [`compute_checkpoint_artifacts_digest`], and the proof envelopes
    /// over arbitrary leaf sets — complementing the synthetic spot
    /// checks above and the real-chain fixture tests in
    /// `tests/ocs_fixture.rs`.
    #[cfg(feature = "proptest")]
    mod proptests {
        use super::*;

        use proptest::collection::vec;
        use proptest::prelude::*;
        use test_strategy::proptest;

        // See the matching comment in `merkle::tests::proptests` for why
        // this explicit binding is needed on wasm.
        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen_test::wasm_bindgen_test as test;

        /// Derive an [`ObjectReference`] from a u32 seed. The seed lands
        /// in the address's last 4 bytes (the high bytes are zero) so
        /// that sorting by `ObjectReference` agrees with sorting by the
        /// seed, which keeps the sorted-leaves invariant cheap to
        /// reason about in the non-inclusion property below.
        fn synthetic_ref(seed: u32) -> ObjectReference {
            let mut addr = [0u8; 32];
            addr[28..32].copy_from_slice(&seed.to_be_bytes());
            let mut digest = [0u8; 32];
            digest[..4].copy_from_slice(&seed.to_le_bytes());
            ObjectReference::new(
                Address::new(addr),
                u64::from(seed).max(1),
                Digest::new(digest),
            )
        }

        /// Sorted, deduplicated `Vec<ObjectReference>` of length 1..=32,
        /// generated from distinct u32 seeds. This is the shape an OCS
        /// tree is built over.
        fn sorted_unique_refs() -> impl Strategy<Value = Vec<ObjectReference>> {
            vec(any::<u32>(), 1..=32).prop_map(|mut seeds| {
                seeds.sort();
                seeds.dedup();
                seeds.into_iter().map(synthetic_ref).collect()
            })
        }

        /// For any non-empty leaf set, an [`OcsInclusionProof`]
        /// constructed by the canonical recipe (build tree, get_proof,
        /// wrap with `tree_root`) verifies against a summary that
        /// commits to the matching artifacts digest.
        #[proptest]
        fn ocs_inclusion_proof_round_trips(
            #[strategy(sorted_unique_refs())] refs: Vec<ObjectReference>,
        ) {
            let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
            let tree_root = Digest::new(tree.root().bytes());
            let artifacts_digest = compute_checkpoint_artifacts_digest(&[tree_root]);
            let summary = summary_committing_to(artifacts_digest);

            for (index, object_ref) in refs.iter().enumerate() {
                let proof = OcsInclusionProof {
                    merkle_proof: tree.get_proof(index).unwrap(),
                    leaf_index: index as u64,
                    tree_root,
                };
                proof.verify(&summary, object_ref).unwrap();
            }
        }

        /// A summary that commits to the wrong artifacts digest is
        /// always rejected at the digest-comparison step, regardless of
        /// the proof's merkle correctness. This pins the
        /// [`ProofError::ArtifactsDigestMismatch`] return path for
        /// arbitrary inputs.
        #[proptest]
        fn ocs_inclusion_proof_rejects_summary_with_wrong_artifacts_digest(
            #[strategy(sorted_unique_refs())] refs: Vec<ObjectReference>,
            #[strategy(any::<[u8; 32]>())] bogus_artifacts: [u8; 32],
        ) {
            let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
            let tree_root = Digest::new(tree.root().bytes());
            let correct_artifacts_digest = compute_checkpoint_artifacts_digest(&[tree_root]);
            // Skip the (negligible) case where the random bogus digest
            // happens to equal the correct one — the proof would
            // then verify, which is the right behaviour but not the
            // case under test.
            prop_assume!(bogus_artifacts != correct_artifacts_digest.into_inner());

            let summary = summary_committing_to(Digest::new(bogus_artifacts));
            let proof = OcsInclusionProof {
                merkle_proof: tree.get_proof(0).unwrap(),
                leaf_index: 0,
                tree_root,
            };
            prop_assert_eq!(
                proof.verify(&summary, &refs[0]),
                Err(ProofError::ArtifactsDigestMismatch),
            );
        }

        /// For any sorted leaf set and any target that does not appear
        /// in it, an [`OcsNonInclusionProof`] verifies against the
        /// summary that commits to the matching artifacts digest.
        #[proptest]
        fn ocs_non_inclusion_proof_round_trips(
            #[strategy(sorted_unique_refs())] refs: Vec<ObjectReference>,
            #[strategy(any::<u32>())] target_seed: u32,
        ) {
            // Skip the case where the synthetic target collides with a
            // seed already in the leaf set — the API would refuse to
            // build a non-inclusion proof for a present leaf, and the
            // round-trip we want to test doesn't apply.
            prop_assume!(
                refs.iter()
                    .all(|r| r.object_id().inner()[28..32] != target_seed.to_be_bytes())
            );

            let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
            let tree_root = Digest::new(tree.root().bytes());
            let artifacts_digest = compute_checkpoint_artifacts_digest(&[tree_root]);
            let summary = summary_committing_to(artifacts_digest);

            let target = synthetic_ref(target_seed);
            let non_inclusion = tree.compute_non_inclusion_proof(&refs, &target).unwrap();
            let proof = OcsNonInclusionProof {
                non_inclusion_proof: non_inclusion,
                tree_root,
            };
            proof.verify(&summary, target.object_id()).unwrap();
        }
    }
}
