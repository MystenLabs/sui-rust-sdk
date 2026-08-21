use prost::bytes::Bytes;
use tonic::Status;

use super::super::super::Result;
use super::CHECKPOINT_CURSOR_OVERFLOW;
use super::ListScanDirection;

/// A resumable cursor plus known checkpoint coverage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::client::ledger_streams) struct Progress<C> {
    pub(in crate::client::ledger_streams) cursor: C,
    /// Always equals a dense checkpoint cursor; present for opaque cursors when reported.
    pub(in crate::client::ledger_streams) checkpoint: Option<u64>,
}

/// Cursor-specific coverage and replay behavior.
///
/// Dense checkpoint cursors provide their own coverage and replay whole checkpoints. Opaque
/// transaction and event cursors rely on reported coverage and replay strictly after the token.
pub(in crate::client::ledger_streams) trait CursorDomain:
    Clone + Eq + Send + 'static
{
    /// The resumable position a watermark names, or `None` if it does not name one yet.
    fn position(cursor: &Bytes, checkpoint: Option<u64>) -> Option<Progress<Self>>;

    /// Builds the replay interval strictly after this committed cursor and ending at `upper`.
    fn gap(&self, committed_checkpoint: u64, upper: GapUpper<'_, Self>) -> Result<RecoveryGap>;
}

pub(in crate::client::ledger_streams) enum GapUpper<'a, C> {
    Cursor(&'a C, u64),
    EndOfCheckpoint(u64),
}

/// Recovery selected by comparing a subscription's first frame with committed progress.
pub(in crate::client::ledger_streams) enum Recovery {
    /// Deliver immediately from the committed position.
    Live,
    /// Replay the missing interval while buffering live frames.
    Replay(RecoveryGap),
    /// Hide frames until the subscription reaches committed progress.
    MuteUntilCommitted,
}

/// Historical List interval between subscription positions.
pub(in crate::client::ledger_streams) enum RecoveryGap {
    /// Inclusive start and exclusive end typed range. The replay re-delivers its boundary item.
    Checkpoints {
        start_checkpoint: u64,
        end_checkpoint: u64,
    },
    /// Strictly after `after`, ending before a cursor or after an inclusive checkpoint.
    Cursors { after: Bytes, upper: CursorGapUpper },
}

pub(in crate::client::ledger_streams) enum CursorGapUpper {
    Before(Bytes),
    EndOfCheckpoint(u64),
}

impl RecoveryGap {
    pub(in crate::client::ledger_streams) fn replays_boundary_item(&self) -> bool {
        matches!(self, Self::Checkpoints { .. })
    }
}

/// Advancement between ordered frames from one subscription.
///
/// Same-cursor coverage growth advances knowledge; only identical progress is a duplicate.
pub(in crate::client::ledger_streams) enum ProgressAdvance {
    Unchanged,
    CheckpointCoverageAdvanced,
    CursorAdvanced,
}

impl<C: CursorDomain> Progress<C> {
    pub(in crate::client::ledger_streams) fn has_checkpoint_coverage(&self) -> bool {
        self.checkpoint.is_some()
    }

    pub(in crate::client::ledger_streams) fn same_position(&self, other: &Self) -> bool {
        self.cursor == other.cursor
    }

    pub(in crate::client::ledger_streams) fn inherit_checkpoint_coverage(
        &mut self,
        previous: &Self,
    ) {
        if self.checkpoint.is_none() {
            self.checkpoint = previous.checkpoint;
        }
    }

    pub(in crate::client::ledger_streams) fn classify_consecutive_subscription_progress(
        &self,
        next: &Self,
        item_present: bool,
    ) -> Result<ProgressAdvance> {
        let current_checkpoint = self.checkpoint;
        let next_checkpoint = next.checkpoint;
        if matches!((current_checkpoint, next_checkpoint), (Some(_), None)) {
            Err(Status::data_loss(
                "subscription checkpoint coverage became unavailable",
            ))
        } else if matches!(
            (current_checkpoint, next_checkpoint),
            (Some(current), Some(next)) if next < current
        ) {
            Err(Status::data_loss(
                "subscription checkpoint coverage regressed",
            ))
        } else if self.same_position(next) && item_present {
            Err(Status::data_loss("subscription item repeated its cursor"))
        } else if !self.same_position(next) {
            // Frames from one subscription stream are already in ledger order. Checkpoint
            // coverage is needed only to compare frames from different subscriptions.
            Ok(ProgressAdvance::CursorAdvanced)
        } else if matches!(
            (current_checkpoint, next_checkpoint),
            (Some(current), Some(next)) if next > current
        ) || matches!((current_checkpoint, next_checkpoint), (None, Some(_)))
        {
            Ok(ProgressAdvance::CheckpointCoverageAdvanced)
        } else {
            Ok(ProgressAdvance::Unchanged)
        }
    }

    pub(in crate::client::ledger_streams) fn validate_list_successor(
        &self,
        next: &Self,
        direction: ListScanDirection,
    ) -> Result<()> {
        let current_checkpoint = self.checkpoint;
        let next_checkpoint = next.checkpoint;
        if matches!((current_checkpoint, next_checkpoint), (Some(_), None)) {
            return Err(Status::data_loss(
                "List checkpoint coverage became unavailable",
            ));
        }
        let regressed = match (current_checkpoint, next_checkpoint) {
            (Some(current), Some(next)) => match direction {
                ListScanDirection::Ascending => next < current,
                ListScanDirection::Descending => next > current,
            },
            _ => false,
        };
        if regressed {
            Err(Status::data_loss("List checkpoint coverage regressed"))
        } else {
            Ok(())
        }
    }

    pub(in crate::client::ledger_streams) fn plan_recovery(
        &self,
        new: &Self,
        known_coverage: Option<u64>,
    ) -> Result<Recovery> {
        let committed = self.checkpoint.max(known_coverage);
        if self.same_position(new) {
            return if matches!(
                (committed, new.checkpoint),
                (Some(committed), Some(new_checkpoint)) if new_checkpoint < committed
            ) {
                Ok(Recovery::MuteUntilCommitted)
            } else {
                Ok(Recovery::Live)
            };
        }
        match (committed, new.checkpoint) {
            (Some(committed), Some(new_checkpoint)) if new_checkpoint < committed => {
                Ok(Recovery::MuteUntilCommitted)
            }
            (Some(committed), Some(new_checkpoint)) if new_checkpoint > committed => {
                Ok(Recovery::Replay(self.cursor.gap(
                    committed,
                    GapUpper::Cursor(&new.cursor, new_checkpoint),
                )?))
            }
            (Some(committed), Some(_)) => Ok(Recovery::Replay(
                self.cursor
                    .gap(committed, GapUpper::EndOfCheckpoint(committed))?,
            )),
            _ => Ok(Recovery::MuteUntilCommitted),
        }
    }
}

impl CursorDomain for u64 {
    fn position(_cursor: &Bytes, checkpoint: Option<u64>) -> Option<Progress<Self>> {
        checkpoint.map(|checkpoint| Progress {
            cursor: checkpoint,
            checkpoint: Some(checkpoint),
        })
    }

    fn gap(&self, committed_checkpoint: u64, upper: GapUpper<'_, Self>) -> Result<RecoveryGap> {
        let upper_checkpoint = match upper {
            GapUpper::Cursor(_, checkpoint) | GapUpper::EndOfCheckpoint(checkpoint) => checkpoint,
        };
        let start_checkpoint = committed_checkpoint
            .checked_add(1)
            .ok_or_else(|| Status::out_of_range(CHECKPOINT_CURSOR_OVERFLOW))?;
        let end_checkpoint = upper_checkpoint
            .checked_add(1)
            .ok_or_else(|| Status::out_of_range(CHECKPOINT_CURSOR_OVERFLOW))?;
        Ok(RecoveryGap::Checkpoints {
            start_checkpoint,
            end_checkpoint,
        })
    }
}

impl CursorDomain for Bytes {
    fn position(cursor: &Bytes, checkpoint: Option<u64>) -> Option<Progress<Self>> {
        Some(Progress {
            cursor: cursor.clone(),
            checkpoint,
        })
    }

    fn gap(&self, _committed_checkpoint: u64, upper: GapUpper<'_, Self>) -> Result<RecoveryGap> {
        let upper = match upper {
            GapUpper::Cursor(cursor, _) => CursorGapUpper::Before(cursor.clone()),
            GapUpper::EndOfCheckpoint(checkpoint) => CursorGapUpper::EndOfCheckpoint(checkpoint),
        };
        Ok(RecoveryGap::Cursors {
            after: self.clone(),
            upper,
        })
    }
}
