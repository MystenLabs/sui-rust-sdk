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
use std::collections::BTreeMap;
use sui_sdk_types::Address;
use sui_sdk_types::Identifier;
use sui_sdk_types::StructTag;

pub struct CoinWithBalance {
    coin_type: StructTag,
    balance: u64,
    use_gas_coin: bool,
}

impl CoinWithBalance {
    pub fn new(coin_type: StructTag, balance: u64) -> Self {
        Self {
            coin_type,
            balance,
            use_gas_coin: true,
        }
    }

    pub fn sui(balance: u64) -> Self {
        Self {
            coin_type: StructTag::sui(),
            balance,
            use_gas_coin: true,
        }
    }

    // Used to explicitly opt out of using the gas coin.
    // This parameter is only respected when coin type is `SUI`.
    pub fn with_use_gas_coin(self, use_gas_coin: bool) -> Self {
        Self {
            use_gas_coin,
            ..self
        }
    }
}

impl Intent for CoinWithBalance {
    fn register(self, builder: &mut TransactionBuilder) -> Argument {
        builder.register_resolver(CoinWithBalanceResolver);
        builder.unresolved(self)
    }
}

#[derive(Debug)]
struct CoinWithBalanceResolver;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CoinType {
    Gas,
    Coin(StructTag),
}

#[async_trait::async_trait]
impl IntentResolver for CoinWithBalanceResolver {
    async fn resolve(
        &self,
        builder: &mut TransactionBuilder,
        client: &mut sui_rpc::Client,
    ) -> Result<(), BoxError> {
        // Collect all the requests
        let mut requests: BTreeMap<CoinType, Vec<(usize, u64)>> = BTreeMap::new();
        let mut zero_values = Vec::new();

        for (id, intent) in builder.intents.extract_if(.., |_id, intent| {
            intent.downcast_ref::<CoinWithBalance>().is_some()
        }) {
            let request = intent.downcast_ref::<CoinWithBalance>().unwrap();

            if request.balance == 0 {
                zero_values.push((id, request.coin_type.clone()));
            } else {
                let coin_type = if request.coin_type == StructTag::sui() && request.use_gas_coin {
                    CoinType::Gas
                } else {
                    CoinType::Coin(request.coin_type.clone())
                };
                requests
                    .entry(coin_type)
                    .or_default()
                    .push((id, request.balance));
            }
        }

        for (id, coin_type) in zero_values {
            CoinWithBalanceResolver::resolve_zero_balance_coin(builder, coin_type, id);
        }

        for (coin_type, requests) in requests {
            match coin_type {
                CoinType::Gas => {
                    CoinWithBalanceResolver::resolve_gas_coin(builder, client, &requests).await?;
                }
                CoinType::Coin(coin_type) => {
                    CoinWithBalanceResolver::resolve_coin_type(
                        builder, client, &coin_type, &requests,
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }
}

impl CoinWithBalanceResolver {
    fn resolve_zero_balance_coin(
        builder: &mut TransactionBuilder,
        coin_type: StructTag,
        request_id: usize,
    ) {
        let coin = builder.move_call(
            Function::new(
                Address::TWO,
                Identifier::from_static("coin"),
                Identifier::from_static("zero"),
            )
            .with_type_args(vec![coin_type.into()]),
            vec![],
        );

        *builder.arguments.get_mut(&request_id).unwrap() = ResolvedArgument::ReplaceWith(coin);
    }

    async fn resolve_coin_type(
        builder: &mut TransactionBuilder,
        client: &mut sui_rpc::Client,
        coin_type: &StructTag,
        requests: &[(usize, u64)],
    ) -> Result<(), BoxError> {
        let sender = builder
            .sender()
            .ok_or("Sender must be set to resolve CoinWithBalance")?;

        if requests.is_empty() {
            return Err("BUG: requests is empty".into());
        }

        let sum = requests.iter().map(|(_, balance)| *balance).sum();

        let coins = client
            //TODO populate excludes
            .select_coins(&sender, &(coin_type.clone().into()), sum, &[])
            .await?;

        // For SUI need to handle working with gas coin
        let split_coin_args = if let [first, rest @ ..] = coins
            .into_iter()
            .map(|coin| ObjectInput::try_from_object_proto(&coin).map(|coin| builder.object(coin)))
            .collect::<Result<Vec<_>, _>>()?
            .as_slice()
        {
            let mut deps = Vec::new();
            for chunk in rest.chunks(MAX_ARGUMENTS) {
                builder.merge_coins(*first, chunk.to_vec());
                deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
            }

            let amounts = requests
                .iter()
                .map(|(_, balance)| builder.pure(balance))
                .collect();
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
            coin_outputs
        } else {
            return Err(format!("unable to find sufficient coins of type {}", coin_type).into());
        };

        for (coin, request_index) in split_coin_args
            .into_iter()
            .zip(requests.iter().map(|(index, _)| *index))
        {
            *builder.arguments.get_mut(&request_index).unwrap() =
                ResolvedArgument::ReplaceWith(coin);
        }

        Ok(())
    }

    async fn resolve_gas_coin(
        builder: &mut TransactionBuilder,
        client: &mut sui_rpc::Client,
        requests: &[(usize, u64)],
    ) -> Result<(), BoxError> {
        let sender = builder
            .sender()
            .ok_or("Sender must be set to resolve CoinWithBalance")?;

        if requests.is_empty() {
            return Err("BUG: requests is empty".into());
        }

        let sum = requests.iter().map(|(_, balance)| *balance).sum();

        let mut coins = client
            //TODO populate excludes
            .select_coins(&sender, &(StructTag::sui().into()), sum, &[])
            .await?
            .into_iter()
            .map(|coin| ObjectInput::try_from_object_proto(&coin))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();

        let gas = builder.gas();
        let mut deps = Vec::new();

        // Append to gas coin up to 250 coins
        builder
            .add_gas_objects((&mut coins).take(MAX_GAS_OBJECTS.saturating_sub(builder.gas.len())));

        // Any remaining do a merge coins
        let remaining = coins.map(|coin| builder.object(coin)).collect::<Vec<_>>();

        for chunk in remaining.chunks(MAX_ARGUMENTS) {
            builder.merge_coins(gas, chunk.to_vec());
            deps.push(Argument::new(*builder.commands.last_key_value().unwrap().0));
        }

        let amounts = requests
            .iter()
            .map(|(_, balance)| builder.pure(balance))
            .collect();
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

        for (coin, request_index) in split_coin_args
            .into_iter()
            .zip(requests.iter().map(|(index, _)| *index))
        {
            *builder.arguments.get_mut(&request_index).unwrap() =
                ResolvedArgument::ReplaceWith(coin);
        }

        Ok(())
    }
}
