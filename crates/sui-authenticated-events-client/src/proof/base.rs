// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;
use sui_sdk_types::CheckpointData;
use sui_sdk_types::CheckpointSummary;
use sui_sdk_types::Event;
use sui_sdk_types::Object;
use sui_sdk_types::ObjectReference;
use sui_sdk_types::SignedCheckpointSummary;
use sui_sdk_types::ValidatorCommittee;

use crate::proof::committee::CommitteeProof;
use crate::proof::committee::CommitteeTarget;
use crate::proof::error::ProofError;
use crate::proof::error::ProofResult;
use crate::proof::events::EventsTarget;
use crate::proof::objects::ObjectsTarget;
use crate::proof::ocs::OCSProof;
use crate::proof::ocs::OCSTarget;
use crate::proof::transaction_proof::TransactionProof;
use crate::types::EventId;

pub trait ProofBuilder {
    fn construct(self, checkpoint: &CheckpointData) -> ProofResult<Proof>;
}

pub trait ProofVerifier {
    fn verify(self, committee: &ValidatorCommittee) -> ProofResult<()>;
}

pub trait ProofContentsVerifier {
    fn verify(self, targets: &ProofTarget, summary: &CheckpointSummary) -> ProofResult<()>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProofTarget {
    Objects(ObjectsTarget),
    Events(EventsTarget),
    Committee(CommitteeTarget),
    ObjectCheckpointState(OCSTarget),
}

impl ProofTarget {
    pub fn new_objects(objects: Vec<(ObjectReference, Object)>) -> Self {
        ProofTarget::Objects(ObjectsTarget { objects })
    }

    pub fn new_events(events: Vec<(EventId, Event)>) -> Self {
        ProofTarget::Events(EventsTarget { events })
    }

    pub fn new_committee(committee: ValidatorCommittee) -> Self {
        ProofTarget::Committee(CommitteeTarget { committee })
    }

    pub fn new_ocs_inclusion(object_ref: ObjectReference) -> Self {
        ProofTarget::ObjectCheckpointState(OCSTarget::new_inclusion_target(object_ref))
    }

    pub fn new_ocs_non_inclusion(object_id: sui_sdk_types::Address) -> Self {
        ProofTarget::ObjectCheckpointState(OCSTarget::new_non_inclusion_target(object_id))
    }
}

impl ProofBuilder for ProofTarget {
    fn construct(self, checkpoint: &CheckpointData) -> ProofResult<Proof> {
        match self {
            ProofTarget::Objects(target) => target.construct(checkpoint),
            ProofTarget::Events(target) => target.construct(checkpoint),
            ProofTarget::Committee(target) => target.construct(checkpoint),
            ProofTarget::ObjectCheckpointState(target) => target.construct(checkpoint),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {
    pub targets: ProofTarget,
    pub checkpoint_summary: SignedCheckpointSummary,
    pub proof_contents: ProofContents,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ProofContents {
    TransactionProof(TransactionProof),
    CommitteeProof(CommitteeProof),
    ObjectCheckpointStateProof(OCSProof),
}

impl ProofVerifier for Proof {
    fn verify(self, committee: &ValidatorCommittee) -> ProofResult<()> {
        use sui_crypto::bls12381::ValidatorCommitteeSignatureVerifier;

        let verifier = ValidatorCommitteeSignatureVerifier::new(committee.clone())
            .map_err(|e| ProofError::SummaryVerificationFailed(e.to_string()))?;
        verifier
            .verify_checkpoint_summary(
                &self.checkpoint_summary.checkpoint,
                &self.checkpoint_summary.signature,
            )
            .map_err(|e| ProofError::SummaryVerificationFailed(e.to_string()))?;

        let verified_summary = &self.checkpoint_summary.checkpoint;

        match &self.targets {
            ProofTarget::Objects(_) | ProofTarget::Events(_) => {
                if !matches!(self.proof_contents, ProofContents::TransactionProof(_)) {
                    return Err(ProofError::MismatchedTargetAndProofType);
                }
            }
            ProofTarget::Committee(_) => {
                if !matches!(self.proof_contents, ProofContents::CommitteeProof(_)) {
                    return Err(ProofError::MismatchedTargetAndProofType);
                }
            }
            ProofTarget::ObjectCheckpointState(_) => {
                if !matches!(
                    self.proof_contents,
                    ProofContents::ObjectCheckpointStateProof(_)
                ) {
                    return Err(ProofError::MismatchedTargetAndProofType);
                }
            }
        }

        self.proof_contents.verify(&self.targets, verified_summary)
    }
}

impl ProofContentsVerifier for ProofContents {
    fn verify(self, targets: &ProofTarget, summary: &CheckpointSummary) -> ProofResult<()> {
        match self {
            ProofContents::TransactionProof(proof) => proof.verify(targets, summary),
            ProofContents::CommitteeProof(proof) => proof.verify(targets, summary),
            ProofContents::ObjectCheckpointStateProof(proof) => proof.verify(targets, summary),
        }
    }
}
