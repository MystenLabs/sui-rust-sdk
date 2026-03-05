// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use bnum::BUintD8;
use serde::Deserialize;
use serde::Serialize;
use sui_sdk_types::Address;
use sui_sdk_types::Digest;
use sui_sdk_types::ObjectReference;

pub type U256 = BUintD8<32>;

pub use sui_sdk_types::EventId;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct EventStreamHead {
    pub mmr: Vec<U256>,
    pub checkpoint_seq: u64,
    pub num_events: u64,
}

impl EventStreamHead {
    pub fn new() -> Self {
        Self {
            mmr: vec![],
            checkpoint_seq: 0,
            num_events: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventCommitment {
    pub checkpoint_seq: u64,
    pub transaction_idx: u64,
    pub event_idx: u64,
    pub digest: Digest,
}

impl EventCommitment {
    pub fn new(checkpoint_seq: u64, transaction_idx: u64, event_idx: u64, digest: Digest) -> Self {
        Self {
            checkpoint_seq,
            transaction_idx,
            event_idx,
            digest,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccumulatorKey {
    pub owner: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field<K, V> {
    pub id: Address,
    pub name: K,
    pub value: V,
}

/// An ObjectRef that is BCS-compatible with `(ObjectID, SequenceNumber, ObjectDigest)` from sui-types.
/// Implements Ord so it can be used in Merkle non-inclusion proofs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderedObjectRef(pub Address, pub u64, pub Digest);

impl OrderedObjectRef {
    pub fn from_object_reference(obj_ref: &ObjectReference) -> Self {
        Self(*obj_ref.object_id(), obj_ref.version(), *obj_ref.digest())
    }

    pub fn object_id(&self) -> &Address {
        &self.0
    }
}

pub fn event_digest(event: &sui_sdk_types::Event) -> Digest {
    use blake2::digest::Digest as _;
    use blake2::digest::consts::U32;

    let bcs_bytes = bcs::to_bytes(event).expect("Event serialization should not fail");
    let mut hasher = blake2::Blake2b::<U32>::new();
    hasher.update(&bcs_bytes);
    let result: [u8; 32] = hasher.finalize().into();
    sui_sdk_types::Digest::new(result)
}

pub fn build_event_merkle_root(events: &[EventCommitment]) -> Digest {
    use sui_crypto::merkle::MerkleTree;

    debug_assert!(
        events.windows(2).all(|w| w[0] <= w[1]),
        "Events must be ordered by (checkpoint_seq, transaction_idx, event_idx)"
    );

    let merkle_tree = MerkleTree::build_from_unserialized(events.to_vec())
        .expect("failed to serialize event commitments for merkle root");
    let root_node = merkle_tree.root();
    let root_digest = root_node.bytes();
    Digest::new(root_digest)
}

pub fn derive_event_stream_head_object_id(stream_id: Address) -> Result<Address, anyhow::Error> {
    use blake2::digest::Digest as _;
    use blake2::digest::consts::U32;

    let sui_accumulator_root_object_id =
        Address::from_static("0x0000000000000000000000000000000000000000000000000000000000000acc");

    let key = AccumulatorKey { owner: stream_id };
    let key_bcs = bcs::to_bytes(&key)?;
    let key_type_tag_bcs = build_accumulator_key_type_tag_bcs();

    // hash(intent_scope || parent || len(key_bcs) || key_bcs || key_type_tag_bcs)
    let mut hasher = blake2::Blake2b::<U32>::new();
    hasher.update([0xf0u8]); // HashingIntentScope::ChildObjectId
    hasher.update(sui_accumulator_root_object_id.as_bytes());
    hasher.update(key_bcs.len().to_le_bytes().as_slice());
    hasher.update(&key_bcs);
    hasher.update(&key_type_tag_bcs);

    let result: [u8; 32] = hasher.finalize().into();
    Ok(Address::new(result))
}

/// BCS-serialize the TypeTag for AccumulatorKey<EventStreamHead>.
///
/// This is equivalent to:
/// ```text
/// TypeTag::Struct(StructTag {
///     address: 0x2,
///     module: "accumulator",
///     name: "Key",
///     type_params: [TypeTag::Struct(StructTag {
///         address: 0x2,
///         module: "accumulator_settlement",
///         name: "EventStreamHead",
///         type_params: [],
///     })]
/// })
/// ```
fn build_accumulator_key_type_tag_bcs() -> Vec<u8> {
    let mut buf = Vec::new();
    bcs_push_struct_tag(
        &mut buf,
        &SUI_FRAMEWORK_ADDRESS,
        "accumulator",
        "Key",
        &[|b: &mut Vec<u8>| {
            bcs_push_struct_tag(
                b,
                &SUI_FRAMEWORK_ADDRESS,
                "accumulator_settlement",
                "EventStreamHead",
                &[],
            );
        }],
    );
    buf
}

const SUI_FRAMEWORK_ADDRESS: [u8; 32] = {
    let mut addr = [0u8; 32];
    addr[31] = 2;
    addr
};

fn bcs_push_struct_tag(
    buf: &mut Vec<u8>,
    address: &[u8; 32],
    module: &str,
    name: &str,
    type_params: &[fn(&mut Vec<u8>)],
) {
    buf.push(7u8); // TypeTag::Struct variant
    buf.extend_from_slice(address);
    bcs_push_string(buf, module);
    bcs_push_string(buf, name);
    bcs_push_uleb128(buf, type_params.len() as u64);
    for param_fn in type_params {
        param_fn(buf);
    }
}

fn bcs_push_string(buf: &mut Vec<u8>, s: &str) {
    bcs_push_uleb128(buf, s.len() as u64);
    buf.extend_from_slice(s.as_bytes());
}

fn bcs_push_uleb128(buf: &mut Vec<u8>, mut value: u64) {
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            buf.push(byte);
            break;
        } else {
            buf.push(byte | 0x80);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumulator_key_type_tag_bcs() {
        let bcs = build_accumulator_key_type_tag_bcs();

        // Expected: TypeTag::Struct(StructTag {
        //   address: 0x2, module: "accumulator", name: "Key",
        //   type_params: [TypeTag::Struct(StructTag {
        //     address: 0x2, module: "accumulator_settlement", name: "EventStreamHead",
        //     type_params: []
        //   })]
        // })
        let mut expected = Vec::new();
        // Outer: TypeTag::Struct variant
        expected.push(0x07);
        // address 0x2 (32 bytes)
        expected.extend_from_slice(&SUI_FRAMEWORK_ADDRESS);
        // module "accumulator" (len=11)
        expected.push(11);
        expected.extend_from_slice(b"accumulator");
        // name "Key" (len=3)
        expected.push(3);
        expected.extend_from_slice(b"Key");
        // type_params length = 1
        expected.push(1);
        // Inner: TypeTag::Struct variant
        expected.push(0x07);
        // address 0x2 (32 bytes)
        expected.extend_from_slice(&SUI_FRAMEWORK_ADDRESS);
        // module "accumulator_settlement" (len=22)
        expected.push(22);
        expected.extend_from_slice(b"accumulator_settlement");
        // name "EventStreamHead" (len=15)
        expected.push(15);
        expected.extend_from_slice(b"EventStreamHead");
        // type_params length = 0
        expected.push(0);

        assert_eq!(bcs, expected, "Type tag BCS mismatch");
    }

    #[test]
    fn test_derive_event_stream_head_object_id_deterministic() {
        let addr = Address::new([0u8; 32]);
        let id1 = derive_event_stream_head_object_id(addr).unwrap();
        let id2 = derive_event_stream_head_object_id(addr).unwrap();
        assert_eq!(id1, id2);
    }
}
