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
    pub fn owned_objects_stream(
        &self,
        request: ListOwnedObjectsRequest,
    ) -> impl Stream<Item = Result<Object>> + '_ {
        let client = self.clone();
        stream::unfold(
            (
                vec![],                     // current batch of objects (reversed for pop efficiency)
                request.page_token.clone(), // current page token
                true,                       // has_next_page
                request,                    // original request template
                client,                     // client for making requests
            ),
            move |(mut data, page_token, has_next_page, mut request, mut client)| async move {
                if let Some(item) = data.pop() {
                    Some((Ok(item), (data, page_token, has_next_page, request, client)))
                } else if has_next_page {
                    // Fetch next page
                    request.page_token = page_token;

                    match client
                        .state_client()
                        .list_owned_objects(request.clone())
                        .await
                    {
                        Ok(response) => {
                            let response = response.into_inner();
                            let mut data = response.objects;
                            data.reverse(); // Reverse for efficient pop()
                            let next_page_token = response.next_page_token;
                            let has_next_page = next_page_token.is_some();
                            data.pop().map(|item| {
                                (
                                    Ok(item),
                                    (data, next_page_token, has_next_page, request, client),
                                )
                            })
                        }
                        Err(e) => {
                            // Return error and terminate stream
                            Some((Err(e), (vec![], None, false, request, client)))
                        }
                    }
                } else {
                    None
                }
            },
        )
    }
}
