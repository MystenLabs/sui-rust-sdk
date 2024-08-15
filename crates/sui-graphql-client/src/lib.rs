// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

pub mod client;
pub mod graphql_types;

use graphql_types::{
    ChainIdentifierQuery, CheckpointArgs, CheckpointId, CheckpointQuery, EpochSummaryArgs,
    EpochSummaryQuery, ProtocolConfigQuery, ProtocolConfigs, ProtocolVersionArgs, ServiceConfig,
    ServiceConfigQuery, TransactionBlockArgs, TransactionBlockQuery, Uint53,
};
use sui_types::types::{
    Address, CheckpointSequenceNumber, CheckpointSummary, ObjectId, SignedTransaction,
};

use anyhow::{ensure, Error};
use cynic::QueryBuilder;
use reqwest::Client;

#[derive(Clone, Debug)]
pub struct GraphQLClient {
    rpc_server: String,
    rpc_version: Option<String>,
    url: String,
    http_client: Client,
}

#[derive(Clone, Debug)]
pub struct Api {
    graphql_client: GraphQLClient,
}

pub struct WriteApi<'a> {
    graphql_client: &'a GraphQLClient,
}

impl<'a> WriteApi<'a> {
    pub(crate) fn new(graphql_client: &'a GraphQLClient) -> Self {
        Self { graphql_client }
    }
}

/// Call the RPC service with the typed query information and additional arguments as needed.
///
/// Examples
/// ```rust
///
/// #[derive(cynic::QueryFragment, Debug)]
/// #[cynic(schema = "rpc", graphql_type = "Query")]
/// pub struct ChainIdentifierQuery {
///    pub chain_identifier: String,
/// }
///
/// #[derive(cynic::QueryVariables, Debug)]
/// pub struct EpochSummaryArgs {
///     pub id: Option<Uint53>,
/// }
///
/// #[derive(cynic::QueryFragment, Debug)]
/// #[cynic(schema = "rpc", graphql_type = "Query", variables = "EpochSummaryArgs")]
/// pub struct EpochSummaryQuery {
///     #[arguments(id: $id)]
///     pub epoch: Option<EpochSummary>,
/// }
///
/// fn main() {
///     let client = SuiClient::new();
///     let api = client.read_api();
///     let chain_id = execute_graphql_query(&api, ChainIdentifierQuery, ());
///     let epoch = execute_graphql_query!(
///         &api,
///         EpochSummaryQuery,
///         EpochSummaryArgs {
///             id: epoch.map(Uint53)
///         }
///     );
/// }
/// ```
#[macro_export]
macro_rules! execute_graphql_query {
    ($self:ident, $type:ty, $operation:expr) => {{
        let operation = <$type>::build($operation);

        let response = $self
            .http_client()
            .post(&$self.url())
            .json(&operation)
            .send()
            .await?
            .json::<cynic::GraphQlResponse<$type>>()
            .await?;

        response
    }};
}

impl Api {
    pub(crate) fn new(graphql_client: GraphQLClient) -> Self {
        Self { graphql_client }
    }

    pub fn graphql_client(&self) -> &GraphQLClient {
        &self.graphql_client
    }

    pub fn http_client(&self) -> &Client {
        &self.graphql_client.http_client
    }

    pub fn url(&self) -> String {
        self.graphql_client.url()
    }

    // ===========================================================================
    // Objects API
    // ===========================================================================

    // TODO: implement
    /// Return An object's bcs content [`Vec<u8>`] based on the provided [ObjectID], or an error upon failure.
    pub async fn move_object_bcs(&self, object_id: ObjectId) {
        todo!()
    }

    // TODO: implement
    pub async fn owned_objects(&self, address: Address) {
        todo!()
    }

    // ===========================================================================
    // Network info API
    // ===========================================================================

    /// Get the chain identifier.
    pub async fn chain_id(&self) -> Result<String, Error> {
        execute_graphql_query!(self, ChainIdentifierQuery, ())
            .data
            .map(|e| e.chain_identifier)
            .ok_or_else(|| Error::msg("No data in response"))
    }

    // TODO: implement
    pub async fn committee_info(&self, epoch: Option<u64>) {
        todo!()
    }

    /// Get the reference gas price for the provided epoch or the last known one if no epoch is
    /// provided.
    ///
    /// This will return `Ok(None)` if the epoch requested is not available in the RPC service
    /// (e.g., due to prunning).
    pub async fn reference_gas_price(&self, epoch: Option<u64>) -> Result<Option<u64>, Error> {
        let response = execute_graphql_query!(
            self,
            EpochSummaryQuery,
            EpochSummaryArgs {
                id: epoch.map(Uint53)
            }
        );

        if let Some(data) = response.data {
            data.epoch
                .and_then(|e| e.reference_gas_price.map(|x| x.try_into()))
                .transpose()
        } else {
            Err(Error::msg("No data in response"))
        }
    }

    // TODO: implement
    pub async fn protocol_config(
        &self,
        version: Option<u64>,
    ) -> Result<Option<ProtocolConfigs>, Error> {
        Ok(execute_graphql_query!(
            self,
            ProtocolConfigQuery,
            ProtocolVersionArgs {
                id: version.map(Uint53)
            }
        )
        .data
        .map(|p| p.protocol_config))
    }

    /// Get the RPC service configuration, including complexity limits, read and mutation limits,
    /// supported versions, and others.
    pub async fn service_config(&self) -> Result<ServiceConfig, Error> {
        execute_graphql_query!(self, ServiceConfigQuery, ())
            .data
            .map(|s| s.service_config)
            .ok_or_else(|| Error::msg("No data in response"))
    }

    // ===========================================================================
    // Checkpoints API
    // ===========================================================================

    /// Get the `CheckpointSummary` for a given checkpoint digest or checkpoint id. If none is
    /// provided, it will use the last known checkpoint id.
    pub async fn checkpoint(
        &self,
        digest: Option<String>,
        seq_num: Option<u64>,
    ) -> Result<Option<CheckpointSummary>, Error> {
        ensure!(
            digest.is_some() != seq_num.is_some(),
            "Either digest or seq_num must be provided"
        );

        let response = execute_graphql_query!(
            self,
            CheckpointQuery,
            CheckpointArgs {
                id: CheckpointId {
                    digest,
                    sequence_number: seq_num.map(Uint53)
                }
            }
        );

        if let Some(data) = response.data {
            data.checkpoint.map(|c| c.try_into()).transpose()
        } else {
            Ok(None)
        }
    }

    // TODO: implement
    pub async fn checkpoints(&self) {
        todo!()
    }

    /// Return the sequence number of the latest checkpoint that has been executed.  
    pub async fn latest_checkpoint_sequence_number(
        &self,
    ) -> Result<Option<CheckpointSequenceNumber>, Error> {
        Ok(self
            .checkpoint(None, None)
            .await?
            .map(|c| c.sequence_number))
    }

    // ===========================================================================
    // Epoch API
    // ===========================================================================

    /// Return the number of checkpoints in this epoch. This will return `Ok(None)` if the epoch
    /// requested is not available in the RPC service (e.g., due to prunning).
    pub async fn epoch_total_checkpoints(&self, epoch: Option<u64>) -> Result<Option<u64>, Error> {
        let response = self.epoch_summary(epoch).await?;

        if let Some(data) = response {
            Ok(data
                .epoch
                .and_then(|e| e.total_checkpoints.map(|x| x.into())))
        } else {
            Ok(None)
        }
    }

    /// Return the number of transaction blocks in this epoch. This will return `Ok(None)` if the
    /// epoch requested is not available in the RPC service (e.g., due to prunning).
    pub async fn epoch_total_transaction_blocks(
        &self,
        epoch: Option<u64>,
    ) -> Result<Option<u64>, Error> {
        let response = self.epoch_summary(epoch).await?;

        if let Some(data) = response {
            Ok(data
                .epoch
                .and_then(|e| e.total_transactions.map(|x| x.into())))
        } else {
            Ok(None)
        }
    }

    /// Internal method for getting the epoch summary that is called in a few other APIs for
    /// convenience.
    async fn epoch_summary(&self, epoch: Option<u64>) -> Result<Option<EpochSummaryQuery>, Error> {
        Ok(execute_graphql_query!(
            self,
            EpochSummaryQuery,
            EpochSummaryArgs {
                id: epoch.map(Uint53)
            }
        )
        .data)
    }

    // ===========================================================================
    // Transaction API
    // ===========================================================================

    // pub async fn transaction(&self, digest: String) -> Result<Option<SignedTransaction>, Error> {
    //     let response =
    //         execute_graphql_query!(self, TransactionBlockQuery, TransactionBlockArgs { digest });
    //
    //     let signed_tx = response
    //         .data
    //         .map(|tbq| tbq.transaction_block)
    //         .flatten()
    //         .map(|tb| tb.bcs)
    //         .flatten()
    //         .map(|bcs| from_bytes::<SignedTransaction>(bcs.0.as_bytes()).unwrap());
    //     Ok(signed_tx)
    // }
}

impl GraphQLClient {
    /// Initialize the GraphQL client with the testnet server and stable version.
    pub fn new(
        rpc_server: String,
        rpc_version: Option<String>,
        http_client: reqwest::Client,
    ) -> Self {
        let url = if let Some(version) = &rpc_version {
            format!("{}/{}", rpc_server, version)
        } else {
            rpc_server.clone()
        };
        GraphQLClient {
            rpc_server,
            rpc_version,
            url,
            http_client,
        }
    }

    fn url(&self) -> String {
        if let Some(version) = &self.rpc_version {
            format!("{}/{}", self.rpc_server, version)
        } else {
            self.rpc_server.clone()
        }
    }

    /// Set the server address for the GraphQL client.
    pub(crate) fn set_rpc_server(&mut self, server: &str) {
        self.rpc_server = server.to_string();
    }

    /// Set the version for the GraphQL client. The default version is stable.
    ///
    /// By default, the GraphQL service can serve three versions: stable, beta, and legacy. Stable
    /// version does not receive schema updates, legacy only receives critical bug-fixes, and beta
    /// corresponds to the devnet network and it is deployed with latest changes every week.
    pub(crate) fn set_version(&mut self, version: &str) {
        self.rpc_version = Some(version.to_string());
    }
}
