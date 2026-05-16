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
/// stream_id }`.
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
}
