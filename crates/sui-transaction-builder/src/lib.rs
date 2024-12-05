// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod error;

use error::Error;
use sui_types::types::unresolved;
use sui_types::types::Address;
use sui_types::types::Argument;
use sui_types::types::Command;
use sui_types::types::GasPayment;
use sui_types::types::Identifier;
use sui_types::types::Input;
use sui_types::types::MakeMoveVector;
use sui_types::types::MergeCoins;
use sui_types::types::MoveCall;
use sui_types::types::Object;
use sui_types::types::ObjectId;
use sui_types::types::ObjectReference;
use sui_types::types::Publish;
use sui_types::types::SplitCoins;
use sui_types::types::Transaction;
use sui_types::types::TransactionExpiration;
use sui_types::types::TransferObjects;
use sui_types::types::TypeTag;
use sui_types::types::Upgrade;

use base64ct::Encoding;
use serde::Serialize;

/// A builder for creating transactions. Use [`resolve`] to finalize the transaction data.
#[derive(Clone, Default, Debug)]
pub struct TransactionBuilder {
    /// The inputs to the transaction.
    inputs: Vec<unresolved::Input>,
    /// The list of commands in the transaction. A command is a single operation in a programmable
    /// transaction.
    commands: Vec<Command>,
    /// The gas objects that will be used to pay for the transaction. The most common way is to
    /// use [`unresolved::Input::owned`] function to create a gas object and use the [`add_gas`]
    /// method to set the gas objects.
    gas: Vec<unresolved::Input>,
    /// The gas budget for the transaction.
    gas_budget: Option<u64>,
    /// The gas price for the transaction.
    gas_price: Option<u64>,
    /// The sender of the transaction.
    sender: Option<Address>,
    /// The sponsor of the transaction. If None, the sender is also the sponsor.
    sponsor: Option<Address>,
    /// The expiration of the transaction.
    expiration: TransactionExpiration,
}

/// A transaction input that bypasses serialization. The input contents is already BCS serialized
/// and is put verbatim into the transaction.
struct RawBytes(Vec<u8>);

/// A transaction input that will be serialized from BCS.
pub struct Serialized<'a, T: Serialize>(pub &'a T);

/// A separate type to support denoting a function by a more structured representation.
pub struct Function {
    /// The package that contains the module with the function.
    package: Address,
    /// The module that contains the function.
    module: Identifier,
    /// The function name.
    function: Identifier,
    /// The type arguments for the function.
    type_args: Vec<TypeTag>,
}

/// A trait to convert a type into an [`unresolved::Input`]. This is mostly used for gas objects,
/// to pass them either as literals, as an object id, as a reference to an object, or as a
/// [`unresolved::Input`] object.
pub trait IntoInput {
    /// Convert the type into an [`unresolved::Input`].
    fn into_input(self) -> unresolved::Input;
}

/// A transaction builder to build transactions.
impl TransactionBuilder {
    /// Create a new transaction builder and initialize its elements to default.
    /// This sets the transaction expiration to have no expiration. Use `set_expiration` to set the
    /// transaction expiration to a specific epoch.
    pub fn new() -> Self {
        Self::default()
    }

    // Transaction Inputs

    /// Make a value available to the transaction as an input.
    pub fn input(&mut self, i: impl Into<unresolved::Input>) -> Argument {
        let input = i.into();
        self.inputs.push(input);
        Argument::Input((self.inputs.len() - 1) as u16)
    }

    /// Return the argument to be the gas object.
    pub fn gas(&self) -> Argument {
        Argument::Gas
    }

    // Metadata

    /// Set the gas objects that will be used to pay for the transaction. Most commonly the gas
    /// object can be either passed as literals and the builder will resolve it to the actual
    /// object, as a reference to an [`Object`], or can created using one of the constructors of the
    /// [`unresolved::Input`] enum, e.g., [`unresolved::Input::by_id`].
    ///
    /// Examples
    /// ```rust,ignore
    /// let mut tx = TransactionBuilder::new();
    /// // using literals
    /// tx.add_gas(vec!["0x19406ea4d9609cd9422b85e6bf2486908f790b778c757aff805241f3f609f9b4"]);
    ///
    /// // using input object and passing an ObjectId to the `by_id` function
    /// let gas_obj =
    /// unresolved::Input::by_id("0x19406ea4d9609cd9422b85e6bf2486908f790b778c757aff805241f3f609f9b4".parse().unwrap());
    /// tx.add_gas(vec![gas_obj]);
    // Define a trait in your crate
    pub fn add_gas<O, I>(&mut self, gas: I)
    where
        O: IntoInput,
        I: IntoIterator<Item = O>,
    {
        self.gas.extend(gas.into_iter().map(IntoInput::into_input));
    }

    /// Set the gas budget for the transaction.
    pub fn add_gas_budget(&mut self, budget: u64) {
        self.gas_budget = Some(budget);
    }

    /// Set the gas price for the transaction.
    pub fn add_gas_price(&mut self, price: u64) {
        self.gas_price = Some(price);
    }

    /// Set the sender of the transaction.
    pub fn set_sender(&mut self, sender: Address) {
        self.sender = Some(sender);
    }

    /// Set the sponsor of the transaction.
    pub fn set_sponsor(&mut self, sponsor: Address) {
        self.sponsor = Some(sponsor);
    }

    /// Set the expiration of the transaction.
    pub fn set_expiration(&mut self, epoch: u64) {
        self.expiration = TransactionExpiration::Epoch(epoch);
    }

    // Commands
    /// Call a public or a Move function with the given arguments.
    ///
    /// - `function` is a structured representation of a package::module::function argument.
    ///
    /// The return value is a result argument that can be used in subsequent commands.
    /// If the move call returns multiple results, you can access them using the
    /// [`Argument::nested`] method.
    pub fn move_call(&mut self, function: Function, arguments: Vec<Argument>) -> Argument {
        let cmd = Command::MoveCall(MoveCall {
            package: function.package.into(),
            module: function.module,
            function: function.function,
            type_arguments: function.type_args,
            arguments,
        });
        self.commands.push(cmd);
        Argument::Result(self.commands.len() as u16 - 1)
    }

    /// Transfer a list of objects to the given address, without producing any result.
    pub fn transfer_objects(&mut self, objects: Vec<Argument>, address: Argument) {
        let cmd = Command::TransferObjects(TransferObjects { objects, address });
        self.commands.push(cmd);
    }

    /// Splits a coin by the provided amounts, returning multiple results (as many as there are
    /// amounts). To access the results, use the [`Argument::nested`] method to access the desired
    /// coin by its index.
    pub fn split_coins(&mut self, coin: Argument, amounts: Vec<Argument>) -> Argument {
        let cmd = Command::SplitCoins(SplitCoins { coin, amounts });
        self.commands.push(cmd);
        Argument::Result(self.commands.len() as u16 - 1)
    }

    /// Merge a list of coins into a single coin, without producing any result.
    pub fn merge_coins(&mut self, coin: Argument, coins_to_merge: Vec<Argument>) {
        let cmd = Command::MergeCoins(MergeCoins {
            coin,
            coins_to_merge,
        });
        self.commands.push(cmd);
    }

    /// Make a move vector from a list of elements. If the elements are not objects, or the vector
    /// is empty, a type must be supplied.
    /// It returns the Move vector as an argument, that can be used in subsequent commands.
    pub fn make_move_vec(&mut self, type_: Option<TypeTag>, elements: Vec<Argument>) -> Argument {
        let cmd = Command::MakeMoveVector(MakeMoveVector { type_, elements });
        self.commands.push(cmd);
        Argument::Result(self.commands.len() as u16 - 1)
    }

    /// Publish a list of modules with the given dependencies. The result is the
    /// `0x2::package::UpgradeCap` move type. Note that the upgrade capability needs to be handled
    /// after this call:
    ///  - transfer it to the transaction sender or another address
    ///  - burn it
    ///  - wrap it for access control
    ///  - discard the it to make a package immutable
    ///
    /// The arguments required for this command are:
    ///  - `modules`: is the modules' bytecode to be published
    ///  - `dependencies`: is the list of IDs of the transitive dependencies of the package to be
    pub fn publish(&mut self, modules: Vec<Vec<u8>>, dependencies: Vec<ObjectId>) -> Argument {
        let cmd = Command::Publish(Publish {
            modules,
            dependencies,
        });
        self.commands.push(cmd);
        Argument::Result(self.commands.len() as u16 - 1)
    }

    /// Upgrade a Move package.
    ///
    ///  - `modules`: is the modules' bytecode for the modules to be published
    ///  - `dependencies`: is the list of IDs of the transitive dependencies of the package to be
    ///     upgraded
    ///  - `package`: is the ID of the current package being upgraded
    ///  - `ticket`: is the upgrade ticket
    ///
    ///  To get the ticket, you have to call the `0x2::package::authorize_upgrade` function,
    ///  and pass the package ID, the upgrade policy, and package digest.
    ///
    ///  Examples:
    ///  ### Upgrade a package with some pre-known data.
    ///  ```rust,ignore
    ///  use sui_graphql_client::Client;
    ///  use sui_sdk_types::types::unresolved;
    ///  use sui_transaction_builder::TransactionBuilder;
    ///  use sui_transaction_builder::Function;
    ///
    ///  let mut tx = TransactionBuilder::new();
    ///  let package_id = "0x...".parse().unwrap();
    ///  let upgrade_cap = tx.input(unresolved::Input::by_id("0x...".parse().unwrap());
    ///  let upgrade_policy = tx.input(Serialized(&0u8));
    ///  // the digest of the new package that was compiled
    ///  let package_digest: &[u8] = &[
    ///       68, 89, 156, 51, 190, 35, 155, 216, 248, 49, 135, 170, 106, 42, 190, 4, 208, 59, 155,
    ///       89, 74, 63, 70, 95, 207, 78, 227, 22, 136, 146, 57, 79,
    ///  ];
    ///  let digest_arg = tx.input(Serialized(&package_digest));
    ///
    ///  // we need this ticket to authorize the upgrade
    ///  let upgrade_ticket = tx.move_call(
    ///      Function::new(
    ///        "0x2".parse().unwrap(),
    ///         "package".parse().unwrap(),
    ///         "authorize_upgrade".parse().unwrap(),
    ///         vec![],
    ///      ),
    ///      vec![upgrade_cap, upgrade_policy, digest_arg],
    ///    );
    ///  // now we can upgrade the package
    ///  let upgrade_receipt = tx.upgrade(
    ///       updated_modules,
    ///       deps,
    ///       package_id,
    ///       upgrade_ticket,
    ///  );
    ///
    ///  // commit the upgrade
    ///  tx.move_call(
    ///       Function::new(
    ///          "0x2".parse().unwrap(),
    ///          "package".parse().unwrap(),
    ///          "commit_upgrade".parse().unwrap(),
    ///          vec![],
    ///      ),
    ///      vec![upgrade_cap, upgrade_receipt],
    ///  );
    ///
    ///  let client = Client::new_mainnet();
    ///  let tx = tx.resolve(&client)?;
    ///  ```
    pub fn upgrade(
        &mut self,
        modules: Vec<Vec<u8>>,
        dependencies: Vec<ObjectId>,
        package: ObjectId,
        ticket: Argument,
    ) -> Argument {
        let cmd = Command::Upgrade(Upgrade {
            modules,
            dependencies,
            package,
            ticket,
        });
        self.commands.push(cmd);
        Argument::Result(self.commands.len() as u16 - 1)
    }

    /// Assuming everything is resolved, convert this transaction into the
    /// resolved form. Returns a [`Transaction`] if successful, or an [`Error`] if not.
    /// Use the [`resolve`] function to resolve the transaction if not all the data is provided.
    pub fn finish(self) -> Result<Transaction, Error> {
        let Some(sender) = self.sender else {
            return Err(Error::MissingSender);
        };
        if self.gas.is_empty() {
            return Err(Error::MissingGasObjects);
        }
        let Some(budget) = self.gas_budget else {
            return Err(Error::MissingGasBudget);
        };
        let Some(price) = self.gas_price else {
            return Err(Error::MissingGasPrice);
        };

        Ok(Transaction {
            kind: sui_types::types::TransactionKind::ProgrammableTransaction(
                sui_types::types::ProgrammableTransaction {
                    inputs: self
                        .inputs
                        .into_iter()
                        .map(try_from_unresolved_input_arg)
                        .collect::<Result<Vec<_>, _>>()?,
                    commands: self.commands,
                },
            ),
            sender,
            gas_payment: {
                GasPayment {
                    objects: self
                        .gas
                        .into_iter()
                        .map(try_from_gas_unresolved_input_to_unresolved_obj_ref)
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .map(try_from_unresolved_obj_ref)
                        .collect::<Result<Vec<_>, _>>()?,
                    owner: self.sponsor.unwrap_or(sender),
                    price,
                    budget,
                }
            },
            expiration: self.expiration,
        })
    }
}

impl Function {
    /// Constructor for the function type.
    pub fn new(
        package: Address,
        module: Identifier,
        function: Identifier,
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

impl From<RawBytes> for unresolved::Input {
    fn from(raw: RawBytes) -> Self {
        Self {
            kind: Some(unresolved::InputKind::Pure),
            value: Some(unresolved::Value::String(base64ct::Base64::encode_string(
                &raw.0,
            ))),
            object_id: None,
            version: None,
            digest: None,
            mutable: None,
        }
    }
}

impl<'a, T: Serialize> From<Serialized<'a, T>> for unresolved::Input {
    fn from(value: Serialized<'a, T>) -> Self {
        Self::from(RawBytes(bcs::to_bytes(value.0).unwrap()))
    }
}

impl IntoInput for unresolved::Input {
    /// Pass the input as is.
    fn into_input(self) -> unresolved::Input {
        self
    }
}

impl IntoInput for Object {
    /// Convert the [`Object`] type into an [`unresolved::Input`] of kind immutable.
    fn into_input(self) -> unresolved::Input {
        unresolved::Input::immutable(self.object_id(), self.version(), self.digest())
    }
}

impl IntoInput for ObjectId {
    /// Convert the [`ObjectId`] type into an [`unresolved::Input`] with only object id as partial
    /// information.
    fn into_input(self) -> unresolved::Input {
        unresolved::Input::by_id(self)
    }
}

/// Convert from an [`unresolved::Input`] to a [`unresolved::ObjectReference`]. This is used to
/// convert gas objects into unresolved object references.
fn try_from_gas_unresolved_input_to_unresolved_obj_ref(
    input: unresolved::Input,
) -> Result<unresolved::ObjectReference, Error> {
    match input.kind {
        Some(unresolved::InputKind::ImmutableOrOwned) => {
            let object_id = input.object_id.ok_or(Error::MissingObjectId)?;
            let version = input.version;
            let digest = input.digest;
            Ok(unresolved::ObjectReference {
                object_id,
                version,
                digest,
            })
        }
        _ => Err(Error::WrongGasObject),
    }
}

/// Convert from an [`unresolved::ObjectReference`] to a [`ObjectReference`].
fn try_from_unresolved_obj_ref(obj: unresolved::ObjectReference) -> Result<ObjectReference, Error> {
    let obj_id = obj.object_id;
    let version = obj.version.ok_or(Error::MissingVersion(obj_id))?;
    let digest = obj.digest.ok_or(Error::MissingDigest(obj_id))?;
    Ok(ObjectReference::new(obj_id, version, digest))
}

/// Convert from an [`unresolved::Input`] into an [`Input`] for resolving the
/// transaction.

fn try_from_unresolved_input_arg(value: unresolved::Input) -> Result<Input, Error> {
    if let Some(kind) = value.kind {
        match kind {
            unresolved::InputKind::Pure => {
                let Some(value) = value.value else {
                    return Err(Error::MissingPureValue);
                };

                match value {
                    unresolved::Value::String(v) => {
                        let bytes = base64ct::Base64::decode_vec(&v).map_err(Error::Decoding)?;
                        Ok(Input::Pure { value: bytes })
                    }
                    _ => Err(Error::Input(
                        "expected a base64 string value for the Pure input argument".to_string(),
                    )),
                }
            }
            unresolved::InputKind::ImmutableOrOwned => {
                let Some(object_id) = value.object_id else {
                    return Err(Error::MissingObjectId);
                };
                let Some(version) = value.version else {
                    return Err(Error::MissingVersion(object_id));
                };
                let Some(digest) = value.digest else {
                    return Err(Error::MissingDigest(object_id));
                };
                Ok(Input::ImmutableOrOwned(ObjectReference::new(
                    object_id, version, digest,
                )))
            }
            unresolved::InputKind::Shared => {
                let Some(object_id) = value.object_id else {
                    return Err(Error::MissingObjectId);
                };
                let Some(initial_shared_version) = value.version else {
                    return Err(Error::MissingInitialSharedVersion(object_id));
                };
                let Some(mutable) = value.mutable else {
                    return Err(Error::SharedObjectMutability(object_id));
                };

                Ok(Input::Shared {
                    object_id,
                    initial_shared_version,
                    mutable,
                })
            }
            unresolved::InputKind::Receiving => {
                let Some(object_id) = value.object_id else {
                    return Err(Error::MissingObjectId);
                };
                let Some(version) = value.version else {
                    return Err(Error::MissingVersion(object_id));
                };
                let Some(digest) = value.digest else {
                    return Err(Error::MissingDigest(object_id));
                };
                Ok(Input::Receiving(ObjectReference::new(
                    object_id, version, digest,
                )))
            }
            unresolved::InputKind::Literal => Err(Error::UnsupportedLiteral),
        }
    } else {
        Err(Error::Input(
            "unresolved::Input must have a kind that is not None".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Context;
    use base64ct::Encoding;
    use serde::de;
    use serde::Deserialize;
    use serde::Deserializer;
    use sui_crypto::ed25519::Ed25519PrivateKey;
    use sui_crypto::SuiSigner;
    use sui_graphql_client::faucet::CoinInfo;
    use sui_graphql_client::faucet::FaucetClient;
    use sui_graphql_client::Client;
    use sui_graphql_client::PaginationFilter;
    use sui_types::types::Address;
    use sui_types::types::ExecutionStatus;
    use sui_types::types::IdOperation;
    use sui_types::types::ObjectId;
    use sui_types::types::ObjectType;
    use sui_types::types::Transaction;
    use sui_types::types::TransactionEffects;
    use sui_types::types::TypeTag;

    use crate::unresolved;
    use crate::Function;
    use crate::Serialized;
    use crate::TransactionBuilder;

    /// Type corresponding to the output of `sui move build --dump-bytecode-as-base64`
    #[derive(serde::Deserialize, Debug)]
    struct MovePackageData {
        #[serde(deserialize_with = "bcs_from_str")]
        modules: Vec<Vec<u8>>,
        #[serde(deserialize_with = "deps_from_str")]
        dependencies: Vec<ObjectId>,
        digest: Vec<u8>,
    }

    fn bcs_from_str<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bcs = Vec::<String>::deserialize(deserializer)?;
        bcs.into_iter()
            .map(|s| base64ct::Base64::decode_vec(&s).map_err(de::Error::custom))
            .collect()
    }

    fn deps_from_str<'de, D>(deserializer: D) -> Result<Vec<ObjectId>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deps = Vec::<String>::deserialize(deserializer)?;
        deps.into_iter()
            .map(|s| ObjectId::from_str(&s).map_err(de::Error::custom))
            .collect()
    }

    /// This is used to read the json file that contains the modules/deps/digest generated with sui
    /// move build --dump-bytecode-as-base64 on the `test_example_v1 and test_example_v2` projects
    /// in the tests directory.
    /// The json files are generated automatically when running `make test-with-localnet` in the
    /// root of the sui-transaction-builder crate.
    fn move_package_data(file: &str) -> MovePackageData {
        let data = std::fs::read_to_string(file)
            .with_context(|| {
                format!(
                    "Failed to read {file}. \
                    Run `make test-with-localnet` from the root of the repository that will \
                    generate the right json files with the package data and then run the tests."
                )
            })
            .unwrap();
        serde_json::from_str(&data).unwrap()
    }

    /// Generate a random private key and its corresponding address
    fn helper_address_pk() -> (Address, Ed25519PrivateKey) {
        let pk = Ed25519PrivateKey::generate(rand::thread_rng());
        let address = pk.public_key().to_address();
        (address, pk)
    }

    /// Generate a private key and its corresponding address, call faucet which returns 5 coin
    /// objects, set the sender to this address, set the gas object (last coin from the returned
    /// Vec<CoinInfo>), set gas price, set gas budget, and return the address, private key, and
    /// coins.
    ///
    /// NB! This assumes that these tests run on a network whose faucet returns 5 coins per
    /// each faucet request.
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
        println!("Coins: {:?}", coins);
        let gas = coins.last().unwrap().id;
        let gas_obj = client.object(gas.into(), None).await.unwrap().unwrap();
        tx.add_gas(vec![gas_obj]);
        tx.add_gas_budget(500000000);
        tx.add_gas_price(1000);
        tx.set_sender(address);

        (address, pk, coins)
    }

    async fn wait_for_tx_and_check_effects_status_success(
        client: &Client,
        tx: &Transaction,
        effects: Result<Option<TransactionEffects>, sui_graphql_client::error::Error>,
    ) {
        assert!(effects.is_ok(), "Execution failed. Effects: {:?}", effects);
        // wait for the transaction to be finalized
        while client
            .transaction(tx.digest().into())
            .await
            .unwrap()
            .is_none()
        {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        // check that it succeeded
        let status = effects.unwrap();
        let expected_status = ExecutionStatus::Success;
        assert_eq!(&expected_status, status.as_ref().unwrap().status());
    }

    #[tokio::test]
    async fn test_finish() {
        let mut tx = TransactionBuilder::new();
        let coin_obj_id = "0x19406ea4d9609cd9422b85e6bf2486908f790b778c757aff805241f3f609f9b4";
        let coin_digest = "7opR9rFUYivSTqoJHvFb9p6p54THyHTatMG6id4JKZR9";
        let coin_version = 2;
        let coin = tx.input(unresolved::Input::owned(
            coin_obj_id.parse().unwrap(),
            coin_version,
            coin_digest.parse().unwrap(),
        ));

        let addr = Address::generate(rand::thread_rng());
        let recipient = tx.input(Serialized(&addr));

        let result = tx.clone().finish();
        assert!(result.is_err());

        tx.transfer_objects(vec![coin], recipient);
        tx.add_gas_budget(500000000);
        tx.add_gas_price(1000);
        tx.add_gas(vec![unresolved::Input::immutable(
            "0xd8792bce2743e002673752902c0e7348dfffd78638cb5367b0b85857bceb9821"
                .parse()
                .unwrap(),
            2,
            "2ZigdvsZn5BMeszscPQZq9z8ebnS2FpmAuRbAi9ednCk"
                .parse()
                .unwrap(),
        )]);
        tx.set_sender(
            "0xc574ea804d9c1a27c886312e96c0e2c9cfd71923ebaeb3000d04b5e65fca2793"
                .parse()
                .unwrap(),
        );

        let tx = tx.finish();
        assert!(tx.is_ok());
    }

    /// Test TransferObj PTB.
    #[tokio::test]
    async fn test_transfer_obj_execution() {
        let mut tx = TransactionBuilder::new();
        let (_, pk, coins) = helper_setup(&mut tx, &Client::new_localhost()).await;

        // get the object information from the client
        let client = Client::new_localhost();
        let first = coins.first().unwrap().id;
        let coin = client.object(first.into(), None).await.unwrap().unwrap();
        let coin_digest = coin.digest();
        let coin_version = coin.version();

        let coin_input = tx.input(unresolved::Input::owned(
            coin.object_id(),
            coin_version,
            coin_digest,
        ));
        let recipient = Address::generate(rand::thread_rng());
        let recipient_input = tx.input(Serialized(&recipient));
        tx.transfer_objects(vec![coin_input], recipient_input);

        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();

        let effects = client.execute_tx(vec![sig], &tx).await;
        wait_for_tx_and_check_effects_status_success(&client, &tx, effects).await;

        // check that recipient has 1 coin
        let recipient_coins = client
            .coins(recipient, None, PaginationFilter::default())
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
            "option".parse().unwrap(),
            "is_none".parse().unwrap(),
            vec![TypeTag::U64],
        );
        let input = tx.input(Serialized(&vec![1u64]));
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
        let (_, pk, _) = helper_setup(&mut tx, &client).await;

        // transfer 1 SUI from Gas coin
        let amount = tx.input(Serialized(&1_000_000_000u64));
        let result = tx.split_coins(tx.gas(), vec![amount]);
        let recipient_address = Address::generate(rand::thread_rng());
        let recipient = tx.input(Serialized(&recipient_address));
        tx.transfer_objects(vec![result], recipient);

        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();

        let effects = client.execute_tx(vec![sig], &tx).await;
        wait_for_tx_and_check_effects_status_success(&client, &tx, effects).await;

        // check that recipient has 1 coin
        let recipient_coins = client
            .coins(recipient_address, None, PaginationFilter::default())
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

        let coin_input = tx.input(unresolved::Input::owned(
            coin_obj.object_id(),
            coin_version,
            coin_digest,
        ));
        // transfer 1 SUI
        let amount = tx.input(Serialized(&1_000_000_000u64));
        tx.split_coins(coin_input, vec![amount]);

        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();

        let effects = client.execute_tx(vec![sig], &tx).await;
        assert!(effects.is_ok());

        // wait for the transaction to be finalized
        loop {
            let tx_digest = client.transaction(tx.digest().into()).await.unwrap();
            if tx_digest.is_some() {
                break;
            }
        }
        assert!(effects.is_ok());
        let status = effects.unwrap();
        let expected_status = ExecutionStatus::Success;
        // The tx failed, so we expect Failure instead of Success
        assert_ne!(&expected_status, status.as_ref().unwrap().status());
    }

    #[tokio::test]
    async fn test_merge_coins() {
        let client = Client::new_localhost();
        let mut tx = TransactionBuilder::new();
        let (address, pk, coins) = helper_setup(&mut tx, &client).await;

        let coin1 = coins.first().unwrap().id;
        let coin1_obj = client.object(coin1.into(), None).await.unwrap().unwrap();
        let coin1_digest = coin1_obj.digest();
        let coin1_version = coin1_obj.version();

        let coin_to_merge = tx.input(unresolved::Input::owned(
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
            coins_to_merge.push(tx.input(unresolved::Input::owned(
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

        // check that there are two coins
        let coins_after = client
            .coins(address, None, PaginationFilter::default())
            .await
            .unwrap();
        assert_eq!(coins_after.data().len(), 2);
    }

    #[tokio::test]
    async fn test_make_move_vec() {
        let client = Client::new_localhost();
        let mut tx = TransactionBuilder::new();
        let (_, pk, _) = helper_setup(&mut tx, &client).await;

        let input = tx.input(Serialized(&1u64));
        tx.make_move_vec(Some(TypeTag::U64), vec![input]);

        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();

        let effects = client.execute_tx(vec![sig], &tx).await;
        wait_for_tx_and_check_effects_status_success(&client, &tx, effects).await;
    }

    #[tokio::test]
    async fn test_publish() {
        let client = Client::new_localhost();
        let mut tx = TransactionBuilder::new();
        let (address, pk, _) = helper_setup(&mut tx, &client).await;

        let package = move_package_data("test_example_v1_build_output.json");
        let sender = tx.input(Serialized(&address));
        let upgrade_cap = tx.publish(package.modules, package.dependencies);
        tx.transfer_objects(vec![upgrade_cap], sender);
        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();
        let effects = client.execute_tx(vec![sig], &tx).await;
        wait_for_tx_and_check_effects_status_success(&client, &tx, effects).await;
    }

    #[tokio::test]
    async fn test_upgrade() {
        let client = Client::new_localhost();
        let mut tx = TransactionBuilder::new();
        let (address, pk, coins) = helper_setup(&mut tx, &client).await;

        let package = move_package_data("test_example_v1_build_output.json");
        let sender = tx.input(Serialized(&address));
        let upgrade_cap = tx.publish(package.modules, package.dependencies);
        tx.transfer_objects(vec![upgrade_cap], sender);
        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();
        let effects = client.execute_tx(vec![sig], &tx).await;
        let mut package_id: Option<ObjectId> = None;
        let mut created_objs = vec![];
        if let Ok(Some(ref effects)) = effects {
            match effects {
                TransactionEffects::V2(e) => {
                    for obj in e.changed_objects.clone() {
                        if obj.change.id_operation == IdOperation::Created {
                            let change = obj.change.output_state;
                            match change {
                                sui_types::types::ObjectOut::PackageWrite { .. } => {
                                    package_id = Some(obj.object_id);
                                }
                                sui_types::types::ObjectOut::ObjectWrite { .. } => {
                                    created_objs.push(obj.object_id);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => panic!("Expected V2 effects"),
            }
        }
        wait_for_tx_and_check_effects_status_success(&client, &tx, effects).await;

        let mut tx = TransactionBuilder::new();
        let mut upgrade_cap = None;
        for o in created_objs {
            let obj = client.object(*o.as_address(), None).await.unwrap().unwrap();
            match obj.object_type() {
                ObjectType::Struct(x) if x.name.to_string() == "UpgradeCap" => {
                    match obj.owner() {
                        sui_types::types::Owner::Address(_) => {
                            upgrade_cap = Some(
                            tx.input(unresolved::Input::immutable(o, obj.version(), obj.digest())));
                        }
                        sui_types::types::Owner::Shared(x) => {
                            upgrade_cap = Some(tx.input(unresolved::Input::shared(o, *x, true)));
                        }
                        // If the capability is owned by an object, then the module defining the owning
                        // object gets to decide how the upgrade capability should be used.
                        sui_types::types::Owner::Object(_) => {
                            panic!("Upgrade capability controlled by object")
                        }
                        sui_types::types::Owner::Immutable => panic!("Upgrade capability is stored immutably and cannot be used for upgrades"),
                    };
                    break;
                }
                _ => {}
            };
        }

        let upgrade_policy = tx.input(Serialized(&0u8));
        let updated_package = move_package_data("test_example_v2_build_output.json");
        let digest_arg = tx.input(Serialized(&updated_package.digest));

        // we need this ticket to authorize the upgrade
        let upgrade_ticket = tx.move_call(
            Function::new(
                "0x2".parse().unwrap(),
                "package".parse().unwrap(),
                "authorize_upgrade".parse().unwrap(),
                vec![],
            ),
            vec![upgrade_cap.unwrap(), upgrade_policy, digest_arg],
        );
        // now we can upgrade the package
        let upgrade_receipt = tx.upgrade(
            updated_package.modules,
            updated_package.dependencies,
            package_id.unwrap(),
            upgrade_ticket,
        );

        // commit the upgrade
        tx.move_call(
            Function::new(
                "0x2".parse().unwrap(),
                "package".parse().unwrap(),
                "commit_upgrade".parse().unwrap(),
                vec![],
            ),
            vec![upgrade_cap.unwrap(), upgrade_receipt],
        );

        // we should be able to pass gas by obj id, but we need the resolution api for that so for
        // now we need to pass the object or a reference.
        let gas = coins.last().unwrap().id;
        let gas_obj = client.object(gas.into(), None).await.unwrap().unwrap();
        tx.add_gas(vec![gas_obj]);
        tx.add_gas_budget(500000000);
        tx.add_gas_price(1000);
        tx.set_sender(address);
        let tx = tx.finish().unwrap();
        let sig = pk.sign_transaction(&tx).unwrap();
        let effects = client.execute_tx(vec![sig], &tx).await;
        wait_for_tx_and_check_effects_status_success(&client, &tx, effects).await;
    }
}
