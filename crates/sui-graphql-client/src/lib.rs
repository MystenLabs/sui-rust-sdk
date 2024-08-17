// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../README.md")]

pub mod graphql_types;

use base64ct::Encoding;
use graphql_types::{
    ChainIdentifierQuery, CheckpointArgs, CheckpointId, CheckpointQuery, CoinMetadata,
    CoinMetadataArgs, EpochSummaryArgs, EpochSummaryQuery, EventFilter, ObjectFilter, PageInfo,
    ProtocolConfigQuery, ProtocolConfigs, ProtocolVersionArgs, ServiceConfig, ServiceConfigQuery,
    TransactionBlockArgs, TransactionBlockQuery, Uint53,
};
use sui_types::types::{
    Address, CheckpointSequenceNumber, CheckpointSummary, Event, ObjectId, SignedTransaction,
};

use anyhow::{ensure, Error, Result};
use async_trait::async_trait;
use cynic::{serde, GraphQlResponse, Operation, QueryBuilder};
use reqwest::Client;

use crate::graphql_types::{
    CoinMetadataQuery, EventsQuery, EventsQueryArgs, ObjectsQuery, ObjectsQueryArgs,
};

const MAINNET_HOST: &str = "https://sui-mainnet.mystenlabs.com/graphql";
const TESTNET_HOST: &str = "https://sui-testnet.mystenlabs.com/graphql";
const DEVNET_HOST: &str = "https://sui-devnet.mystenlabs.com/graphql";
const LOCAL_HOST: &str = "http://localhost:9125/graphql";

/// The `HttpClient` trait defines the interface for the HTTP client used by the SuiClient to make
/// requests to the GraphQL server.
#[async_trait]
pub trait HttpClient {
    /// Send a POST request to the GraphQL server with the provided URL and query data.
    ///
    /// The response is deserialized into a [`cynic::GraphQlResponse`] object.
    async fn post<
        T: serde::de::DeserializeOwned + Send,
        V: serde::Serialize + Send + std::marker::Sync,
    >(
        &self,
        url: &str,
        operation: &Operation<T, V>,
    ) -> Result<GraphQlResponse<T>>;
}

/// An HTTP client based on reqwest
pub struct ReqwestHttpClient {
    inner: Client,
}

impl ReqwestHttpClient {
    fn new() -> Self {
        Self {
            inner: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl HttpClient for ReqwestHttpClient {
    async fn post<
        T: serde::de::DeserializeOwned + Send,
        V: serde::Serialize + Send + std::marker::Sync,
    >(
        &self,
        url: &str,
        operation: &Operation<T, V>,
    ) -> Result<GraphQlResponse<T>> {
        let res = self
            .inner
            .post(url)
            .json(&operation)
            .send()
            .await?
            .json::<GraphQlResponse<T>>()
            .await?;
        Ok(res)
    }
}

#[derive(Debug)]
pub struct Page<T> {
    page_info: PageInfo,
    nodes: Vec<T>,
}

/// The SuiClient is a GraphQL client for interacting with the Sui blockchain.
/// By default, it uses Reqwest as the HTTP client but a custom client can be provided by
/// implementing the `HttpClient` trait.
pub struct SuiClient<C: HttpClient> {
    /// The URL of the GraphQL server.
    rpc: String,
    /// GraphQL supports multiple versions of the service. By default, the client uses the stable
    /// version of the service.
    rpc_version: Option<String>,
    inner: C,
}

impl Default for SuiClient<ReqwestHttpClient> {
    /// Create a new [`SuiClient`] with testnet as the default GraphQL server.
    fn default() -> Self {
        Self {
            rpc: TESTNET_HOST.to_string(),
            rpc_version: None,
            inner: ReqwestHttpClient::new(),
        }
    }
}

impl<C: HttpClient> SuiClient<C> {
    // ===========================================================================
    // Client Misc API
    // ===========================================================================

    /// Create a new SuiClient with a custom HTTP client.
    pub fn new_with_http_client(http_client: C) -> Self {
        Self {
            rpc: TESTNET_HOST
                .parse()
                .expect("Cannot parse GraphQL server host"),
            rpc_version: None,
            inner: http_client,
        }
    }

    /// Set the server address for the GraphQL GraphQL client. It should be a valid URL with a host and
    /// optionally a port number.
    pub fn set_rpc_server(&mut self, server: &str) -> Result<(), anyhow::Error> {
        reqwest::Url::parse(server)?;
        self.rpc = server.to_string();
        Ok(())
    }

    /// Set the version for the GraphQL GraphQL client. The default set version is stable.
    ///
    /// Use the `service_config` API to get the `available_versions` field to know which versions
    /// are supported by this service.
    pub fn set_version(&mut self, version: Option<&str>) {
        self.rpc_version = version.map(|v| v.to_string());
    }

    /// Return the URL for the GraphQL GraphQL client after checking if the version is set.
    fn url(&self) -> String {
        if let Some(version) = &self.rpc_version {
            return format!("{}/{}", self.rpc, version);
        } else {
            return self.rpc.to_string();
        }
    }

    /// Set the GraphQL to localhost and port 9125.
    pub fn set_localhost(&mut self) -> &Self {
        self.rpc = LOCAL_HOST.to_string();
        self
    }

    /// Set GraphQL client to devnet.
    pub fn set_devnet(&mut self) -> &Self {
        self.rpc = DEVNET_HOST.to_string();
        self.set_version(None);
        self
    }

    /// Set GraphQL client to testnet.
    pub fn set_testnet(&mut self) -> &Self {
        self.rpc = TESTNET_HOST.to_string();
        self
    }

    /// Set GraphQL client to mainnet.
    pub fn set_mainnet(&mut self) -> &Self {
        self.rpc = MAINNET_HOST.to_string();
        self
    }

    /// Run a query on the GraphQL server and return the response.
    /// This method returns an [`cynic::GraphQlResponse`]  over the query type `T`, and it is
    /// intended to be used with custom queries.
    pub async fn run_query<
        T: serde::de::DeserializeOwned + Send,
        V: serde::Serialize + Send + std::marker::Sync,
    >(
        &self,
        operation: &Operation<T, V>,
    ) -> Result<GraphQlResponse<T>> {
        self.inner.post(&self.url(), &operation).await
    }

    // ===========================================================================
    // Objects API
    // ===========================================================================

    // TODO: implement
    /// Return An object's bcs content [`Vec<u8>`] based on the provided [ObjectID], or an error upon failure.
    pub async fn move_object_bcs(&self, _object_id: ObjectId) {
        todo!()
    }

    // TODO: implement
    pub async fn owned_objects(&self, _address: Address) {
        todo!()
    }

    // ===========================================================================
    // Network info API
    // ===========================================================================

    /// Get the chain identifier.
    pub async fn chain_id(&self) -> Result<String, Error> {
        let operation = ChainIdentifierQuery::build(());
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        response
            .data
            .map(|e| e.chain_identifier)
            .ok_or_else(|| Error::msg("No data in response"))
    }

    // TODO: implement
    pub async fn committee_info(&self, _epoch: Option<u64>) {
        todo!()
    }

    /// Get the reference gas price for the provided epoch or the last known one if no epoch is
    /// provided.
    ///
    /// This will return `Ok(None)` if the epoch requested is not available in the GraphQL service
    /// (e.g., due to prunning).
    pub async fn reference_gas_price(&self, epoch: Option<u64>) -> Result<Option<u64>, Error> {
        let operation = EpochSummaryQuery::build(EpochSummaryArgs {
            id: epoch.map(Uint53),
        });
        let response = self.run_query(&operation).await?;

        if let Some(data) = response.data {
            data.epoch
                .and_then(|e| e.reference_gas_price.map(|x| x.try_into()))
                .transpose()
        } else if let Some(errors) = response.errors {
            Err(Error::msg(format!("{:?}", errors)))
        } else {
            Err(Error::msg("No data in response"))
        }
    }

    // TODO: implement
    pub async fn protocol_config(
        &self,
        version: Option<u64>,
    ) -> Result<Option<ProtocolConfigs>, Error> {
        let operation = ProtocolConfigQuery::build(ProtocolVersionArgs {
            id: version.map(Uint53),
        });
        let response = self.run_query(&operation).await?;
        Ok(response.data.map(|p| p.protocol_config))
    }

    /// Get the GraphQL service configuration, including complexity limits, read and mutation limits,
    /// supported versions, and others.
    pub async fn service_config(&self) -> Result<ServiceConfig, Error> {
        let operation = ServiceConfigQuery::build(());
        let response = self.run_query(&operation).await?;

        response
            .data
            .map(|s| s.service_config)
            .ok_or_else(|| Error::msg("No data in response"))
    }

    // ===========================================================================
    // Balance API
    // ===========================================================================

    // TODO: implement
    /// Get the total balance of an address.
    ///
    pub async fn total_balance(&self, _address: Address) -> Result<Option<u64>, Error> {
        todo!()
    }

    // ===========================================================================
    // Coin API
    // ===========================================================================

    pub async fn coin_metadata(&self, coin_type: &str) -> Result<Option<CoinMetadata>, Error> {
        let operation = CoinMetadataQuery::build(CoinMetadataArgs { coin_type });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        Ok(response.data.map(|x| x.coin_metadata).flatten())
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

        let operation = CheckpointQuery::build(CheckpointArgs {
            id: CheckpointId {
                digest,
                sequence_number: seq_num.map(Uint53),
            },
        });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        response
            .data
            .map(|c| c.checkpoint.map(|c| c.try_into()).transpose())
            .ok_or_else(|| Error::msg("No data in response"))?
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
    /// requested is not available in the GraphQL service (e.g., due to prunning).
    pub async fn epoch_total_checkpoints(&self, epoch: Option<u64>) -> Result<Option<u64>, Error> {
        let response = self.epoch_summary(epoch).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        Ok(response
            .data
            .map(|d| d.epoch)
            .flatten()
            .map(|e| e.total_checkpoints)
            .flatten()
            .map(|x| x.into()))
    }

    /// Return the number of transaction blocks in this epoch. This will return `Ok(None)` if the
    /// epoch requested is not available in the GraphQL service (e.g., due to prunning).
    pub async fn epoch_total_transaction_blocks(
        &self,
        epoch: Option<u64>,
    ) -> Result<Option<u64>, Error> {
        let response = self.epoch_summary(epoch).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        Ok(response
            .data
            .map(|d| d.epoch)
            .flatten()
            .map(|e| e.total_transactions)
            .flatten()
            .map(|x| x.into()))
    }

    /// Internal method for getting the epoch summary that is called in a few other APIs for
    /// convenience.
    async fn epoch_summary(
        &self,
        epoch: Option<u64>,
    ) -> Result<GraphQlResponse<EpochSummaryQuery>, Error> {
        let operation = EpochSummaryQuery::build(EpochSummaryArgs {
            id: epoch.map(Uint53),
        });
        self.run_query(&operation).await
    }

    // ===========================================================================
    // Events API
    // ===========================================================================

    pub async fn events(
        &self,
        after: Option<String>,
        before: Option<String>,
        filter: Option<EventFilter>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Option<Page<Event>>, Error> {
        let operation = EventsQuery::build(EventsQueryArgs {
            after,
            before,
            filter,
            first,
            last,
        });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        println!("{:?}", response.data);

        // TODO bcs from bytes into Event fails due to custom type parser error
        // called `Result::unwrap()` on an `Err` value: Custom("TypeParseError")
        if let Some(events) = response.data {
            let ec = events.events;
            let page_info = ec.page_info;
            let nodes = ec
                .nodes
                .iter()
                .map(|e| base64ct::Base64::decode_vec(e.bcs.0.as_str()))
                .collect::<Result<Vec<_>, base64ct::Error>>()
                .map_err(|e| {
                    Error::msg(format!(
                        "Cannot decode Base64 event bcs bytes: {}",
                        e.to_string()
                    ))
                })?
                .iter()
                .map(|b| bcs::from_bytes::<Event>(&b))
                .collect::<Result<Vec<_>, bcs::Error>>()
                .map_err(|e| {
                    Error::msg(format!(
                        "Cannot decode bcs bytes into Event: {}",
                        e.to_string()
                    ))
                })?;

            Ok(Some(Page { page_info, nodes }))
        } else {
            Ok(None)
        }
    }

    // ===========================================================================
    // Objects API
    // ===========================================================================

    pub async fn objects(
        &self,
        after: Option<&str>,
        before: Option<&str>,
        filter: Option<ObjectFilter<'_>>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Option<Page<sui_types::types::Object>>, Error> {
        let operation = ObjectsQuery::build(ObjectsQueryArgs {
            after,
            before,
            filter,
            first,
            last,
        });

        let response = self.run_query(&operation).await?;
        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(objects) = response.data {
            let oc = objects.objects;
            let page_info = oc.page_info;
            let bcs = oc
                .nodes
                .iter()
                .map(|o| &o.bcs)
                .map(|b64| {
                    b64.as_ref()
                        .map(|b| base64ct::Base64::decode_vec(b.0.as_str()))
                })
                .flatten()
                .collect::<Result<Vec<_>, base64ct::Error>>()
                .map_err(|e| {
                    Error::msg(format!(
                        "Cannot decode Base64 object bcs bytes: {}",
                        e.to_string()
                    ))
                })?;
            let objects = bcs
                .iter()
                .map(|b| bcs::from_bytes::<sui_types::types::Object>(&b))
                .collect::<Result<Vec<_>, bcs::Error>>()
                .map_err(|e| {
                    Error::msg(format!(
                        "Cannot decode bcs bytes into Object: {}",
                        e.to_string()
                    ))
                })?;

            Ok(Some(Page {
                page_info,
                nodes: objects,
            }))
        } else {
            Ok(None)
        }
    }

    // ===========================================================================
    // Transaction API
    // ===========================================================================

    // TODO: From Brandon: this fails due to SignedTransaction in Sui core type being technically inaccurate but it is fixed in this SDK here. in particular core incorrectly appends the signing intent when it shouldn't so my guess is that's whats wrong
    /// Get a transaction by its digest.
    pub async fn transaction(&self, digest: &str) -> Result<Option<SignedTransaction>, Error> {
        let operation = TransactionBlockQuery::build(TransactionBlockArgs {
            digest: digest.to_string(),
        });
        let response = self.run_query(&operation).await?;

        let signed_tx = response
            .data
            .map(|tbq| tbq.transaction_block)
            .flatten()
            .map(|tb| tb.bcs)
            .flatten()
            .map(|bcs| bcs::from_bytes::<SignedTransaction>(bcs.0.as_bytes()).unwrap());
        Ok(signed_tx)
    }
}
