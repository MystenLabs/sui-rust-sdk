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
use super::state::extract_event_stream_head;
use super::state::process_response_batch;
use super::state::reconcile_local_head;
use crate::light_client::LightClient;
use crate::light_client::error::LightClientError;
use crate::proto::sui::rpc::v2alpha::EventFilter;
use crate::proto::sui::rpc::v2alpha::EventLiteral;
use crate::proto::sui::rpc::v2alpha::EventPredicate;
use crate::proto::sui::rpc::v2alpha::EventStreamHeadFilter;
use crate::proto::sui::rpc::v2alpha::EventTerm;
use crate::proto::sui::rpc::v2alpha::ListEventsRequest;
use crate::proto::sui::rpc::v2alpha::QueryEndReason;
use crate::proto::sui::rpc::v2alpha::QueryOptions;
use crate::proto::sui::rpc::v2alpha::event_literal;
use crate::proto::sui::rpc::v2alpha::event_predicate;
use crate::proto::sui::rpc::v2alpha::list_events_response;

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
/// Steady state: page through `ListEvents`, fold each page into the
/// local MMR, and at each [`AuthenticatedEventsConfig::head_check_interval`]
/// tick fetch the on-chain head and reconcile. On match, drain the
/// confirmed events through the channel. On mismatch, send the error
/// and exit.
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
    let filter = build_filter(&config.stream_id.to_string());

    loop {
        // Decide whether to fetch the next page or reconcile.
        let should_reconcile = last_head_check.elapsed() >= config.head_check_interval
            || !state.pending.is_empty()
                && next_cursor.is_none()
                && page_drain_done(&state, next_checkpoint);

        if should_reconcile {
            match reconcile_once(&mut light, &mut state, &stream_head_object_id).await {
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
                limit_items: Some(config.page_size),
                after: next_cursor.clone(),
                before: None,
                ordering: 0, // ascending (default)
            }),
        };

        match fetch_one_page(&mut light, request).await {
            Ok(page) => {
                consecutive_failures = 0;
                let PageResult {
                    events,
                    end_cursor,
                    end_reason,
                } = page;

                if let Err(e) = process_response_batch(&mut state, events) {
                    let _ = tx.send(Err(e)).await;
                    return;
                }

                match end_reason {
                    Some(QueryEndReason::ItemLimit) => {
                        next_cursor = end_cursor;
                    }
                    _ => {
                        // Server signalled tip / scan-limit / range bound.
                        // Reset cursor and bump start checkpoint to one past
                        // the latest fold so the next page picks up new
                        // events as they arrive.
                        next_cursor = None;
                        next_checkpoint = state
                            .local_head
                            .checkpoint_seq
                            .checked_add(1)
                            .unwrap_or(next_checkpoint);
                        // Sleep briefly so we don't spin when at the tip.
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
async fn initial_state(
    light: &mut LightClient,
    config: &AuthenticatedEventsConfig,
) -> Result<InitialState, LightClientError> {
    let latest_tip = light.latest_checkpoint_seq().await?;
    let stream_head_object_id = derive_event_stream_head_object_id(config.stream_id);

    // Attempt to fetch and authenticate the current head at the tip.
    // If the object doesn't exist yet (no events ever folded into this
    // stream), fall back to the tip as the resume point.
    let head_result = light
        .verify_object_at_checkpoint(&stream_head_object_id, latest_tip)
        .await;

    let (initial_head, start_checkpoint) = match head_result {
        Ok((_object_ref, object)) => {
            let head = extract_event_stream_head(&object)?;
            let cp = head.checkpoint_seq;
            (head, cp)
        }
        // If the proof RPC reports NotFound, treat that as "no head
        // yet" and start from the tip. Any other error is a real
        // failure that the caller should see.
        Err(LightClientError::Rpc(status)) if status.code() == tonic::Code::NotFound => (
            EventStreamHead::default(),
            config.start_checkpoint.unwrap_or(latest_tip),
        ),
        Err(e) => return Err(e),
    };

    let start_checkpoint = config.start_checkpoint.unwrap_or(start_checkpoint);
    Ok(InitialState {
        initial_head,
        start_checkpoint,
    })
}

struct PageResult {
    events: Vec<AuthenticatedEvent>,
    end_cursor: Option<prost::bytes::Bytes>,
    end_reason: Option<QueryEndReason>,
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
    let mut end_cursor = None;
    let mut end_reason = None;

    while let Some(frame) = stream.next().await {
        let frame = frame?;
        match frame.response {
            Some(list_events_response::Response::Item(item)) => {
                let ev = AuthenticatedEvent::try_from(&item)?;
                events.push(ev);
            }
            Some(list_events_response::Response::End(end)) => {
                end_cursor = end.cursor;
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
    })
}

async fn reconcile_once(
    light: &mut LightClient,
    state: &mut StreamState,
    stream_head_object_id: &sui_sdk_types::Address,
) -> Result<Vec<AuthenticatedEvent>, LightClientError> {
    let tip = light.latest_checkpoint_seq().await?;
    let (_object_ref, object) = light
        .verify_object_at_checkpoint(stream_head_object_id, tip)
        .await?;
    let chain_head = extract_event_stream_head(&object)?;
    reconcile_local_head(state, chain_head)
}

fn build_filter(stream_id_hex: &str) -> EventFilter {
    EventFilter {
        terms: vec![EventTerm {
            literals: vec![EventLiteral {
                polarity: Some(event_literal::Polarity::Include(EventPredicate {
                    predicate: Some(event_predicate::Predicate::EventStreamHead(
                        EventStreamHeadFilter {
                            stream_id: Some(stream_id_hex.to_string()),
                        },
                    )),
                })),
            }],
        }],
    }
}

/// True if pending events were already drained up through the last
/// folded checkpoint, so we're idle and may as well reconcile sooner.
fn page_drain_done(state: &StreamState, next_checkpoint: u64) -> bool {
    state.local_head.checkpoint_seq < next_checkpoint
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
