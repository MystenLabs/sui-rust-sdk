//! Rust definitions of move/sui framework types.

use super::Address;
use super::Object;
use super::TypeTag;
use std::borrow::Cow;

#[cfg(feature = "unstable")]
use super::Digest;
#[cfg(feature = "unstable")]
use super::U256;

#[derive(Debug, Clone)]
pub struct Coin<'a> {
    coin_type: Cow<'a, TypeTag>,
    id: Address,
    balance: u64,
}

impl<'a> Coin<'a> {
    pub fn coin_type(&self) -> &TypeTag {
        &self.coin_type
    }

    pub fn id(&self) -> &Address {
        &self.id
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn try_from_object(object: &'a Object) -> Option<Self> {
        match &object.data {
            super::ObjectData::Struct(move_struct) => {
                let coin_type = move_struct.type_.is_coin()?;

                let contents = &move_struct.contents;
                if contents.len() != Address::LENGTH + std::mem::size_of::<u64>() {
                    return None;
                }

                let id = Address::new((&contents[..Address::LENGTH]).try_into().unwrap());
                let balance =
                    u64::from_le_bytes((&contents[Address::LENGTH..]).try_into().unwrap());

                Some(Self {
                    coin_type: Cow::Borrowed(coin_type),
                    id,
                    balance,
                })
            }
            _ => None, // package
        }
    }

    pub fn into_owned(self) -> Coin<'static> {
        Coin {
            coin_type: Cow::Owned(self.coin_type.into_owned()),
            id: self.id,
            balance: self.balance,
        }
    }
}

/// A commitment to a single event for inclusion in an authenticated event
/// stream's Merkle Mountain Range.
///
/// Each leaf of the per-checkpoint merkle tree is the BCS encoding of this
/// struct. The four fields together identify the event's position in the
/// ledger and bind it to its content:
///
/// - `checkpoint_seq`: the checkpoint containing the emitting transaction.
/// - `transaction_idx`: the emitting transaction's 0-based index within its
///   checkpoint (user transactions are numbered first, settlement transactions
///   continue the same sequence).
/// - `event_idx`: the event's 0-based index within its transaction's event
///   list.
/// - `digest`: the per-event digest, `BLAKE2b-256(BCS of `[`Event`])`.
///
/// Ordering is lexicographic over `(checkpoint_seq, transaction_idx,
/// event_idx)` only; the per-event digest is not part of the comparison since
/// the positional tuple already uniquely identifies an event.
///
/// Mirrors `sui::accumulator_settlement::EventCommitment` on the Move side.
///
/// [`Event`]: crate::Event
///
/// # BCS
///
/// ```text
/// event-commitment = u64 u64 u64 digest
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "unstable")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct EventCommitment {
    pub checkpoint_seq: u64,
    pub transaction_idx: u64,
    pub event_idx: u64,
    pub digest: Digest,
}

#[cfg(feature = "unstable")]
impl PartialOrd for EventCommitment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "unstable")]
impl Ord for EventCommitment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.checkpoint_seq, self.transaction_idx, self.event_idx).cmp(&(
            other.checkpoint_seq,
            other.transaction_idx,
            other.event_idx,
        ))
    }
}

/// On-chain head of an authenticated event stream.
///
/// The framework maintains one of these per event stream as a dynamic field
/// on the accumulator root object (`0xacc`), keyed by
/// `accumulator::Key<accumulator_settlement::EventStreamHead> { owner:
/// stream_id }`. Use [`derive_event_stream_head_object_id`] to compute the
/// dynamic field's object id from the stream id.
///
/// Each settlement transaction that processes events for the stream folds a
/// per-checkpoint merkle tree root into the MMR using carry-propagation. The
/// framework guarantees at most one such settlement per stream per
/// checkpoint, so [`checkpoint_seq`] strictly identifies which checkpoint the
/// head reflects.
///
/// Mirrors `sui::accumulator_settlement::EventStreamHead`.
///
/// [`checkpoint_seq`]: Self::checkpoint_seq
///
/// # BCS
///
/// ```text
/// event-stream-head = vector u256 u64 u64
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "unstable")))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct EventStreamHead {
    /// Merkle Mountain Range peaks, ordered from the lowest level upward.
    /// Empty slots hold `U256::ZERO`; the vector grows by one slot whenever
    /// the carry propagates past the current highest level.
    pub mmr: Vec<U256>,
    /// The latest checkpoint whose events have been folded into the MMR.
    pub checkpoint_seq: u64,
    /// Total number of events ever folded into the MMR.
    pub num_events: u64,
}

/// Compute the object id of the [`EventStreamHead`] dynamic field for a
/// given stream.
///
/// The framework stores each stream's head as a dynamic field on the
/// accumulator root object (`0xacc`). The field is keyed by
/// `sui::accumulator::Key<sui::accumulator_settlement::EventStreamHead>`,
/// with the `owner` field of `Key` set to the stream id. This helper
/// reproduces that derivation so a client can fetch the head via an OCS
/// inclusion proof anchored to a verified checkpoint.
///
/// The BCS encoding of `Key { owner: stream_id }` is identical to that of
/// the bare 32-byte address, since `Key` is a single-field struct over the
/// owner.
///
/// ```
/// use sui_sdk_types::Address;
/// use sui_sdk_types::framework::derive_event_stream_head_object_id;
///
/// // Pinned interop vector cross-verified against
/// // `sui_types::accumulator_root::derive_event_stream_head_object_id`.
/// let stream_id = Address::ZERO;
/// let object_id = derive_event_stream_head_object_id(stream_id);
/// assert_eq!(
///     object_id,
///     Address::from_static("0x9461a724d957b41485e094fdced6c668bd388070108dbfbdc12277ad68a2717f"),
/// );
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "unstable")))]
pub fn derive_event_stream_head_object_id(stream_id: Address) -> Address {
    use super::Identifier;
    use super::StructTag;

    // Parent: the accumulator root object at `0xacc`.
    let accumulator_root = const { Address::from_static("0xacc") };

    // Value type: `sui::accumulator_settlement::EventStreamHead`. Captured
    // inline rather than via a dedicated constant since this is the only
    // call site that needs it.
    let value_type = StructTag::new(
        Address::TWO,
        Identifier::from_static("accumulator_settlement"),
        Identifier::from_static("EventStreamHead"),
        vec![],
    );

    // Key type: `sui::accumulator::Key<EventStreamHead>`.
    let key_type_tag: TypeTag = StructTag::new(
        Address::TWO,
        Identifier::from_static("accumulator"),
        Identifier::from_static("Key"),
        vec![value_type.into()],
    )
    .into();

    // Key bytes: BCS of `AccumulatorKey { owner: stream_id }` reduces to
    // the 32 raw address bytes because BCS encodes a single-field struct
    // identically to the bare field.
    accumulator_root.derive_dynamic_child_id(&key_type_tag, stream_id.as_ref())
}

/// Build the per-checkpoint merkle root over an ordered slice of event
/// commitments.
///
/// Each leaf is the BCS encoding of an [`EventCommitment`], hashed through
/// the standard Blake2b256 leaf/inner-prefix scheme defined in
/// [`crate::merkle`]. The output matches the root the framework computes
/// when sealing a per-checkpoint event tree.
///
/// Callers must provide `commitments` pre-sorted by
/// `(checkpoint_seq, transaction_idx, event_idx)`. The framework folds the
/// sorted sequence into its MMR, so an out-of-order input would yield a
/// root that fails reconciliation. Debug builds verify the ordering with a
/// `debug_assert!`; in release builds, ordering is the caller's
/// responsibility.
///
/// An empty input yields the all-zero "empty" root from [`crate::merkle`].
/// This is a degenerate case the framework never produces — there is no
/// merkle tree to fold when a checkpoint has no events for a stream — but
/// the function itself is defined for any input length so it can be used
/// as a primitive elsewhere.
#[cfg(feature = "unstable")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "unstable")))]
pub fn build_event_merkle_root(commitments: &[EventCommitment]) -> Digest {
    debug_assert!(
        commitments.windows(2).all(|w| w[0] <= w[1]),
        "EventCommitments must be sorted by (checkpoint_seq, transaction_idx, event_idx)",
    );
    let tree = crate::merkle::MerkleTree::build_from_unserialized(commitments.iter())
        .expect("EventCommitment BCS encoding is infallible");
    Digest::new(tree.root().bytes())
}

/// A non-empty group of event commitments folded into an [`EventStreamHead`]
/// as a single MMR update.
///
/// The framework guarantees at most one accumulator settlement per
/// `(checkpoint, stream)` pair, so callers grouping incoming events by
/// `checkpoint_seq` reproduce the on-chain batching exactly. Within a
/// batch, every commitment's `checkpoint_seq` must equal the batch's, and
/// commitments must be sorted by their positional tuple
/// `(checkpoint_seq, transaction_idx, event_idx)`.
#[cfg(feature = "unstable")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "unstable")))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventBatch {
    /// The checkpoint sequence number that produced these commitments.
    pub checkpoint_seq: u64,
    /// Commitments to fold, sorted by their positional tuple. Must be
    /// non-empty and every commitment's `checkpoint_seq` must equal
    /// [`Self::checkpoint_seq`].
    pub commitments: Vec<EventCommitment>,
}

/// Reasons [`apply_stream_updates`] can reject a batch.
#[cfg(feature = "unstable")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "unstable")))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ApplyStreamError {
    /// A batch contained no commitments. The framework never folds an
    /// empty batch into the MMR, so a verifier should not produce one.
    EmptyBatch {
        /// Index into the `batches` slice where the empty batch was found.
        batch_index: usize,
    },

    /// A commitment's `checkpoint_seq` field disagreed with the enclosing
    /// batch's `checkpoint_seq`. A batch corresponds to a single settlement
    /// transaction; all of its commitments share the settlement's
    /// checkpoint.
    CommitmentCheckpointMismatch {
        /// Index into the `batches` slice.
        batch_index: usize,
        /// Index into the offending batch's `commitments` vector.
        commitment_index: usize,
        /// The batch's authoritative checkpoint sequence number.
        batch_checkpoint_seq: u64,
        /// The commitment's contradictory checkpoint sequence number.
        commitment_checkpoint_seq: u64,
    },

    /// A batch's `checkpoint_seq` was strictly less than the head's last
    /// folded checkpoint. Batches must be applied in monotonic order.
    NonMonotonicCheckpoint {
        /// Index into the `batches` slice.
        batch_index: usize,
        /// The head's `checkpoint_seq` before this batch was attempted.
        previous_checkpoint_seq: u64,
        /// The offending batch's `checkpoint_seq`.
        batch_checkpoint_seq: u64,
    },
}

#[cfg(feature = "unstable")]
impl std::fmt::Display for ApplyStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyBatch { batch_index } => {
                write!(f, "batch {batch_index} is empty")
            }
            Self::CommitmentCheckpointMismatch {
                batch_index,
                commitment_index,
                batch_checkpoint_seq,
                commitment_checkpoint_seq,
            } => write!(
                f,
                "batch {batch_index} declares checkpoint {batch_checkpoint_seq} \
                 but commitment {commitment_index} carries checkpoint \
                 {commitment_checkpoint_seq}",
            ),
            Self::NonMonotonicCheckpoint {
                batch_index,
                previous_checkpoint_seq,
                batch_checkpoint_seq,
            } => write!(
                f,
                "batch {batch_index} at checkpoint {batch_checkpoint_seq} would \
                 regress the head's checkpoint {previous_checkpoint_seq}",
            ),
        }
    }
}

#[cfg(feature = "unstable")]
impl std::error::Error for ApplyStreamError {}

/// Fold a sequence of `batches` into `head`, returning the updated head.
///
/// For each batch in order, this:
///
/// 1. Computes the merkle root over the batch's commitments via
///    [`build_event_merkle_root`].
/// 2. Reinterprets the 32-byte root as a [`U256`] in little-endian.
/// 3. Folds the result into `head.mmr` using BLAKE2b-256
///    carry-propagation: while the lowest empty slot is occupied, hash
///    that peak with the carry and try the next slot up; otherwise drop
///    the carry into the slot. If the carry propagates past the highest
///    occupied level, push a new slot.
/// 4. Advances `head.checkpoint_seq` to the batch's checkpoint and
///    increments `head.num_events` by the number of commitments folded.
///
/// The MMR fold matches the on-chain Move implementation byte-for-byte —
/// see `test_mmr_digest_compat_with_rust` under
/// `sui-framework/sources/accumulator_settlement.move`. Any divergence
/// here breaks reconciliation against fetched [`EventStreamHead`]
/// objects.
#[cfg(feature = "unstable")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "unstable")))]
pub fn apply_stream_updates(
    head: EventStreamHead,
    batches: &[EventBatch],
) -> Result<EventStreamHead, ApplyStreamError> {
    let mut head = head;
    for (batch_index, batch) in batches.iter().enumerate() {
        if batch.commitments.is_empty() {
            return Err(ApplyStreamError::EmptyBatch { batch_index });
        }
        for (commitment_index, commitment) in batch.commitments.iter().enumerate() {
            if commitment.checkpoint_seq != batch.checkpoint_seq {
                return Err(ApplyStreamError::CommitmentCheckpointMismatch {
                    batch_index,
                    commitment_index,
                    batch_checkpoint_seq: batch.checkpoint_seq,
                    commitment_checkpoint_seq: commitment.checkpoint_seq,
                });
            }
        }
        // The head has not been folded into yet when `num_events == 0`, so
        // any starting checkpoint is acceptable; only subsequent batches
        // must be monotonic relative to the prior fold.
        if head.num_events != 0 && batch.checkpoint_seq < head.checkpoint_seq {
            return Err(ApplyStreamError::NonMonotonicCheckpoint {
                batch_index,
                previous_checkpoint_seq: head.checkpoint_seq,
                batch_checkpoint_seq: batch.checkpoint_seq,
            });
        }

        let root_digest = build_event_merkle_root(&batch.commitments);
        let merkle_root = U256::from_digits(root_digest.into_inner());
        fold_into_mmr(&mut head.mmr, merkle_root);
        head.num_events += batch.commitments.len() as u64;
        head.checkpoint_seq = batch.checkpoint_seq;
    }
    Ok(head)
}

/// MMR carry-propagation fold.
///
/// Walks `mmr` from the lowest level upward. If a slot is empty
/// (`U256::ZERO`), drop the carry into it and stop. Otherwise hash the
/// existing peak together with the carry, clear the slot, and continue
/// with the result as the new carry at the next level up. If the carry
/// propagates past the highest occupied slot, append a new one.
#[cfg(feature = "unstable")]
fn fold_into_mmr(mmr: &mut Vec<U256>, mut carry: U256) {
    let mut i = 0;
    while i < mmr.len() {
        if mmr[i] == U256::ZERO {
            mmr[i] = carry;
            return;
        }
        carry = hash_two_to_one(mmr[i], carry);
        mmr[i] = U256::ZERO;
        i += 1;
    }
    mmr.push(carry);
}

/// Compute `BLAKE2b-256(bcs(left) || bcs(right))` and reinterpret the
/// 32-byte digest as a `U256` in little-endian.
///
/// `U256`'s BCS encoding is its 32 little-endian bytes verbatim (see
/// [`U256`]'s `Serialize` impl), so `digits()` is byte-equivalent to
/// `bcs::to_bytes(&u256)` and we avoid two throwaway allocations per
/// fold by hashing the digit slices directly.
#[cfg(feature = "unstable")]
fn hash_two_to_one(left: U256, right: U256) -> U256 {
    use super::hash::Hasher;

    let mut hasher = Hasher::new();
    hasher.update(left.digits());
    hasher.update(right.digits());
    U256::from_digits(hasher.finalize().into_inner())
}

#[cfg(test)]
#[cfg(feature = "unstable")]
mod test {
    use super::*;

    // Pinned BCS encoding of an `EventCommitment`. The shape must agree with
    // `sui::accumulator_settlement::EventCommitment` upstream byte-for-byte,
    // since this is what the framework hashes to produce the per-checkpoint
    // merkle root.
    #[test]
    fn event_commitment_bcs_shape() {
        let commitment = EventCommitment {
            checkpoint_seq: 0x0102030405060708,
            transaction_idx: 0x1112131415161718,
            event_idx: 0x2122232425262728,
            digest: Digest::new([0xaa; 32]),
        };

        let mut expected = Vec::new();
        expected.extend_from_slice(&0x0102030405060708u64.to_le_bytes());
        expected.extend_from_slice(&0x1112131415161718u64.to_le_bytes());
        expected.extend_from_slice(&0x2122232425262728u64.to_le_bytes());
        // The `Digest` BCS shape is length-prefixed: a single ULEB128 byte
        // `0x20` (=32) followed by the 32 digest bytes.
        expected.push(0x20);
        expected.extend_from_slice(&[0xaa; 32]);

        let bytes = bcs::to_bytes(&commitment).unwrap();
        assert_eq!(bytes, expected);
        assert_eq!(bytes.len(), 8 + 8 + 8 + 33);

        let back: EventCommitment = bcs::from_bytes(&bytes).unwrap();
        assert_eq!(back, commitment);
    }

    // `EventCommitment` ordering matches the ledger-position tuple
    // `(checkpoint_seq, transaction_idx, event_idx)` and ignores the digest
    // so that two distinct digests sharing the same position compare equal
    // under the ordering. This mirrors the upstream `Ord` impl, which is
    // what the framework relies on when sorting commitments before
    // building the per-checkpoint merkle tree.
    #[test]
    fn event_commitment_ord_ignores_digest() {
        let a = EventCommitment {
            checkpoint_seq: 1,
            transaction_idx: 2,
            event_idx: 3,
            digest: Digest::new([0x00; 32]),
        };
        let b = EventCommitment {
            digest: Digest::new([0xff; 32]),
            ..a
        };
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);

        let c = EventCommitment { event_idx: 4, ..a };
        assert!(a < c);
    }

    // Pinned BCS encoding of an `EventStreamHead`. As with
    // `EventCommitment`, the framework reads/writes this struct byte-for-byte
    // on chain; any divergence here breaks every consumer's ability to
    // reconcile a locally-replayed MMR against a fetched head.
    #[test]
    fn event_stream_head_bcs_shape() {
        let head = EventStreamHead {
            mmr: vec![U256::ZERO, U256::ONE],
            checkpoint_seq: 0x4142434445464748,
            num_events: 0x5152535455565758,
        };

        let mut expected = Vec::new();
        // `mmr`: ULEB128 length (=2), then two 32-byte little-endian `u256`s.
        expected.push(0x02);
        expected.extend_from_slice(&[0u8; 32]);
        let mut one_le = [0u8; 32];
        one_le[0] = 1;
        expected.extend_from_slice(&one_le);
        expected.extend_from_slice(&0x4142434445464748u64.to_le_bytes());
        expected.extend_from_slice(&0x5152535455565758u64.to_le_bytes());

        let bytes = bcs::to_bytes(&head).unwrap();
        assert_eq!(bytes, expected);

        let back: EventStreamHead = bcs::from_bytes(&bytes).unwrap();
        assert_eq!(back, head);
    }

    #[test]
    fn event_stream_head_default_is_empty() {
        let head = EventStreamHead::default();
        assert!(head.mmr.is_empty());
        assert_eq!(head.checkpoint_seq, 0);
        assert_eq!(head.num_events, 0);
    }

    // Cross-implementation pin: the expected merkle root was captured by
    // running `sui_types::accumulator_root::build_event_merkle_root`
    // upstream on the same three commitments. A regression in either the
    // `EventCommitment` BCS shape or the underlying merkle tree
    // construction (leaf/inner prefix, padding, hash function) would shift
    // this digest. The `merkle` module's root construction and
    // `event_commitment_bcs_shape` above already pin those pieces against
    // their respective upstream sources, so this test seals the
    // composition end-to-end.
    #[test]
    fn build_event_merkle_root_pinned_vector() {
        let commitments = vec![
            EventCommitment {
                checkpoint_seq: 1,
                transaction_idx: 0,
                event_idx: 0,
                digest: Digest::new([0x11; 32]),
            },
            EventCommitment {
                checkpoint_seq: 1,
                transaction_idx: 0,
                event_idx: 1,
                digest: Digest::new([0x22; 32]),
            },
            EventCommitment {
                checkpoint_seq: 1,
                transaction_idx: 1,
                event_idx: 0,
                digest: Digest::new([0x33; 32]),
            },
        ];
        const EXPECTED: [u8; 32] = [
            254, 183, 87, 247, 72, 14, 90, 116, 221, 195, 244, 87, 250, 236, 226, 161, 99, 106,
            199, 246, 85, 138, 180, 110, 112, 50, 103, 77, 160, 104, 239, 61,
        ];
        assert_eq!(build_event_merkle_root(&commitments).into_inner(), EXPECTED);
    }

    #[test]
    fn build_event_merkle_root_empty_input_is_empty_node() {
        // Documented degenerate case: the merkle tree over zero leaves is the
        // all-zero "empty" node. `apply_stream_updates` rejects this so a
        // verifier would never reach it via the normal path, but the helper
        // is defined for arbitrary input and should not panic.
        assert_eq!(build_event_merkle_root(&[]).into_inner(), [0u8; 32]);
    }

    // Cross-implementation pin: each `(stream_id, object_id)` pair was
    // captured by running upstream's
    // `sui_types::accumulator_root::derive_event_stream_head_object_id`
    // directly on the listed stream id. The derivation composes
    // `Address::derive_dynamic_child_id` (validated against the Move-side
    // snapshot test) with a fixed `StructTag`, so a regression here would
    // most likely indicate an unintended change to either the type tag or
    // the parent accumulator root address.
    #[test]
    fn derive_event_stream_head_object_id_pinned_vectors() {
        let cases: &[(Address, Address)] = &[
            (
                Address::ZERO,
                Address::from_static(
                    "0x9461a724d957b41485e094fdced6c668bd388070108dbfbdc12277ad68a2717f",
                ),
            ),
            (
                Address::TWO,
                Address::from_static(
                    "0x1b877f5c7664df8957f127a95d1b2c8c1c239fd49566f9f69205df44133fc37f",
                ),
            ),
            (
                Address::from_static("0xacc"),
                Address::from_static(
                    "0x452652326e8df295af20a4e0744acac9a74f87d93ba976dd97d3e93e1a542e37",
                ),
            ),
            (
                Address::from_static("0x42424242"),
                Address::from_static(
                    "0xdbe2cd3f24c357c434991a4348e6ceb43bcb7d22696eb6797a6739e5351cb149",
                ),
            ),
        ];
        for (stream_id, expected) in cases {
            assert_eq!(
                derive_event_stream_head_object_id(*stream_id),
                *expected,
                "mismatch for stream id {stream_id}",
            );
        }
    }

    fn u256_from_decimal(s: &str) -> U256 {
        s.parse().expect("decimal U256 literal must parse")
    }

    fn single_event_batch(checkpoint_seq: u64, digest_byte: u8) -> EventBatch {
        EventBatch {
            checkpoint_seq,
            commitments: vec![EventCommitment {
                checkpoint_seq,
                transaction_idx: 0,
                event_idx: 0,
                digest: Digest::new([digest_byte; 32]),
            }],
        }
    }

    // Load-bearing interop pin against the Move test
    // `test_mmr_digest_compat_with_rust` in
    // `sui-framework/sources/accumulator_settlement.move`. Inserting
    // `U256::from(50..58)` as eight successive carries must collapse to a
    // single peak at level 3 with the exact decimal value below; the lower
    // three slots are zeroed. Any divergence in the carry-propagation
    // loop or the `BLAKE2b-256(bcs(left) || bcs(right))` two-to-one step
    // breaks reconciliation against on-chain `EventStreamHead` objects.
    #[test]
    fn fold_into_mmr_matches_move_compat_fixture() {
        let mut mmr = Vec::new();
        for value in 50u64..58 {
            fold_into_mmr(&mut mmr, U256::from(value));
        }
        assert_eq!(mmr.len(), 4);
        assert_eq!(mmr[0], U256::ZERO);
        assert_eq!(mmr[1], U256::ZERO);
        assert_eq!(mmr[2], U256::ZERO);
        assert_eq!(
            mmr[3],
            u256_from_decimal(
                "69725770072863840208899320192042305265295220676851872214494910464384102654361",
            ),
        );
    }

    // Two carries collapse into one peak at level 1, leaving level 0
    // empty. Stand-alone sanity check that the carry propagation is wired
    // correctly even for tiny inputs.
    #[test]
    fn fold_into_mmr_two_inserts_collapse_to_level_one() {
        let mut mmr = Vec::new();
        fold_into_mmr(&mut mmr, U256::from(7u64));
        assert_eq!(mmr, vec![U256::from(7u64)]);

        fold_into_mmr(&mut mmr, U256::from(11u64));
        assert_eq!(mmr.len(), 2);
        assert_eq!(mmr[0], U256::ZERO);
        assert_eq!(mmr[1], hash_two_to_one(U256::from(7u64), U256::from(11u64)));
    }

    // End-to-end interop pin: feed a 3-commitment batch through
    // `apply_stream_updates` starting from an empty head and compare to
    // the `EventStreamHead` produced by
    // `sui_light_client::authenticated_events::mmr::apply_stream_updates`
    // upstream on the identical input.
    #[test]
    fn apply_stream_updates_single_batch_matches_upstream() {
        let batch = EventBatch {
            checkpoint_seq: 1,
            commitments: vec![
                EventCommitment {
                    checkpoint_seq: 1,
                    transaction_idx: 0,
                    event_idx: 0,
                    digest: Digest::new([0x11; 32]),
                },
                EventCommitment {
                    checkpoint_seq: 1,
                    transaction_idx: 0,
                    event_idx: 1,
                    digest: Digest::new([0x22; 32]),
                },
                EventCommitment {
                    checkpoint_seq: 1,
                    transaction_idx: 1,
                    event_idx: 0,
                    digest: Digest::new([0x33; 32]),
                },
            ],
        };
        let head = apply_stream_updates(EventStreamHead::default(), &[batch]).unwrap();
        assert_eq!(head.checkpoint_seq, 1);
        assert_eq!(head.num_events, 3);
        assert_eq!(
            head.mmr,
            vec![u256_from_decimal(
                "28014082315424315761761458464083312323394111104237010481447392654866601457662",
            )],
        );
    }

    // End-to-end interop pin across two sequential batches at different
    // checkpoints. The merkle root from batch 2 carries up through level 0
    // and lands at level 1, vacating slot 0.
    #[test]
    fn apply_stream_updates_two_batches_match_upstream() {
        let batch_1 = EventBatch {
            checkpoint_seq: 1,
            commitments: vec![
                EventCommitment {
                    checkpoint_seq: 1,
                    transaction_idx: 0,
                    event_idx: 0,
                    digest: Digest::new([0x11; 32]),
                },
                EventCommitment {
                    checkpoint_seq: 1,
                    transaction_idx: 0,
                    event_idx: 1,
                    digest: Digest::new([0x22; 32]),
                },
                EventCommitment {
                    checkpoint_seq: 1,
                    transaction_idx: 1,
                    event_idx: 0,
                    digest: Digest::new([0x33; 32]),
                },
            ],
        };
        let batch_2 = EventBatch {
            checkpoint_seq: 2,
            commitments: vec![
                EventCommitment {
                    checkpoint_seq: 2,
                    transaction_idx: 0,
                    event_idx: 0,
                    digest: Digest::new([0x44; 32]),
                },
                EventCommitment {
                    checkpoint_seq: 2,
                    transaction_idx: 0,
                    event_idx: 1,
                    digest: Digest::new([0x55; 32]),
                },
            ],
        };
        let head = apply_stream_updates(EventStreamHead::default(), &[batch_1, batch_2]).unwrap();
        assert_eq!(head.checkpoint_seq, 2);
        assert_eq!(head.num_events, 5);
        assert_eq!(head.mmr.len(), 2);
        assert_eq!(head.mmr[0], U256::ZERO);
        assert_eq!(
            head.mmr[1],
            u256_from_decimal(
                "80180905428222716273420959625814881301112107405105460786291242224918309625423",
            ),
        );
    }

    // Four single-event batches drive the carry up through two levels,
    // leaving levels 0 and 1 empty and one peak at level 2. End-to-end pin
    // against upstream.
    #[test]
    fn apply_stream_updates_four_single_batches_match_upstream() {
        let batches: Vec<EventBatch> = (1u64..=4)
            .map(|cp| single_event_batch(cp, cp as u8))
            .collect();
        let head = apply_stream_updates(EventStreamHead::default(), &batches).unwrap();
        assert_eq!(head.checkpoint_seq, 4);
        assert_eq!(head.num_events, 4);
        assert_eq!(head.mmr.len(), 3);
        assert_eq!(head.mmr[0], U256::ZERO);
        assert_eq!(head.mmr[1], U256::ZERO);
        assert_eq!(
            head.mmr[2],
            u256_from_decimal(
                "43434128249102587327404298804800250101556402749045331898264216785541514599480",
            ),
        );
    }

    // Folding zero batches into an arbitrary head must return the head
    // unchanged.
    #[test]
    fn apply_stream_updates_no_batches_is_identity() {
        let head = EventStreamHead {
            mmr: vec![U256::ONE, U256::ZERO, U256::from(42u64)],
            checkpoint_seq: 17,
            num_events: 9,
        };
        let out = apply_stream_updates(head.clone(), &[]).unwrap();
        assert_eq!(out, head);
    }

    // Same-checkpoint re-application is permitted (a stream can produce
    // multiple settlement transactions in one checkpoint in unrelated
    // accumulator object spaces, although per-stream the framework
    // guarantees only one — the SDK helper still allows equal
    // `checkpoint_seq` to keep the contract symmetric with strict less-than
    // being the violation).
    #[test]
    fn apply_stream_updates_equal_checkpoint_seq_is_allowed() {
        let head = apply_stream_updates(
            EventStreamHead::default(),
            &[single_event_batch(5, 0x01), single_event_batch(5, 0x02)],
        )
        .unwrap();
        assert_eq!(head.checkpoint_seq, 5);
        assert_eq!(head.num_events, 2);
    }

    #[test]
    fn apply_stream_updates_rejects_empty_batch() {
        let err = apply_stream_updates(
            EventStreamHead::default(),
            &[
                single_event_batch(1, 0x01),
                EventBatch {
                    checkpoint_seq: 2,
                    commitments: vec![],
                },
            ],
        )
        .unwrap_err();
        assert_eq!(err, ApplyStreamError::EmptyBatch { batch_index: 1 });
    }

    #[test]
    fn apply_stream_updates_rejects_commitment_checkpoint_mismatch() {
        let err = apply_stream_updates(
            EventStreamHead::default(),
            &[EventBatch {
                checkpoint_seq: 5,
                commitments: vec![
                    EventCommitment {
                        checkpoint_seq: 5,
                        transaction_idx: 0,
                        event_idx: 0,
                        digest: Digest::new([0x01; 32]),
                    },
                    EventCommitment {
                        checkpoint_seq: 6, // wrong
                        transaction_idx: 0,
                        event_idx: 1,
                        digest: Digest::new([0x02; 32]),
                    },
                ],
            }],
        )
        .unwrap_err();
        assert_eq!(
            err,
            ApplyStreamError::CommitmentCheckpointMismatch {
                batch_index: 0,
                commitment_index: 1,
                batch_checkpoint_seq: 5,
                commitment_checkpoint_seq: 6,
            },
        );
    }

    #[test]
    fn apply_stream_updates_rejects_non_monotonic_checkpoint() {
        let err = apply_stream_updates(
            EventStreamHead::default(),
            &[single_event_batch(10, 0x01), single_event_batch(9, 0x02)],
        )
        .unwrap_err();
        assert_eq!(
            err,
            ApplyStreamError::NonMonotonicCheckpoint {
                batch_index: 1,
                previous_checkpoint_seq: 10,
                batch_checkpoint_seq: 9,
            },
        );
    }
}
