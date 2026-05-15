mod v1;
mod v2;

pub use v1::ModifiedAtVersion;
pub use v1::ObjectReferenceWithOwner;
pub use v1::TransactionEffectsV1;
pub use v2::AccumulatorOperation;
pub use v2::AccumulatorValue;
pub use v2::AccumulatorWrite;
pub use v2::ChangedObject;
pub use v2::IdOperation;
pub use v2::ObjectIn;
pub use v2::ObjectOut;
pub use v2::TransactionEffectsV2;
pub use v2::UnchangedConsensusKind;
pub use v2::UnchangedConsensusObject;

use crate::Address;
use crate::Digest;
use crate::execution_status::ExecutionStatus;
use crate::object::Owner;
use crate::object::Version;

/// The output or effects of executing a transaction
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// transaction-effects =  %x00 effects-v1
///                     =/ %x01 effects-v2
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum TransactionEffects {
    V1(Box<TransactionEffectsV1>),
    V2(Box<TransactionEffectsV2>),
}

impl TransactionEffects {
    /// Return the status of the transaction.
    pub fn status(&self) -> &ExecutionStatus {
        match self {
            TransactionEffects::V1(e) => e.status(),
            TransactionEffects::V2(e) => e.status(),
        }
    }

    /// Return the epoch in which this transaction was executed.
    pub fn epoch(&self) -> u64 {
        match self {
            TransactionEffects::V1(e) => e.epoch(),
            TransactionEffects::V2(e) => e.epoch(),
        }
    }

    /// Return the gas cost summary of the transaction.
    pub fn gas_summary(&self) -> &crate::gas::GasCostSummary {
        match self {
            TransactionEffects::V1(e) => e.gas_summary(),
            TransactionEffects::V2(e) => e.gas_summary(),
        }
    }

    /// Return the digest of the transaction that produced these effects.
    pub fn transaction_digest(&self) -> &Digest {
        match self {
            TransactionEffects::V1(e) => &e.transaction_digest,
            TransactionEffects::V2(e) => &e.transaction_digest,
        }
    }

    /// Return the digest of the events emitted by this transaction, or `None`
    /// if the transaction did not emit any events.
    pub fn events_digest(&self) -> Option<&Digest> {
        match self {
            TransactionEffects::V1(e) => e.events_digest.as_ref(),
            TransactionEffects::V2(e) => e.events_digest.as_ref(),
        }
    }

    /// Iterate over the per-object changes reported by this transaction.
    ///
    /// Yields one [`ObjectChange`] for every object affected by the
    /// transaction — created, mutated, unwrapped, deleted, wrapped, or
    /// unwrapped-then-deleted. V1 effects drain in this order: `created`,
    /// `mutated`, `unwrapped`, `deleted`, `unwrapped_then_deleted`,
    /// `wrapped`. V2 effects drain in the order they appear in
    /// `changed_objects`.
    ///
    /// Accumulator writes are not surfaced here, since they describe a
    /// write to an accumulator rather than a change to an ordinary object.
    ///
    /// This is the "base" view used to verify a transaction's effects.
    /// Narrower iterators (e.g. one that yields only [`ObjectReference`]s
    /// written by this transaction, for building an OCS Merkle leaf set)
    /// can be composed on top via `.filter(...)` / `.filter_map(...)`.
    ///
    /// V1 effects do not store input digests or input owners, so the
    /// optional input fields are always `None` on V1.
    ///
    /// [`ObjectReference`]: crate::ObjectReference
    pub fn object_changes(&self) -> impl Iterator<Item = ObjectChange<'_>> + '_ {
        ObjectChanges::new(self)
    }
}

/// A per-object change reported by a transaction's effects.
///
/// This is a unified, classified view over both V1 and V2 effects. Each
/// variant carries the fields that are meaningful for that kind of change.
///
/// `Address`, `Digest`, and `Owner` fields are borrowed from the underlying
/// effects rather than copied, so the enum is parameterised by the lifetime
/// of the borrowed [`TransactionEffects`]. `Version` (a `u64`) is returned
/// by value, since a reference to it would not be any smaller.
///
/// Fields that V1 effects do not store (input digests, input owners) are
/// modelled as `Option`, and are always `None` for V1-sourced changes.
///
/// The enum is `#[non_exhaustive]` because the Sui protocol may add new kinds
/// of object change in the future; pattern matches on it should include a
/// catch-all arm.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ObjectChange<'a> {
    /// A new object was created by this transaction.
    Created {
        object_id: &'a Address,
        output_version: Version,
        output_digest: &'a Digest,
        output_owner: &'a Owner,
    },

    /// An existing object's contents (and possibly owner) were modified by
    /// this transaction.
    Mutated {
        object_id: &'a Address,
        /// The object's version before execution.
        ///
        /// Always `Some` on V2 effects (read from `ObjectIn::Exist`). On V1
        /// effects this is looked up in the effects' `modified_at_versions`
        /// table, which may be absent for malformed effects; in that case
        /// it is `None`.
        input_version: Option<Version>,
        /// The object's digest before execution. `None` on V1 effects.
        input_digest: Option<&'a Digest>,
        /// The object's owner before execution. `None` on V1 effects.
        input_owner: Option<&'a Owner>,
        output_version: Version,
        output_digest: &'a Digest,
        output_owner: &'a Owner,
    },

    /// A previously wrapped object was extracted (unwrapped) by this
    /// transaction and now appears at the root level of the object store.
    Unwrapped {
        object_id: &'a Address,
        output_version: Version,
        output_digest: &'a Digest,
        output_owner: &'a Owner,
    },

    /// An object was deleted by this transaction.
    Deleted {
        object_id: &'a Address,
        /// The object's version before deletion. See [`ObjectChange::Mutated`]
        /// for when this may be `None`.
        input_version: Option<Version>,
        input_digest: Option<&'a Digest>,
        input_owner: Option<&'a Owner>,
    },

    /// An object was wrapped inside another object by this transaction (and
    /// is therefore no longer at the root level of the object store).
    Wrapped {
        object_id: &'a Address,
        /// The object's version before being wrapped. See
        /// [`ObjectChange::Mutated`] for when this may be `None`.
        input_version: Option<Version>,
        input_digest: Option<&'a Digest>,
        input_owner: Option<&'a Owner>,
    },

    /// An object was unwrapped and then deleted in the same transaction.
    /// Neither the prior nor the new state is recorded.
    UnwrappedThenDeleted { object_id: &'a Address },
}

impl<'a> ObjectChange<'a> {
    /// The id of the object affected by this change.
    pub fn object_id(&self) -> &'a Address {
        match self {
            Self::Created { object_id, .. }
            | Self::Mutated { object_id, .. }
            | Self::Unwrapped { object_id, .. }
            | Self::Deleted { object_id, .. }
            | Self::Wrapped { object_id, .. }
            | Self::UnwrappedThenDeleted { object_id } => object_id,
        }
    }
}

/// Private iterator state behind [`TransactionEffects::object_changes`].
///
/// V1 drains in this order: `created`, `mutated`, `unwrapped`, `deleted`,
/// `unwrapped_then_deleted`, `wrapped`. V2 drains in the order entries appear
/// in `changed_objects`, skipping `(input_state, output_state, id_operation)`
/// triples that do not match any known kind (forward-compatible with future
/// protocol additions).
enum ObjectChanges<'a> {
    V1 {
        /// Slice borrowed from `TransactionEffectsV1::modified_at_versions`.
        /// Walking `mutated`, `deleted`, and `wrapped` entries needs the
        /// prior version for each object id from here. The lookup is a
        /// linear scan rather than a `BTreeMap` so that the iterator
        /// allocates nothing — V1 effects typically carry only a handful of
        /// modified-at entries, so the scan beats an upfront `BTreeMap`
        /// build cost.
        modified_at: &'a [ModifiedAtVersion],
        created: std::slice::Iter<'a, ObjectReferenceWithOwner>,
        mutated: std::slice::Iter<'a, ObjectReferenceWithOwner>,
        unwrapped: std::slice::Iter<'a, ObjectReferenceWithOwner>,
        deleted: std::slice::Iter<'a, crate::ObjectReference>,
        unwrapped_then_deleted: std::slice::Iter<'a, crate::ObjectReference>,
        wrapped: std::slice::Iter<'a, crate::ObjectReference>,
    },
    V2 {
        lamport_version: Version,
        inner: std::slice::Iter<'a, ChangedObject>,
    },
}

/// Linear scan of `modified_at_versions` for the prior version of `id`.
fn find_modified_at(modified_at: &[ModifiedAtVersion], id: &Address) -> Option<Version> {
    modified_at
        .iter()
        .find(|m| &m.object_id == id)
        .map(|m| m.version)
}

impl<'a> ObjectChanges<'a> {
    fn new(effects: &'a TransactionEffects) -> Self {
        match effects {
            TransactionEffects::V1(e) => Self::V1 {
                modified_at: &e.modified_at_versions,
                created: e.created.iter(),
                mutated: e.mutated.iter(),
                unwrapped: e.unwrapped.iter(),
                deleted: e.deleted.iter(),
                unwrapped_then_deleted: e.unwrapped_then_deleted.iter(),
                wrapped: e.wrapped.iter(),
            },
            TransactionEffects::V2(e) => Self::V2 {
                lamport_version: e.lamport_version,
                inner: e.changed_objects.iter(),
            },
        }
    }
}

impl<'a> Iterator for ObjectChanges<'a> {
    type Item = ObjectChange<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::V1 {
                modified_at,
                created,
                mutated,
                unwrapped,
                deleted,
                unwrapped_then_deleted,
                wrapped,
            } => {
                if let Some(o) = created.next() {
                    return Some(ObjectChange::Created {
                        object_id: o.reference.object_id(),
                        output_version: o.reference.version(),
                        output_digest: o.reference.digest(),
                        output_owner: &o.owner,
                    });
                }
                if let Some(o) = mutated.next() {
                    let object_id = o.reference.object_id();
                    return Some(ObjectChange::Mutated {
                        object_id,
                        input_version: find_modified_at(modified_at, object_id),
                        input_digest: None,
                        input_owner: None,
                        output_version: o.reference.version(),
                        output_digest: o.reference.digest(),
                        output_owner: &o.owner,
                    });
                }
                if let Some(o) = unwrapped.next() {
                    return Some(ObjectChange::Unwrapped {
                        object_id: o.reference.object_id(),
                        output_version: o.reference.version(),
                        output_digest: o.reference.digest(),
                        output_owner: &o.owner,
                    });
                }
                if let Some(r) = deleted.next() {
                    let object_id = r.object_id();
                    return Some(ObjectChange::Deleted {
                        object_id,
                        input_version: find_modified_at(modified_at, object_id),
                        input_digest: None,
                        input_owner: None,
                    });
                }
                if let Some(r) = unwrapped_then_deleted.next() {
                    return Some(ObjectChange::UnwrappedThenDeleted {
                        object_id: r.object_id(),
                    });
                }
                if let Some(r) = wrapped.next() {
                    let object_id = r.object_id();
                    return Some(ObjectChange::Wrapped {
                        object_id,
                        input_version: find_modified_at(modified_at, object_id),
                        input_digest: None,
                        input_owner: None,
                    });
                }
                None
            }
            Self::V2 {
                lamport_version,
                inner,
            } => {
                for changed in inner.by_ref() {
                    if let Some(change) = v2_object_change(changed, *lamport_version) {
                        return Some(change);
                    }
                }
                None
            }
        }
    }
}

/// `Owner::Immutable` materialised in static memory so that variants which
/// require `&'a Owner` (e.g. `ObjectChange::Created::output_owner`) can hand
/// out a borrow when classifying a Move package write — packages are always
/// immutable but `ObjectOut::PackageWrite` does not carry an `Owner` field.
static IMMUTABLE_OWNER: Owner = Owner::Immutable;

/// Classify a single V2 [`ChangedObject`] into an [`ObjectChange`].
///
/// Returns `None` for `(input_state, output_state, id_operation)` triples
/// that do not match any known kind (forward-compatibility with future
/// protocol additions). All currently-defined combinations are handled.
fn v2_object_change<'a>(
    changed: &'a ChangedObject,
    lamport_version: Version,
) -> Option<ObjectChange<'a>> {
    let object_id = &changed.object_id;
    match (
        &changed.input_state,
        &changed.output_state,
        changed.id_operation,
    ) {
        // Created: nothing before, an object now exists.
        (ObjectIn::NotExist, ObjectOut::ObjectWrite { digest, owner }, IdOperation::Created) => {
            Some(ObjectChange::Created {
                object_id,
                output_version: lamport_version,
                output_digest: digest,
                output_owner: owner,
            })
        }

        // Created package: surfaced as `Created` with `Owner::Immutable`.
        // `PackageWrite` carries its own `version` (packages start at 1 and
        // increment by 1 on upgrade) rather than reusing `lamport_version`.
        (ObjectIn::NotExist, ObjectOut::PackageWrite { version, digest }, IdOperation::Created) => {
            Some(ObjectChange::Created {
                object_id,
                output_version: *version,
                output_digest: digest,
                output_owner: &IMMUTABLE_OWNER,
            })
        }

        // Mutated: existed before, exists after (same id).
        (
            ObjectIn::Exist {
                version: input_version,
                digest: input_digest,
                owner: input_owner,
            },
            ObjectOut::ObjectWrite { digest, owner },
            IdOperation::None,
        ) => Some(ObjectChange::Mutated {
            object_id,
            input_version: Some(*input_version),
            input_digest: Some(input_digest),
            input_owner: Some(input_owner),
            output_version: lamport_version,
            output_digest: digest,
            output_owner: owner,
        }),

        // Upgraded package: surfaced as `Mutated` with `Owner::Immutable`.
        // See the package-create arm above for why `version` comes from
        // `PackageWrite` rather than `lamport_version`.
        (
            ObjectIn::Exist {
                version: input_version,
                digest: input_digest,
                owner: input_owner,
            },
            ObjectOut::PackageWrite { version, digest },
            IdOperation::None,
        ) => Some(ObjectChange::Mutated {
            object_id,
            input_version: Some(*input_version),
            input_digest: Some(input_digest),
            input_owner: Some(input_owner),
            output_version: *version,
            output_digest: digest,
            output_owner: &IMMUTABLE_OWNER,
        }),

        // Unwrapped: nothing at root before, an object now exists, id is not new.
        (ObjectIn::NotExist, ObjectOut::ObjectWrite { digest, owner }, IdOperation::None) => {
            Some(ObjectChange::Unwrapped {
                object_id,
                output_version: lamport_version,
                output_digest: digest,
                output_owner: owner,
            })
        }

        // Deleted: existed before, gone now, id is deleted.
        (
            ObjectIn::Exist {
                version: input_version,
                digest: input_digest,
                owner: input_owner,
            },
            ObjectOut::NotExist,
            IdOperation::Deleted,
        ) => Some(ObjectChange::Deleted {
            object_id,
            input_version: Some(*input_version),
            input_digest: Some(input_digest),
            input_owner: Some(input_owner),
        }),

        // Unwrapped then deleted: nothing before, nothing after, id is deleted.
        (ObjectIn::NotExist, ObjectOut::NotExist, IdOperation::Deleted) => {
            Some(ObjectChange::UnwrappedThenDeleted { object_id })
        }

        // Wrapped: existed at root before, no longer at root, id still alive.
        (
            ObjectIn::Exist {
                version: input_version,
                digest: input_digest,
                owner: input_owner,
            },
            ObjectOut::NotExist,
            IdOperation::None,
        ) => Some(ObjectChange::Wrapped {
            object_id,
            input_version: Some(*input_version),
            input_digest: Some(input_digest),
            input_owner: Some(input_owner),
        }),

        // Accumulator writes are not surfaced as `ObjectChange`s; they
        // describe a write to an accumulator rather than a change to an
        // ordinary object.
        (_, ObjectOut::AccumulatorWrite(_), _) => None,

        // Future-protocol or otherwise unrecognised combination. Skip rather
        // than crash so that newer effects formats degrade gracefully.
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::ObjectChange;
    use super::ObjectIn;
    use super::ObjectOut;
    use super::TransactionEffects;

    use base64ct::Base64;
    use base64ct::Encoding;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    const GENESIS_EFFECTS: &str = include_str!("fixtures/genesis-transaction-effects");
    const PYTH_WORMHOLE_V2: &str = include_str!("fixtures/pyth-wormhole-v2");

    fn decode(fixture: &str) -> TransactionEffects {
        let bytes = Base64::decode_vec(fixture.trim()).unwrap();
        bcs::from_bytes(&bytes).unwrap()
    }

    #[test]
    fn effects_fixtures() {
        for fixture in [GENESIS_EFFECTS, PYTH_WORMHOLE_V2] {
            let bytes = Base64::decode_vec(fixture.trim()).unwrap();
            let fx: TransactionEffects = bcs::from_bytes(&bytes).unwrap();
            assert_eq!(bcs::to_bytes(&fx).unwrap(), bytes);

            let json = serde_json::to_string_pretty(&fx).unwrap();
            println!("{json}");
            assert_eq!(fx, serde_json::from_str(&json).unwrap());
        }
    }

    /// V1 effects drain in the canonical order, the total count matches the
    /// union of the six per-kind vectors, V1's lossy fields are honestly
    /// `None`, and V1 never emits the V2-only variants.
    #[test]
    fn v1_object_changes_match_fields() {
        let fx = decode(GENESIS_EFFECTS);
        let TransactionEffects::V1(v1) = &fx else {
            panic!("expected V1 fixture");
        };

        assert_eq!(fx.transaction_digest(), &v1.transaction_digest);
        assert_eq!(fx.events_digest(), v1.events_digest.as_ref());

        let changes: Vec<ObjectChange<'_>> = fx.object_changes().collect();
        let expected_total = v1.created.len()
            + v1.mutated.len()
            + v1.unwrapped.len()
            + v1.deleted.len()
            + v1.unwrapped_then_deleted.len()
            + v1.wrapped.len();
        assert_eq!(changes.len(), expected_total);

        // First `created.len()` entries must be Created.
        for (i, change) in changes.iter().copied().take(v1.created.len()).enumerate() {
            let expected = &v1.created[i];
            match change {
                ObjectChange::Created {
                    object_id,
                    output_version,
                    output_digest,
                    output_owner,
                } => {
                    assert_eq!(object_id, expected.reference.object_id());
                    assert_eq!(output_version, expected.reference.version());
                    assert_eq!(output_digest, expected.reference.digest());
                    assert_eq!(*output_owner, expected.owner);
                }
                other => panic!("expected Created at index {i}, got {other:?}"),
            }
        }

        // V1 never carries an input digest or input owner.
        for change in changes.iter().copied() {
            match change {
                ObjectChange::Mutated {
                    input_digest,
                    input_owner,
                    ..
                }
                | ObjectChange::Deleted {
                    input_digest,
                    input_owner,
                    ..
                }
                | ObjectChange::Wrapped {
                    input_digest,
                    input_owner,
                    ..
                } => {
                    assert!(input_digest.is_none(), "V1 has no input digest");
                    assert!(input_owner.is_none(), "V1 has no input owner");
                }
                _ => {}
            }
        }
    }

    /// V2 effects carry full input state. Every ObjectWrite is paired with
    /// the effects' `lamport_version`, and (when present) Mutated entries
    /// round-trip the input version, digest, and owner as `Some`.
    #[test]
    fn v2_object_changes_match_fields() {
        let fx = decode(PYTH_WORMHOLE_V2);
        let TransactionEffects::V2(v2) = &fx else {
            panic!("expected V2 fixture");
        };

        assert_eq!(fx.transaction_digest(), &v2.transaction_digest);
        assert_eq!(fx.events_digest(), v2.events_digest.as_ref());

        let changes: Vec<ObjectChange<'_>> = fx.object_changes().collect();
        // Every non-accumulator entry in `changed_objects` should yield a change.
        let expected_count = v2
            .changed_objects
            .iter()
            .filter(|c| !matches!(c.output_state, ObjectOut::AccumulatorWrite(_)))
            .count();
        assert_eq!(changes.len(), expected_count);

        let mut saw_object_write = false;
        let raws = v2
            .changed_objects
            .iter()
            .filter(|c| !matches!(c.output_state, ObjectOut::AccumulatorWrite(_)));
        for (raw, change) in raws.zip(changes.iter().copied()) {
            match (&raw.input_state, &raw.output_state) {
                (
                    ObjectIn::Exist {
                        version,
                        digest,
                        owner,
                    },
                    ObjectOut::ObjectWrite { .. },
                ) => {
                    saw_object_write = true;
                    let ObjectChange::Mutated {
                        input_version,
                        input_digest,
                        input_owner,
                        output_version,
                        ..
                    } = change
                    else {
                        panic!("expected Mutated, got {change:?}");
                    };
                    assert_eq!(input_version, Some(*version));
                    assert_eq!(input_digest, Some(digest));
                    assert_eq!(input_owner, Some(owner));
                    assert_eq!(output_version, v2.lamport_version);
                }
                (ObjectIn::NotExist, ObjectOut::ObjectWrite { .. }) => {
                    saw_object_write = true;
                    match change {
                        ObjectChange::Created { output_version, .. }
                        | ObjectChange::Unwrapped { output_version, .. } => {
                            assert_eq!(output_version, v2.lamport_version);
                        }
                        other => panic!("expected Created or Unwrapped, got {other:?}"),
                    }
                }
                _ => {}
            }
        }
        assert!(saw_object_write, "fixture should exercise ObjectWrite");
    }

    /// `object_id()` agrees with the variant's inner field on every change.
    #[test]
    fn object_change_object_id_covers_every_variant() {
        let v1 = decode(GENESIS_EFFECTS);
        let v2 = decode(PYTH_WORMHOLE_V2);

        for change in v1.object_changes().chain(v2.object_changes()) {
            let id_from_accessor = change.object_id();
            let id_from_match = match change {
                ObjectChange::Created { object_id, .. }
                | ObjectChange::Mutated { object_id, .. }
                | ObjectChange::Unwrapped { object_id, .. }
                | ObjectChange::Deleted { object_id, .. }
                | ObjectChange::Wrapped { object_id, .. }
                | ObjectChange::UnwrappedThenDeleted { object_id } => object_id,
            };
            assert_eq!(id_from_accessor, id_from_match);
        }
    }
}
