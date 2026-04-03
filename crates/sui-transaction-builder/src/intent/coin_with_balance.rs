//! [`Coin`] and [`Balance`] intents and their shared resolver.
//!
//! Sui accounts hold funds in two forms: **coin objects** (`Coin<T>`)
//! and **address balances** (fungible balances attached directly to an
//! address). Move contracts can accept either form, so transactions
//! need a way to automatically source the right one.
//!
//! The [`Coin`] and [`Balance`] intents let callers declare how much of
//! a given type they need and whether they need a `Coin<T>` or a
//! `Balance<T>`. During transaction building the resolver replaces each
//! intent with concrete PTB commands that source funds from owned coins
//! and/or address balances.
//!
//! # Resolution algorithm
//!
//! The resolver prefers address balances over coins -- it only touches
//! coin objects when the address balance is insufficient. This enables
//! parallel execution of transactions. When coins are needed, the
//! resolver consolidates as many as possible (up to 500) to reduce
//! dust.
//!
//! Resolution runs once per coin type. It collects every [`Coin`] and
//! [`Balance`] intent for that type, loads available funds, and picks
//! one of two paths.
//!
//! ## Zero-balance intents
//!
//! Handled before path selection:
//!
//! - `balance::zero<T>()` for [`Balance`] intents.
//! - `coin::zero<T>()` for [`Coin`] intents.
//!
//! Zero-balance intents do not participate in path selection or the
//! combined split.
//!
//! ## Path selection
//!
//! 1. Collect all non-zero intents for the type, compute
//!    `total_required`.
//! 2. Fetch `address_balance` via `getBalance()`.
//! 3. Verify sufficient funds.
//! 4. If `address_balance >= total_required` --> **path 1**.
//! 5. Otherwise --> **path 2**.
//!
//! ## Path 1: individual withdrawal
//!
//! When the address balance covers everything, each intent becomes a
//! single direct withdrawal. No coins are touched, which enables
//! parallel execution.
//!
//! ```text
//! for each intent:
//!     Coin    -> coin::redeem_funds(withdrawal(amount))
//!     Balance -> balance::redeem_funds(withdrawal(amount))
//! ```
//!
//! ## Path 2: merge and split
//!
//! Used when the address balance alone is insufficient. Coins are
//! loaded first (up to 500 for dust consolidation), and address
//! balance covers whatever the coins can't.
//!
//! ### Step 1 -- build sources and merge
//!
//! ```text
//! coins = select_coins(total_required)   // loads up to 500
//! shortfall = max(0, total_required - loaded_coin_balance)
//!
//! if SUI and gas coin enabled:
//!     // coins added as gas payment objects
//!     sources = [tx.gas]
//!     if shortfall > 0:
//!         sources.push(coin::redeem_funds(withdrawal(shortfall)))
//! else:
//!     sources = [...coins]
//!     if shortfall > 0:
//!         sources.push(coin::redeem_funds(withdrawal(shortfall)))
//!
//! base_coin = sources[0]
//! MergeCoins(base_coin, sources[1..])   // chunked at 500
//! ```
//!
//! ### Step 2 -- split and convert
//!
//! ```text
//! SplitCoins(base_coin, [amt_1, amt_2, ..., amt_n])
//!     -> results mapped to intents
//!
//! for each Balance intent:
//!     coin::into_balance(split_result)  ->  intent result
//! ```
//!
//! ### Step 3 -- remainder
//!
//! After splitting, the merged coin may have leftover balance from
//! coin consolidation.
//!
//! - **AB was used or Balance intents exist**: send the remainder
//!   back to the sender's address balance via `coin::send_funds`.
//! - **Coin-only, no AB**: the merged coin stays with the sender as
//!   an owned object.
//! - **Gas coin**: no remainder handling -- the leftover stays in
//!   the gas coin.

use crate::Argument;
use crate::Function;
use crate::ObjectInput;
use crate::TransactionBuilder;
use crate::builder::ResolvedArgument;
use crate::intent::BoxError;
use crate::intent::Intent;
use crate::intent::IntentResolver;
use crate::intent::MAX_ARGUMENTS;
use crate::intent::MAX_GAS_OBJECTS;
use futures::StreamExt;
use std::collections::BTreeMap;
use std::collections::HashSet;
use sui_rpc::field::FieldMask;
use sui_rpc::field::FieldMaskUtil;
use sui_rpc::proto::sui::rpc::v2::GetBalanceRequest;
use sui_rpc::proto::sui::rpc::v2::ListOwnedObjectsRequest;
use sui_sdk_types::Address;
use sui_sdk_types::Identifier;
use sui_sdk_types::StructTag;

/// An intent requesting a `Coin<T>` of a specific type and balance.
///
/// When resolved via [`TransactionBuilder::build`](crate::TransactionBuilder::build), the
/// resolver selects coins owned by the sender, merges them if necessary, and splits off the
/// requested amount.
///
/// # Examples
///
/// ```
/// use sui_transaction_builder::intent::Coin;
///
/// // Request 1 SUI (uses gas coin by default)
/// let coin = Coin::sui(1_000_000_000);
///
/// // Request a custom coin type
/// use sui_sdk_types::StructTag;
/// let usdc = Coin::new(
///     "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC"
///         .parse()
///         .unwrap(),
///     1_000_000,
/// );
/// ```
pub struct Coin {
    coin_type: StructTag,
    balance: u64,
    use_gas_coin: bool,
}

/// Backward-compatible alias for [`Coin`].
pub type CoinWithBalance = Coin;

impl Coin {
    /// Create a new [`Coin`] intent for the given coin type and amount (in base units).
    ///
    /// ```
    /// use sui_sdk_types::StructTag;
    /// use sui_transaction_builder::intent::Coin;
    ///
    /// let coin = Coin::new(StructTag::sui(), 1_000_000_000);
    /// ```
    pub fn new(coin_type: StructTag, balance: u64) -> Self {
        Self {
            coin_type,
            balance,
            use_gas_coin: true,
        }
    }

    /// Shorthand for requesting a SUI coin with the given balance.
    ///
    /// By default the resolver will draw from the gas coin. Call
    /// [`with_use_gas_coin(false)`](Self::with_use_gas_coin) to opt out.
    ///
    /// ```
    /// use sui_transaction_builder::intent::Coin;
    ///
    /// let one_sui = Coin::sui(1_000_000_000);
    /// ```
    pub fn sui(balance: u64) -> Self {
        Self {
            coin_type: StructTag::sui(),
            balance,
            use_gas_coin: true,
        }
    }

    /// Control whether the resolver should draw from the gas coin.
    ///
    /// This is only meaningful when the coin type is SUI. Pass `false` to force the resolver
    /// to select non-gas SUI coins instead.
    ///
    /// ```
    /// use sui_transaction_builder::intent::Coin;
    ///
    /// // Don't touch the gas coin -- select other SUI coins instead
    /// let coin = Coin::sui(1_000_000_000).with_use_gas_coin(false);
    /// ```
    pub fn with_use_gas_coin(self, use_gas_coin: bool) -> Self {
        Self {
            use_gas_coin,
            ..self
        }
    }
}

/// An intent requesting a `Balance<T>` of a specific type and amount.
///
/// When resolved via [`TransactionBuilder::build`](crate::TransactionBuilder::build), the
/// resolver prefers withdrawing directly from the sender's address balance when possible,
/// falling back to coin selection and conversion when necessary.
///
/// # Examples
///
/// ```
/// use sui_transaction_builder::intent::Balance;
///
/// // Request 1 SUI as a Balance<SUI>
/// let bal = Balance::sui(1_000_000_000);
///
/// // Request a custom type
/// use sui_sdk_types::StructTag;
/// let bal = Balance::new(
///     "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC"
///         .parse()
///         .unwrap(),
///     1_000_000,
/// );
/// ```
pub struct Balance {
    coin_type: StructTag,
    balance: u64,
    use_gas_coin: bool,
}

impl Balance {
    /// Create a new [`Balance`] intent for the given coin type and amount (in base units).
    ///
    /// ```
    /// use sui_sdk_types::StructTag;
    /// use sui_transaction_builder::intent::Balance;
    ///
    /// let bal = Balance::new(StructTag::sui(), 1_000_000_000);
    /// ```
    pub fn new(coin_type: StructTag, balance: u64) -> Self {
        Self {
            coin_type,
            balance,
            use_gas_coin: true,
        }
    }

    /// Shorthand for requesting a SUI balance with the given amount.
    ///
    /// By default the resolver will draw from the gas coin when falling back to the
    /// merge-and-split path. Call [`with_use_gas_coin(false)`](Self::with_use_gas_coin) to
    /// opt out.
    ///
    /// ```
    /// use sui_transaction_builder::intent::Balance;
    ///
    /// let one_sui = Balance::sui(1_000_000_000);
    /// ```
    pub fn sui(balance: u64) -> Self {
        Self {
            coin_type: StructTag::sui(),
            balance,
            use_gas_coin: true,
        }
    }

    /// Control whether the resolver should draw from the gas coin when the merge-and-split
    /// path is used.
    ///
    /// This is only meaningful when the coin type is SUI. Pass `false` to force the resolver
    /// to select non-gas SUI coins instead.
    ///
    /// ```
    /// use sui_transaction_builder::intent::Balance;
    ///
    /// let bal = Balance::sui(1_000_000_000).with_use_gas_coin(false);
    /// ```
    pub fn with_use_gas_coin(self, use_gas_coin: bool) -> Self {
        Self {
            use_gas_coin,
            ..self
        }
    }
}

impl Intent for Coin {
    fn register(self, builder: &mut TransactionBuilder) -> Argument {
        builder.register_resolver(Resolver);
        builder.unresolved(self)
    }
}

impl Intent for Balance {
    fn register(self, builder: &mut TransactionBuilder) -> Argument {
        builder.register_resolver(Resolver);
        builder.unresolved(self)
    }
}

#[derive(Debug)]
struct Resolver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntentKind {
    Coin,
    Balance,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CoinType {
    Gas,
    Coin(StructTag),
}

struct Request {
    id: usize,
    balance: u64,
    kind: IntentKind,
}

#[async_trait::async_trait]
impl IntentResolver for Resolver {
    async fn resolve(
        &self,
        builder: &mut TransactionBuilder,
        client: &mut sui_rpc::Client,
    ) -> Result<(), BoxError> {
        // Collect all the requests (both Coin and Balance intents).
        let mut requests: BTreeMap<CoinType, Vec<Request>> = BTreeMap::new();
        let mut zero_values = Vec::new();

        for (id, intent) in builder.intents.extract_if(.., |_id, intent| {
            intent.downcast_ref::<Coin>().is_some() || intent.downcast_ref::<Balance>().is_some()
        }) {
            if let Some(coin) = intent.downcast_ref::<Coin>() {
                if coin.balance == 0 {
                    zero_values.push((id, coin.coin_type.clone(), IntentKind::Coin));
                } else {
                    let coin_type = if coin.coin_type == StructTag::sui() && coin.use_gas_coin {
                        CoinType::Gas
                    } else {
                        CoinType::Coin(coin.coin_type.clone())
                    };
                    requests.entry(coin_type).or_default().push(Request {
                        id,
                        balance: coin.balance,
                        kind: IntentKind::Coin,
                    });
                }
            } else if let Some(bal) = intent.downcast_ref::<Balance>() {
                if bal.balance == 0 {
                    zero_values.push((id, bal.coin_type.clone(), IntentKind::Balance));
                } else {
                    let coin_type = if bal.coin_type == StructTag::sui() && bal.use_gas_coin {
                        CoinType::Gas
                    } else {
                        CoinType::Coin(bal.coin_type.clone())
                    };
                    requests.entry(coin_type).or_default().push(Request {
                        id,
                        balance: bal.balance,
                        kind: IntentKind::Balance,
                    });
                }
            }
        }

        for (id, coin_type, kind) in zero_values {
            Resolver::resolve_zero(builder, coin_type, id, kind);
        }

        for (coin_type, requests) in requests {
            Resolver::resolve_merge_and_split(builder, client, coin_type, &requests).await?;
        }

        Ok(())
    }
}

impl Resolver {
    fn resolve_zero(
        builder: &mut TransactionBuilder,
        coin_type: StructTag,
        request_id: usize,
        kind: IntentKind,
    ) {
        let (module, function) = match kind {
            IntentKind::Coin => ("coin", "zero"),
            IntentKind::Balance => ("balance", "zero"),
        };

        let result = builder.move_call(
            Function::new(
                Address::TWO,
                Identifier::from_static(module),
                Identifier::from_static(function),
            )
            .with_type_args(vec![coin_type.into()]),
            vec![],
        );

        *builder.arguments.get_mut(&request_id).unwrap() = ResolvedArgument::ReplaceWith(result);
    }

    /// Convert a `Coin<T>` into a `Balance<T>` via `0x2::coin::into_balance`.
    fn into_balance(
        builder: &mut TransactionBuilder,
        coin_type: &StructTag,
        coin_arg: Argument,
    ) -> Argument {
        builder.move_call(
            Function::new(
                Address::TWO,
                Identifier::from_static("coin"),
                Identifier::from_static("into_balance"),
            )
            .with_type_args(vec![coin_type.clone().into()]),
            vec![coin_arg],
        )
    }

    /// Send a coin's remaining value back to an address's address balance via
    /// `0x2::coin::send_funds`.
    fn send_funds(
        builder: &mut TransactionBuilder,
        coin_type: &StructTag,
        coin_arg: Argument,
        owner: Address,
    ) {
        let owner_arg = builder.pure(&owner);
        builder.move_call(
            Function::new(
                Address::TWO,
                Identifier::from_static("coin"),
                Identifier::from_static("send_funds"),
            )
            .with_type_args(vec![coin_type.clone().into()]),
            vec![coin_arg, owner_arg],
        );
    }

    /// Path 1: AB covers everything -- individual withdrawals per intent.
    fn resolve_individual_withdrawals(
        builder: &mut TransactionBuilder,
        coin_type: &StructTag,
        requests: &[Request],
    ) {
        for request in requests {
            let result = match request.kind {
                IntentKind::Coin => {
                    builder.funds_withdrawal_coin(coin_type.clone().into(), request.balance)
                }
                IntentKind::Balance => {
                    builder.funds_withdrawal_balance(coin_type.clone().into(), request.balance)
                }
            };
            *builder.arguments.get_mut(&request.id).unwrap() =
                ResolvedArgument::ReplaceWith(result);
        }
    }

    /// Resolve all intents for a single coin type via path 2 (merge and
    /// split). The `use_gas_coin` flag controls how coins become sources:
    ///
    /// - **gas coin**: coins are added as gas payment objects and the gas
    ///   coin (`tx.gas`) is the merge base. AB shortfall includes a ~0.5
    ///   SUI buffer for gas fees. No remainder handling.
    /// - **non-gas**: first loaded coin is the merge base. Remainder is
    ///   sent back to AB when AB was used or Balance intents are present.
    async fn resolve_merge_and_split(
        builder: &mut TransactionBuilder,
        client: &mut sui_rpc::Client,
        coin_type: CoinType,
        requests: &[Request],
    ) -> Result<(), BoxError> {
        let (coin_type, use_gas_coin) = match coin_type {
            CoinType::Gas => (StructTag::sui(), true),
            CoinType::Coin(t) => (t, false),
        };
        let coin_type = &coin_type;
        let sender = builder
            .sender()
            .ok_or("Sender must be set to resolve Coin/Balance intents")?;

        if requests.is_empty() {
            return Err("BUG: requests is empty".into());
        }

        let sum = requests.iter().map(|r| r.balance).sum();

        let balance_response = client
            .state_client()
            .get_balance(
                GetBalanceRequest::default()
                    .with_owner(sender)
                    .with_coin_type(coin_type),
            )
            .await?;

        // Extract chain_id and epoch from the response metadata for the
        // gas coin reservation ref.
        let _chain_id = {
            use sui_rpc::client::ResponseExt;
            (balance_response.chain_id(), balance_response.epoch())
        };

        let balance = balance_response
            .into_inner()
            .balance
            .take()
            .unwrap_or_default();

        // Early return with an error if the sender does not have sufficient balance.
        if balance.balance() < sum {
            return Err(format!(
                "address {} does not have sufficient balance of {}: requested {} available {}",
                sender,
                coin_type,
                sum,
                balance.balance()
            )
            .into());
        }

        // Path 1: AB covers everything -- individual withdrawals per intent.
        // No coins are touched, enabling parallel execution.
        if balance.address_balance() >= sum {
            Self::resolve_individual_withdrawals(builder, coin_type, requests);
            return Ok(());
        }

        // Path 2: AB insufficient, need coins. Coins are loaded first (up
        // to 500 for consolidation), AB covers whatever the coins can't.
        let excludes = builder.used_object_ids();
        let (coins, remaining) =
            Self::select_coins(client, &sender, coin_type, sum, &excludes).await?;

        if balance.address_balance() < remaining {
            return Err(format!(
                "unable to find sufficient coins of type {}. \
                 requested {} found {} and AB of {} is insufficient to cover difference.",
                coin_type,
                sum,
                sum - remaining,
                balance.address_balance(),
            )
            .into());
        }

        if coins.is_empty() {
            // No coins found but AB covers the full amount (caught by
            // path 1 above in normal flow). Included for completeness.
            Self::resolve_individual_withdrawals(builder, coin_type, requests);
            return Ok(());
        }

        // Build the merge base and fold all sources into it.
        let has_balance_intents = requests.iter().any(|r| r.kind == IntentKind::Balance);
        let used_ab = remaining > 0;

        let (base, deps) = if use_gas_coin {
            Self::prepare_gas_coin_sources(
                builder,
                &coins,
                remaining,
                coin_type,
                // sender,
                balance.address_balance(),
                // chain_id,
            )
        } else {
            Self::prepare_coin_sources(builder, coins, remaining, coin_type)
        };

        // Split all requested amounts from the merged base.
        let amounts = requests.iter().map(|r| builder.pure(&r.balance)).collect();
        let outputs = builder.split_coins(base, amounts);
        if !deps.is_empty() {
            builder
                .commands
                .last_entry()
                .unwrap()
                .get_mut()
                .dependencies
                .extend(deps);
        }

        // Remainder handling.
        if !use_gas_coin && (used_ab || has_balance_intents) {
            Self::send_funds(builder, coin_type, base, sender);
        }

        // Assign results, converting Balance intents via coin::into_balance.
        for (result, request) in outputs.into_iter().zip(requests.iter()) {
            let final_arg = match request.kind {
                IntentKind::Balance => Self::into_balance(builder, coin_type, result),
                IntentKind::Coin => result,
            };
            *builder.arguments.get_mut(&request.id).unwrap() =
                ResolvedArgument::ReplaceWith(final_arg);
        }

        Ok(())
    }

    /// Prepare coin sources for the **non-gas** path. The first loaded coin
    /// becomes the merge base and all remaining coins (plus an optional AB
    /// shortfall withdrawal) are merged into it.
    fn prepare_coin_sources(
        builder: &mut TransactionBuilder,
        coins: Vec<ObjectInput>,
        remaining: u64,
        coin_type: &StructTag,
    ) -> (Argument, Vec<Argument>) {
        let coin_args: Vec<Argument> = coins.into_iter().map(|c| builder.object(c)).collect();
        let (&first, rest) = coin_args.split_first().expect("coins must not be empty");
        let mut deps = Vec::new();

        for chunk in rest.chunks(MAX_ARGUMENTS) {
            builder.merge_coins(first, chunk.to_vec());
            deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
        }

        if remaining > 0 {
            let ab_coin = builder.funds_withdrawal_coin(coin_type.clone().into(), remaining);
            deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
            builder.merge_coins(first, vec![ab_coin]);
            deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
        }

        (first, deps)
    }

    /// Prepare coin sources for the **gas coin** path. Coins are added as
    /// gas payment objects (up to 250); the excess is merged into the gas
    /// coin. If there is an AB shortfall, it is withdrawn via
    /// `FundsWithdrawal` and a synthetic gas reservation ref is prepended
    /// to the gas payment array so the server reserves the remaining AB
    /// for gas.
    fn prepare_gas_coin_sources(
        builder: &mut TransactionBuilder,
        coins: &[ObjectInput],
        remaining: u64,
        coin_type: &StructTag,
        // sender: Address,
        address_balance: u64,
        // chain_info: (Option<sui_sdk_types::Digest>, Option<u64>),
    ) -> (Argument, Vec<Argument>) {
        let gas = builder.gas();
        let mut deps = Vec::new();

        // Insert a gas coin reservation ref so the server reserves
        // address balance for gas payment.
        // let reservation_amount = address_balance.saturating_sub(remaining);
        // if let (Some(chain_id), Some(epoch)) = chain_info
        //     && reservation_amount > 0
        // {
        //     let reservation = sui_sdk_types::ObjectReference::coin_reservation(
        //         coin_type,
        //         reservation_amount,
        //         epoch,
        //         chain_id,
        //         sender,
        //     );
        //     let (id, version, digest) = reservation.into_parts();
        //     builder.add_gas_objects([ObjectInput::owned(id, version, digest)]);
        // }

        let (use_as_gas, excess) = coins.split_at(std::cmp::min(
            coins.len(),
            MAX_GAS_OBJECTS.saturating_sub(builder.gas.len()),
        ));
        builder.add_gas_objects(use_as_gas.iter().cloned());

        for chunk in excess
            .iter()
            .map(|coin| builder.object(coin.clone()))
            .collect::<Vec<_>>()
            .chunks(MAX_ARGUMENTS)
        {
            builder.merge_coins(gas, chunk.to_vec());
            deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
        }

        if remaining > 0 {
            // Reserve the remaining AB to account for gas
            let ab_coin = builder.funds_withdrawal_coin(coin_type.clone().into(), address_balance);
            deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
            builder.merge_coins(gas, vec![ab_coin]);
            deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
        }

        (gas, deps)
    }

    async fn select_coins(
        client: &mut sui_rpc::Client,
        owner_address: &Address,
        coin_type: &StructTag,
        amount: u64,
        excludes: &HashSet<Address>,
    ) -> Result<(Vec<ObjectInput>, u64), BoxError> {
        let coin_struct = StructTag::coin(coin_type.clone().into());
        let list_request = ListOwnedObjectsRequest::default()
            .with_owner(owner_address)
            .with_object_type(&coin_struct)
            .with_page_size(500u32)
            .with_read_mask(FieldMask::from_paths([
                "object_id",
                "version",
                "digest",
                "balance",
                "owner",
            ]));

        let mut coin_stream = Box::pin(client.list_owned_objects(list_request));
        let mut selected_coins = Vec::new();
        let mut remaining = amount;

        while let Some(object_result) = coin_stream.next().await {
            let object = object_result?;
            let coin = ObjectInput::try_from_object_proto(&object)?;

            if excludes.contains(&coin.object_id()) {
                continue;
            }

            remaining = remaining.saturating_sub(object.balance());
            selected_coins.push(coin);

            // If we've found enough, continue collecting coins to smash up to ~500.
            if remaining == 0 && selected_coins.len() >= 500 {
                break;
            }
        }

        Ok((selected_coins, remaining))
    }
}
