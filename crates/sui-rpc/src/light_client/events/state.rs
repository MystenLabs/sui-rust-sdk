//! Pure verification helpers for the streaming client.
//!
//! Factored out of the spawned task so they can be unit-tested without
//! standing up an RPC harness. The streaming task in `client` calls
//! [`buffer_response_batch`] for each incoming page of events and
//! [`fold_and_reconcile`] each reconciliation tick — passing in the
//! settlement boundaries fetched from `ListTransactions(affected_object =
//! event_stream_head)` so events are folded into the local MMR in the
//! same per-settlement partitions the framework used on chain.

use std::collections::VecDeque;

use sui_sdk_types::Object;
use sui_sdk_types::framework::EventBatch;
use sui_sdk_types::framework::EventCommitment;
use sui_sdk_types::framework::EventStreamHead;
use sui_sdk_types::framework::apply_stream_updates;

use super::envelope::AuthenticatedEvent;
use crate::light_client::error::LightClientError;
use crate::light_client::error::MmrMismatch;

/// In-memory state the streaming task carries across iterations of the
/// list-events / reconcile loop.
///
/// Events are buffered (not folded) as `ListEvents` pages arrive: the
/// framework can run multiple `settle_events` transactions per
/// checkpoint (one per consensus commit per stream), and folding events
/// into the wrong settlement bucket produces an MMR shape that will
/// never match the on-chain head. The buffer is flushed at
/// reconciliation time, once `ListTransactions(affected_object =
/// event_stream_head)` has surfaced the per-settlement boundaries.
#[derive(Debug, Default)]
pub(super) struct StreamState {
    /// Events received from `ListEvents` but not yet folded into
    /// [`Self::local_head`]. Once a reconciliation tick succeeds, every
    /// entry with `checkpoint <= reconciled_checkpoint` is folded into
    /// the local MMR, released to the consumer, and drained from this
    /// queue.
    ///
    /// Items are stored in receipt order — which matches strictly
    /// ascending `(checkpoint, transaction_offset, event_index)` per the
    /// v2alpha contract.
    pub buffer: VecDeque<AuthenticatedEvent>,

    /// The MMR head we've replayed by folding the settlement-partitioned
    /// event batches. Compared byte-for-byte against the on-chain head
    /// during reconciliation.
    pub local_head: EventStreamHead,

    /// The most recent checkpoint at which `local_head` was confirmed
    /// to match the on-chain head. Subsequent reconciliations only need
    /// to fold events landed strictly after this point.
    pub confirmed_through: u64,

    /// Highest checkpoint number whose events `ListEvents` has fully
    /// emitted to us, derived from the latest `Watermark.checkpoint`
    /// across both standalone watermarks and per-item watermarks.
    ///
    /// Reconciliation uses this as the upper bound when fetching
    /// settlements: settlements at checkpoints we haven't fully scanned
    /// for events would otherwise produce an empty fold (with the head
    /// stamping a different `num_events`) and fail reconciliation.
    pub events_scanned_through: u64,
}

impl StreamState {
    pub(super) fn new(initial_head: EventStreamHead, confirmed_through: u64) -> Self {
        Self {
            buffer: VecDeque::new(),
            local_head: initial_head,
            confirmed_through,
            events_scanned_through: confirmed_through,
        }
    }
}

/// Append `events` to `state.buffer` and advance
/// `state.events_scanned_through` to `watermark_hi.max(events.last())` if
/// either is greater than the current value.
///
/// `events` must be in strictly ascending
/// `(checkpoint, transaction_offset, event_index)` order with respect to
/// each other and to whatever's already at the back of `state.buffer` —
/// the v2alpha contract guarantees this for `ListEvents` responses, and
/// the fold-time partitioner relies on it. Empty input is a no-op.
///
/// `watermark_hi` is the most recent `Watermark.checkpoint` observed
/// during this page, including the watermark embedded on the last item.
/// Pass `None` when the page produced neither a standalone watermark nor
/// an item-embedded one (e.g., an immediate `End` frame at genesis).
pub(super) fn buffer_response_batch(
    state: &mut StreamState,
    events: Vec<AuthenticatedEvent>,
    watermark_hi: Option<u64>,
) {
    let last_event_cp = events.last().map(|e| e.checkpoint);
    for event in events {
        state.buffer.push_back(event);
    }

    if let Some(hi) = watermark_hi {
        state.events_scanned_through = state.events_scanned_through.max(hi);
    }
    if let Some(cp) = last_event_cp {
        // Defensive: even if no watermark frame surfaced for this page,
        // every emitted event implies the scan reached its checkpoint.
        state.events_scanned_through = state.events_scanned_through.max(cp);
    }
}

/// Partition the events at the front of `state.buffer` by settlement,
/// fold each partition into `state.local_head`, and reconcile the
/// updated head against `chain_head` at the last settlement's
/// checkpoint.
///
/// `settlements` is the ascending sequence of `(checkpoint,
/// transaction_offset)` pairs from `ListTransactions(affected_object =
/// event_stream_head)`, covering `(state.confirmed_through,
/// reconcile_checkpoint]`. `reconcile_checkpoint` is the checkpoint at
/// which `chain_head` was authenticated — by construction the
/// checkpoint of the last entry in `settlements`.
///
/// Each event in the partitioned prefix is assigned to the next
/// settlement whose `(cp, tx_offset)` lies at-or-after the event's
/// position, and a settlement reject is a hard error (it indicates the
/// stream's `ListEvents` and `ListTransactions` indexes have diverged).
///
/// On a matching reconciliation: drains the folded events from
/// `state.buffer`, advances `state.confirmed_through` to
/// `reconcile_checkpoint`, and returns the released events in receipt
/// order. On any mismatch: returns the error without mutating the
/// observable parts of `state` — the caller should abort the stream.
pub(super) fn fold_and_reconcile(
    state: &mut StreamState,
    settlements: &[(u64, u64)],
    chain_head: EventStreamHead,
    reconcile_checkpoint: u64,
) -> Result<Vec<AuthenticatedEvent>, LightClientError> {
    // Settlements must be non-empty and the last one's checkpoint must
    // equal the reconciliation point — that's the load-bearing invariant
    // the caller is contracted to uphold.
    let last_settlement =
        settlements
            .last()
            .copied()
            .ok_or(LightClientError::UnexpectedObjectShape {
                reason: "fold_and_reconcile called with no settlements; \
                     reconciliation needs at least one settlement to anchor the chain head",
            })?;
    if last_settlement.0 != reconcile_checkpoint {
        return Err(LightClientError::UnexpectedObjectShape {
            reason: "settlement boundaries disagree with reconciliation checkpoint",
        });
    }

    // Count the buffered events whose checkpoint lies at or below the
    // reconciliation point — these are the ones we'll partition and
    // fold. Anything past `reconcile_checkpoint` stays buffered for the
    // next round.
    let fold_count = state
        .buffer
        .iter()
        .take_while(|e| e.checkpoint <= reconcile_checkpoint)
        .count();

    let batches = bucket_events_by_settlement(
        state.buffer.iter().take(fold_count),
        settlements,
        state.confirmed_through,
    )?;

    let new_head =
        apply_stream_updates(state.local_head.clone(), &batches).map_err(LightClientError::from)?;

    if new_head != chain_head {
        return Err(LightClientError::MmrMismatch(Box::new(MmrMismatch {
            checkpoint: reconcile_checkpoint,
            expected: chain_head,
            actual: new_head,
        })));
    }

    // Reconciliation succeeded: commit the fold and drain the released
    // events.
    state.local_head = new_head;
    state.confirmed_through = reconcile_checkpoint;
    let released: Vec<AuthenticatedEvent> = state.buffer.drain(..fold_count).collect();
    Ok(released)
}

/// Bucket `events` into per-settlement [`EventBatch`]es using the
/// ascending `settlements` boundary list. Returns an empty `Vec` only if
/// `events` is empty after the `floor_checkpoint` filter — every kept
/// event must map to a settlement at its own checkpoint, or the function
/// errors.
///
/// Mirrors the upstream e2e helper that drives the on-chain
/// reconciliation: each event maps to the first settlement with `(cp ==
/// event.cp, tx_offset >= event.tx_offset)`, and events sharing a
/// settlement key form a single batch.
fn bucket_events_by_settlement<'a, I>(
    events: I,
    settlements: &[(u64, u64)],
    floor_checkpoint: u64,
) -> Result<Vec<EventBatch>, LightClientError>
where
    I: IntoIterator<Item = &'a AuthenticatedEvent>,
{
    let mut batches: Vec<EventBatch> = Vec::new();
    let mut current_key: Option<(u64, u64)> = None;
    let mut settlement_idx: usize = 0;

    for event in events {
        // Skip events that fall at-or-below the floor — those have
        // already been folded into `state.local_head` by an earlier
        // reconciliation (or are part of the initial-state head).
        if event.checkpoint <= floor_checkpoint {
            continue;
        }

        // Advance to the first settlement at-or-past the event's
        // ledger position.
        while settlement_idx < settlements.len()
            && (settlements[settlement_idx].0 < event.checkpoint
                || (settlements[settlement_idx].0 == event.checkpoint
                    && settlements[settlement_idx].1 < event.transaction_offset))
        {
            settlement_idx += 1;
        }

        if settlement_idx >= settlements.len() || settlements[settlement_idx].0 != event.checkpoint
        {
            // The framework guarantees a `settle_events` transaction
            // for every checkpoint that emitted events on this stream;
            // a missing settlement means the server's event index and
            // transaction index have diverged.
            return Err(LightClientError::UnexpectedObjectShape {
                reason: "no settlement transaction covering buffered event",
            });
        }

        let settlement_key = settlements[settlement_idx];
        let commitment = EventCommitment {
            checkpoint_seq: event.checkpoint,
            transaction_idx: event.transaction_offset,
            event_idx: event.event_index as u64,
            digest: event.event.digest(),
        };

        match current_key {
            Some(key) if key == settlement_key => {
                batches
                    .last_mut()
                    .expect("current_key set ⇒ at least one batch exists")
                    .commitments
                    .push(commitment);
            }
            _ => {
                batches.push(EventBatch {
                    checkpoint_seq: event.checkpoint,
                    commitments: vec![commitment],
                });
                current_key = Some(settlement_key);
            }
        }
    }

    Ok(batches)
}

/// Extract the BCS-encoded [`EventStreamHead`] out of an on-chain
/// `Object` returned by the OCS inclusion proof flow.
///
/// The framework stores each stream head as the value of a dynamic
/// field on the accumulator root object. The Move-side type is
/// `sui::dynamic_field::Field<sui::accumulator::Key<EventStreamHead>,
/// EventStreamHead>`, which BCS-serializes as:
///
/// ```text
/// field = uid key value
///
/// uid   = address                 ; 32 bytes (Field.id)
/// key   = address                 ; 32 bytes (AccumulatorKey { owner: address })
/// value = bytes-of-EventStreamHead
/// ```
///
/// Both the UID and the single-field-struct `AccumulatorKey` are
/// fixed 32-byte addresses, so the head's BCS bytes start at offset
/// 64. We BCS-decode just the tail rather than parsing through the
/// dynamic-field wrapper to keep this independent of any generic
/// `Field<K, V>` representation in the SDK.
pub(super) fn extract_event_stream_head(
    object: &Object,
) -> Result<EventStreamHead, LightClientError> {
    const HEAD_OFFSET: usize = 64;

    let move_struct = object
        .as_struct()
        .ok_or(LightClientError::UnexpectedObjectShape {
            reason: "expected a Move struct (dynamic field), got a package",
        })?;
    let contents = move_struct.contents();
    let head_bytes =
        contents
            .get(HEAD_OFFSET..)
            .ok_or(LightClientError::UnexpectedObjectShape {
                reason: "dynamic-field contents too short for EventStreamHead value",
            })?;
    bcs::from_bytes(head_bytes).map_err(LightClientError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sui_sdk_types::Address;
    use sui_sdk_types::Digest;
    use sui_sdk_types::Event;
    use sui_sdk_types::Identifier;
    use sui_sdk_types::StructTag;
    use sui_sdk_types::U256;

    fn sample_event(checkpoint: u64, tx_idx: u64, event_idx: u32) -> AuthenticatedEvent {
        AuthenticatedEvent {
            checkpoint,
            transaction_offset: tx_idx,
            event_index: event_idx,
            transaction_digest: Digest::new([0xaa; 32]),
            event: Event {
                package_id: Address::TWO,
                module: Identifier::from_static("m"),
                sender: Address::TWO,
                type_: StructTag::new(
                    Address::TWO,
                    Identifier::from_static("m"),
                    Identifier::from_static("E"),
                    vec![],
                ),
                contents: vec![checkpoint as u8, tx_idx as u8, event_idx as u8],
            },
        }
    }

    /// Build the chain-head state expected after folding `events`
    /// bucketed against `settlements`. Mirrors what the test cluster
    /// would produce on chain so reconciliation calls can be exercised
    /// against an authoritative reference.
    fn expected_chain_head(
        events: &[AuthenticatedEvent],
        settlements: &[(u64, u64)],
    ) -> EventStreamHead {
        let batches = bucket_events_by_settlement(events.iter(), settlements, 0).unwrap();
        apply_stream_updates(EventStreamHead::default(), &batches).unwrap()
    }

    fn u256_from_decimal(s: &str) -> U256 {
        s.parse().expect("decimal U256 literal must parse")
    }

    /// `buffer_response_batch` only buffers — folding is deferred to
    /// `fold_and_reconcile`. The local head must not move just from
    /// receiving events.
    #[test]
    fn buffer_response_batch_defers_folding() {
        let mut state = StreamState::new(EventStreamHead::default(), 0);
        let events = vec![sample_event(7, 0, 0), sample_event(7, 0, 1)];

        buffer_response_batch(&mut state, events, Some(7));

        assert_eq!(state.local_head, EventStreamHead::default());
        assert_eq!(state.buffer.len(), 2);
        assert_eq!(state.events_scanned_through, 7);
    }

    /// Events arriving without a watermark still bump
    /// `events_scanned_through` to at least the last event's
    /// checkpoint — otherwise reconciliation would never see those
    /// checkpoints as "scanned" and skip them when fetching
    /// settlements.
    #[test]
    fn buffer_response_batch_advances_scan_floor_from_events() {
        let mut state = StreamState::new(EventStreamHead::default(), 0);
        buffer_response_batch(&mut state, vec![sample_event(11, 0, 0)], None);
        assert_eq!(state.events_scanned_through, 11);
    }

    /// `events_scanned_through` is monotonic — a smaller watermark
    /// (e.g., a retry that re-emits earlier items) does not regress it.
    #[test]
    fn buffer_response_batch_does_not_regress_scan_floor() {
        let mut state = StreamState::new(EventStreamHead::default(), 0);
        state.events_scanned_through = 20;
        buffer_response_batch(&mut state, vec![sample_event(5, 0, 0)], Some(5));
        assert_eq!(state.events_scanned_through, 20);
    }

    /// Single settlement per checkpoint — the legacy case the previous
    /// implementation also supported. All events at one checkpoint
    /// fold into a single batch and the chain head matches.
    #[test]
    fn fold_and_reconcile_single_settlement_per_checkpoint() {
        let events = vec![
            sample_event(7, 0, 0),
            sample_event(7, 0, 1),
            sample_event(7, 1, 0),
        ];
        let settlements = vec![(7u64, 2u64)];

        let chain_head = expected_chain_head(&events, &settlements);

        let mut state = StreamState::new(EventStreamHead::default(), 0);
        buffer_response_batch(&mut state, events.clone(), Some(7));

        let released = fold_and_reconcile(&mut state, &settlements, chain_head.clone(), 7).unwrap();

        assert_eq!(released, events);
        assert_eq!(state.local_head, chain_head);
        assert_eq!(state.confirmed_through, 7);
        assert!(state.buffer.is_empty());
    }

    /// Multiple settlements per checkpoint — the failure mode that
    /// motivated this whole module. Three transactions in checkpoint
    /// 10 settle in two `settle_events` calls, so events 0–1 fold as
    /// one batch and event 2 folds as another. The chain head must
    /// match only if both batches are folded separately.
    #[test]
    fn fold_and_reconcile_multiple_settlements_per_checkpoint() {
        let events = vec![
            sample_event(10, 0, 0),
            sample_event(10, 1, 0),
            sample_event(10, 3, 0),
        ];
        // First two events settle at txn 2; the third settles at txn 4.
        let settlements = vec![(10u64, 2u64), (10u64, 4u64)];

        let chain_head = expected_chain_head(&events, &settlements);

        let mut state = StreamState::new(EventStreamHead::default(), 0);
        buffer_response_batch(&mut state, events.clone(), Some(10));

        // Sanity-check that the two-batch fold differs from the
        // (incorrect) single-batch fold the old code would have
        // produced — this is the regression we're guarding against.
        let single_batch = apply_stream_updates(
            EventStreamHead::default(),
            &[EventBatch {
                checkpoint_seq: 10,
                commitments: events
                    .iter()
                    .map(|e| EventCommitment {
                        checkpoint_seq: e.checkpoint,
                        transaction_idx: e.transaction_offset,
                        event_idx: e.event_index as u64,
                        digest: e.event.digest(),
                    })
                    .collect(),
            }],
        )
        .unwrap();
        assert_ne!(single_batch, chain_head);

        let released =
            fold_and_reconcile(&mut state, &settlements, chain_head.clone(), 10).unwrap();

        assert_eq!(released, events);
        assert_eq!(state.local_head, chain_head);
        assert_eq!(state.confirmed_through, 10);
    }

    /// Reconciliation only consumes the buffer prefix at-or-below the
    /// reconciliation point; events past it stay buffered for the next
    /// round.
    #[test]
    fn fold_and_reconcile_keeps_unfolded_tail_buffered() {
        let events = vec![
            sample_event(5, 0, 0),
            sample_event(5, 0, 1),
            sample_event(9, 0, 0), // past the reconciliation point
        ];
        // Only the cp=5 settlement is in range; the cp=9 settlement
        // hasn't been queried yet.
        let settlements = vec![(5u64, 0u64)];

        // Construct a chain head that represents folding only the cp=5
        // events — this is what `prove_object_at_checkpoint(.., 5)`
        // would return on chain.
        let cp5_events: Vec<_> = events.iter().take(2).cloned().collect();
        let chain_head_at_5 = expected_chain_head(&cp5_events, &settlements);

        let mut state = StreamState::new(EventStreamHead::default(), 0);
        buffer_response_batch(&mut state, events.clone(), Some(9));

        let released =
            fold_and_reconcile(&mut state, &settlements, chain_head_at_5.clone(), 5).unwrap();

        assert_eq!(released, cp5_events);
        assert_eq!(state.local_head, chain_head_at_5);
        assert_eq!(state.confirmed_through, 5);
        assert_eq!(state.buffer.len(), 1);
        assert_eq!(state.buffer.front().unwrap().checkpoint, 9);
    }

    /// A chain head that doesn't match the locally-folded head returns
    /// `MmrMismatch` and does NOT advance `confirmed_through` or drain
    /// the buffer.
    #[test]
    fn fold_and_reconcile_rejects_divergence_without_mutating_observable_state() {
        let events = vec![sample_event(3, 0, 0)];
        let settlements = vec![(3u64, 0u64)];
        let bogus = EventStreamHead {
            mmr: vec![u256_from_decimal("999")],
            checkpoint_seq: 3,
            num_events: 1,
        };

        let mut state = StreamState::new(EventStreamHead::default(), 0);
        buffer_response_batch(&mut state, events.clone(), Some(3));
        let pre_local_head = state.local_head.clone();
        let pre_buffer_len = state.buffer.len();
        let pre_confirmed = state.confirmed_through;

        let err = fold_and_reconcile(&mut state, &settlements, bogus.clone(), 3).unwrap_err();
        assert!(matches!(err, LightClientError::MmrMismatch(_)));

        assert_eq!(state.local_head, pre_local_head);
        assert_eq!(state.buffer.len(), pre_buffer_len);
        assert_eq!(state.confirmed_through, pre_confirmed);
    }

    /// Calling `fold_and_reconcile` with no settlements is a hard
    /// error — the caller is contracted to only invoke it after
    /// `ListTransactions` has surfaced at least one settlement in the
    /// scan range.
    #[test]
    fn fold_and_reconcile_rejects_empty_settlements() {
        let mut state = StreamState::new(EventStreamHead::default(), 0);
        let err = fold_and_reconcile(&mut state, &[], EventStreamHead::default(), 0).unwrap_err();
        assert!(matches!(
            err,
            LightClientError::UnexpectedObjectShape { .. }
        ));
    }

    /// The reconciliation checkpoint must match the last settlement's
    /// checkpoint — otherwise the local fold would terminate at a
    /// different head than the on-chain proof attests to.
    #[test]
    fn fold_and_reconcile_rejects_misaligned_reconcile_checkpoint() {
        let settlements = vec![(5u64, 0u64)];
        let mut state = StreamState::new(EventStreamHead::default(), 0);
        let err = fold_and_reconcile(&mut state, &settlements, EventStreamHead::default(), 7)
            .unwrap_err();
        assert!(matches!(
            err,
            LightClientError::UnexpectedObjectShape { .. }
        ));
    }

    /// A buffered event whose checkpoint has no settlement in range is
    /// a server-side index inconsistency — surface it as
    /// `UnexpectedObjectShape` rather than fold silently into the
    /// wrong bucket.
    #[test]
    fn fold_and_reconcile_rejects_event_without_matching_settlement() {
        // Event at cp=5 but settlement only exists at cp=7.
        let events = vec![sample_event(5, 0, 0)];
        let settlements = vec![(7u64, 0u64)];
        let mut state = StreamState::new(EventStreamHead::default(), 0);
        buffer_response_batch(&mut state, events, Some(7));

        let err = fold_and_reconcile(&mut state, &settlements, EventStreamHead::default(), 7)
            .unwrap_err();
        assert!(matches!(
            err,
            LightClientError::UnexpectedObjectShape { .. }
        ));
    }

    /// `extract_event_stream_head` skips the dynamic-field UID and
    /// `AccumulatorKey` prefix (64 bytes total) and BCS-decodes the
    /// tail. Synthesize a `MoveStruct` whose `contents` start with two
    /// arbitrary 32-byte addresses followed by the BCS bytes of a
    /// known `EventStreamHead`, then confirm we recover that exact
    /// head. The 64-byte offset is the load-bearing assumption here;
    /// shift it by one byte and the BCS decode fails.
    #[test]
    fn extract_event_stream_head_skips_uid_and_key_prefix() {
        use sui_sdk_types::Object;
        use sui_sdk_types::ObjectData;
        use sui_sdk_types::Owner;
        use sui_sdk_types::Version;
        use sui_sdk_types::framework::derive_event_stream_head_object_id;

        let head = EventStreamHead {
            mmr: vec![U256::ZERO, U256::ONE],
            checkpoint_seq: 17,
            num_events: 5,
        };
        let stream_id = Address::TWO;

        // Synthesize the dynamic-field contents: UID = derived object
        // id, AccumulatorKey = stream_id, then the EventStreamHead
        // bytes.
        let mut contents = Vec::new();
        let uid = derive_event_stream_head_object_id(stream_id);
        contents.extend_from_slice(uid.as_bytes());
        contents.extend_from_slice(stream_id.as_bytes());
        contents.extend(bcs::to_bytes(&head).unwrap());

        let move_struct = sui_sdk_types::MoveStruct::new(
            StructTag::new(
                Address::TWO,
                Identifier::from_static("dynamic_field"),
                Identifier::from_static("Field"),
                vec![],
            ),
            true,
            Version::from(1u64),
            contents,
        )
        .expect("contents are at least 32 bytes");
        let object = Object::new(
            ObjectData::Struct(move_struct),
            Owner::Address(Address::ZERO),
            Digest::new([0; 32]),
            0,
        );

        let recovered = extract_event_stream_head(&object).unwrap();
        assert_eq!(recovered, head);
    }

    /// Extracting from a package (not a Move struct) is a clean
    /// `UnexpectedObjectShape` error rather than a panic.
    #[test]
    fn extract_event_stream_head_rejects_non_struct() {
        use sui_sdk_types::MovePackage;
        use sui_sdk_types::Object;
        use sui_sdk_types::ObjectData;
        use sui_sdk_types::Owner;
        use sui_sdk_types::Version;

        let pkg = MovePackage {
            id: Address::TWO,
            version: Version::from(1u64),
            modules: Default::default(),
            type_origin_table: Vec::new(),
            linkage_table: Default::default(),
        };
        let object = Object::new(
            ObjectData::Package(pkg),
            Owner::Immutable,
            Digest::new([0; 32]),
            0,
        );

        let err = extract_event_stream_head(&object).unwrap_err();
        assert!(matches!(
            err,
            LightClientError::UnexpectedObjectShape { .. }
        ));
    }
}
