use tonic::Status;

use super::super::super::Result;
use super::super::types::Position;
use super::ListScanDirection;
use crate::proto::sui::rpc::v2::LedgerPosition;

/// The fields of an item key, which also bound the fields of positions in its family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::client::ledger_streams) enum KeyGranularity {
    /// Keys are `(checkpoint)`.
    Checkpoint,
    /// Keys are `(checkpoint, transaction_index)`.
    Transaction,
    /// Keys are `(checkpoint, transaction_index, event_index)`.
    Event,
}

/// Checkpoint range fields that bound one end of a List request at a position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::client::ledger_streams) struct BoundFields {
    pub(in crate::client::ledger_streams) checkpoint: u64,
    pub(in crate::client::ledger_streams) transaction_index: Option<u64>,
    pub(in crate::client::ledger_streams) event_index: Option<u32>,
}

impl Position {
    /// The boundary before the first item of `checkpoint`.
    pub(in crate::client::ledger_streams) fn checkpoint_start(checkpoint: u64) -> Self {
        Self {
            checkpoint,
            transaction_index: 0,
            event_index: 0,
        }
    }

    /// Request fields that start a scan at this position: `start_checkpoint`, plus
    /// `start_transaction_index` when either index is nonzero and `start_event_index` when the
    /// event index is nonzero.
    pub(in crate::client::ledger_streams) fn start_fields(self) -> BoundFields {
        let has_index = self.transaction_index != 0 || self.event_index != 0;
        BoundFields {
            checkpoint: self.checkpoint,
            transaction_index: has_index.then_some(self.transaction_index),
            event_index: (self.event_index != 0).then_some(self.event_index),
        }
    }

    /// Request fields that end a scan at this position: `end_checkpoint` alone at a checkpoint
    /// start; otherwise `end_checkpoint = checkpoint + 1` plus the end indexes.
    pub(in crate::client::ledger_streams) fn end_fields(self) -> Result<BoundFields> {
        if self.transaction_index == 0 && self.event_index == 0 {
            return Ok(BoundFields {
                checkpoint: self.checkpoint,
                transaction_index: None,
                event_index: None,
            });
        }
        let checkpoint = self.checkpoint.checked_add(1).ok_or_else(|| {
            Status::out_of_range("ledger position cannot be converted to an exclusive end bound")
        })?;
        Ok(BoundFields {
            checkpoint,
            transaction_index: Some(self.transaction_index),
            event_index: (self.event_index != 0).then_some(self.event_index),
        })
    }
}

impl KeyGranularity {
    /// Reads a wire position, with unset fields as 0, ignoring fields finer than this
    /// granularity.
    pub(in crate::client::ledger_streams) fn position(self, position: &LedgerPosition) -> Position {
        self.normalize(Position {
            checkpoint: position.checkpoint_opt().unwrap_or(0),
            transaction_index: position.transaction_index_opt().unwrap_or(0),
            event_index: position.event_index_opt().unwrap_or(0),
        })
    }

    /// Zeroes the fields of `position` finer than this granularity.
    pub(in crate::client::ledger_streams) fn normalize(self, position: Position) -> Position {
        match self {
            Self::Checkpoint => Position::checkpoint_start(position.checkpoint),
            Self::Transaction => Position {
                event_index: 0,
                ..position
            },
            Self::Event => position,
        }
    }

    /// The key of the item an ascending item frame at `position` carries: `position` with its
    /// finest index decremented. Exact, because item frames never carry their position into the
    /// next transaction or checkpoint. `None` when that index is 0, which no item frame names.
    pub(in crate::client::ledger_streams) fn ascending_item_key(
        self,
        position: Position,
    ) -> Option<Position> {
        match self {
            Self::Checkpoint => position
                .checkpoint
                .checked_sub(1)
                .map(Position::checkpoint_start),
            Self::Transaction => {
                position
                    .transaction_index
                    .checked_sub(1)
                    .map(|transaction_index| Position {
                        transaction_index,
                        ..position
                    })
            }
            Self::Event => position
                .event_index
                .checked_sub(1)
                .map(|event_index| Position {
                    event_index,
                    ..position
                }),
        }
    }
}

/// The error for a frame whose watermark lacks `cursor_position`.
pub(in crate::client::ledger_streams) fn missing_cursor_position(source: &str) -> Status {
    Status::data_loss(format!(
        "{source} watermark is missing its cursor_position; servers that predate cursor \
         positions do not send it"
    ))
}

/// A frame's resume point plus the checkpoint coverage reported with it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::client::ledger_streams) struct Progress<C> {
    /// Every match before this position has been delivered (ascending).
    pub(in crate::client::ledger_streams) position: Position,
    /// Opaque cursor naming `position`, passed back to servers unchanged and never compared.
    pub(in crate::client::ledger_streams) cursor: C,
    /// Inclusive checkpoint the scan reported fully covered, when reported.
    pub(in crate::client::ledger_streams) checkpoint: Option<u64>,
}

impl<C: Clone> Progress<C> {
    pub(in crate::client::ledger_streams) fn inherit_checkpoint_coverage(
        &mut self,
        previous: &Self,
    ) {
        if self.checkpoint.is_none() {
            self.checkpoint = previous.checkpoint;
        }
    }

    /// Folds `next`, a later frame of the same `source` scan or subscription in `direction`, into
    /// `self`, the furthest progress seen. Returns whether `next` advanced
    /// `(position, coverage)`.
    ///
    /// A frame without an item whose position lies behind `self`'s makes no progress rather than
    /// failing: different positions can name one boundary, such as `(cp, tx_count)` and
    /// `(cp + 1, 0)`. An item frame's position must lie beyond `self`'s, and coverage must not
    /// regress or disappear.
    pub(in crate::client::ledger_streams) fn advance(
        &mut self,
        next: &Self,
        direction: ListScanDirection,
        item_present: bool,
        source: &str,
    ) -> Result<(bool, bool)> {
        let coverage_regressed = match (self.checkpoint, next.checkpoint) {
            (Some(_), None) => {
                return Err(Status::data_loss(format!(
                    "{source} checkpoint coverage became unavailable"
                )));
            }
            (Some(current), Some(next)) => match direction {
                ListScanDirection::Ascending => next < current,
                ListScanDirection::Descending => next > current,
            },
            (None, _) => false,
        };
        if coverage_regressed {
            return Err(Status::data_loss(format!(
                "{source} checkpoint coverage regressed"
            )));
        }
        let position_advanced = direction.beyond(next.position, self.position);
        if item_present && !position_advanced {
            return Err(Status::data_loss(format!(
                "{source} item did not advance its cursor_position"
            )));
        }
        let coverage_advanced = next.checkpoint != self.checkpoint;
        if position_advanced {
            self.position = next.position;
            self.cursor = next.cursor.clone();
        }
        self.checkpoint = next.checkpoint;
        Ok((position_advanced, coverage_advanced))
    }
}

/// An item with its ledger key.
pub(in crate::client::ledger_streams) struct KeyedItem<I> {
    pub(in crate::client::ledger_streams) key: Position,
    pub(in crate::client::ledger_streams) payload: I,
}

/// One frame of an ascending scan or subscription: an optional item and its progress.
pub(in crate::client::ledger_streams) struct LedgerFrame<I, C> {
    pub(in crate::client::ledger_streams) item: Option<KeyedItem<I>>,
    pub(in crate::client::ledger_streams) progress: Progress<C>,
}

impl<I, C> LedgerFrame<I, C> {
    /// Keys `item` from the frame's ascending position.
    pub(in crate::client::ledger_streams) fn new(
        item: Option<I>,
        progress: Progress<C>,
        granularity: KeyGranularity,
    ) -> Result<Self> {
        let item = item
            .map(|payload| {
                granularity
                    .ascending_item_key(progress.position)
                    .map(|key| KeyedItem { key, payload })
                    .ok_or_else(|| {
                        Status::data_loss("item frame cursor_position does not follow an item")
                    })
            })
            .transpose()?;
        Ok(Self { item, progress })
    }

    pub(in crate::client::ledger_streams) fn item_key(&self) -> Option<Position> {
        self.item.as_ref().map(|item| item.key)
    }

    /// The boundary before this frame's content: its item's key, or its position when it carries
    /// no item.
    pub(in crate::client::ledger_streams) fn leading_boundary(&self) -> Position {
        self.item_key().unwrap_or(self.progress.position)
    }
}

/// Historical List interval `[start, end)` between committed progress and a new subscription.
pub(in crate::client::ledger_streams) struct RecoveryGap<C> {
    pub(in crate::client::ledger_streams) start: Position,
    /// Cursor naming `start`, sent as `options.after`.
    pub(in crate::client::ledger_streams) after: C,
    pub(in crate::client::ledger_streams) end: Position,
    /// Cursor naming `end`, sent as `options.before`.
    pub(in crate::client::ledger_streams) before: C,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(checkpoint: u64, transaction_index: u64, event_index: u32) -> Position {
        Position {
            checkpoint,
            transaction_index,
            event_index,
        }
    }

    #[test]
    fn start_fields_set_indexes_only_when_nonzero() {
        assert_eq!(
            position(7, 0, 0).start_fields(),
            BoundFields {
                checkpoint: 7,
                transaction_index: None,
                event_index: None,
            }
        );
        assert_eq!(
            position(7, 3, 0).start_fields(),
            BoundFields {
                checkpoint: 7,
                transaction_index: Some(3),
                event_index: None,
            }
        );
        assert_eq!(
            position(7, 0, 2).start_fields(),
            BoundFields {
                checkpoint: 7,
                transaction_index: Some(0),
                event_index: Some(2),
            }
        );
    }

    #[test]
    fn end_fields_end_inside_the_next_checkpoint_range_unless_at_a_checkpoint_start() {
        assert_eq!(
            position(7, 0, 0).end_fields().unwrap(),
            BoundFields {
                checkpoint: 7,
                transaction_index: None,
                event_index: None,
            }
        );
        assert_eq!(
            position(7, 3, 0).end_fields().unwrap(),
            BoundFields {
                checkpoint: 8,
                transaction_index: Some(3),
                event_index: None,
            }
        );
        assert_eq!(
            position(7, 3, 2).end_fields().unwrap(),
            BoundFields {
                checkpoint: 8,
                transaction_index: Some(3),
                event_index: Some(2),
            }
        );
        assert_eq!(
            position(u64::MAX, 1, 0).end_fields().unwrap_err().code(),
            tonic::Code::OutOfRange
        );
    }

    #[test]
    fn ascending_item_keys_decrement_the_finest_index() {
        assert_eq!(
            KeyGranularity::Checkpoint.ascending_item_key(position(8, 0, 0)),
            Some(position(7, 0, 0))
        );
        assert_eq!(
            KeyGranularity::Transaction.ascending_item_key(position(8, 3, 0)),
            Some(position(8, 2, 0))
        );
        assert_eq!(
            KeyGranularity::Event.ascending_item_key(position(8, 3, 2)),
            Some(position(8, 3, 1))
        );
        assert_eq!(
            KeyGranularity::Transaction.ascending_item_key(position(8, 0, 0)),
            None
        );
        assert_eq!(
            KeyGranularity::Event.ascending_item_key(position(8, 3, 0)),
            None
        );
    }

    #[test]
    fn positions_ignore_fields_finer_than_their_granularity() {
        let ledger_position = LedgerPosition::default()
            .with_checkpoint(4)
            .with_transaction_index(5)
            .with_event_index(6);
        assert_eq!(
            KeyGranularity::Checkpoint.position(&ledger_position),
            position(4, 0, 0)
        );
        assert_eq!(
            KeyGranularity::Transaction.position(&ledger_position),
            position(4, 5, 0)
        );
        assert_eq!(
            KeyGranularity::Event.position(&ledger_position),
            position(4, 5, 6)
        );
        assert_eq!(
            KeyGranularity::Event.position(&LedgerPosition::default()),
            Position::default()
        );
    }
}
