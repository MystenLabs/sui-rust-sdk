// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../README.md")]

pub mod query_types;

use base64ct::Encoding;
use query_types::{
    ChainIdentifierQuery, CheckpointArgs, CheckpointId, CheckpointQuery, CoinMetadata,
    CoinMetadataArgs, CoinMetadataQuery, EpochSummaryArgs, EpochSummaryQuery, EventFilter,
    EventsQuery, EventsQueryArgs, ObjectFilter, ObjectQuery, ObjectQueryArgs, ObjectsQuery,
    ObjectsQueryArgs, PageInfo, ProtocolConfigQuery, ProtocolConfigs, ProtocolVersionArgs,
    ServiceConfig, ServiceConfigQuery, TransactionBlockArgs, TransactionBlockQuery,
    TransactionBlocksQuery, TransactionBlocksQueryArgs, TransactionsFilter, Uint53,
};
use reqwest::Url;
use sui_types::types::{
    Address, CheckpointSequenceNumber, CheckpointSummary, Event, Object, SignedTransaction,
};

use anyhow::{anyhow, ensure, Error, Result};
use cynic::{serde, GraphQlResponse, Operation, QueryBuilder};

const MAINNET_HOST: &str = "https://sui-mainnet.mystenlabs.com/graphql";
const TESTNET_HOST: &str = "https://sui-testnet.mystenlabs.com/graphql";
const DEVNET_HOST: &str = "https://sui-devnet.mystenlabs.com/graphql";
const DEFAULT_LOCAL_HOST: &str = "http://localhost:9125/graphql";
static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug)]
/// A page of items returned by the GraphQL server.
pub struct Page<T> {
    /// Information about the page, such as the cursor and whether there are more pages.
    page_info: PageInfo,
    /// The data returned by the server.
    data: Vec<T>,
}

impl<T> Page<T> {
    pub fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    fn new(page_info: PageInfo, data: Vec<T>) -> Self {
        Self { page_info, data }
    }
}

/// The GraphQL client for interacting with the Sui blockchain.
/// By default, it uses the `reqwest` crate as the HTTP client.
pub struct Client {
    /// The URL of the GraphQL server.
    rpc: Url,
    /// GraphQL supports multiple versions of the service. By default, the client uses the stable
    /// version of the service.
    rpc_version: Option<String>,
    inner: reqwest::Client,
}

impl Client {
    // ===========================================================================
    // Client Misc API
    // ===========================================================================

    /// Create a new GraphQL client with the provided server address.
    pub fn new(server: &str) -> Result<Self, Error> {
        let rpc = reqwest::Url::parse(server).map_err(|_| anyhow!("Invalid URL: {}", server))?;

        let client = Client {
            rpc,
            rpc_version: None,
            inner: reqwest::Client::builder().user_agent(USER_AGENT).build()?,
        };
        Ok(client)
    }

    /// Create a new GraphQL client connected to the `mainnet` GraphQL server: {MAINNET_HOST}.
    pub fn new_mainnet() -> Self {
        Self::new(MAINNET_HOST).expect("Invalid mainnet URL")
    }

    /// Create a new GraphQL client connected to the `testnet` GraphQL server: {TESTNET_HOST}.
    pub fn new_testnet() -> Self {
        Self::new(TESTNET_HOST).expect("Invalid testnet URL")
    }

    /// Create a new GraphQL client connected to the `devnet` GraphQL server: {DEVNET_HOST}.
    pub fn new_devnet() -> Self {
        Self::new(DEVNET_HOST).expect("Invalid devnet URL")
    }

    /// Create a new GraphQL client connected to the `localhost` GraphQL server:
    /// {DEFAULT_LOCAL_HOST}.
    pub fn new_localhost(&mut self) -> Self {
        Self::new(DEFAULT_LOCAL_HOST).expect("Invalid localhost URL")
    }

    /// Set the version for the GraphQL GraphQL client. The default set version is stable.
    ///
    /// Use the `service_config` API to get the `available_versions` field to know which versions
    /// are supported by this service.
    pub fn set_version(&mut self, version: Option<&str>) -> Result<&mut Self, Error> {
        match (&self.rpc_version, version) {
            // prev version needs to be switched with the new requested version
            (Some(_), Some(new_v)) => {
                let mut segments = self.rpc.path_segments_mut().map_err(|_| {
                    anyhow!("Cannot get the path segments to add a version to the rpc URL")
                })?;
                segments.pop();
                segments.push(new_v);
            }
            // no previous set version, just set the requested version
            (None, Some(new_v)) => {
                println!("New version: {}", new_v);
                let mut segments = self.rpc.path_segments_mut().map_err(|_| {
                    anyhow!("Cannot get the path segments to add a version to the rpc URL")
                })?;
                segments.push(new_v);
            }
            // request to set version to nothing
            (Some(_), None) => {
                let mut segments = self.rpc.path_segments_mut().map_err(|_| {
                    anyhow!("Cannot get the path segments to add a version to the rpc URL")
                })?;
                segments.pop();
            }
            (None, None) => { /* no_op */ }
        }
        self.rpc_version = version.map(|v| v.to_string());
        Ok(self)
    }

    /// Set the server address for the GraphQL GraphQL client. It should be a valid URL with a host and
    /// optionally a port number.
    pub fn set_rpc_server(&mut self, server: &str) -> Result<&mut Self, Error> {
        let rpc = reqwest::Url::parse(server)?;
        self.rpc = rpc;
        Ok(self)
    }

    /// Get the version if it is set, or None otherwise.
    pub fn version(&self) -> Option<&str> {
        self.rpc_version.as_deref()
    }

    /// Return the URL for the GraphQL server.
    fn rpc_server(&self) -> &str {
        self.rpc.as_str()
    }

    /// Run a query on the GraphQL server and return the response.
    /// This method returns [`cynic::GraphQlResponse`]  over the query type `T`, and it is
    /// intended to be used with custom queries.
    pub async fn run_query<T, V>(&self, operation: &Operation<T, V>) -> Result<GraphQlResponse<T>>
    where
        T: serde::de::DeserializeOwned,
        V: serde::Serialize,
    {
        let res = self
            .inner
            .post(self.rpc_server())
            .json(&operation)
            .send()
            .await?
            .json::<GraphQlResponse<T>>()
            .await?;
        Ok(res)
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

    /// Get the protocol configuration.
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
    /// Get the total balance of an address for a coin type.
    pub async fn balance(
        &self,
        _address: Address,
        _coin_type: Option<&str>,
    ) -> Result<Option<u128>, Error> {
        todo!()
    }

    // ===========================================================================
    // Coin API
    // ===========================================================================

    // TODO: implement
    pub async fn coins(
        &self,
        _address: Address,
        _coin_type: Option<&str>,
    ) -> Result<Option<Page<Object>>, Error> {
        Ok(None)
    }

    // TODO: implement
    pub async fn all_coins(&self, _address: Address) -> Result<Option<Page<Object>>, Error> {
        Ok(None)
    }

    pub async fn coin_metadata(&self, coin_type: &str) -> Result<Option<CoinMetadata>, Error> {
        let operation = CoinMetadataQuery::build(CoinMetadataArgs { coin_type });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        Ok(response.data.and_then(|x| x.coin_metadata))
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

    // TODO: implement, does not have bcs field in GraphQL
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
            .and_then(|d| d.epoch)
            .and_then(|e| e.total_checkpoints)
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
            .and_then(|d| d.epoch)
            .and_then(|e| e.total_transactions)
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
                .map_err(|e| Error::msg(format!("Cannot decode Base64 event bcs bytes: {e}",)))?
                .iter()
                .map(|b| bcs::from_bytes::<Event>(b))
                .collect::<Result<Vec<_>, bcs::Error>>()
                .map_err(|e| Error::msg(format!("Cannot decode bcs bytes into Event: {e}",)))?;

            Ok(Some(Page::new(page_info, nodes)))
        } else {
            Ok(None)
        }
    }

    // ===========================================================================
    // Objects API
    // ===========================================================================

    /// Return an object based on the provided [`Address`].
    ///
    /// If the object does not exist (e.g., due to prunning), this will return `Ok(None)`.
    /// Similarly, if this is not an object but an address, it will return `Ok(None)`.
    pub async fn object(
        &self,
        address: Address,
        version: Option<u64>,
    ) -> Result<Option<Object>, Error> {
        let operation = ObjectQuery::build(ObjectQueryArgs {
            address: address.into(),
            version: version.map(Uint53),
        });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(object) = response.data {
            let obj = object.object;
            let bcs = obj
                .and_then(|o| o.bcs)
                .map(|bcs| base64ct::Base64::decode_vec(bcs.0.as_str()))
                .transpose()
                .map_err(|e| Error::msg(format!("Cannot decode Base64 object bcs bytes: {e}",)))?;
            let object = bcs
                .map(|b| bcs::from_bytes::<sui_types::types::Object>(&b))
                .transpose()
                .map_err(|e| Error::msg(format!("Cannot decode bcs bytes into Object: {e}",)))?;

            Ok(object)
        } else {
            Ok(None)
        }
    }

    /// Return a page of objects based on the provided parameters.
    ///
    /// Use this function together with the [`ObjectFilter::owner`] to get the objects owned by an
    /// address.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let filter = ObjectFilter {
    ///     type_: None,
    ///     owner: Some(Address::from_str("test").unwrap().into()),
    ///     object_ids: None,
    ///     object_keys: None,
    /// };
    ///
    /// let owned_objects = client.objects(None, None, Some(filter), None, None).await;
    /// ```
    pub async fn objects(
        &self,
        after: Option<&str>,
        before: Option<&str>,
        filter: Option<ObjectFilter<'_>>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Option<Page<Object>>, Error> {
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
                .filter_map(|b64| {
                    b64.as_ref()
                        .map(|b| base64ct::Base64::decode_vec(b.0.as_str()))
                })
                .collect::<Result<Vec<_>, base64ct::Error>>()
                .map_err(|e| Error::msg(format!("Cannot decode Base64 object bcs bytes: {e}")))?;
            let objects = bcs
                .iter()
                .map(|b| bcs::from_bytes::<sui_types::types::Object>(b))
                .collect::<Result<Vec<_>, bcs::Error>>()
                .map_err(|e| Error::msg(format!("Cannot decode bcs bytes into Object: {e}")))?;

            Ok(Some(Page::new(page_info, objects)))
        } else {
            Ok(None)
        }
    }

    /// Return the object's bcs content [`Vec<u8>`] based on the provided [`Address`].
    pub async fn object_bcs(&self, object_id: Address) -> Result<Option<Vec<u8>>, Error> {
        let operation = ObjectQuery::build(ObjectQueryArgs {
            address: object_id.into(),
            version: None,
        });

        let response = self.run_query(&operation).await.unwrap();

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(object) = response.data.map(|d| d.object) {
            object
                .and_then(|o| o.bcs)
                .map(|bcs| base64ct::Base64::decode_vec(bcs.0.as_str()))
                .transpose()
                .map_err(|e| Error::msg(format!("Cannot decode Base64 object bcs bytes: {e}")))
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
            .and_then(|tbq| tbq.transaction_block)
            .and_then(|tb| tb.bcs)
            .map(|bcs| bcs::from_bytes::<SignedTransaction>(bcs.0.as_bytes()).unwrap());
        Ok(signed_tx)
    }

    /// Get a page of transactions based on the provided filters.
    pub async fn transactions(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        filter: Option<TransactionsFilter>,
    ) -> Result<Option<Page<SignedTransaction>>, Error> {
        let operation = TransactionBlocksQuery::build(TransactionBlocksQueryArgs {
            after,
            before,
            filter,
            first,
            last,
        });

        let response = self.run_query(&operation).await?;

        if let Some(txb) = response.data {
            let txc = txb.transaction_blocks;
            let page_info = txc.page_info;
            let bcs = txc
                .nodes
                .iter()
                .map(|tx| &tx.bcs)
                .filter_map(|b64| {
                    b64.as_ref()
                        .map(|b| base64ct::Base64::decode_vec(b.0.as_str()))
                })
                .collect::<Result<Vec<_>, base64ct::Error>>()
                .map_err(|e| {
                    Error::msg(format!("Cannot decode Base64 transaction bcs bytes: {e}"))
                })?;

            let transactions = bcs
                .iter()
                .map(|tx| bcs::from_bytes::<SignedTransaction>(tx))
                .collect::<Result<Vec<_>, bcs::Error>>()
                .map_err(|e| Error::msg(format!("Cannot decode bcs bytes into Object: {e}")))?;
            let page = Page::new(page_info, transactions);
            Ok(Some(page))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, DEVNET_HOST, MAINNET_HOST, TESTNET_HOST};

    #[test]
    fn test_rpc_server() {
        let mut client = Client::new_mainnet();
        assert_eq!(client.rpc_server(), MAINNET_HOST);
        client.set_rpc_server(TESTNET_HOST).unwrap();
        assert_eq!(client.rpc_server(), TESTNET_HOST);
        client.set_rpc_server(DEVNET_HOST).unwrap();
        assert_eq!(client.rpc_server(), DEVNET_HOST);
    }

    #[test]
    fn test_rpc_version() {
        let mut client = Client::new_mainnet();

        client.set_version(Some("beta")).unwrap();
        assert_eq!(client.rpc_version, Some("beta".to_string()));
        assert_eq!(client.rpc_server(), vec![MAINNET_HOST, "beta"].join("/"));

        client.set_version(None).unwrap();
        assert_eq!(client.rpc_version, None);
        assert_eq!(client.rpc_server(), MAINNET_HOST);
    }
}
