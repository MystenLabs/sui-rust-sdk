//! `AuthenticatedEventsClient` — the public surface for streaming
//! authenticated events.

use std::time::Duration;

use futures::Stream;
use futures::StreamExt;
use sui_sdk_types::framework::EventStreamHead;
use sui_sdk_types::framework::derive_event_stream_head_object_id;
use tokio::sync::mpsc;
use tokio::time::Instant;

use super::config::AuthenticatedEventsConfig;
use super::envelope::AuthenticatedEvent;
use super::state::StreamState;
use super::state::apply_stream_frame;
use super::state::extract_event_stream_head;
use super::state::fold_and_reconcile;
use crate::client::Delivery;
use crate::client::EventStreamFrame;
use crate::client::EventStreamRequest;
use crate::client::EventStreamStart;
use crate::client::LedgerStreamConfig;
use crate::field::FieldMask;
use crate::field::FieldMaskUtil;
use crate::light_client::CheckpointObjectProof;
use crate::light_client::LightClient;
use crate::light_client::error::LightClientError;
use crate::proto::sui::rpc::v2::EventFilter;
use crate::proto::sui::rpc::v2::ExecutedTransaction;
use crate::proto::sui::rpc::v2::ListTransactionsRequest;
use crate::proto::sui::rpc::v2::QueryOptions;
use crate::proto::sui::rpc::v2::TransactionFilter;
use crate::proto::sui::rpc::v2::filter::event;
use crate::proto::sui::rpc::v2::filter::transaction;

/// A streaming verifier for a single authenticated event stream.
///
/// Construct with [`AuthenticatedEventsClient::new`], passing a
/// configured [`LightClient`] and an [`AuthenticatedEventsConfig`].
/// Call [`Self::stream`] to spawn the background verifier task and
/// receive an async stream of cryptographically-authenticated events.
///
/// Once [`Self::stream`] is called the client is consumed. The
/// returned stream pulls events from the spawned task via a bounded
/// channel; events are yielded only after the periodic reconciliation
/// confirms the local MMR matches the on-chain
/// [`EventStreamHead`](sui_sdk_types::framework::EventStreamHead).
pub struct AuthenticatedEventsClient {
    light: LightClient,
    config: AuthenticatedEventsConfig,
}

impl AuthenticatedEventsClient {
    /// Construct a new streaming client.
    pub fn new(light: LightClient, config: AuthenticatedEventsConfig) -> Self {
        Self { light, config }
    }

    /// Spawn the verifier task and return a stream of authenticated
    /// events.
    ///
    /// The stream terminates when the spawned task exits — either
    /// because the consumer dropped the receiver, or because the task
    /// hit an unrecoverable error (the error is yielded as the last
    /// item before termination).
    pub fn stream(
        self,
    ) -> impl Stream<Item = Result<AuthenticatedEvent, LightClientError>> + Send + 'static {
        let (tx, rx) = mpsc::channel(self.config.channel_capacity);
        tokio::spawn(run_stream_task(self.light, self.config, tx));
        futures::stream::unfold(rx, |mut rx| async move { rx.recv().await.map(|v| (v, rx)) })
    }
}

/// Driver loop for a single stream subscription.
///
/// On startup: fetch the latest checkpoint as the race-free floor,
/// then fetch the on-chain stream head. If the head exists, resume
/// from `head.checkpoint + 1`; otherwise start from
/// `latest_checkpoint + 1`. Either way the floor ensures no event can
/// land in the interval without being picked up by the head check.
///
/// Steady state: follow the ledger event stream
/// ([`crate::Client::stream_events_with_config`], polling the List
/// API) from the first unconfirmed checkpoint, buffering its events and
/// recording the highest `covered_checkpoint` it claims. A checkpoint
/// is complete only once that claim reaches it. Reconcile when complete
/// checkpoints are buffered and either
/// [`AuthenticatedEventsConfig::head_check_interval`] has elapsed since
/// the last reconciliation or a progress-only frame's claim covers every
/// buffered event (the stream has caught up). A reconciliation fetches
/// the settlement boundaries for the complete, unconfirmed range via
/// `ListTransactions(affected_object = event_stream_head)`, folds the
/// buffered events into the local MMR partitioned by settlement, and
/// reconciles the resulting head against the on-chain head proven at
/// the last settled checkpoint. On match, drain the confirmed events
/// through the channel. On mismatch, send the error and exit.
async fn run_stream_task(
    mut light: LightClient,
    config: AuthenticatedEventsConfig,
    tx: mpsc::Sender<Result<AuthenticatedEvent, LightClientError>>,
) {
    // The startup helper can fail; route the error through the channel
    // and exit cleanly so the consumer sees one final item.
    let start = match initial_state(&mut light, &config).await {
        Ok(start) => start,
        Err(e) => {
            let _ = tx.send(Err(e)).await;
            return;
        }
    };

    let mut state = StreamState::new(start.initial_head, start.start_checkpoint);
    let stream_head_object_id = derive_event_stream_head_object_id(config.stream_id);

    let request = EventStreamRequest::new()
        .with_read_mask(AuthenticatedEvent::read_mask())
        .with_filter(build_filter(config.stream_id))
        .with_start(EventStreamStart::Checkpoint(
            start.start_checkpoint.saturating_add(1),
        ))
        .with_delivery(Delivery::Poll);
    let mut frames = std::pin::pin!(
        light
            .rpc()
            .stream_events_with_config(request, config.event_stream_config())
    );

    let mut last_reconcile = Instant::now();
    // Set when the stream catches up with buffered events still waiting
    // to be released; cleared by the next successful reconciliation.
    let mut release_due = false;

    loop {
        // While complete checkpoints are buffered, wait for the next frame
        // no longer than the head-check deadline, so a stream waiting on
        // its next poll never holds back reconciliation. Abandoning the
        // wait keeps the stream's in-flight work for the next call.
        let next_frame = if state.has_complete_buffered_events() {
            tokio::time::timeout_at(last_reconcile + config.head_check_interval, frames.next())
                .await
                .ok()
        } else {
            Some(frames.next().await)
        };

        match next_frame {
            // The head-check deadline passed first.
            None => {}
            Some(Some(Ok(frame))) => match apply_frame(&mut state, frame) {
                Ok(caught_up) => release_due |= caught_up,
                Err(e) => {
                    let _ = tx.send(Err(e)).await;
                    return;
                }
            },
            // The stream retries transient failures itself, so any error
            // it yields is terminal.
            Some(Some(Err(status))) => {
                let _ = tx.send(Err(status.into())).await;
                return;
            }
            Some(None) => {
                let _ = tx
                    .send(Err(LightClientError::Rpc(tonic::Status::internal(
                        "ledger event stream ended",
                    ))))
                    .await;
                return;
            }
        }

        let reconcile_due = release_due || last_reconcile.elapsed() >= config.head_check_interval;
        if !reconcile_due || !state.has_complete_buffered_events() {
            continue;
        }

        match reconcile_once(&mut light, &mut state, &stream_head_object_id, &config).await {
            Ok(released) => {
                last_reconcile = Instant::now();
                release_due = false;
                for ev in released {
                    if tx.send(Ok(ev)).await.is_err() {
                        return;
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(Err(e)).await;
                return;
            }
        }
    }
}

/// Decode `frame` and apply it to `state` with [`apply_stream_frame`].
/// Returns whether the stream has caught up with every buffered event.
fn apply_frame(state: &mut StreamState, frame: EventStreamFrame) -> Result<bool, LightClientError> {
    let event = frame
        .event
        .as_ref()
        .map(AuthenticatedEvent::try_from)
        .transpose()?;
    Ok(apply_stream_frame(state, event, frame.covered_checkpoint))
}

struct InitialState {
    initial_head: EventStreamHead,
    start_checkpoint: u64,
}

/// Establish the starting position. Fetch the latest checkpoint
/// *before* the head so that if an event lands in between, the head
/// fetch observes it and we resume from `head.checkpoint + 1` rather
/// than the now-stale tip.
///
/// Known limitation: an `EventStreamHead` object that exists on chain
/// but was *not modified* in the checkpoint we query produces an
/// authenticated `NonInclusion` result, which this function treats the
/// same as "no head yet" — it falls back to the tip. The OCS-per-
/// checkpoint commitment scheme attests only to objects modified at a
/// given checkpoint, not to the contents of unmodified objects, so we
/// cannot distinguish the two cases through this API alone. In
/// practice this is rarely an issue because new events on a stream
/// modify its head, so a head that hasn't been touched at the tip
/// means the stream has been quiet — and starting from the tip is the
/// correct resume point in both cases.
async fn initial_state(
    light: &mut LightClient,
    config: &AuthenticatedEventsConfig,
) -> Result<InitialState, LightClientError> {
    let latest_tip = light.latest_checkpoint_seq().await?;
    let stream_head_object_id = derive_event_stream_head_object_id(config.stream_id);

    let proof = light
        .prove_object_at_checkpoint(&stream_head_object_id, latest_tip)
        .await?;

    let (initial_head, start_checkpoint) = match proof {
        CheckpointObjectProof::Inclusion {
            object: Some(object),
            ..
        } => {
            let head = extract_event_stream_head(&object)?;
            let cp = head.checkpoint_seq;
            (head, cp)
        }
        CheckpointObjectProof::Inclusion { object: None, .. } => {
            // The head object was deleted or wrapped at the tip.
            // For an authenticated event stream this is unrecoverable
            // — the on-chain anchor is gone, so no future reconciliation
            // can succeed.
            return Err(LightClientError::UnexpectedObjectShape {
                reason: "event stream head was deleted or wrapped at the initial tip",
            });
        }
        CheckpointObjectProof::NonInclusion => (
            EventStreamHead::default(),
            config.start_checkpoint.unwrap_or(latest_tip),
        ),
    };

    let start_checkpoint = config.start_checkpoint.unwrap_or(start_checkpoint);
    Ok(InitialState {
        initial_head,
        start_checkpoint,
    })
}

/// Drive one reconciliation round: fetch settlements for the complete,
/// unconfirmed range, fold the buffered events through the latest
/// settled checkpoint into the local MMR, prove the on-chain head at
/// that checkpoint, and release the folded events on a match.
///
/// Returns the released events on success. Returns `Ok(Vec::new())`
/// when there's nothing to reconcile yet (no settlement modified the
/// head anywhere in the range the settlement listing covered, so
/// there's no chain anchor to compare against).
async fn reconcile_once(
    light: &mut LightClient,
    state: &mut StreamState,
    stream_head_object_id: &sui_sdk_types::Address,
    config: &AuthenticatedEventsConfig,
) -> Result<Vec<AuthenticatedEvent>, LightClientError> {
    // Settlements are only fetched for checkpoints the event stream has
    // claimed complete — partitioning a checkpoint whose events haven't
    // all been buffered yields an MMR that can never match.
    let Some(checkpoints) = state.complete_unconfirmed_checkpoints() else {
        return Ok(Vec::new());
    };

    let settlements =
        fetch_settlements_for_range(light, stream_head_object_id, checkpoints, config).await?;

    let Some((reconcile_cp, _)) = settlements.last().copied() else {
        // No settlement in the listed range — the head wasn't modified
        // there, so there's no anchor to reconcile against. Pending
        // events stay buffered; the next reconciliation round retries
        // once a settlement lands.
        return Ok(Vec::new());
    };

    let mut consecutive_failures = 0u32;
    let proof = loop {
        match light
            .prove_object_at_checkpoint(stream_head_object_id, reconcile_cp)
            .await
        {
            Ok(proof) => break proof,
            Err(e) => backoff_or_give_up(config, &mut consecutive_failures, e).await?,
        }
    };
    let chain_head = match proof {
        CheckpointObjectProof::Inclusion {
            object: Some(object),
            ..
        } => extract_event_stream_head(&object)?,
        CheckpointObjectProof::Inclusion { object: None, .. } => {
            // The on-chain head was deleted or wrapped at the
            // settlement checkpoint. The local replay cannot be
            // reconciled against a missing head; this is terminal for
            // the stream.
            return Err(LightClientError::UnexpectedObjectShape {
                reason: "event stream head was deleted or wrapped at the reconciliation tip",
            });
        }
        CheckpointObjectProof::NonInclusion => {
            // `ListTransactions(affected_object)` placed a settlement
            // at this checkpoint, but the OCS proof says the head
            // wasn't modified there — the two indexes are
            // inconsistent.
            return Err(LightClientError::UnexpectedObjectShape {
                reason: "settlement transaction listed at checkpoint but OCS proof reports \
                         the event stream head was not modified",
            });
        }
    };

    fold_and_reconcile(state, &settlements, chain_head, reconcile_cp)
}

/// List transactions filtered on `affected_object =
/// stream_head_object_id` over `checkpoints` and return the ascending
/// `(checkpoint, transaction_index)` settlement boundaries at
/// checkpoints the listing claimed complete.
///
/// Each `settle_events` transaction mutates the stream's head object,
/// so this filter returns exactly the per-stream settlement boundaries.
/// `checkpoints` is half-open, matching the proto `start_checkpoint` /
/// `end_checkpoint` semantics.
///
/// The listing can end early at the ledger tip when it reaches a server
/// that has not yet indexed the whole range. Only settlements at or
/// below the highest `Watermark.checkpoint` the listing reported are
/// returned: a later checkpoint may hold settlements the listing has
/// not reached, so it is left for a later reconciliation round.
async fn fetch_settlements_for_range(
    light: &mut LightClient,
    stream_head_object_id: &sui_sdk_types::Address,
    checkpoints: std::ops::Range<u64>,
    config: &AuthenticatedEventsConfig,
) -> Result<Vec<(u64, u64)>, LightClientError> {
    let request = ListTransactionsRequest {
        read_mask: Some(settlement_read_mask()),
        start_checkpoint: Some(checkpoints.start),
        end_checkpoint: Some(checkpoints.end),
        filter: Some(build_affected_object_filter(stream_head_object_id)),
        options: Some(QueryOptions {
            limit: Some(config.page_size),
            after: None,
            before: None,
            ordering: None, // ascending
        }),
        start_transaction_index: None,
        end_transaction_index: None,
    };
    let mut frames = std::pin::pin!(
        light
            .rpc()
            .list_transactions_with_config(request, config.settlement_list_config())
    );

    let mut settlements: Vec<(u64, u64)> = Vec::new();
    let mut covered_checkpoint: Option<u64> = None;
    while let Some(frame) = frames.next().await {
        let frame = frame?;
        if let Some(checkpoint) = frame.watermark.as_ref().and_then(|w| w.checkpoint) {
            covered_checkpoint = covered_checkpoint.max(Some(checkpoint));
        }
        if let Some(transaction) = frame.transaction.as_ref() {
            let checkpoint =
                transaction
                    .checkpoint
                    .ok_or(LightClientError::UnexpectedObjectShape {
                        reason: "settlement transaction missing checkpoint",
                    })?;
            let tx_offset =
                transaction
                    .transaction_index
                    .ok_or(LightClientError::UnexpectedObjectShape {
                        reason: "settlement transaction missing transaction_index",
                    })?;
            settlements.push((checkpoint, tx_offset));
        }
    }

    let complete = match covered_checkpoint {
        Some(covered) => settlements.partition_point(|(checkpoint, _)| *checkpoint <= covered),
        None => 0,
    };
    settlements.truncate(complete);
    Ok(settlements)
}

fn build_filter(stream_id: sui_sdk_types::Address) -> EventFilter {
    EventFilter::matching(event::event_stream_head(stream_id))
}

fn build_affected_object_filter(object_id: &sui_sdk_types::Address) -> TransactionFilter {
    TransactionFilter::matching(transaction::affected_object(*object_id))
}

/// Read mask selecting the settlement position fields
/// [`fetch_settlements_for_range`] reads from each listed transaction.
fn settlement_read_mask() -> FieldMask {
    FieldMask::from_paths([
        ExecutedTransaction::CHECKPOINT_FIELD.name,
        ExecutedTransaction::TRANSACTION_INDEX_FIELD.name,
    ])
}

fn e_is_retryable(err: &LightClientError) -> bool {
    matches!(
        err,
        LightClientError::Rpc(status)
            if matches!(
                status.code(),
                tonic::Code::Unavailable
                    | tonic::Code::DeadlineExceeded
                    | tonic::Code::ResourceExhausted
                    | tonic::Code::Aborted
            )
    )
}

/// Decide whether to retry a failed RPC the client issues itself.
///
/// Returns `err` when it is not transient or when
/// [`AuthenticatedEventsConfig::max_consecutive_retries`] consecutive
/// retries have already been made. Otherwise sleeps for
/// `retry_backoff * attempts`, capped at the ledger stream's default
/// maximum retry delay, plus jitter, and returns `Ok(())` so the caller
/// reissues the RPC.
async fn backoff_or_give_up(
    config: &AuthenticatedEventsConfig,
    consecutive_failures: &mut u32,
    err: LightClientError,
) -> Result<(), LightClientError> {
    if !e_is_retryable(&err)
        || config
            .max_consecutive_retries
            .is_some_and(|max_retries| *consecutive_failures >= max_retries)
    {
        return Err(err);
    }
    *consecutive_failures = consecutive_failures.saturating_add(1);
    let base = config
        .retry_backoff
        .saturating_mul(*consecutive_failures)
        .min(LedgerStreamConfig::default().max_retry_delay);
    let jitter = pseudo_jitter(*consecutive_failures, config.retry_jitter);
    tokio::time::sleep(base.saturating_add(jitter)).await;
    Ok(())
}

/// Deterministic jitter derived from the attempt count. Avoids
/// pulling in `rand` for a single call; the goal is just to avoid
/// lockstep retries across many concurrent streams, not cryptographic
/// randomness.
fn pseudo_jitter(attempts: u32, ceiling: Duration) -> Duration {
    if ceiling.is_zero() {
        return Duration::ZERO;
    }
    // Mix the attempt count with a fixed prime to spread values.
    let mix = (u64::from(attempts).wrapping_mul(0x9E3779B97F4A7C15)) as u128;
    let ceiling_ms = ceiling.as_millis().max(1);
    let offset_ms = (mix % ceiling_ms) as u64;
    Duration::from_millis(offset_ms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldMaskTree;
    use crate::merge::Merge;

    /// A server honoring the settlement read mask returns both position
    /// fields the settlement listing reads.
    #[test]
    fn settlement_read_mask_selects_settlement_position() {
        let transaction = ExecutedTransaction::default()
            .with_checkpoint(12)
            .with_transaction_index(4)
            .with_digest("unused");

        let mask = FieldMaskTree::from(settlement_read_mask());
        let masked = ExecutedTransaction::merge_from(&transaction, &mask);

        assert_eq!(masked.checkpoint, Some(12));
        assert_eq!(masked.transaction_index, Some(4));
        assert_eq!(masked.digest, None);
    }
}
