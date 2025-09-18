use futures::stream::Stream;
use futures::stream::{self};

use crate::client::v2::Client;
use crate::client::v2::Result;
use crate::proto::sui::rpc::v2::ListOwnedObjectsRequest;
use crate::proto::sui::rpc::v2::Object;

impl Client {
    /// Creates a stream of objects based on the provided request.
    ///
    /// The stream handles pagination automatically by using the page_token from responses
    /// to fetch subsequent pages. The original request's page_token is used as the starting point.
    ///
    /// # Arguments
    /// * `request` - The initial `ListOwnedObjectsRequest` with search criteria
    ///
    /// # Returns
    /// A stream that yields `Result<Object>` instances. If any RPC call fails, the
    /// tonic::Status from that request is returned.
    pub fn list_owned_objects(
        &self,
        request: impl tonic::IntoRequest<ListOwnedObjectsRequest>,
    ) -> impl Stream<Item = Result<Object>> + 'static {
        let client = self.clone();

        let (metadata, extensions, request_inner) = request.into_request().into_parts();

        stream::unfold(
            (
                Vec::new().into_iter(), // current batch of objects
                true,                   // has_next_page
                request_inner,          // request (page_token will be updated as we paginate)
                client,                 // client for making requests
                metadata,               // original metadata to preserve
                extensions,             // original extensions to preserve
            ),
            move |(
                mut iter,
                has_next_page,
                mut request_inner,
                mut client,
                metadata,
                extensions,
            )| async move {
                if let Some(item) = iter.next() {
                    return Some((
                        Ok(item),
                        (
                            iter,
                            has_next_page,
                            request_inner,
                            client,
                            metadata,
                            extensions,
                        ),
                    ));
                }

                if has_next_page {
                    let new_request = tonic::Request::from_parts(
                        metadata.clone(),
                        extensions.clone(),
                        request_inner.clone(),
                    );

                    match client.state_client().list_owned_objects(new_request).await {
                        Ok(response) => {
                            let response = response.into_inner();
                            let mut iter = response.objects.into_iter();

                            request_inner.page_token = response.next_page_token;
                            let has_next_page = request_inner.page_token.is_some();

                            iter.next().map(|item| {
                                (
                                    Ok(item),
                                    (
                                        iter,
                                        has_next_page,
                                        request_inner,
                                        client,
                                        metadata,
                                        extensions,
                                    ),
                                )
                            })
                        }
                        Err(e) => {
                            // Return error and terminate stream
                            request_inner.page_token = None;
                            Some((
                                Err(e),
                                (
                                    Vec::new().into_iter(),
                                    false,
                                    request_inner,
                                    client,
                                    metadata,
                                    extensions,
                                ),
                            ))
                        }
                    }
                } else {
                    None
                }
            },
        )
    }
}
