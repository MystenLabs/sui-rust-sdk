// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Ergonomic constructors for the transaction and event filter DNF types.
//!
//! The generated [`TransactionFilter`] and [`EventFilter`] messages model a
//! filter in disjunctive normal form: a filter is an OR of terms, each term is
//! an AND of literals, and each literal is a predicate that may be negated.
//! Building even the simplest filter by hand therefore means nesting three
//! levels of message plus a single-field predicate wrapper. The helpers in this
//! module collapse that nesting so the common cases read as one expression.
//!
//! The predicate constructors live in the [`transaction`] and [`event`]
//! submodules (one function per predicate kind) because the generated messages
//! already expose the natural names -- `sender`, `affected_object`, and so on
//! -- as field accessors. Each constructor returns a plain, un-negated literal,
//! so a term whose literals are all positive can be written from bare
//! constructor calls; call [`TransactionLiteral::negate`] (or the event
//! equivalent) to require instead that a predicate not match.
//!
//! # Examples
//!
//! A transaction filter, from the simplest predicate up to a full DNF
//! expression:
//!
//! ```
//! use sui_rpc::proto::sui::rpc::v2alpha::TransactionFilter;
//! use sui_rpc::proto::sui::rpc::v2alpha::TransactionTerm;
//! use sui_rpc::proto::sui::rpc::v2alpha::filter::transaction as tx;
//! use sui_sdk_types::Address;
//!
//! let a = Address::ZERO;
//! let o = Address::TWO;
//!
//! // Match transactions sent by `a`.
//! let _ = TransactionFilter::matching(tx::sender(a));
//!
//! // Match transactions that were sent by `a` and also touched object `o`.
//! let _ = TransactionFilter::all([tx::sender(a), tx::affected_object(o)]);
//!
//! // Match `(sender = a AND NOT package_write) OR (affected_object = o)`.
//! let _ = TransactionFilter::any([
//!     TransactionTerm::all([tx::sender(a), tx::package_write().negate()]),
//!     TransactionTerm::all([tx::affected_object(o)]),
//! ]);
//! ```
//!
//! Event filters use the [`event`] submodule and the same combinators:
//!
//! ```
//! use sui_rpc::proto::sui::rpc::v2alpha::EventFilter;
//! use sui_rpc::proto::sui::rpc::v2alpha::filter::event as ev;
//! use sui_sdk_types::Address;
//! use sui_sdk_types::Identifier;
//!
//! let a = Address::ZERO;
//! let coin_module: Identifier = "coin".parse().unwrap();
//!
//! // Match events emitted by transactions sent by `a` from the `0x2::coin`
//! // module.
//! let _ = EventFilter::all([ev::sender(a), ev::emit_module((Address::TWO, coin_module))]);
//! ```
//!
//! Move-path and type predicates take typed components as readily as strings,
//! so callers never have to hand-format a `::`-delimited path:
//!
//! ```
//! use sui_rpc::proto::sui::rpc::v2alpha::filter::transaction as tx;
//! use sui_sdk_types::Address;
//! use sui_sdk_types::Identifier;
//! use sui_sdk_types::StructTag;
//!
//! let package = Address::TWO;
//! let module: Identifier = "coin".parse().unwrap();
//! let function: Identifier = "mint".parse().unwrap();
//!
//! // Each of these builds a Move-call predicate at a different specificity.
//! let _ = tx::move_call(package); // whole package
//! let _ = tx::move_call((package, module.clone())); // package::module
//! let _ = tx::move_call((package, module, function)); // exact function
//! let _ = tx::move_call("0x2::coin::mint"); // or a raw path string
//!
//! // Event-type predicates accept a `StructTag`, a `TypeTag`, or a string.
//! let coin = StructTag::new(
//!     package,
//!     "coin".parse().unwrap(),
//!     "Coin".parse().unwrap(),
//!     Vec::new(),
//! );
//! let _ = tx::event_type(coin);
//! let _ = tx::event_type("0x2::coin::Coin");
//! ```

use super::*;
use sui_sdk_types::Address;
use sui_sdk_types::Identifier;
use sui_sdk_types::StructTag;
use sui_sdk_types::TypeTag;

// Layer 1: let the generated `with_*`/`set_*` accessors accept native types by
// converting them into the single-field predicate wrappers.

impl From<Address> for SenderFilter {
    fn from(address: Address) -> Self {
        Self::default().with_address(address.to_string())
    }
}

impl From<Address> for AffectedAddressFilter {
    fn from(address: Address) -> Self {
        Self::default().with_address(address.to_string())
    }
}

impl From<Address> for AffectedObjectFilter {
    fn from(object_id: Address) -> Self {
        Self::default().with_object_id(object_id.to_string())
    }
}

impl From<Address> for EventStreamHeadFilter {
    fn from(stream_id: Address) -> Self {
        Self::default().with_stream_id(stream_id.to_string())
    }
}

impl From<String> for MoveCallFilter {
    fn from(function: String) -> Self {
        Self::default().with_function(function)
    }
}

impl From<&str> for MoveCallFilter {
    fn from(function: &str) -> Self {
        Self::from(function.to_owned())
    }
}

impl From<Address> for MoveCallFilter {
    fn from(package: Address) -> Self {
        Self::from(package.to_string())
    }
}

impl From<(Address, Identifier)> for MoveCallFilter {
    fn from((package, module): (Address, Identifier)) -> Self {
        Self::from(format!("{package}::{module}"))
    }
}

impl From<(Address, Identifier, Identifier)> for MoveCallFilter {
    fn from((package, module, function): (Address, Identifier, Identifier)) -> Self {
        Self::from(format!("{package}::{module}::{function}"))
    }
}

impl From<String> for EmitModuleFilter {
    fn from(module: String) -> Self {
        Self::default().with_module(module)
    }
}

impl From<&str> for EmitModuleFilter {
    fn from(module: &str) -> Self {
        Self::from(module.to_owned())
    }
}

impl From<Address> for EmitModuleFilter {
    fn from(package: Address) -> Self {
        Self::from(package.to_string())
    }
}

impl From<(Address, Identifier)> for EmitModuleFilter {
    fn from((package, module): (Address, Identifier)) -> Self {
        Self::from(format!("{package}::{module}"))
    }
}

impl From<String> for EventTypeFilter {
    fn from(event_type: String) -> Self {
        Self::default().with_event_type(event_type)
    }
}

impl From<&str> for EventTypeFilter {
    fn from(event_type: &str) -> Self {
        Self::from(event_type.to_owned())
    }
}

impl From<StructTag> for EventTypeFilter {
    fn from(event_type: StructTag) -> Self {
        Self::from(event_type.to_string())
    }
}

impl From<TypeTag> for EventTypeFilter {
    fn from(event_type: TypeTag) -> Self {
        Self::from(event_type.to_string())
    }
}

// Layer 3 (transaction): negation on literals and the term/filter combinators.
// A predicate constructor already yields an un-negated literal, and a literal
// converts into a single-literal term, so the all-positive cases need no
// explicit wrapping.

impl TransactionLiteral {
    /// Negate this literal: require that its predicate not match.
    pub fn negate(self) -> Self {
        self.with_negated(true)
    }
}

impl From<TransactionLiteral> for TransactionTerm {
    fn from(literal: TransactionLiteral) -> Self {
        Self::all([literal])
    }
}

impl TransactionTerm {
    /// A term matching transactions that satisfy every one of `literals`
    /// (the literals are ANDed together).
    pub fn all(literals: impl IntoIterator<Item = impl Into<TransactionLiteral>>) -> Self {
        Self::default().with_literals(literals.into_iter().map(Into::into).collect())
    }
}

impl TransactionFilter {
    /// A filter matching transactions that satisfy any one of `terms`
    /// (the terms are ORed together).
    pub fn any(terms: impl IntoIterator<Item = impl Into<TransactionTerm>>) -> Self {
        Self::default().with_terms(terms.into_iter().map(Into::into).collect())
    }

    /// A filter with a single term matching transactions that satisfy every
    /// one of `literals`.
    pub fn all(literals: impl IntoIterator<Item = impl Into<TransactionLiteral>>) -> Self {
        Self::any([TransactionTerm::all(literals)])
    }

    /// A filter matching transactions that satisfy a single literal.
    pub fn matching(literal: impl Into<TransactionLiteral>) -> Self {
        Self::all([literal])
    }
}

// Layer 3 (event): the event-side mirror of the transaction combinators.

impl EventLiteral {
    /// Negate this literal: require that its predicate not match.
    pub fn negate(self) -> Self {
        self.with_negated(true)
    }
}

impl From<EventLiteral> for EventTerm {
    fn from(literal: EventLiteral) -> Self {
        Self::all([literal])
    }
}

impl EventTerm {
    /// A term matching events that satisfy every one of `literals`
    /// (the literals are ANDed together).
    pub fn all(literals: impl IntoIterator<Item = impl Into<EventLiteral>>) -> Self {
        Self::default().with_literals(literals.into_iter().map(Into::into).collect())
    }
}

impl EventFilter {
    /// A filter matching events that satisfy any one of `terms`
    /// (the terms are ORed together).
    pub fn any(terms: impl IntoIterator<Item = impl Into<EventTerm>>) -> Self {
        Self::default().with_terms(terms.into_iter().map(Into::into).collect())
    }

    /// A filter with a single term matching events that satisfy every one of
    /// `literals`.
    pub fn all(literals: impl IntoIterator<Item = impl Into<EventLiteral>>) -> Self {
        Self::any([EventTerm::all(literals)])
    }

    /// A filter matching events that satisfy a single literal.
    pub fn matching(literal: impl Into<EventLiteral>) -> Self {
        Self::all([literal])
    }
}

// Layer 2: predicate constructors. These are free functions rather than
// associated functions because the generated messages already use the same
// names for their field accessors.

/// Predicate constructors for [`TransactionFilter`].
pub mod transaction {
    use super::super::*;
    use sui_sdk_types::Address;

    /// Match transactions sent by the specified address.
    pub fn sender(address: impl Into<Address>) -> TransactionLiteral {
        TransactionLiteral::default().with_sender(address.into())
    }

    /// Match transactions where the specified address's state moved as a side
    /// effect.
    pub fn affected_address(address: impl Into<Address>) -> TransactionLiteral {
        TransactionLiteral::default().with_affected_address(address.into())
    }

    /// Match transactions whose effects include a change for the specified
    /// object.
    pub fn affected_object(object_id: impl Into<Address>) -> TransactionLiteral {
        TransactionLiteral::default().with_affected_object(object_id.into())
    }

    /// Match transactions that made a Move call matching the specified
    /// `package[::module[::function]]` path. Accepts a `::`-delimited path
    /// string, an `Address` (package), an `(Address, Identifier)` package and
    /// module pair, or an `(Address, Identifier, Identifier)` package, module,
    /// and function triple.
    pub fn move_call(function: impl Into<MoveCallFilter>) -> TransactionLiteral {
        TransactionLiteral::default().with_move_call(function)
    }

    /// Match transactions that emitted an event whose package/module fields
    /// match the specified `package[::module]` path. Accepts a `::`-delimited
    /// path string, an `Address` (package), or an `(Address, Identifier)`
    /// package and module pair.
    pub fn emit_module(module: impl Into<EmitModuleFilter>) -> TransactionLiteral {
        TransactionLiteral::default().with_emit_module(module)
    }

    /// Match transactions that emitted an event whose type matches the
    /// specified Move type. Accepts a canonical type string, a `StructTag`, or
    /// a `TypeTag`.
    pub fn event_type(event_type: impl Into<EventTypeFilter>) -> TransactionLiteral {
        TransactionLiteral::default().with_event_type(event_type)
    }

    /// Match transactions that wrote to the specified authenticated event
    /// stream head.
    pub fn event_stream_head(stream_id: impl Into<Address>) -> TransactionLiteral {
        TransactionLiteral::default().with_event_stream_head(stream_id.into())
    }

    /// Match transactions that wrote a Move package -- a first publish or an
    /// upgrade of any package.
    pub fn package_write() -> TransactionLiteral {
        TransactionLiteral::default().with_package_write(PackageWriteFilter::default())
    }
}

/// Predicate constructors for [`EventFilter`].
pub mod event {
    use super::super::*;
    use sui_sdk_types::Address;

    /// Match events from transactions sent by the specified address.
    pub fn sender(address: impl Into<Address>) -> EventLiteral {
        EventLiteral::default().with_sender(address.into())
    }

    /// Match events whose package/module fields match the specified
    /// `package[::module]` path. Accepts a `::`-delimited path string, an
    /// `Address` (package), or an `(Address, Identifier)` package and module
    /// pair.
    pub fn emit_module(module: impl Into<EmitModuleFilter>) -> EventLiteral {
        EventLiteral::default().with_emit_module(module)
    }

    /// Match events whose type matches the specified Move type. Accepts a
    /// canonical type string, a `StructTag`, or a `TypeTag`.
    pub fn event_type(event_type: impl Into<EventTypeFilter>) -> EventLiteral {
        EventLiteral::default().with_event_type(event_type)
    }

    /// Match events committed to the specified authenticated event stream head.
    pub fn event_stream_head(stream_id: impl Into<Address>) -> EventLiteral {
        EventLiteral::default().with_event_stream_head(stream_id.into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::event_literal;
    use super::super::transaction_literal;
    use super::*;

    #[test]
    fn matching_builds_single_predicate_transaction_filter() {
        let address = Address::ZERO;
        let built = TransactionFilter::matching(transaction::sender(address));

        let expected = TransactionFilter {
            terms: vec![TransactionTerm {
                literals: vec![TransactionLiteral {
                    negated: false,
                    predicate: Some(transaction_literal::Predicate::Sender(SenderFilter {
                        address: Some(address.to_string()),
                    })),
                }],
            }],
        };

        assert_eq!(built, expected);
    }

    #[test]
    fn all_ands_included_predicates_into_one_term() {
        let sender = Address::ZERO;
        let object = Address::TWO;
        let built = TransactionFilter::all([
            transaction::sender(sender),
            transaction::affected_object(object),
        ]);

        assert_eq!(built.terms.len(), 1);
        let literals = &built.terms[0].literals;
        assert_eq!(literals.len(), 2);
        assert!(
            literals
                .iter()
                .all(|literal| !literal.negated && literal.predicate.is_some())
        );
    }

    #[test]
    fn any_ors_terms_and_negate_sets_negated() {
        let sender = Address::ZERO;
        let object = Address::TWO;
        let built = TransactionFilter::any([
            TransactionTerm::all([
                transaction::sender(sender),
                transaction::package_write().negate(),
            ]),
            TransactionTerm::all([transaction::affected_object(object)]),
        ]);

        assert_eq!(built.terms.len(), 2);
        assert!(!built.terms[0].literals[0].negated);
        assert!(built.terms[0].literals[1].negated);
    }

    #[test]
    fn event_matching_builds_single_predicate_event_filter() {
        let stream_id = Address::TWO;
        let built = EventFilter::matching(event::event_stream_head(stream_id));

        let expected = EventFilter {
            terms: vec![EventTerm {
                literals: vec![EventLiteral {
                    negated: false,
                    predicate: Some(event_literal::Predicate::EventStreamHead(
                        EventStreamHeadFilter {
                            stream_id: Some(stream_id.to_string()),
                        },
                    )),
                }],
            }],
        };

        assert_eq!(built, expected);
    }

    #[test]
    fn event_type_accepts_struct_tag() {
        let tag = StructTag::new(
            Address::TWO,
            "coin".parse().unwrap(),
            "Coin".parse().unwrap(),
            Vec::new(),
        );
        let expected = tag.to_string();

        let literal = event::event_type(tag);
        let Some(event_literal::Predicate::EventType(filter)) = literal.predicate else {
            panic!("expected an event_type predicate");
        };

        assert_eq!(filter.event_type.as_deref(), Some(expected.as_str()));
    }

    #[test]
    fn move_call_accepts_typed_path_components() {
        let package = Address::TWO;
        let module: Identifier = "coin".parse().unwrap();
        let function: Identifier = "mint".parse().unwrap();
        let expected = format!("{package}::{module}::{function}");

        let literal = transaction::move_call((package, module, function));
        let Some(transaction_literal::Predicate::MoveCall(filter)) = literal.predicate else {
            panic!("expected a move_call predicate");
        };

        assert_eq!(filter.function.as_deref(), Some(expected.as_str()));
    }

    #[test]
    fn emit_module_accepts_package_address() {
        let package = Address::TWO;
        let expected = package.to_string();

        let literal = event::emit_module(package);
        let Some(event_literal::Predicate::EmitModule(filter)) = literal.predicate else {
            panic!("expected an emit_module predicate");
        };

        assert_eq!(filter.module.as_deref(), Some(expected.as_str()));
    }
}
