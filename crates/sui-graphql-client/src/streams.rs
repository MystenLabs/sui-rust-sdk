use crate::Error;
use crate::Page;

use futures::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

/// A stream that yields items from a paginated query.
pub struct PageStream<T, F, Fut>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
    query_fn: F,
    current_page: Option<Page<T>>,
    current_future: Option<Pin<Box<Fut>>>,
    current_index: usize,
    finished: bool,
}

// Implement Unpin for PageStream since none of our fields need to be pinned
impl<T, F, Fut> Unpin for PageStream<T, F, Fut>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
}

impl<T, F, Fut> PageStream<T, F, Fut>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
    pub fn new(query_fn: F) -> Self {
        Self {
            query_fn,
            current_page: None,
            current_future: None,
            current_index: 0,
            finished: false,
        }
    }
}

impl<T, F, Fut> Stream for PageStream<T, F, Fut>
where
    T: Clone,
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
    type Item = Result<T, Error>;

    /// Polls the stream for the next item.
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // We can now use self directly since we implemented Unpin
        if self.finished {
            return Poll::Ready(None);
        }

        // If we have a current page, return the next item
        if let Some(page) = &self.current_page {
            if self.current_index < page.data().len() {
                let item = page.data()[self.current_index].clone();
                self.current_index += 1;
                return Poll::Ready(Some(Ok(item)));
            }

            // If no more items and no next page, mark as finished
            if !page.page_info().has_next_page {
                self.finished = true;
                return Poll::Ready(None);
            }
        }

        // Get cursor from current page
        let current_cursor = self
            .current_page
            .as_ref()
            .and_then(|page| {
                page.page_info()
                    .has_next_page
                    .then(|| page.page_info().end_cursor.clone())
            })
            .flatten();

        // If there's no future yet, create one
        if self.current_future.is_none() {
            let future = (self.query_fn)(current_cursor);
            self.current_future = Some(Box::pin(future));
        }

        // Poll the future
        if let Some(future) = &mut self.current_future {
            match future.as_mut().poll(cx) {
                Poll::Ready(Ok(page)) => {
                    if page.is_empty() {
                        self.finished = true;
                        return Poll::Ready(None);
                    }

                    // Store the new page and reset the index
                    self.current_page = Some(page);
                    self.current_index = 0;

                    if let Some(page) = self.current_page.as_ref() {
                        let item = page.data()[self.current_index].clone();
                        self.current_index += 1;

                        // Clear the future as we no longer need it
                        self.current_future = None;

                        Poll::Ready(Some(Ok(item)))
                    } else {
                        Poll::Ready(None)
                    }
                }
                Poll::Ready(Err(e)) => {
                    self.finished = true;
                    self.current_future = None;
                    Poll::Ready(Some(Err(e)))
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Pending
        }
    }
}

/// Creates a new `PageStream` for a paginated query.
///
/// Examples
/// ```rust, no_run
/// use futures::StreamExt;
/// use sui_graphql_client::streams::stream_paginated_query;
///
/// let client = Client::new_testnet();
/// let cursor = None;
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
