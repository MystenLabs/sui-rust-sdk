// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    query_types::{ObjectFilter, PageInfo},
    Client, Page,
};
use anyhow::Error;
use std::ops::{Deref, DerefMut};
use sui_types::types::{
    CheckpointSummary as InnerCheckpoint, Object as InnerObject, ObjectId,
    SignedTransaction as InnerTransaction, TransactionDigest,
};

// Newtype wrapper
pub struct Object(InnerObject);
pub struct Transaction(InnerTransaction);
pub struct Checkpoint(InnerCheckpoint);

pub struct ObjectKey {
    pub id: ObjectId,
    pub version: Option<u64>,
}

// Implement Deref and DerefMut to allow access to inner type methods
impl Deref for Object {
    type Target = InnerObject;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Transaction {
    type Target = InnerTransaction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Transaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Checkpoint {
    type Target = InnerCheckpoint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Checkpoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct Pagination {
    cursor: Option<String>,
    direction: Direction,
    limit: u32,
}

enum Direction {
    Forward,
    Backward,
}

impl Object {
    pub async fn latest(client: &Client, id: ObjectId) -> Result<Object, Error> {
        // Implement using client
        let inner_object = client.object(*id.as_address(), None).await?;
        let obj =
            inner_object.ok_or_else(|| anyhow::anyhow!("Error retrieving object with id: {id}"))?;
        Ok(Object(obj))
    }

    pub async fn at_version(client: &Client, id: ObjectId, version: u64) -> Result<Self, Error> {
        // Implement using client
        let inner_object = client.object(*id.as_address(), Some(version)).await?;
        let obj =
            inner_object.ok_or_else(|| anyhow::anyhow!("Error retrieving object with id: {id}"))?;
        Ok(Object(obj))
    }

    pub async fn paginate<'a>(
        client: &Client,
        pagination: Pagination,
        filter: ObjectFilter<'a>,
    ) -> Result<Option<Page<Object>>, Error> {
        let (after, before, first, last) = match pagination.direction {
            Direction::Forward => (pagination.cursor, None, Some(pagination.limit), None),
            Direction::Backward => (None, pagination.cursor, None, Some(pagination.limit)),
        };
        let inner_page = client.objects(after, before, first, last, filter).await?;

        Ok(Page::new(
            PageInfo {
                has_previous_page: false,
                has_next_page: false,
                start_cursor,
                end_cursor,
            },
            vec![],
        ))
    }

    // pub async fn stream(
    //     client: &Client,
    //     page_ifo: Option<PageInfo>,
    //     filter: ObjectFilter,
    // ) -> Result<Stream<Object>, Error> {
    //     Ok(Stream::new())
    // }

    // Implement other methods...
}

impl Transaction {
    pub async fn by_digest(
        client: &Client,
        digest: TransactionDigest,
    ) -> Result<Transaction, Error> {
        // Implement using client
        let inner_transaction = client.transaction(&digest.to_string()).await?;
        let tx = inner_transaction
            .ok_or_else(|| anyhow::anyhow!("Error retrieving transaction with digest: {digest}"))?;
        Ok(Transaction(tx))
    }

    pub async fn multi_get(
        client: &Client,
        digests: Vec<TransactionDigest>,
    ) -> Result<Vec<Transaction>, Error> {
        // Implement using client

        let inner_transactions = client
            .transactions(
                None,
                None,
                None,
                None,
                digests.iter().map(|d| d.to_string()),
            )
            .await?;
        let txs = inner_transactions
            .ok_or_else(|| anyhow::anyhow!("Error retrieving transactions with digests"))?;
        Ok(txs.into_iter().map(Transaction).collect())
    }
}
