// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::query_types::CoinMetadata;
use crate::query_types::ObjectFilter;
use crate::query_types::ObjectKey;
use crate::query_types::ProtocolConfigs;
use crate::query_types::TransactionsFilter;
use crate::query_types::Validator;
use crate::Client;
use crate::Domain;
use crate::DryRunResult;
use crate::EventFilter;
use crate::Page;
use crate::PaginationFilter;

use sui_types::types::framework::Coin;
use sui_types::types::Address;
use sui_types::types::CheckpointSummary;
use sui_types::types::Digest;
use sui_types::types::Event;
use sui_types::types::Object;
use sui_types::types::ObjectId;
use sui_types::types::SignedTransaction;
use sui_types::types::Transaction;
use sui_types::types::TransactionEffects;
use sui_types::types::UserSignature;

use anyhow::Error;

pub struct Api {
    inner: Client,
}

impl Api {
    /// Create a new `ReadApi` instance with the required GraphQL [`Client`].
    pub fn new(client: Client) -> Self {
        Self { inner: client }
    }

    /// Return the chain id.
    pub async fn chain_id(&self) -> Result<String, Error> {
        self.inner.chain_id().await
    }

    /// Return the protocol configuration for the specified version.
    pub async fn protocol_config(
        &self,
        version: Option<u64>,
    ) -> Result<Option<ProtocolConfigs>, Error> {
        self.inner.protocol_config(version).await
    }

    /// Return the reference gas price for the given epoch. If no epoch is provided, returns the
    /// current reference gas price.
    pub async fn reference_gas_price(&self, epoch: Option<u64>) -> Result<Option<u64>, Error> {
        self.inner.reference_gas_price(epoch).await
    }

    /// Get the list of active validators for the provided epoch, including related metadata (apy,
    /// description, name, operation cap, staking pool, etc.).
    /// If no epoch is provided, it will return the active validators for the current epoch.
    pub async fn active_validators(
        &self,
        epoch: Option<u64>,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<Validator>, Error> {
        self.inner.active_validators(epoch, pagination_filter).await
    }

    /// Execute a transaction with the provided signatures.
    pub async fn execute_transaction(
        &self,
        transaction: &Transaction,
        signatures: Vec<UserSignature>,
    ) -> Result<Option<TransactionEffects>, Error> {
        self.inner.execute_tx(signatures, transaction).await
    }

    /// Dry run a [`Transaction`] and return the transaction effects and dry run error (if any).
    ///
    /// `skipChecks` optional flag disables the usual verification checks that prevent access to
    /// objects that are owned by addresses other than the sender, and calling non-public,
    /// non-entry functions, and some other checks. Defaults to false.
    pub async fn dry_run_transaction(
        &self,
        transaction: &Transaction,
        skip_checks: Option<bool>,
    ) -> Result<DryRunResult, Error> {
        self.inner.dry_run_tx(transaction, skip_checks).await
    }

    // ========================================================================
    // Balance & Coins
    // ========================================================================

    /// Return the balance of a specific coin type for this address.
    ///
    /// Example:
    /// ```rust,no_run
    /// let api = ReadApi::new(client);
    /// let balance = api.balance(owner, "0x2::sui::SUI").await?;
    /// ```
    pub async fn balance(&self, owner: ObjectId, coin_type: &str) -> Result<Option<u128>, Error> {
        self.inner.balance(owner.into(), Some(coin_type)).await
    }

    /// Return the metadata for a specific coin type.
    ///
    /// Example:
    /// ```rust,no_run
    /// let api = ReadApi::new(client);
    /// let metadata = api.coin_metadata("0x2::sui::SUI").await?;
    /// ```
    pub async fn coin_metadata(&self, coin_type: &str) -> Result<Option<CoinMetadata>, Error> {
        self.inner.coin_metadata(coin_type).await
    }

    /// Return the total supply for a specific coin type.
    pub async fn coin_total_supply(&self, coin_type: &str) -> Result<Option<u64>, Error> {
        self.inner.total_supply(coin_type).await
    }

    /// Return a page of coins of `coin_type` owned by the specified address.
    pub async fn coins(
        &self,
        owner: ObjectId,
        coin_type: &str,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<Coin>, Error> {
        self.inner
            .coins(owner.into(), Some(coin_type), pagination_filter)
            .await
    }

    // ========================================================================
    // Checkpoint(s)
    // ========================================================================
    /// Return the latest checkpoint.
    pub async fn latest_checkpoint(&self) -> Result<Option<CheckpointSummary>, Error> {
        self.inner.checkpoint(None, None).await
    }

    /// Return the latest checkpoint seq number
    pub async fn latest_checkpoint_seq_num(&self) -> Result<Option<u64>, Error> {
        self.inner.latest_checkpoint_sequence_number().await
    }

    /// Return the latest checkpoint contents digest
    pub async fn latest_checkpoint_contents_digest(&self) -> Result<Option<Digest>, Error> {
        Ok(self
            .latest_checkpoint()
            .await?
            .map(|checkpoint| checkpoint.content_digest.into()))
    }

    /// Return the latest checkpoint previous digest
    pub async fn latest_checkpoint_previous_digest(&self) -> Result<Option<Digest>, Error> {
        Ok(self
            .latest_checkpoint()
            .await?
            .and_then(|checkpoint| checkpoint.previous_digest)
            .and_then(|x| Some(x.into())))
    }

    /// Return a checkpoint by its sequence number.
    pub async fn checkpoint_by_seq_num(
        &self,
        sequence_number: u64,
    ) -> Result<Option<CheckpointSummary>, Error> {
        self.inner.checkpoint(None, Some(sequence_number)).await
    }

    /// Return a checkpoint by its digest.
    pub async fn checkpoint_by_digest(
        &self,
        digest: Digest,
    ) -> Result<Option<CheckpointSummary>, Error> {
        self.inner.checkpoint(Some(digest), None).await
    }

    /// Return a page of checkpoint summary.
    pub async fn checkpoints(
        &self,
        pagination_filter: PaginationFilter,
    ) -> Result<Option<Page<CheckpointSummary>>, Error> {
        self.inner.checkpoints(pagination_filter).await
    }

    // ========================================================================
    // Event(s)
    // ========================================================================

    /// Fetch a page of events.
    ///
    /// Example:
    /// ```rust,no_run
    /// let api = ReadApi::new(client);
    /// let events = api.events(EventFilter::default(), PaginationFilter::default()).await?;
    /// ```
    pub async fn events(
        &self,
        filter: EventFilter,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<Event>, Error> {
        self.inner.events(Some(filter), pagination_filter).await
    }

    // ========================================================================
    // Object(s)
    // ========================================================================

    /// Fetch the latest version of this object by its id.
    pub async fn object(&self, id: ObjectId) -> Result<Option<Object>, Error> {
        self.inner.object(id.into(), None).await
    }

    /// Fetch this object at the specified version.
    pub async fn object_at_version(
        &self,
        id: ObjectId,
        version: u64,
    ) -> Result<Option<Object>, Error> {
        self.inner.object(id.into(), Some(version)).await
    }

    /// Fetch a page of objects based on the provided filter.
    pub async fn objects<'a>(
        &self,
        filter: ObjectFilter<'a>,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<Object>, Error> {
        self.inner.objects(Some(filter), pagination_filter).await
    }

    /// Fetch all objects by their ids and their version
    pub async fn objects_by_ids_and_versions(
        &self,
        ids: impl IntoIterator<Item = ObjectKey>,
    ) -> Result<Vec<Object>, Error> {
        let mut output = vec![];
        let filter = ObjectFilter {
            type_: None,
            owner: None,
            object_ids: None,
            object_keys: Some(ids.into_iter().collect()),
        };

        let mut cursor = None;
        let mut page = self
            .inner
            .objects(
                Some(filter.clone()),
                PaginationFilter {
                    direction: crate::Direction::Forward,
                    cursor,
                    limit: None,
                },
            )
            .await?;
        output.extend(page.data.clone());
        loop {
            if !page.page_info().has_next_page {
                break;
            }
            cursor = page.page_info().end_cursor.clone();
            page = self
                .inner
                .objects(
                    Some(filter.clone()),
                    PaginationFilter {
                        direction: crate::Direction::Forward,
                        cursor,
                        limit: None,
                    },
                )
                .await?;
        }
        Ok(output)
    }

    /// Fetch a page of objects owned by the specified address.
    pub async fn owned_objects(
        &self,
        address: Address,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<Object>, Error> {
        self.inner
            .objects(
                Some(ObjectFilter {
                    type_: None,
                    owner: Some(address),
                    object_ids: None,
                    object_keys: None,
                }),
                pagination_filter,
            )
            .await
    }

    // ========================================================================
    // Transaction(s)
    // ========================================================================

    /// Return a [`SignedTransaction`] by its digest.
    pub async fn transaction(&self, digest: Digest) -> Result<Option<SignedTransaction>, Error> {
        self.inner.transaction(digest).await
    }

    /// Return a list of [`SignedTransaction`]s. Note that this method will try to fetch all
    /// transactions by digests, which requires multiple calls to the server to fetch all the
    /// pages.
    pub async fn transactions_by_digests(
        &self,
        digests: Vec<&str>,
    ) -> Result<Vec<SignedTransaction>, Error> {
        let filter = TransactionsFilter {
            function: None,
            kind: None,
            after_checkpoint: None,
            at_checkpoint: None,
            before_checkpoint: None,
            affected_address: None,
            sent_address: None,
            input_object: None,
            changed_object: None,
            transaction_ids: Some(digests),
        };

        let mut output = vec![];

        loop {
            let page = self
                .transactions_paginate(&self.inner, filter.clone(), PaginationFilter::default())
                .await?;

            output.extend(page.data.clone());

            if !page.page_info().has_next_page {
                break;
            }
        }

        Ok(output)
    }

    /// Return the number of transaction blocks in this epoch. This will return `Ok(None)` if the
    /// epoch requested is not available in the GraphQL service (e.g., due to prunning).
    pub async fn total_transactions_by_epoch(
        &self,
        epoch: Option<u64>,
    ) -> Result<Option<u64>, Error> {
        self.inner.epoch_total_transaction_blocks(epoch).await
    }

    /// Return a page of [`SignedTransaction`]s.
    pub async fn transactions_paginate<'a>(
        &self,
        client: &Client,
        tx_filter: TransactionsFilter<'a>,
        pagination_filter: PaginationFilter,
    ) -> Result<Page<SignedTransaction>, Error> {
        client
            .transactions(Some(tx_filter), pagination_filter)
            .await
    }

    // ========================================================================
    // SuiNS
    // ========================================================================
    /// Resolve a SuiNS domain name to an address.
    pub async fn resolve_suins_to_address(&self, domain: &str) -> Result<Option<Address>, Error> {
        self.inner.resolve_suins_to_address(domain).await?
    }

    /// Get the SuiNS registrations for a specific address.
    pub async fn suins_registrations(
        &self,
        address: Address,
        pagination_filter: PaginationFilter<'_>,
    ) -> Result<Page<(Object, Domain)>, Error> {
        self.inner
            .suins_registrations(address, pagination_filter)
            .await
    }
}
