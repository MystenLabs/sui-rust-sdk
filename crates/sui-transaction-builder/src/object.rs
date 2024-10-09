// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use sui_types::types::ObjectDigest;
use sui_types::types::ObjectId;
use sui_types::types::ObjectReference;

/// Type representing potentially unresolved object types, with a builder API.
#[derive(Clone, Debug)]
pub struct Object {
    pub id: ObjectId,
    pub kind: Option<Kind>,
    pub version: Option<u64>,
    pub digest: Option<ObjectDigest>,
    pub initial_shared_version: Option<u64>,
    pub mutable: Option<bool>,
}

#[derive(Clone, Debug)]
pub enum Kind {
    ImmOrOwned,
    Receiving,
    Shared,
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
    pub fn owned(id: ObjectId, version: u64, digest: ObjectDigest) -> Self {
        Self {
            id,
            kind: Some(Kind::ImmOrOwned),
            version: Some(version),
            digest: Some(digest),
            initial_shared_version: None,
            mutable: None,
        }
    }

    pub fn immutable(id: ObjectId, version: u64, digest: ObjectDigest) -> Self {
        Self {
            id,
            kind: Some(Kind::ImmOrOwned),
            version: Some(version),
            digest: Some(digest),
            initial_shared_version: None,
            mutable: None,
        }
    }

    pub fn receiving(id: ObjectId, version: u64, digest: ObjectDigest) -> Self {
        Self {
            id,
            kind: Some(Kind::Receiving),
            version: Some(version),
            digest: Some(digest),
            initial_shared_version: None,
            mutable: None,
        }
    }

    pub fn shared(id: ObjectId, initial_shared_version: u64, mutable: bool) -> Self {
        Self {
            id,
            kind: Some(Kind::Shared),
            version: None,
            digest: None,
            initial_shared_version: Some(initial_shared_version),
            mutable: Some(mutable),
        }
    }

    // Add partial information

    // Kind
    pub fn as_owned(self) -> Self {
        Self {
            id: self.id,
            kind: Some(Kind::ImmOrOwned),
            version: self.version,
            digest: self.digest,
            initial_shared_version: self.initial_shared_version,
            mutable: self.mutable,
        }
    }
    // Redundant, but who ever liked saying "imm_or_owned"?
    pub fn as_immutable(self) -> Self {
        Self {
            id: self.id,
            kind: Some(Kind::ImmOrOwned),
            version: self.version,
            digest: self.digest,
            initial_shared_version: self.initial_shared_version,
            mutable: Some(false),
        }
    }

    pub fn as_receiving(self) -> Self {
        todo!()
    }

    pub fn as_shared(self) -> Self {
        todo!()
    }

    // ObjectRef fields
    pub fn versioned_at(self, version: u64) -> Self {
        todo!()
    }

    pub fn with_digest(self, digest: ObjectDigest) -> Self {
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

impl TryInto<ObjectReference> for Object {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<ObjectReference, Self::Error> {
        Ok(ObjectReference::new(
            self.id,
            self.version.ok_or_else(|| anyhow!("version not set"))?,
            self.digest.ok_or_else(|| anyhow!("digest not set"))?,
        ))
    }
}
