// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod object;
use anyhow::anyhow;
pub use object::Object;

use sui_types::types::Address;
use sui_types::types::GasPayment;
use sui_types::types::ObjectId;
use sui_types::types::Transaction as SuiTransaction;
use sui_types::types::TransactionExpiration;
use sui_types::types::TypeTag;

use anyhow::Error;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct Transaction {
    inputs: Vec<Input>,
    commands: Vec<Value>,
    gas: Vec<Object>,
    gas_budget: u64,
    gas_price: u64,
    sender: Address,
    sponsor: Option<Address>,
    expiration: Option<TransactionExpiration>,
}

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Gas,
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
pub struct Function {
    package: Address,
    module: String,
    function: String,
    type_args: Vec<TypeTag>,
}

impl Function {
    pub fn new(
        package: Address,
        module: String,
        function: String,
        type_args: Vec<TypeTag>,
    ) -> Self {
        Self {
            package,
            module,
            function,
            type_args,
        }
    }
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
            gas: Vec::new(),
            gas_budget: 0,
            gas_price: 0,
            sender: Address::default(),
            sponsor: None,
            expiration: None,
        }
    }

    // Transaction Inputs

    /// The gas coin -- it doesn't actually need to be called on Transaction
    /// but including it here for uniformity.
    fn gas(&self) -> Vec<Object> {
        vec![]
    }

    /// Make a value available to the transaction as an input.
    pub fn input(&mut self, i: impl Into<Input>) -> Value {
        let input = i.into();
        self.inputs.push(input);
        Value::Input((self.inputs.len() - 1) as u16)
    }

    // Metadata
    pub fn set_gas(&mut self, gas: Vec<Object>) {
        self.gas = gas;
    }

    pub fn set_gas_budget(&mut self, budget: u64) {
        self.gas_budget = budget;
    }

    pub fn set_gas_price(&mut self, price: u64) {
        self.gas_price = price;
    }

    pub fn set_sender(&mut self, sender: Address) {
        self.sender = sender;
    }

    pub fn set_sponsor(&mut self, sponsor: Address) {
        self.sponsor = Some(sponsor);
    }

    pub fn set_expiration(&mut self, epoch: u64) {
        self.expiration = Some(TransactionExpiration::Epoch(epoch));
    }

    // Commands
    pub fn move_call(&mut self, function: Function, arguments: Vec<Value>) -> Value {
        for arg in arguments {
            self.input(arg);
        }
        // let cmd =
        todo!()
    }
    pub fn transfer_objects(&mut self, objects: Vec<Value>, address: Value) {
        todo!()
    }
    pub fn split_coins(&mut self, coin: Value, amounts: Vec<Value>) -> Value {
        todo!()
    }
    pub fn merge_coins(&mut self, first: Value, rest: Vec<Value>) {
        todo!()
    }
    pub fn make_move_vec(&mut self, type_: Option<TypeTag>, vec: Vec<Value>) {
        todo!()
    }

    pub fn publish(&mut self, modules: Vec<Vec<u8>>, deps: Vec<ObjectId>) -> Value {
        todo!()
    }

    pub fn upgrade(
        &mut self,
        modules: Vec<Vec<u8>>,
        deps: Vec<ObjectId>,
        prev: ObjectId,
        ticket: Value,
    ) -> Value {
        todo!()
    }

    /// Assuming everything is resolved, convert this transaction into the
    /// resolved form. Fails if there are unresolved parts.
    pub fn finish(&self) -> Result<SuiTransaction, Error> {
        Ok(SuiTransaction {
            kind: sui_types::types::TransactionKind::ProgrammableTransaction(
                sui_types::types::ProgrammableTransaction {
                    inputs: vec![],
                    commands: vec![],
                },
            ),
            sender: self.sender,
            gas_payment: {
                GasPayment {
                    objects: self
                        .gas
                        .clone()
                        .into_iter()
                        .map(|o| o.try_into())
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|_| anyhow!("Failed to convert gas objects into GasPayment"))?,
                    owner: {
                        if let Some(sponsor) = self.sponsor {
                            sponsor
                        } else {
                            self.sender
                        }
                    },
                    price: self.gas_price,
                    budget: self.gas_budget,
                }
            },
            expiration: sui_types::types::TransactionExpiration::None,
        })
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
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

impl From<RawBytes> for Input {
    fn from(raw: RawBytes) -> Input {
        Input::Pure(raw.0)
    }
}

impl From<Value> for Input {
    fn from(value: Value) -> Input {
        match value {
            Value::Input(i) => Input::Pure(i.to_be_bytes().to_vec()),
            _ => todo!(),
        }
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
