// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod object;
use anyhow::anyhow;
use object::Kind;
pub use object::Object;

use sui_types::types::Address;
use sui_types::types::Argument;
use sui_types::types::Command;
use sui_types::types::GasPayment;
use sui_types::types::Identifier;
use sui_types::types::InputArgument;
use sui_types::types::MakeMoveVector;
use sui_types::types::MergeCoins;
use sui_types::types::MoveCall;
use sui_types::types::ObjectId;
use sui_types::types::Publish;
use sui_types::types::SplitCoins;
use sui_types::types::Transaction;
use sui_types::types::TransactionExpiration;
use sui_types::types::TransferObjects;
use sui_types::types::TypeTag;

use anyhow::Error;
use serde::Serialize;
use sui_types::types::UnresolvedGasPayment;
use sui_types::types::UnresolvedInputArgument;
use sui_types::types::UnresolvedObjectReference;
use sui_types::types::UnresolvedProgrammableTransaction;
use sui_types::types::UnresolvedTransaction;
use sui_types::types::Upgrade;

#[derive(Clone, Debug)]
pub struct TransactionBuilder {
    inputs: Vec<Input>,
    commands: Vec<Command>,
    gas: Vec<Object>,
    gas_budget: u64,
    gas_price: u64,
    sender: Address,
    sponsor: Option<Address>,
    expiration: Option<TransactionExpiration>,
}

/// A transaction input that bypasses serialization -- the input contents is
/// put verbatim into the transaction.
struct RawBytes(Vec<u8>);

/// A transaction input that will be serialized from BCS.
pub struct Serialized<'a, T: Serialize>(pub &'a T);

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Gas,
    Input(u16),
    Result(u16),
    NestedResult(u16, u16),
}

/// Inputs are converted into this type when they are added to a Transaction.
/// We will offer a number of type conversion trait impls to make this seamless.
#[derive(Clone, Debug)]
pub enum Input {
    Pure(Vec<u8>),
    // Literal(Literal),
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

/// A transaction state that can be resolved or unresolved. When calling [`try_finish`]` on the
/// transaction builder, it will return either the resolved transaction if no error was
/// encountered, or the Unresolved. The unresolved transaction can be used to call the transaction
/// resolution API, that will attempt to resolve the transaction.
pub enum TransactionResolution {
    Resolved(Transaction),
    Unresolved(UnresolvedTransaction),
}

impl Value {
    /// Turn a Result into a NestedResult.
    fn nested(&self, ix: u16) -> Value {
        Value::NestedResult(
            ix,
            match self {
                Value::Result(i) => *i,
                _ => panic!("Cannot nest a non-result value"),
            },
        )
    }
}

impl TransactionBuilder {
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
        let cmd = Command::MoveCall(MoveCall {
            arguments: arguments.into_iter().map(|a| a.into()).collect(),
            package: function.package.into(),
            module: Identifier::new(function.module).unwrap(),
            type_arguments: function.type_args,
            function: Identifier::new(function.function).unwrap(),
        });
        self.commands.push(cmd);
        Value::Result(self.commands.len() as u16 - 1)
    }

    /// Transfer a list of objects to the given address.
    pub fn transfer_objects(&mut self, objects: Vec<Value>, address: Value) {
        let cmd = Command::TransferObjects(TransferObjects {
            objects: objects.into_iter().map(|o| o.into()).collect(),
            address: address.into(),
        });
        self.commands.push(cmd);
    }

    /// Split a coin by amounts.
    pub fn split_coins(&mut self, coin: Value, amounts: Vec<Value>) -> Value {
        let cmd = Command::SplitCoins(SplitCoins {
            coin: coin.into(),
            amounts: amounts.into_iter().map(|a| a.into()).collect(),
        });
        self.commands.push(cmd);
        Value::Result(self.commands.len() as u16 - 1)
    }

    /// Merge a list of coins into a single coin.
    pub fn merge_coins(&mut self, into: Value, coins: Vec<Value>) {
        let cmd = Command::MergeCoins(MergeCoins {
            coin: into.into(),
            coins_to_merge: coins.into_iter().map(|c| c.into()).collect(),
        });
        self.commands.push(cmd);
    }

    /// Make a move vector from a list of elements.
    pub fn make_move_vec(&mut self, type_: Option<TypeTag>, elements: Vec<Value>) {
        let cmd = Command::MakeMoveVector(MakeMoveVector {
            type_,
            elements: elements.into_iter().map(|v| v.into()).collect(),
        });
        self.commands.push(cmd);
    }

    /// Publish a new module.
    pub fn publish(&mut self, modules: Vec<Vec<u8>>, dependencies: Vec<ObjectId>) -> Value {
        let cmd = Command::Publish(Publish {
            modules,
            dependencies,
        });
        self.commands.push(cmd);
        Value::Result(self.commands.len() as u16 - 1)
    }

    /// Upgrade a module.
    pub fn upgrade(
        &mut self,
        modules: Vec<Vec<u8>>,
        dependencies: Vec<ObjectId>,
        prev: ObjectId,
        ticket: Value,
    ) -> Value {
        let cmd = Command::Upgrade(Upgrade {
            modules,
            dependencies,
            package: prev,
            ticket: ticket.into(),
        });
        self.commands.push(cmd);
        Value::Result(self.commands.len() as u16 - 1)
    }

    /// Assuming everything is resolved, convert this transaction into the
    /// resolved form. Fails if there are unresolved parts.
    pub fn finish(&self) -> Result<Transaction, Error> {
        if self.gas.is_empty() {
            return Err(anyhow!("No gas objects provided"));
        }
        if self.gas_budget == 0 {
            return Err(anyhow!("No gas budget provided"));
        }
        if self.gas_price == 0 {
            return Err(anyhow!("No gas price provided"));
        }
        if self.sender == Address::default() {
            return Err(anyhow!("No sender provided"));
        }
        if self.commands.is_empty() && !self.inputs.is_empty() {
            return Err(anyhow!("No commands provided, but only inputs."));
        }
        if !self.commands.is_empty() && self.inputs.is_empty() {
            return Err(anyhow!("Commands provided, but no inputs."));
        }
        if self.gas.is_empty() {
            return Err(anyhow!("No gas objects provided"));
        }

        Ok(Transaction {
            kind: sui_types::types::TransactionKind::ProgrammableTransaction(
                sui_types::types::ProgrammableTransaction {
                    inputs: self
                        .inputs
                        .iter()
                        .map(|i| i.try_into())
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|e| anyhow!("Failed to convert inputs into InputArgument: {e}"))?,
                    commands: self.commands.clone(),
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
                    owner: self.sponsor.unwrap_or(self.sender),
                    price: self.gas_price,
                    budget: self.gas_budget,
                }
            },
            expiration: self.expiration.unwrap_or(TransactionExpiration::None),
        })
    }

    /// Try to finish the transaction, but if not all parts are resolved, it will return an
    /// [`UnresolvedTransaction`]. This can be used to resolve the transaction later by calling the
    /// [`resolve_transaction`] method.
    pub fn try_finish(&self) -> Result<TransactionResolution, Error> {
        match self.finish() {
            Ok(tx) => Ok(TransactionResolution::Resolved(tx)),
            Err(_) => Ok(TransactionResolution::Unresolved(UnresolvedTransaction {
                ptb: UnresolvedProgrammableTransaction {
                    inputs: self.inputs.clone().into_iter().map(|x| x.into()).collect(),
                    commands: self.commands.clone(),
                },
                sender: self.sender,
                gas_payment: Some(UnresolvedGasPayment {
                    objects: self
                        .gas
                        .clone()
                        .into_iter()
                        .map(to_unresolved_obj_ref)
                        .collect(),
                    owner: self.sponsor.unwrap_or(self.sender),
                    price: Some(self.gas_price),
                    budget: Some(self.gas_budget),
                }),
                expiration: self.expiration.unwrap_or(TransactionExpiration::None),
            })),
        }
    }

    /// Take an unresolved transaction and attempt to resolve it. It requires a fullnode.
    pub fn resolve_transaction(
        &self,
        unresolved_tx: UnresolvedTransaction,
    ) -> Result<Transaction, Error> {
        todo!()
    }

    /// Attempt to finish the transaction, but if it fails, resolve it instead.
    pub fn finish_or_resolve_tx(&self) -> Result<Transaction, Error> {
        match self.try_finish()? {
            TransactionResolution::Resolved(tx) => Ok(tx),
            TransactionResolution::Unresolved(tx) => self.resolve_transaction(tx),
        }
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self::new()
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

impl From<Value> for Argument {
    fn from(value: Value) -> Self {
        match value {
            Value::Gas => Argument::Gas,
            Value::Input(i) => Argument::Input(i),
            Value::Result(i) => Argument::Result(i),
            Value::NestedResult(i, j) => Argument::NestedResult(i, j),
        }
    }
}

impl TryFrom<&Input> for InputArgument {
    type Error = Error;
    fn try_from(value: &Input) -> Result<Self, Error> {
        match value {
            Input::Object(ref object) => {
                if let Some(obj) = &object.kind {
                    match obj {
                        Kind::ImmOrOwned => Ok(InputArgument::ImmutableOrOwned(object.try_into()?)),
                        Kind::Receiving => Ok(InputArgument::Receiving(object.try_into()?)),
                        Kind::Shared => Ok(InputArgument::Shared {
                            object_id: object.id,
                            initial_shared_version: object
                                .initial_shared_version
                                .ok_or_else(|| Error::msg("Initial shared version not found"))?,
                            mutable: object.mutable.ok_or_else(|| {
                                Error::msg("Expected mutable object, but mutable field is None")
                            })?,
                        }),
                    }
                } else {
                    Err(anyhow!("Object kind not found"))
                }
            }

            Input::Pure(v) => Ok(InputArgument::Pure { value: v.clone() }),
        }
    }
}

/// Convert an object into an [`UnresolvedObjectReference`].
fn to_unresolved_obj_ref(obj: Object) -> UnresolvedObjectReference {
    UnresolvedObjectReference {
        object_id: obj.id,
        version: obj.version,
        digest: obj.digest,
    }
}

impl From<Input> for UnresolvedInputArgument {
    fn from(arg: Input) -> Self {
        match arg {
            Input::Object(obj) => match obj.kind.as_ref() {
                Some(Kind::ImmOrOwned) => {
                    UnresolvedInputArgument::ImmutableOrOwned(to_unresolved_obj_ref(obj))
                }
                Some(Kind::Receiving) => {
                    UnresolvedInputArgument::Receiving(to_unresolved_obj_ref(obj))
                }
                Some(Kind::Shared) => UnresolvedInputArgument::Shared {
                    object_id: obj.id,
                    initial_shared_version: obj.initial_shared_version,
                    mutable: obj.mutable,
                },
                None => UnresolvedInputArgument::ImmutableOrOwned(UnresolvedObjectReference {
                    object_id: obj.id,
                    version: obj.version,
                    digest: obj.digest,
                }),
            },
            Input::Pure(v) => UnresolvedInputArgument::Pure { value: v },
        }
    }
}
