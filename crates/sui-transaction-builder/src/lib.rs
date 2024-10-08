// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod object;
pub use object::Object;

use sui_types::types::Address;
use sui_types::types::Command;
use sui_types::types::ObjectId;
use sui_types::types::Transaction as SuiTransaction;
use sui_types::types::TypeTag;

use anyhow::Error;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<Input>,
    pub commands: Vec<Command>,
}

#[derive(Clone, Copy)]
pub enum Value {
    Input(u16),
    Result(u16),
    NestedResult(u16, u16),
}

/// A special kind of input for resolution that supports a basic form of type
/// inference and BCS serialization. Developers will not typically interact with
/// this type, but it is used to convert literals and primitive types in Rust into
/// values that can be used in a PTB.
#[derive(Clone, Debug)]
pub enum Literal {
    Bool(bool),
    Number(i32),
    String(String),
    Vec(Vec<Literal>),
    None,
    Some(Box<Literal>),
}

/// A transaction input that bypasses serialization -- the input contents is
/// put verbatim into the transaction.
struct RawBytes(Vec<u8>);

/// A transaction input that will be serialized from BCS.
pub struct Serialized<'a, T: Serialize>(pub &'a T);

/// Inputs are converted into this type when they are added to a Transaction.
/// We will offer a number of type conversion trait impls to make this seamless.
#[derive(Clone, Debug)]
pub enum Input {
    Pure(Vec<u8>),
    Literal(Literal),
    Object(Object),
}

/// A separate type to support denoting a function by a string, or by a more
/// structured representation.
struct Function {
    package: Address,
    module: String,
    function: String,
    type_args: Vec<TypeTag>,
}

impl Value {
    /// Turn a Result into a NestedResult.
    fn nested(&self, ix: u16) -> Value {
        todo!()
    }
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            commands: Vec::new(),
        }
    }

    /// Assuming everything is resolved, convert this transaction into the
    /// resolved form. Fails if there are unresolved parts.
    fn finish(self) -> Result<SuiTransaction, Error> {
        OK(SuiTransaction {
            kind: sui_types::types::TransactionKind::ProgrammableTransaction(
                sui_types::types::ProgrammableTransaction {
                    inputs: self.inputs,
                    commands: self.commands,
                },
            ),
            sender: todo!(),
            gas_payment: todo!(),
            expiration: todo!(),
        })
    }

    // Transaction Inputs

    /// The gas coin -- it doesn't actually need to be called on Transaction
    /// but including it here for uniformity.
    fn gas(&self) -> Value {
        todo!()
    }

    /// Make a value available to the transaction as an input.
    pub fn input(&mut self, i: impl Into<Input>) -> Value {
        let input = i.into();
        self.inputs.push(input);
        Value::Input((self.inputs.len() + 1) as u16)
    }

    // Metadata
    fn set_gas(&mut self, gas: Vec<Object>) {
        todo!()
    }
    fn set_sender(&mut self, sender: Address) {
        todo!()
    }
    fn set_sponsor(&mut self, sponsor: Address) {
        todo!()
    }

    // Commands
    fn move_call(&mut self, function: Function, arguments: Vec<Value>) -> Value {
        todo!()
    }
    fn transfer_objects(&mut self, objects: Vec<Value>, address: Value) {
        todo!()
    }
    fn split_coins(&mut self, coin: Value, amounts: Vec<Value>) -> Value {
        todo!()
    }
    fn merge_coins(&mut self, first: Value, rest: Vec<Value>) {
        todo!()
    }
    fn make_move_vec(&mut self, type_: Option<TypeTag>, vec: Vec<Value>) {
        todo!()
    }

    fn publish(&mut self, modules: Vec<Vec<u8>>, deps: Vec<ObjectId>) -> Value {
        todo!()
    }

    fn upgrade(
        &mut self,
        modules: Vec<Vec<u8>>,
        deps: Vec<ObjectId>,
        prev: ObjectId,
        ticket: Value,
    ) -> Value {
        todo!()
    }
}

impl From<Literal> for Input {
    fn from(literal: Literal) -> Input {
        Input::Literal(literal)
    }
}

impl From<Object> for Input {
    fn from(object: Object) -> Input {
        Input::Object(object)
    }
}

impl<'a, T: Serialize> Serialized<'a, T> {
    pub fn serialize(&self) -> Vec<u8> {
        bcs::to_bytes(self.0).unwrap()
    }
}

// Allow converting Serialized<Address> into Input
impl<'a> From<Serialized<'a, Address>> for Input {
    fn from(serialized: Serialized<'a, Address>) -> Input {
        // Convert Serialized<Address> into Input::Pure variant
        Input::Pure(serialized.serialize()) // Here we use `Pure(Vec<u8>)`
    }
}
