// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;
use sui_sdk_types::CheckpointContents;
use sui_sdk_types::CheckpointData;
use sui_sdk_types::CheckpointSummary;
use sui_sdk_types::Digest;
use sui_sdk_types::Object;
use sui_sdk_types::ObjectReference;
use sui_sdk_types::SignedTransaction;
use sui_sdk_types::TransactionEffects;
use sui_sdk_types::TransactionEvents;

use crate::proof::base::ProofContentsVerifier;
use crate::proof::base::ProofTarget;
use crate::proof::error::ProofError;
use crate::proof::error::ProofResult;
use crate::types::EventId;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionProof {
    pub checkpoint_contents: CheckpointContents,
    pub transaction: SignedTransaction,
    pub effects: TransactionEffects,
    pub events: Option<TransactionEvents>,
}

impl TransactionProof {
    pub fn new(
        tx_digest: &Digest,
        checkpoint: &CheckpointData,
        add_events: bool,
    ) -> ProofResult<Self> {
        let tx = checkpoint
            .transactions
            .iter()
            .find(|t| &t.transaction.transaction.digest() == tx_digest)
            .ok_or(ProofError::TransactionNotFound)?;

        Ok(Self {
            checkpoint_contents: checkpoint.checkpoint_contents.clone(),
            transaction: tx.transaction.clone(),
            effects: tx.effects.clone(),
            events: if add_events { tx.events.clone() } else { None },
        })
    }

    fn verify_objects(&self, target_objects: &[(ObjectReference, Object)]) -> ProofResult<()> {
        let changed_objects = self.effects.changed_objects();

        for (object_ref, object) in target_objects {
            let computed_ref =
                ObjectReference::new(object.object_id(), object.version(), object.digest());
            if *object_ref != computed_ref {
                return Err(ProofError::ObjectReferenceMismatch);
            }

            changed_objects
                .iter()
                .find(|effects_object_ref| effects_object_ref == &object_ref)
                .ok_or(ProofError::ObjectNotFound)?;
        }
        Ok(())
    }

    fn verify_events(
        &self,
        target_events: &[(EventId, sui_sdk_types::Event)],
        tx_digest: &Digest,
    ) -> ProofResult<()> {
        let effects_events_digest = self.effects.events_digest();
        let computed_events_digest = self.events.as_ref().map(|e| e.digest());

        if effects_events_digest != computed_events_digest.as_ref() {
            return Err(ProofError::EventsDigestMismatch);
        }

        match &self.events {
            Some(tx_events) => {
                for (event_id, event) in target_events {
                    if &event_id.tx_digest != tx_digest {
                        return Err(ProofError::EventTransactionMismatch);
                    }

                    if event_id.event_seq as usize >= tx_events.0.len() {
                        return Err(ProofError::EventSequenceOutOfBounds);
                    }

                    if tx_events.0[event_id.event_seq as usize] != *event {
                        return Err(ProofError::EventContentsMismatch);
                    }
                }
            }
            None => {
                if !target_events.is_empty() {
                    return Err(ProofError::EventsMissing);
                }
            }
        }

        Ok(())
    }
}

impl ProofContentsVerifier for TransactionProof {
    fn verify(self, targets: &ProofTarget, summary: &CheckpointSummary) -> ProofResult<()> {
        let contents_digest = self.checkpoint_contents.digest();
        if contents_digest != summary.content_digest {
            return Err(ProofError::ContentsDigestMismatch);
        }

        let tx_digest = self.transaction.transaction.digest();
        let effects_tx_digest = self.effects.transaction_digest();

        if &tx_digest != effects_tx_digest {
            return Err(ProofError::TransactionDigestMismatch);
        }

        let effects_digest = self.effects.digest();

        if !self
            .checkpoint_contents
            .transactions()
            .iter()
            .any(|info| info.transaction() == &tx_digest && info.effects() == &effects_digest)
        {
            return Err(ProofError::TransactionDigestNotFound);
        }

        match targets {
            ProofTarget::Objects(target) => self.verify_objects(&target.objects),
            ProofTarget::Events(target) => self.verify_events(&target.events, &tx_digest),
            _ => Err(ProofError::MismatchedTargetAndProofType),
        }
    }
}
