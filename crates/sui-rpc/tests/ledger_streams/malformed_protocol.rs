use super::support::ScriptedStreamServer;
use super::support::StreamScript;
use super::support::bounded_event_request;
use super::support::bytes;
use super::support::checkpoint_identity_mask;
use super::support::checkpoint_list_frame;
use super::support::checkpoint_live_frame;
use super::support::event_at;
use super::support::event_identity_mask;
use super::support::event_list_frame;
use super::support::event_live_frame;
use super::support::event_positioned_live_frame;
use super::support::fast_config;
use super::support::fast_list_config;
use super::support::first_event_error;
use super::support::first_list_event_error;
use super::support::observed_client;
use super::support::service_info;
use super::support::spawn_server;
use futures::StreamExt;
use sui_rpc::client::CheckpointStreamRequest;
use sui_rpc::client::CheckpointStreamStart;
use sui_rpc::client::EventStreamRequest;
use sui_rpc::client::EventStreamStart;
use sui_rpc::proto::sui::rpc::v2 as proto;

#[tokio::test]
async fn malformed_list_and_subscription_frames_are_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(0)), Ok(service_info(0))]);
    let missing_watermark = proto::ListEventsResponse::default();
    let mut missing_cursor = proto::ListEventsResponse::default();
    missing_cursor.watermark = Some(proto::Watermark::default());
    let mut missing_reason = event_list_frame(None, 1, None);
    missing_reason.end = Some(proto::QueryEnd::default());
    let mut missing_item_identity = event_list_frame(Some(1), 1, None);
    missing_item_identity.event.as_mut().unwrap().event_index = None;
    server.push_event_lists([
        StreamScript::frames([Ok(missing_watermark)]),
        StreamScript::frames([Ok(missing_cursor)]),
        StreamScript::frames([Ok(missing_reason)]),
        StreamScript::frames([Ok(event_list_frame(
            Some(1),
            1,
            Some(proto::QueryEndReason::LedgerTip),
        ))]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            1,
            Some(proto::QueryEndReason::ItemLimit),
        ))]),
        StreamScript::frames([]),
        StreamScript::frames([Ok(missing_item_identity)]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
    ]);
    let mut missing_subscription_identity =
        event_positioned_live_frame(Some(event_at(1, 0, 0, "missing-index")), 1, 1);
    missing_subscription_identity
        .event
        .as_mut()
        .unwrap()
        .event_index = None;
    server.push_event_subscriptions([StreamScript::frames([Ok(missing_subscription_identity)])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    for expected_message in [
        "List frame is missing its watermark",
        "List watermark is missing its cursor",
        "List QueryEnd has a missing or unknown reason",
        "non-ItemLimit QueryEnd unexpectedly contains an item",
        "ItemLimit QueryEnd is missing its item",
        "List stream ended before its QueryEnd frame",
    ] {
        let status =
            first_list_event_error(&client, bounded_event_request(), fast_list_config()).await;
        assert_eq!(status.code(), tonic::Code::DataLoss);
        assert_eq!(status.message(), expected_message);
    }
    let subscription_request = EventStreamRequest::new()
        .with_read_mask(event_identity_mask())
        .with_start(EventStreamStart::Resume(bytes(0)));
    for _ in 0..2 {
        let status = first_event_error(&client, subscription_request.clone(), fast_config()).await;
        assert_eq!(status.code(), tonic::Code::DataLoss);
    }
}

#[tokio::test]
async fn checkpoint_specific_malformed_frames_are_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(0))]);
    let mut missing_list_checkpoint = checkpoint_list_frame(Some(1), 1, None);
    missing_list_checkpoint
        .watermark
        .as_mut()
        .unwrap()
        .checkpoint = None;
    server.push_checkpoint_lists([
        StreamScript::frames([Ok(missing_list_checkpoint)]),
        StreamScript::frames([Ok(checkpoint_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::frames([Ok(checkpoint_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::frames([Ok(checkpoint_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
    ]);
    server.push_checkpoint_subscriptions([StreamScript::frames([Ok(
        proto::SubscribeCheckpointsResponse::default(),
    )])]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);

    let list_stream = client.list_checkpoints(
        proto::ListCheckpointsRequest::default()
            .with_read_mask(checkpoint_identity_mask())
            .with_end_checkpoint(2),
    );
    futures::pin_mut!(list_stream);
    let list_status = list_stream.next().await.unwrap().unwrap_err();
    assert_eq!(list_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        list_status.message(),
        "List item watermark names no resumable position"
    );

    let terminal_stream = client.list_checkpoints(
        proto::ListCheckpointsRequest::default()
            .with_read_mask(checkpoint_identity_mask())
            .with_end_checkpoint(2),
    );
    futures::pin_mut!(terminal_stream);
    let terminal_status = terminal_stream.next().await.unwrap().unwrap_err();
    assert_eq!(terminal_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        terminal_status.message(),
        "List CheckpointBound watermark does not match the requested bound"
    );

    let empty_bound_stream = client.list_checkpoints(
        proto::ListCheckpointsRequest::default()
            .with_read_mask(checkpoint_identity_mask())
            .with_end_checkpoint(0),
    );
    futures::pin_mut!(empty_bound_stream);
    let empty_bound_status = empty_bound_stream.next().await.unwrap().unwrap_err();
    assert_eq!(empty_bound_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        empty_bound_status.message(),
        "List CheckpointBound watermark does not match the requested bound"
    );

    let subscription_stream = client.stream_checkpoints(
        CheckpointStreamRequest::new()
            .with_read_mask(checkpoint_identity_mask())
            .with_start(CheckpointStreamStart::Checkpoint(0)),
    );
    futures::pin_mut!(subscription_stream);
    let subscription_progress = subscription_stream.next().await.unwrap().unwrap();
    assert_eq!(subscription_progress.cursor, 0);
    let subscription_status = subscription_stream.next().await.unwrap().unwrap_err();
    assert_eq!(subscription_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        subscription_status.message(),
        "checkpoint subscription frame is missing its cursor"
    );
}

#[tokio::test]
async fn unfiltered_checkpoint_progress_only_live_frame_is_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_checkpoint_subscriptions([StreamScript::frames([
        Ok(checkpoint_live_frame(Some(0), 0)),
        Ok(checkpoint_live_frame(None, 1)),
        Ok(checkpoint_live_frame(Some(2), 2)),
    ])]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);
    let stream = client.stream_checkpoints(
        CheckpointStreamRequest::new().with_read_mask(checkpoint_identity_mask()),
    );
    futures::pin_mut!(stream);

    let first = stream.next().await.unwrap().unwrap();
    assert_eq!(first.cursor, 0);
    assert_eq!(first.checkpoint.unwrap().sequence_number, Some(0),);
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(
        status.message(),
        "unfiltered checkpoint subscription frame is missing its checkpoint"
    );
    assert!(stream.next().await.is_none());
}

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

#[tokio::test]
async fn live_checkpoint_coverage_retraction_is_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(5)), Ok(service_info(5))]);
    let mut retracted = event_live_frame(None, 6);
    retracted.watermark.as_mut().unwrap().checkpoint = None;
    server.push_event_subscriptions([
        StreamScript::frames([Ok(event_live_frame(None, 5))]),
        StreamScript::frames([Ok(event_live_frame(None, 5)), Ok(retracted)]),
    ]);
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        5,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);

    let stream = client.stream_events_with_config(
        EventStreamRequest::new()
            .with_read_mask(event_identity_mask())
            .with_start(EventStreamStart::Checkpoint(0)),
        fast_config(),
    );
    futures::pin_mut!(stream);
    stream.next().await.unwrap().unwrap();
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(
        status.message(),
        "subscription checkpoint coverage became unavailable"
    );
    assert!(stream.next().await.is_none());
}
