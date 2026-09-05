use super::support::ScriptedStreamServer;
use super::support::StreamScript;
use super::support::bytes;
use super::support::checkpoint_identity_mask;
use super::support::checkpoint_list_frame;
use super::support::checkpoint_live_frame;
use super::support::event_identity_mask;
use super::support::event_list_frame;
use super::support::event_live_frame;
use super::support::fast_config;
use super::support::observed_client;
use super::support::recording_config;
use super::support::service_info;
use super::support::spawn_server;
use super::support::transaction_identity_mask;
use super::support::transaction_list_frame;
use super::support::transaction_live_frame;
use futures::StreamExt;
use futures::TryStreamExt;
use std::time::Duration;
use sui_rpc::client::CheckpointStreamRequest;
use sui_rpc::client::CheckpointStreamStart;
use sui_rpc::client::EventStreamRequest;
use sui_rpc::client::EventStreamStart;
use sui_rpc::client::LedgerStreamConfig;
use sui_rpc::client::LedgerStreamEvent;
use sui_rpc::client::TransactionStreamRequest;
use sui_rpc::client::TransactionStreamStart;
use sui_rpc::proto::sui::rpc::v2 as proto;

#[tokio::test]
async fn steady_live_streams_yield_advanced_progress_only_frames() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([
        Ok(service_info(0)),
        Ok(service_info(0)),
        Ok(service_info(0)),
    ]);
    server.push_checkpoint_subscriptions([StreamScript::frames([
        Ok(checkpoint_live_frame(None, 0)),
        Ok(checkpoint_live_frame(None, 2)),
    ])]);
    server.push_checkpoint_lists([StreamScript::frames([Ok(checkpoint_list_frame(
        None,
        0,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    server.push_transaction_subscriptions([StreamScript::frames([
        Ok(transaction_live_frame(None, 0)),
        Ok(transaction_live_frame(None, 2)),
    ])]);
    server.push_transaction_lists([StreamScript::frames([Ok(transaction_list_frame(
        None,
        0,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    server.push_event_subscriptions([StreamScript::frames([
        Ok(event_live_frame(None, 0)),
        Ok(event_live_frame(None, 2)),
    ])]);
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        0,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);
    let (config, mut observer_events) = recording_config(LedgerStreamConfig::default());

    let checkpoint_frames = client
        .stream_checkpoints_with_config(
            CheckpointStreamRequest::new()
                .with_read_mask(checkpoint_identity_mask())
                .with_filter(proto::TransactionFilter::default())
                .with_start(CheckpointStreamStart::Checkpoint(0)),
            config.clone(),
        )
        .take(2)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert!(
        checkpoint_frames
            .iter()
            .all(|frame| frame.checkpoint.is_none())
    );
    assert_eq!(
        checkpoint_frames
            .iter()
            .map(|frame| frame.cursor)
            .collect::<Vec<_>>(),
        [0, 2]
    );

    let transaction_frames = client
        .stream_transactions_with_config(
            TransactionStreamRequest::new()
                .with_read_mask(transaction_identity_mask())
                .with_start(TransactionStreamStart::Checkpoint(0)),
            config.clone(),
        )
        .take(2)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert!(
        transaction_frames
            .iter()
            .all(|frame| frame.transaction.is_none())
    );
    assert_eq!(
        transaction_frames
            .iter()
            .map(|frame| frame.cursor.clone())
            .collect::<Vec<_>>(),
        [bytes(0), bytes(2)]
    );

    let event_frames = client
        .stream_events_with_config(
            EventStreamRequest::new()
                .with_read_mask(event_identity_mask())
                .with_start(EventStreamStart::Checkpoint(0)),
            config,
        )
        .take(2)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert!(event_frames.iter().all(|frame| frame.event.is_none()));
    assert_eq!(
        event_frames
            .iter()
            .map(|frame| frame.cursor.clone())
            .collect::<Vec<_>>(),
        [bytes(0), bytes(2)]
    );

    let mut events = Vec::new();
    while let Ok(event) = observer_events.try_recv() {
        events.push(event);
    }
    assert_eq!(events.len(), 9);
    assert!(
        events
            .iter()
            .all(|event| matches!(event, LedgerStreamEvent::RpcResponse { .. }))
    );
}

#[tokio::test]
async fn live_tip_uses_subscribe_delivery_for_all_families() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_checkpoint_subscriptions([StreamScript::frames([Ok(checkpoint_live_frame(
        Some(11),
        11,
    ))])]);
    server.push_transaction_subscriptions([StreamScript::frames([
        Ok(transaction_live_frame(None, 11)),
        Ok(transaction_live_frame(Some(12), 12)),
    ])]);
    server.push_event_subscriptions([StreamScript::frames([
        Ok(event_live_frame(None, 12)),
        Ok(event_live_frame(Some(13), 13)),
    ])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let checkpoint_stream = client.stream_checkpoints(
        CheckpointStreamRequest::new().with_read_mask(checkpoint_identity_mask()),
    );
    futures::pin_mut!(checkpoint_stream);
    let checkpoint_frame = tokio::time::timeout(Duration::from_secs(2), checkpoint_stream.next())
        .await
        .expect("timed out waiting for live-tip checkpoint")
        .unwrap()
        .unwrap();
    assert_eq!(
        checkpoint_frame.checkpoint.unwrap().sequence_number,
        Some(11)
    );

    let transaction_frames = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_transactions(
                TransactionStreamRequest::new().with_read_mask(transaction_identity_mask()),
            )
            .take(2)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for live-tip transactions")
    .unwrap();
    assert!(transaction_frames[0].transaction.is_none());
    assert_eq!(transaction_frames[0].covered_checkpoint, Some(11));
    assert_eq!(
        transaction_frames[1]
            .transaction
            .as_ref()
            .unwrap()
            .digest
            .as_deref(),
        Some("tx-12")
    );

    let event_frames = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_events(EventStreamRequest::new().with_read_mask(event_identity_mask()))
            .take(2)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for live-tip events")
    .unwrap();
    assert!(event_frames[0].event.is_none());
    assert_eq!(event_frames[0].covered_checkpoint, Some(12));
    assert_eq!(
        event_frames[1]
            .event
            .as_ref()
            .unwrap()
            .event_type
            .as_deref(),
        Some("event-13")
    );

    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        [
            "subscribe_checkpoints",
            "subscribe_transactions",
            "subscribe_events"
        ]
    );
    assert!(state.service_info_requests.is_empty());
    assert!(state.checkpoint_requests.is_empty());
    assert!(state.transaction_requests.is_empty());
    assert!(state.event_requests.is_empty());
}

#[tokio::test]
async fn subscription_initial_frame_without_checkpoint_coverage_is_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    let mut transaction_boundary = transaction_live_frame(None, 20);
    transaction_boundary.watermark.as_mut().unwrap().checkpoint = None;
    let mut event_boundary = event_live_frame(None, 30);
    event_boundary.watermark.as_mut().unwrap().checkpoint = None;
    server.push_transaction_subscriptions([StreamScript::frames([Ok(transaction_boundary)])]);
    server.push_event_subscriptions([StreamScript::frames([Ok(event_boundary)])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let transactions = client.stream_transactions(
        TransactionStreamRequest::new().with_read_mask(transaction_identity_mask()),
    );
    futures::pin_mut!(transactions);
    let transaction_status = tokio::time::timeout(Duration::from_secs(2), transactions.next())
        .await
        .expect("timed out waiting for invalid transaction boundary")
        .unwrap()
        .unwrap_err();
    assert_eq!(transaction_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        transaction_status.message(),
        "subscription initial frame is missing checkpoint coverage"
    );
    assert!(transactions.next().await.is_none());

    let events =
        client.stream_events(EventStreamRequest::new().with_read_mask(event_identity_mask()));
    futures::pin_mut!(events);
    let event_status = tokio::time::timeout(Duration::from_secs(2), events.next())
        .await
        .expect("timed out waiting for invalid event boundary")
        .unwrap()
        .unwrap_err();
    assert_eq!(event_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        event_status.message(),
        "subscription initial frame is missing checkpoint coverage"
    );
    assert!(events.next().await.is_none());
    assert_eq!(
        server.state.lock().unwrap().calls,
        ["subscribe_transactions", "subscribe_events"]
    );
}

#[tokio::test]
async fn startup_transient_failure_uses_list_baseline_before_reconnecting() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(40))]);
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        40,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    server.push_event_subscriptions([
        StreamScript::frames([Err(tonic::Status::unavailable(
            "initial subscription failed before its first frame",
        ))]),
        StreamScript::frames([
            Ok(event_live_frame(None, 40)),
            Ok(event_live_frame(Some(41), 41)),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let frames = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_events_with_config(
                EventStreamRequest::new().with_read_mask(event_identity_mask()),
                fast_config(),
            )
            .take(1)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for startup retry")
    .unwrap();
    assert_eq!(frames.len(), 1);
    assert_eq!(
        frames[0].event.as_ref().unwrap().event_type.as_deref(),
        Some("event-41")
    );
    assert_eq!(frames[0].cursor, bytes(41));
    assert_eq!(
        server.state.lock().unwrap().calls,
        [
            "subscribe_events",
            "get_service_info",
            "list_events",
            "subscribe_events",
        ]
    );
}

#[tokio::test]
async fn startup_nontransient_failure_is_terminal() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_subscriptions([StreamScript::frames([Err(
        tonic::Status::invalid_argument("invalid startup request"),
    )])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let stream = client.stream_events_with_config(
        EventStreamRequest::new().with_read_mask(event_identity_mask()),
        fast_config(),
    );
    futures::pin_mut!(stream);

    let status = tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .expect("timed out waiting for terminal startup failure")
        .unwrap()
        .unwrap_err();
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
    assert_eq!(status.message(), "invalid startup request");
    assert!(stream.next().await.is_none());
    assert_eq!(server.state.lock().unwrap().calls, ["subscribe_events"]);
}

#[tokio::test]
async fn stream_read_masks_require_identity_but_lists_allow_any_projection() {
    async fn assert_invalid<T: std::fmt::Debug>(
        stream: impl futures::Stream<Item = Result<T, tonic::Status>>,
        expected_message: &str,
    ) {
        futures::pin_mut!(stream);
        let status = tokio::time::timeout(Duration::from_secs(2), stream.next())
            .await
            .expect("timed out waiting for lazy read-mask validation")
            .unwrap()
            .unwrap_err();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert_eq!(status.message(), expected_message);
    }

    fn field_mask(paths: &[&str]) -> prost_types::FieldMask {
        prost_types::FieldMask {
            paths: paths.iter().map(|path| (*path).to_owned()).collect(),
        }
    }

    let (server, _calls) = ScriptedStreamServer::new();
    let mut projected_checkpoint = checkpoint_list_frame(Some(1), 1, None);
    projected_checkpoint
        .checkpoint
        .as_mut()
        .unwrap()
        .sequence_number = None;
    server.push_checkpoint_lists([StreamScript::frames([
        Ok(projected_checkpoint),
        Ok(checkpoint_list_frame(
            None,
            1,
            Some(proto::QueryEndReason::LedgerTip),
        )),
    ])]);
    let mut projected_transaction = transaction_list_frame(Some(1), 1, None);
    let projected_transaction_item = projected_transaction.transaction.as_mut().unwrap();
    projected_transaction_item.checkpoint = None;
    projected_transaction_item.transaction_index = None;
    server.push_transaction_lists([StreamScript::frames([
        Ok(projected_transaction),
        Ok(transaction_list_frame(
            None,
            1,
            Some(proto::QueryEndReason::LedgerTip),
        )),
    ])]);
    let mut projected_event = event_list_frame(Some(1), 1, None);
    let projected_event_item = projected_event.event.as_mut().unwrap();
    projected_event_item.checkpoint = None;
    projected_event_item.transaction_index = None;
    projected_event_item.event_index = None;
    server.push_event_lists([StreamScript::frames([
        Ok(projected_event),
        Ok(event_list_frame(
            None,
            1,
            Some(proto::QueryEndReason::LedgerTip),
        )),
    ])]);
    let address = spawn_server(server.clone()).await;
    let (client, observations) = observed_client(address);

    let checkpoint_message = r#"read_mask must include "sequence_number" or "*""#;
    for read_mask in [
        None,
        Some(prost_types::FieldMask::default()),
        Some(field_mask(&["digest"])),
    ] {
        let mut stream_request = CheckpointStreamRequest::new();
        stream_request.read_mask = read_mask;
        assert_invalid(
            client.stream_checkpoints(stream_request),
            checkpoint_message,
        )
        .await;
        assert!(server.state.lock().unwrap().calls.is_empty());
    }

    let transaction_message =
        r#"read_mask must include "checkpoint" and "transaction_index" or "*""#;
    for read_mask in [
        None,
        Some(prost_types::FieldMask::default()),
        Some(field_mask(&["checkpoint"])),
        Some(field_mask(&["transaction_index"])),
    ] {
        let mut stream_request = TransactionStreamRequest::new();
        stream_request.read_mask = read_mask;
        assert_invalid(
            client.stream_transactions(stream_request),
            transaction_message,
        )
        .await;
        assert!(server.state.lock().unwrap().calls.is_empty());
    }

    let event_message =
        r#"read_mask must include "checkpoint", "transaction_index", and "event_index" or "*""#;
    for read_mask in [
        None,
        Some(prost_types::FieldMask::default()),
        Some(field_mask(&["transaction_index", "event_index"])),
        Some(field_mask(&["checkpoint", "event_index"])),
        Some(field_mask(&["checkpoint", "transaction_index"])),
    ] {
        let mut stream_request = EventStreamRequest::new();
        stream_request.read_mask = read_mask;
        assert_invalid(client.stream_events(stream_request), event_message).await;
        assert!(server.state.lock().unwrap().calls.is_empty());
    }
    assert!(observations.lock().unwrap().is_empty());

    let checkpoint = client.list_checkpoints(
        proto::ListCheckpointsRequest::default().with_read_mask(field_mask(&["digest"])),
    );
    futures::pin_mut!(checkpoint);
    assert!(
        checkpoint
            .next()
            .await
            .unwrap()
            .unwrap()
            .checkpoint
            .is_some()
    );
    assert!(
        checkpoint
            .next()
            .await
            .unwrap()
            .unwrap()
            .checkpoint
            .is_none()
    );
    assert!(checkpoint.next().await.is_none());

    let transaction = client.list_transactions(
        proto::ListTransactionsRequest::default().with_read_mask(field_mask(&["digest"])),
    );
    futures::pin_mut!(transaction);
    assert!(
        transaction
            .next()
            .await
            .unwrap()
            .unwrap()
            .transaction
            .is_some()
    );
    assert!(
        transaction
            .next()
            .await
            .unwrap()
            .unwrap()
            .transaction
            .is_none()
    );
    assert!(transaction.next().await.is_none());

    let event = client.list_events(
        proto::ListEventsRequest::default().with_read_mask(field_mask(&["event_type"])),
    );
    futures::pin_mut!(event);
    assert!(event.next().await.unwrap().unwrap().event.is_some());
    assert!(event.next().await.unwrap().unwrap().event.is_none());
    assert!(event.next().await.is_none());
    assert_eq!(
        server.state.lock().unwrap().calls,
        ["list_checkpoints", "list_transactions", "list_events"]
    );
}

#[tokio::test]
async fn service_info_bootstrap_and_subscription_handoff_have_no_gap_or_duplicate() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(5))]);
    server.push_checkpoint_subscriptions([StreamScript::frames([
        Ok(checkpoint_live_frame(Some(7), 7)),
        Ok(checkpoint_live_frame(Some(8), 8)),
    ])]);
    server.push_checkpoint_lists([
        StreamScript::frames(
            (0..=5)
                .map(|value| Ok(checkpoint_list_frame(Some(value), value, None)))
                .chain([Ok(checkpoint_list_frame(
                    None,
                    5,
                    Some(proto::QueryEndReason::CheckpointBound),
                ))]),
        ),
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(6), 6, None)),
            Ok(checkpoint_list_frame(Some(7), 7, None)),
            Ok(checkpoint_list_frame(
                None,
                7,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, observations) = observed_client(address);

    let body = CheckpointStreamRequest::new()
        .with_read_mask(prost_types::FieldMask {
            paths: vec!["sequence_number".to_owned()],
        })
        .with_filter(proto::TransactionFilter::default())
        .with_start(CheckpointStreamStart::Checkpoint(0));
    let checkpoints = client
        .stream_checkpoints_with_config(body, fast_config())
        .take(9)
        .map_ok(|checkpoint| checkpoint.checkpoint.unwrap().sequence_number.unwrap())
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(checkpoints, (0..=8).collect::<Vec<_>>());

    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        [
            "get_service_info",
            "list_checkpoints",
            "subscribe_checkpoints",
            "list_checkpoints"
        ]
    );
    assert_eq!(state.checkpoint_requests[0].body.end_checkpoint, Some(6));
    assert_eq!(state.checkpoint_requests[1].body.start_checkpoint, Some(6));
    assert_eq!(state.checkpoint_requests[1].body.end_checkpoint, Some(8));
    assert_eq!(state.checkpoint_subscriptions.len(), 1);
    assert_eq!(
        state.checkpoint_subscriptions[0].body.read_mask,
        Some(prost_types::FieldMask {
            paths: vec!["sequence_number".to_owned()],
        })
    );
    assert_eq!(
        state.checkpoint_subscriptions[0].body.filter,
        Some(proto::TransactionFilter::default())
    );
    drop(state);

    let observations = observations.lock().unwrap();
    assert_eq!(observations.len(), 4);
    assert!(
        observations
            .iter()
            .any(|observation| observation.path.contains("GetServiceInfo"))
    );
    assert!(
        observations
            .iter()
            .any(|observation| observation.path.contains("Subscribe"))
    );
    assert!(
        observations
            .iter()
            .any(|observation| observation.path.contains("List"))
    );
}

#[tokio::test]
async fn checkpoint_start_preserves_stream_fields_and_internal_page_limit() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([
        Ok(service_info(0)),
        Ok(service_info(0)),
        Ok(service_info(0)),
    ]);
    server.push_checkpoint_lists([
        StreamScript::frames([Ok(checkpoint_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(1), 1, None)),
            Ok(checkpoint_list_frame(
                None,
                2,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    server.push_transaction_lists([
        StreamScript::frames([Ok(transaction_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::frames([
            Ok(transaction_list_frame(Some(1), 1, None)),
            Ok(transaction_list_frame(
                None,
                2,
                Some(proto::QueryEndReason::CursorBound),
            )),
        ]),
    ]);
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::frames([
            Ok(event_list_frame(Some(1), 1, None)),
            Ok(event_list_frame(
                None,
                2,
                Some(proto::QueryEndReason::CursorBound),
            )),
        ]),
    ]);
    server.push_checkpoint_subscriptions([StreamScript::frames([Ok(checkpoint_live_frame(
        Some(2),
        2,
    ))])]);
    server.push_transaction_subscriptions([StreamScript::frames([Ok(transaction_live_frame(
        Some(2),
        2,
    ))])]);
    server.push_event_subscriptions([StreamScript::frames([Ok(event_live_frame(Some(2), 2))])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let mut config = fast_config();
    config.list_page_limit = Some(7);
    let checkpoint_frames = client
        .stream_checkpoints_with_config(
            CheckpointStreamRequest::new()
                .with_read_mask(prost_types::FieldMask {
                    paths: vec!["digest".to_owned(), "sequence_number".to_owned()],
                })
                .with_filter(proto::TransactionFilter::default())
                .with_start(CheckpointStreamStart::Checkpoint(0)),
            config.clone(),
        )
        .take(3)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert!(checkpoint_frames[0].checkpoint.is_none());
    assert_eq!(
        checkpoint_frames[1]
            .checkpoint
            .as_ref()
            .unwrap()
            .sequence_number,
        Some(1)
    );

    let transaction_frames = client
        .stream_transactions_with_config(
            TransactionStreamRequest::new()
                .with_read_mask(prost_types::FieldMask {
                    paths: vec![
                        "digest".to_owned(),
                        "checkpoint".to_owned(),
                        "transaction_index".to_owned(),
                    ],
                })
                .with_filter(proto::TransactionFilter::default())
                .with_start(TransactionStreamStart::Checkpoint(0)),
            config.clone(),
        )
        .take(3)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert!(transaction_frames[0].transaction.is_none());
    let transaction = transaction_frames[1].transaction.as_ref().unwrap();
    assert_eq!(transaction.checkpoint, Some(1));
    assert_eq!(transaction.transaction_index, Some(0));

    let event_frames = client
        .stream_events_with_config(
            EventStreamRequest::new()
                .with_read_mask(prost_types::FieldMask {
                    paths: vec![
                        "event_type".to_owned(),
                        "checkpoint".to_owned(),
                        "transaction_index".to_owned(),
                        "event_index".to_owned(),
                    ],
                })
                .with_filter(proto::EventFilter::default())
                .with_start(EventStreamStart::Checkpoint(0)),
            config,
        )
        .take(3)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert!(event_frames[0].event.is_none());
    let event = event_frames[1].event.as_ref().unwrap();
    assert_eq!(event.checkpoint, Some(1));
    assert_eq!(event.transaction_index, Some(0));
    assert_eq!(event.event_index, Some(0));

    let state = server.state.lock().unwrap();
    for (start_checkpoint, end_checkpoint, options) in [
        (
            state.checkpoint_requests[0].body.start_checkpoint,
            state.checkpoint_requests[0].body.end_checkpoint,
            state.checkpoint_requests[0].body.options.as_ref().unwrap(),
        ),
        (
            state.transaction_requests[0].body.start_checkpoint,
            state.transaction_requests[0].body.end_checkpoint,
            state.transaction_requests[0].body.options.as_ref().unwrap(),
        ),
        (
            state.event_requests[0].body.start_checkpoint,
            state.event_requests[0].body.end_checkpoint,
            state.event_requests[0].body.options.as_ref().unwrap(),
        ),
    ] {
        assert_eq!(start_checkpoint, Some(0));
        assert_eq!(end_checkpoint, Some(1));
        assert_eq!(options.limit, Some(7));
        assert!(options.ordering.is_none());
    }
    let checkpoint_recovery = &state.checkpoint_requests[1].body;
    assert_eq!(checkpoint_recovery.start_checkpoint, Some(1));
    assert_eq!(checkpoint_recovery.end_checkpoint, Some(3));
    let checkpoint_recovery_options = checkpoint_recovery.options.as_ref().unwrap();
    assert!(checkpoint_recovery_options.after.is_none());
    assert!(checkpoint_recovery_options.before.is_none());
    assert_eq!(checkpoint_recovery_options.limit, Some(7));
    assert!(checkpoint_recovery_options.ordering.is_none());
    for (start_checkpoint, end_checkpoint, options) in [
        (
            state.transaction_requests[1].body.start_checkpoint,
            state.transaction_requests[1].body.end_checkpoint,
            state.transaction_requests[1].body.options.as_ref().unwrap(),
        ),
        (
            state.event_requests[1].body.start_checkpoint,
            state.event_requests[1].body.end_checkpoint,
            state.event_requests[1].body.options.as_ref().unwrap(),
        ),
    ] {
        assert!(start_checkpoint.is_none());
        assert!(end_checkpoint.is_none());
        assert_eq!(options.after, Some(bytes(0)));
        assert_eq!(options.before, Some(bytes(2)));
        assert_eq!(options.limit, Some(7));
        assert!(options.ordering.is_none());
    }
    assert_eq!(
        state.checkpoint_subscriptions[0].body.read_mask,
        state.checkpoint_requests[0].body.read_mask
    );
    assert_eq!(
        state.checkpoint_subscriptions[0].body.filter,
        state.checkpoint_requests[0].body.filter
    );
    assert_eq!(
        state.transaction_subscriptions[0].body.read_mask,
        state.transaction_requests[0].body.read_mask
    );
    assert_eq!(
        state.transaction_subscriptions[0].body.filter,
        state.transaction_requests[0].body.filter
    );
    assert_eq!(
        state.event_subscriptions[0].body.read_mask,
        state.event_requests[0].body.read_mask
    );
    assert_eq!(
        state.event_subscriptions[0].body.filter,
        state.event_requests[0].body.filter
    );
}

#[tokio::test]
async fn first_post_history_subscription_requires_checkpoint_coverage() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(5))]);
    let mut invalid_boundary = event_live_frame(None, 6);
    invalid_boundary.watermark.as_mut().unwrap().checkpoint = None;
    server.push_event_subscriptions([StreamScript::frames([Ok(invalid_boundary)])]);
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        5,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let stream = client.stream_events_with_config(
        EventStreamRequest::new()
            .with_read_mask(event_identity_mask())
            .with_start(EventStreamStart::Checkpoint(0)),
        fast_config(),
    );
    futures::pin_mut!(stream);

    let history = stream.next().await.unwrap().unwrap();
    assert_eq!(history.cursor, bytes(5));
    assert_eq!(history.covered_checkpoint, Some(5));
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(
        status.message(),
        "subscription initial frame is missing checkpoint coverage"
    );
    assert!(stream.next().await.is_none());
    let state = server.state.lock().unwrap();
    assert_eq!(state.event_subscriptions.len(), 1);
    assert_eq!(state.event_requests.len(), 1);
}

#[tokio::test]
async fn maximum_service_info_or_new_subscription_checkpoint_is_out_of_range() {
    let (service_info_server, _calls) = ScriptedStreamServer::new();
    service_info_server.push_service_infos([Ok(service_info(u64::MAX))]);
    let address = spawn_server(service_info_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let stream = client.stream_checkpoints_with_config(
        CheckpointStreamRequest::new()
            .with_read_mask(checkpoint_identity_mask())
            .with_start(CheckpointStreamStart::Checkpoint(0)),
        fast_config(),
    );
    futures::pin_mut!(stream);
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::OutOfRange);
    assert!(
        service_info_server
            .state
            .lock()
            .unwrap()
            .checkpoint_requests
            .is_empty()
    );
    assert!(
        service_info_server
            .state
            .lock()
            .unwrap()
            .checkpoint_subscriptions
            .is_empty()
    );

    let (new_subscription_server, _calls) = ScriptedStreamServer::new();
    new_subscription_server.push_service_infos([Ok(service_info(0))]);
    new_subscription_server.push_checkpoint_subscriptions([StreamScript::frames([Ok(
        checkpoint_live_frame(None, u64::MAX),
    )])]);
    new_subscription_server.push_checkpoint_lists([StreamScript::frames([Ok(
        checkpoint_list_frame(None, 0, Some(proto::QueryEndReason::CheckpointBound)),
    )])]);
    let address = spawn_server(new_subscription_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let stream = client.stream_checkpoints_with_config(
        CheckpointStreamRequest::new()
            .with_read_mask(checkpoint_identity_mask())
            .with_filter(proto::TransactionFilter::default())
            .with_start(CheckpointStreamStart::Checkpoint(0)),
        fast_config(),
    );
    futures::pin_mut!(stream);
    let progress = stream.next().await.unwrap().unwrap();
    assert!(progress.checkpoint.is_none());
    assert_eq!(progress.cursor, 0);
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::OutOfRange);
    assert_eq!(
        new_subscription_server
            .state
            .lock()
            .unwrap()
            .checkpoint_requests
            .len(),
        1
    );
}
