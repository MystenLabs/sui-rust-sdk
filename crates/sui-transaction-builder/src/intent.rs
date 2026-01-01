#![allow(dead_code)]

use std::any::TypeId;
use std::collections::BTreeMap;
use tokio::sync::SetOnce;

use sui_rpc::Client;
use sui_types::Address;
use sui_types::Argument;
use sui_types::Command;
use sui_types::Digest;
use sui_types::StructTag;
use sui_types::TransactionExpiration;
use sui_types::TypeTag;

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait TransactionIntent: 'static {
    // fn into_box(self) -> Box<dyn TransactionIntent>
    // where
    //     Self: Sized,
    // {
    //     Box::new(self)
    // }

    fn register(self, builder: &mut TransactionBuilder) -> Result<ArgumentHandle, BoxError>;
}

// impl TransactionIntent for Box<dyn TransactionIntent> {
//     fn into_box(self) -> Box<dyn TransactionIntent> {
//         self
//     }

//     fn register(
//        self,
//        builder: &mut TransactionBuilder,
//     ) -> Result<ArgumentHandle, BoxError> {
//        (*self).register(builder)
//     }
// }

#[async_trait::async_trait]
trait IntentResolver {
    fn intent_type(&self) -> std::any::TypeId;

    async fn resolve(&self, intents: &[&dyn TransactionIntent]);
}

// impl<F> TransactionIntent for F
// where
//     F: Fn(&mut TransactionBuilder),
// {
//     fn resolve<'life0, 'life1, 'async_trait>(
//         &'life0 self,
//         builder: &'life1 mut TransactionBuilder,
//     ) -> ::core::pin::Pin<
//         Box<
//             dyn ::core::future::Future<Output = Result<Option<Argument>, BoxError>>
//                 + ::core::marker::Send
//                 + 'async_trait,
//         >,
//     >
//     where
//         'life0: 'async_trait,
//         'life1: 'async_trait,
//         Self: 'async_trait,
//     {
//         todo!()
//     }

//     // fn resolve(
//     //     &self,
//     //     builder: &mut TransactionBuilder,
//     // ) -> Result<Option<Argument>, BoxError> {
//     //     (*self).resolve(builder).await
//     // }
// }

/// A builder for creating transactions. Use `resolve` to finalize the transaction data.
struct Inner {
    /// The inputs to the transaction.
    // inputs: Vec<unresolved::Input>,

    /// The list of commands in the transaction. A command is a single operation in a programmable
    /// transaction.
    commands: Vec<Command>,

    /// The gas objects that will be used to pay for the transaction. The most common way is to
    /// use [`unresolved::Input::owned`] function to create a gas object and use the [`add_gas`]
    /// method to set the gas objects.
    // gas: Vec<unresolved::Input>,
    /// The gas budget for the transaction.
    gas_budget: Option<u64>,
    /// The gas price for the transaction.
    gas_price: Option<u64>,
    /// The sender of the transaction.
    sender: Option<Address>,
    /// The sponsor of the transaction. If None, the sender is also the sponsor.
    sponsor: Option<Address>,
    /// The expiration of the transaction. The default value of this type is no expiration.
    expiration: TransactionExpiration,

    client: Option<sui_rpc::Client>,

    intents: Vec<(Box<dyn TransactionIntent>, ArgumentHandle)>,
    resolvers: BTreeMap<TypeId, Box<dyn IntentResolver>>,
    objects: BTreeMap<Address, ObjectInput>,
    addresses: BTreeMap<AddressInput, Address>,
    pure_args: Vec<(Vec<u8>, ArgumentHandle)>,

    error: Option<BoxError>,
}

pub struct TransactionBuilder {
    inner: Box<Inner>,
}

pub struct ArgumentHandle {
    finalized: SetOnce<sui_types::Argument>,
    num_return_values: Option<usize>,
    sub_index: Option<usize>,
}

impl ArgumentHandle {}

enum Intent {}

struct ObjectInput {
    pub object_id: Address,
    pub version: Option<u64>,
    pub digest: Option<Digest>,
    pub mutable: Option<bool>,
    pub input_kind: Option<ObjectInputKind>,
}

enum ObjectInputKind {
    Shared,
    Receiving,
    ImmutableOrOwned,
}

enum AddressInput {
    Addr(Address),
    String(String),
}

#[async_trait::async_trait]
trait AddressResolver {
    async fn resolve(&self, unresolved_address: &str) -> Result<Option<Address>, BoxError>;
}

impl TransactionBuilder {
    pub fn sender(&self) -> Option<Address> {
        self.inner.sender
    }

    pub fn client(&self) -> Result<Client, BoxError> {
        self.inner
            .client
            .clone()
            .ok_or("No configured Client, unable to build transaction offline")
            .map_err(Into::into)
    }

    /// Merge a list of coins into a single coin, without producing any result.
    pub fn merge_coins(&mut self, coin: Argument, coins_to_merge: Vec<Argument>) {
        let cmd = Command::MergeCoins(sui_types::MergeCoins {
            coin,
            coins_to_merge,
        });
        self.inner.commands.push(cmd);
    }

    pub fn split_coins(&mut self, coin: Argument, amounts: impl Into<Vec<Argument>>) -> Argument {
        let cmd = Command::SplitCoins(sui_types::SplitCoins {
            coin,
            amounts: amounts.into(),
        });
        self.inner.commands.push(cmd);
        Argument::Result(self.inner.commands.len() as u16 - 1)
    }

    // pub fn add_command(&mut self, command:
    pub fn register_intent<I: TransactionIntent>(&mut self, _intent: I) -> ArgumentHandle {
        // self.in
        todo!()
    }
}

pub struct CoinWithBalance {
    coin_type: StructTag,
    balance: u64,
}

impl CoinWithBalance {
    pub fn new(coin_type: StructTag, balance: u64) -> Self {
        Self { coin_type, balance }
    }

    pub fn sui(balance: u64) -> Self {
        Self {
            coin_type: StructTag::sui(),
            balance,
        }
    }
}

// #[async_trait::async_trait]
// impl TransactionIntent for CoinWithBalance {
//     async fn resolve(
//         &self,
//         builder: &mut TransactionBuilder,
//     ) -> Result<Option<Argument>, BoxError> {
//         let sender = builder
//             .sender()
//             .ok_or("Sender must be set to resolve CoinWithBalance")?;
//         let client = builder.client()?;

//         let coins = client
//             .select_coins(&sender, &(self.coin_type.clone().into()), self.balance, &[])
//             .await?;

//         let arg = if let [first, rest @ ..] = coins
//             .into_iter()
//             .map(|coin| object_to_input(&coin).map(|coin| builder.add_input(coin)))
//             .collect::<Result<Vec<_>, _>>()?
//             .as_slice()
//         {
//             if !rest.is_empty() {
//                 builder.merge_coins(*first, rest.into());
//             }

//             let amount = [builder.add_input(pure(&self.balance))];
//             builder.split_coins(*first, amount)
//         } else {
//             return Err(
//                 format!("unable to find sufficient coins of type {}", self.coin_type).into(),
//             );
//         };

//         Ok(Some(arg))
//     }
// }

pub struct TransferObjects {
    /// Set of objects to transfer
    pub objects: Vec<Argument>,

    /// The address to transfer ownership to
    pub address: Argument,
}

pub struct SplitCoins {
    /// The coin to split
    pub coin: Argument,

    /// The amounts to split off
    pub amounts: Vec<Argument>,
}

pub struct MergeCoins {
    /// Coin to merge coins into
    pub coin: Argument,

    /// Set of coins to merge into `coin`
    ///
    /// All listed coins must be of the same type and be the same type as `coin`
    pub coins_to_merge: Vec<Argument>,
}

pub struct Publish {
    /// The serialized move modules
    pub modules: Vec<Vec<u8>>,

    /// Set of packages that the to-be published package depends on
    pub dependencies: Vec<Address>,
}

pub struct MakeMoveVector {
    /// Type of the individual elements
    ///
    /// This is required to be set when the type can't be inferred, for example when the set of
    /// provided arguments are all pure input values.
    pub type_: Option<TypeTag>,

    /// The set individual elements to build the vector with
    pub elements: Vec<Argument>,
}

pub struct Upgrade {
    /// The serialized move modules
    pub modules: Vec<Vec<u8>>,

    /// Set of packages that the to-be published package depends on
    pub dependencies: Vec<Address>,

    /// Package id of the package to upgrade
    pub package: Address,

    /// Ticket authorizing the upgrade
    pub ticket: Argument,
}

pub struct MoveCall {
    /// The package containing the module and function.
    package: Address,

    /// The specific module in the package containing the function.
    module: String,

    /// The function to be called.
    function: String,

    /// The type arguments to the function.
    type_arguments: Vec<TypeTag>,

    /// The arguments to the function.
    arguments: Vec<Box<dyn TransactionIntent>>,
    // Return value count??
}

impl MoveCall {
    pub fn new(package: Address, module: &str, function: &str) -> Self {
        Self {
            package,
            module: module.to_owned(),
            function: function.to_owned(),
            type_arguments: vec![],
            arguments: vec![],
        }
    }

    pub fn type_args<I, T>(mut self, type_args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<TypeTag>,
    {
        for type_arg in type_args {
            self.type_arguments.push(type_arg.as_ref().to_owned());
        }
        self
    }

    pub fn arg<I: TransactionIntent>(mut self, arg: I) -> Self {
        self.arguments.push(Box::new(arg));
        self
    }

    pub fn args<Iter, Intent>(mut self, args: Iter) -> Self
    where
        Iter: IntoIterator<Item = Intent>,
        Intent: TransactionIntent,
    {
        for arg in args {
            self.arguments.push(Box::new(arg));
        }
        self
    }
}

pub struct MoveCall2 {
    /// The package containing the module and function.
    package: Address,

    /// The specific module in the package containing the function.
    module: String,

    /// The function to be called.
    function: String,

    /// The type arguments to the function.
    type_arguments: Vec<TypeTag>,

    /// The arguments to the function.
    arguments: Vec<ArgumentHandle>,
    // Return value count??
}
