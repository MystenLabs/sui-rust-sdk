// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_types::types::ObjectDigest;
use sui_types::types::ObjectId;
use sui_types::types::TransactionKind;

/// Type representing potentially unresolved object types, with a builder API.
#[derive(Clone, Debug)]
pub struct Object {
    pub id: ObjectId,
    pub kind: Option<TransactionKind>,
    pub version: Option<u64>,
    pub digest: Option<ObjectDigest>,
    pub initial_shared_version: Option<u64>,
    pub mutable: Option<bool>,
}

impl Object {
    // Minimal information
    pub fn by_id(id: ObjectId) -> Self {
        Self {
            id,
            kind: None,
            version: None,
            digest: None,
            initial_shared_version: None,
            mutable: None,
        }
    }

    // Fully resolved
    fn owned(id: ObjectId, v: u64, d: ObjectDigest) -> Self {
        todo!()
    }
    fn immutable(id: ObjectId, v: u64, d: ObjectDigest) -> Self {
        todo!()
    }
    fn receiving(id: ObjectId, v: u64, d: ObjectDigest) -> Self {
        todo!()
    }
    fn shared(id: ObjectId, i: u64, mutable: bool) -> Self {
        todo!()
    }

    // Add partial information

    // Kind
    fn as_owned(self) -> Self {
        todo!()
    }
    // Redundant, but who ever liked saying "imm_or_owned"?
    fn as_immutable(self) -> Self {
        todo!()
    }
    fn as_receiving(self) -> Self {
        todo!()
    }
    fn as_shared(self) -> Self {
        todo!()
    }

    // ObjectRef fields
    fn versioned_at(self, v: u64) -> Self {
        todo!()
    }
    fn with_digest(self, d: ObjectDigest) -> Self {
        todo!()
    }

    // Shared fields

    // Initial shared version
    fn shared_at(self, i: u64) -> Self {
        todo!()
    }

    // Shared value mutability
    fn by_val(self) -> Self {
        todo!()
    }
    fn by_ref(self) -> Self {
        todo!()
    }
    fn by_mut(self) -> Self {
        todo!()
    }
}
