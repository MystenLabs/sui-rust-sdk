// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;
use sui_sdk_types::CheckpointData;
use sui_sdk_types::CheckpointSummary;
use sui_sdk_types::SignedCheckpointSummary;
use sui_sdk_types::ValidatorCommittee;

use crate::proof::base::Proof;
use crate::proof::base::ProofBuilder;
use crate::proof::base::ProofContents;
use crate::proof::base::ProofContentsVerifier;
use crate::proof::base::ProofTarget;
use crate::proof::error::ProofError;
use crate::proof::error::ProofResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitteeTarget {
    pub committee: ValidatorCommittee,
}

impl ProofBuilder for CommitteeTarget {
    fn construct(self, checkpoint: &CheckpointData) -> ProofResult<Proof> {
        if checkpoint.checkpoint_summary.checkpoint.epoch + 1 != self.committee.epoch {
            return Err(ProofError::EpochMismatch);
        }

        if checkpoint
            .checkpoint_summary
            .checkpoint
            .end_of_epoch_data
            .is_none()
        {
            return Err(ProofError::ExpectedEndOfEpochCheckpoint);
        }

        Ok(Proof {
            targets: ProofTarget::Committee(self),
            checkpoint_summary: checkpoint.checkpoint_summary.clone(),
            proof_contents: ProofContents::CommitteeProof(CommitteeProof {}),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitteeProof {}

impl ProofContentsVerifier for CommitteeProof {
    fn verify(self, targets: &ProofTarget, summary: &CheckpointSummary) -> ProofResult<()> {
        match targets {
            ProofTarget::Committee(target) => {
                let new_committee = extract_new_committee_info(summary)?;
                if new_committee.epoch != target.committee.epoch
                    || new_committee.members != target.committee.members
                {
                    return Err(ProofError::CommitteeMismatch);
                }
                Ok(())
            }
            _ => Err(ProofError::MismatchedTargetAndProofType),
        }
    }
}

pub fn extract_new_committee_info(summary: &CheckpointSummary) -> ProofResult<ValidatorCommittee> {
    if let Some(ref end_of_epoch_data) = summary.end_of_epoch_data {
        let next_epoch = summary
            .epoch
            .checked_add(1)
            .ok_or(ProofError::EpochAddOverflow)?;
        Ok(ValidatorCommittee {
            epoch: next_epoch,
            members: end_of_epoch_data.next_epoch_committee.clone(),
        })
    } else {
        Err(ProofError::ExpectedEndOfEpochCheckpoint)
    }
}

pub fn extract_new_committee_info_from_signed(
    summary: &SignedCheckpointSummary,
) -> ProofResult<ValidatorCommittee> {
    extract_new_committee_info(&summary.checkpoint)
}
