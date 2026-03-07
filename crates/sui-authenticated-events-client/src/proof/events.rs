// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;
use sui_sdk_types::CheckpointData;
use sui_sdk_types::Event;

use crate::proof::base::Proof;
use crate::proof::base::ProofBuilder;
use crate::proof::base::ProofContents;
use crate::proof::base::ProofTarget;
use crate::proof::error::ProofError;
use crate::proof::error::ProofResult;
use crate::proof::transaction_proof::TransactionProof;
use crate::types::EventId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventsTarget {
    pub events: Vec<(EventId, Event)>,
}

impl ProofBuilder for EventsTarget {
    fn construct(self, checkpoint: &CheckpointData) -> ProofResult<Proof> {
        let mut event_txs = self.events.iter().map(|(eid, _)| &eid.tx_digest);

        let target_tx = event_txs.next().ok_or(ProofError::NoTargetsFound)?;

        if !event_txs.all(|tx| tx == target_tx) {
            return Err(ProofError::MultipleTransactionsNotSupported);
        }

        let transaction_proof = TransactionProof::new(target_tx, checkpoint, true)?;

        Ok(Proof {
            targets: ProofTarget::Events(self),
            checkpoint_summary: checkpoint.checkpoint_summary.clone(),
            proof_contents: ProofContents::TransactionProof(transaction_proof),
        })
    }
}
