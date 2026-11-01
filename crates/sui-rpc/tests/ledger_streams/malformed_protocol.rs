use super::support::MISSING_LIST_CURSOR_POSITION;
use super::support::ScriptedStreamServer;
use super::support::StreamScript;
use super::support::bounded_event_request;
use super::support::bytes;
use super::support::checkpoint_identity_mask;
use super::support::checkpoint_list_frame;
use super::support::checkpoint_live_frame;
use super::support::checkpoint_position;
use super::support::event_at;
use super::support::event_descending_list_frame;
use super::support::event_identity_mask;
use super::support::event_list_frame;
use super::support::event_live_frame;
use super::support::event_position;
use super::support::event_positioned_list_frame;
use super::support::fast_config;
use super::support::fast_list_config;
use super::support::first_event_error;
use super::support::first_list_event_error;
use super::support::observed_client;
use super::support::service_info;
use super::support::spawn_server;
use super::support::transaction_at;
use super::support::transaction_list_frame;
use super::support::transaction_live_frame;
use super::support::transaction_position;
use super::support::transaction_positioned_list_frame;
use super::support::transaction_positioned_live_frame;
use super::support::wire_position;
use super::support::with_position;
use super::support::without_position;
use futures::StreamExt;
use futures::TryStreamExt;
use std::time::Duration;
use sui_rpc::client::CheckpointStreamRequest;
use sui_rpc::client::CheckpointStreamStart;
use sui_rpc::client::EventStreamRequest;
use sui_rpc::client::EventStreamStart;
use sui_rpc::client::TransactionStreamFrame;
use sui_rpc::client::TransactionStreamRequest;
use sui_rpc::client::TransactionStreamStart;
use sui_rpc::proto::sui::rpc::v2 as proto;

const MISSING_SUBSCRIPTION_POSITION: &str = "subscription watermark is missing its \
     cursor_position; servers that predate cursor positions do not send it";

/// Reads `stream` up to its terminal error, returning the frames yielded before it.
async fn transaction_frames_until_error(
    stream: impl futures::Stream<Item = Result<TransactionStreamFrame, tonic::Status>>,
) -> (Vec<TransactionStreamFrame>, tonic::Status) {
    futures::pin_mut!(stream);
    let mut frames = Vec::new();
    loop {
        match tokio::time::timeout(Duration::from_secs(2), stream.next())
            .await
            .expect("timed out waiting for terminal stream error")
            .expect("stream ended without a terminal error")
        {
            Ok(frame) => frames.push(frame),
            Err(status) => {
                assert!(
                    tokio::time::timeout(Duration::from_secs(2), stream.next())
                        .await
                        .expect("timed out waiting for stream termination")
                        .is_none()
                );
                return (frames, status);
            }
        }
    }
}

#[tokio::test]
async fn malformed_list_frames_are_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    let missing_watermark = proto::ListEventsResponse::default();
    let mut missing_cursor = proto::ListEventsResponse::default();
    missing_cursor.watermark = Some(proto::Watermark::default());
    let mut missing_reason = event_list_frame(None, 1, None);
    missing_reason.end = Some(proto::QueryEnd::default());
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
    ]);
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
    assert_eq!(server.state.lock().unwrap().event_requests.len(), 6);
}

#[tokio::test]
async fn checkpoint_specific_malformed_frames_are_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(0))]);
    server.push_checkpoint_lists([
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
async fn scan_limit_without_position_advancement_is_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            5,
            Some(proto::QueryEndReason::ScanLimit),
        ))]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            5,
            Some(proto::QueryEndReason::ScanLimit),
        ))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let stream = client.list_events_with_config(bounded_event_request(), fast_list_config());
    futures::pin_mut!(stream);
    let first_page = stream.next().await.unwrap().unwrap();
    assert_eq!(
        first_page.watermark.unwrap().cursor_position,
        Some(wire_position(checkpoint_position(6)))
    );
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(
        status.message(),
        "ScanLimit QueryEnd did not advance its cursor_position"
    );
    assert!(stream.next().await.is_none());
    assert_eq!(
        server.state.lock().unwrap().calls,
        ["list_events", "list_events"]
    );
}

#[tokio::test]
async fn list_rejects_coverage_regression_retraction_and_items_behind_the_scan() {
    let (server, _calls) = ScriptedStreamServer::new();
    let mut retracted_coverage = event_list_frame(None, 6, None);
    retracted_coverage.watermark.as_mut().unwrap().checkpoint = None;
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Ok(event_list_frame(Some(4), 4, None)),
        ]),
        StreamScript::frames([
            Ok(event_descending_list_frame(Some(5), 5, None)),
            Ok(event_descending_list_frame(Some(6), 6, None)),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Ok(event_list_frame(Some(5), 5, None)),
        ]),
        // An item before the scan's position, reported with coverage that does not regress.
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Ok(event_positioned_list_frame(
                Some(event_at(4, 0, 0, "behind")),
                6,
                5,
                None,
            )),
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

    // A second item frame at the same position names no item past the first.
    let duplicate = client.list_events(bounded_event_request());
    futures::pin_mut!(duplicate);
    duplicate.next().await.unwrap().unwrap();
    let duplicate_status = duplicate.next().await.unwrap().unwrap_err();
    assert_eq!(duplicate_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        duplicate_status.message(),
        "List item did not advance its cursor_position"
    );

    let behind = client.list_events(bounded_event_request());
    futures::pin_mut!(behind);
    behind.next().await.unwrap().unwrap();
    let behind_status = behind.next().await.unwrap().unwrap_err();
    assert_eq!(behind_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        behind_status.message(),
        "List item did not advance its cursor_position"
    );

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

/// Subscribe frames from servers that predate cursor positions carry no `cursor_position`,
/// whether they open the subscription or follow a valid frame.
#[tokio::test]
async fn event_subscription_frame_without_cursor_position_is_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_subscriptions([
        StreamScript::frames([Ok(without_position(event_live_frame(None, 5)))]),
        StreamScript::frames([
            Ok(event_live_frame(None, 5)),
            Ok(without_position(event_live_frame(Some(6), 6))),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let status = first_event_error(&client, EventStreamRequest::new(), fast_config()).await;
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(status.message(), MISSING_SUBSCRIPTION_POSITION);

    let stream = client.stream_events_with_config(EventStreamRequest::new(), fast_config());
    futures::pin_mut!(stream);
    let first = stream.next().await.unwrap().unwrap();
    assert_eq!(first.position, event_position(6, 0, 0));
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(status.message(), MISSING_SUBSCRIPTION_POSITION);
    assert!(stream.next().await.is_none());
    assert_eq!(
        server.state.lock().unwrap().calls,
        ["subscribe_events", "subscribe_events"]
    );
}

/// A stream's prologue List frames must carry `cursor_position`, even the item-free `LedgerTip`
/// echo of a request that resumes past a replica's tip, which `list_*` accepts without one.
#[tokio::test]
async fn transaction_prologue_list_frame_without_cursor_position_is_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(6))]);
    let mut past_tip_echo = without_position(transaction_list_frame(
        None,
        50,
        Some(proto::QueryEndReason::LedgerTip),
    ));
    past_tip_echo.watermark.as_mut().unwrap().checkpoint = None;
    server.push_transaction_lists([StreamScript::frames([Ok(past_tip_echo)])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let (frames, status) = transaction_frames_until_error(client.stream_transactions_with_config(
        TransactionStreamRequest::new().with_start(TransactionStreamStart::Resume {
            cursor: bytes(50),
            position: transaction_position(6, 1),
        }),
        fast_config(),
    ))
    .await;
    assert!(frames.is_empty());
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(status.message(), MISSING_LIST_CURSOR_POSITION);

    let state = server.state.lock().unwrap();
    assert_eq!(state.calls, ["get_service_info", "list_transactions"]);
    let resumed = &state.transaction_requests[0].body;
    assert_eq!(resumed.start_checkpoint, Some(6));
    assert_eq!(resumed.start_transaction_index, Some(1));
    assert_eq!(resumed.options.as_ref().unwrap().after, Some(bytes(50)));
}

/// An ascending item frame's position is the boundary just after its item, so its finest index is
/// at least 1.
#[tokio::test]
async fn subscription_item_frame_at_a_zero_finest_index_is_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_transaction_subscriptions([StreamScript::frames([Ok(with_position(
        transaction_live_frame(Some(5), 5),
        transaction_position(5, 0),
    ))])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let (frames, status) = transaction_frames_until_error(
        client.stream_transactions_with_config(TransactionStreamRequest::new(), fast_config()),
    )
    .await;
    assert!(frames.is_empty());
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(
        status.message(),
        "item frame cursor_position does not follow an item"
    );
    assert_eq!(
        server.state.lock().unwrap().calls,
        ["subscribe_transactions"]
    );
}

/// A frame without an item may name an earlier tuple for a boundary the subscription reached, but
/// an item must not lie before the subscription's furthest position.
#[tokio::test]
async fn subscription_item_behind_its_furthest_position_is_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_transaction_subscriptions([StreamScript::frames([
        Ok(transaction_live_frame(None, 6)),
        Ok(with_position(
            transaction_live_frame(None, 6),
            transaction_position(6, 0),
        )),
        Ok(transaction_positioned_live_frame(
            Some(transaction_at(6, 0, "behind")),
            7,
            6,
        )),
    ])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let (frames, status) = transaction_frames_until_error(
        client.stream_transactions_with_config(TransactionStreamRequest::new(), fast_config()),
    )
    .await;
    assert_eq!(
        frames
            .iter()
            .map(|frame| frame.position)
            .collect::<Vec<_>>(),
        [transaction_position(7, 0)]
    );
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(
        status.message(),
        "subscription item did not advance its cursor_position"
    );
    assert_eq!(
        server.state.lock().unwrap().calls,
        ["subscribe_transactions"]
    );
}

/// `(6, 3)` and `(7, 0)` name the same boundary when checkpoint 6 holds three transactions, so a
/// frame naming the lower tuple after the higher one makes no progress and is not yielded.
#[tokio::test]
async fn subscription_continues_past_an_earlier_tuple_for_the_same_boundary() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_transaction_subscriptions([StreamScript::frames([
        Ok(with_position(
            transaction_live_frame(None, 6),
            transaction_position(7, 0),
        )),
        Ok(with_position(
            transaction_live_frame(None, 6),
            transaction_position(6, 3),
        )),
        Ok(transaction_positioned_live_frame(
            Some(transaction_at(7, 0, "next")),
            70,
            6,
        )),
    ])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let frames = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_transactions_with_config(TransactionStreamRequest::new(), fast_config())
            .take(2)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("stream stalled after an earlier tuple for the same boundary")
    .unwrap();
    assert_eq!(
        frames
            .iter()
            .map(|frame| (
                frame
                    .transaction
                    .as_ref()
                    .map(|transaction| transaction.digest.clone().unwrap()),
                frame.position,
            ))
            .collect::<Vec<_>>(),
        [
            (None, transaction_position(7, 0)),
            (Some("next".to_owned()), transaction_position(7, 1)),
        ]
    );
    assert_eq!(
        server.state.lock().unwrap().calls,
        ["subscribe_transactions"]
    );
}

/// The List counterpart: the scan keeps resuming from its furthest position.
#[tokio::test]
async fn list_continues_past_an_earlier_tuple_for_the_same_boundary() {
    let (server, _calls) = ScriptedStreamServer::new();
    let frames = vec![
        transaction_positioned_list_frame(Some(transaction_at(6, 2, "last-of-6")), 62, 5, None),
        with_position(
            transaction_list_frame(None, 6, None),
            transaction_position(7, 0),
        ),
        with_position(
            transaction_list_frame(None, 6, None),
            transaction_position(6, 3),
        ),
        transaction_positioned_list_frame(Some(transaction_at(7, 0, "first-of-7")), 70, 6, None),
        transaction_list_frame(None, 9, Some(proto::QueryEndReason::CheckpointBound)),
    ];
    server.push_transaction_lists([StreamScript::frames(frames.clone().into_iter().map(Ok))]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let responses = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .list_transactions_with_config(
                proto::ListTransactionsRequest::default()
                    .with_start_checkpoint(6)
                    .with_end_checkpoint(10),
                fast_list_config(),
            )
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("List stalled after an earlier tuple for the same boundary")
    .unwrap();
    assert_eq!(responses, frames);
    assert_eq!(server.state.lock().unwrap().transaction_requests.len(), 1);
}
