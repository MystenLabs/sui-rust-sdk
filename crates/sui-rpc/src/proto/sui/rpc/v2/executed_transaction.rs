use super::*;
use crate::field::FieldMaskTree;
use crate::merge::Merge;

impl Merge<&ExecutedTransaction> for ExecutedTransaction {
    fn merge(&mut self, source: &ExecutedTransaction, mask: &FieldMaskTree) {
        let ExecutedTransaction {
            digest,
            transaction,
            signatures,
            effects,
            events,
            checkpoint,
            timestamp,
            balance_changes,
            objects,
        } = source;

        if mask.contains(Self::DIGEST_FIELD.name) {
            self.digest = digest.clone();
        }

        if let Some(submask) = mask.subtree(Self::TRANSACTION_FIELD.name) {
            self.transaction = transaction
                .as_ref()
                .map(|t| Transaction::merge_from(t, &submask));
        }

        if let Some(submask) = mask.subtree(Self::SIGNATURES_FIELD.name) {
            self.signatures = signatures
                .iter()
                .map(|s| UserSignature::merge_from(s, &submask))
                .collect();
        }

        if let Some(submask) = mask.subtree(Self::EFFECTS_FIELD.name) {
            self.effects = effects
                .as_ref()
                .map(|e| TransactionEffects::merge_from(e, &submask));
        }

        if let Some(submask) = mask.subtree(Self::EVENTS_FIELD.name) {
            self.events = events
                .as_ref()
                .map(|events| TransactionEvents::merge_from(events, &submask));
        }

        if mask.contains(Self::CHECKPOINT_FIELD.name) {
            self.checkpoint = *checkpoint;
        }

        if mask.contains(Self::TIMESTAMP_FIELD.name) {
            self.timestamp = *timestamp;
        }

        if mask.contains(Self::BALANCE_CHANGES_FIELD.name) {
            self.balance_changes = balance_changes.clone();
        }

        if let Some(submask) = mask.subtree(Self::OBJECTS_FIELD) {
            self.objects = objects
                .as_ref()
                .map(|objects| ObjectSet::merge_from(objects, &submask));
        }
    }
}
