// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_sdk_types::hash::Hasher;

use crate::types::EventCommitment;
use crate::types::EventStreamHead;
use crate::types::U256;
use crate::types::build_event_merkle_root;

const U256_ZERO: U256 = U256::ZERO;

fn hash_two_to_one_u256(left: U256, right: U256) -> U256 {
    let mut concatenated = bcs::to_bytes(&left).expect("Failed to serialize left U256");
    concatenated.extend_from_slice(&bcs::to_bytes(&right).expect("Failed to serialize right U256"));
    let digest = Hasher::digest(&concatenated);
    U256::from_le_slice(digest.as_bytes()).expect("32 bytes should always fit U256")
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
) -> Result<EventStreamHead, crate::ClientError> {
    let mut new_head = head.clone();
    for cp_events in events {
        if cp_events.is_empty() {
            return Err(crate::ClientError::InternalError(
                "Empty event batch in stream update".to_string(),
            ));
        }

        let checkpoint_seq = cp_events[0].checkpoint_seq;
        if !cp_events.iter().all(|e| e.checkpoint_seq == checkpoint_seq) {
            return Err(crate::ClientError::InternalError(
                "All events in a batch must share the same checkpoint_seq".to_string(),
            ));
        }
        if checkpoint_seq < new_head.checkpoint_seq && new_head.num_events != 0 {
            return Err(crate::ClientError::InternalError(format!(
                "checkpoint_seq must be non-decreasing: head={}, new={}",
                new_head.checkpoint_seq, checkpoint_seq,
            )));
        }

        let digest = build_event_merkle_root(&cp_events);
        let merkle_root =
            U256::from_le_slice(digest.as_bytes()).expect("32 bytes should always fit U256");
        add_to_stream(&mut new_head.mmr, merkle_root);
        new_head.num_events += cp_events.len() as u64;
        new_head.checkpoint_seq = checkpoint_seq;
    }
    Ok(new_head)
}
