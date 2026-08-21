use super::support::ScriptedStreamServer;
use super::support::StreamScript;
use super::support::bounded_event_request;
use super::support::bytes;
use super::support::event_identity_mask;
use super::support::event_list_frame;
use super::support::observed_client;
use super::support::spawn_server;
use futures::StreamExt;
use sui_rpc::proto::sui::rpc::v2 as proto;

#[tokio::test]
async fn list_rejects_first_item_at_exclusive_resume_bound() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([StreamScript::frames([
        Ok(event_list_frame(Some(5), 5, None)),
        Ok(event_list_frame(
            None,
            6,
            Some(proto::QueryEndReason::CursorBound),
        )),
    ])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let stream = client.list_events(
        proto::ListEventsRequest::default()
            .with_read_mask(event_identity_mask())
            .with_options(proto::QueryOptions::default().with_after(bytes(5))),
    );
    futures::pin_mut!(stream);

    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(
        status.message(),
        "List item cursor equals its exclusive request resume bound"
    );
    assert!(stream.next().await.is_none());
    assert_eq!(server.state.lock().unwrap().calls, ["list_events"]);
}

#[tokio::test]
async fn scan_limit_without_cursor_advancement_is_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        5,
        Some(proto::QueryEndReason::ScanLimit),
    ))])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let stream = client.list_events(
        proto::ListEventsRequest::default()
            .with_read_mask(event_identity_mask())
            .with_options(proto::QueryOptions::default().with_after(bytes(5))),
    );
    futures::pin_mut!(stream);

    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(
        status.message(),
        "ScanLimit QueryEnd did not advance its cursor"
    );
    assert_eq!(server.state.lock().unwrap().calls, ["list_events"]);
}

#[tokio::test]
async fn list_rejects_checkpoint_regression_retraction_and_duplicate_items() {
    let (server, _calls) = ScriptedStreamServer::new();
    let mut retracted_coverage = event_list_frame(None, 6, None);
    retracted_coverage.watermark.as_mut().unwrap().checkpoint = None;
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Ok(event_list_frame(Some(4), 4, None)),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Ok(event_list_frame(Some(6), 6, None)),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Ok(event_list_frame(Some(5), 5, None)),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Ok(retracted_coverage),
        ]),
    ]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);

    let ascending = client.list_events(bounded_event_request());
    futures::pin_mut!(ascending);
    ascending.next().await.unwrap().unwrap();
    let ascending_status = ascending.next().await.unwrap().unwrap_err();
    assert_eq!(ascending_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        ascending_status.message(),
        "List checkpoint coverage regressed"
    );

    let descending = client.list_events(
        proto::ListEventsRequest::default()
            .with_read_mask(event_identity_mask())
            .with_start_checkpoint(0)
            .with_end_checkpoint(10)
            .with_options(
                proto::QueryOptions::default().with_ordering(proto::Ordering::Descending),
            ),
    );
    futures::pin_mut!(descending);
    descending.next().await.unwrap().unwrap();
    let descending_status = descending.next().await.unwrap().unwrap_err();
    assert_eq!(descending_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        descending_status.message(),
        "List checkpoint coverage regressed"
    );

    let duplicate = client.list_events(bounded_event_request());
    futures::pin_mut!(duplicate);
    duplicate.next().await.unwrap().unwrap();
    let duplicate_status = duplicate.next().await.unwrap().unwrap_err();
    assert_eq!(duplicate_status.code(), tonic::Code::DataLoss);
    assert_eq!(duplicate_status.message(), "List item repeated its cursor");

    let retraction = client.list_events(bounded_event_request());
    futures::pin_mut!(retraction);
    retraction.next().await.unwrap().unwrap();
    let retraction_status = retraction.next().await.unwrap().unwrap_err();
    assert_eq!(retraction_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        retraction_status.message(),
        "List checkpoint coverage became unavailable"
    );
}
