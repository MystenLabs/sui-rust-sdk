//! `AuthenticatedEventsClient` — the public surface for streaming
//! authenticated events.

use std::time::Duration;

use futures::Stream;
use futures::StreamExt;
use sui_sdk_types::framework::EventStreamHead;
use sui_sdk_types::framework::derive_event_stream_head_object_id;
use tokio::sync::mpsc;

use super::config::AuthenticatedEventsConfig;
use super::envelope::AuthenticatedEvent;
use super::state::StreamState;
use super::state::buffer_response_batch;
use super::state::extract_event_stream_head;
use super::state::fold_and_reconcile;
use crate::light_client::CheckpointObjectProof;
use crate::light_client::LightClient;
use crate::light_client::error::LightClientError;
use crate::proto::sui::rpc::v2alpha::EventFilter;
use crate::proto::sui::rpc::v2alpha::ListEventsRequest;
use crate::proto::sui::rpc::v2alpha::ListTransactionsRequest;
use crate::proto::sui::rpc::v2alpha::QueryEndReason;
use crate::proto::sui::rpc::v2alpha::QueryOptions;
use crate::proto::sui::rpc::v2alpha::TransactionFilter;
use crate::proto::sui::rpc::v2alpha::filter::event;
use crate::proto::sui::rpc::v2alpha::filter::transaction;
use crate::proto::sui::rpc::v2alpha::list_events_response;
use crate::proto::sui::rpc::v2alpha::list_transactions_response;

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
/// Steady state: page through `ListEvents` and buffer the items, then at
/// each [`AuthenticatedEventsConfig::head_check_interval`] tick fetch
/// the settlement boundaries for the unconfirmed range via
/// `ListTransactions(affected_object = event_stream_head)`, fold the
/// buffered events into the local MMR partitioned by settlement, and
/// reconcile the resulting head against the on-chain head proven at the
/// last settled checkpoint. On match, drain the confirmed events
/// through the channel. On mismatch, send the error and exit.
async fn run_stream_task(
    mut light: LightClient,
    config: AuthenticatedEventsConfig,
    tx: mpsc::Sender<Result<AuthenticatedEvent, LightClientError>>,
) {
    // The two startup helpers can fail; route the error through the
    // channel and exit cleanly so the consumer sees one final item.
    let start = match initial_state(&mut light, &config).await {
        Ok(start) => start,
        Err(e) => {
            let _ = tx.send(Err(e)).await;
            return;
        }
    };

    let mut state = StreamState::new(start.initial_head, start.start_checkpoint);
    let mut next_checkpoint = start
        .start_checkpoint
        .checked_add(1)
        .unwrap_or(start.start_checkpoint);
    let mut next_cursor: Option<prost::bytes::Bytes> = None;
    let mut consecutive_failures = 0u32;
    let mut last_head_check = std::time::Instant::now();

    let stream_head_object_id = derive_event_stream_head_object_id(config.stream_id);
    let filter = build_filter(config.stream_id);

    loop {
        // Decide whether to fetch the next page or reconcile.
        //
        // The "idle" condition (non-empty buffer, no cursor, drained
        // through the next checkpoint) lets us reconcile as soon as
        // we've drawn level with the indexed tip rather than waiting
        // out the full interval.
        let should_reconcile = last_head_check.elapsed() >= config.head_check_interval
            || !state.buffer.is_empty()
                && next_cursor.is_none()
                && page_drain_done(&state, next_checkpoint);

        if should_reconcile {
            match reconcile_once(&mut light, &mut state, &stream_head_object_id, &config).await {
                Ok(released) => {
                    consecutive_failures = 0;
                    last_head_check = std::time::Instant::now();
                    for ev in released {
                        if tx.send(Ok(ev)).await.is_err() {
                            return;
                        }
                    }
                }
                Err(e) if e_is_retryable(&e) => {
                    if !backoff_or_give_up(&tx, &config, &mut consecutive_failures, e).await {
                        return;
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(e)).await;
                    return;
                }
            }
            continue;
        }

        // Fetch the next page of events.
        let request = ListEventsRequest {
            read_mask: None,
            start_checkpoint: Some(next_checkpoint),
            end_checkpoint: None,
            filter: Some(filter.clone()),
            options: Some(QueryOptions {
                limit: Some(config.page_size),
                after: next_cursor.clone(),
                before: None,
                ordering: None, // ascending (default)
            }),
        };

        match fetch_one_page(&mut light, request).await {
            Ok(page) => {
                let PageResult {
                    events,
                    end_cursor,
                    end_reason,
                    watermark,
                    partial_error,
                } = page;

                buffer_response_batch(&mut state, events, watermark);

                // Mid-stream transport error: commit whatever items we
                // got into the buffer (done above), advance
                // `next_cursor` to the latest watermark the server
                // sent, then backoff-or-give-up the same way a
                // pre-stream error would. This is the fix for the
                // resumption-on-timeout case: without it, repeated
                // server-side timeouts would each lose the cursor
                // accumulated during the failed page and the loop
                // would replay the same stale position forever.
                if let Some(err) = partial_error {
                    if e_is_retryable(&err) {
                        next_cursor = end_cursor;
                        if !backoff_or_give_up(&tx, &config, &mut consecutive_failures, err).await {
                            return;
                        }
                    } else {
                        let _ = tx.send(Err(err)).await;
                        return;
                    }
                    continue;
                }

                consecutive_failures = 0;
                match end_reason {
                    // The server stopped mid-range with unscanned work
                    // remaining — resume from the latest in-stream
                    // cursor (item or watermark) so we don't skip the
                    // unscanned tail. Same shape as ItemLimit: keep
                    // advancing the cursor, no checkpoint bump, no
                    // backoff sleep.
                    Some(QueryEndReason::ItemLimit | QueryEndReason::ScanLimit) => {
                        next_cursor = end_cursor;
                    }
                    Some(_) => {
                        // Server reached the indexed tip, a requested
                        // checkpoint range bound, or a cursor bound —
                        // no more events available in this scan. Reset
                        // cursor and bump start checkpoint to one past
                        // the watermark / last buffered event so the
                        // next page picks up new events as they
                        // arrive. We can't use `local_head.checkpoint_seq`
                        // here because folding is deferred to
                        // reconciliation — it can lag the scan by an
                        // entire interval.
                        next_cursor = None;
                        next_checkpoint = state
                            .events_scanned_through
                            .checked_add(1)
                            .unwrap_or(next_checkpoint)
                            .max(next_checkpoint);
                        // Sleep briefly so we don't spin when at the tip.
                        tokio::time::sleep(config.retry_backoff).await;
                    }
                    None => {
                        // Stream ended cleanly (no transport error) but
                        // without an `End` frame. This shouldn't happen
                        // in normal operation; preserve the latest
                        // cursor and back off briefly rather than
                        // assume the scan finished and skip ahead.
                        next_cursor = end_cursor;
                        tokio::time::sleep(config.retry_backoff).await;
                    }
                }
            }
            Err(e) if e_is_retryable(&e) => {
                if !backoff_or_give_up(&tx, &config, &mut consecutive_failures, e).await {
                    return;
                }
            }
            Err(e) => {
                let _ = tx.send(Err(e)).await;
                return;
            }
        }
    }
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

struct PageResult {
    events: Vec<AuthenticatedEvent>,
    /// Latest opaque cursor delivered during the page — most recent
    /// `Watermark.cursor` or per-item cursor, whichever arrived later in
    /// stream order. This is the safe resume point on any termination:
    /// the server no longer carries a cursor on `QueryEnd`.
    end_cursor: Option<prost::bytes::Bytes>,
    end_reason: Option<QueryEndReason>,
    /// Most recent `Watermark.checkpoint` observed in this page,
    /// across both standalone watermarks and per-item watermarks. The
    /// streaming state uses this as the "events scanned through" floor
    /// when picking the settlement-fetch range.
    watermark: Option<u64>,
    /// Mid-stream transport error that interrupted page accumulation.
    /// When present, `events` and `end_cursor` reflect what was
    /// received before the error. The caller advances `next_cursor` to
    /// `end_cursor` before dispatching retry vs. terminal — without
    /// this, propagating the error via `?` would discard the cursor
    /// progress and the retry loop would replay the same stale
    /// position on every server-side timeout.
    partial_error: Option<LightClientError>,
}

async fn fetch_one_page(
    light: &mut LightClient,
    request: ListEventsRequest,
) -> Result<PageResult, LightClientError> {
    let mut stream = light
        .rpc()
        .ledger_client_alpha()
        .list_events(request)
        .await?
        .into_inner();

    let mut events = Vec::new();
    let mut end_cursor: Option<prost::bytes::Bytes> = None;
    let mut end_reason = None;
    let mut watermark: Option<u64> = None;
    let mut partial_error: Option<LightClientError> = None;

    while let Some(frame) = stream.next().await {
        let frame = match frame {
            Ok(f) => f,
            Err(status) => {
                partial_error = Some(status.into());
                break;
            }
        };
        match frame.response {
            Some(list_events_response::Response::Item(item)) => {
                if let Some(w) = item.watermark.as_ref() {
                    if let Some(c) = w.cursor.clone() {
                        end_cursor = Some(c);
                    }
                    if let Some(hi) = w.checkpoint {
                        watermark = Some(watermark.map_or(hi, |prev| prev.max(hi)));
                    }
                }
                let ev = AuthenticatedEvent::try_from(&item)?;
                events.push(ev);
            }
            Some(list_events_response::Response::Watermark(w)) => {
                if let Some(c) = w.cursor {
                    end_cursor = Some(c);
                }
                if let Some(hi) = w.checkpoint {
                    watermark = Some(watermark.map_or(hi, |prev| prev.max(hi)));
                }
            }
            Some(list_events_response::Response::End(end)) => {
                end_reason = QueryEndReason::try_from(end.reason).ok();
                break;
            }
            None => break,
        }
    }

    Ok(PageResult {
        events,
        end_cursor,
        end_reason,
        watermark,
        partial_error,
    })
}

/// Drive one reconciliation tick: fetch settlements for the unconfirmed
/// range, fold the buffered events through the latest settled
/// checkpoint into the local MMR, prove the on-chain head at that
/// checkpoint, and release the folded events on a match.
///
/// Returns the released events on success. Returns `Ok(Vec::new())`
/// when there's nothing to reconcile yet (the head wasn't modified
/// anywhere in the unconfirmed range, so there's no chain anchor to
/// compare against).
async fn reconcile_once(
    light: &mut LightClient,
    state: &mut StreamState,
    stream_head_object_id: &sui_sdk_types::Address,
    config: &AuthenticatedEventsConfig,
) -> Result<Vec<AuthenticatedEvent>, LightClientError> {
    // Settlements only happen up through events we've fully scanned —
    // querying past `events_scanned_through` risks partitioning into a
    // settlement bucket whose events haven't all been buffered yet.
    let settlement_upper_inclusive = state.events_scanned_through;
    if settlement_upper_inclusive <= state.confirmed_through {
        return Ok(Vec::new());
    }

    let settlements = fetch_settlements_for_range(
        light,
        stream_head_object_id,
        state.confirmed_through.saturating_add(1),
        settlement_upper_inclusive.saturating_add(1),
        config.page_size,
    )
    .await?;

    let Some((reconcile_cp, _)) = settlements.last().copied() else {
        // No settlement in the unconfirmed range — the head wasn't
        // modified, so there's no anchor to reconcile against. Pending
        // events stay buffered; the next reconciliation tick will
        // retry once a settlement lands.
        return Ok(Vec::new());
    };

    let proof = light
        .prove_object_at_checkpoint(stream_head_object_id, reconcile_cp)
        .await?;
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

/// Page through `ListTransactions` filtered on `affected_object =
/// stream_head_object_id` and return the ascending `(checkpoint,
/// transaction_offset)` settlement boundaries for `[start, end)`.
///
/// Each `settle_events` transaction mutates the stream's head object,
/// so this filter returns exactly the per-stream settlement boundaries.
/// `start` is inclusive and `end` is exclusive — matching the proto
/// `start_checkpoint` / `end_checkpoint` semantics — so the caller
/// passes `confirmed_through + 1` and `events_scanned_through + 1`.
async fn fetch_settlements_for_range(
    light: &mut LightClient,
    stream_head_object_id: &sui_sdk_types::Address,
    start_checkpoint: u64,
    end_checkpoint_exclusive: u64,
    page_size: u32,
) -> Result<Vec<(u64, u64)>, LightClientError> {
    if end_checkpoint_exclusive <= start_checkpoint {
        return Ok(Vec::new());
    }

    let filter = build_affected_object_filter(stream_head_object_id);
    let mut settlements: Vec<(u64, u64)> = Vec::new();
    let mut cursor: Option<prost::bytes::Bytes> = None;

    loop {
        let request = ListTransactionsRequest {
            read_mask: None,
            start_checkpoint: Some(start_checkpoint),
            end_checkpoint: Some(end_checkpoint_exclusive),
            filter: Some(filter.clone()),
            options: Some(QueryOptions {
                limit: Some(page_size),
                after: cursor.clone(),
                before: None,
                ordering: None, // ascending
            }),
        };

        let page = fetch_settlements_page(light, request).await?;
        settlements.extend(page.entries);

        match page.end_reason {
            // Server hit its per-request bound but the range may have
            // more — keep paging from the latest cursor it gave us.
            Some(QueryEndReason::ItemLimit | QueryEndReason::ScanLimit) => {
                if page.end_cursor.is_none() {
                    // Defensive: no cursor advance means we'd loop
                    // forever. Treat as done; the next reconciliation
                    // tick will retry with a fresh window.
                    break;
                }
                cursor = page.end_cursor;
            }
            // Range fully scanned (CheckpointBound / CursorBound /
            // LedgerTip / Unspecified) or end frame missing: nothing
            // more to fetch in this window.
            _ => break,
        }
    }

    Ok(settlements)
}

struct SettlementsPage {
    entries: Vec<(u64, u64)>,
    end_cursor: Option<prost::bytes::Bytes>,
    end_reason: Option<QueryEndReason>,
}

async fn fetch_settlements_page(
    light: &mut LightClient,
    request: ListTransactionsRequest,
) -> Result<SettlementsPage, LightClientError> {
    let mut stream = light
        .rpc()
        .ledger_client_alpha()
        .list_transactions(request)
        .await?
        .into_inner();

    let mut entries = Vec::new();
    let mut end_cursor: Option<prost::bytes::Bytes> = None;
    let mut end_reason = None;

    while let Some(frame) = stream.next().await {
        let frame = frame?;
        match frame.response {
            Some(list_transactions_response::Response::Item(item)) => {
                if let Some(c) = item.watermark.as_ref().and_then(|w| w.cursor.clone()) {
                    end_cursor = Some(c);
                }
                let checkpoint = item
                    .transaction
                    .as_ref()
                    .and_then(|tx| tx.checkpoint)
                    .ok_or(LightClientError::UnexpectedObjectShape {
                        reason: "settlement transaction missing checkpoint",
                    })?;
                let tx_offset =
                    item.transaction_offset
                        .ok_or(LightClientError::UnexpectedObjectShape {
                            reason: "settlement transaction missing transaction_offset",
                        })?;
                entries.push((checkpoint, tx_offset));
            }
            Some(list_transactions_response::Response::Watermark(w)) => {
                if let Some(c) = w.cursor {
                    end_cursor = Some(c);
                }
            }
            Some(list_transactions_response::Response::End(end)) => {
                end_reason = QueryEndReason::try_from(end.reason).ok();
                break;
            }
            None => break,
        }
    }

    Ok(SettlementsPage {
        entries,
        end_cursor,
        end_reason,
    })
}

fn build_filter(stream_id: sui_sdk_types::Address) -> EventFilter {
    EventFilter::matching(event::event_stream_head(stream_id))
}

fn build_affected_object_filter(object_id: &sui_sdk_types::Address) -> TransactionFilter {
    TransactionFilter::matching(transaction::affected_object(*object_id))
}

/// True if buffered events were already scanned up through `next_checkpoint`,
/// so the page-fetch loop is idle and may as well reconcile sooner.
fn page_drain_done(state: &StreamState, next_checkpoint: u64) -> bool {
    state.events_scanned_through.saturating_add(1) >= next_checkpoint
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

/// Backoff for `retry_backoff * attempts + jitter`. Returns `true` if
/// the task should continue, `false` if `max_connect_retries` has been
/// exhausted (in which case the error has been forwarded to the
/// channel).
async fn backoff_or_give_up(
    tx: &mpsc::Sender<Result<AuthenticatedEvent, LightClientError>>,
    config: &AuthenticatedEventsConfig,
    consecutive_failures: &mut u32,
    err: LightClientError,
) -> bool {
    *consecutive_failures += 1;
    if *consecutive_failures > config.max_connect_retries {
        let _ = tx.send(Err(err)).await;
        return false;
    }
    let base = config.retry_backoff.saturating_mul(*consecutive_failures);
    let jitter = pseudo_jitter(*consecutive_failures, config.retry_jitter);
    tokio::time::sleep(base.saturating_add(jitter)).await;
    true
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
