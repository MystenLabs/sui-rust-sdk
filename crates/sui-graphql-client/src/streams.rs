// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::error;
use crate::query_types::PageInfo;
use crate::Direction;
use crate::Page;
use crate::PaginationFilter;

use futures::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

/// A stream that yields items from a paginated query with support for bidirectional pagination.
pub struct PageStream<T, F, Fut> {
    query_fn: F,
    direction: Direction,
    current_page: Option<(PageInfo, std::vec::IntoIter<T>)>,
    current_future: Option<Pin<Box<Fut>>>,
    finished: bool,
    is_first_page: bool,
}

impl<T, F, Fut> PageStream<T, F, Fut> {
    pub fn new(query_fn: F, direction: Direction) -> Self {
        Self {
            query_fn,
            direction,
            current_page: None,
            current_future: None,
            finished: false,
            is_first_page: true,
        }
    }
}

impl<T, F, Fut> Stream for PageStream<T, F, Fut>
where
    T: Clone + Unpin,
    F: Fn(PaginationFilter) -> Fut,
    F: Unpin,
    Fut: Future<Output = Result<Page<T>, error::Error>>,
{
    type Item = Result<T, error::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.finished {
            return Poll::Ready(None);
        }

        loop {
            let direction = self.direction.clone();
            // If we have a current page, return the next item
            if let Some((page_info, iter)) = &mut self.current_page {
                if let Some(item) = iter.next() {
                    return Poll::Ready(Some(Ok(item)));
                }

                // For backward pagination, we check for previous page
                // For the first page in backward pagination, we don't need to check has_previous_page
                let should_continue = match direction {
                    Direction::Forward => page_info.has_next_page,
                    Direction::Backward => page_info.has_previous_page,
                };
                if !should_continue {
                    self.finished = true;
                    return Poll::Ready(None);
                }
            }

            // Get cursor from current page
            let current_cursor = self
                .current_page
                .as_ref()
                .and_then(|(page_info, _iter)| {
                    match self.direction {
                        Direction::Forward => page_info
                            .has_next_page
                            .then(|| page_info.end_cursor.clone()),
                        Direction::Backward => {
                            // For the first page in backward pagination, we don't use a cursor
                            // This ensures we start from the last page
                            if self.is_first_page {
                                None
                            } else {
                                page_info
                                    .has_previous_page
                                    .then(|| page_info.start_cursor.clone())
                            }
                        }
                    }
                })
                .flatten();

            // If there's no future yet, create one
            if self.current_future.is_none() {
                if self.is_first_page && current_cursor.is_some() {
                    self.is_first_page = false;
                }
                let filter = PaginationFilter {
                    direction: self.direction.clone(),
                    cursor: current_cursor,
                    limit: None,
                };
                let future = (self.query_fn)(filter);
                self.current_future = Some(Box::pin(future));
            }

            // Poll the future
            match self.current_future.as_mut().unwrap().as_mut().poll(cx) {
                Poll::Ready(Ok(page)) => {
                    self.current_future = None;

                    if page.is_empty() {
                        self.finished = true;
                        return Poll::Ready(None);
                    }

                    let (page_info, data) = page.into_parts();
                    // For backward pagination, we need to reverse the items
                    let iter = match self.direction {
                        Direction::Forward => data.into_iter(),
                        Direction::Backward => {
                            let mut vec = data;
                            vec.reverse();
                            vec.into_iter()
                        }
                    };
                    self.current_page = Some((page_info, iter));

                    if self.is_first_page {
                        self.is_first_page = false;
                    }
                }
                Poll::Ready(Err(e)) => {
                    if self.is_first_page {
                        self.is_first_page = false;
                    }
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
/// use sui_graphql_client::Direction;
///
/// let client = Client::new_testnet();
/// let stream = stream_paginated_query(|pagination_filter, Direction::Forward| {
///    client.coins(owner, coin_type, pagination_filter })
/// });
/// while let Some(result) = stream.next().await {
///    match result {
///        Ok(coin) => println!("Got coin: {:?}", coin),
///        Err(e) => eprintln!("Error: {}", e),
///    }
/// }
/// ```
pub fn stream_paginated_query<T, F, Fut>(query_fn: F, direction: Direction) -> PageStream<T, F, Fut>
where
    F: Fn(PaginationFilter) -> Fut,
    Fut: Future<Output = Result<Page<T>, error::Error>>,
{
    PageStream::new(query_fn, direction)
}
