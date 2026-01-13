//! Pagination utilities for GraphQL connections.

use std::future::Future;

use futures::stream::Stream;
use futures::stream::{self};
use serde::Deserialize;

use crate::error::Error;

/// A page of results from a paginated GraphQL query.
#[derive(Debug, Clone)]
pub struct Page<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// Whether there are more pages after this one.
    pub has_next_page: bool,
    /// Cursor to use for fetching the next page.
    pub end_cursor: Option<String>,
}

/// GraphQL PageInfo from Connection responses.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    /// Whether there are more pages after this one.
    pub has_next_page: bool,
    /// Cursor pointing to the last item in this page.
    pub end_cursor: Option<String>,
}

/// Creates a paginated stream from a fetch function.
///
/// This helper handles the common pattern of:
/// 1. Fetching a page of results
/// 2. Yielding items one by one
/// 3. Fetching the next page when the current batch is exhausted
/// 4. Stopping when there are no more pages
///
/// The closure should capture any state it needs (e.g., client, owner address).
///
/// # Example
///
/// ```no_run
/// # use sui_graphql::pagination::{paginate, Page};
/// # struct Client;
/// # impl Clone for Client { fn clone(&self) -> Self { Client } }
/// # impl Client {
/// #     async fn fetch_page(&self, _cursor: Option<&str>) -> Result<Page<String>, sui_graphql::Error> {
/// #         Ok(Page { items: vec![], has_next_page: false, end_cursor: None })
/// #     }
/// # }
/// # fn example(client: Client) {
/// let client = client.clone();
/// let stream = paginate(move |cursor| {
///     let client = client.clone();
///     async move {
///         client.fetch_page(cursor.as_deref()).await
///     }
/// });
/// # }
/// ```
pub fn paginate<T, F, Fut>(fetch_page: F) -> impl Stream<Item = Result<T, Error>>
where
    T: 'static,
    F: Fn(Option<String>) -> Fut + Clone + 'static,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
    stream::unfold(
        (
            Vec::new().into_iter(), // current batch iterator
            true,                   // has_next_page
            None,                   // cursor
            fetch_page,             // fetch function (captures its own state)
        ),
        |(mut iter, has_next, cursor, fetch)| async move {
            // First, try to yield from current batch
            if let Some(item) = iter.next() {
                return Some((Ok(item), (iter, has_next, cursor, fetch)));
            }

            // Batch exhausted - check if there are more pages
            if !has_next {
                return None;
            }

            // Fetch next page
            match fetch(cursor).await {
                Ok(page) => {
                    let mut iter = page.items.into_iter();
                    // Get first item from new batch
                    match iter.next() {
                        Some(item) => {
                            Some((Ok(item), (iter, page.has_next_page, page.end_cursor, fetch)))
                        }
                        None => {
                            // Empty page - end stream
                            None
                        }
                    }
                }
                Err(e) => {
                    // Return error and terminate stream
                    Some((Err(e), (Vec::new().into_iter(), false, None, fetch)))
                }
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    #[test]
    fn test_page_info_deserialization() {
        let json = r#"{"hasNextPage": true, "endCursor": "abc123"}"#;
        let page_info: PageInfo = serde_json::from_str(json).unwrap();
        assert!(page_info.has_next_page);
        assert_eq!(page_info.end_cursor, Some("abc123".to_string()));
    }

    #[test]
    fn test_page_info_deserialization_no_cursor() {
        let json = r#"{"hasNextPage": false, "endCursor": null}"#;
        let page_info: PageInfo = serde_json::from_str(json).unwrap();
        assert!(!page_info.has_next_page);
        assert_eq!(page_info.end_cursor, None);
    }

    #[tokio::test]
    async fn test_paginate_single_page() {
        let stream = paginate(|_cursor| async {
            Ok(Page {
                items: vec![1, 2, 3],
                has_next_page: false,
                end_cursor: None,
            })
        });

        let results: Vec<_> = stream.collect().await;
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].as_ref().unwrap(), &1);
        assert_eq!(results[1].as_ref().unwrap(), &2);
        assert_eq!(results[2].as_ref().unwrap(), &3);
    }

    #[tokio::test]
    async fn test_paginate_multiple_pages() {
        let page_count = Arc::new(AtomicUsize::new(0));

        let stream = paginate({
            let page_count = page_count.clone();
            move |cursor| {
                let page_count = page_count.clone();
                async move {
                    let page_num = page_count.fetch_add(1, Ordering::SeqCst);
                    match page_num {
                        0 => {
                            assert!(cursor.is_none());
                            Ok(Page {
                                items: vec![1, 2],
                                has_next_page: true,
                                end_cursor: Some("cursor1".to_string()),
                            })
                        }
                        1 => {
                            assert_eq!(cursor, Some("cursor1".to_string()));
                            Ok(Page {
                                items: vec![3, 4],
                                has_next_page: true,
                                end_cursor: Some("cursor2".to_string()),
                            })
                        }
                        2 => {
                            assert_eq!(cursor, Some("cursor2".to_string()));
                            Ok(Page {
                                items: vec![5],
                                has_next_page: false,
                                end_cursor: None,
                            })
                        }
                        _ => panic!("unexpected page request"),
                    }
                }
            }
        });

        let results: Vec<i32> = stream.map(|r| r.unwrap()).collect().await;
        assert_eq!(results, vec![1, 2, 3, 4, 5]);
        assert_eq!(page_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_paginate_empty_page() {
        let stream = paginate(|_cursor| async {
            Ok(Page::<i32> {
                items: vec![],
                has_next_page: false,
                end_cursor: None,
            })
        });

        let results: Vec<_> = stream.collect().await;
        assert!(results.is_empty());
    }
}
