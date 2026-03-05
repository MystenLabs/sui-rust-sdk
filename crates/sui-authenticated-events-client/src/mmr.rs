// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use blake2::digest::Digest as _;
use blake2::digest::consts::U32;

use crate::types::EventCommitment;
use crate::types::EventStreamHead;
use crate::types::U256;
use crate::types::build_event_merkle_root;

const U256_ZERO: U256 = U256::ZERO;

fn hash_two_to_one_u256(left: U256, right: U256) -> U256 {
    let mut concatenated = bcs::to_bytes(&left).expect("Failed to serialize left U256");
    concatenated.extend_from_slice(&bcs::to_bytes(&right).expect("Failed to serialize right U256"));
    let hash: [u8; 32] = blake2::Blake2b::<U32>::digest(&concatenated).into();
    U256::from_le_slice(&hash).expect("32 bytes should always fit U256")
}

fn add_to_stream(mmr: &mut Vec<U256>, new_val: U256) {
    let mut i = 0;
    let mut cur = new_val;

    while i < mmr.len() {
        let r = &mut mmr[i];
        if *r == U256_ZERO {
            *r = cur;
            return;
        } else {
            cur = hash_two_to_one_u256(*r, cur);
            *r = U256_ZERO;
        }
        i += 1;
    }

    mmr.push(cur);
}

pub fn apply_stream_updates(
    head: &EventStreamHead,
    events: Vec<Vec<EventCommitment>>,
) -> EventStreamHead {
    let mut new_head = head.clone();
    for cp_events in events {
        assert!(!cp_events.is_empty());

        let checkpoint_seq = cp_events[0].checkpoint_seq;
        assert!(
            cp_events.iter().all(|e| e.checkpoint_seq == checkpoint_seq),
            "All events in a batch must share the same checkpoint_seq"
        );
        assert!(
            checkpoint_seq >= new_head.checkpoint_seq || new_head.num_events == 0,
            "checkpoint_seq must be non-decreasing: head={}, new={}",
            new_head.checkpoint_seq,
            checkpoint_seq,
        );

        let digest = build_event_merkle_root(&cp_events);
        let merkle_root =
            U256::from_le_slice(digest.as_bytes()).expect("32 bytes should always fit U256");
        add_to_stream(&mut new_head.mmr, merkle_root);
        new_head.num_events += cp_events.len() as u64;
        new_head.checkpoint_seq = checkpoint_seq;
    }
    new_head
}
