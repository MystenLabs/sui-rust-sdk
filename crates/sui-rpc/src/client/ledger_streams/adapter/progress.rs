use prost::bytes::Bytes;
use tonic::Status;

use super::super::super::Result;
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
}

impl<C: CursorDomain> Progress<C> {
    pub(in crate::client::ledger_streams) fn inherit_checkpoint_coverage(
        &mut self,
        previous: &Self,
    ) {
        if self.checkpoint.is_none() {
            self.checkpoint = previous.checkpoint;
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
}

impl CursorDomain for u64 {
    fn position(_cursor: &Bytes, checkpoint: Option<u64>) -> Option<Progress<Self>> {
        checkpoint.map(|checkpoint| Progress {
            cursor: checkpoint,
            checkpoint: Some(checkpoint),
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
}
