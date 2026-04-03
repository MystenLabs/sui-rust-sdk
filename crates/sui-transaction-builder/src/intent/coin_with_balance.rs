use crate::Function;
use crate::ObjectInput;
use crate::intent::BoxError;
use crate::intent::Intent;
use crate::intent::IntentId;
use crate::intent::IntentResolver;
use crate::intent::MAX_ARGUMENTS;
use crate::intent::MAX_GAS_OBJECTS;
use crate::intent::ResolveContext;
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

/// An intent requesting a coin of a specific type and balance.
///
/// When resolved via [`TransactionBuilder::build`](crate::TransactionBuilder::build), the
/// resolver selects coins owned by the sender, merges them if necessary, and splits off the
/// requested amount.
///
/// # Examples
///
/// ```
/// use sui_transaction_builder::intent::CoinWithBalance;
///
/// // Request 1 SUI (uses gas coin by default)
/// let coin = CoinWithBalance::sui(1_000_000_000);
///
/// // Request a custom coin type
/// use sui_sdk_types::StructTag;
/// let usdc = CoinWithBalance::new(
///     "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC"
///         .parse()
///         .unwrap(),
///     1_000_000,
/// );
/// ```
pub struct CoinWithBalance {
    coin_type: StructTag,
    balance: u64,
    use_gas_coin: bool,
}

impl CoinWithBalance {
    /// Create a new [`CoinWithBalance`] intent for the given coin type and amount (in base
    /// units).
    ///
    /// ```
    /// use sui_sdk_types::StructTag;
    /// use sui_transaction_builder::intent::CoinWithBalance;
    ///
    /// let coin = CoinWithBalance::new(StructTag::sui(), 1_000_000_000);
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
    /// use sui_transaction_builder::intent::CoinWithBalance;
    ///
    /// let one_sui = CoinWithBalance::sui(1_000_000_000);
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
    /// use sui_transaction_builder::intent::CoinWithBalance;
    ///
    /// // Don't touch the gas coin -- select other SUI coins instead
    /// let coin = CoinWithBalance::sui(1_000_000_000).with_use_gas_coin(false);
    /// ```
    pub fn with_use_gas_coin(self, use_gas_coin: bool) -> Self {
        Self {
            use_gas_coin,
            ..self
        }
    }
}

impl Intent for CoinWithBalance {
    type Resolver = CoinWithBalanceResolver;
}

/// Resolver for [`CoinWithBalance`] intents.
///
/// This resolver is registered automatically when a [`CoinWithBalance`]
/// intent is added to a [`TransactionBuilder`](crate::TransactionBuilder).
/// You do not need to construct it directly.
#[derive(Debug, Default)]
pub struct CoinWithBalanceResolver;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CoinType {
    Gas,
    Coin(StructTag),
}

#[async_trait::async_trait]
impl IntentResolver for CoinWithBalanceResolver {
    async fn resolve(
        &self,
        ctx: &mut ResolveContext<'_>,
        client: &mut sui_rpc::Client,
    ) -> Result<(), BoxError> {
        // Collect all the requests.
        let mut requests: BTreeMap<CoinType, Vec<(IntentId, u64)>> = BTreeMap::new();
        let mut zero_values = Vec::new();

        for (id, request) in ctx.take_intents::<CoinWithBalance>() {
            if request.balance == 0 {
                zero_values.push((id, request.coin_type));
            } else {
                let coin_type = if request.coin_type == StructTag::sui() && request.use_gas_coin {
                    CoinType::Gas
                } else {
                    CoinType::Coin(request.coin_type)
                };
                requests
                    .entry(coin_type)
                    .or_default()
                    .push((id, request.balance));
            }
        }

        for (id, coin_type) in zero_values {
            Self::resolve_zero_balance_coin(ctx, coin_type, id);
        }

        for (coin_type, requests) in requests {
            match coin_type {
                CoinType::Gas => {
                    Self::resolve_gas_coin(ctx, client, &requests).await?;
                }
                CoinType::Coin(coin_type) => {
                    Self::resolve_coin_type(ctx, client, &coin_type, &requests).await?;
                }
            }
        }

        Ok(())
    }
}

impl CoinWithBalanceResolver {
    fn resolve_zero_balance_coin(
        ctx: &mut ResolveContext<'_>,
        coin_type: StructTag,
        request_id: IntentId,
    ) {
        let coin = ctx.move_call(
            Function::new(
                Address::TWO,
                Identifier::from_static("coin"),
                Identifier::from_static("zero"),
            )
            .with_type_args(vec![coin_type.into()]),
            vec![],
        );

        ctx.redirect_argument(request_id, coin);
    }

    async fn resolve_coin_type(
        ctx: &mut ResolveContext<'_>,
        client: &mut sui_rpc::Client,
        coin_type: &StructTag,
        requests: &[(IntentId, u64)],
    ) -> Result<(), BoxError> {
        let sender = ctx
            .sender()
            .ok_or("Sender must be set to resolve CoinWithBalance")?;

        if requests.is_empty() {
            return Err("BUG: requests is empty".into());
        }

        let sum = requests.iter().map(|(_, balance)| *balance).sum();

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

        let excludes = ctx.used_object_ids();
        let (coins, remaining) =
            Self::select_coins(client, &sender, coin_type, sum, &excludes).await?;

        // If address balance amount isn't enough to cover the remaining requested amount we need
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

        let split_coin_args = if let [first, rest @ ..] = coins
            .into_iter()
            .map(|coin| ctx.object(coin))
            .collect::<Vec<_>>()
            .as_slice()
        {
            // We have at least 1 coin.

            let mut deps = Vec::new();
            for chunk in rest.chunks(MAX_ARGUMENTS) {
                ctx.merge_coins(*first, chunk.to_vec());
                deps.push(ctx.last_command_argument());
            }

            // If the coins we selected were not enough we need to pull from AB for the remaining
            // amount and merge it into the first coin.
            if remaining > 0 {
                let ab_coin = ctx.funds_withdrawal_coin(coin_type.clone().into(), remaining);
                deps.push(ctx.last_command_argument());
                ctx.merge_coins(*first, vec![ab_coin]);
                deps.push(ctx.last_command_argument());
            }

            let amounts = requests
                .iter()
                .map(|(_, balance)| ctx.pure(balance))
                .collect();
            let coin_outputs = ctx.split_coins(*first, amounts);
            if !deps.is_empty() {
                ctx.add_dependencies_to_last_command(deps);
            }

            //TODO send remaining to AB

            coin_outputs
        } else {
            // We have no coins, but have sufficient AB to cover all requested amounts.
            requests
                .iter()
                .map(|(_, balance)| ctx.funds_withdrawal_coin(coin_type.clone().into(), *balance))
                .collect()
        };

        for (coin, (request_id, _)) in split_coin_args.into_iter().zip(requests.iter()) {
            ctx.redirect_argument(*request_id, coin);
        }

        Ok(())
    }

    async fn resolve_gas_coin(
        ctx: &mut ResolveContext<'_>,
        client: &mut sui_rpc::Client,
        requests: &[(IntentId, u64)],
    ) -> Result<(), BoxError> {
        let sender = ctx
            .sender()
            .ok_or("Sender must be set to resolve CoinWithBalance")?;

        if requests.is_empty() {
            return Err("BUG: requests is empty".into());
        }

        let coin_type = StructTag::sui();

        let sum = requests.iter().map(|(_, balance)| *balance).sum();

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

        let excludes = ctx.used_object_ids();
        let (coins, remaining) =
            Self::select_coins(client, &sender, &coin_type, sum, &excludes).await?;

        // If address balance amount isn't enough to cover the remaining requested amount we need
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

        let coin_args = if coins.is_empty() {
            // We have no coins, but have sufficient AB to cover all requested amounts.
            requests
                .iter()
                .map(|(_, balance)| {
                    ctx.funds_withdrawal_coin(coin_type.clone().into(), *balance)
                })
                .collect()
        } else {
            let gas = ctx.gas();
            let mut deps = Vec::new();

            // Append to gas coin up to 250 coins.
            let (use_as_gas, remaining_coins) = coins.split_at(std::cmp::min(
                coins.len(),
                MAX_GAS_OBJECTS.saturating_sub(ctx.gas_object_count()),
            ));
            ctx.add_gas_objects(use_as_gas.iter().cloned());

            // Any remaining do a merge coins.
            for chunk in remaining_coins
                .iter()
                .map(|coin| ctx.object(coin.clone()))
                .collect::<Vec<_>>()
                .chunks(MAX_ARGUMENTS)
            {
                ctx.merge_coins(gas, chunk.to_vec());
                deps.push(ctx.last_command_argument());
            }

            // If the coins we selected were not enough we need to pull from AB for the remaining
            // amount and merge it into the gas coin.
            if remaining > 0 {
                // Reserve a small amount more to account for budget ~.5 SUI's worth.
                let ab_coin =
                    ctx.funds_withdrawal_coin(coin_type.clone().into(), remaining + 500_000_000);
                deps.push(ctx.last_command_argument());
                ctx.merge_coins(gas, vec![ab_coin]);
                deps.push(ctx.last_command_argument());
            }

            let amounts = requests
                .iter()
                .map(|(_, balance)| ctx.pure(balance))
                .collect();
            let split_coin_args = ctx.split_coins(gas, amounts);
            if !deps.is_empty() {
                ctx.add_dependencies_to_last_command(deps);
            }

            // We can't send gas coin to AB so we'll leave as-is.

            split_coin_args
        };

        for (coin, (request_id, _)) in coin_args.into_iter().zip(requests.iter()) {
            ctx.redirect_argument(*request_id, coin);
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
