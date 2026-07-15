//! Regression tests for the automatic pagination in the `list_*` stream
//! helpers.
//!
//! The list-pagination contract allows a server to return an empty page that
//! still carries a `next_page_token` (for example, a filtered scan that hit a
//! server-side budget before finding a match). The pagination streams must
//! keep following the token instead of treating an empty page as
//! end-of-stream, which would silently truncate the results.

use bytes::Bytes;
use futures::StreamExt;
use proto::state_service_server::StateService;
use proto::state_service_server::StateServiceServer;
use sui_rpc::Client;
use sui_rpc::proto::sui::rpc::v2 as proto;

/// A scripted `StateService`: each `ListOwnedObjects` page is selected by the
/// request's `page_token` (the page index, or absent for the first page).
#[derive(Clone)]
struct ScriptedServer {
    /// One entry per page: the objects to return and whether a next page
    /// exists.
    pages: Vec<(Vec<proto::Object>, bool)>,
}

impl ScriptedServer {
    fn page_index(token: Option<&Bytes>) -> usize {
        token
            .map(|t| {
                String::from_utf8(t.to_vec())
                    .expect("test tokens are utf-8")
                    .parse()
                    .expect("test tokens are page indexes")
            })
            .unwrap_or(0)
    }
}

#[tonic::async_trait]
impl StateService for ScriptedServer {
    async fn list_owned_objects(
        &self,
        request: tonic::Request<proto::ListOwnedObjectsRequest>,
    ) -> Result<tonic::Response<proto::ListOwnedObjectsResponse>, tonic::Status> {
        let index = Self::page_index(request.get_ref().page_token.as_ref());
        let Some((objects, has_next)) = self.pages.get(index) else {
            return Err(tonic::Status::invalid_argument(format!(
                "requested page {index} beyond the scripted {} pages",
                self.pages.len()
            )));
        };
        let mut response = proto::ListOwnedObjectsResponse::default();
        response.objects = objects.clone();
        response.next_page_token = has_next.then(|| Bytes::from(format!("{}", index + 1)));
        Ok(tonic::Response::new(response))
    }
}

fn object(id: &str) -> proto::Object {
    let mut object = proto::Object::default();
    object.object_id = Some(id.to_owned());
    object
}

async fn spawn_scripted_server(pages: Vec<(Vec<proto::Object>, bool)>) -> Client {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock server listener");
    let addr = listener.local_addr().expect("mock server local addr");
    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(StateServiceServer::new(ScriptedServer { pages }))
            .serve_with_incoming(tonic::transport::server::TcpIncoming::from(listener))
            .await
            .expect("mock server exited with an error");
    });
    Client::new(format!("http://{addr}")).expect("client")
}

/// Empty pages that still carry a `next_page_token` must be skipped, not
/// treated as end-of-stream.
#[tokio::test(flavor = "multi_thread")]
async fn empty_page_with_token_does_not_truncate_the_stream() {
    let client = spawn_scripted_server(vec![
        (vec![object("0x1"), object("0x2")], true),
        (Vec::new(), true),
        (Vec::new(), true),
        (vec![object("0x3")], false),
    ])
    .await;

    let stream = client.list_owned_objects(proto::ListOwnedObjectsRequest::default());
    let items: Vec<_> = stream.collect().await;

    let ids: Vec<_> = items
        .into_iter()
        .map(|item| item.expect("no page should fail").object_id().to_owned())
        .collect();
    assert_eq!(ids, ["0x1", "0x2", "0x3"]);
}

/// A page-fetch error is yielded as the final stream item and terminates
/// pagination.
#[tokio::test(flavor = "multi_thread")]
async fn page_error_terminates_the_stream() {
    // Two scripted pages; the third fetch (page index 2) falls off the end of
    // the script and the mock returns InvalidArgument.
    let client = spawn_scripted_server(vec![
        (vec![object("0x1")], true),
        (vec![object("0x2")], true),
    ])
    .await;

    let mut stream = Box::pin(client.list_owned_objects(proto::ListOwnedObjectsRequest::default()));

    assert_eq!(stream.next().await.unwrap().unwrap().object_id(), "0x1");
    assert_eq!(stream.next().await.unwrap().unwrap().object_id(), "0x2");
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
    assert!(stream.next().await.is_none());
}
