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
}
