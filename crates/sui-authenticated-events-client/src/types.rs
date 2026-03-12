// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use bnum::BUintD8;
use serde::Deserialize;
use serde::Serialize;
use sui_sdk_types::Address;
use sui_sdk_types::Digest;
use sui_sdk_types::ObjectData;
use sui_sdk_types::hash::Hasher;
use sui_sdk_types::Identifier;
use sui_sdk_types::StructTag;
use sui_sdk_types::TypeTag;

use crate::ClientError;

pub(crate) type U256 = BUintD8<32>;

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
struct AccumulatorKey {
    owner: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Field<K, V> {
    id: Address,
    name: K,
    value: V,
}

pub(crate) fn extract_stream_head_from_object(
    object_bcs: &[u8],
) -> Result<EventStreamHead, ClientError> {
    let object: sui_sdk_types::Object = bcs::from_bytes(object_bcs)
        .map_err(|e| ClientError::InternalError(format!("Failed to deserialize Object: {e}")))?;

    let ObjectData::Struct(move_struct) = object.data() else {
        return Err(ClientError::InternalError("Expected a Move struct, got a package".to_string()));
    };

    let field: Field<AccumulatorKey, EventStreamHead> = bcs::from_bytes(move_struct.contents())
        .map_err(|e| ClientError::InternalError(format!("Failed to deserialize dynamic field: {e}")))?;

    Ok(field.value)
}

pub fn event_digest(event: &sui_sdk_types::Event) -> Digest {
    let bcs_bytes = bcs::to_bytes(event).expect("Event serialization should not fail");
    Hasher::digest(bcs_bytes)
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

const SUI_FRAMEWORK_ADDRESS: Address = {
    let mut addr = [0u8; 32];
    addr[31] = 2;
    Address::new(addr)
};

fn build_accumulator_key_type_tag() -> TypeTag {
    let inner = StructTag::new(
        SUI_FRAMEWORK_ADDRESS,
        Identifier::new("accumulator_settlement").unwrap(),
        Identifier::new("EventStreamHead").unwrap(),
        vec![],
    );
    let outer = StructTag::new(
        SUI_FRAMEWORK_ADDRESS,
        Identifier::new("accumulator").unwrap(),
        Identifier::new("Key").unwrap(),
        vec![TypeTag::Struct(Box::new(inner))],
    );
    TypeTag::Struct(Box::new(outer))
}

pub fn derive_event_stream_head_object_id(stream_id: Address) -> Address {
    let sui_accumulator_root_object_id =
        Address::from_static("0x0000000000000000000000000000000000000000000000000000000000000acc");

    let key = AccumulatorKey { owner: stream_id };
    let key_bcs = bcs::to_bytes(&key).expect("AccumulatorKey serialization should not fail");
    let type_tag = build_accumulator_key_type_tag();

    sui_accumulator_root_object_id.derive_dynamic_child_id(&type_tag, &key_bcs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumulator_key_type_tag_bcs() {
        let type_tag = build_accumulator_key_type_tag();
        let bcs = bcs::to_bytes(&type_tag).unwrap();

        let mut expected = Vec::new();
        let sui_addr = {
            let mut addr = [0u8; 32];
            addr[31] = 2;
            addr
        };
        // Outer: TypeTag::Struct variant
        expected.push(0x07);
        expected.extend_from_slice(&sui_addr);
        expected.push(11);
        expected.extend_from_slice(b"accumulator");
        expected.push(3);
        expected.extend_from_slice(b"Key");
        expected.push(1);
        // Inner: TypeTag::Struct variant
        expected.push(0x07);
        expected.extend_from_slice(&sui_addr);
        expected.push(22);
        expected.extend_from_slice(b"accumulator_settlement");
        expected.push(15);
        expected.extend_from_slice(b"EventStreamHead");
        expected.push(0);

        assert_eq!(bcs, expected, "Type tag BCS mismatch");
    }

    #[test]
    fn test_derive_event_stream_head_object_id_deterministic() {
        let addr = Address::new([0u8; 32]);
        let id1 = derive_event_stream_head_object_id(addr);
        let id2 = derive_event_stream_head_object_id(addr);
        assert_eq!(id1, id2);
    }
}
