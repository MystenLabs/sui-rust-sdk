// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::Error;
use crate::Page;

use futures::Stream;
use futures::StreamExt;
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
        // SAFETY: we're not moving any pinned fields
        let this = unsafe { self.get_unchecked_mut() };

        if this.finished {
            return Poll::Ready(None);
        }

        let current_cursor = this.current_page.as_ref().and_then(|page| {
            page.page_info()
                .has_next_page
                .then(|| page.page_info().end_cursor.clone())
        });

        let future = (this.query_fn)(current_cursor.flatten());
        let mut future = Box::pin(future);

        match future.as_mut().poll(cx) {
            Poll::Ready(Ok(page)) => {
                if page.is_empty() {
                    this.finished = true;
                    return Poll::Ready(None);
                }

                let mut data = page.data().to_vec();
                let item = data.remove(0);
                this.current_page = Some(Page::new(page.page_info().clone(), data));

                Poll::Ready(Some(Ok(item)))
            }
            Poll::Ready(Err(e)) => {
                this.finished = true;
                Poll::Ready(Some(Err(e)))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub fn stream_pages<T, F, Fut>(query_fn: F) -> PageStream<T, F, Fut>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
    PageStream::new(query_fn)
}

//
//
// /// A stream that yields items from paginated queries
// pub struct PageStream<T, F, Fut>
// where
//     F: Fn(Option<String>) -> Fut,
//     Fut: Future<Output = Result<Page<T>, Error>>,
// {
//     query_fn: F,
//     current_page: Option<Page<T>>,
//     finished: bool,
// }
//
// impl<T, F, Fut> PageStream<T, F, Fut>
// where
//     F: Fn(Option<String>) -> Fut,
//     Fut: Future<Output = Result<Page<T>, Error>>,
// {
//     pub fn new(query_fn: F) -> Self {
//         Self {
//             query_fn,
//             current_page: None,
//             finished: false,
//         }
//     }
// }
//
// impl<T, F, Fut> Stream for PageStream<T, F, Fut>
// where
//     T: Clone,
//     F: Fn(Option<String>) -> Fut,
//     Fut: Future<Output = Result<Page<T>, Error>>,
// {
//     type Item = Result<T, Error>;
//
//     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         if self.finished {
//             return Poll::Ready(None);
//         }
//
//         let current_cursor = self.current_page.as_ref().and_then(|page| {
//             page.page_info()
//                 .has_next_page
//                 .then(|| page.page_info().end_cursor.clone())
//         });
//
//         let future = (self.query_fn)(current_cursor.flatten());
//         let mut future = Box::pin(future);
//
//         match future.as_mut().poll(cx) {
//             Poll::Ready(Ok(page)) => {
//                 if page.is_empty() {
//                     self.finished = true;
//                     return Poll::Ready(None);
//                 }
//
//                 let mut data = page.data().to_vec();
//                 let item = data.remove(0);
//                 self.current_page = Some(Page::new(page.page_info().clone(), data));
//
//                 Poll::Ready(Some(Ok(item)))
//             }
//             Poll::Ready(Err(e)) => {
//                 self.finished = true;
//                 Poll::Ready(Some(Err(e)))
//             }
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }
//
// // Helper function to create a stream
// pub fn stream_pages<T, F, Fut>(query_fn: F) -> PageStream<T, F, Fut>
// where
//     F: Fn(Option<String>) -> Fut,
//     Fut: Future<Output = Result<Page<T>, Error>>,
// {
//     PageStream::new(query_fn)
// }
