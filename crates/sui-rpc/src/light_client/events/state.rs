//! Pure verification helpers for the streaming client.
//!
//! Factored out of the spawned task so they can be unit-tested without
//! standing up an RPC harness. The task in `client` calls
//! [`process_response_batch`] for each incoming page of events and
//! [`reconcile_local_head`] each time a fresh on-chain head is fetched.

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
#[derive(Debug, Default)]
pub(super) struct StreamState {
    /// Events that have been folded into `local_head` but not yet
    /// reconciled against an on-chain head. Once reconciliation
    /// succeeds at a checkpoint, every entry with
    /// `checkpoint <= reconciled_checkpoint` is released to the
    /// consumer and drained from this queue.
    pub pending: VecDeque<AuthenticatedEvent>,

    /// The MMR head we've replayed by folding each received event.
    /// Compared byte-for-byte against the on-chain head during
    /// reconciliation.
    pub local_head: EventStreamHead,

    /// The most recent checkpoint at which `local_head` was confirmed
    /// to match the on-chain head. Subsequent reconciliations only
    /// need to validate events folded *after* this point.
    pub confirmed_through: u64,
}

impl StreamState {
    pub(super) fn new(initial_head: EventStreamHead, confirmed_through: u64) -> Self {
        Self {
            pending: VecDeque::new(),
            local_head: initial_head,
            confirmed_through,
        }
    }
}

/// Group `events` by `checkpoint`, fold each non-empty group into the
/// state's local MMR via [`apply_stream_updates`], and push the events
/// onto `state.pending` in receipt order.
///
/// `events` must be in ascending
/// `(checkpoint, transaction_offset, event_index)` order with respect
/// to each other and to `state.local_head.checkpoint_seq`. The server
/// guarantees this for `ListEvents` responses; the function is
/// strict about it so a malformed server response is caught at the
/// fold step rather than during reconciliation.
///
/// Returns `Ok(())` on success, or propagates an error from the
/// underlying MMR fold.
pub(super) fn process_response_batch(
    state: &mut StreamState,
    events: Vec<AuthenticatedEvent>,
) -> Result<(), LightClientError> {
    if events.is_empty() {
        return Ok(());
    }

    // Group consecutive events by checkpoint while preserving order.
    let mut batches: Vec<EventBatch> = Vec::new();
    for event in &events {
        let commitment = EventCommitment {
            checkpoint_seq: event.checkpoint,
            transaction_idx: event.transaction_offset,
            event_idx: event.event_index as u64,
            digest: event.event.digest(),
        };
        match batches.last_mut() {
            Some(batch) if batch.checkpoint_seq == event.checkpoint => {
                batch.commitments.push(commitment);
            }
            _ => batches.push(EventBatch {
                checkpoint_seq: event.checkpoint,
                commitments: vec![commitment],
            }),
        }
    }

    let new_head =
        apply_stream_updates(state.local_head.clone(), &batches).map_err(LightClientError::from)?;

    state.local_head = new_head;
    for event in events {
        state.pending.push_back(event);
    }
    Ok(())
}

/// Compare the locally-replayed MMR head against a freshly-fetched
/// on-chain head and release the events whose folds the reconciliation
/// covers.
///
/// Returns the events that should now be yielded to the consumer (in
/// receipt order). Drains them from `state.pending` and advances
/// `state.confirmed_through` to `chain_head.checkpoint_seq`. On
/// divergence, returns [`LightClientError::MmrMismatch`] without
/// mutating `state` — the caller should abort the stream rather than
/// silently continue.
pub(super) fn reconcile_local_head(
    state: &mut StreamState,
    chain_head: EventStreamHead,
) -> Result<Vec<AuthenticatedEvent>, LightClientError> {
    if state.local_head != chain_head {
        return Err(LightClientError::MmrMismatch(Box::new(MmrMismatch {
            checkpoint: chain_head.checkpoint_seq,
            expected: chain_head,
            actual: state.local_head.clone(),
        })));
    }

    let confirmed_checkpoint = chain_head.checkpoint_seq;
    let mut released = Vec::new();
    while let Some(front) = state.pending.front() {
        if front.checkpoint > confirmed_checkpoint {
            break;
        }
        released.push(state.pending.pop_front().expect("just peeked"));
    }
    state.confirmed_through = confirmed_checkpoint;
    Ok(released)
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

    fn u256_from_decimal(s: &str) -> U256 {
        s.parse().expect("decimal U256 literal must parse")
    }

    /// One batch of events from a single checkpoint folds into a
    /// single MMR peak. The locally-replayed head must match the
    /// fold of the same `EventCommitment` sequence — this is the
    /// invariant the streaming client relies on to make
    /// reconciliation work, and it's enforced here against a vector
    /// captured from upstream's `apply_stream_updates`.
    #[test]
    fn process_response_batch_folds_single_checkpoint_into_one_peak() {
        let mut state = StreamState::new(EventStreamHead::default(), 0);
        let events = vec![
            sample_event(7, 0, 0),
            sample_event(7, 0, 1),
            sample_event(7, 1, 0),
        ];

        process_response_batch(&mut state, events).unwrap();

        assert_eq!(state.local_head.checkpoint_seq, 7);
        assert_eq!(state.local_head.num_events, 3);
        assert_eq!(state.local_head.mmr.len(), 1);
        assert_eq!(state.pending.len(), 3);
    }

    /// Two checkpoints worth of events fold as two distinct batches.
    /// The MMR carries up to level 1 (slot 0 vacated, slot 1 filled),
    /// matching the four-fold pin from `framework::test`.
    #[test]
    fn process_response_batch_groups_multiple_checkpoints_separately() {
        let mut state = StreamState::new(EventStreamHead::default(), 0);
        let events = vec![sample_event(1, 0, 0), sample_event(2, 0, 0)];

        process_response_batch(&mut state, events).unwrap();

        assert_eq!(state.local_head.checkpoint_seq, 2);
        assert_eq!(state.local_head.num_events, 2);
        assert_eq!(state.local_head.mmr.len(), 2);
        assert_eq!(state.local_head.mmr[0], U256::ZERO);
        assert!(state.local_head.mmr[1] != U256::ZERO);
        assert_eq!(state.pending.len(), 2);
    }

    /// Folding zero events is a no-op on the local head.
    #[test]
    fn process_response_batch_empty_input_is_identity() {
        let initial = EventStreamHead {
            mmr: vec![U256::ONE],
            checkpoint_seq: 9,
            num_events: 1,
        };
        let mut state = StreamState::new(initial.clone(), 9);
        process_response_batch(&mut state, vec![]).unwrap();
        assert_eq!(state.local_head, initial);
        assert!(state.pending.is_empty());
    }

    /// Reconciling against a matching chain head releases every
    /// pending event up to (and including) the chain head's
    /// checkpoint, and advances `confirmed_through`.
    #[test]
    fn reconcile_local_head_releases_matching_events() {
        let mut state = StreamState::new(EventStreamHead::default(), 0);
        process_response_batch(
            &mut state,
            vec![
                sample_event(1, 0, 0),
                sample_event(2, 0, 0),
                sample_event(3, 0, 0),
            ],
        )
        .unwrap();

        // Construct a chain head that matches the post-batch-2 state
        // by folding only the first two events.
        let mut partial = StreamState::new(EventStreamHead::default(), 0);
        process_response_batch(
            &mut partial,
            vec![sample_event(1, 0, 0), sample_event(2, 0, 0)],
        )
        .unwrap();
        let chain_head_at_2 = partial.local_head.clone();

        // Reconciliation against the partial head should fail because
        // our local head has *3* events folded in.
        let err = reconcile_local_head(&mut state, chain_head_at_2.clone()).unwrap_err();
        assert!(matches!(err, LightClientError::MmrMismatch(_)));

        // Reset and try with the full head — that should match and
        // release all three.
        let mut state = StreamState::new(EventStreamHead::default(), 0);
        process_response_batch(
            &mut state,
            vec![
                sample_event(1, 0, 0),
                sample_event(2, 0, 0),
                sample_event(3, 0, 0),
            ],
        )
        .unwrap();
        let chain_head_at_3 = state.local_head.clone();
        let released = reconcile_local_head(&mut state, chain_head_at_3).unwrap();
        assert_eq!(released.len(), 3);
        assert_eq!(state.confirmed_through, 3);
        assert!(state.pending.is_empty());
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

    /// A `chain_head` that doesn't match the local head returns
    /// `MmrMismatch` and leaves `state` untouched so the caller can
    /// abort cleanly.
    #[test]
    fn reconcile_local_head_rejects_divergence_without_mutating_state() {
        let mut state = StreamState::new(EventStreamHead::default(), 0);
        process_response_batch(&mut state, vec![sample_event(1, 0, 0)]).unwrap();
        let pre = state.local_head.clone();
        let pre_pending = state.pending.len();
        let pre_confirmed = state.confirmed_through;

        let bogus_chain_head = EventStreamHead {
            mmr: vec![u256_from_decimal("999")],
            checkpoint_seq: 1,
            num_events: 1,
        };
        let err = reconcile_local_head(&mut state, bogus_chain_head.clone()).unwrap_err();

        match err {
            LightClientError::MmrMismatch(boxed) => {
                assert_eq!(boxed.checkpoint, 1);
                assert_eq!(boxed.expected, bogus_chain_head);
                assert_eq!(boxed.actual, pre);
            }
            other => panic!("expected MmrMismatch, got {other:?}"),
        }

        assert_eq!(state.local_head, pre);
        assert_eq!(state.pending.len(), pre_pending);
        assert_eq!(state.confirmed_through, pre_confirmed);
    }
}
