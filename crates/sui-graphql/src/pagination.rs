//! Pagination utilities for GraphQL connections.

use std::future::Future;

use futures::Stream;
use futures::TryStreamExt;
use futures::stream;
use serde::Deserialize;

use crate::error::Error;

/// A page of results from a paginated GraphQL query.
///
/// Supports both forward pagination (`has_next_page`, `end_cursor`) and
/// backward pagination (`has_previous_page`, `start_cursor`).
#[derive(Debug, Clone)]
pub struct Page<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// Whether there are more pages after this one (forward pagination).
    pub has_next_page: bool,
    /// Cursor pointing to the last item in this page (forward pagination).
    pub end_cursor: Option<String>,
    /// Whether there are more pages before this one (backward pagination).
    pub has_previous_page: bool,
    /// Cursor pointing to the first item in this page (backward pagination).
    pub start_cursor: Option<String>,
}

impl<T> Default for Page<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            has_next_page: false,
            end_cursor: None,
            has_previous_page: false,
            start_cursor: None,
        }
    }
}

/// GraphQL PageInfo from Connection responses.
///
/// Supports both forward pagination (`has_next_page`, `end_cursor`) and
/// backward pagination (`has_previous_page`, `start_cursor`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PageInfo {
    /// Whether there are more pages after this one (forward pagination).
    pub has_next_page: bool,
    /// Cursor pointing to the last item in this page (forward pagination).
    pub end_cursor: Option<String>,
    /// Whether there are more pages before this one (backward pagination).
    pub has_previous_page: bool,
    /// Cursor pointing to the first item in this page (backward pagination).
    pub start_cursor: Option<String>,
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
/// ```ignore
/// let stream = paginate(move |cursor| {
///     let client = client.clone();
///     async move {
///         client.fetch_page(cursor.as_deref()).await
///     }
/// });
/// ```
pub fn paginate<T, F, Fut>(fetch_page: F) -> impl Stream<Item = Result<T, Error>>
where
    T: 'static,
    F: Fn(Option<String>) -> Fut + Clone + 'static,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
    // Step 1: unfold produces a stream of pages (each page is a stream of items)
    stream::unfold(
        (None, true, fetch_page), // cursor, has_next_page, fetch function
        |(cursor, has_next, fetch)| async move {
            if !has_next {
                return None;
            }

            match fetch(cursor).await {
                Ok(page) => {
                    let items = stream::iter(page.items.into_iter().map(Ok));
                    Some((Ok(items), (page.end_cursor, page.has_next_page, fetch)))
                }
                Err(e) => {
                    // Yield error and stop pagination
                    Some((Err(e), (None, false, fetch)))
                }
            }
        },
    )
    // Step 2: try_flatten converts stream of pages into stream of items
    .try_flatten()
}

/// Creates a backward paginated stream from a fetch function.
///
/// Similar to [`paginate`], but iterates backward through pages using
/// `has_previous_page` and `start_cursor` instead of `has_next_page` and `end_cursor`.
///
/// Note: Items within each page are yielded in the order returned by the server.
/// The backward pagination only affects which pages are fetched, not item order within pages.
///
/// # Requirements
///
/// The fetch function must return a [`Page`] with `has_previous_page` and `start_cursor`
/// populated. The GraphQL query should:
///
/// 1. Use `last` and `before` parameters (instead of `first` and `after` for forward pagination)
/// 2. Query `hasPreviousPage` and `startCursor` in `pageInfo`
///
/// # Page Size
///
/// The `last` parameter specifies how many items to fetch per page. If `last` exceeds the
/// server's maximum page size, the request will fail. Query `serviceConfig` to discover
/// the page size limits:
///
/// ```graphql
/// query {
///   serviceConfig {
///     defaultPageSize(type: "Query", field: "objects")
///     maxPageSize(type: "Query", field: "objects")
///   }
/// }
/// ```
///
/// # Example
///
/// ```ignore
/// let stream = paginate_backward(move |cursor| {
///     let client = client.clone();
///     async move {
///         // Query with `last` and `before` parameters
///         // and fetch `hasPreviousPage` and `startCursor` in pageInfo
///         client.fetch_page_backward(cursor.as_deref()).await
///     }
/// });
/// ```
pub fn paginate_backward<T, F, Fut>(fetch_page: F) -> impl Stream<Item = Result<T, Error>>
where
    T: 'static,
    F: Fn(Option<String>) -> Fut + Clone + 'static,
    Fut: Future<Output = Result<Page<T>, Error>>,
{
    stream::unfold(
        (None, true, fetch_page),
        |(cursor, has_prev, fetch)| async move {
            if !has_prev {
                return None;
            }

            match fetch(cursor).await {
                Ok(page) => {
                    let items = stream::iter(page.items.into_iter().map(Ok));
                    Some((
                        Ok(items),
                        (page.start_cursor, page.has_previous_page, fetch),
                    ))
                }
                Err(e) => Some((Err(e), (None, false, fetch))),
            }
        },
    )
    .try_flatten()
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
                ..Default::default()
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
                                ..Default::default()
                            })
                        }
                        1 => {
                            assert_eq!(cursor, Some("cursor1".to_string()));
                            Ok(Page {
                                items: vec![3, 4],
                                has_next_page: true,
                                end_cursor: Some("cursor2".to_string()),
                                ..Default::default()
                            })
                        }
                        2 => {
                            assert_eq!(cursor, Some("cursor2".to_string()));
                            Ok(Page {
                                items: vec![5],
                                ..Default::default()
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
        let stream = paginate(|_cursor| async { Ok(Page::<i32>::default()) });

        let results: Vec<_> = stream.collect().await;
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_paginate_backward_single_page() {
        let stream = paginate_backward(|_cursor| async {
            Ok(Page {
                items: vec![1, 2, 3],
                ..Default::default()
            })
        });

        let results: Vec<_> = stream.collect().await;
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].as_ref().unwrap(), &1);
        assert_eq!(results[1].as_ref().unwrap(), &2);
        assert_eq!(results[2].as_ref().unwrap(), &3);
    }

    #[tokio::test]
    async fn test_paginate_backward_multiple_pages() {
        let page_count = Arc::new(AtomicUsize::new(0));

        let stream = paginate_backward({
            let page_count = page_count.clone();
            move |cursor| {
                let page_count = page_count.clone();
                async move {
                    let page_num = page_count.fetch_add(1, Ordering::SeqCst);
                    match page_num {
                        0 => {
                            assert!(cursor.is_none());
                            Ok(Page {
                                items: vec![5, 4],
                                has_previous_page: true,
                                start_cursor: Some("cursor1".to_string()),
                                ..Default::default()
                            })
                        }
                        1 => {
                            assert_eq!(cursor, Some("cursor1".to_string()));
                            Ok(Page {
                                items: vec![3, 2],
                                has_previous_page: true,
                                start_cursor: Some("cursor2".to_string()),
                                ..Default::default()
                            })
                        }
                        2 => {
                            assert_eq!(cursor, Some("cursor2".to_string()));
                            Ok(Page {
                                items: vec![1],
                                ..Default::default()
                            })
                        }
                        _ => panic!("unexpected page request"),
                    }
                }
            }
        });

        let results: Vec<i32> = stream.map(|r| r.unwrap()).collect().await;
        assert_eq!(results, vec![5, 4, 3, 2, 1]);
        assert_eq!(page_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_paginate_backward_empty_page() {
        let stream = paginate_backward(|_cursor| async { Ok(Page::<i32>::default()) });

        let results: Vec<_> = stream.collect().await;
        assert!(results.is_empty());
    }
}
