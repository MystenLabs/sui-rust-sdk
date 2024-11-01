// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../README.md")]

pub mod faucet;
pub mod query_types;
pub mod streams;

use query_types::ActiveValidatorsArgs;
use query_types::ActiveValidatorsQuery;
use query_types::BalanceArgs;
use query_types::BalanceQuery;
use query_types::ChainIdentifierQuery;
use query_types::CheckpointArgs;
use query_types::CheckpointId;
use query_types::CheckpointQuery;
use query_types::CheckpointsArgs;
use query_types::CheckpointsQuery;
use query_types::CoinMetadata;
use query_types::CoinMetadataArgs;
use query_types::CoinMetadataQuery;
use query_types::DefaultSuinsNameQuery;
use query_types::DefaultSuinsNameQueryArgs;
use query_types::DryRunArgs;
use query_types::DryRunQuery;
use query_types::DynamicFieldArgs;
use query_types::DynamicFieldConnectionArgs;
use query_types::DynamicFieldQuery;
use query_types::DynamicFieldsOwnerQuery;
use query_types::DynamicObjectFieldQuery;
use query_types::EpochSummaryArgs;
use query_types::EpochSummaryQuery;
use query_types::EventFilter;
use query_types::EventsQuery;
use query_types::EventsQueryArgs;
use query_types::ExecuteTransactionArgs;
use query_types::ExecuteTransactionQuery;
use query_types::LatestPackageQuery;
use query_types::MoveFunction;
use query_types::MoveModule;
use query_types::MovePackageVersionFilter;
use query_types::NormalizedMoveFunctionQuery;
use query_types::NormalizedMoveFunctionQueryArgs;
use query_types::NormalizedMoveModuleQuery;
use query_types::NormalizedMoveModuleQueryArgs;
use query_types::ObjectFilter;
use query_types::ObjectQuery;
use query_types::ObjectQueryArgs;
use query_types::ObjectsQuery;
use query_types::ObjectsQueryArgs;
use query_types::PackageArgs;
use query_types::PackageByNameArgs;
use query_types::PackageByNameQuery;
use query_types::PackageCheckpointFilter;
use query_types::PackageQuery;
use query_types::PackageVersionsArgs;
use query_types::PackageVersionsQuery;
use query_types::PackagesQuery;
use query_types::PackagesQueryArgs;
use query_types::PageInfo;
use query_types::ProtocolConfigQuery;
use query_types::ProtocolConfigs;
use query_types::ProtocolVersionArgs;
use query_types::ResolveSuinsQuery;
use query_types::ResolveSuinsQueryArgs;
use query_types::ServiceConfig;
use query_types::ServiceConfigQuery;
use query_types::TransactionBlockArgs;
use query_types::TransactionBlockQuery;
use query_types::TransactionBlocksQuery;
use query_types::TransactionBlocksQueryArgs;
use query_types::TransactionMetadata;
use query_types::TransactionsFilter;
use query_types::Validator;
use streams::stream_paginated_query;

use sui_types::types::framework::Coin;
use sui_types::types::Address;
use sui_types::types::CheckpointSequenceNumber;
use sui_types::types::CheckpointSummary;
use sui_types::types::Digest;
use sui_types::types::Event;
use sui_types::types::MovePackage;
use sui_types::types::Object;
use sui_types::types::SignedTransaction;
use sui_types::types::Transaction;
use sui_types::types::TransactionEffects;
use sui_types::types::TransactionKind;
use sui_types::types::TypeTag;
use sui_types::types::UserSignature;

use anyhow::anyhow;
use anyhow::ensure;
use anyhow::Error;
use anyhow::Result;
use base64ct::Encoding;
use cynic::serde;
use cynic::GraphQlResponse;
use cynic::MutationBuilder;
use cynic::Operation;
use cynic::QueryBuilder;
use futures::Stream;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::str::FromStr;

use crate::query_types::CheckpointTotalTxQuery;

const DEFAULT_ITEMS_PER_PAGE: i32 = 10;

const MAINNET_HOST: &str = "https://sui-mainnet.mystenlabs.com/graphql";
const TESTNET_HOST: &str = "https://sui-testnet.mystenlabs.com/graphql";
const DEVNET_HOST: &str = "https://sui-devnet.mystenlabs.com/graphql";
const LOCAL_HOST: &str = "http://localhost:9125/graphql";
static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

// ===========================================================================
// Output Types
// ===========================================================================

/// The result of a dry run, which includes the effects of the transaction and any errors that may
/// have occurred.
#[derive(Debug)]
pub struct DryRunResult {
    pub effects: Option<TransactionEffects>,
    pub error: Option<String>,
}

/// The name part of a dynamic field, including its type, bcs, and json representation.
#[derive(Clone, Debug)]
pub struct DynamicFieldName {
    /// The type name of this dynamic field name
    pub type_: TypeTag,
    /// The bcs bytes of this dynamic field name
    pub bcs: Vec<u8>,
    /// The json representation of the dynamic field name
    pub json: Option<serde_json::Value>,
}

/// The output of a dynamic field query, that includes the name, value, and value's json
/// representation.
#[derive(Clone, Debug)]
pub struct DynamicFieldOutput {
    /// The name of the dynamic field
    pub name: DynamicFieldName,
    /// The dynamic field value typename and bcs
    pub value: Option<(TypeTag, Vec<u8>)>,
    /// The json representation of the dynamic field value object
    pub value_as_json: Option<serde_json::Value>,
}

/// Helper struct for passing a value that has a type that implements Serialize, for the dynamic
/// fields API.
pub struct NameValue(Vec<u8>);
/// Helper struct for passing a raw bcs value.
pub struct BcsName(pub Vec<u8>);

#[derive(Clone, Debug)]
/// A page of items returned by the GraphQL server.
pub struct Page<T> {
    /// Information about the page, such as the cursor and whether there are more pages.
    page_info: PageInfo,
    /// The data returned by the server.
    data: Vec<T>,
}

impl<T> Page<T> {
    /// Return the page information.
    pub fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    /// Return the data in the page.
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Internal function to create a new page with the provided data and page information.
    fn new(page_info: PageInfo, data: Vec<T>) -> Self {
        Self { page_info, data }
    }

    /// Check if the page has no data.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Internal function to create a page with no data.
    fn new_empty() -> Self {
        Self::new(PageInfo::default(), vec![])
    }

    pub fn into_parts(self) -> (PageInfo, Vec<T>) {
        (self.page_info, self.data)
    }
}

/// Pagination direction.
#[derive(Clone, Debug, Default)]
pub enum Direction {
    #[default]
    Forward,
    Backward,
}

/// Pagination options for querying the GraphQL server. It defaults to forward pagination with the
/// GraphQL server's default items per page limit.
#[derive(Clone, Debug, Default)]
pub struct PaginationFilter {
    /// The direction of pagination.
    pub direction: Direction,
    /// An opaque cursor used for pagination.
    pub cursor: Option<String>,
    /// The maximum number of items to return. Use `service_config` to find the limit.
    pub limit: Option<i32>,
}

impl<T: Serialize> From<T> for NameValue {
    fn from(value: T) -> Self {
        NameValue(bcs::to_bytes(&value).unwrap())
    }
}

impl From<BcsName> for NameValue {
    fn from(value: BcsName) -> Self {
        NameValue(value.0)
    }
}

impl DynamicFieldOutput {
    /// Deserialize the name of the dynamic field into the specified type.
    pub fn deserialize_name<T: DeserializeOwned>(
        &self,
        expected_type: &TypeTag,
    ) -> Result<T, anyhow::Error> {
        assert_eq!(
            expected_type, &self.name.type_,
            "Expected type {}, but got {}",
            expected_type, &self.name.type_
        );

        let bcs = &self.name.bcs;
        bcs::from_bytes::<T>(bcs).map_err(|_| anyhow!("Cannot decode BCS bytes"))
    }

    /// Deserialize the value of the dynamic field into the specified type.
    pub fn deserialize_value<T: DeserializeOwned>(
        &self,
        expected_type: &TypeTag,
    ) -> Result<T, anyhow::Error> {
        let typetag = self.value.as_ref().map(|(typename, _)| typename);
        assert_eq!(
            Some(&expected_type),
            typetag.as_ref(),
            "Expected type {}, but got {:?}",
            expected_type,
            typetag
        );

        if let Some((_, bcs)) = &self.value {
            bcs::from_bytes::<T>(bcs).map_err(|_| anyhow!("Cannot decode BCS bytes"))
        } else {
            Err(anyhow!("No value found"))
        }
    }
}

/// The GraphQL client for interacting with the Sui blockchain.
/// By default, it uses the `reqwest` crate as the HTTP client.
pub struct Client {
    /// The URL of the GraphQL server.
    rpc: Url,
    /// The reqwest client.
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
    pub fn new_localhost() -> Self {
        Self::new(LOCAL_HOST).expect("Invalid localhost URL")
    }

    /// Set the server address for the GraphQL GraphQL client. It should be a valid URL with a host and
    /// optionally a port number.
    pub fn set_rpc_server(&mut self, server: &str) -> Result<(), Error> {
        let rpc = reqwest::Url::parse(server)?;
        self.rpc = rpc;
        Ok(())
    }

    /// Return the URL for the GraphQL server.
    fn rpc_server(&self) -> &str {
        self.rpc.as_str()
    }

    /// Internal function to handle pagination filters and return the appropriate values.
    async fn pagination_filter(
        &self,
        pagination_filter: PaginationFilter,
    ) -> (Option<String>, Option<String>, Option<i32>, Option<i32>) {
        let limit = if let Some(limit) = pagination_filter.limit {
            limit
        } else {
            let cfg = self.service_config().await;
            if let Ok(cfg) = cfg {
                cfg.max_page_size
            } else {
                DEFAULT_ITEMS_PER_PAGE
            }
        };
        let (after, before, first, last) = match pagination_filter.direction {
            Direction::Forward => (pagination_filter.cursor, None, Some(limit), None),
            Direction::Backward => (None, pagination_filter.cursor, None, Some(limit)),
        };
        (after, before, first, last)
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

    /// Get the reference gas price for the provided epoch or the last known one if no epoch is
    /// provided.
    ///
    /// This will return `Ok(None)` if the epoch requested is not available in the GraphQL service
    /// (e.g., due to pruning).
    pub async fn reference_gas_price(&self, epoch: Option<u64>) -> Result<Option<u64>, Error> {
        let operation = EpochSummaryQuery::build(EpochSummaryArgs { id: epoch });
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
        let operation = ProtocolConfigQuery::build(ProtocolVersionArgs { id: version });
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

    /// Get the list of active validators for the provided epoch, including related metadata.
    /// If no epoch is provided, it will return the active validators for the current epoch.
    pub async fn active_validators<'a>(
        &self,
        epoch: Option<u64>,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<Validator>, Error> {
        let (after, before, first, last) = self.pagination_filter(pagination_filter).await;

        let operation = ActiveValidatorsQuery::build(ActiveValidatorsArgs {
            id: epoch,
            after: after.as_deref(),
            before: before.as_deref(),
            first,
            last,
        });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(validators) = response
            .data
            .and_then(|d| d.epoch)
            .and_then(|v| v.validator_set)
        {
            let page_info = validators.active_validators.page_info;
            let nodes = validators
                .active_validators
                .nodes
                .into_iter()
                .collect::<Vec<_>>();
            Ok(Page::new(page_info, nodes))
        } else {
            Ok(Page::new_empty())
        }
    }

    /// The total number of transaction blocks in the network by the end of the provided
    /// checkpoint digest.
    pub async fn total_transaction_blocks_by_digest(
        &self,
        digest: Digest,
    ) -> Result<Option<u64>, Error> {
        self.internal_total_transaction_blocks(Some(digest.to_string()), None)
            .await
    }

    /// The total number of transaction blocks in the network by the end of the provided checkpoint
    /// sequence number.
    pub async fn total_transaction_blocks_by_seq_num(
        &self,
        seq_num: u64,
    ) -> Result<Option<u64>, Error> {
        self.internal_total_transaction_blocks(None, Some(seq_num))
            .await
    }

    /// The total number of transaction blocks in the network by the end of the last known
    /// checkpoint.
    pub async fn total_transaction_blocks(&self) -> Result<Option<u64>, Error> {
        self.internal_total_transaction_blocks(None, None).await
    }

    /// Internal function to get the total number of transaction blocks based on the provided
    /// checkpoint digest or sequence number.
    async fn internal_total_transaction_blocks(
        &self,
        digest: Option<String>,
        seq_num: Option<u64>,
    ) -> Result<Option<u64>, Error> {
        ensure!(
            !(digest.is_some() && seq_num.is_some()),
            "Cannot provide both digest and seq_num."
        );

        let operation = CheckpointTotalTxQuery::build(CheckpointArgs {
            id: CheckpointId {
                digest,
                sequence_number: seq_num,
            },
        });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        Ok(response
            .data
            .and_then(|x| x.checkpoint)
            .and_then(|c| c.network_total_transactions))
    }

    // ===========================================================================
    // Balance API
    // ===========================================================================

    /// Get the balance of all the coins owned by address for the provided coin type.
    /// Coin type will default to `0x2::coin::Coin<0x2::sui::SUI>` if not provided.
    pub async fn balance(
        &self,
        address: Address,
        coin_type: Option<&str>,
    ) -> Result<Option<u128>, Error> {
        let operation = BalanceQuery::build(BalanceArgs {
            address,
            coin_type: coin_type.map(|x| x.to_string()),
        });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        let total_balance = response
            .data
            .map(|b| b.owner.and_then(|o| o.balance.map(|b| b.total_balance)))
            .ok_or_else(|| Error::msg("No data in response"))?
            .flatten()
            .map(|x| x.0.parse::<u128>())
            .transpose()
            .map_err(|e| Error::msg(format!("Cannot parse balance into u128: {e}")))?;
        Ok(total_balance)
    }

    // ===========================================================================
    // Coin API
    // ===========================================================================

    /// Get the list of coins for the specified address.
    ///
    /// If `coin_type` is not provided, it will default to `0x2::coin::Coin`, which will return all
    /// coins. For SUI coin, pass in the coin type: `0x2::coin::Coin<0x2::sui::SUI>`.
    pub async fn coins<'a>(
        &self,
        owner: Address,
        coin_type: Option<&str>,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<Coin>, Error> {
        let response = self
            .objects(
                Some(ObjectFilter {
                    type_: Some(coin_type.unwrap_or("0x2::coin::Coin")),
                    owner: Some(owner),
                    object_ids: None,
                    object_keys: None,
                }),
                pagination_filter,
            )
            .await?;

        Ok(Page::new(
            response.page_info,
            response
                .data
                .iter()
                .flat_map(Coin::try_from_object)
                .map(|c| c.into_owned())
                .collect::<Vec<_>>(),
        ))
    }

    /// Get the list of coins for the specified address as a stream.
    ///
    /// If `coin_type` is not provided, it will default to `0x2::coin::Coin`, which will return all
    /// coins. For SUI coin, pass in the coin type: `0x2::coin::Coin<0x2::sui::SUI>`.
    pub async fn coins_stream(
        &self,
        address: Address,
        coin_type: Option<&'static str>,
        streaming_direction: Direction,
    ) -> impl Stream<Item = Result<Coin, Error>> {
        stream_paginated_query(
            move |filter| self.coins(address, coin_type, filter),
            streaming_direction,
        )
    }

    /// Get the coin metadata for the coin type.
    pub async fn coin_metadata(&self, coin_type: &str) -> Result<Option<CoinMetadata>, Error> {
        let operation = CoinMetadataQuery::build(CoinMetadataArgs { coin_type });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        Ok(response.data.and_then(|x| x.coin_metadata))
    }

    /// Get total supply for the coin type.
    pub async fn total_supply(&self, coin_type: &str) -> Result<Option<u64>, Error> {
        let coin_metadata = self.coin_metadata(coin_type).await?;

        coin_metadata
            .and_then(|c| c.supply)
            .map(|c| c.try_into())
            .transpose()
    }

    // ===========================================================================
    // Checkpoints API
    // ===========================================================================

    /// Get the [`CheckpointSummary`] for a given checkpoint digest or checkpoint id. If none is
    /// provided, it will use the last known checkpoint id.
    pub async fn checkpoint(
        &self,
        digest: Option<Digest>,
        seq_num: Option<u64>,
    ) -> Result<Option<CheckpointSummary>, Error> {
        ensure!(
            !(digest.is_some() && seq_num.is_some()),
            "Either digest or seq_num must be provided"
        );

        let operation = CheckpointQuery::build(CheckpointArgs {
            id: CheckpointId {
                digest: digest.map(|d| d.to_string()),
                sequence_number: seq_num,
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

    /// Get a page of [`CheckpointSummary`] for the provided parameters.
    pub async fn checkpoints<'a>(
        &self,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<CheckpointSummary>, Error> {
        let (after, before, first, last) = self.pagination_filter(pagination_filter).await;

        let operation = CheckpointsQuery::build(CheckpointsArgs {
            after: after.as_deref(),
            before: before.as_deref(),
            first,
            last,
        });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(checkpoints) = response.data {
            let cc = checkpoints.checkpoints;
            let page_info = cc.page_info;
            let nodes = cc
                .nodes
                .into_iter()
                .map(|c| c.try_into())
                .collect::<Result<Vec<CheckpointSummary>, _>>()?;

            Ok(Page::new(page_info, nodes))
        } else {
            Ok(Page::new_empty())
        }
    }

    /// Get a stream of [`CheckpointSummary`]. Note that this will fetch all checkpoints which may
    /// trigger a lot of requests.
    pub async fn checkpoints_stream(
        &self,
        streaming_direction: Direction,
    ) -> impl Stream<Item = Result<CheckpointSummary, Error>> + '_ {
        stream_paginated_query(move |filter| self.checkpoints(filter), streaming_direction)
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
    // Dynamic Field(s) API
    // ===========================================================================

    /// Access a dynamic field on an object using its name. Names are arbitrary Move values whose
    /// type have copy, drop, and store, and are specified using their type, and their BCS
    /// contents, Base64 encoded.
    ///
    /// The `name` argument can be either a [`BcsName`] for passing raw bcs bytes or a type that
    /// implements Serialize.
    ///
    /// This returns [`DynamicFieldOutput`] which contains the name, the value as json, and object.
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let client = sui_graphql_client::Client::new_devnet();
    /// let address = Address::from_str("0x5").unwrap();
    /// let df = client.dynamic_field_with_name(address, "u64", 2u64).await.unwrap();
    ///
    /// # alternatively, pass in the bcs bytes
    /// let bcs = base64ct::Base64::decode_vec("AgAAAAAAAAA=").unwrap();
    /// let df = client.dynamic_field(address, "u64", BcsName(bcs)).await.unwrap();
    /// ```
    pub async fn dynamic_field(
        &self,
        address: Address,
        type_: TypeTag,
        name: impl Into<NameValue>,
    ) -> Result<Option<DynamicFieldOutput>, Error> {
        let bcs = name.into().0;
        let operation = DynamicFieldQuery::build(DynamicFieldArgs {
            address,
            name: crate::query_types::DynamicFieldName {
                type_: type_.to_string(),
                bcs: crate::query_types::Base64(base64ct::Base64::encode_string(&bcs)),
            },
        });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        let result = response
            .data
            .and_then(|d| d.owner)
            .and_then(|o| o.dynamic_field)
            .map(|df| df.try_into())
            .transpose()
            .map_err(|e| Error::msg(format!("{:?}", e)))?;

        Ok(result)
    }

    /// Access a dynamic object field on an object using its name. Names are arbitrary Move values whose
    /// type have copy, drop, and store, and are specified using their type, and their BCS
    /// contents, Base64 encoded.
    ///
    /// The `name` argument can be either a [`BcsName`] for passing raw bcs bytes or a type that
    /// implements Serialize.
    ///
    /// This returns [`DynamicFieldOutput`] which contains the name, the value as json, and object.
    pub async fn dynamic_object_field(
        &self,
        address: Address,
        type_: TypeTag,
        name: impl Into<NameValue>,
    ) -> Result<Option<DynamicFieldOutput>, Error> {
        let bcs = name.into().0;
        let operation = DynamicObjectFieldQuery::build(DynamicFieldArgs {
            address,
            name: crate::query_types::DynamicFieldName {
                type_: type_.to_string(),
                bcs: crate::query_types::Base64(base64ct::Base64::encode_string(&bcs)),
            },
        });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        let result: Option<DynamicFieldOutput> = response
            .data
            .and_then(|d| d.object)
            .and_then(|o| o.dynamic_object_field)
            .map(|df| df.try_into())
            .transpose()
            .map_err(|e| Error::msg(format!("{:?}", e)))?;
        Ok(result)
    }

    /// Get a page of dynamic fields for the provided address. Note that this will also fetch
    /// dynamic fields on wrapped objects.
    ///
    /// This returns [`Page`] of [`DynamicFieldOutput`]s.
    pub async fn dynamic_fields<'a>(
        &self,
        address: Address,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<DynamicFieldOutput>, Error> {
        let (after, before, first, last) = self.pagination_filter(pagination_filter).await;
        let operation = DynamicFieldsOwnerQuery::build(DynamicFieldConnectionArgs {
            address,
            after: after.as_deref(),
            before: before.as_deref(),
            first,
            last,
        });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        let Some(DynamicFieldsOwnerQuery { owner: Some(dfs) }) = response.data else {
            return Ok(Page::new_empty());
        };

        Ok(Page::new(
            dfs.dynamic_fields.page_info,
            dfs.dynamic_fields
                .nodes
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, Error>>()
                .map_err(|e| Error::msg(format!("{:?}", e)))?,
        ))
    }

    /// Get a stream of dynamic fields for the provided address. Note that this will also fetch
    /// dynamic fields on wrapped objects.
    pub async fn dynamic_fields_stream(
        &self,
        address: Address,
        streaming_direction: Direction,
    ) -> impl Stream<Item = Result<DynamicFieldOutput, Error>> + '_ {
        stream_paginated_query(
            move |filter| self.dynamic_fields(address, filter),
            streaming_direction,
        )
    }

    // ===========================================================================
    // Epoch API
    // ===========================================================================

    /// Return the number of checkpoints in this epoch. This will return `Ok(None)` if the epoch
    /// requested is not available in the GraphQL service (e.g., due to pruning).
    pub async fn epoch_total_checkpoints(&self, epoch: Option<u64>) -> Result<Option<u64>, Error> {
        let response = self.epoch_summary(epoch).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        Ok(response
            .data
            .and_then(|d| d.epoch)
            .and_then(|e| e.total_checkpoints))
    }

    /// Return the number of transaction blocks in this epoch. This will return `Ok(None)` if the
    /// epoch requested is not available in the GraphQL service (e.g., due to pruning).
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
            .and_then(|e| e.total_transactions))
    }

    /// Internal method for getting the epoch summary that is called in a few other APIs for
    /// convenience.
    async fn epoch_summary(
        &self,
        epoch: Option<u64>,
    ) -> Result<GraphQlResponse<EpochSummaryQuery>, Error> {
        let operation = EpochSummaryQuery::build(EpochSummaryArgs { id: epoch });
        self.run_query(&operation).await
    }

    // ===========================================================================
    // Events API
    // ===========================================================================

    /// Return a page of events based on the (optional) event filter.
    pub async fn events(
        &self,
        filter: Option<EventFilter>,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<Event>, Error> {
        let (after, before, first, last) = self.pagination_filter(pagination_filter).await;

        let operation = EventsQuery::build(EventsQueryArgs {
            filter,
            after: after.as_deref(),
            before: before.as_deref(),
            first,
            last,
        });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(events) = response.data {
            let ec = events.events;
            let page_info = ec.page_info;
            let nodes = ec
                .nodes
                .into_iter()
                .map(|e| e.bcs.0)
                .map(|b| base64ct::Base64::decode_vec(&b))
                .collect::<Result<Vec<_>, base64ct::Error>>()
                .map_err(|e| Error::msg(format!("Cannot decode Base64 event bcs bytes: {e}")))?
                .iter()
                .map(|b| bcs::from_bytes::<Event>(b))
                .collect::<Result<Vec<_>, bcs::Error>>()
                .map_err(|e| Error::msg(format!("Cannot decode bcs bytes into Event: {e}")))?;

            Ok(Page::new(page_info, nodes))
        } else {
            Ok(Page::new_empty())
        }
    }

    /// Return a stream of events based on the (optional) event filter.
    pub async fn events_stream(
        &self,
        filter: Option<EventFilter>,
        streaming_direction: Direction,
    ) -> impl Stream<Item = Result<Event, Error>> + '_ {
        stream_paginated_query(
            move |pag_filter| self.events(filter.clone(), pag_filter),
            streaming_direction,
        )
    }

    // ===========================================================================
    // Objects API
    // ===========================================================================

    /// Return an object based on the provided [`Address`].
    ///
    /// If the object does not exist (e.g., due to pruning), this will return `Ok(None)`.
    /// Similarly, if this is not an object but an address, it will return `Ok(None)`.
    pub async fn object(
        &self,
        address: Address,
        version: Option<u64>,
    ) -> Result<Option<Object>, Error> {
        let operation = ObjectQuery::build(ObjectQueryArgs { address, version });

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
        filter: Option<ObjectFilter<'_>>,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<Object>, Error> {
        let (after, before, first, last) = self.pagination_filter(pagination_filter).await;
        let operation = ObjectsQuery::build(ObjectsQueryArgs {
            after: after.as_deref(),
            before: before.as_deref(),
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

            Ok(Page::new(page_info, objects))
        } else {
            Ok(Page::new_empty())
        }
    }

    /// Return a stream of objects based on the (optional) object filter.
    pub async fn objects_stream<'a>(
        &'a self,
        filter: Option<ObjectFilter<'a>>,
        streaming_direction: Direction,
    ) -> impl Stream<Item = Result<Object, Error>> + 'a {
        stream_paginated_query(
            move |pag_filter| self.objects(filter.clone(), pag_filter),
            streaming_direction,
        )
    }

    /// Return the object's bcs content [`Vec<u8>`] based on the provided [`Address`].
    pub async fn object_bcs(&self, object_id: Address) -> Result<Option<Vec<u8>>, Error> {
        let operation = ObjectQuery::build(ObjectQueryArgs {
            address: object_id,
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

    /// Return the contents' JSON of an object that is a Move object.
    ///
    /// If the object does not exist (e.g., due to pruning), this will return `Ok(None)`.
    /// Similarly, if this is not an object but an address, it will return `Ok(None)`.
    pub async fn move_object_contents(
        &self,
        address: Address,
        version: Option<u64>,
    ) -> Result<Option<serde_json::Value>, Error> {
        let operation = ObjectQuery::build(ObjectQueryArgs { address, version });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(object) = response.data {
            Ok(object
                .object
                .and_then(|o| o.as_move_object)
                .and_then(|o| o.contents)
                .and_then(|mv| mv.json))
        } else {
            Ok(None)
        }
    }
    /// Return the BCS of an object that is a Move object.
    ///
    /// If the object does not exist (e.g., due to pruning), this will return `Ok(None)`.
    /// Similarly, if this is not an object but an address, it will return `Ok(None)`.
    pub async fn move_object_contents_bcs(
        &self,
        address: Address,
        version: Option<u64>,
    ) -> Result<Option<Vec<u8>>, Error> {
        let operation = ObjectQuery::build(ObjectQueryArgs { address, version });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(object) = response.data {
            object
                .object
                .and_then(|o| o.as_move_object)
                .and_then(|o| o.contents)
                .map(|bcs| base64ct::Base64::decode_vec(bcs.bcs.0.as_str()))
                .transpose()
                .map_err(|e| Error::msg(format!("Cannot decode Base64 object bcs bytes: {e}")))
        } else {
            Ok(None)
        }
    }

    // ===========================================================================
    // Package API
    // ===========================================================================

    /// The package corresponding to the given address (at the optionally given version).
    /// When no version is given, the package is loaded directly from the address given. Otherwise,
    /// the address is translated before loading to point to the package whose original ID matches
    /// the package at address, but whose version is version. For non-system packages, this
    /// might result in a different address than address because different versions of a package,
    /// introduced by upgrades, exist at distinct addresses.
    ///
    /// Note that this interpretation of version is different from a historical object read (the
    /// interpretation of version for the object query).
    pub async fn package(
        &self,
        address: Address,
        version: Option<u64>,
    ) -> Result<Option<MovePackage>, Error> {
        let operation = PackageQuery::build(PackageArgs { address, version });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        response
            .data
            .and_then(|x| x.package)
            .and_then(|x| x.package_bcs)
            .map(|bcs| base64ct::Base64::decode_vec(bcs.0.as_str()))
            .transpose()
            .map_err(|e| Error::msg(format!("Cannot decode Base64 package bcs bytes: {e}")))?
            .map(|bcs| bcs::from_bytes::<MovePackage>(&bcs))
            .transpose()
            .map_err(|e| Error::msg(format!("Cannot decode bcs bytes into MovePackage: {e}")))
    }

    /// Fetch all versions of package at address (packages that share this package's original ID),
    /// optionally bounding the versions exclusively from below with afterVersion, or from above
    /// with beforeVersion.
    pub async fn package_versions(
        &self,
        address: Address,
        pagination_filter: PaginationFilter,
        after_version: Option<u64>,
        before_version: Option<u64>,
    ) -> Result<Page<MovePackage>, Error> {
        let (after, before, first, last) = self.pagination_filter(pagination_filter).await;
        let operation = PackageVersionsQuery::build(PackageVersionsArgs {
            address,
            after: after.as_deref(),
            before: before.as_deref(),
            first,
            last,
            filter: Some(MovePackageVersionFilter {
                after_version,
                before_version,
            }),
        });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(packages) = response.data {
            let pc = packages.package_versions;
            let page_info = pc.page_info;
            let bcs = pc
                .nodes
                .iter()
                .map(|p| &p.package_bcs)
                .filter_map(|b64| {
                    b64.as_ref()
                        .map(|b| base64ct::Base64::decode_vec(b.0.as_str()))
                })
                .collect::<Result<Vec<_>, base64ct::Error>>()
                .map_err(|e| Error::msg(format!("Cannot decode Base64 package bcs bytes: {e}")))?;
            let packages = bcs
                .iter()
                .map(|b| bcs::from_bytes::<MovePackage>(b))
                .collect::<Result<Vec<_>, bcs::Error>>()
                .map_err(|e| {
                    Error::msg(format!("Cannot decode bcs bytes into MovePackage: {e}"))
                })?;

            Ok(Page::new(page_info, packages))
        } else {
            Ok(Page::new_empty())
        }
    }

    /// Fetch the latest version of the package at address.
    /// This corresponds to the package with the highest version that shares its original ID with
    /// the package at address.
    pub async fn package_latest(&self, address: Address) -> Result<Option<MovePackage>, Error> {
        let operation = LatestPackageQuery::build(PackageArgs {
            address,
            version: None,
        });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        let pkg = response
            .data
            .and_then(|x| x.latest_package)
            .and_then(|x| x.package_bcs)
            .map(|bcs| base64ct::Base64::decode_vec(bcs.0.as_str()))
            .transpose()
            .map_err(|e| Error::msg(format!("Cannot decode Base64 package bcs bytes: {e}")))?
            .map(|bcs| bcs::from_bytes::<MovePackage>(&bcs))
            .transpose()
            .map_err(|e| Error::msg(format!("Cannot decode bcs bytes into MovePackage: {e}")))?;

        Ok(pkg)
    }

    /// Fetch a package by its name (using Move Registry Service)
    pub async fn package_by_name(&self, name: &str) -> Result<Option<MovePackage>, Error> {
        let operation = PackageByNameQuery::build(PackageByNameArgs { name });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            println!("{:?}", errors);
            return Err(Error::msg(format!("{:?}", errors)));
        }

        Ok(response
            .data
            .and_then(|x| x.package_by_name)
            .and_then(|x| x.package_bcs)
            .and_then(|bcs| base64ct::Base64::decode_vec(bcs.0.as_str()).ok())
            .and_then(|bcs| bcs::from_bytes::<MovePackage>(&bcs).ok()))
    }

    /// The Move packages that exist in the network, optionally filtered to be strictly before
    /// beforeCheckpoint and/or strictly after afterCheckpoint.
    ///
    /// This query returns all versions of a given user package that appear between the specified
    /// checkpoints, but only records the latest versions of system packages.
    pub async fn packages(
        &self,
        after: Option<&str>,
        before: Option<&str>,
        first: Option<i32>,
        last: Option<i32>,
        after_checkpoint: Option<u64>,
        before_checkpoint: Option<u64>,
    ) -> Result<Page<MovePackage>, Error> {
        if first.is_some() && last.is_some() {
            return Err(Error::msg("Cannot specify both first and last"));
        }

        let operation = PackagesQuery::build(PackagesQueryArgs {
            after,
            before,
            first,
            last,
            filter: Some(PackageCheckpointFilter {
                after_checkpoint,
                before_checkpoint,
            }),
        });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(packages) = response.data {
            let pc = packages.packages;
            let page_info = pc.page_info;
            let bcs = pc
                .nodes
                .iter()
                .map(|p| &p.package_bcs)
                .filter_map(|b64| {
                    b64.as_ref()
                        .map(|b| base64ct::Base64::decode_vec(b.0.as_str()))
                })
                .collect::<Result<Vec<_>, base64ct::Error>>()
                .map_err(|e| Error::msg(format!("Cannot decode Base64 package bcs bytes: {e}")))?;
            let packages = bcs
                .iter()
                .map(|b| bcs::from_bytes::<MovePackage>(b))
                .collect::<Result<Vec<_>, bcs::Error>>()
                .map_err(|e| {
                    Error::msg(format!("Cannot decode bcs bytes into MovePackage: {e}"))
                })?;

            Ok(Page::new(page_info, packages))
        } else {
            Ok(Page::new_empty())
        }
    }

    // ===========================================================================
    // Dry Run API
    // ===========================================================================

    /// Dry run a [`Transaction`] and return the transaction effects and dry run error (if any).
    ///
    /// `skipChecks` optional flag disables the usual verification checks that prevent access to
    /// objects that are owned by addresses other than the sender, and calling non-public,
    /// non-entry functions, and some other checks. Defaults to false.
    pub async fn dry_run_tx(
        &self,
        tx: &Transaction,
        skip_checks: Option<bool>,
    ) -> Result<DryRunResult, Error> {
        let tx_bytes = base64ct::Base64::encode_string(
            &bcs::to_bytes(&tx).map_err(|_| Error::msg("Cannot encode Transaction as BCS"))?,
        );
        self.dry_run(tx_bytes, skip_checks, None).await
    }

    /// Dry run a [`TransactionKind`] and return the transaction effects and dry run error (if any).
    ///
    /// `skipChecks` optional flag disables the usual verification checks that prevent access to
    /// objects that are owned by addresses other than the sender, and calling non-public,
    /// non-entry functions, and some other checks. Defaults to false.
    ///
    /// `tx_meta` is the transaction metadata.
    pub async fn dry_run_tx_kind(
        &self,
        tx_kind: &TransactionKind,
        skip_checks: Option<bool>,
        tx_meta: TransactionMetadata,
    ) -> Result<DryRunResult, Error> {
        let tx_bytes = base64ct::Base64::encode_string(
            &bcs::to_bytes(&tx_kind).map_err(|_| Error::msg("Cannot encode Transaction as BCS"))?,
        );
        self.dry_run(tx_bytes, skip_checks, Some(tx_meta)).await
    }

    /// Internal implementation of the dry run API.
    async fn dry_run(
        &self,
        tx_bytes: String,
        skip_checks: Option<bool>,
        tx_meta: Option<TransactionMetadata>,
    ) -> Result<DryRunResult, Error> {
        let skip_checks = skip_checks.unwrap_or(false);
        let operation = DryRunQuery::build(DryRunArgs {
            tx_bytes,
            skip_checks,
            tx_meta,
        });
        let response = self.run_query(&operation).await?;

        // Query errors
        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        // Dry Run errors
        let error = response
            .data
            .as_ref()
            .and_then(|tx| tx.dry_run_transaction_block.error.clone());

        let effects = response
            .data
            .map(|tx| tx.dry_run_transaction_block)
            .and_then(|tx| tx.transaction)
            .and_then(|tx| tx.effects)
            .and_then(|bcs| bcs.bcs)
            .map(|bcs| base64ct::Base64::decode_vec(bcs.0.as_str()))
            .transpose()
            .map_err(|_| Error::msg("Cannot decode bcs bytes from Base64 for transaction effects"))?
            .map(|bcs| bcs::from_bytes::<TransactionEffects>(&bcs))
            .transpose()
            .map_err(|_| Error::msg("Cannot decode bcs bytes into TransactionEffects"))?;

        Ok(DryRunResult { effects, error })
    }

    // ===========================================================================
    // Transaction API
    // ===========================================================================

    /// Get a transaction by its digest.
    pub async fn transaction(&self, digest: Digest) -> Result<Option<SignedTransaction>, Error> {
        let operation = TransactionBlockQuery::build(TransactionBlockArgs {
            digest: digest.to_string(),
        });
        let response = self.run_query(&operation).await?;

        response
            .data
            .and_then(|d| d.transaction_block)
            .map(|tx| tx.try_into())
            .transpose()
            .map_err(|e| Error::msg(format!("Cannot decode transaction: {e}")))
    }

    /// Get a page of transactions based on the provided filters.
    pub async fn transactions<'a>(
        &self,
        filter: Option<TransactionsFilter<'a>>,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<SignedTransaction>, Error> {
        let (after, before, first, last) = self.pagination_filter(pagination_filter).await;

        let operation = TransactionBlocksQuery::build(TransactionBlocksQueryArgs {
            after: after.as_deref(),
            before: before.as_deref(),
            filter,
            first,
            last,
        });

        let response = self.run_query(&operation).await?;

        if let Some(txb) = response.data {
            let txc = txb.transaction_blocks;
            let page_info = txc.page_info;

            let transactions = txc
                .nodes
                .into_iter()
                .map(|n| n.try_into())
                .collect::<Result<Vec<_>>>()?;
            let page = Page::new(page_info, transactions);
            Ok(page)
        } else {
            Ok(Page::new_empty())
        }
    }

    /// Get a stream of transactions based on the (optional) transaction filter.
    pub async fn transactions_stream<'a>(
        &'a self,
        filter: Option<TransactionsFilter<'a>>,
        streaming_direction: Direction,
    ) -> impl Stream<Item = Result<SignedTransaction, Error>> + 'a {
        stream_paginated_query(
            move |pag_filter| self.transactions(filter.clone(), pag_filter),
            streaming_direction,
        )
    }

    /// Execute a transaction.
    pub async fn execute_tx(
        &self,
        signatures: Vec<UserSignature>,
        tx: &Transaction,
    ) -> Result<Option<TransactionEffects>, Error> {
        let operation = ExecuteTransactionQuery::build(ExecuteTransactionArgs {
            signatures: signatures.iter().map(|s| s.to_base64()).collect(),
            tx_bytes: base64ct::Base64::encode_string(bcs::to_bytes(tx).unwrap().as_ref()),
        });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(data) = response.data {
            let result = data.execute_transaction_block;
            let bcs =
                base64ct::Base64::decode_vec(result.effects.bcs.0.as_str()).map_err(|_| {
                    Error::msg("Cannot decode bcs bytes from Base64 for transaction effects")
                })?;
            let effects: TransactionEffects = bcs::from_bytes(&bcs)
                .map_err(|_| Error::msg("Cannot decode bcs bytes into TransactionEffects"))?;

            Ok(Some(effects))
        } else {
            Ok(None)
        }
    }

    // ===========================================================================
    // Normalized Move Package API
    // ===========================================================================
    /// Return the normalized Move function data for the provided package, module, and function.
    pub async fn normalized_move_function(
        &self,
        package: &str,
        module: &str,
        function: &str,
        version: Option<u64>,
    ) -> Result<Option<MoveFunction>, Error> {
        let operation = NormalizedMoveFunctionQuery::build(NormalizedMoveFunctionQueryArgs {
            address: Address::from_str(package)?,
            module,
            function,
            version,
        });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        Ok(response
            .data
            .and_then(|p| p.package)
            .and_then(|p| p.module)
            .and_then(|m| m.function))
    }

    /// Return the normalized Move module data for the provided module.
    // TODO: do we want to self paginate everything and return all the data, or keep pagination
    // options?
    #[allow(clippy::too_many_arguments)]
    pub async fn normalized_move_module(
        &self,
        package: &str,
        module: &str,
        version: Option<u64>,
        pagination_filter_enums: PaginationFilter,
        pagination_filter_friends: PaginationFilter,
        pagination_filter_functions: PaginationFilter,
        pagination_filter_structs: PaginationFilter,
    ) -> Result<Option<MoveModule>, Error> {
        let (after_enums, before_enums, first_enums, last_enums) =
            self.pagination_filter(pagination_filter_enums).await;
        let (after_friends, before_friends, first_friends, last_friends) =
            self.pagination_filter(pagination_filter_friends).await;
        let (after_functions, before_functions, first_functions, last_functions) =
            self.pagination_filter(pagination_filter_functions).await;
        let (after_structs, before_structs, first_structs, last_structs) =
            self.pagination_filter(pagination_filter_structs).await;
        let operation = NormalizedMoveModuleQuery::build(NormalizedMoveModuleQueryArgs {
            package: Address::from_str(package)?,
            module,
            version,
            after_enums: after_enums.as_deref(),
            after_functions: after_functions.as_deref(),
            after_structs: after_structs.as_deref(),
            after_friends: after_friends.as_deref(),
            before_enums: before_enums.as_deref(),
            before_functions: before_functions.as_deref(),
            before_structs: before_structs.as_deref(),
            before_friends: before_friends.as_deref(),
            first_enums,
            first_functions,
            first_structs,
            first_friends,
            last_enums,
            last_functions,
            last_structs,
            last_friends,
        });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        Ok(response.data.and_then(|p| p.package).and_then(|p| p.module))
    }

    // ===========================================================================
    // SuiNS
    // ===========================================================================

    /// Get the address for the provided Suins domain name.
    pub async fn resolve_suins_to_address(&self, domain: &str) -> Result<Option<Address>, Error> {
        let operation = ResolveSuinsQuery::build(ResolveSuinsQueryArgs { name: domain });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }
        Ok(response
            .data
            .and_then(|d| d.resolve_suins_address)
            .map(|a| a.address))
    }

    /// Get the default Suins domain name for the provided address.
    pub async fn default_suins_name(&self, address: Address) -> Result<Option<String>, Error> {
        let operation = DefaultSuinsNameQuery::build(DefaultSuinsNameQueryArgs { address });

        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }
        Ok(response
            .data
            .and_then(|d| d.address)
            .and_then(|a| a.default_suins_name))
    }
}

// This function is used in tests to create a new client instance for the local server.
#[cfg(test)]
mod tests {
    use base64ct::Encoding;
    use futures::StreamExt;
    use sui_types::types::Ed25519PublicKey;
    use sui_types::types::TypeTag;

    use crate::faucet::FaucetClient;
    use crate::BcsName;
    use crate::Client;
    use crate::Direction;
    use crate::PaginationFilter;
    use crate::DEVNET_HOST;
    use crate::LOCAL_HOST;
    use crate::MAINNET_HOST;
    use crate::TESTNET_HOST;

    fn test_client() -> Client {
        let network = std::env::var("NETWORK").unwrap_or_else(|_| "local".to_string());
        match network.as_str() {
            "mainnet" => Client::new_mainnet(),
            "testnet" => Client::new_testnet(),
            "devnet" => Client::new_devnet(),
            "local" => Client::new_localhost(),
            _ => Client::new(&network).expect("Invalid network URL: {network}"),
        }
    }

    #[test]
    fn test_rpc_server() {
        let mut client = Client::new_mainnet();
        assert_eq!(client.rpc_server(), MAINNET_HOST);
        client.set_rpc_server(TESTNET_HOST).unwrap();
        assert_eq!(client.rpc_server(), TESTNET_HOST);
        client.set_rpc_server(DEVNET_HOST).unwrap();
        assert_eq!(client.rpc_server(), DEVNET_HOST);
        client.set_rpc_server(LOCAL_HOST).unwrap();
        assert_eq!(client.rpc_server(), LOCAL_HOST);

        assert!(client.set_rpc_server("localhost:9125/graphql").is_ok());
        assert!(client.set_rpc_server("9125/graphql").is_err());
    }

    #[tokio::test]
    async fn test_balance_query() {
        let client = test_client();
        let balance = client.balance("0x1".parse().unwrap(), None).await;
        assert!(
            balance.is_ok(),
            "Balance query failed for {} network",
            client.rpc_server()
        );
    }

    #[tokio::test]
    async fn test_chain_id() {
        let client = test_client();
        let chain_id = client.chain_id().await;
        assert!(chain_id.is_ok());
    }

    #[tokio::test]
    async fn test_reference_gas_price_query() {
        let client = test_client();
        let rgp = client.reference_gas_price(None).await;
        assert!(
            rgp.is_ok(),
            "Reference gas price query failed for {} network",
            client.rpc_server()
        );
    }

    #[tokio::test]
    async fn test_protocol_config_query() {
        let client = test_client();
        let pc = client.protocol_config(None).await;
        assert!(pc.is_ok());

        // test specific version
        let pc = client.protocol_config(Some(50)).await;
        assert!(pc.is_ok());
        let pc = pc.unwrap();
        if let Some(pc) = pc {
            assert_eq!(
                pc.protocol_version,
                50,
                "Protocol version query mismatch for {} network. Expected: 50, received: {}",
                client.rpc_server(),
                pc.protocol_version
            );
        }
    }

    #[tokio::test]
    async fn test_service_config_query() {
        let client = test_client();
        let sc = client.service_config().await;
        assert!(
            sc.is_ok(),
            "Service config query failed for {} network",
            client.rpc_server()
        );
    }

    #[tokio::test]
    async fn test_active_validators() {
        let client = test_client();
        let av = client
            .active_validators(None, PaginationFilter::default())
            .await;
        assert!(
            av.is_ok(),
            "Active validators query failed for {} network. Error: {}",
            client.rpc_server(),
            av.unwrap_err()
        );

        assert!(
            !av.unwrap().is_empty(),
            "Active validators query returned None for {} network",
            client.rpc_server()
        );
    }

    #[tokio::test]
    async fn test_coin_metadata_query() {
        let client = test_client();
        let cm = client.coin_metadata("0x2::sui::SUI").await;
        assert!(
            cm.is_ok(),
            "Coin metadata query failed for {} network",
            client.rpc_server()
        );
    }

    #[tokio::test]
    async fn test_checkpoint_query() {
        let client = test_client();
        let c = client.checkpoint(None, None).await;
        assert!(
            c.is_ok(),
            "Checkpoint query failed for {} network. Error: {}",
            client.rpc_server(),
            c.unwrap_err()
        );
    }
    #[tokio::test]
    async fn test_checkpoints_query() {
        let client = test_client();
        let c = client.checkpoints(PaginationFilter::default()).await;
        assert!(
            c.is_ok(),
            "Checkpoints query failed for {} network. Error: {}",
            client.rpc_server(),
            c.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_latest_checkpoint_sequence_number_query() {
        let client = test_client();
        let last_checkpoint = client.latest_checkpoint_sequence_number().await;
        assert!(
            last_checkpoint.is_ok(),
            "Latest checkpoint sequence number query failed for {} network. Error: {}",
            client.rpc_server(),
            last_checkpoint.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_epoch_total_checkpoints_query() {
        let client = test_client();
        let e = client.epoch_total_checkpoints(None).await;
        assert!(
            e.is_ok(),
            "Epoch total checkpoints query failed for {} network. Error: {}",
            client.rpc_server(),
            e.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_epoch_total_transaction_blocks_query() {
        let client = test_client();
        let e = client.epoch_total_transaction_blocks(None).await;
        assert!(
            e.is_ok(),
            "Epoch total transaction blocks query failed for {} network. Error: {}",
            client.rpc_server(),
            e.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_epoch_summary_query() {
        let client = test_client();
        let e = client.epoch_summary(None).await;
        assert!(
            e.is_ok(),
            "Epoch summary query failed for {} network. Error: {}",
            client.rpc_server(),
            e.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_events_query() {
        let client = test_client();
        let events = client.events(None, PaginationFilter::default()).await;
        assert!(
            events.is_ok(),
            "Events query failed for {} network. Error: {}",
            client.rpc_server(),
            events.unwrap_err()
        );
        assert!(
            !events.unwrap().is_empty(),
            "Events query returned no data for {} network",
            client.rpc_server()
        );
    }

    #[tokio::test]
    async fn test_objects_query() {
        let client = test_client();
        let objects = client.objects(None, PaginationFilter::default()).await;
        assert!(
            objects.is_ok(),
            "Objects query failed for {} network. Error: {}",
            client.rpc_server(),
            objects.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_object_query() {
        let client = test_client();
        let object = client.object("0x5".parse().unwrap(), None).await;
        assert!(
            object.is_ok(),
            "Object query failed for {} network. Error: {}",
            client.rpc_server(),
            object.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_object_bcs_query() {
        let client = test_client();
        let object_bcs = client.object_bcs("0x5".parse().unwrap()).await;
        assert!(
            object_bcs.is_ok(),
            "Object bcs query failed for {} network. Error: {}",
            client.rpc_server(),
            object_bcs.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_coins_query() {
        let client = test_client();
        let coins = client
            .coins("0x1".parse().unwrap(), None, PaginationFilter::default())
            .await;
        assert!(
            coins.is_ok(),
            "Coins query failed for {} network. Error: {}",
            client.rpc_server(),
            coins.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_coins_stream() {
        const NUM_COINS_FROM_FAUCET: usize = 5;
        let client = test_client();
        let faucet = match client.rpc_server() {
            LOCAL_HOST => FaucetClient::local(),
            TESTNET_HOST => FaucetClient::testnet(),
            DEVNET_HOST => FaucetClient::devnet(),
            _ => return,
        };
        let key = Ed25519PublicKey::generate(rand::thread_rng());
        let address = key.to_address();
        faucet.request_and_wait(address).await.unwrap();
        let mut stream = client
            .coins_stream(address, None, Direction::default())
            .await;
        let mut num_coins = 0;

        while let Some(result) = stream.next().await {
            assert!(result.is_ok());
            num_coins += 1;
        }
        assert!(num_coins == NUM_COINS_FROM_FAUCET);
    }

    #[tokio::test]
    async fn test_transactions_query() {
        let client = test_client();
        let transactions = client.transactions(None, PaginationFilter::default()).await;
        assert!(
            transactions.is_ok(),
            "Transactions query failed for {} network. Error: {}",
            client.rpc_server(),
            transactions.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_total_supply() {
        let client = test_client();
        let ts = client.total_supply("0x2::sui::SUI").await;
        assert!(
            ts.is_ok(),
            "Total supply query failed for {} network. Error: {}",
            client.rpc_server(),
            ts.unwrap_err()
        );
        assert_eq!(
            ts.unwrap().unwrap(),
            10_000_000_000,
            "Total supply mismatch for {} network",
            client.rpc_server()
        );
    }

    // This needs the tx builder to be able to be tested properly
    #[tokio::test]
    async fn test_dry_run() {
        let client = Client::new_testnet();
        // this tx bytes works on testnet
        let tx_bytes = "AAACAAiA8PoCAAAAAAAg7q6yDns6nPznaKLd9pUD2K6NFiiibC10pDVQHJKdP2kCAgABAQAAAQECAAABAQBGLuHCJ/xjZfhC4vTJt/Zrvq1gexKLaKf3aVzyIkxRaAFUHzz8ftiZdY25qP4f9zySuT1K/qyTWjbGiTu0i0Z1ZFA4gwUAAAAAILeG86EeQm3qY3ajat3iUnY2Gbrk/NbdwV/d9MZviAwwRi7hwif8Y2X4QuL0ybf2a76tYHsSi2in92lc8iJMUWjoAwAAAAAAAECrPAAAAAAAAA==";

        let dry_run = client.dry_run(tx_bytes.to_string(), None, None).await;

        assert!(dry_run.is_ok());
    }

    #[tokio::test]
    async fn test_dynamic_field_query() {
        let client = test_client();
        let bcs = base64ct::Base64::decode_vec("AgAAAAAAAAA=").unwrap();
        let dynamic_field = client
            .dynamic_field("0x5".parse().unwrap(), TypeTag::U64, BcsName(bcs))
            .await;

        assert!(dynamic_field.is_ok());

        let dynamic_field = client
            .dynamic_field("0x5".parse().unwrap(), TypeTag::U64, 2u64)
            .await;

        assert!(dynamic_field.is_ok());
    }

    #[tokio::test]
    async fn test_dynamic_fields_query() {
        let client = test_client();
        let dynamic_fields = client
            .dynamic_fields("0x5".parse().unwrap(), PaginationFilter::default())
            .await;
        assert!(
            dynamic_fields.is_ok(),
            "Dynamic fields query failed for {} network. Error: {}",
            client.rpc_server(),
            dynamic_fields.unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_total_transaction_blocks() {
        let client = test_client();
        let total_transaction_blocks = client.total_transaction_blocks().await;
        assert!(
            total_transaction_blocks
                .as_ref()
                .is_ok_and(|f| f.is_some_and(|tx| tx > 0)),
            "Total transaction blocks query failed for {} network. Error: {}",
            client.rpc_server(),
            total_transaction_blocks.unwrap_err()
        );

        let chckp = client.latest_checkpoint_sequence_number().await;
        assert!(
            chckp.is_ok(),
            "Latest checkpoint sequence number query failed for {} network. Error: {}",
            client.rpc_server(),
            chckp.unwrap_err()
        );
        let chckp_id = chckp.unwrap().unwrap();
        let total_transaction_blocks = client.total_transaction_blocks_by_seq_num(chckp_id).await;
        assert!(total_transaction_blocks.is_ok());
        assert!(total_transaction_blocks.unwrap().is_some_and(|tx| tx > 0));

        let chckp = client.checkpoint(None, Some(chckp_id)).await;
        assert!(chckp.is_ok());
        let digest = chckp.unwrap().unwrap().content_digest;
        let total_transaction_blocks = client
            .total_transaction_blocks_by_digest(digest.into())
            .await;
        assert!(total_transaction_blocks.is_ok());
        assert!(total_transaction_blocks.unwrap().is_some_and(|tx| tx > 0));
    }

    #[tokio::test]
    async fn test_package() {
        let client = test_client();
        let package = client.package("0x2".parse().unwrap(), None).await;
        assert!(
            package.is_ok(),
            "Package query failed for {} network. Error: {}",
            client.rpc_server(),
            package.unwrap_err()
        );

        assert!(
            package.unwrap().is_some(),
            "Package query returned None for {} network",
            client.rpc_server()
        );
    }

    #[tokio::test]
    #[ignore] // don't know which name is not malformed
    async fn test_package_by_name() {
        let client = Client::new_testnet();
        let package = client.package_by_name("sui@sui").await;
        assert!(package.is_ok());
    }

    #[tokio::test]
    async fn test_latest_package_query() {
        let client = test_client();
        let package = client.package_latest("0x2".parse().unwrap()).await;
        assert!(
            package.is_ok(),
            "Latest package query failed for {} network. Error: {}",
            client.rpc_server(),
            package.unwrap_err()
        );

        assert!(
            package.unwrap().is_some(),
            "Latest package for 0x2 query returned None for {} network",
            client.rpc_server()
        );
    }

    #[tokio::test]
    async fn test_packages_query() {
        let client = test_client();
        let packages = client.packages(None, None, None, None, None, None).await;
        assert!(
            packages.is_ok(),
            "Packages query failed for {} network. Error: {}",
            client.rpc_server(),
            packages.unwrap_err()
        );

        assert!(
            !packages.unwrap().is_empty(),
            "Packages query returned no data for {} network",
            client.rpc_server()
        );
    }
}
