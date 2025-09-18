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
        let request = request.into_request();

        // Extract headers and extensions from the original request
        let headers = request.metadata().clone();
        let extensions = request.extensions().clone();
        let request_inner = request.into_inner();

        stream::unfold(
            (
                vec![],        // current batch of objects
                true,          // has_next_page
                request_inner, // request (page_token will be updated as we paginate)
                client,        // client for making requests
                headers,       // original headers to preserve
                extensions,    // original extensions to preserve
            ),
            move |(mut data, has_next_page, mut request_inner, mut client, headers, extensions)| async move {
                if let Some(item) = data.pop() {
                    Some((
                        Ok(item),
                        (
                            data,
                            has_next_page,
                            request_inner,
                            client,
                            headers,
                            extensions,
                        ),
                    ))
                } else if has_next_page {
                    let mut new_request = tonic::Request::new(request_inner.clone());
                    *new_request.metadata_mut() = headers.clone();
                    *new_request.extensions_mut() = extensions.clone();

                    match client.state_client().list_owned_objects(new_request).await {
                        Ok(response) => {
                            let response = response.into_inner();
                            let mut data = response.objects;
                            data.reverse(); // Reverse for efficient pop()

                            request_inner.page_token = response.next_page_token;
                            let has_next_page = request_inner.page_token.is_some();

                            data.pop().map(|item| {
                                (
                                    Ok(item),
                                    (
                                        data,
                                        has_next_page,
                                        request_inner,
                                        client,
                                        headers,
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
                                (vec![], false, request_inner, client, headers, extensions),
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
