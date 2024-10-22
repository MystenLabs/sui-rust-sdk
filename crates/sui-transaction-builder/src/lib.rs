// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod object;
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
use sui_types::types::UnresolvedGasPayment;
use sui_types::types::UnresolvedInputArgument;
use sui_types::types::UnresolvedInputArgumentKind;
use sui_types::types::UnresolvedObjectReference;
use sui_types::types::UnresolvedProgrammableTransaction;
use sui_types::types::UnresolvedTransaction;
use sui_types::types::UnresolvedValue;
use sui_types::types::Upgrade;

use anyhow::anyhow;
use anyhow::Error;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct TransactionBuilder {
    /// The inputs to the transaction.
    inputs: Vec<Input>,
    /// The list of commands in the transaction. A command is a single operation in a programmable
    /// transaction.
    commands: Vec<Command>,
    /// The gas objects that will be used to pay for the transaction.
    gas: Vec<Object>,
    /// The gas budget for the transaction.
    gas_budget: u64,
    /// The gas price for the transaction.
    gas_price: u64,
    /// The sender of the transaction.
    sender: Address,
    /// The sponsor of the transaction. If None, the sender is also the sponsor.
    sponsor: Option<Address>,
    /// The expiration of the transaction.
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

/// A transaction state that can be resolved or unresolved. When calling [`try_finish`]` on the
/// transaction builder, it will return either the resolved transaction if no error was
/// encountered, or the [`UnresolvedTransaction`]. The unresolved transaction can be used to call the transaction
/// resolution API, that will attempt to resolve the transaction.
pub enum TransactionResolution {
    Resolved(Transaction),
    Unresolved(UnresolvedTransaction),
}

/// A transaction builder to build transactions.
impl TransactionBuilder {
    /// Create a new transaction builder and initialize its elements to default.
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

    /// Set the gas objects that will be used to pay for the transaction.
    pub fn set_gas(&mut self, gas: Vec<Object>) {
        self.gas = gas;
    }

    /// Set the gas budget for the transaction.
    pub fn set_gas_budget(&mut self, budget: u64) {
        self.gas_budget = budget;
    }

    /// Set the gas price for the transaction.
    pub fn set_gas_price(&mut self, price: u64) {
        self.gas_price = price;
    }

    /// Set the sender of the transaction.
    pub fn set_sender(&mut self, sender: Address) {
        self.sender = sender;
    }

    /// Set the sponsor of the transaction.
    pub fn set_sponsor(&mut self, sponsor: Address) {
        self.sponsor = Some(sponsor);
    }

    /// Set the expiration of the transaction.
    pub fn set_expiration(&mut self, epoch: u64) {
        self.expiration = Some(TransactionExpiration::Epoch(epoch));
    }

    // Commands
    /// Call a Move function with the given arguments.
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
    pub fn merge_coins(&mut self, into_coin: Value, coins: Vec<Value>) {
        let cmd = Command::MergeCoins(MergeCoins {
            coin: into_coin.into(),
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

    /// Publish a list of modules with the given dependencies.
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
            Value::Input(i) => Input::Pure(bcs::to_bytes(&i.to_be_bytes()).unwrap()),
            _ => panic!("Cannot convert Value to Input"),
        }
    }
}

impl From<Vec<u8>> for Input {
    fn from(bytes: Vec<u8>) -> Input {
        Input::Pure(bytes)
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
                Some(Kind::ImmOrOwned) => UnresolvedInputArgument {
                    kind: Some(UnresolvedInputArgumentKind::ImmutableOrOwned),
                    value: None,
                    object_id: Some(obj.id),
                    version: obj.version,
                    digest: obj.digest,
                    mutable: obj.mutable,
                },
                Some(Kind::Receiving) => UnresolvedInputArgument {
                    kind: Some(UnresolvedInputArgumentKind::Receiving),
                    value: None,
                    object_id: Some(obj.id),
                    version: obj.version,
                    digest: obj.digest,
                    mutable: obj.mutable,
                },
                Some(Kind::Shared) => UnresolvedInputArgument {
                    kind: Some(UnresolvedInputArgumentKind::Shared),
                    value: None,
                    object_id: Some(obj.id),
                    version: obj.version,
                    digest: obj.digest,
                    mutable: obj.mutable,
                },
                None => UnresolvedInputArgument {
                    kind: Some(UnresolvedInputArgumentKind::ImmutableOrOwned),
                    value: None,
                    object_id: Some(obj.id),
                    version: obj.version,
                    digest: obj.digest,
                    mutable: obj.mutable,
                },
            },
            Input::Pure(v) => UnresolvedInputArgument {
                kind: Some(UnresolvedInputArgumentKind::Pure),
                value: Some(UnresolvedValue::Array(
                    v.into_iter()
                        .map(|x| UnresolvedValue::Number(x as u64))
                        .collect(),
                )),
                object_id: None,
                version: None,
                digest: None,
                mutable: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Error};
    use base64ct::Encoding;
    use std::str::FromStr;
    use sui_crypto::ed25519::Ed25519PrivateKey;
    use sui_crypto::SuiSigner;
    use sui_graphql_client::{Client, PaginationFilter};

    use sui_graphql_client::faucet::{CoinInfo, FaucetClient};
    use sui_types::types::{
        Address, ExecutionStatus, ObjectDigest, ObjectId, Transaction, TransactionEffects, TypeTag,
    };

    use crate::object::Object;
    use crate::{Function, Serialized, TransactionBuilder};

    /// Derive an address from a known private key (was generated for testing)
    fn helper_address_pk() -> (Address, Ed25519PrivateKey) {
        let pk_bcs =
            base64ct::Base64::decode_vec("AAjgPs/zbxDsObju6Tp2/8W5lNWP/sjUDsVxR1vgdmyT").unwrap();
        let pk = Ed25519PrivateKey::new(pk_bcs[1..].try_into().expect("slice has wrong length"));
        let address = pk.public_key().to_address();
        (address, pk)
    }

    /// Generate an address from a known ED25519 private key, call faucet, set the sender,
    /// set the gas object (last coin from the returned Vec<CoinInfo>), set gas price, set gas
    /// budget, and return the address, private key, and coins.
    async fn helper_setup(
        tx: &mut TransactionBuilder,
        client: &Client,
    ) -> (Address, Ed25519PrivateKey, Vec<CoinInfo>) {
        let (address, pk) = helper_address_pk();
        let coins = FaucetClient::local()
            .request_and_wait(address)
            .await
            .unwrap()
            .unwrap()
            .sent;
        let gas = coins.last().unwrap().id;
        let gas_obj = client.object(gas.into(), None).await.unwrap().unwrap();
        tx.set_gas(vec![Object::owned(
            gas,
            gas_obj.version(),
            gas_obj.digest(),
        )]);
        tx.set_gas_budget(500000000);
        tx.set_gas_price(1000);
        tx.set_sender(address);

        (address, pk, coins)
    }

    async fn wait_for_tx_and_check_effects_status_success(
        client: &Client,
        tx: &Transaction,
        effects: Result<Option<TransactionEffects>, Error>,
    ) {
        assert!(effects.is_ok(), "Execution failed. Effects: {:?}", effects);
        // wait for the transaction to be finalized
        loop {
            let tx_digest = client.transaction(&tx.digest().to_base58()).await.unwrap();
            if tx_digest.is_some() {
                break;
            }
        }

        // check that it succeeded
        let status = match effects.unwrap().unwrap() {
            sui_types::types::TransactionEffects::V2(e) => Some(e.status),
            _ => None,
        };
        let expected_status = ExecutionStatus::Success;
        assert_eq!(expected_status, status.unwrap());
    }

    #[tokio::test]
    async fn test_tx_finish() {
        let mut tx = TransactionBuilder::new();
        let coin_obj_id = "0x19406ea4d9609cd9422b85e6bf2486908f790b778c757aff805241f3f609f9b4";
        let coin_digest = "7opR9rFUYivSTqoJHvFb9p6p54THyHTatMG6id4JKZR9";
        let coin_version = 2;
        let coin = tx.input(Object::owned(
            ObjectId::from_str(coin_obj_id).unwrap(),
            coin_version,
            ObjectDigest::from_str(coin_digest)
                .map_err(|_| anyhow!("Invalid object digest"))
                .unwrap(),
        ));

        let recipient = tx.input(Serialized(
            &Address::from_str(
                "0xeeaeb20e7b3a9cfce768a2ddf69503d8ae8d1628a26c2d74a435501c929d3f69",
            )
            .map_err(|_| anyhow!("Invalid address"))
            .unwrap(),
        ));

        assert!(tx.finish().is_err());

        tx.transfer_objects(vec![coin], recipient);
        tx.set_gas_budget(500000000);
        tx.set_gas_price(1000);
        tx.set_gas(vec![Object::owned(
            ObjectId::from_str(
                "0xd8792bce2743e002673752902c0e7348dfffd78638cb5367b0b85857bceb9821",
            )
            .unwrap(),
            2,
            ObjectDigest::from_str("2ZigdvsZn5BMeszscPQZq9z8ebnS2FpmAuRbAi9ednCk")
                .map_err(|_| anyhow!("Invalid object digest"))
                .unwrap(),
        )]);
        tx.set_sender(
            Address::from_str("0xc574ea804d9c1a27c886312e96c0e2c9cfd71923ebaeb3000d04b5e65fca2793")
                .unwrap(),
        );

        let tx = tx.finish();
        assert!(tx.is_ok());
    }

    #[tokio::test]
    async fn test_try_finish() {
        let mut tx = TransactionBuilder::new();
        let coin_obj_id = "0x19406ea4d9609cd9422b85e6bf2486908f790b778c757aff805241f3f609f9b4";
        let coin_digest = "7opR9rFUYivSTqoJHvFb9p6p54THyHTatMG6id4JKZR9";
        let coin_version = 2;
        let _coin = tx.input(Object::owned(
            ObjectId::from_str(coin_obj_id).unwrap(),
            coin_version,
            ObjectDigest::from_str(coin_digest)
                .map_err(|_| anyhow!("Invalid object digest"))
                .unwrap(),
        ));

        let tx = tx.try_finish();
        assert!(tx.is_ok());
        assert!(matches!(
            tx.unwrap(),
            crate::TransactionResolution::Unresolved(_)
        ));
    }

    /// Test TransferObj PTB.
    #[tokio::test]
    async fn test_transfer_obj_execution() {
        // a specific random private key

        let mut tx = TransactionBuilder::new();
        let (_, pk, coins) = helper_setup(&mut tx, &Client::new_localhost()).await;

        // get the object information from the client
        let client = Client::new_localhost();
        let first = coins.first().unwrap().id;
        let coin = client.object(first.into(), None).await.unwrap().unwrap();
        let coin_digest = coin.digest();
        let coin_version = coin.version();

        let coin_input = tx.input(Object::owned(coin.object_id(), coin_version, coin_digest));
        let recipient = Address::generate(rand::thread_rng());
        let recipient_input = tx.input(Serialized(&recipient));
        tx.transfer_objects(vec![coin_input], recipient_input);

        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();

        let effects = client.execute_tx(vec![sig], &tx).await;
        wait_for_tx_and_check_effects_status_success(&client, &tx, effects).await;

        // check that recipient has 1
        let recipient_coins = client
            .coins(recipient, None, Some(PaginationFilter::default()))
            .await
            .unwrap();
        assert_eq!(recipient_coins.data().len(), 1);
    }

    #[tokio::test]
    async fn test_move_call() {
        // Check that `0x1::option::is_none` move call works when passing `1`
        let client = Client::new_localhost();
        let mut tx = TransactionBuilder::new();
        // set up the sender, gas object, gas budget, and gas price and return the pk to sign
        let (_, pk, _) = helper_setup(&mut tx, &client).await;
        let function = Function::new(
            "0x1".parse().unwrap(),
            "option".to_string(),
            "is_none".to_string(),
            vec![TypeTag::U64],
        );
        let input = tx.input(Serialized(&vec![1u64]).serialize());
        tx.move_call(function, vec![input]);

        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();
        let effects = client.execute_tx(vec![sig], &tx).await;
        wait_for_tx_and_check_effects_status_success(&client, &tx, effects).await;
    }

    #[tokio::test]
    async fn test_split_transfer() {
        let client = Client::new_localhost();
        let mut tx = TransactionBuilder::new();
        let (_, pk, coins) = helper_setup(&mut tx, &client).await;

        let coin = coins.first().unwrap().id;
        let coin_obj = client.object(coin.into(), None).await.unwrap().unwrap();
        let coin_digest = coin_obj.digest();
        let coin_version = coin_obj.version();

        let coin_input = tx.input(Object::owned(
            coin_obj.object_id(),
            coin_version,
            coin_digest,
        ));
        // transfer 1 SUI
        let amount = tx.input(Serialized(&1_000_000_000u64).serialize());
        let result = tx.split_coins(coin_input, vec![amount]);
        let recipient_address = Address::generate(rand::thread_rng());
        let recipient = tx.input(Serialized(&recipient_address));
        tx.transfer_objects(vec![result], recipient);

        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();

        let effects = client.execute_tx(vec![sig], &tx).await;
        wait_for_tx_and_check_effects_status_success(&client, &tx, effects).await;

        // check that recipient has 1 coin
        let recipient_coins = client
            .coins(recipient_address, None, Some(PaginationFilter::default()))
            .await
            .unwrap();
        assert_eq!(recipient_coins.data().len(), 1);
    }

    #[tokio::test]
    async fn test_split_without_transfer_should_fail() {
        let client = Client::new_localhost();
        let mut tx = TransactionBuilder::new();
        let (_, pk, coins) = helper_setup(&mut tx, &client).await;

        let coin = coins.first().unwrap().id;
        let coin_obj = client.object(coin.into(), None).await.unwrap().unwrap();
        let coin_digest = coin_obj.digest();
        let coin_version = coin_obj.version();

        let coin_input = tx.input(Object::owned(
            coin_obj.object_id(),
            coin_version,
            coin_digest,
        ));
        // transfer 1 SUI
        let amount = tx.input(Serialized(&1_000_000_000u64).serialize());
        tx.split_coins(coin_input, vec![amount]);

        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();

        let effects = client.execute_tx(vec![sig], &tx).await;
        assert!(effects.is_ok());

        // wait for the transaction to be finalized
        loop {
            let tx_digest = client.transaction(&tx.digest().to_base58()).await.unwrap();
            if tx_digest.is_some() {
                break;
            }
        }
        assert!(effects.is_ok());
        let status = match effects.unwrap().unwrap() {
            sui_types::types::TransactionEffects::V2(e) => Some(e.status),
            _ => None,
        };
        let expected_status = ExecutionStatus::Success;
        assert_ne!(expected_status, status.unwrap());
    }

    #[tokio::test]
    async fn test_merge_coins() {
        let client = Client::new_localhost();
        let mut tx = TransactionBuilder::new();
        let (_, pk, coins) = helper_setup(&mut tx, &client).await;

        let coin1 = coins.first().unwrap().id;
        let coin1_obj = client.object(coin1.into(), None).await.unwrap().unwrap();
        let coin1_digest = coin1_obj.digest();
        let coin1_version = coin1_obj.version();

        let coin_to_merge = tx.input(Object::owned(
            coin1_obj.object_id(),
            coin1_version,
            coin1_digest,
        ));

        let mut coins_to_merge = vec![];
        // last coin is used for gas, first coin is the one we merge into
        for c in coins[1..&coins.len() - 1].iter() {
            let coin = client.object(c.id.into(), None).await.unwrap().unwrap();
            let coin_digest = coin.digest();
            let coin_version = coin.version();
            coins_to_merge.push(tx.input(Object::owned(
                coin.object_id(),
                coin_version,
                coin_digest,
            )));
        }

        tx.merge_coins(coin_to_merge, coins_to_merge);
        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();

        let effects = client.execute_tx(vec![sig], &tx).await;
        wait_for_tx_and_check_effects_status_success(&client, &tx, effects).await;
    }

    #[tokio::test]
    async fn test_make_move_vec() {
        let client = Client::new_localhost();
        let mut tx = TransactionBuilder::new();
        let (_, pk, _) = helper_setup(&mut tx, &client).await;

        let input = tx.input(Serialized(&1u64).serialize());
        tx.make_move_vec(Some(TypeTag::U64), vec![input]);

        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();

        let effects = client.execute_tx(vec![sig], &tx).await;
        assert!(effects.is_ok(), "Execution failed. Effects: {:?}", effects);

        // wait for the transaction to be finalized
        loop {
            let tx_digest = client.transaction(&tx.digest().to_base58()).await.unwrap();
            if tx_digest.is_some() {
                break;
            }
        }

        // check that it succeeded
        let status = match effects.unwrap().unwrap() {
            sui_types::types::TransactionEffects::V2(e) => Some(e.status),
            _ => None,
        };
        let expected_status = ExecutionStatus::Success;
        assert_eq!(expected_status, status.unwrap());
    }

    #[tokio::test]
    async fn test_publish() {
        let client = Client::new_localhost();
        let mut tx = TransactionBuilder::new();
        let (address, pk, _) = helper_setup(&mut tx, &client).await;

        // publish very dummy package (first_package from sui/examples/move/first_package)
        let modules: [u8; 506] = [
            161, 28, 235, 11, 6, 0, 0, 0, 10, 1, 0, 8, 2, 8, 16, 3, 24, 46, 4, 70, 2, 5, 72, 49, 7,
            121, 133, 1, 8, 254, 1, 64, 10, 190, 2, 18, 12, 208, 2, 119, 13, 199, 3, 6, 0, 4, 1,
            10, 1, 15, 1, 16, 0, 1, 12, 0, 0, 0, 8, 0, 1, 3, 4, 0, 3, 2, 2, 0, 0, 6, 0, 1, 0, 0, 7,
            2, 3, 0, 0, 12, 2, 3, 0, 0, 14, 4, 3, 0, 0, 13, 5, 6, 0, 0, 9, 7, 6, 0, 1, 8, 0, 8, 0,
            2, 15, 12, 1, 1, 8, 3, 11, 9, 10, 0, 7, 11, 1, 7, 8, 3, 0, 1, 6, 8, 0, 1, 3, 1, 6, 8,
            1, 3, 3, 3, 7, 8, 3, 1, 8, 0, 4, 7, 8, 1, 3, 3, 7, 8, 3, 1, 8, 2, 1, 6, 8, 3, 1, 5, 1,
            8, 1, 2, 9, 0, 5, 5, 70, 111, 114, 103, 101, 5, 83, 119, 111, 114, 100, 9, 84, 120, 67,
            111, 110, 116, 101, 120, 116, 3, 85, 73, 68, 7, 101, 120, 97, 109, 112, 108, 101, 2,
            105, 100, 4, 105, 110, 105, 116, 5, 109, 97, 103, 105, 99, 3, 110, 101, 119, 9, 110,
            101, 119, 95, 115, 119, 111, 114, 100, 6, 111, 98, 106, 101, 99, 116, 6, 115, 101, 110,
            100, 101, 114, 8, 115, 116, 114, 101, 110, 103, 116, 104, 12, 115, 119, 111, 114, 100,
            95, 99, 114, 101, 97, 116, 101, 14, 115, 119, 111, 114, 100, 115, 95, 99, 114, 101, 97,
            116, 101, 100, 8, 116, 114, 97, 110, 115, 102, 101, 114, 10, 116, 120, 95, 99, 111,
            110, 116, 101, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 3, 5, 8, 2, 7, 3, 12, 3, 1, 2, 2, 5, 8,
            2, 14, 3, 0, 0, 0, 0, 1, 9, 10, 0, 17, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 18, 1, 11, 0, 46,
            17, 8, 56, 0, 2, 1, 1, 0, 0, 1, 4, 11, 0, 16, 0, 20, 2, 2, 1, 0, 0, 1, 4, 11, 0, 16, 1,
            20, 2, 3, 1, 0, 0, 1, 4, 11, 0, 16, 2, 20, 2, 4, 1, 0, 0, 1, 6, 11, 2, 17, 6, 11, 0,
            11, 1, 18, 0, 2, 5, 1, 0, 0, 1, 14, 10, 0, 16, 2, 20, 6, 1, 0, 0, 0, 0, 0, 0, 0, 22,
            11, 0, 15, 2, 21, 11, 3, 17, 6, 11, 1, 11, 2, 18, 0, 2, 0, 1, 0, 2, 1, 1, 0,
        ];
        let deps = vec![
            ObjectId::from_str("0x1").unwrap(),
            ObjectId::from_str("0x2").unwrap(),
        ];
        let sender = tx.input(Serialized(&address));
        let upgrade_cap = tx.publish(vec![modules.to_vec()], deps);
        tx.transfer_objects(vec![upgrade_cap], sender);
        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();
        let effects = client.execute_tx(vec![sig], &tx).await;
        wait_for_tx_and_check_effects_status_success(&client, &tx, effects).await;
    }
}
