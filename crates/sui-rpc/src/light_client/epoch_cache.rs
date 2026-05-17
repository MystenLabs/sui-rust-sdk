use std::sync::Arc;

use sui_sdk_types::ValidatorCommittee;

use super::error::LightClientError;

/// A cache of validator committees indexed by epoch.
///
/// The cache always knows about the *current* committee (the one in
/// effect at the network's most recently observed epoch) and remembers
/// each completed epoch's committee. Lookups via
/// [`Self::committee_for_epoch`] return the committee that was active
/// during the requested epoch, or `None` if the epoch falls outside
/// the half-open range `[start_epoch, current_epoch]`.
///
/// The cache is advanced by calling [`Self::apply_ratchet_update`] with
/// the committee that takes effect for the next epoch. The driver in
/// `super::ratchet` is responsible for fetching and BLS-verifying the
/// end-of-epoch summaries that feed these updates.
///
/// Committees are stored behind `Arc` so lookups don't have to clone
/// the (potentially ~100-member) committee body. Callers receive an
/// `Arc<ValidatorCommittee>` they can clone cheaply.
#[derive(Debug, Clone)]
pub struct EpochCache {
    /// Committees for `[start_epoch, current_epoch)`, one entry per
    /// epoch in ascending order. `completed_committees[i]` covers
    /// `start_epoch + i`.
    completed_committees: Vec<Arc<ValidatorCommittee>>,

    /// The epoch number of `completed_committees[0]`, or — if the
    /// vector is empty — the epoch of `current_committee` (i.e. the
    /// only committee the cache knows about).
    start_epoch: u64,

    /// The committee in effect for the current (open-ended) epoch.
    current_committee: Arc<ValidatorCommittee>,
}

impl EpochCache {
    /// Build a fresh cache seeded with the genesis committee. The
    /// genesis committee is assumed to be in effect for epoch 0 until
    /// the first ratchet update is applied.
    pub fn new(genesis_committee: ValidatorCommittee) -> Self {
        Self {
            completed_committees: Vec::new(),
            start_epoch: 0,
            current_committee: Arc::new(genesis_committee),
        }
    }

    /// The committee in effect for the current epoch.
    pub fn current_committee(&self) -> &ValidatorCommittee {
        &self.current_committee
    }

    /// The epoch number the cache is currently tracking.
    pub fn current_epoch(&self) -> u64 {
        self.current_committee.epoch
    }

    /// Look up the validator committee that was in effect during
    /// `epoch`.
    ///
    /// Returns `None` if `epoch` falls outside the range the cache
    /// knows about — either before `start_epoch` or strictly after
    /// [`Self::current_epoch`].
    pub fn committee_for_epoch(&self, epoch: u64) -> Option<Arc<ValidatorCommittee>> {
        if epoch == self.current_epoch() {
            return Some(self.current_committee.clone());
        }
        if epoch < self.start_epoch {
            return None;
        }
        let idx = usize::try_from(epoch - self.start_epoch).ok()?;
        self.completed_committees.get(idx).cloned()
    }

    /// Advance the cache: move the current epoch into
    /// `completed_committees` and install `new_committee` as the new
    /// current epoch.
    ///
    /// `new_committee.epoch` must be exactly one greater than the
    /// cache's current epoch.
    pub fn apply_ratchet_update(
        &mut self,
        new_committee: ValidatorCommittee,
    ) -> Result<(), LightClientError> {
        let current_epoch = self.current_epoch();
        if new_committee.epoch != current_epoch + 1 {
            return Err(LightClientError::InvalidEpochAdvance {
                current: current_epoch,
                provided: new_committee.epoch,
            });
        }

        let completed = std::mem::replace(&mut self.current_committee, Arc::new(new_committee));
        self.completed_committees.push(completed);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use sui_sdk_types::ValidatorCommittee;

    fn committee(epoch: u64) -> ValidatorCommittee {
        ValidatorCommittee {
            epoch,
            members: Vec::new(),
        }
    }

    /// Fresh cache reports the genesis committee for epoch 0 and
    /// nothing else.
    #[test]
    fn fresh_cache_returns_genesis_only_for_epoch_zero() {
        let cache = EpochCache::new(committee(0));
        assert_eq!(cache.current_epoch(), 0);
        assert_eq!(cache.committee_for_epoch(0).map(|c| c.epoch), Some(0));
        assert!(cache.committee_for_epoch(1).is_none());
        assert!(cache.committee_for_epoch(1_000_000).is_none());
    }

    /// After one ratchet update, the previous committee covers its own
    /// epoch and the new committee covers the next.
    #[test]
    fn single_ratchet_records_completed_epoch() {
        let mut cache = EpochCache::new(committee(0));
        cache.apply_ratchet_update(committee(1)).unwrap();

        assert_eq!(cache.current_epoch(), 1);
        assert_eq!(cache.committee_for_epoch(0).map(|c| c.epoch), Some(0));
        assert_eq!(cache.committee_for_epoch(1).map(|c| c.epoch), Some(1));
        assert!(cache.committee_for_epoch(2).is_none());
    }

    /// Lookups across many completed epochs land on the right entry
    /// in O(1) time.
    #[test]
    fn lookup_indexes_into_completed_committees() {
        let mut cache = EpochCache::new(committee(0));
        for epoch in 1..=4 {
            cache.apply_ratchet_update(committee(epoch)).unwrap();
        }

        for epoch in 0..=4 {
            assert_eq!(
                cache.committee_for_epoch(epoch).map(|c| c.epoch),
                Some(epoch),
                "epoch {epoch} should be in the cache"
            );
        }
        assert!(cache.committee_for_epoch(5).is_none());
    }

    /// Ratchet updates that skip an epoch are rejected.
    #[test]
    fn rejects_non_consecutive_epoch_advance() {
        let mut cache = EpochCache::new(committee(0));
        let err = cache.apply_ratchet_update(committee(2)).unwrap_err();
        assert!(
            matches!(
                err,
                LightClientError::InvalidEpochAdvance {
                    current: 0,
                    provided: 2,
                }
            ),
            "got {err:?}"
        );
    }

    /// Ratchet updates that re-issue the current epoch are rejected.
    #[test]
    fn rejects_repeating_current_epoch() {
        let mut cache = EpochCache::new(committee(7));
        let err = cache.apply_ratchet_update(committee(7)).unwrap_err();
        assert!(
            matches!(
                err,
                LightClientError::InvalidEpochAdvance {
                    current: 7,
                    provided: 7,
                }
            ),
            "got {err:?}"
        );
    }
}
