// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../README.md")]

pub mod faucet;
pub mod query_types;

use base64ct::Encoding;
use query_types::ActiveValidatorsArgs;
use query_types::ActiveValidatorsQuery;
use query_types::BalanceArgs;
use query_types::BalanceQuery;
use query_types::ChainIdentifierQuery;
use query_types::CheckpointArgs;
use query_types::CheckpointId;
use query_types::CheckpointQuery;
use query_types::CoinMetadata;
use query_types::CoinMetadataArgs;
use query_types::CoinMetadataQuery;
use query_types::DryRunArgs;
use query_types::DryRunQuery;
use query_types::EpochSummaryArgs;
use query_types::EpochSummaryQuery;
use query_types::EventFilter;
use query_types::EventsQuery;
use query_types::EventsQueryArgs;
use query_types::ExecuteTransactionArgs;
use query_types::ExecuteTransactionQuery;
use query_types::ObjectFilter;
use query_types::ObjectQuery;
use query_types::ObjectQueryArgs;
use query_types::ObjectsQuery;
use query_types::ObjectsQueryArgs;
use query_types::PageInfo;
use query_types::ProtocolConfigQuery;
use query_types::ProtocolConfigs;
use query_types::ProtocolVersionArgs;
use query_types::ServiceConfig;
use query_types::ServiceConfigQuery;
use query_types::TransactionBlockArgs;
use query_types::TransactionBlockQuery;
use query_types::TransactionBlocksQuery;
use query_types::TransactionBlocksQueryArgs;
use query_types::TransactionMetadata;
use query_types::TransactionsFilter;
use query_types::Validator;

use sui_types::types::framework::Coin;
use sui_types::types::Address;
use sui_types::types::CheckpointSequenceNumber;
use sui_types::types::CheckpointSummary;
use sui_types::types::Event;
use sui_types::types::Object;
use sui_types::types::SignedTransaction;
use sui_types::types::Transaction;
use sui_types::types::TransactionEffects;
use sui_types::types::UserSignature;

use anyhow::anyhow;
use anyhow::ensure;
use anyhow::Error;
use anyhow::Result;
use cynic::serde;
use cynic::GraphQlResponse;
use cynic::MutationBuilder;
use cynic::Operation;
use cynic::QueryBuilder;
use futures::Stream;
use reqwest::Url;
use std::pin::Pin;

const MAINNET_HOST: &str = "https://sui-mainnet.mystenlabs.com/graphql";
const TESTNET_HOST: &str = "https://sui-testnet.mystenlabs.com/graphql";
const DEVNET_HOST: &str = "https://sui-devnet.mystenlabs.com/graphql";
const LOCAL_HOST: &str = "http://localhost:9125/graphql";
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
    /// (e.g., due to prunning).
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
    pub async fn active_validators(
        &self,
        epoch: Option<u64>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Option<Page<Validator>>, Error> {
        let operation = ActiveValidatorsQuery::build(ActiveValidatorsArgs {
            id: epoch,
            after,
            before,
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
            Ok(Some(Page::new(page_info, nodes)))
        } else {
            Ok(None)
        }
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
    pub async fn coins(
        &self,
        owner: Address,
        after: Option<&str>,
        before: Option<&str>,
        first: Option<i32>,
        last: Option<i32>,
        coin_type: Option<&str>,
    ) -> Result<Option<Page<Coin>>, Error> {
        let response = self
            .objects(
                after,
                before,
                Some(ObjectFilter {
                    type_: Some(coin_type.unwrap_or("0x2::coin::Coin")),
                    owner: Some(owner),
                    object_ids: None,
                    object_keys: None,
                }),
                first,
                last,
            )
            .await?;

        Ok(response.map(|x| {
            Page::new(
                x.page_info,
                x.data
                    .iter()
                    .flat_map(Coin::try_from_object)
                    .map(|c| c.into_owned())
                    .collect::<Vec<_>>(),
            )
        }))
    }

    /// Stream of coins for the specified address and coin type.
    pub fn coins_stream<'a>(
        &'a self,
        owner: Address,
        coin_type: Option<&'a str>,
    ) -> Pin<Box<dyn Stream<Item = Result<Coin, Error>> + 'a>> {
        Box::pin(async_stream::try_stream! {
            let mut after = None;
            loop {
                let response = self.objects(
                    after.as_deref(),
                    None,
                    Some(ObjectFilter {
                        type_: Some(coin_type.unwrap_or("0x2::coin::Coin")),
                        owner: Some(owner),
                        object_ids: None,
                        object_keys: None,
                    }),
                    None,
                    None,
                ).await?;

                if let Some(page) = response {
                    for object in page.data {
                        if let Some(coin) = Coin::try_from_object(&object) {
                            yield coin.into_owned();
                        }
                    }

                    if let Some(end_cursor) = page.page_info.end_cursor {
                        after = Some(end_cursor);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        })
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

    /// Get the `CheckpointSummary` for a given checkpoint digest or checkpoint id. If none is
    /// provided, it will use the last known checkpoint id.
    pub async fn checkpoint(
        &self,
        digest: Option<String>,
        seq_num: Option<u64>,
    ) -> Result<Option<CheckpointSummary>, Error> {
        ensure!(
            !(digest.is_some() && seq_num.is_some()),
            "Either digest or seq_num must be provided"
        );

        let operation = CheckpointQuery::build(CheckpointArgs {
            id: CheckpointId {
                digest,
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
            .and_then(|e| e.total_checkpoints))
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

    pub async fn events(
        &self,
        filter: Option<EventFilter>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Option<Page<Event>>, Error> {
        let operation = EventsQuery::build(EventsQueryArgs {
            filter,
            after,
            before,
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
                .map(Event::try_from)
                .collect::<Result<Vec<Event>, _>>()?;

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

    // ===========================================================================
    // Transaction API
    // ===========================================================================

    /// Dry run a transaction and return the transaction.
    ///
    /// `tx` is either a [`Transaction`] struct or a [`TransactionKind`] struct. The expected type
    /// is controlled by the presence or absence of `txMeta`:
    ///
    ///  - if present, tx is assumed to be a [`TransactionKind`],
    ///  - if absent, tx is assumed to be [`Transaction`].
    ///
    /// `skipChecks` optional flag disables the usual verification checks that prevent access to
    /// objects that are owned by addresses other than the sender, and calling non-public,
    /// non-entry functions, and some other checks. Defaults to false.
    pub async fn dry_run(
        &self,
        tx: &Transaction,
        skip_checks: Option<bool>,
        tx_meta: Option<TransactionMetadata>,
    ) -> Result<Option<Transaction>, Error> {
        let skip_checks = skip_checks.unwrap_or(false);
        let tx_bytes = base64ct::Base64::encode_string(
            &bcs::to_bytes(&tx).map_err(|_| Error::msg("Cannot encode Transaction as BCS"))?,
        );
        let operation = DryRunQuery::build(DryRunArgs {
            tx_bytes,
            skip_checks,
            tx_meta,
        });
        let response = self.run_query(&operation).await?;

        if let Some(errors) = response.errors {
            return Err(Error::msg(format!("{:?}", errors)));
        }

        if let Some(data) = response.data {
            data.dry_run_transaction_block
                .transaction
                .and_then(|d| d.bcs)
                .map(|bcs| {
                    let bcs = base64ct::Base64::decode_vec(bcs.0.as_str()).map_err(|e| {
                        Error::msg(format!("Cannot decode Base64 transaction bcs bytes: {e}"))
                    })?;
                    bcs::from_bytes::<Transaction>(&bcs).map_err(|e| {
                        Error::msg(format!("Cannot decode bcs bytes into Transaction: {e}"))
                    })
                })
                .transpose()
        } else {
            Ok(None)
        }
    }

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
            .map(|bcs| base64ct::Base64::decode_vec(bcs.0.as_str()))
            .transpose()
            .map_err(|e| Error::msg(format!("Cannot decode Base64 transaction bcs bytes: {e}")))?
            .map(|bcs| bcs::from_bytes::<SignedTransaction>(&bcs))
            .transpose()?;
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
                .map_err(|e| {
                    Error::msg(format!(
                        "Cannot decode bcs bytes into SignedTransaction: {e}"
                    ))
                })?;
            let page = Page::new(page_info, transactions);
            Ok(Some(page))
        } else {
            Ok(None)
        }
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
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    use crate::Client;
    use crate::DEVNET_HOST;
    use crate::LOCAL_HOST;
    use crate::MAINNET_HOST;
    use crate::TESTNET_HOST;
    const NETWORKS: [(&str, &str); 2] = [(MAINNET_HOST, "35834a8a"), (TESTNET_HOST, "4c78adac")];

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
        for (n, _) in NETWORKS.iter() {
            let client = Client::new(n).unwrap();
            let balance = client.balance("0x1".parse().unwrap(), None).await;
            assert!(balance.is_ok(), "Balance query failed for network: {n}");
        }
    }

    #[tokio::test]
    async fn test_chain_id() {
        for (n, id) in NETWORKS.iter() {
            let client = Client::new(n).unwrap();
            let chain_id = client.chain_id().await;
            assert!(chain_id.is_ok());
            assert_eq!(&chain_id.unwrap(), id);
        }
    }

    #[tokio::test]
    async fn test_reference_gas_price_query() {
        for (n, _) in NETWORKS.iter() {
            let client = Client::new(n).unwrap();
            let rgp = client.reference_gas_price(None).await;
            assert!(
                rgp.is_ok(),
                "Reference gas price query failed for network: {n}"
            );
        }
    }

    #[tokio::test]
    async fn test_protocol_config_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let pc = client.protocol_config(None).await;
            assert!(pc.is_ok());

            // test specific version
            let pc = client.protocol_config(Some(50)).await;
            assert!(pc.is_ok());
            let pc = pc.unwrap();
            if let Some(pc) = pc {
                assert_eq!(
                    pc.protocol_version, 50,
                    "Protocol version query mismatch for network: {n}. Expected: 50, received: {}",
                    pc.protocol_version
                );
            }
        }
    }

    #[tokio::test]
    async fn test_service_config_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let sc = client.service_config().await;
            assert!(sc.is_ok(), "Service config query failed for network: {n}");
        }
    }

    #[tokio::test]
    async fn test_active_validators() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let av = client.active_validators(None, None, None, None, None).await;
            assert!(
                av.is_ok(),
                "Active validators query failed for network: {n}. Error: {}",
                av.unwrap_err()
            );

            assert!(
                av.unwrap().is_some(),
                "Active validators query returned None for network: {n}"
            );
        }
    }

    #[tokio::test]
    async fn test_coin_metadata_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let cm = client.coin_metadata("0x2::sui::SUI").await;
            assert!(cm.is_ok(), "Coin metadata query failed for network: {n}");
        }
    }

    #[tokio::test]
    async fn test_checkpoint_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let c = client.checkpoint(None, None).await;
            assert!(
                c.is_ok(),
                "Checkpoint query failed for network: {n}. Error: {}",
                c.unwrap_err()
            );
        }
    }

    #[tokio::test]
    async fn test_latest_checkpoint_sequence_number_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let last_checkpoint = client.latest_checkpoint_sequence_number().await;
            assert!(
                last_checkpoint.is_ok(),
                "Latest checkpoint sequence number query failed for network: {n}. Error: {}",
                last_checkpoint.unwrap_err()
            );
        }
    }

    #[tokio::test]
    async fn test_epoch_total_checkpoints_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let e = client.epoch_total_checkpoints(None).await;
            assert!(
                e.is_ok(),
                "Epoch total checkpoints query failed for network: {n}. Error: {}",
                e.unwrap_err()
            );
        }
    }

    #[tokio::test]
    async fn test_epoch_total_transaction_blocks_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let e = client.epoch_total_transaction_blocks(None).await;
            assert!(
                e.is_ok(),
                "Epoch total transaction blocks query failed for network: {n}. Error: {}",
                e.unwrap_err()
            );
        }
    }

    #[tokio::test]
    async fn test_epoch_summary_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let e = client.epoch_summary(None).await;
            assert!(
                e.is_ok(),
                "Epoch summary query failed for network: {n}. Error: {}",
                e.unwrap_err()
            );
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_events_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let events = client.events(None, None, None, None, Some(10)).await;
            assert!(
                events.is_ok(),
                "Events query failed for network: {n}. Error: {}",
                events.unwrap_err()
            );
            assert!(
                events.unwrap().is_some(),
                "Events query returned no data for network: {n}"
            );
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_objects_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let objects = client.objects(None, None, None, None, None).await;
            assert!(
                objects.is_ok(),
                "Objects query failed for network: {n}. Error: {}",
                objects.unwrap_err()
            );
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_object_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let object = client.object("0x5".parse().unwrap(), None).await;
            assert!(
                object.is_ok(),
                "Object query failed for network: {n}. Error: {}",
                object.unwrap_err()
            );
        }
    }

    #[tokio::test]
    async fn test_object_bcs_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let object_bcs = client.object_bcs("0x5".parse().unwrap()).await;
            assert!(
                object_bcs.is_ok(),
                "Object bcs query failed for network: {n}. Error: {}",
                object_bcs.unwrap_err()
            );
        }
    }

    #[tokio::test]
    async fn test_coins_query() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let coins = client
                .coins("0x1".parse().unwrap(), None, None, None, None, None)
                .await;
            assert!(
                coins.is_ok(),
                "Coins query failed for network: {n}. Error: {}",
                coins.unwrap_err()
            );
        }
    }

    #[tokio::test]
    async fn test_coins_stream() {
        let client = Client::new_testnet();
        let mut stream = client.coins_stream("0x1".parse().unwrap(), None);
        let mut num_coins = 0;
        while let Some(result) = stream.next().await {
            assert!(result.is_ok());
            num_coins += 1;
        }
        assert!(num_coins > 0);
    }

    #[tokio::test]
    async fn test_total_supply() {
        for (n, _) in NETWORKS {
            let client = Client::new(n).unwrap();
            let ts = client.total_supply("0x2::sui::SUI").await;
            assert!(
                ts.is_ok(),
                "Total supply query failed for network: {n}. Error: {}",
                ts.unwrap_err()
            );
            assert_eq!(
                ts.unwrap().unwrap(),
                10_000_000_000,
                "Total supply mismatch for network: {n}"
            );
        }
    }
}
