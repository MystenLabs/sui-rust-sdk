//! High-level transaction intents.
//!
//! Intents describe *what* a transaction should accomplish without
//! requiring the caller to manually manage coin selection or object
//! resolution. When the transaction is built with
//! [`TransactionBuilder::build`](crate::TransactionBuilder::build),
//! registered resolvers communicate with the network to fill in
//! the details.
//!
//! # Built-in intents
//!
//! Currently the only built-in intent is [`CoinWithBalance`], which
//! requests a coin of a given type and amount.
//!
//! # Custom intents
//!
//! Implement [`Intent`] and [`IntentResolver`] to define custom
//! intents that third-party crates can use:
//!
//! ```
//! use sui_transaction_builder::intent::Intent;
//! use sui_transaction_builder::intent::IntentResolver;
//! use sui_transaction_builder::intent::ResolveContext;
//! use sui_transaction_builder::intent::BoxError;
//!
//! pub struct MyIntent {
//!     pub amount: u64,
//! }
//!
//! #[derive(Debug, Default)]
//! pub struct MyResolver;
//!
//! impl Intent for MyIntent {
//!     type Resolver = MyResolver;
//! }
//!
//! #[async_trait::async_trait]
//! impl IntentResolver for MyResolver {
//!     async fn resolve(
//!         &self,
//!         ctx: &mut ResolveContext<'_>,
//!         client: &mut sui_rpc::Client,
//!     ) -> Result<(), BoxError> {
//!         for (id, intent) in ctx.take_intents::<MyIntent>() {
//!             let amt = ctx.pure(&intent.amount);
//!             let gas = ctx.gas();
//!             let coins = ctx.split_coins(gas, vec![amt]);
//!             ctx.redirect_argument(id, coins[0]);
//!         }
//!         Ok(())
//!     }
//! }
//! ```

use std::collections::HashSet;

use crate::Argument;
use crate::Function;
use crate::ObjectInput;
use crate::TransactionBuilder;
use crate::builder::ResolvedArgument;
use sui_sdk_types::Address;
use sui_sdk_types::TypeTag;

mod coin_with_balance;
pub use coin_with_balance::CoinWithBalance;
pub use coin_with_balance::CoinWithBalanceResolver;

/// Maximum number of gas objects that can be used in a transaction.
pub const MAX_GAS_OBJECTS: usize = 250; // 256

/// Maximum number of commands in a single transaction.
#[allow(unused)]
pub const MAX_COMMANDS: usize = 1000; // 1024

/// Maximum number of input objects in a single transaction.
#[allow(unused)]
pub const MAX_INPUT_OBJECTS: usize = 2000; // 2048

/// Maximum number of arguments to a single command.
pub const MAX_ARGUMENTS: usize = 500; // 512

/// Error type for intent resolution.
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Opaque handle identifying a registered intent instance.
///
/// Returned by [`ResolveContext::take_intents`] and passed to
/// [`ResolveContext::redirect_argument`] to connect the resolved
/// value back to the original placeholder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntentId(usize);

/// A high-level transaction intent.
///
/// Intents describe *what* a transaction needs without specifying
/// *how* to achieve it. Each intent type has a corresponding
/// [`IntentResolver`] that turns the intent into concrete
/// transaction inputs and commands.
///
/// # Implementing a custom intent
///
/// ```
/// use sui_transaction_builder::intent::Intent;
/// use sui_transaction_builder::intent::IntentResolver;
/// use sui_transaction_builder::intent::ResolveContext;
/// use sui_transaction_builder::intent::BoxError;
///
/// pub struct MyIntent { pub value: u64 }
///
/// #[derive(Debug, Default)]
/// pub struct MyResolver;
///
/// impl Intent for MyIntent {
///     type Resolver = MyResolver;
/// }
///
/// #[async_trait::async_trait]
/// impl IntentResolver for MyResolver {
///     async fn resolve(
///         &self,
///         ctx: &mut ResolveContext<'_>,
///         _client: &mut sui_rpc::Client,
///     ) -> Result<(), BoxError> {
///         for (id, intent) in ctx.take_intents::<MyIntent>() {
///             let amt = ctx.pure(&intent.value);
///             let gas = ctx.gas();
///             let coins = ctx.split_coins(gas, vec![amt]);
///             ctx.redirect_argument(id, coins[0]);
///         }
///         Ok(())
///     }
/// }
/// ```
pub trait Intent: std::any::Any + Send + Sync {
    /// The resolver type that handles all instances of this intent.
    ///
    /// A single resolver instance is registered per intent type and
    /// handles all instances of that type in one batch.
    type Resolver: IntentResolver + Default;
}

/// Resolves registered intents into concrete transaction commands.
///
/// Each resolver type handles all instances of one [`Intent`] type.
/// During [`TransactionBuilder::build`](crate::TransactionBuilder::build),
/// the builder calls [`resolve`](Self::resolve) once per resolver,
/// passing a [`ResolveContext`] that provides access to the subset
/// of builder operations needed for resolution.
#[async_trait::async_trait]
pub trait IntentResolver: std::any::Any + std::fmt::Debug + Send + Sync {
    /// Resolve all intents that this resolver is responsible for.
    ///
    /// Use [`ResolveContext::take_intents`] to extract intent
    /// instances, then use the context's builder methods to add
    /// commands and redirect arguments via
    /// [`ResolveContext::redirect_argument`].
    async fn resolve(
        &self,
        ctx: &mut ResolveContext<'_>,
        client: &mut sui_rpc::Client,
    ) -> Result<(), BoxError>;
}

/// A restricted view of [`TransactionBuilder`] available during
/// intent resolution.
///
/// This type exposes only the operations that resolvers need:
/// extracting intents, building commands, querying transaction
/// metadata, and redirecting arguments. It intentionally does
/// **not** expose configuration or finalization methods like
/// `set_sender`, `set_gas_budget`, `try_build`, or `build`.
pub struct ResolveContext<'a> {
    builder: &'a mut TransactionBuilder,
}

impl<'a> ResolveContext<'a> {
    pub(crate) fn new(builder: &'a mut TransactionBuilder) -> Self {
        Self { builder }
    }

    // -- Intent extraction --------------------------------------------------

    /// Extract all registered intents of type `I`, removing them
    /// from the builder.
    ///
    /// This is the primary way resolvers obtain the intent instances
    /// they are responsible for. Each returned pair contains an
    /// [`IntentId`] (used later with [`redirect_argument`](Self::redirect_argument))
    /// and the intent value itself.
    pub fn take_intents<I: Intent>(&mut self) -> Vec<(IntentId, I)> {
        self.builder
            .intents
            .extract_if(.., |_id, intent| intent.downcast_ref::<I>().is_some())
            .map(|(id, boxed)| {
                let intent = *boxed.downcast::<I>().expect("type was checked above");
                (IntentId(id), intent)
            })
            .collect()
    }

    // -- Argument redirection -----------------------------------------------

    /// Redirect the argument for `intent_id` to point at
    /// `replacement`.
    ///
    /// After this call, any command that references the intent's
    /// original argument will instead use `replacement` when the
    /// transaction is finalized.
    pub fn redirect_argument(&mut self, intent_id: IntentId, replacement: Argument) {
        *self
            .builder
            .arguments
            .get_mut(&intent_id.0)
            .expect("intent id must be valid") = ResolvedArgument::ReplaceWith(replacement);
    }

    // -- Query methods ------------------------------------------------------

    /// Returns the transaction sender, if set.
    pub fn sender(&self) -> Option<Address> {
        self.builder.sender()
    }

    /// Returns the set of object IDs already used in the
    /// transaction (gas objects and input objects).
    ///
    /// Useful for avoiding duplicate object selection during coin
    /// resolution.
    pub fn used_object_ids(&self) -> HashSet<Address> {
        self.builder.used_object_ids()
    }

    /// Returns the number of gas objects currently registered.
    pub fn gas_object_count(&self) -> usize {
        self.builder.gas.len()
    }

    // -- Command builders (delegated) ---------------------------------------

    /// Add a Move call command.
    ///
    /// See [`TransactionBuilder::move_call`](crate::TransactionBuilder::move_call).
    pub fn move_call(&mut self, function: Function, arguments: Vec<Argument>) -> Argument {
        self.builder.move_call(function, arguments)
    }

    /// Split a coin into multiple coins by amount.
    ///
    /// See [`TransactionBuilder::split_coins`](crate::TransactionBuilder::split_coins).
    pub fn split_coins(&mut self, coin: Argument, amounts: Vec<Argument>) -> Vec<Argument> {
        self.builder.split_coins(coin, amounts)
    }

    /// Merge coins into a single coin.
    ///
    /// See [`TransactionBuilder::merge_coins`](crate::TransactionBuilder::merge_coins).
    pub fn merge_coins(&mut self, coin: Argument, coins_to_merge: Vec<Argument>) {
        self.builder.merge_coins(coin, coins_to_merge)
    }

    /// Transfer objects to a recipient.
    ///
    /// See [`TransactionBuilder::transfer_objects`](crate::TransactionBuilder::transfer_objects).
    pub fn transfer_objects(&mut self, objects: Vec<Argument>, recipient: Argument) {
        self.builder.transfer_objects(objects, recipient)
    }

    /// Create a Move vector from elements.
    ///
    /// See [`TransactionBuilder::make_move_vec`](crate::TransactionBuilder::make_move_vec).
    pub fn make_move_vec(
        &mut self,
        type_tag: Option<TypeTag>,
        elements: Vec<Argument>,
    ) -> Argument {
        self.builder.make_move_vec(type_tag, elements)
    }

    /// Add a pure input value.
    ///
    /// See [`TransactionBuilder::pure`](crate::TransactionBuilder::pure).
    pub fn pure<T: serde::Serialize>(&mut self, value: &T) -> Argument {
        self.builder.pure(value)
    }

    /// Add an object input.
    ///
    /// See [`TransactionBuilder::object`](crate::TransactionBuilder::object).
    pub fn object(&mut self, object: ObjectInput) -> Argument {
        self.builder.object(object)
    }

    /// Return the gas coin argument.
    ///
    /// See [`TransactionBuilder::gas`](crate::TransactionBuilder::gas).
    pub fn gas(&mut self) -> Argument {
        self.builder.gas()
    }

    /// Withdraw funds from address balance as a `Coin<T>`.
    ///
    /// See [`TransactionBuilder::funds_withdrawal_coin`](crate::TransactionBuilder::funds_withdrawal_coin).
    pub fn funds_withdrawal_coin(&mut self, coin_type: TypeTag, amount: u64) -> Argument {
        self.builder.funds_withdrawal_coin(coin_type, amount)
    }

    /// Withdraw funds from address balance as a `Balance<T>`.
    ///
    /// See [`TransactionBuilder::funds_withdrawal_balance`](crate::TransactionBuilder::funds_withdrawal_balance).
    pub fn funds_withdrawal_balance(&mut self, coin_type: TypeTag, amount: u64) -> Argument {
        self.builder.funds_withdrawal_balance(coin_type, amount)
    }

    /// Add gas objects to the transaction.
    ///
    /// See [`TransactionBuilder::add_gas_objects`](crate::TransactionBuilder::add_gas_objects).
    pub fn add_gas_objects<O, I>(&mut self, gas: I)
    where
        O: Into<ObjectInput>,
        I: IntoIterator<Item = O>,
    {
        self.builder.add_gas_objects(gas)
    }

    // -- Command dependency management --------------------------------------

    /// Returns an [`Argument`] referencing the last command added.
    ///
    /// Useful for building dependency chains between commands that
    /// don't have explicit input/output relationships.
    ///
    /// # Panics
    ///
    /// Panics if no commands have been added.
    pub fn last_command_argument(&self) -> Argument {
        Argument::new(*self.builder.commands.last_key_value().unwrap().0)
    }

    /// Add dependency arguments to the last command.
    ///
    /// Dependencies ensure that the referenced commands execute
    /// before the last command, even when there is no explicit data
    /// dependency.
    pub fn add_dependencies_to_last_command(&mut self, deps: Vec<Argument>) {
        self.builder
            .commands
            .last_entry()
            .unwrap()
            .get_mut()
            .dependencies
            .extend(deps);
    }
}
