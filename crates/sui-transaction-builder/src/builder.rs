use indexmap::IndexMap;
use sui_types::Address;
use sui_types::Digest;
use sui_types::Identifier;
use sui_types::Transaction;
use sui_types::TransactionExpiration;
use sui_types::TypeTag;

use crate::error::Error;

// pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// A builder for creating transactions. Use `resolve` to finalize the transaction data.
pub struct TransactionBuilder {
    arguments: IndexMap<ArgKind, Arg>,

    /// The gas objects that will be used to pay for the transaction. The most common way is to
    /// use [`unresolved::Input::owned`] function to create a gas object and use the [`add_gas`]
    /// method to set the gas objects.
    gas: Vec<ObjectInput>,
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
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ArgKind {
    Gas,
    ObjectInput(Address),
    PureInput(Vec<u8>),
    UniquePureInput(usize),
    CommandResult(usize),
}

enum Arg {
    Gas,
    Pure(Vec<u8>),
    Object(ObjectInput),
    Command(Command),
}

impl TransactionBuilder {
    /// Create a new transaction builder and initialize its elements to default.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            gas_budget: Default::default(),
            gas_price: Default::default(),
            sender: Default::default(),
            sponsor: Default::default(),
            expiration: Default::default(),
            arguments: Default::default(),
            gas: Default::default(),
        }
    }

    // Transaction Inputs

    pub fn gas(&mut self) -> Argument {
        let (index, _) = self.arguments.insert_full(ArgKind::Gas, Arg::Gas);
        Argument::new(index)
    }

    pub fn pure_bytes(&mut self, bytes: Vec<u8>) -> Argument {
        let (index, _) = self
            .arguments
            .insert_full(ArgKind::PureInput(bytes.clone()), Arg::Pure(bytes));
        Argument::new(index)
    }

    pub fn pure<T: serde::Serialize>(&mut self, value: &T) -> Argument {
        let bytes = bcs::to_bytes(value).expect("bcs serialization failed");
        self.pure_bytes(bytes)
    }

    pub fn pure_bytes_unique(&mut self, bytes: Vec<u8>) -> Argument {
        let (index, _) = self.arguments.insert_full(
            ArgKind::UniquePureInput(self.arguments.len()),
            Arg::Pure(bytes),
        );
        Argument::new(index)
    }

    pub fn pure_unique<T: serde::Serialize>(&mut self, value: &T) -> Argument {
        let bytes = bcs::to_bytes(value).expect("bcs serialization failed");
        self.pure_bytes_unique(bytes)
    }

    pub fn object(&mut self, object: ObjectInput) -> Argument {
        let id = object.object_id;

        let entry = self.arguments.entry(ArgKind::ObjectInput(id));
        let index = entry.index();
        match entry {
            indexmap::map::Entry::Occupied(mut o) => {
                let Arg::Object(object2) = o.get_mut() else {
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
            }
            indexmap::map::Entry::Vacant(v) => {
                v.insert(Arg::Object(object));
            }
        }

        Argument::new(index)
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
        let idx = self.arguments.len();
        let (index, maybe_old_value) = self
            .arguments
            .insert_full(ArgKind::CommandResult(idx), Arg::Command(command));
        assert_eq!(idx, index);
        assert!(maybe_old_value.is_none());

        Argument::new(index)
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
        let cmd = Command::MoveCall(MoveCall {
            package: function.package,
            module: function.module,
            function: function.function,
            type_arguments: function.type_args,
            arguments,
        });
        self.command(cmd)
    }

    /// Transfer a list of objects to the given address, without producing any result.
    pub fn transfer_objects(&mut self, objects: Vec<Argument>, address: Argument) {
        let cmd = Command::TransferObjects(TransferObjects { objects, address });
        self.command(cmd);
    }

    /// Split a coin by the provided amounts, returning multiple results (as many as there are
    /// amounts). The returned vector of `Arguments` is guaranteed to be the same length as the
    /// provided `amounts` vector.
    pub fn split_coins(&mut self, coin: Argument, amounts: Vec<Argument>) -> Vec<Argument> {
        let amounts_len = amounts.len();
        let cmd = Command::SplitCoins(SplitCoins { coin, amounts });
        self.command(cmd).to_nested(amounts_len)
    }

    /// Merge a list of coins into a single coin, without producing any result.
    pub fn merge_coins(&mut self, coin: Argument, coins_to_merge: Vec<Argument>) {
        let cmd = Command::MergeCoins(MergeCoins {
            coin,
            coins_to_merge,
        });
        self.command(cmd);
    }

    /// Make a move vector from a list of elements. If the elements are not objects, or the vector
    /// is empty, a type must be supplied.
    /// It returns the Move vector as an argument, that can be used in subsequent commands.
    pub fn make_move_vec(&mut self, type_: Option<TypeTag>, elements: Vec<Argument>) -> Argument {
        let cmd = Command::MakeMoveVector(MakeMoveVector { type_, elements });
        self.command(cmd)
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
        let cmd = Command::Publish(Publish {
            modules,
            dependencies,
        });
        self.command(cmd)
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
        let cmd = Command::Upgrade(Upgrade {
            modules,
            dependencies,
            package,
            ticket,
        });
        self.command(cmd)
    }

    /// Assuming everything is resolved, convert this transaction into the
    /// resolved form. Returns a [`Transaction`] if successful, or an `Error` if not.
    pub fn try_build(&self) -> Result<Transaction, Error> {
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
        let gas_payment = sui_types::GasPayment {
            objects: self
                .gas
                .iter()
                .map(ObjectInput::try_into_object_reference)
                .collect::<Result<Vec<_>, _>>()?,
            owner: self.sponsor.unwrap_or(sender),
            price,
            budget,
        };

        let mut resolved_arguments = Vec::new();
        let mut resolved_inputs = Vec::new();
        let mut resolved_commands = Vec::new();

        for (id, arg) in self.arguments.values().enumerate() {
            assert!(id == resolved_arguments.len());

            let a = match arg {
                Arg::Gas => sui_types::Argument::Gas,
                Arg::Pure(value) => {
                    resolved_inputs.push(sui_types::Input::Pure(value.clone()));
                    sui_types::Argument::Input(resolved_inputs.len() as u16 - 1)
                }
                Arg::Object(object_input) => {
                    resolved_inputs.push(object_input.try_into_input()?);
                    sui_types::Argument::Input(resolved_inputs.len() as u16 - 1)
                }
                Arg::Command(command) => {
                    resolved_commands.push(command.try_resolve(&resolved_arguments)?);
                    sui_types::Argument::Result(resolved_commands.len() as u16 - 1)
                }
            };

            resolved_arguments.push(a);
        }

        Ok(Transaction {
            kind: sui_types::TransactionKind::ProgrammableTransaction(
                sui_types::ProgrammableTransaction {
                    inputs: resolved_inputs,
                    commands: resolved_commands,
                },
            ),
            sender,
            gas_payment,
            expiration: self.expiration.unwrap_or(TransactionExpiration::None),
        })
    }

    pub async fn build(&self, client: &mut sui_rpc::Client) -> Result<Transaction, Error> {
        use sui_rpc::field::FieldMask;
        use sui_rpc::field::FieldMaskUtil;
        use sui_rpc::proto::sui::rpc::v2::Input;
        use sui_rpc::proto::sui::rpc::v2::SimulateTransactionRequest;
        use sui_rpc::proto::sui::rpc::v2::SimulateTransactionResponse;
        use sui_rpc::proto::sui::rpc::v2::input::InputKind;

        let Some(sender) = self.sender else {
            return Err(Error::MissingSender);
        };

        let mut request =
            SimulateTransactionRequest::default().with_read_mask(FieldMask::from_paths([
                SimulateTransactionResponse::path_builder()
                    .transaction()
                    .transaction()
                    .finish(),
                SimulateTransactionResponse::path_builder()
                    .transaction()
                    .effects()
                    .finish(),
            ]));
        let t = request.transaction_mut();

        t.set_sender(sender);

        // Gas payment
        {
            let payment = t.gas_payment_mut();
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

        let mut resolved_arguments = Vec::new();
        let mut resolved_inputs = Vec::new();
        let mut resolved_commands = Vec::new();

        for (id, arg) in self.arguments.values().enumerate() {
            assert!(id == resolved_arguments.len());

            let a = match arg {
                Arg::Gas => sui_types::Argument::Gas,
                Arg::Pure(value) => {
                    resolved_inputs.push(
                        Input::default()
                            .with_kind(InputKind::Pure)
                            .with_pure(value.clone()),
                    );
                    sui_types::Argument::Input(resolved_inputs.len() as u16 - 1)
                }
                Arg::Object(object_input) => {
                    resolved_inputs.push(object_input.to_input_proto());
                    sui_types::Argument::Input(resolved_inputs.len() as u16 - 1)
                }
                Arg::Command(command) => {
                    resolved_commands.push(command.try_resolve(&resolved_arguments)?);
                    sui_types::Argument::Result(resolved_commands.len() as u16 - 1)
                }
            };

            resolved_arguments.push(a);
        }

        t.kind_mut()
            .programmable_transaction_mut()
            .set_inputs(resolved_inputs);
        t.kind_mut()
            .programmable_transaction_mut()
            .set_commands(resolved_commands.into_iter().map(Into::into).collect());

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
}

#[derive(Clone, Copy)]
pub struct Argument {
    id: usize,
    sub_index: Option<usize>,
}

impl Argument {
    fn new(id: usize) -> Self {
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
        resolved_arguments: &[sui_types::Argument],
    ) -> Result<sui_types::Argument, Error> {
        let arg = resolved_arguments[self.id];

        if let Some(sub_index) = self.sub_index {
            if let Some(arg) = arg.nested(sub_index as u16) {
                return Ok(arg);
            } else {
                return Err(Error::Input("unable to create nested argument".to_owned()));
            }
        }

        Ok(arg)
    }

    fn try_resolve_many(
        arguments: &[Self],
        resolved_arguments: &[sui_types::Argument],
    ) -> Result<Vec<sui_types::Argument>, Error> {
        arguments
            .iter()
            .map(|a| a.try_resolve(resolved_arguments))
            .collect::<Result<_, _>>()
    }
}

enum Command {
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
        resolved_arguments: &[sui_types::Argument],
    ) -> Result<sui_types::Command, Error> {
        use sui_types::Command as C;

        let cmd = match self {
            Command::MoveCall(MoveCall {
                package,
                module,
                function,
                type_arguments,
                arguments,
            }) => C::MoveCall(sui_types::MoveCall {
                package: *package,
                module: module.to_owned(),
                function: function.to_owned(),
                type_arguments: type_arguments.to_owned(),
                arguments: Argument::try_resolve_many(arguments, resolved_arguments)?,
            }),

            Command::TransferObjects(TransferObjects { objects, address }) => {
                C::TransferObjects(sui_types::TransferObjects {
                    objects: Argument::try_resolve_many(objects, resolved_arguments)?,
                    address: address.try_resolve(resolved_arguments)?,
                })
            }

            Command::SplitCoins(SplitCoins { coin, amounts }) => {
                C::SplitCoins(sui_types::SplitCoins {
                    coin: coin.try_resolve(resolved_arguments)?,
                    amounts: Argument::try_resolve_many(amounts, resolved_arguments)?,
                })
            }

            Command::MergeCoins(MergeCoins {
                coin,
                coins_to_merge,
            }) => C::MergeCoins(sui_types::MergeCoins {
                coin: coin.try_resolve(resolved_arguments)?,
                coins_to_merge: Argument::try_resolve_many(coins_to_merge, resolved_arguments)?,
            }),

            Command::Publish(Publish {
                modules,
                dependencies,
            }) => C::Publish(sui_types::Publish {
                modules: modules.to_owned(),
                dependencies: dependencies.to_owned(),
            }),

            Command::MakeMoveVector(MakeMoveVector { type_, elements }) => {
                C::MakeMoveVector(sui_types::MakeMoveVector {
                    type_: type_.to_owned(),
                    elements: Argument::try_resolve_many(elements, resolved_arguments)?,
                })
            }

            Command::Upgrade(Upgrade {
                modules,
                dependencies,
                package,
                ticket,
            }) => C::Upgrade(sui_types::Upgrade {
                modules: modules.to_owned(),
                dependencies: dependencies.to_owned(),
                package: *package,
                ticket: ticket.try_resolve(resolved_arguments)?,
            }),
        };
        Ok(cmd)
    }
}

struct TransferObjects {
    /// Set of objects to transfer
    pub objects: Vec<Argument>,

    /// The address to transfer ownership to
    pub address: Argument,
}

struct SplitCoins {
    /// The coin to split
    pub coin: Argument,

    /// The amounts to split off
    pub amounts: Vec<Argument>,
}

struct MergeCoins {
    /// Coin to merge coins into
    pub coin: Argument,

    /// Set of coins to merge into `coin`
    ///
    /// All listed coins must be of the same type and be the same type as `coin`
    pub coins_to_merge: Vec<Argument>,
}

struct Publish {
    /// The serialized move modules
    pub modules: Vec<Vec<u8>>,

    /// Set of packages that the to-be published package depends on
    pub dependencies: Vec<Address>,
}

struct MakeMoveVector {
    /// Type of the individual elements
    ///
    /// This is required to be set when the type can't be inferred, for example when the set of
    /// provided arguments are all pure input values.
    pub type_: Option<TypeTag>,

    /// The set individual elements to build the vector with
    pub elements: Vec<Argument>,
}

struct Upgrade {
    /// The serialized move modules
    pub modules: Vec<Vec<u8>>,

    /// Set of packages that the to-be published package depends on
    pub dependencies: Vec<Address>,

    /// Package id of the package to upgrade
    pub package: Address,

    /// Ticket authorizing the upgrade
    pub ticket: Argument,
}

struct MoveCall {
    /// The package containing the module and function.
    package: Address,

    /// The specific module in the package containing the function.
    module: Identifier,

    /// The function to be called.
    function: Identifier,

    /// The type arguments to the function.
    type_arguments: Vec<TypeTag>,

    /// The arguments to the function.
    arguments: Vec<Argument>,
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

impl From<&sui_types::Object> for ObjectInput {
    fn from(object: &sui_types::Object) -> Self {
        let input = Self::new(object.object_id())
            .with_version(object.version())
            .with_digest(object.digest());

        match object.owner() {
            sui_types::Owner::Address(_) => input.as_owned(),
            sui_types::Owner::Object(_) => input,
            sui_types::Owner::Shared(version) => input.with_version(*version).as_shared(),
            sui_types::Owner::Immutable => input.as_immutable(),
            sui_types::Owner::ConsensusAddress { start_version, .. } => {
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
    fn try_into_object_reference(&self) -> Result<sui_types::ObjectReference, Error> {
        if matches!(self.kind, Some(ObjectKind::ImmutableOrOwned) | None)
            && let Some(version) = self.version
            && let Some(digest) = self.digest
        {
            Ok(sui_types::ObjectReference::new(
                self.object_id,
                version,
                digest,
            ))
        } else {
            Err(Error::WrongGasObject)
        }
    }

    fn try_into_input(&self) -> Result<sui_types::Input, Error> {
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
            } => sui_types::Input::ImmutableOrOwned(sui_types::ObjectReference::new(
                *object_id, *version, *digest,
            )),

            // Receiving
            Self {
                object_id,
                kind: Some(ObjectKind::Receiving),
                version: Some(version),
                digest: Some(digest),
                ..
            } => sui_types::Input::Receiving(sui_types::ObjectReference::new(
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
            } => sui_types::Input::Shared(sui_types::SharedInput::new(
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
        let coin = tx.object(ObjectInput::owned(
            Address::from_static(
                "0x19406ea4d9609cd9422b85e6bf2486908f790b778c757aff805241f3f609f9b4",
            ),
            2,
            Digest::from_static("7opR9rFUYivSTqoJHvFb9p6p54THyHTatMG6id4JKZR9"),
        ));
        let gas = tx.gas();

        let recipient = tx.pure(&Address::from_static("0xabc"));

        assert!(tx.try_build().is_err());

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
}
