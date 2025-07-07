use prost_types::FieldMask;
use sui_sdk_types::{Address, ObjectId};

use crate::field::FieldMaskUtil;
use crate::proto::sui::rpc::v2beta2::simulate_transaction_request::TransactionChecks;
use crate::proto::sui::rpc::v2beta2::{
    Argument, GetObjectRequest, Input, MoveCall, ProgrammableTransaction,
};
use crate::proto::sui::rpc::v2beta2::{SimulateTransactionRequest, Transaction};

use super::Client;
use super::Result;

#[derive(Debug)]
pub struct DelegatedStake {
    pub staked_sui_id: ObjectId,
    /// Validator's Address.
    pub validator_address: Address,
    /// Staking pool object id.
    pub staking_pool: ObjectId,
    pub activation_epoch: u64,
    pub principal: u64,
    pub rewards: u64,
}

#[derive(serde::Deserialize, Debug)]
struct StakedSui {
    id: ObjectId,
    /// ID of the staking pool we are staking with.
    pool_id: ObjectId,
    /// The epoch at which the stake becomes active.
    stake_activation_epoch: u64,
    /// The staked SUI tokens.
    principal: u64,
}

impl Client {
    pub async fn get_delegated_stake(
        &mut self,
        staked_sui_id: &ObjectId,
    ) -> Result<DelegatedStake> {
        let staked_sui = {
            let resp = self
                .ledger_client()
                .get_object(GetObjectRequest {
                    object_id: Some(staked_sui_id.to_string()),
                    version: None,
                    read_mask: Some(FieldMask::from_str("contents")),
                })
                .await?
                .into_inner();
            resp.object
                .unwrap_or_default()
                .contents
                .unwrap_or_default()
                .deserialize::<StakedSui>()
                .map_err(Into::into)
                .map_err(tonic::Status::from_error)?
        };

        let rewards = self.calculate_rewards(&[staked_sui.id]).await?[0].1;
        let validator_address = self
            .get_validator_address_by_pool_id(&[staked_sui.pool_id])
            .await?[0]
            .1;

        Ok(DelegatedStake {
            staked_sui_id: staked_sui.id,
            validator_address,
            staking_pool: staked_sui.pool_id,
            activation_epoch: staked_sui.stake_activation_epoch,
            principal: staked_sui.principal,
            rewards,
        })
    }

    pub async fn list_delegated_stake(&mut self, address: &Address) -> Result<Vec<DelegatedStake>> {
        todo!()
    }

    async fn calculate_rewards(
        &mut self,
        staked_sui_ids: &[ObjectId],
    ) -> Result<Vec<(ObjectId, u64)>> {
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
        pool_ids: &[ObjectId],
    ) -> Result<Vec<(ObjectId, Address)>> {
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
            let bcs_rewards = output
                .return_values
                .first()
                .and_then(|o| o.value.as_ref())
                .ok_or_else(|| tonic::Status::from_error("missing bcs".into()))?;

            let address = if bcs_rewards.name() == "address"
                && bcs_rewards.value().len() == Address::LENGTH
            {
                Address::from_bytes(bcs_rewards.value())
                    .map_err(|e| tonic::Status::from_error(e.into()))?
            } else {
                return Err(tonic::Status::from_error("missing address".into()));
            };
            addresses.push((*id, address));
        }

        Ok(addresses)
    }
}
