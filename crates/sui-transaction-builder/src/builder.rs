use crate::error::Error;
use crate::intent::Intent;
use crate::intent::IntentResolver;
use std::any::TypeId;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use sui_sdk_types::Address;
use sui_sdk_types::Digest;
use sui_sdk_types::Identifier;
use sui_sdk_types::Transaction;
use sui_sdk_types::TransactionExpiration;
use sui_sdk_types::TypeTag;

/// A builder for creating transactions. Use `resolve` to finalize the transaction data.
#[derive(Default)]
pub struct TransactionBuilder {
    /// The gas objects that will be used to pay for the transaction. The most common way is to
    /// use [`unresolved::Input::owned`] function to create a gas object and use the [`add_gas`]
    /// method to set the gas objects.
    pub(crate) gas: Vec<ObjectInput>,
    /// The gas budget for the transaction.
    gas_budget: Option<u64>,
    /// The gas price for the transaction.
    gas_price: Option<u64>,
    /// The sender of the transaction.
    sender: Option<Address>,
    /// The sponsor of the transaction. If None, the sender is also the sponsor.
    sponsor: Option<Address>,
    /// The expiration of the transaction. The default value of this type is no expiration.
    expiration: Option<TransactionExpiration>,

    // Resolvers
    pub(crate) resolvers: BTreeMap<TypeId, Box<dyn IntentResolver>>,

    pub(crate) arguments: BTreeMap<usize, ResolvedArgument>,
    inputs: HashMap<InputArgKind, (usize, InputArg)>,
    pub(crate) commands: BTreeMap<usize, Command>,
    pub(crate) intents: BTreeMap<usize, Box<dyn std::any::Any + Send + Sync>>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum ResolvedArgument {
    Unresolved,
    ReplaceWith(Argument),
    Resolved(sui_sdk_types::Argument),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum InputArgKind {
    Gas,
    ObjectInput(Address),
    PureInput(Vec<u8>),
    UniquePureInput(usize),
}

pub(crate) enum InputArg {
    Gas,
    Pure(Vec<u8>),
    Object(ObjectInput),
}

impl TransactionBuilder {
    /// Create a new transaction builder and initialize its elements to default.
    pub fn new() -> Self {
        Self::default()
    }

    // Transaction Inputs

    pub fn gas(&mut self) -> Argument {
        if let Some((index, arg)) = self.inputs.get(&InputArgKind::Gas) {
            assert!(matches!(arg, InputArg::Gas));
            Argument::new(*index)
        } else {
            let id = self.arguments.len();
            self.arguments.insert(id, ResolvedArgument::Unresolved);
            self.inputs.insert(InputArgKind::Gas, (id, InputArg::Gas));
            Argument::new(id)
        }
    }

    pub fn pure_bytes(&mut self, bytes: Vec<u8>) -> Argument {
        match self.inputs.entry(InputArgKind::PureInput(bytes.clone())) {
            std::collections::hash_map::Entry::Occupied(o) => {
                assert!(matches!(o.get().1, InputArg::Pure(_)));
                Argument::new(o.get().0)
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                let id = self.arguments.len();
                self.arguments.insert(id, ResolvedArgument::Unresolved);
                v.insert((id, InputArg::Pure(bytes)));
                Argument::new(id)
            }
        }
    }

    pub fn pure<T: serde::Serialize>(&mut self, value: &T) -> Argument {
        let bytes = bcs::to_bytes(value).expect("bcs serialization failed");
        self.pure_bytes(bytes)
    }

    pub fn pure_bytes_unique(&mut self, bytes: Vec<u8>) -> Argument {
        let id = self.arguments.len();
        self.arguments.insert(id, ResolvedArgument::Unresolved);
        self.inputs.insert(
            InputArgKind::UniquePureInput(id),
            (id, InputArg::Pure(bytes)),
        );
        Argument::new(id)
    }

    pub fn pure_unique<T: serde::Serialize>(&mut self, value: &T) -> Argument {
        let bytes = bcs::to_bytes(value).expect("bcs serialization failed");
        self.pure_bytes_unique(bytes)
    }

    pub fn object(&mut self, object: ObjectInput) -> Argument {
        match self
            .inputs
            .entry(InputArgKind::ObjectInput(object.object_id))
        {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                let id = o.get().0;
                let InputArg::Object(object2) = &mut o.get_mut().1 else {
                    panic!("BUG: invariant violation");
                };

                assert_eq!(
                    object.object_id, object2.object_id,
                    "BUG: invariant violation"
                );

                match (object.mutable, object2.mutable) {
                    (Some(_), None) => object2.mutable = object.mutable,
                    (Some(true), Some(false)) => object2.mutable = Some(true),
                    _ => {}
                }

                if let (Some(kind), None) = (object.kind, object2.kind) {
                    object2.kind = Some(kind);
                }

                if let (Some(version), None) = (object.version, object2.version) {
                    object2.version = Some(version);
                }

                if let (Some(digest), None) = (object.digest, object2.digest) {
                    object2.digest = Some(digest);
                }

                Argument::new(id)
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                let id = self.arguments.len();
                self.arguments.insert(id, ResolvedArgument::Unresolved);
                v.insert((id, InputArg::Object(object)));
                Argument::new(id)
            }
        }
    }

    // Metadata

    /// Add one or more gas objects to use to pay for the transaction.
    pub fn add_gas_objects<O, I>(&mut self, gas: I)
    where
        O: Into<ObjectInput>,
        I: IntoIterator<Item = O>,
    {
        self.gas.extend(gas.into_iter().map(|x| x.into()));
    }

    /// Set the gas budget for the transaction.
    pub fn set_gas_budget(&mut self, budget: u64) {
        self.gas_budget = Some(budget);
    }

    /// Set the gas price for the transaction.
    pub fn set_gas_price(&mut self, price: u64) {
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
    pub fn set_expiration(&mut self, expiration: TransactionExpiration) {
        self.expiration = Some(expiration);
    }

    // Commands

    fn command(&mut self, command: Command) -> Argument {
        let id = self.arguments.len();
        self.arguments.insert(id, ResolvedArgument::Unresolved);
        self.commands.insert(id, command);
        Argument::new(id)
    }

    /// Call a Move function with the given arguments.
    ///
    /// - `function` is a structured representation of a package::module::function argument,
    ///   optionally with type arguments.
    //
    // The return value is a result argument that can be used in subsequent commands.
    // If the move call returns multiple results, you can access them using the
    // [`Argument::nested`] method.
    pub fn move_call(&mut self, function: Function, arguments: Vec<Argument>) -> Argument {
        let cmd = CommandKind::MoveCall(MoveCall {
            package: function.package,
            module: function.module,
            function: function.function,
            type_arguments: function.type_args,
            arguments,
        });
        self.command(cmd.into())
    }

    /// Transfer a list of objects to the given address, without producing any result.
    pub fn transfer_objects(&mut self, objects: Vec<Argument>, address: Argument) {
        let cmd = CommandKind::TransferObjects(TransferObjects { objects, address });
        self.command(cmd.into());
    }

    /// Split a coin by the provided amounts, returning multiple results (as many as there are
    /// amounts). The returned vector of `Arguments` is guaranteed to be the same length as the
    /// provided `amounts` vector.
    pub fn split_coins(&mut self, coin: Argument, amounts: Vec<Argument>) -> Vec<Argument> {
        let amounts_len = amounts.len();
        let cmd = CommandKind::SplitCoins(SplitCoins { coin, amounts });
        self.command(cmd.into()).to_nested(amounts_len)
    }

    /// Merge a list of coins into a single coin, without producing any result.
    pub fn merge_coins(&mut self, coin: Argument, coins_to_merge: Vec<Argument>) {
        let cmd = CommandKind::MergeCoins(MergeCoins {
            coin,
            coins_to_merge,
        });
        self.command(cmd.into());
    }

    /// Make a move vector from a list of elements. If the elements are not objects, or the vector
    /// is empty, a type must be supplied.
    /// It returns the Move vector as an argument, that can be used in subsequent commands.
    pub fn make_move_vec(&mut self, type_: Option<TypeTag>, elements: Vec<Argument>) -> Argument {
        let cmd = CommandKind::MakeMoveVector(MakeMoveVector { type_, elements });
        self.command(cmd.into())
    }

    /// Publish a list of modules with the given dependencies. The result is the
    /// `0x2::package::UpgradeCap` Move type. Note that the upgrade capability needs to be handled
    /// after this call:
    ///  - transfer it to the transaction sender or another address
    ///  - burn it
    ///  - wrap it for access control
    ///  - discard the it to make a package immutable
    ///
    /// The arguments required for this command are:
    ///  - `modules`: is the modules' bytecode to be published
    ///  - `dependencies`: is the list of IDs of the transitive dependencies of the package
    pub fn publish(&mut self, modules: Vec<Vec<u8>>, dependencies: Vec<Address>) -> Argument {
        let cmd = CommandKind::Publish(Publish {
            modules,
            dependencies,
        });
        self.command(cmd.into())
    }

    /// Upgrade a Move package.
    ///
    ///  - `modules`: is the modules' bytecode for the modules to be published
    ///  - `dependencies`: is the list of IDs of the transitive dependencies of the package to be
    ///    upgraded
    ///  - `package`: is the ID of the current package being upgraded
    ///  - `ticket`: is the upgrade ticket
    ///
    ///  To get the ticket, you have to call the `0x2::package::authorize_upgrade` function,
    ///  and pass the package ID, the upgrade policy, and package digest.
    pub fn upgrade(
        &mut self,
        modules: Vec<Vec<u8>>,
        dependencies: Vec<Address>,
        package: Address,
        ticket: Argument,
    ) -> Argument {
        let cmd = CommandKind::Upgrade(Upgrade {
            modules,
            dependencies,
            package,
            ticket,
        });
        self.command(cmd.into())
    }

    // Intents

    // Register a transaction intent which may be resolved later to either an input or a sequence
    // of commands.
    #[allow(private_bounds)]
    pub fn intent<I: Intent>(&mut self, intent: I) -> Argument {
        intent.register(self)
    }

    // Building and resolving

    /// Assuming everything is resolved, convert this transaction into the
    /// resolved form. Returns a [`Transaction`] if successful, or an `Error` if not.
    pub fn try_build(mut self) -> Result<Transaction, Error> {
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

        // Gas payment
        let gas_payment = sui_sdk_types::GasPayment {
            objects: self
                .gas
                .iter()
                .map(ObjectInput::try_into_object_reference)
                .collect::<Result<Vec<_>, _>>()?,
            owner: self.sponsor.unwrap_or(sender),
            price,
            budget,
        };

        // Error out if there are any unresolved intents
        if !self.intents.is_empty() {
            return Err(Error::Input("unable to resolve intents offline".to_owned()));
        }

        //
        // Inputs
        //

        let mut unresolved_inputs = self.inputs.into_values().collect::<Vec<_>>();
        unresolved_inputs.sort_by_key(|(id, _input)| *id);

        let mut resolved_inputs = Vec::new();
        for (id, input) in unresolved_inputs {
            let arg = match input {
                InputArg::Gas => sui_sdk_types::Argument::Gas,
                InputArg::Pure(value) => {
                    resolved_inputs.push(sui_sdk_types::Input::Pure(value));
                    sui_sdk_types::Argument::Input(resolved_inputs.len() as u16 - 1)
                }
                InputArg::Object(object_input) => {
                    resolved_inputs.push(object_input.try_into_input()?);
                    sui_sdk_types::Argument::Input(resolved_inputs.len() as u16 - 1)
                }
            };

            *self.arguments.get_mut(&id).unwrap() = ResolvedArgument::Resolved(arg);
        }

        //
        // Commands
        //

        let mut resolved_commands = Vec::new();

        for (id, command) in self.commands {
            resolved_commands.push(
                command
                    .try_resolve(&self.arguments)
                    .map_err(|e| e.unwrap_err())?,
            );
            let arg = sui_sdk_types::Argument::Result(resolved_commands.len() as u16 - 1);

            *self.arguments.get_mut(&id).unwrap() = ResolvedArgument::Resolved(arg);
        }

        Ok(Transaction {
            kind: sui_sdk_types::TransactionKind::ProgrammableTransaction(
                sui_sdk_types::ProgrammableTransaction {
                    inputs: resolved_inputs,
                    commands: resolved_commands,
                },
            ),
            sender,
            gas_payment,
            expiration: self.expiration.unwrap_or(TransactionExpiration::None),
        })
    }

    pub async fn build(mut self, client: &mut sui_rpc::Client) -> Result<Transaction, Error> {
        use sui_rpc::field::FieldMask;
        use sui_rpc::field::FieldMaskUtil;
        use sui_rpc::proto::sui::rpc::v2::Input;
        use sui_rpc::proto::sui::rpc::v2::SimulateTransactionRequest;
        use sui_rpc::proto::sui::rpc::v2::SimulateTransactionResponse;
        use sui_rpc::proto::sui::rpc::v2::input::InputKind;

        let Some(sender) = self.sender else {
            return Err(Error::MissingSender);
        };

        let mut request = SimulateTransactionRequest::default()
            .with_read_mask(FieldMask::from_paths([
                SimulateTransactionResponse::path_builder()
                    .transaction()
                    .transaction()
                    .finish(),
                SimulateTransactionResponse::path_builder()
                    .transaction()
                    .effects()
                    .finish(),
            ]))
            .with_do_gas_selection(true);
        request.transaction_mut().set_sender(sender);

        //
        // Intents
        //

        // For now we'll be dumb and just run through the registered resolvers one by one and if we
        // still have intents left we'll bail

        let resolvers = std::mem::take(&mut self.resolvers);
        for resolver in resolvers.values() {
            resolver
                .resolve(&mut self, client)
                .await
                .map_err(|e| Error::Input(e.to_string()))?;
        }
        // Error out if there are any remaining unresolved intents
        if !self.intents.is_empty() {
            return Err(Error::Input("unable to resolve all intents".to_owned()));
        }

        //
        // Inputs
        //

        let mut unresolved_inputs = self.inputs.into_values().collect::<Vec<_>>();
        unresolved_inputs.sort_by_key(|(id, _input)| *id);

        let mut resolved_inputs = Vec::new();
        for (id, input) in unresolved_inputs {
            let arg = match input {
                InputArg::Gas => sui_sdk_types::Argument::Gas,
                InputArg::Pure(value) => {
                    resolved_inputs
                        .push(Input::default().with_kind(InputKind::Pure).with_pure(value));
                    sui_sdk_types::Argument::Input(resolved_inputs.len() as u16 - 1)
                }
                InputArg::Object(object_input) => {
                    resolved_inputs.push(object_input.to_input_proto());
                    sui_sdk_types::Argument::Input(resolved_inputs.len() as u16 - 1)
                }
            };

            *self.arguments.get_mut(&id).unwrap() = ResolvedArgument::Resolved(arg);
        }

        //
        // Commands
        //

        let mut resolved_commands = Vec::new();

        let mut stack = Vec::new();
        let mut to_resolve = self.commands.pop_first();
        while let Some((id, command)) = to_resolve.take() {
            let resolved = match command.try_resolve(&self.arguments) {
                Ok(resolved) => resolved,
                Err(Ok(next)) => {
                    // Push the current command on the stack
                    stack.push((id, command));
                    // set the next one to be processed
                    to_resolve = Some(
                        self.commands
                            .remove_entry(&next)
                            .expect("command must be there if it wasn't resolved yet"),
                    );
                    continue;
                }
                Err(Err(e)) => return Err(e),
            };

            resolved_commands.push(resolved);
            let arg = sui_sdk_types::Argument::Result(resolved_commands.len() as u16 - 1);
            *self.arguments.get_mut(&id).unwrap() = ResolvedArgument::Resolved(arg);

            // Pick the next command to resolve, either walked back down the stack or getting the
            // next in order
            if let Some(from_stack) = stack.pop() {
                to_resolve = Some(from_stack);
            } else {
                to_resolve = self.commands.pop_first();
            }
        }

        let t = request.transaction_mut();
        t.kind_mut()
            .programmable_transaction_mut()
            .set_inputs(resolved_inputs);
        t.kind_mut()
            .programmable_transaction_mut()
            .set_commands(resolved_commands.into_iter().map(Into::into).collect());

        // Gas payment
        {
            let payment = request.transaction_mut().gas_payment_mut();
            payment.set_owner(self.sponsor.unwrap_or(sender));

            if let Some(budget) = self.gas_budget {
                payment.set_budget(budget);
            }
            if let Some(price) = self.gas_price {
                payment.set_price(price);
            };
            payment.set_objects(
                self.gas
                    .iter()
                    .map(ObjectInput::try_into_object_reference_proto)
                    .collect::<Result<_, _>>()?,
            );
        }

        let response = client
            .execution_client()
            .simulate_transaction(request)
            .await
            .map_err(|e| Error::Input(format!("error simulating transaction: {e}")))?;

        if !response
            .get_ref()
            .transaction()
            .effects()
            .status()
            .success()
        {
            return Err(Error::Input(format!(
                "txn failed to execute: {}",
                response
                    .get_ref()
                    .transaction()
                    .effects()
                    .status()
                    .error()
                    .description()
            )));
        }

        response
            .get_ref()
            .transaction()
            .transaction()
            .bcs()
            .deserialize()
            .map_err(|e| Error::Input(e.to_string()))
    }

    pub(crate) fn register_resolver<R: IntentResolver>(&mut self, resolver: R) {
        self.resolvers
            .insert(resolver.type_id(), Box::new(resolver));
    }

    pub(crate) fn unresolved<T: std::any::Any + Send + Sync>(&mut self, unresolved: T) -> Argument {
        let id = self.arguments.len();
        self.arguments.insert(id, ResolvedArgument::Unresolved);
        self.intents.insert(id, Box::new(unresolved));
        Argument::new(id)
    }

    pub(crate) fn sender(&self) -> Option<Address> {
        self.sender
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Argument {
    id: usize,
    sub_index: Option<usize>,
}

impl Argument {
    pub(crate) fn new(id: usize) -> Self {
        Self {
            id,
            sub_index: None,
        }
    }

    pub fn to_nested(self, count: usize) -> Vec<Self> {
        (0..count)
            .map(|sub_index| Argument {
                sub_index: Some(sub_index),
                ..self
            })
            .collect()
    }

    fn try_resolve(
        self,
        resolved_arguments: &BTreeMap<usize, ResolvedArgument>,
    ) -> Result<sui_sdk_types::Argument, Result<usize, Error>> {
        let mut sub_index = self.sub_index;
        let arg = {
            let mut visited = BTreeSet::new();
            let mut next_id = self.id;

            loop {
                if visited.contains(&next_id) {
                    panic!("BUG: cyclic dependency");
                }
                visited.insert(next_id);

                match resolved_arguments.get(&next_id).unwrap() {
                    ResolvedArgument::Unresolved => return Err(Ok(next_id)),
                    ResolvedArgument::ReplaceWith(argument) => {
                        next_id = argument.id;
                        sub_index = argument.sub_index;
                    }
                    ResolvedArgument::Resolved(argument) => break argument,
                }
            }
        };

        if let Some(sub_index) = sub_index {
            if let Some(arg) = arg.nested(sub_index as u16) {
                return Ok(arg);
            } else {
                return Err(Err(Error::Input(
                    "unable to create nested argument".to_owned(),
                )));
            }
        }

        Ok(*arg)
    }

    fn try_resolve_many(
        arguments: &[Self],
        resolved_arguments: &BTreeMap<usize, ResolvedArgument>,
    ) -> Result<Vec<sui_sdk_types::Argument>, Result<usize, Error>> {
        arguments
            .iter()
            .map(|a| a.try_resolve(resolved_arguments))
            .collect::<Result<_, _>>()
    }
}

pub(crate) struct Command {
    kind: CommandKind,
    // A way to encode dependencies between commands when there aren't dependencies via explicit
    // input/outputs
    pub(crate) dependencies: Vec<Argument>,
}

impl From<CommandKind> for Command {
    fn from(value: CommandKind) -> Self {
        Self {
            kind: value,
            dependencies: Vec::new(),
        }
    }
}

pub(crate) enum CommandKind {
    /// A call to either an entry or a public Move function
    MoveCall(MoveCall),

    /// `(Vec<forall T:key+store. T>, address)`
    /// It sends n-objects to the specified address. These objects must have store
    /// (public transfer) and either the previous owner must be an address or the object must
    /// be newly created.
    TransferObjects(TransferObjects),

    /// `(&mut Coin<T>, Vec<u64>)` -> `Vec<Coin<T>>`
    /// It splits off some amounts into a new coins with those amounts
    SplitCoins(SplitCoins),

    /// `(&mut Coin<T>, Vec<Coin<T>>)`
    /// It merges n-coins into the first coin
    MergeCoins(MergeCoins),

    /// Publishes a Move package. It takes the package bytes and a list of the package's transitive
    /// dependencies to link against on-chain.
    Publish(Publish),

    /// `forall T: Vec<T> -> vector<T>`
    /// Given n-values of the same type, it constructs a vector. For non objects or an empty vector,
    /// the type tag must be specified.
    MakeMoveVector(MakeMoveVector),

    /// Upgrades a Move package
    /// Takes (in order):
    /// 1. A vector of serialized modules for the package.
    /// 2. A vector of object ids for the transitive dependencies of the new package.
    /// 3. The object ID of the package being upgraded.
    /// 4. An argument holding the `UpgradeTicket` that must have been produced from an earlier command in the same
    ///    programmable transaction.
    Upgrade(Upgrade),
}

impl Command {
    fn try_resolve(
        &self,
        resolved_arguments: &BTreeMap<usize, ResolvedArgument>,
    ) -> Result<sui_sdk_types::Command, Result<usize, Error>> {
        use sui_sdk_types::Command as C;

        // try to resolve all dependencies first
        Argument::try_resolve_many(&self.dependencies, resolved_arguments)?;

        let cmd = match &self.kind {
            CommandKind::MoveCall(MoveCall {
                package,
                module,
                function,
                type_arguments,
                arguments,
            }) => C::MoveCall(sui_sdk_types::MoveCall {
                package: *package,
                module: module.to_owned(),
                function: function.to_owned(),
                type_arguments: type_arguments.to_owned(),
                arguments: Argument::try_resolve_many(arguments, resolved_arguments)?,
            }),

            CommandKind::TransferObjects(TransferObjects { objects, address }) => {
                C::TransferObjects(sui_sdk_types::TransferObjects {
                    objects: Argument::try_resolve_many(objects, resolved_arguments)?,
                    address: address.try_resolve(resolved_arguments)?,
                })
            }

            CommandKind::SplitCoins(SplitCoins { coin, amounts }) => {
                C::SplitCoins(sui_sdk_types::SplitCoins {
                    coin: coin.try_resolve(resolved_arguments)?,
                    amounts: Argument::try_resolve_many(amounts, resolved_arguments)?,
                })
            }

            CommandKind::MergeCoins(MergeCoins {
                coin,
                coins_to_merge,
            }) => C::MergeCoins(sui_sdk_types::MergeCoins {
                coin: coin.try_resolve(resolved_arguments)?,
                coins_to_merge: Argument::try_resolve_many(coins_to_merge, resolved_arguments)?,
            }),

            CommandKind::Publish(Publish {
                modules,
                dependencies,
            }) => C::Publish(sui_sdk_types::Publish {
                modules: modules.to_owned(),
                dependencies: dependencies.to_owned(),
            }),

            CommandKind::MakeMoveVector(MakeMoveVector { type_, elements }) => {
                C::MakeMoveVector(sui_sdk_types::MakeMoveVector {
                    type_: type_.to_owned(),
                    elements: Argument::try_resolve_many(elements, resolved_arguments)?,
                })
            }

            CommandKind::Upgrade(Upgrade {
                modules,
                dependencies,
                package,
                ticket,
            }) => C::Upgrade(sui_sdk_types::Upgrade {
                modules: modules.to_owned(),
                dependencies: dependencies.to_owned(),
                package: *package,
                ticket: ticket.try_resolve(resolved_arguments)?,
            }),
        };
        Ok(cmd)
    }
}

pub(crate) struct TransferObjects {
    /// Set of objects to transfer
    pub objects: Vec<Argument>,

    /// The address to transfer ownership to
    pub address: Argument,
}

pub(crate) struct SplitCoins {
    /// The coin to split
    pub coin: Argument,

    /// The amounts to split off
    pub amounts: Vec<Argument>,
}

pub(crate) struct MergeCoins {
    /// Coin to merge coins into
    pub coin: Argument,

    /// Set of coins to merge into `coin`
    ///
    /// All listed coins must be of the same type and be the same type as `coin`
    pub coins_to_merge: Vec<Argument>,
}

pub(crate) struct Publish {
    /// The serialized move modules
    pub modules: Vec<Vec<u8>>,

    /// Set of packages that the to-be published package depends on
    pub dependencies: Vec<Address>,
}

pub(crate) struct MakeMoveVector {
    /// Type of the individual elements
    ///
    /// This is required to be set when the type can't be inferred, for example when the set of
    /// provided arguments are all pure input values.
    pub type_: Option<TypeTag>,

    /// The set individual elements to build the vector with
    pub elements: Vec<Argument>,
}

pub(crate) struct Upgrade {
    /// The serialized move modules
    pub modules: Vec<Vec<u8>>,

    /// Set of packages that the to-be published package depends on
    pub dependencies: Vec<Address>,

    /// Package id of the package to upgrade
    pub package: Address,

    /// Ticket authorizing the upgrade
    pub ticket: Argument,
}

pub(crate) struct MoveCall {
    /// The package containing the module and function.
    pub package: Address,

    /// The specific module in the package containing the function.
    pub module: Identifier,

    /// The function to be called.
    pub function: Identifier,

    /// The type arguments to the function.
    pub type_arguments: Vec<TypeTag>,

    /// The arguments to the function.
    pub arguments: Vec<Argument>,
    // Return value count??
}

pub struct ObjectInput {
    object_id: Address,
    kind: Option<ObjectKind>,
    version: Option<u64>,
    digest: Option<Digest>,
    mutable: Option<bool>,
}

#[derive(Clone, Copy)]
enum ObjectKind {
    Shared,
    Receiving,
    ImmutableOrOwned,
}

impl ObjectInput {
    pub fn new(object_id: Address) -> Self {
        Self {
            kind: None,
            object_id,
            version: None,
            digest: None,
            mutable: None,
        }
    }

    /// Return an owned kind of object with all required fields.
    pub fn owned(object_id: Address, version: u64, digest: Digest) -> Self {
        Self {
            kind: Some(ObjectKind::ImmutableOrOwned),
            object_id,
            version: Some(version),
            digest: Some(digest),
            mutable: None,
        }
    }

    /// Return an immutable kind of object with all required fields.
    pub fn immutable(object_id: Address, version: u64, digest: Digest) -> Self {
        Self {
            kind: Some(ObjectKind::ImmutableOrOwned),
            object_id,
            version: Some(version),
            digest: Some(digest),
            mutable: None,
        }
    }

    /// Return a receiving kind of object with all required fields.
    pub fn receiving(object_id: Address, version: u64, digest: Digest) -> Self {
        Self {
            kind: Some(ObjectKind::Receiving),
            object_id,
            version: Some(version),
            digest: Some(digest),
            mutable: None,
        }
    }

    /// Return a shared object.
    /// - `mutable` controls whether a command can accept the object by value or mutable reference.
    /// - `version` is the first version the object was shared at.
    pub fn shared(object_id: Address, version: u64, mutable: bool) -> Self {
        Self {
            kind: Some(ObjectKind::Shared),
            object_id,
            version: Some(version),
            mutable: Some(mutable),
            digest: None,
        }
    }

    /// Set the object kind to immutable.
    pub fn as_immutable(self) -> Self {
        Self {
            kind: Some(ObjectKind::ImmutableOrOwned),
            ..self
        }
    }

    /// Set the object kind to owned.
    pub fn as_owned(self) -> Self {
        Self {
            kind: Some(ObjectKind::ImmutableOrOwned),
            ..self
        }
    }

    /// Set the object kind to receiving.
    pub fn as_receiving(self) -> Self {
        Self {
            kind: Some(ObjectKind::Receiving),
            ..self
        }
    }

    /// Set the object kind to shared.
    pub fn as_shared(self) -> Self {
        Self {
            kind: Some(ObjectKind::Shared),
            ..self
        }
    }

    /// Set the specified version.
    pub fn with_version(self, version: u64) -> Self {
        Self {
            version: Some(version),
            ..self
        }
    }

    /// Set the specified digest.
    pub fn with_digest(self, digest: Digest) -> Self {
        Self {
            digest: Some(digest),
            ..self
        }
    }

    pub fn with_mutable(self, mutable: bool) -> Self {
        Self {
            mutable: Some(mutable),
            ..self
        }
    }
}

impl From<&sui_sdk_types::Object> for ObjectInput {
    fn from(object: &sui_sdk_types::Object) -> Self {
        let input = Self::new(object.object_id())
            .with_version(object.version())
            .with_digest(object.digest());

        match object.owner() {
            sui_sdk_types::Owner::Address(_) => input.as_owned(),
            sui_sdk_types::Owner::Object(_) => input,
            sui_sdk_types::Owner::Shared(version) => input.with_version(*version).as_shared(),
            sui_sdk_types::Owner::Immutable => input.as_immutable(),
            sui_sdk_types::Owner::ConsensusAddress { start_version, .. } => {
                input.with_version(*start_version).as_shared()
            }
            _ => input,
        }
    }
}

// impl TryFrom<&sui_rpc::proto::sui::rpc::v2::Object> for ObjectInput {
//     type Error = sui_rpc::proto::TryFromProtoError;

//     fn try_from(object: &sui_rpc::proto::sui::rpc::v2::Object) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

// private conversions
impl ObjectInput {
    fn try_into_object_reference(&self) -> Result<sui_sdk_types::ObjectReference, Error> {
        if matches!(self.kind, Some(ObjectKind::ImmutableOrOwned) | None)
            && let Some(version) = self.version
            && let Some(digest) = self.digest
        {
            Ok(sui_sdk_types::ObjectReference::new(
                self.object_id,
                version,
                digest,
            ))
        } else {
            Err(Error::WrongGasObject)
        }
    }

    fn try_into_input(&self) -> Result<sui_sdk_types::Input, Error> {
        let input = match self {
            // ImmutableOrOwned
            Self {
                object_id,
                kind: Some(ObjectKind::ImmutableOrOwned),
                version: Some(version),
                digest: Some(digest),
                ..
            }
            | Self {
                object_id,
                kind: None,
                version: Some(version),
                digest: Some(digest),
                mutable: None,
            } => sui_sdk_types::Input::ImmutableOrOwned(sui_sdk_types::ObjectReference::new(
                *object_id, *version, *digest,
            )),

            // Receiving
            Self {
                object_id,
                kind: Some(ObjectKind::Receiving),
                version: Some(version),
                digest: Some(digest),
                ..
            } => sui_sdk_types::Input::Receiving(sui_sdk_types::ObjectReference::new(
                *object_id, *version, *digest,
            )),

            // Shared
            Self {
                object_id,
                kind: Some(ObjectKind::Shared),
                version: Some(version),
                mutable: Some(mutable),
                ..
            }
            | Self {
                object_id,
                kind: None,
                version: Some(version),
                digest: None,
                mutable: Some(mutable),
            } => sui_sdk_types::Input::Shared(sui_sdk_types::SharedInput::new(
                *object_id, *version, *mutable,
            )),

            _ => {
                return Err(Error::Input(format!(
                    "Input object {} is incomplete",
                    self.object_id
                )));
            }
        };
        Ok(input)
    }

    fn to_input_proto(&self) -> sui_rpc::proto::sui::rpc::v2::Input {
        use sui_rpc::proto::sui::rpc::v2::input::InputKind;

        let mut input =
            sui_rpc::proto::sui::rpc::v2::Input::default().with_object_id(self.object_id);
        match self.kind {
            Some(ObjectKind::Shared) => input.set_kind(InputKind::Shared),
            Some(ObjectKind::Receiving) => input.set_kind(InputKind::Receiving),
            Some(ObjectKind::ImmutableOrOwned) => input.set_kind(InputKind::ImmutableOrOwned),
            None => {}
        }

        if let Some(version) = self.version {
            input.set_version(version);
        }

        if let Some(digest) = self.digest {
            input.set_digest(digest);
        }

        if let Some(mutable) = self.mutable {
            input.set_mutable(mutable);
        }

        input
    }

    fn try_into_object_reference_proto(
        &self,
    ) -> Result<sui_rpc::proto::sui::rpc::v2::ObjectReference, Error> {
        if !matches!(self.kind, Some(ObjectKind::ImmutableOrOwned) | None) {
            return Err(Error::WrongGasObject);
        }

        let mut input =
            sui_rpc::proto::sui::rpc::v2::ObjectReference::default().with_object_id(self.object_id);
        if let Some(version) = self.version {
            input.set_version(version);
        }
        if let Some(digest) = self.digest {
            input.set_digest(digest);
        }
        Ok(input)
    }

    pub(crate) fn try_from_object_proto(
        object: &sui_rpc::proto::sui::rpc::v2::Object,
    ) -> Result<Self, Error> {
        use sui_rpc::proto::sui::rpc::v2::owner::OwnerKind;

        let input = Self::new(
            object
                .object_id()
                .parse()
                .map_err(|_e| Error::MissingObjectId)?,
        );

        Ok(match object.owner().kind() {
            OwnerKind::Address | OwnerKind::Immutable => {
                input.as_owned().with_version(object.version()).with_digest(
                    object
                        .digest()
                        .parse()
                        .map_err(|_| Error::Input("can't parse digest".to_owned()))?,
                )
            }
            OwnerKind::Object => return Err(Error::Input("invalid object type".to_owned())),
            OwnerKind::Shared | OwnerKind::ConsensusAddress => input
                .as_shared()
                .with_version(object.owner().version())
                .with_mutable(true),
            OwnerKind::Unknown | _ => input,
        })
    }
}

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

impl Function {
    /// Constructor for the function type.
    pub fn new(package: Address, module: Identifier, function: Identifier) -> Self {
        Self {
            package,
            module,
            function,
            type_args: Vec::new(),
        }
    }

    pub fn with_type_args(self, type_args: Vec<TypeTag>) -> Self {
        Self { type_args, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_try_build() {
        let mut tx = TransactionBuilder::new();
        let _coin = tx.object(ObjectInput::owned(
            Address::from_static(
                "0x19406ea4d9609cd9422b85e6bf2486908f790b778c757aff805241f3f609f9b4",
            ),
            2,
            Digest::from_static("7opR9rFUYivSTqoJHvFb9p6p54THyHTatMG6id4JKZR9"),
        ));
        let _gas = tx.gas();

        let _recipient = tx.pure(&Address::from_static("0xabc"));

        assert!(tx.try_build().is_err());

        let mut tx = TransactionBuilder::new();
        let coin = tx.object(ObjectInput::owned(
            Address::from_static(
                "0x19406ea4d9609cd9422b85e6bf2486908f790b778c757aff805241f3f609f9b4",
            ),
            2,
            Digest::from_static("7opR9rFUYivSTqoJHvFb9p6p54THyHTatMG6id4JKZR9"),
        ));
        let gas = tx.gas();

        let recipient = tx.pure(&Address::from_static("0xabc"));
        tx.transfer_objects(vec![coin, gas], recipient);
        tx.set_gas_budget(500000000);
        tx.set_gas_price(1000);
        tx.add_gas_objects([ObjectInput::owned(
            Address::from_static(
                "0xd8792bce2743e002673752902c0e7348dfffd78638cb5367b0b85857bceb9821",
            ),
            2,
            Digest::from_static("2ZigdvsZn5BMeszscPQZq9z8ebnS2FpmAuRbAi9ednCk"),
        )]);
        tx.set_sender(Address::from_static(
            "0xc574ea804d9c1a27c886312e96c0e2c9cfd71923ebaeb3000d04b5e65fca2793",
        ));

        assert!(tx.try_build().is_ok());
    }

    #[test]
    fn test_split_transfer() {
        let mut tx = TransactionBuilder::new();

        // transfer 1 SUI from Gas coin
        let amount = tx.pure(&1_000_000_000u64);
        let gas = tx.gas();
        let result = tx.split_coins(gas, vec![amount; 5]);
        let recipient = tx.pure(&Address::from_static("0xabc"));
        tx.transfer_objects(result, recipient);

        tx.set_gas_budget(500000000);
        tx.set_gas_price(1000);
        tx.add_gas_objects([ObjectInput::owned(
            Address::from_static(
                "0xd8792bce2743e002673752902c0e7348dfffd78638cb5367b0b85857bceb9821",
            ),
            2,
            Digest::from_static("2ZigdvsZn5BMeszscPQZq9z8ebnS2FpmAuRbAi9ednCk"),
        )]);
        tx.set_sender(Address::from_static(
            "0xc574ea804d9c1a27c886312e96c0e2c9cfd71923ebaeb3000d04b5e65fca2793",
        ));

        assert!(tx.try_build().is_ok());
    }

    #[test]
    fn test_deterministic_building() {
        let build_tx = || {
            let mut tx = TransactionBuilder::new();
            let coin = tx.object(ObjectInput::owned(
                Address::from_static(
                    "0x19406ea4d9609cd9422b85e6bf2486908f790b778c757aff805241f3f609f9b4",
                ),
                2,
                Digest::from_static("7opR9rFUYivSTqoJHvFb9p6p54THyHTatMG6id4JKZR9"),
            ));
            let _ = tx.object(ObjectInput::owned(
                Address::from_static("0x12345"),
                2,
                Digest::from_static("7opR9rFUYivSTqoJHvFb9p6p54THyHTatMG6id4JKZR9"),
            ));
            let _ = tx.object(ObjectInput::owned(
                Address::from_static("0x12345"),
                2,
                Digest::from_static("7opR9rFUYivSTqoJHvFb9p6p54THyHTatMG6id4JKZR9"),
            ));
            let gas = tx.gas();
            let _ = tx.pure(&Address::from_static("0xabc"));
            let _ = tx.pure(&Address::from_static("0xabc"));
            let _ = tx.pure(&Address::from_static("0xabc"));
            let _ = tx.pure(&Address::from_static("0xdef"));
            let _ = tx.pure(&1u64);
            let _ = tx.pure(&1u64);
            let _ = tx.pure(&1u64);
            let _ = tx.pure(&Some(2u8));
            let _ = tx.pure_unique(&Address::from_static("0xabc"));
            let _ = tx.pure_unique(&Address::from_static("0xabc"));
            let _ = tx.pure_unique(&1u64);

            let recipient = tx.pure(&Address::from_static("0x123"));
            tx.transfer_objects(vec![coin, gas], recipient);
            tx.set_gas_budget(500000000);
            tx.set_gas_price(1000);
            tx.add_gas_objects([ObjectInput::owned(
                Address::from_static(
                    "0xd8792bce2743e002673752902c0e7348dfffd78638cb5367b0b85857bceb9821",
                ),
                2,
                Digest::from_static("2ZigdvsZn5BMeszscPQZq9z8ebnS2FpmAuRbAi9ednCk"),
            )]);
            tx.set_sender(Address::from_static(
                "0xc574ea804d9c1a27c886312e96c0e2c9cfd71923ebaeb3000d04b5e65fca2793",
            ));

            tx.try_build().unwrap()
        };

        let digest = build_tx().digest();

        assert!(
            (0..100)
                .map(|_| build_tx())
                .map(|tx| tx.digest())
                .all(|d| d == digest)
        )
    }
}
