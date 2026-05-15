use sui_sdk_types::ValidatorCommittee;

use super::error::LightClientError;

/// A cache of validator committees indexed by the checkpoint sequence
/// numbers they cover.
///
/// The cache always knows about the *current* committee (the one in
/// effect at the network's most recently observed epoch) and remembers
/// each completed epoch as a closed interval
/// `[start_checkpoint, end_checkpoint]` paired with the committee that
/// was in effect during it. Lookups via
/// [`Self::committee_for_checkpoint`] return whichever committee was
/// active at the given checkpoint sequence number.
///
/// The cache is advanced by calling [`Self::apply_ratchet_update`] with
/// the end-of-epoch checkpoint and the committee that takes effect for
/// the next epoch. The driver in `super::ratchet` is responsible for
/// fetching and BLS-verifying the end-of-epoch summaries that feed
/// these updates.
#[derive(Debug, Clone)]
pub struct EpochCache {
    /// Closed intervals `(start_checkpoint, end_checkpoint, committee)`
    /// for each completed epoch, ordered by `start_checkpoint`. Each
    /// interval immediately precedes the next, so binary search by
    /// `checkpoint_seq` identifies the right entry uniquely.
    completed_committees: Vec<(u64, u64, ValidatorCommittee)>,

    /// The committee in effect for the current (open-ended) epoch.
    current_committee: ValidatorCommittee,

    /// The first checkpoint sequence number that falls within the
    /// current epoch.
    current_epoch_start: u64,
}

impl EpochCache {
    /// Build a fresh cache seeded with the genesis committee. The genesis
    /// committee is assumed to be in effect from checkpoint 0 onwards
    /// until the first ratchet update is applied.
    pub fn new(genesis_committee: ValidatorCommittee) -> Self {
        Self {
            completed_committees: Vec::new(),
            current_committee: genesis_committee,
            current_epoch_start: 0,
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

    /// The first checkpoint sequence number that falls within the current
    /// epoch.
    pub fn current_epoch_start(&self) -> u64 {
        self.current_epoch_start
    }

    /// Look up the validator committee that was in effect at
    /// `checkpoint_seq`.
    ///
    /// Returns `None` if `checkpoint_seq` falls before any committee the
    /// cache knows about (i.e. before checkpoint 0, which shouldn't
    /// happen for a cache seeded with the genesis committee).
    pub fn committee_for_checkpoint(&self, checkpoint_seq: u64) -> Option<&ValidatorCommittee> {
        if checkpoint_seq >= self.current_epoch_start {
            return Some(&self.current_committee);
        }

        // Binary search the completed intervals. Each entry is a closed
        // `[start, end]`; treat `checkpoint_seq < start` as "Greater"
        // and `checkpoint_seq > end` as "Less" so a Found(idx) result
        // identifies the right entry.
        self.completed_committees
            .binary_search_by(|(start, end, _)| {
                if checkpoint_seq < *start {
                    std::cmp::Ordering::Greater
                } else if checkpoint_seq > *end {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .ok()
            .map(|idx| &self.completed_committees[idx].2)
    }

    /// Advance the cache: move the current epoch into `completed_committees`
    /// and install `new_committee` as the new current epoch.
    ///
    /// - `end_of_epoch_checkpoint` is the sequence number of the *last*
    ///   checkpoint of the epoch being closed. The new epoch is taken to
    ///   start at `end_of_epoch_checkpoint + 1`.
    /// - `new_committee.epoch` must be exactly one greater than the
    ///   cache's current epoch.
    /// - `end_of_epoch_checkpoint` must be at least `current_epoch_start`
    ///   (a single-checkpoint epoch is allowed).
    pub fn apply_ratchet_update(
        &mut self,
        end_of_epoch_checkpoint: u64,
        new_committee: ValidatorCommittee,
    ) -> Result<(), LightClientError> {
        let current_epoch = self.current_epoch();
        if new_committee.epoch != current_epoch + 1 {
            return Err(LightClientError::InvalidEpochAdvance {
                current: current_epoch,
                provided: new_committee.epoch,
            });
        }
        if end_of_epoch_checkpoint < self.current_epoch_start {
            return Err(LightClientError::InvalidCheckpointRange {
                current_epoch_start: self.current_epoch_start,
                end_of_epoch_checkpoint,
            });
        }

        let completed_committee = std::mem::replace(&mut self.current_committee, new_committee);
        self.completed_committees.push((
            self.current_epoch_start,
            end_of_epoch_checkpoint,
            completed_committee,
        ));
        self.current_epoch_start = end_of_epoch_checkpoint + 1;
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

    /// Fresh cache reports the genesis committee for every checkpoint at
    /// or after 0, and returns it as the current committee.
    #[test]
    fn fresh_cache_returns_genesis_for_every_checkpoint() {
        let cache = EpochCache::new(committee(0));
        assert_eq!(cache.current_epoch(), 0);
        assert_eq!(cache.current_epoch_start(), 0);
        for seq in [0u64, 1, 10, 1_000_000] {
            assert_eq!(
                cache.committee_for_checkpoint(seq).map(|c| c.epoch),
                Some(0),
                "expected epoch 0 committee at checkpoint {seq}"
            );
        }
    }

    /// After one ratchet, the previous committee covers the closed
    /// interval `[0, end_of_epoch]` and the new committee covers
    /// `end_of_epoch + 1` onwards.
    #[test]
    fn single_ratchet_partitions_the_checkpoint_line() {
        let mut cache = EpochCache::new(committee(0));
        cache.apply_ratchet_update(99, committee(1)).unwrap();

        assert_eq!(cache.current_epoch(), 1);
        assert_eq!(cache.current_epoch_start(), 100);

        // Boundary inside epoch 0.
        assert_eq!(cache.committee_for_checkpoint(0).map(|c| c.epoch), Some(0),);
        assert_eq!(cache.committee_for_checkpoint(99).map(|c| c.epoch), Some(0),);
        // First checkpoint of epoch 1.
        assert_eq!(
            cache.committee_for_checkpoint(100).map(|c| c.epoch),
            Some(1),
        );
        assert_eq!(
            cache.committee_for_checkpoint(1_000_000).map(|c| c.epoch),
            Some(1),
        );
    }

    /// Binary search finds the right interval across many completed
    /// epochs.
    #[test]
    fn binary_search_picks_correct_completed_interval() {
        let mut cache = EpochCache::new(committee(0));
        cache.apply_ratchet_update(9, committee(1)).unwrap();
        cache.apply_ratchet_update(19, committee(2)).unwrap();
        cache.apply_ratchet_update(29, committee(3)).unwrap();
        cache.apply_ratchet_update(39, committee(4)).unwrap();

        // Spot-check each closed range.
        for (seq, expected_epoch) in [
            (0, 0),
            (5, 0),
            (9, 0),
            (10, 1),
            (15, 1),
            (19, 1),
            (20, 2),
            (29, 2),
            (30, 3),
            (39, 3),
            (40, 4),
            (10_000, 4),
        ] {
            assert_eq!(
                cache.committee_for_checkpoint(seq).map(|c| c.epoch),
                Some(expected_epoch),
                "checkpoint {seq} should fall in epoch {expected_epoch}"
            );
        }
    }

    /// Ratchet updates that skip an epoch are rejected.
    #[test]
    fn rejects_non_consecutive_epoch_advance() {
        let mut cache = EpochCache::new(committee(0));
        let err = cache.apply_ratchet_update(99, committee(2)).unwrap_err();
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
        let err = cache.apply_ratchet_update(99, committee(7)).unwrap_err();
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

    /// Ratchet updates whose end-of-epoch checkpoint sits before the
    /// current epoch's start are rejected.
    #[test]
    fn rejects_end_of_epoch_before_current_start() {
        let mut cache = EpochCache::new(committee(0));
        cache.apply_ratchet_update(99, committee(1)).unwrap();
        // current_epoch_start is now 100.
        let err = cache.apply_ratchet_update(50, committee(2)).unwrap_err();
        assert!(
            matches!(
                err,
                LightClientError::InvalidCheckpointRange {
                    current_epoch_start: 100,
                    end_of_epoch_checkpoint: 50,
                }
            ),
            "got {err:?}"
        );
    }

    /// A single-checkpoint epoch (where the end-of-epoch checkpoint
    /// equals the current epoch's start) is permitted.
    #[test]
    fn single_checkpoint_epoch_is_permitted() {
        let mut cache = EpochCache::new(committee(0));
        cache.apply_ratchet_update(0, committee(1)).unwrap();
        assert_eq!(cache.current_epoch_start(), 1);
        assert_eq!(cache.committee_for_checkpoint(0).map(|c| c.epoch), Some(0),);
        assert_eq!(cache.committee_for_checkpoint(1).map(|c| c.epoch), Some(1),);
    }
}
