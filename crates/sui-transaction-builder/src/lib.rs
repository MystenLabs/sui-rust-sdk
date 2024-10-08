// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod error;

use error::ConversionError;
use error::Error;
use error::MissingDataError;
use error::UnsupportedLiteralError;
use sui_graphql_client::Client;
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
            return Err(MissingDataError::Sender.into());
        };
        if self.gas.is_empty() {
            return Err(MissingDataError::GasObjects.into());
        }
        let Some(budget) = self.gas_budget else {
            return Err(MissingDataError::GasBudget.into());
        };
        let Some(price) = self.gas_price else {
            return Err(MissingDataError::GasPrice.into());
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

    /// Attempts to resolve the transaction with the data provided so far by querying the GraphQL
    /// service.
    // TODO: finish the implementation when REST API is ready to provide the transaction resolution
    pub fn resolve(self, _client: &Client) -> Result<(), Error> {
        let sender = self.sender.ok_or(MissingDataError::Sender)?;
        let _unresolved_tx = {
            unresolved::Transaction {
                ptb: unresolved::ProgrammableTransaction {
                    inputs: self.inputs.clone(),
                    commands: self.commands.clone(),
                },
                sender,
                gas_payment: Some(unresolved::GasPayment {
                    objects: self
                        .gas
                        .clone()
                        .into_iter()
                        .map(try_from_gas_unresolved_input_to_unresolved_obj_ref)
                        .collect::<Result<Vec<_>, _>>()?,
                    owner: self.sponsor.unwrap_or(sender),
                    price: self.gas_price,
                    budget: self.gas_budget,
                }),
                expiration: self.expiration,
            }
        };

        Ok(())
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
            let object_id = input.object_id.ok_or(MissingDataError::ObjectId)?;
            let version = input.version;
            let digest = input.digest;
            Ok(unresolved::ObjectReference {
                object_id,
                version,
                digest,
            })
        }
        _ => Err(ConversionError::WrongGasObjectKind.into()),
    }
}

/// Convert from an [`unresolved::ObjectReference`] to a [`ObjectReference`].
fn try_from_unresolved_obj_ref(obj: unresolved::ObjectReference) -> Result<ObjectReference, Error> {
    let obj_id = obj.object_id;
    let version = obj.version.ok_or(MissingDataError::Version(obj_id))?;
    let digest = obj.digest.ok_or(MissingDataError::Digest(obj_id))?;
    Ok(ObjectReference::new(obj_id, version, digest))
}

/// Convert from an [`unresolved::Input`] into an [`Input`] for resolving the
/// transaction.

fn try_from_unresolved_input_arg(value: unresolved::Input) -> Result<Input, Error> {
    if let Some(kind) = value.kind {
        match kind {
            unresolved::InputKind::Pure => {
                let Some(value) = value.value else {
                    return Err(MissingDataError::PureValue.into());
                };

                match value {
                    unresolved::Value::String(v) => {
                        let bytes = base64ct::Base64::decode_vec(&v)
                            .map_err(ConversionError::DecodingError)?;
                        Ok(Input::Pure { value: bytes })
                    }
                    _ => Err(ConversionError::Input(
                        "expected a base64 string value for the Pure input argument".to_string(),
                    )
                    .into()),
                }
            }
            unresolved::InputKind::ImmutableOrOwned => {
                let Some(object_id) = value.object_id else {
                    return Err(MissingDataError::ObjectId.into());
                };
                let Some(version) = value.version else {
                    return Err(MissingDataError::Version(object_id).into());
                };
                let Some(digest) = value.digest else {
                    return Err(MissingDataError::Digest(object_id).into());
                };
                Ok(Input::ImmutableOrOwned(ObjectReference::new(
                    object_id, version, digest,
                )))
            }
            unresolved::InputKind::Shared => {
                let Some(object_id) = value.object_id else {
                    return Err(MissingDataError::ObjectId.into());
                };
                let Some(initial_shared_version) = value.version else {
                    return Err(MissingDataError::InitialSharedVersion(object_id).into());
                };
                let Some(mutable) = value.mutable else {
                    return Err(MissingDataError::SharedObjectMutability(object_id).into());
                };

                Ok(Input::Shared {
                    object_id,
                    initial_shared_version,
                    mutable,
                })
            }
            unresolved::InputKind::Receiving => {
                let Some(object_id) = value.object_id else {
                    return Err(MissingDataError::ObjectId.into());
                };
                let Some(version) = value.version else {
                    return Err(MissingDataError::Version(object_id).into());
                };
                let Some(digest) = value.digest else {
                    return Err(MissingDataError::Digest(object_id).into());
                };
                Ok(Input::Receiving(ObjectReference::new(
                    object_id, version, digest,
                )))
            }
            unresolved::InputKind::Literal => Err(UnsupportedLiteralError.into()),
        }
    } else {
        Err(ConversionError::Input(
            "unresolved::Input must have a kind that is not None".to_string(),
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
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

    // The compiled modules bcs for the Move project in the `sui/examples/move/first_package`
    // This is used for the publish and upgrade tests below.
    // To get the compiled modules bcs, you need to:
    // 1. change the Sui CLI code to print the `compiled_modules` in debug format
    // <https://github.com/mystenlabs/sui/blob/5d70ec6f5d4b7ff611649432aeb8c538efb63e80/crates/sui/src/client_commands.rs#L1018>
    // 2. build the `sui` binary
    // 3. go to `sui/examples/move/first_package` and run `sui client publish '.' --dry-run`
    // 4. copy paste the compiled_modules bcs output here
    const PACKAGE_MODULES: [u8; 506] = [
        161, 28, 235, 11, 6, 0, 0, 0, 10, 1, 0, 8, 2, 8, 16, 3, 24, 46, 4, 70, 2, 5, 72, 49, 7,
        121, 133, 1, 8, 254, 1, 64, 10, 190, 2, 18, 12, 208, 2, 119, 13, 199, 3, 6, 0, 4, 1, 10, 1,
        15, 1, 16, 0, 1, 12, 0, 0, 0, 8, 0, 1, 3, 4, 0, 3, 2, 2, 0, 0, 6, 0, 1, 0, 0, 7, 2, 3, 0,
        0, 12, 2, 3, 0, 0, 14, 4, 3, 0, 0, 13, 5, 6, 0, 0, 9, 7, 6, 0, 1, 8, 0, 8, 0, 2, 15, 12, 1,
        1, 8, 3, 11, 9, 10, 0, 7, 11, 1, 7, 8, 3, 0, 1, 6, 8, 0, 1, 3, 1, 6, 8, 1, 3, 3, 3, 7, 8,
        3, 1, 8, 0, 4, 7, 8, 1, 3, 3, 7, 8, 3, 1, 8, 2, 1, 6, 8, 3, 1, 5, 1, 8, 1, 2, 9, 0, 5, 5,
        70, 111, 114, 103, 101, 5, 83, 119, 111, 114, 100, 9, 84, 120, 67, 111, 110, 116, 101, 120,
        116, 3, 85, 73, 68, 7, 101, 120, 97, 109, 112, 108, 101, 2, 105, 100, 4, 105, 110, 105,
        116, 5, 109, 97, 103, 105, 99, 3, 110, 101, 119, 9, 110, 101, 119, 95, 115, 119, 111, 114,
        100, 6, 111, 98, 106, 101, 99, 116, 6, 115, 101, 110, 100, 101, 114, 8, 115, 116, 114, 101,
        110, 103, 116, 104, 12, 115, 119, 111, 114, 100, 95, 99, 114, 101, 97, 116, 101, 14, 115,
        119, 111, 114, 100, 115, 95, 99, 114, 101, 97, 116, 101, 100, 8, 116, 114, 97, 110, 115,
        102, 101, 114, 10, 116, 120, 95, 99, 111, 110, 116, 101, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 3, 5,
        8, 2, 7, 3, 12, 3, 1, 2, 2, 5, 8, 2, 14, 3, 0, 0, 0, 0, 1, 9, 10, 0, 17, 6, 6, 0, 0, 0, 0,
        0, 0, 0, 0, 18, 1, 11, 0, 46, 17, 8, 56, 0, 2, 1, 1, 0, 0, 1, 4, 11, 0, 16, 0, 20, 2, 2, 1,
        0, 0, 1, 4, 11, 0, 16, 1, 20, 2, 3, 1, 0, 0, 1, 4, 11, 0, 16, 2, 20, 2, 4, 1, 0, 0, 1, 6,
        11, 2, 17, 6, 11, 0, 11, 1, 18, 0, 2, 5, 1, 0, 0, 1, 14, 10, 0, 16, 2, 20, 6, 1, 0, 0, 0,
        0, 0, 0, 0, 22, 11, 0, 15, 2, 21, 11, 3, 17, 6, 11, 1, 11, 2, 18, 0, 2, 0, 1, 0, 2, 1, 1,
        0,
    ];

    // updated package. Has a new function.
    // to update the initial package, go to `sui/examples/move/first_package` and add a
    // function in the `sources/example.move` file. Then follow the comment from the const
    // PACKAGE_MODULES on how to print the bcs bytes of the update compiled modules using the CLI
    // and copy paste the output here.
    const UPDATED_PACKAGE_MODULES: [u8; 587] = [
        161, 28, 235, 11, 6, 0, 0, 0, 10, 1, 0, 8, 2, 8, 16, 3, 24, 51, 4, 75, 2, 5, 77, 57, 7,
        134, 1, 155, 1, 8, 161, 2, 64, 10, 225, 2, 18, 12, 243, 2, 163, 1, 13, 150, 4, 6, 0, 4, 1,
        11, 1, 16, 1, 17, 0, 1, 12, 0, 0, 0, 8, 0, 1, 3, 4, 0, 3, 2, 2, 0, 0, 6, 0, 1, 0, 0, 7, 2,
        3, 0, 0, 13, 2, 3, 0, 0, 15, 4, 3, 0, 0, 14, 5, 6, 0, 0, 10, 7, 6, 0, 0, 9, 8, 6, 0, 1, 8,
        0, 9, 0, 2, 16, 13, 1, 1, 8, 3, 12, 10, 11, 0, 8, 12, 1, 7, 8, 3, 0, 1, 6, 8, 0, 1, 3, 1,
        6, 8, 1, 3, 3, 3, 7, 8, 3, 1, 8, 0, 3, 7, 8, 1, 3, 7, 8, 3, 4, 7, 8, 1, 3, 3, 7, 8, 3, 1,
        8, 2, 1, 6, 8, 3, 1, 5, 1, 8, 1, 2, 9, 0, 5, 5, 70, 111, 114, 103, 101, 5, 83, 119, 111,
        114, 100, 9, 84, 120, 67, 111, 110, 116, 101, 120, 116, 3, 85, 73, 68, 7, 101, 120, 97,
        109, 112, 108, 101, 2, 105, 100, 4, 105, 110, 105, 116, 5, 109, 97, 103, 105, 99, 3, 110,
        101, 119, 9, 110, 101, 119, 95, 115, 119, 111, 114, 100, 21, 110, 101, 119, 95, 115, 119,
        111, 114, 100, 95, 109, 97, 120, 95, 115, 116, 114, 101, 110, 103, 104, 6, 111, 98, 106,
        101, 99, 116, 6, 115, 101, 110, 100, 101, 114, 8, 115, 116, 114, 101, 110, 103, 116, 104,
        12, 115, 119, 111, 114, 100, 95, 99, 114, 101, 97, 116, 101, 14, 115, 119, 111, 114, 100,
        115, 95, 99, 114, 101, 97, 116, 101, 100, 8, 116, 114, 97, 110, 115, 102, 101, 114, 10,
        116, 120, 95, 99, 111, 110, 116, 101, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 3, 5, 8, 2, 7, 3, 13, 3,
        1, 2, 2, 5, 8, 2, 15, 3, 0, 0, 0, 0, 1, 9, 10, 0, 17, 7, 6, 0, 0, 0, 0, 0, 0, 0, 0, 18, 1,
        11, 0, 46, 17, 9, 56, 0, 2, 1, 1, 0, 0, 1, 4, 11, 0, 16, 0, 20, 2, 2, 1, 0, 0, 1, 4, 11, 0,
        16, 1, 20, 2, 3, 1, 0, 0, 1, 4, 11, 0, 16, 2, 20, 2, 4, 1, 0, 0, 1, 6, 11, 2, 17, 7, 11, 0,
        11, 1, 18, 0, 2, 5, 1, 0, 0, 1, 14, 10, 0, 16, 2, 20, 6, 1, 0, 0, 0, 0, 0, 0, 0, 22, 11, 0,
        15, 2, 21, 11, 2, 17, 7, 11, 1, 6, 100, 0, 0, 0, 0, 0, 0, 0, 18, 0, 2, 6, 1, 0, 0, 1, 14,
        10, 0, 16, 2, 20, 6, 1, 0, 0, 0, 0, 0, 0, 0, 22, 11, 0, 15, 2, 21, 11, 3, 17, 7, 11, 1, 11,
        2, 18, 0, 2, 0, 1, 0, 2, 1, 1, 0,
    ];

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

        // publish very dummy package (from sui/examples/move/first_package)

        let deps = vec!["0x1".parse().unwrap(), "0x2".parse().unwrap()];
        let sender = tx.input(Serialized(&address));
        let upgrade_cap = tx.publish(vec![PACKAGE_MODULES.to_vec()], deps);
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

        // publish very dummy package (from sui/examples/move/first_package)

        let deps = vec!["0x1".parse().unwrap(), "0x2".parse().unwrap()];
        let sender = tx.input(Serialized(&address));
        let upgrade_cap = tx.publish(vec![PACKAGE_MODULES.to_vec()], deps.clone());
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
        // the digest of the new package that was compiled. Check PACKAGE_MODULES at the top of the
        // test module to see how to get this updated package digest.
        let package_digest: &[u8] = &[
            68, 89, 156, 51, 190, 35, 155, 216, 248, 49, 135, 170, 106, 42, 190, 4, 208, 59, 155,
            89, 74, 63, 70, 95, 207, 78, 227, 22, 136, 146, 57, 79,
        ];
        let digest_arg = tx.input(Serialized(&package_digest));

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
            vec![UPDATED_PACKAGE_MODULES.to_vec()],
            deps,
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
