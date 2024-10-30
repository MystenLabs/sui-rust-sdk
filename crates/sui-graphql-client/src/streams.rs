// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::query_types::PageInfo;
use crate::Error;
use crate::Page;

use futures::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

/// A stream that yields items from a paginated query.
pub struct PageStream<T, F, Fut> {
    query_fn: F,
    current_page: Option<(PageInfo, std::vec::IntoIter<T>)>,
    current_future: Option<Pin<Box<Fut>>>,
    finished: bool,
}

impl<T, F, Fut> PageStream<T, F, Fut> {
    pub fn new(query_fn: F) -> Self {
        Self {
            query_fn,
            current_page: None,
            current_future: None,
            finished: false,
        }
    }
}

impl<T, F, Fut> Stream for PageStream<T, F, Fut>
where
    T: Clone + Unpin,
    F: Fn(Option<String>) -> Fut,
    F: Unpin,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
    type Item = Result<T, Error>;

    /// Polls the stream for the next item.
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // We can now use self directly since we implemented Unpin
        if self.finished {
            return Poll::Ready(None);
        }

        loop {
            // If we have a current page, return the next item
            if let Some((page_info, iter)) = &mut self.current_page {
                if let Some(item) = iter.next() {
                    return Poll::Ready(Some(Ok(item)));
                }

                // If no more items and no next page, mark as finished
                if !page_info.has_next_page {
                    self.finished = true;
                    return Poll::Ready(None);
                }
            }

            // Get cursor from current page
            let current_cursor = self
                .current_page
                .as_ref()
                .and_then(|(page_info, _iter)| {
                    page_info
                        .has_next_page
                        .then(|| page_info.end_cursor.clone())
                })
                .flatten();

            // If there's no future yet, create one
            if self.current_future.is_none() {
                let future = (self.query_fn)(current_cursor);
                self.current_future = Some(Box::pin(future));
            }

            // Poll the future
            match self.current_future.as_mut().unwrap().as_mut().poll(cx) {
                Poll::Ready(Ok(page)) => {
                    // Clear the future as we no longer need it
                    self.current_future = None;

                    if page.is_empty() {
                        self.finished = true;
                        return Poll::Ready(None);
                    }

                    // Store the new page and reset the index
                    let (page_info, data) = page.into_parts();
                    self.current_page = Some((page_info, data.into_iter()));
                }
                Poll::Ready(Err(e)) => {
                    self.finished = true;
                    self.current_future = None;
                    return Poll::Ready(Some(Err(e)));
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

/// Creates a new `PageStream` for a paginated query.
///
/// Examples
/// ```rust,ignore
/// use futures::StreamExt;
/// use sui_graphql_client::streams::stream_paginated_query;
/// use sui_graphql_client::Client;
/// use sui_graphql_client::PaginationFilter;
///
/// let client = Client::new_testnet();
/// let stream = stream_paginated_query(|cursor| {
///    client.coins(owner, coin_type, PaginationFilter { cursor, ..Default::default() })
/// });
/// while let Some(result) = stream.next().await {
///    match result {
///        Ok(coin) => println!("Got coin: {:?}", coin),
///        Err(e) => eprintln!("Error: {}", e),
///    }
/// }
/// ```
pub fn stream_paginated_query<T, F, Fut>(query_fn: F) -> PageStream<T, F, Fut>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
    PageStream::new(query_fn)
}
