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
//! Resolution runs once per coin type. It collects every [`Coin`] and
//! [`Balance`] intent for that type, loads available funds, and picks
//! one of two paths.
//!
//! ## Loading
//!
//! Owned coins and address balance are loaded in parallel via
//! `getBalance()` and `listCoins()`. The full first page of coins is
//! always loaded; pagination continues only when the loaded coin
//! balance plus address balance is still insufficient. An error is
//! returned when total available < total required.
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
//! 2. Load coins, get `address_balance`.
//! 3. Verify sufficient funds.
//! 4. If **all** intents are [`Balance`] **and**
//!    `address_balance >= total_required` --> **path 1**.
//! 5. Otherwise --> **path 2**.
//!
//! ## Path 1: direct withdrawal
//!
//! When every intent for a type is a [`Balance`] and the address
//! balance covers everything, each intent becomes a single direct
//! withdrawal. No coins are touched, which enables parallel execution.
//!
//! ```text
//! for each Balance intent:
//!     balance::redeem_funds(withdrawal(amount))  ->  intent result
//! ```
//!
//! ## Path 2: merge and split
//!
//! The general-purpose path used whenever [`Coin`] intents exist or
//! the address balance alone is insufficient. It builds a merged coin
//! from available sources, splits all intent amounts in a single call,
//! and handles the remainder.
//!
//! ### Step 1 -- build sources and merge
//!
//! ```text
//! sources = []
//!
//! if SUI and gas coin enabled:
//!     sources.push(tx.gas)
//! else:
//!     sources.push(...loaded_coins)
//!     shortfall = max(0, total_required - loaded_coin_balance)
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
//! After splitting, the merged coin may have leftover balance (loaded
//! coins exceeded what was needed) or be exactly zero.
//!
//! - **Balance intents exist**: send the remainder back to the
//!   sender's address balance via `coin::send_funds`.
//! - **Coin-only, exact match** (AB was used, so the merged coin is
//!   exactly zero after splitting): `coin::destroy_zero`.
//! - **Coin-only, surplus**: the merged coin stays with the sender as
//!   an owned object.
//! - **Gas coin**: no remainder handling -- the leftover stays in the
//!   gas coin.

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
            match coin_type {
                CoinType::Gas => {
                    Resolver::resolve_gas_coin(builder, client, &requests).await?;
                }
                CoinType::Coin(coin_type) => {
                    Resolver::resolve_coin_type(builder, client, &coin_type, &requests).await?;
                }
            }
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

    /// Destroy a zero-balance coin via `0x2::coin::destroy_zero`.
    fn destroy_zero(builder: &mut TransactionBuilder, coin_type: &StructTag, coin_arg: Argument) {
        builder.move_call(
            Function::new(
                Address::TWO,
                Identifier::from_static("coin"),
                Identifier::from_static("destroy_zero"),
            )
            .with_type_args(vec![coin_type.clone().into()]),
            vec![coin_arg],
        );
    }

    /// Path 1: Direct withdrawal from address balance. Used when all intents for a coin type
    /// are [`Balance`] intents and the address balance covers the total.
    fn resolve_direct_withdrawal(
        builder: &mut TransactionBuilder,
        coin_type: &StructTag,
        requests: &[Request],
    ) {
        for request in requests {
            let balance =
                builder.funds_withdrawal_balance(coin_type.clone().into(), request.balance);
            *builder.arguments.get_mut(&request.id).unwrap() =
                ResolvedArgument::ReplaceWith(balance);
        }
    }

    async fn resolve_coin_type(
        builder: &mut TransactionBuilder,
        client: &mut sui_rpc::Client,
        coin_type: &StructTag,
        requests: &[Request],
    ) -> Result<(), BoxError> {
        let sender = builder
            .sender()
            .ok_or("Sender must be set to resolve Coin/Balance intents")?;

        if requests.is_empty() {
            return Err("BUG: requests is empty".into());
        }

        let sum = requests.iter().map(|r| r.balance).sum();

        let balance = client
            .state_client()
            .get_balance(
                GetBalanceRequest::default()
                    .with_owner(sender)
                    .with_coin_type(coin_type),
            )
            .await?
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

        // Path 1: direct withdrawal when all intents are Balance and AB covers the total.
        let all_balance = requests.iter().all(|r| r.kind == IntentKind::Balance);
        if all_balance && balance.address_balance() >= sum {
            Self::resolve_direct_withdrawal(builder, coin_type, requests);
            return Ok(());
        }

        // Path 2: merge and split.
        let excludes = builder.used_object_ids();
        let (coins, remaining) =
            Self::select_coins(client, &sender, coin_type, sum, &excludes).await?;

        // If the address balance isn't enough to cover the remaining requested amount we need
        // to bail.
        if balance.address_balance() < remaining {
            return Err(format!(
                "unable to find sufficient coins of type {}. requested {} found {} and AB of {} is insufficient to cover difference.",
                coin_type,
                sum,
                sum - remaining,
                balance.address_balance(),
            )
            .into());
        }

        let has_balance_intents = requests.iter().any(|r| r.kind == IntentKind::Balance);
        let used_ab = remaining > 0;

        if let [first, rest @ ..] = coins
            .into_iter()
            .map(|coin| builder.object(coin))
            .collect::<Vec<_>>()
            .as_slice()
        {
            // We have at least one coin.
            let mut deps = Vec::new();
            for chunk in rest.chunks(MAX_ARGUMENTS) {
                builder.merge_coins(*first, chunk.to_vec());
                deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
            }

            // If the coins we selected were not enough we need to pull from AB for the
            // remaining amount and merge it into the first coin.
            if remaining > 0 {
                let ab_coin = builder.funds_withdrawal_coin(coin_type.clone().into(), remaining);
                deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
                builder.merge_coins(*first, vec![ab_coin]);
                deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
            }

            let amounts = requests.iter().map(|r| builder.pure(&r.balance)).collect();
            let coin_outputs = builder.split_coins(*first, amounts);
            if !deps.is_empty() {
                builder
                    .commands
                    .last_entry()
                    .unwrap()
                    .get_mut()
                    .dependencies
                    .extend(deps);
            }

            // Remainder handling: the merged coin may have leftover balance after splitting.
            if has_balance_intents {
                Self::send_funds(builder, coin_type, *first, sender);
            } else if used_ab {
                Self::destroy_zero(builder, coin_type, *first);
            }

            // Assign results, converting Balance intents via coin::into_balance.
            for (result, request) in coin_outputs.into_iter().zip(requests.iter()) {
                let final_arg = match request.kind {
                    IntentKind::Balance => Self::into_balance(builder, coin_type, result),
                    IntentKind::Coin => result,
                };
                *builder.arguments.get_mut(&request.id).unwrap() =
                    ResolvedArgument::ReplaceWith(final_arg);
            }
        } else {
            // We have no coins, but have sufficient AB to cover all requested amounts.
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

        Ok(())
    }

    async fn resolve_gas_coin(
        builder: &mut TransactionBuilder,
        client: &mut sui_rpc::Client,
        requests: &[Request],
    ) -> Result<(), BoxError> {
        let sender = builder
            .sender()
            .ok_or("Sender must be set to resolve Coin/Balance intents")?;

        if requests.is_empty() {
            return Err("BUG: requests is empty".into());
        }

        let coin_type = StructTag::sui();

        let sum = requests.iter().map(|r| r.balance).sum();

        let balance = client
            .state_client()
            .get_balance(
                GetBalanceRequest::default()
                    .with_owner(sender)
                    .with_coin_type(&coin_type),
            )
            .await?
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

        // Path 1: direct withdrawal when all intents are Balance and AB covers the total.
        let all_balance = requests.iter().all(|r| r.kind == IntentKind::Balance);
        if all_balance && balance.address_balance() >= sum {
            Self::resolve_direct_withdrawal(builder, &coin_type, requests);
            return Ok(());
        }

        // Path 2: merge and split using gas coin.
        let excludes = builder.used_object_ids();
        let (coins, remaining) =
            Self::select_coins(client, &sender, &coin_type, sum, &excludes).await?;

        // If the address balance isn't enough to cover the remaining requested amount we need
        // to bail.
        if balance.address_balance() < remaining {
            return Err(format!(
                "unable to find sufficient coins of type {}. requested {} found {} and AB of {} is insufficient to cover difference.",
                coin_type,
                sum,
                sum - remaining,
                balance.address_balance(),
            )
            .into());
        }

        if coins.is_empty() {
            // We have no coins, but have sufficient AB to cover all requested amounts.
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
        } else {
            let gas = builder.gas();
            let mut deps = Vec::new();

            // Append to gas coin up to 250 coins.
            let (use_as_gas, remaining_coins) = coins.split_at(std::cmp::min(
                coins.len(),
                MAX_GAS_OBJECTS.saturating_sub(builder.gas.len()),
            ));
            builder.add_gas_objects(use_as_gas.iter().cloned());

            // Any remaining do a merge coins.
            for chunk in remaining_coins
                .iter()
                .map(|coin| builder.object(coin.clone()))
                .collect::<Vec<_>>()
                .chunks(MAX_ARGUMENTS)
            {
                builder.merge_coins(gas, chunk.to_vec());
                deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
            }

            // If the coins we selected were not enough we need to pull from AB for the
            // remaining amount and merge it into the gas coin.
            if remaining > 0 {
                // Reserve a small amount more to account for budget (~0.5 SUI's worth).
                let ab_coin = builder
                    .funds_withdrawal_coin(coin_type.clone().into(), remaining + 500_000_000);
                deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
                builder.merge_coins(gas, vec![ab_coin]);
                deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
            }

            let amounts = requests.iter().map(|r| builder.pure(&r.balance)).collect();
            let split_coin_args = builder.split_coins(gas, amounts);
            if !deps.is_empty() {
                builder
                    .commands
                    .last_entry()
                    .unwrap()
                    .get_mut()
                    .dependencies
                    .extend(deps);
            }

            // Gas coin: no remainder handling -- leftover stays in the gas coin.

            // Assign results, converting Balance intents via coin::into_balance.
            for (result, request) in split_coin_args.into_iter().zip(requests.iter()) {
                let final_arg = match request.kind {
                    IntentKind::Balance => Self::into_balance(builder, &coin_type, result),
                    IntentKind::Coin => result,
                };
                *builder.arguments.get_mut(&request.id).unwrap() =
                    ResolvedArgument::ReplaceWith(final_arg);
            }
        }

        Ok(())
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
