// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::Error;
use crate::Page;

use futures::Stream;
// use futures::StreamExt;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

pub struct PageStream<T, F, Fut>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
    query_fn: F,
    current_page: Option<Page<T>>,
    current_future: Option<Pin<Box<Fut>>>,
    current_index: usize, // Track current index of the page data
    finished: bool,
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
            current_index: 0, // Start at index 0
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

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = unsafe { self.get_unchecked_mut() };

        if this.finished {
            return Poll::Ready(None);
        }

        // If we have a current page, return the next item
        if let Some(page) = &this.current_page {
            if this.current_index < page.data().len() {
                let item = page.data()[this.current_index].clone();
                this.current_index += 1;
                return Poll::Ready(Some(Ok(item)));
            }

            // If no more items and no next page, mark as finished
            if !page.page_info().has_next_page {
                this.finished = true;
                return Poll::Ready(None);
            }
        }

        // Get cursor from current page, will be None on first poll
        let current_cursor = this
            .current_page
            .as_ref()
            .and_then(|page| {
                page.page_info()
                    .has_next_page
                    .then(|| page.page_info().end_cursor.clone())
            })
            .flatten();

        // If there's no future yet, create one
        if this.current_future.is_none() {
            let future = (this.query_fn)(current_cursor);
            this.current_future = Some(Box::pin(future));
        }

        // Poll the future
        if let Some(future) = &mut this.current_future {
            match future.as_mut().poll(cx) {
                Poll::Ready(Ok(page)) => {
                    if page.is_empty() {
                        this.finished = true;
                        return Poll::Ready(None);
                    }

                    // Store the new page and reset the index
                    this.current_page = Some(page);
                    this.current_index = 0; // Reset index for the new page

                    if let Some(page) = this.current_page.as_ref() {
                        let item = page.data()[this.current_index].clone();
                        this.current_index += 1;

                        // Clear the future as we no longer need it
                        this.current_future = None;

                        Poll::Ready(Some(Ok(item)))
                    } else {
                        Poll::Ready(None)
                    }
                }
                Poll::Ready(Err(e)) => {
                    this.finished = true;
                    this.current_future = None;
                    Poll::Ready(Some(Err(e)))
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Pending
        }
    }
}

pub fn stream_paginated_query<T, F, Fut>(query_fn: F) -> PageStream<T, F, Fut>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
    PageStream::new(query_fn)
}
