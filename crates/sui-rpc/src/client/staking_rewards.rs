use prost_types::FieldMask;
use sui_sdk_types::Address;

use crate::field::FieldMaskUtil;
use crate::proto::sui::rpc::v2beta2::Argument;
use crate::proto::sui::rpc::v2beta2::GetObjectRequest;
use crate::proto::sui::rpc::v2beta2::Input;
use crate::proto::sui::rpc::v2beta2::ListOwnedObjectsRequest;
use crate::proto::sui::rpc::v2beta2::MoveCall;
use crate::proto::sui::rpc::v2beta2::Object;
use crate::proto::sui::rpc::v2beta2::ProgrammableTransaction;
use crate::proto::sui::rpc::v2beta2::SimulateTransactionRequest;
use crate::proto::sui::rpc::v2beta2::Transaction;
use crate::proto::sui::rpc::v2beta2::simulate_transaction_request::TransactionChecks;

use super::Client;
use super::Result;

#[derive(Debug)]
pub struct DelegatedStake {
    /// ObjectId of this StakedSui delegation.
    pub staked_sui_id: Address,
    /// Validator's Address.
    pub validator_address: Address,
    /// Staking pool object id.
    pub staking_pool: Address,
    /// The epoch at which the stake becomes active.
    pub activation_epoch: u64,
    /// The staked SUI tokens.
    pub principal: u64,
    /// The accrued rewards.
    pub rewards: u64,
}

#[derive(serde::Deserialize, Debug)]
struct StakedSui {
    id: Address,
    /// ID of the staking pool we are staking with.
    pool_id: Address,
    /// The epoch at which the stake becomes active.
    stake_activation_epoch: u64,
    /// The staked SUI tokens.
    principal: u64,
}

impl Client {
    pub async fn get_delegated_stake(&mut self, staked_sui_id: &Address) -> Result<DelegatedStake> {
        let maybe_staked_sui = self
            .ledger_client()
            .get_object(GetObjectRequest {
                object_id: Some(staked_sui_id.to_string()),
                version: None,
                read_mask: Some(FieldMask::from_str("contents")),
            })
            .await?
            .into_inner()
            .object
            .unwrap_or_default();

        let mut stakes = self
            .try_create_delegated_stake_info(&[maybe_staked_sui])
            .await?;
        Ok(stakes.remove(0))
    }

    pub async fn list_delegated_stake(&mut self, address: &Address) -> Result<Vec<DelegatedStake>> {
        const STAKED_SUI_TYPE: &str = "0x3::staking_pool::StakedSui";

        let mut delegated_stakes = Vec::new();

        let mut list_request = ListOwnedObjectsRequest {
            owner: Some(address.to_string()),
            page_size: Some(500),
            page_token: None,
            read_mask: Some(FieldMask::from_str("contents")),
            object_type: Some(STAKED_SUI_TYPE.to_owned()),
        };

        loop {
            let response = self
                .live_data_client()
                .list_owned_objects(list_request.clone())
                .await?
                .into_inner();

            // with the fetched StakedSui objects, attempt to calculate the rewards and create a
            // DelegatedStake for each.
            delegated_stakes.extend(
                self.try_create_delegated_stake_info(&response.objects)
                    .await?,
            );

            // If there are no more pages then we can break, otherwise update the page_token for
            // the next request
            if response.next_page_token.is_none() {
                break;
            } else {
                list_request.page_token = response.next_page_token;
            }
        }

        Ok(delegated_stakes)
    }

    async fn try_create_delegated_stake_info(
        &mut self,
        maybe_staked_sui: &[Object],
    ) -> Result<Vec<DelegatedStake>> {
        let staked_suis = maybe_staked_sui
            .iter()
            .map(|o| {
                o.contents
                    .clone() // Avoid clone probably with better getters
                    .unwrap_or_default()
                    .deserialize::<StakedSui>()
                    .map_err(Into::into)
                    .map_err(tonic::Status::from_error)
            })
            .collect::<Result<Vec<StakedSui>>>()?;

        let ids = staked_suis.iter().map(|s| s.id).collect::<Vec<_>>();
        let pool_ids = staked_suis.iter().map(|s| s.pool_id).collect::<Vec<_>>();

        let rewards = self.calculate_rewards(&ids).await?;
        let validator_addresses = self.get_validator_address_by_pool_id(&pool_ids).await?;

        Ok(staked_suis
            .into_iter()
            .zip(rewards)
            .zip(validator_addresses)
            .map(
                |((staked_sui, (_id, rewards)), (_pool_id, validator_address))| DelegatedStake {
                    staked_sui_id: staked_sui.id,
                    validator_address,
                    staking_pool: staked_sui.pool_id,
                    activation_epoch: staked_sui.stake_activation_epoch,
                    principal: staked_sui.principal,
                    rewards,
                },
            )
            .collect())
    }

    async fn calculate_rewards(
        &mut self,
        staked_sui_ids: &[Address],
    ) -> Result<Vec<(Address, u64)>> {
        let mut ptb = ProgrammableTransaction {
            inputs: vec![Input {
                object_id: Some("0x5".into()),
                ..Default::default()
            }],
            commands: vec![],
        };
        let system_object = Argument::new_input(0);

        for id in staked_sui_ids {
            let staked_sui = Argument::new_input(ptb.inputs.len() as u16);

            ptb.inputs.push(Input {
                object_id: Some(id.to_string()),
                ..Default::default()
            });

            ptb.commands.push(
                MoveCall {
                    package: Some("0x3".to_owned()),
                    module: Some("sui_system".to_owned()),
                    function: Some("calculate_rewards".to_owned()),
                    type_arguments: vec![],
                    arguments: vec![system_object, staked_sui],
                }
                .into(),
            );
        }

        let transaction = Transaction {
            kind: Some(ptb.into()),
            sender: Some("0x0".into()),
            ..Default::default()
        };

        let resp = self
            .live_data_client()
            .simulate_transaction(SimulateTransactionRequest {
                transaction: Some(transaction),
                read_mask: Some(FieldMask::from_paths([
                    "outputs.return_values.value",
                    "transaction.effects.status",
                ])),
                checks: Some(TransactionChecks::Disabled as _),
                ..Default::default()
            })
            .await?
            .into_inner();

        if !resp
            .transaction
            .as_ref()
            .and_then(|t| t.effects.as_ref().and_then(|e| e.status.as_ref()))
            .is_some_and(|s| s.success())
        {
            return Err(tonic::Status::from_error(
                "transaction execution failed".into(),
            ));
        }

        if staked_sui_ids.len() != resp.outputs.len() {
            return Err(tonic::Status::from_error(
                "missing transaction command outputs".into(),
            ));
        }

        let mut rewards = Vec::with_capacity(staked_sui_ids.len());

        for (id, output) in staked_sui_ids.iter().zip(resp.outputs) {
            let bcs_rewards = output
                .return_values
                .first()
                .and_then(|o| o.value.as_ref())
                .ok_or_else(|| tonic::Status::from_error("missing bcs".into()))?;

            let reward =
                if bcs_rewards.name() == "u64" && bcs_rewards.value().len() == size_of::<u64>() {
                    u64::from_le_bytes(bcs_rewards.value().try_into().unwrap())
                } else {
                    return Err(tonic::Status::from_error("missing rewards".into()));
                };
            rewards.push((*id, reward));
        }

        Ok(rewards)
    }

    async fn get_validator_address_by_pool_id(
        &mut self,
        pool_ids: &[Address],
    ) -> Result<Vec<(Address, Address)>> {
        let mut ptb = ProgrammableTransaction {
            inputs: vec![Input {
                object_id: Some("0x5".into()),
                ..Default::default()
            }],
            commands: vec![],
        };
        let system_object = Argument::new_input(0);

        for id in pool_ids {
            let pool_id = Argument::new_input(ptb.inputs.len() as u16);

            ptb.inputs.push(Input {
                pure: Some(id.into_inner().to_vec().into()),
                ..Default::default()
            });

            ptb.commands.push(
                MoveCall {
                    package: Some("0x3".to_owned()),
                    module: Some("sui_system".to_owned()),
                    function: Some("validator_address_by_pool_id".to_owned()),
                    type_arguments: vec![],
                    arguments: vec![system_object, pool_id],
                }
                .into(),
            );
        }

        let transaction = Transaction {
            kind: Some(ptb.into()),
            sender: Some("0x0".into()),
            ..Default::default()
        };

        let resp = self
            .live_data_client()
            .simulate_transaction(SimulateTransactionRequest {
                transaction: Some(transaction),
                read_mask: Some(FieldMask::from_paths([
                    "outputs.return_values.value",
                    "transaction.effects.status",
                ])),
                checks: Some(TransactionChecks::Disabled as _),
                ..Default::default()
            })
            .await?
            .into_inner();

        if !resp
            .transaction
            .as_ref()
            .and_then(|t| t.effects.as_ref().and_then(|e| e.status.as_ref()))
            .is_some_and(|s| s.success())
        {
            return Err(tonic::Status::from_error(
                "transaction execution failed".into(),
            ));
        }

        if pool_ids.len() != resp.outputs.len() {
            return Err(tonic::Status::from_error(
                "missing transaction command outputs".into(),
            ));
        }

        let mut addresses = Vec::with_capacity(pool_ids.len());

        for (id, output) in pool_ids.iter().zip(resp.outputs) {
            let validator_address = output
                .return_values
                .first()
                .and_then(|o| o.value.as_ref())
                .ok_or_else(|| tonic::Status::from_error("missing bcs".into()))?;

            let address = if validator_address.name() == "address"
                && validator_address.value().len() == Address::LENGTH
            {
                Address::from_bytes(validator_address.value())
                    .map_err(|e| tonic::Status::from_error(e.into()))?
            } else {
                return Err(tonic::Status::from_error("missing address".into()));
            };
            addresses.push((*id, address));
        }

        Ok(addresses)
    }
}
