// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_sdk_types::CheckpointData;
use sui_sdk_types::Object;
use sui_sdk_types::ObjectReference;

use crate::proof::base::Proof;
use crate::proof::base::ProofContents;
use crate::proof::base::ProofTarget;
use crate::proof::error::ProofError;
use crate::proof::error::ProofResult;
use crate::proof::transaction_proof::TransactionProof;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObjectsTarget {
    pub objects: Vec<(ObjectReference, Object)>,
}

impl ObjectsTarget {
    pub fn construct(self, checkpoint: &CheckpointData) -> ProofResult<Proof> {
        let mut object_txs = self.objects.iter().map(|(_, o)| o.previous_transaction());

        let target_tx = object_txs.next().ok_or(ProofError::NoTargetsFound)?;

        if !object_txs.all(|tx| tx == target_tx) {
            return Err(ProofError::MultipleTransactionsNotSupported);
        }

        let tx_digest = checkpoint
            .transactions
            .iter()
            .find(|t| t.effects.transaction_digest() == &target_tx)
            .map(|t| t.effects.transaction_digest())
            .ok_or(ProofError::TransactionNotFound)?;

        let transaction_proof = TransactionProof::new(tx_digest, checkpoint, false)?;

        Ok(Proof {
            targets: ProofTarget::Objects(self.clone()),
            checkpoint_summary: checkpoint.checkpoint_summary.clone(),
            proof_contents: ProofContents::TransactionProof(Box::new(transaction_proof)),
        })
    }
}
