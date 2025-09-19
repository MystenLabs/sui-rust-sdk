use futures::stream;
use futures::stream::Stream;

use crate::client::v2::Client;
use crate::client::v2::Result;
use crate::proto::sui::rpc::v2::Balance;
use crate::proto::sui::rpc::v2::DynamicField;
use crate::proto::sui::rpc::v2::ListBalancesRequest;
use crate::proto::sui::rpc::v2::ListDynamicFieldsRequest;
use crate::proto::sui::rpc::v2::ListOwnedObjectsRequest;
use crate::proto::sui::rpc::v2::ListPackageVersionsRequest;
use crate::proto::sui::rpc::v2::Object;
use crate::proto::sui::rpc::v2::PackageVersion;

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

        stream::unfold(
            (
                Vec::new().into_iter(), // current batch of objects
                true,                   // has_next_page
                request,                // request (page_token will be updated as we paginate)
                client,                 // client for making requests
            ),
            move |(mut iter, has_next_page, mut request, mut client)| async move {
                if let Some(item) = iter.next() {
                    return Some((Ok(item), (iter, has_next_page, request, client)));
                }

                if has_next_page {
                    let new_request = tonic::Request::from_parts(
                        request.metadata().clone(),
                        request.extensions().clone(),
                        request.get_ref().clone(),
                    );

                    match client.state_client().list_owned_objects(new_request).await {
                        Ok(response) => {
                            let response = response.into_inner();
                            let mut iter = response.objects.into_iter();

                            let has_next_page = response.next_page_token.is_some();
                            request.get_mut().page_token = response.next_page_token;

                            iter.next()
                                .map(|item| (Ok(item), (iter, has_next_page, request, client)))
                        }
                        Err(e) => {
                            // Return error and terminate stream
                            request.get_mut().page_token = None;
                            Some((Err(e), (Vec::new().into_iter(), false, request, client)))
                        }
                    }
                } else {
                    None
                }
            },
        )
    }

    /// Creates a stream of `DynamicField`s based on the provided request.
    ///
    /// The stream handles pagination automatically by using the page_token from responses
    /// to fetch subsequent pages. The original request's page_token is used as the starting point.
    ///
    /// # Arguments
    /// * `request` - The initial `ListDynamicFieldsRequest` with search criteria
    ///
    /// # Returns
    /// A stream that yields `Result<DynamicField>` instances. If any RPC call fails, the
    /// tonic::Status from that request is returned.
    pub fn list_dynamic_fields(
        &self,
        request: impl tonic::IntoRequest<ListDynamicFieldsRequest>,
    ) -> impl Stream<Item = Result<DynamicField>> + 'static {
        let client = self.clone();
        let request = request.into_request();

        stream::unfold(
            (
                Vec::new().into_iter(), // current batch of objects
                true,                   // has_next_page
                request,                // request (page_token will be updated as we paginate)
                client,                 // client for making requests
            ),
            move |(mut iter, has_next_page, mut request, mut client)| async move {
                if let Some(item) = iter.next() {
                    return Some((Ok(item), (iter, has_next_page, request, client)));
                }

                if has_next_page {
                    let new_request = tonic::Request::from_parts(
                        request.metadata().clone(),
                        request.extensions().clone(),
                        request.get_ref().clone(),
                    );

                    match client.state_client().list_dynamic_fields(new_request).await {
                        Ok(response) => {
                            let response = response.into_inner();
                            let mut iter = response.dynamic_fields.into_iter();

                            let has_next_page = response.next_page_token.is_some();
                            request.get_mut().page_token = response.next_page_token;

                            iter.next()
                                .map(|item| (Ok(item), (iter, has_next_page, request, client)))
                        }
                        Err(e) => {
                            // Return error and terminate stream
                            request.get_mut().page_token = None;
                            Some((Err(e), (Vec::new().into_iter(), false, request, client)))
                        }
                    }
                } else {
                    None
                }
            },
        )
    }

    /// Creates a stream of `Balance`s based on the provided request.
    ///
    /// The stream handles pagination automatically by using the page_token from responses
    /// to fetch subsequent pages. The original request's page_token is used as the starting point.
    ///
    /// # Arguments
    /// * `request` - The initial `ListBalancesRequest` with search criteria
    ///
    /// # Returns
    /// A stream that yields `Result<Balance>` instances. If any RPC call fails, the
    /// tonic::Status from that request is returned.
    pub fn list_balances(
        &self,
        request: impl tonic::IntoRequest<ListBalancesRequest>,
    ) -> impl Stream<Item = Result<Balance>> + 'static {
        let client = self.clone();
        let request = request.into_request();

        stream::unfold(
            (
                Vec::new().into_iter(), // current batch of objects
                true,                   // has_next_page
                request,                // request (page_token will be updated as we paginate)
                client,                 // client for making requests
            ),
            move |(mut iter, has_next_page, mut request, mut client)| async move {
                if let Some(item) = iter.next() {
                    return Some((Ok(item), (iter, has_next_page, request, client)));
                }

                if has_next_page {
                    let new_request = tonic::Request::from_parts(
                        request.metadata().clone(),
                        request.extensions().clone(),
                        request.get_ref().clone(),
                    );

                    match client.state_client().list_balances(new_request).await {
                        Ok(response) => {
                            let response = response.into_inner();
                            let mut iter = response.balances.into_iter();

                            let has_next_page = response.next_page_token.is_some();
                            request.get_mut().page_token = response.next_page_token;

                            iter.next()
                                .map(|item| (Ok(item), (iter, has_next_page, request, client)))
                        }
                        Err(e) => {
                            // Return error and terminate stream
                            request.get_mut().page_token = None;
                            Some((Err(e), (Vec::new().into_iter(), false, request, client)))
                        }
                    }
                } else {
                    None
                }
            },
        )
    }

    /// Creates a stream of `DynamicField`s based on the provided request.
    ///
    /// The stream handles pagination automatically by using the page_token from responses
    /// to fetch subsequent pages. The original request's page_token is used as the starting point.
    ///
    /// # Arguments
    /// * `request` - The initial `ListPackageVersionsRequest` with search criteria
    ///
    /// # Returns
    /// A stream that yields `Result<DynamicField>` instances. If any RPC call fails, the
    /// tonic::Status from that request is returned.
    pub fn list_package_versions(
        &self,
        request: impl tonic::IntoRequest<ListPackageVersionsRequest>,
    ) -> impl Stream<Item = Result<PackageVersion>> + 'static {
        let client = self.clone();
        let request = request.into_request();

        stream::unfold(
            (
                Vec::new().into_iter(), // current batch of objects
                true,                   // has_next_page
                request,                // request (page_token will be updated as we paginate)
                client,                 // client for making requests
            ),
            move |(mut iter, has_next_page, mut request, mut client)| async move {
                if let Some(item) = iter.next() {
                    return Some((Ok(item), (iter, has_next_page, request, client)));
                }

                if has_next_page {
                    let new_request = tonic::Request::from_parts(
                        request.metadata().clone(),
                        request.extensions().clone(),
                        request.get_ref().clone(),
                    );

                    match client
                        .package_client()
                        .list_package_versions(new_request)
                        .await
                    {
                        Ok(response) => {
                            let response = response.into_inner();
                            let mut iter = response.versions.into_iter();

                            let has_next_page = response.next_page_token.is_some();
                            request.get_mut().page_token = response.next_page_token;

                            iter.next()
                                .map(|item| (Ok(item), (iter, has_next_page, request, client)))
                        }
                        Err(e) => {
                            // Return error and terminate stream
                            request.get_mut().page_token = None;
                            Some((Err(e), (Vec::new().into_iter(), false, request, client)))
                        }
                    }
                } else {
                    None
                }
            },
        )
    }
}
