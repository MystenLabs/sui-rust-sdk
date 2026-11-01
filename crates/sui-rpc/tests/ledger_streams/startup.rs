use super::support::ScriptedStreamServer;
use super::support::StreamScript;
use super::support::bytes;
use super::support::checkpoint_identity_mask;
use super::support::checkpoint_list_frame;
use super::support::checkpoint_live_frame;
use super::support::checkpoint_position;
use super::support::event_at;
use super::support::event_identity_mask;
use super::support::event_list_frame;
use super::support::event_live_frame;
use super::support::event_position;
use super::support::event_positioned_list_frame;
use super::support::fast_config;
use super::support::observed_client;
use super::support::recording_config;
use super::support::service_info;
use super::support::spawn_server;
use super::support::transaction_at;
use super::support::transaction_identity_mask;
use super::support::transaction_list_frame;
use super::support::transaction_live_frame;
use super::support::transaction_position;
use super::support::transaction_positioned_list_frame;
use super::support::transaction_positioned_live_frame;
use super::support::with_position;
use futures::StreamExt;
use futures::TryStreamExt;
use std::time::Duration;
use sui_rpc::client::CheckpointStreamRequest;
use sui_rpc::client::CheckpointStreamStart;
use sui_rpc::client::Delivery;
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

    // Each subscription's first frame names the position the List history committed, so it is
    // not yielded again; the next frame advances the position and is.
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
            .map(|frame| (frame.cursor.clone(), frame.position))
            .collect::<Vec<_>>(),
        [
            (bytes(0), transaction_position(1, 0)),
            (bytes(2), transaction_position(3, 0)),
        ]
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
            .map(|frame| (frame.cursor.clone(), frame.position))
            .collect::<Vec<_>>(),
        [
            (bytes(0), event_position(1, 0, 0)),
            (bytes(2), event_position(3, 0, 0)),
        ]
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
    assert_eq!(checkpoint_frame.cursor, 11);

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
    assert_eq!(transaction_frames[0].position, transaction_position(12, 0));
    assert_eq!(
        transaction_frames[1]
            .transaction
            .as_ref()
            .unwrap()
            .digest
            .as_deref(),
        Some("tx-12")
    );
    assert_eq!(transaction_frames[1].position, transaction_position(12, 1));

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
    assert_eq!(event_frames[0].position, event_position(13, 0, 0));
    assert_eq!(
        event_frames[1]
            .event
            .as_ref()
            .unwrap()
            .event_type
            .as_deref(),
        Some("event-13")
    );
    assert_eq!(event_frames[1].position, event_position(13, 0, 1));

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
async fn live_tip_initial_frame_without_checkpoint_coverage_is_yielded_at_its_position() {
    let (server, _calls) = ScriptedStreamServer::new();
    let mut transaction_boundary = transaction_live_frame(None, 20);
    transaction_boundary.watermark.as_mut().unwrap().checkpoint = None;
    let mut event_boundary = event_live_frame(None, 30);
    event_boundary.watermark.as_mut().unwrap().checkpoint = None;
    server.push_transaction_subscriptions([StreamScript::frames([
        Ok(transaction_boundary),
        Ok(transaction_live_frame(Some(21), 21)),
    ])]);
    server.push_event_subscriptions([StreamScript::frames([
        Ok(event_boundary),
        Ok(event_live_frame(Some(31), 31)),
    ])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let transactions = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_transactions(TransactionStreamRequest::new())
            .take(2)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for live-tip transactions")
    .unwrap();
    assert!(transactions[0].transaction.is_none());
    assert_eq!(transactions[0].covered_checkpoint, None);
    assert_eq!(transactions[0].position, transaction_position(21, 0));
    assert!(transactions[1].transaction.is_some());
    assert_eq!(transactions[1].covered_checkpoint, Some(21));
    assert_eq!(transactions[1].position, transaction_position(21, 1));

    let events = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_events(EventStreamRequest::new())
            .take(2)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for live-tip events")
    .unwrap();
    assert!(events[0].event.is_none());
    assert_eq!(events[0].covered_checkpoint, None);
    assert_eq!(events[0].position, event_position(31, 0, 0));
    assert!(events[1].event.is_some());
    assert_eq!(events[1].covered_checkpoint, Some(31));
    assert_eq!(events[1].position, event_position(31, 0, 1));
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
    assert_eq!(frames[0].position, event_position(41, 0, 1));
    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        [
            "subscribe_events",
            "get_service_info",
            "list_events",
            "subscribe_events",
        ]
    );
    // The baseline covers exactly the tip checkpoint, from no cursor.
    let baseline = &state.event_requests[0].body;
    assert_eq!(baseline.start_checkpoint, Some(40));
    assert_eq!(baseline.start_transaction_index, None);
    assert_eq!(baseline.start_event_index, None);
    assert_eq!(baseline.end_checkpoint, Some(41));
    assert_eq!(baseline.end_transaction_index, None);
    assert_eq!(baseline.end_event_index, None);
    let baseline_options = baseline.options.as_ref().unwrap();
    assert_eq!(baseline_options.after, None);
    assert_eq!(baseline_options.before, None);
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

/// Streams key items by their frame's `cursor_position`, so neither streams nor lists need the
/// item identity fields in their projection.
#[tokio::test]
async fn streams_and_lists_accept_items_without_identity_fields() {
    fn field_mask(paths: &[&str]) -> prost_types::FieldMask {
        prost_types::FieldMask {
            paths: paths.iter().map(|path| (*path).to_owned()).collect(),
        }
    }

    let (server, _calls) = ScriptedStreamServer::new();
    let mut projected_live_checkpoint = checkpoint_live_frame(Some(11), 11);
    projected_live_checkpoint
        .checkpoint
        .as_mut()
        .unwrap()
        .sequence_number = None;
    server.push_checkpoint_subscriptions([StreamScript::frames([Ok(projected_live_checkpoint)])]);
    let mut projected_live_transaction = transaction_live_frame(Some(12), 12);
    let projected_live_transaction_item = projected_live_transaction.transaction.as_mut().unwrap();
    projected_live_transaction_item.checkpoint = None;
    projected_live_transaction_item.transaction_index = None;
    server.push_transaction_subscriptions([StreamScript::frames([Ok(projected_live_transaction)])]);
    let mut projected_live_event = event_live_frame(Some(13), 13);
    let projected_live_event_item = projected_live_event.event.as_mut().unwrap();
    projected_live_event_item.checkpoint = None;
    projected_live_event_item.transaction_index = None;
    projected_live_event_item.event_index = None;
    server.push_event_subscriptions([StreamScript::frames([Ok(projected_live_event)])]);

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
    let (client, _observations) = observed_client(address);

    let checkpoints = client.stream_checkpoints(CheckpointStreamRequest::new());
    futures::pin_mut!(checkpoints);
    let checkpoint_frame = tokio::time::timeout(Duration::from_secs(2), checkpoints.next())
        .await
        .expect("timed out waiting for a projected checkpoint")
        .unwrap()
        .unwrap();
    assert!(checkpoint_frame.checkpoint.is_some());
    assert_eq!(checkpoint_frame.cursor, 11);

    let transactions = client.stream_transactions(
        TransactionStreamRequest::new().with_read_mask(field_mask(&["digest"])),
    );
    futures::pin_mut!(transactions);
    let transaction_frame = tokio::time::timeout(Duration::from_secs(2), transactions.next())
        .await
        .expect("timed out waiting for a projected transaction")
        .unwrap()
        .unwrap();
    assert_eq!(
        transaction_frame.transaction.unwrap().digest.as_deref(),
        Some("tx-12")
    );
    assert_eq!(transaction_frame.position, transaction_position(12, 1));

    let events = client
        .stream_events(EventStreamRequest::new().with_read_mask(prost_types::FieldMask::default()));
    futures::pin_mut!(events);
    let event_frame = tokio::time::timeout(Duration::from_secs(2), events.next())
        .await
        .expect("timed out waiting for a projected event")
        .unwrap()
        .unwrap();
    assert_eq!(
        event_frame.event.unwrap().event_type.as_deref(),
        Some("event-13")
    );
    assert_eq!(event_frame.position, event_position(13, 0, 1));

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

    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        [
            "subscribe_checkpoints",
            "subscribe_transactions",
            "subscribe_events",
            "list_checkpoints",
            "list_transactions",
            "list_events"
        ]
    );
    assert_eq!(state.checkpoint_subscriptions[0].body.read_mask, None);
    assert_eq!(
        state.transaction_subscriptions[0].body.read_mask,
        Some(field_mask(&["digest"]))
    );
    assert_eq!(
        state.event_subscriptions[0].body.read_mask,
        Some(prost_types::FieldMask::default())
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
        // The gap ends before the subscription's first checkpoint, which the subscription
        // delivers itself.
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(6), 6, None)),
            Ok(checkpoint_list_frame(
                None,
                6,
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
    assert_eq!(state.checkpoint_requests[0].body.start_checkpoint, Some(0));
    assert_eq!(state.checkpoint_requests[0].body.end_checkpoint, Some(6));
    assert_eq!(state.checkpoint_requests[1].body.start_checkpoint, Some(6));
    assert_eq!(state.checkpoint_requests[1].body.end_checkpoint, Some(7));
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
    // Each subscription's first item lies at checkpoint 2, past the history's position, so a gap
    // List covers everything from that position up to the item.
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
                1,
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
            Ok(transaction_positioned_list_frame(
                None,
                2,
                1,
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
            Ok(event_positioned_list_frame(
                None,
                2,
                1,
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
                    paths: vec!["digest".to_owned()],
                })
                .with_filter(proto::TransactionFilter::default())
                .with_start(CheckpointStreamStart::Checkpoint(0)),
            config.clone(),
        )
        .take(3)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        checkpoint_frames
            .iter()
            .map(|frame| (frame.checkpoint.is_some(), frame.cursor))
            .collect::<Vec<_>>(),
        [(false, 0), (true, 1), (true, 2)]
    );

    let transaction_frames = client
        .stream_transactions_with_config(
            TransactionStreamRequest::new()
                .with_read_mask(prost_types::FieldMask {
                    paths: vec!["digest".to_owned()],
                })
                .with_filter(proto::TransactionFilter::default())
                .with_start(TransactionStreamStart::Checkpoint(0)),
            config.clone(),
        )
        .take(3)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        transaction_frames
            .iter()
            .map(|frame| (frame.transaction.is_some(), frame.position))
            .collect::<Vec<_>>(),
        [
            (false, transaction_position(1, 0)),
            (true, transaction_position(1, 1)),
            (false, transaction_position(2, 0)),
        ]
    );

    let event_frames = client
        .stream_events_with_config(
            EventStreamRequest::new()
                .with_read_mask(prost_types::FieldMask {
                    paths: vec!["event_type".to_owned()],
                })
                .with_filter(proto::EventFilter::default())
                .with_start(EventStreamStart::Checkpoint(0)),
            config,
        )
        .take(3)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        event_frames
            .iter()
            .map(|frame| (frame.event.is_some(), frame.position))
            .collect::<Vec<_>>(),
        [
            (false, event_position(1, 0, 0)),
            (true, event_position(1, 0, 1)),
            (false, event_position(2, 0, 0)),
        ]
    );

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
        assert!(options.after.is_none());
        assert!(options.before.is_none());
        assert_eq!(options.limit, Some(7));
        assert!(options.ordering.is_none());
    }

    // The checkpoint gap is `[1, 2)`, bounded by checkpoint numbers alone.
    let checkpoint_recovery = &state.checkpoint_requests[1].body;
    assert_eq!(checkpoint_recovery.start_checkpoint, Some(1));
    assert_eq!(checkpoint_recovery.end_checkpoint, Some(2));
    let checkpoint_recovery_options = checkpoint_recovery.options.as_ref().unwrap();
    assert!(checkpoint_recovery_options.after.is_none());
    assert!(checkpoint_recovery_options.before.is_none());
    assert_eq!(checkpoint_recovery_options.limit, Some(7));
    assert!(checkpoint_recovery_options.ordering.is_none());

    // The transaction gap runs from the history's position `(1, 0)` to the subscription item's
    // key `(2, 0)`, both checkpoint starts, so neither end needs an index.
    let transaction_recovery = &state.transaction_requests[1].body;
    assert_eq!(transaction_recovery.start_checkpoint, Some(1));
    assert_eq!(transaction_recovery.start_transaction_index, None);
    assert_eq!(transaction_recovery.end_checkpoint, Some(2));
    assert_eq!(transaction_recovery.end_transaction_index, None);
    let transaction_recovery_options = transaction_recovery.options.as_ref().unwrap();
    assert_eq!(transaction_recovery_options.after, Some(bytes(0)));
    assert_eq!(transaction_recovery_options.before, Some(bytes(2)));
    assert_eq!(transaction_recovery_options.limit, Some(7));
    assert!(transaction_recovery_options.ordering.is_none());

    // The event gap runs from `(1, 0, 0)` to the subscription item's key `(2, 0, 0)`.
    let event_recovery = &state.event_requests[1].body;
    assert_eq!(event_recovery.start_checkpoint, Some(1));
    assert_eq!(event_recovery.start_transaction_index, None);
    assert_eq!(event_recovery.start_event_index, None);
    assert_eq!(event_recovery.end_checkpoint, Some(2));
    assert_eq!(event_recovery.end_transaction_index, None);
    assert_eq!(event_recovery.end_event_index, None);
    let event_recovery_options = event_recovery.options.as_ref().unwrap();
    assert_eq!(event_recovery_options.after, Some(bytes(0)));
    assert_eq!(event_recovery_options.before, Some(bytes(2)));
    assert_eq!(event_recovery_options.limit, Some(7));
    assert!(event_recovery_options.ordering.is_none());

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

/// A stream started at a checkpoint reports no progress before it delivers that checkpoint. Here
/// the first List page ends at the start of checkpoint 0 without an item; a frame for it would
/// report cursor 0, the cursor of checkpoint 0 itself, so the stream yields nothing for it.
#[tokio::test]
async fn checkpoint_start_reports_no_cursor_before_delivering_its_checkpoint() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(0))]);
    let mut scan_limit_at_start = with_position(
        checkpoint_list_frame(None, 0, Some(proto::QueryEndReason::ScanLimit)),
        checkpoint_position(0),
    );
    // Nothing is covered yet at the start of checkpoint 0.
    scan_limit_at_start.watermark.as_mut().unwrap().checkpoint = None;
    server.push_checkpoint_lists([
        StreamScript::frames([Ok(scan_limit_at_start)]),
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(0), 0, None)),
            Ok(checkpoint_list_frame(
                None,
                0,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let stream = client.stream_checkpoints_with_config(
        CheckpointStreamRequest::new()
            .with_read_mask(checkpoint_identity_mask())
            .with_start(CheckpointStreamStart::Checkpoint(0))
            .with_delivery(Delivery::Poll),
        fast_config(),
    );
    futures::pin_mut!(stream);
    let first = tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .expect("timed out waiting for checkpoint 0")
        .unwrap()
        .unwrap();
    assert_eq!(first.checkpoint.unwrap().sequence_number, Some(0));
    assert_eq!(first.cursor, 0);
    assert_eq!(
        server.state.lock().unwrap().calls,
        ["get_service_info", "list_checkpoints", "list_checkpoints"]
    );
}

/// A Subscribe stream resumed from a position `U` past one replica's tip yields nothing at or
/// before `U`.
///
/// `U` is `(11, 3)`, after checkpoint 11's transactions 0..=2, which the caller has already
/// processed; its cursor is 500. The stream waits until the service reports checkpoint 11, then
/// lists `[U, end of checkpoint 11]`. The first List reaches a lagging replica whose index ends at
/// checkpoint 10: it echoes the request's start position with no checkpoint coverage and ends at
/// `LedgerTip`. The stream retries from that position until a replica serves checkpoint 11 through
/// the requested bound. The subscription then starts at the start of checkpoint 11 (cursor 400)
/// and repeats all of checkpoint 11 before reaching checkpoint 12.
#[tokio::test]
async fn subscribe_resumed_past_the_tip_yields_nothing_before_its_position() {
    let resume = 500;
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10)), Ok(service_info(11))]);
    let mut past_tip_echo = with_position(
        transaction_positioned_list_frame(None, resume, 10, Some(proto::QueryEndReason::LedgerTip)),
        transaction_position(11, 3),
    );
    past_tip_echo.watermark.as_mut().unwrap().checkpoint = None;
    server.push_transaction_lists([
        StreamScript::frames([Ok(past_tip_echo)]),
        StreamScript::frames([
            Ok(transaction_positioned_list_frame(
                Some(transaction_at(11, 3, "after-u-0")),
                501,
                10,
                None,
            )),
            Ok(transaction_positioned_list_frame(
                Some(transaction_at(11, 4, "after-u-1")),
                502,
                10,
                None,
            )),
            Ok(transaction_positioned_list_frame(
                None,
                600,
                11,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    let mut subscription = vec![Ok(transaction_positioned_live_frame(None, 400, 10))];
    for (cursor, index, digest) in [
        (401, 0, "before-u-0"),
        (402, 1, "before-u-1"),
        (403, 2, "before-u-2"),
        (501, 3, "after-u-0"),
        (502, 4, "after-u-1"),
    ] {
        subscription.push(Ok(transaction_positioned_live_frame(
            Some(transaction_at(11, index, digest)),
            cursor,
            10,
        )));
    }
    subscription.push(Ok(transaction_positioned_live_frame(None, 600, 11)));
    subscription.push(Ok(transaction_positioned_live_frame(
        Some(transaction_at(12, 0, "live-12-0")),
        701,
        11,
    )));
    server.push_transaction_subscriptions([StreamScript::frames(subscription)]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let mut config = fast_config();
    config.ledger_tip_poll_interval = Duration::from_millis(1);
    let request = TransactionStreamRequest::new()
        .with_read_mask(prost_types::FieldMask {
            paths: vec!["digest".to_owned()],
        })
        .with_start(TransactionStreamStart::Resume {
            cursor: bytes(resume),
            position: transaction_position(11, 3),
        })
        .with_delivery(Delivery::Subscribe);
    let frames = tokio::time::timeout(
        Duration::from_secs(5),
        client
            .stream_transactions_with_config(request, config)
            .take(4)
            .map_ok(|frame| {
                (
                    frame.transaction.and_then(|transaction| transaction.digest),
                    frame.cursor,
                    frame.position,
                    frame.covered_checkpoint,
                )
            })
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("stream stalled")
    .unwrap();
    let digest = |value: &str| Some(value.to_owned());
    assert_eq!(
        frames,
        [
            (
                digest("after-u-0"),
                bytes(501),
                transaction_position(11, 4),
                Some(10)
            ),
            (
                digest("after-u-1"),
                bytes(502),
                transaction_position(11, 5),
                Some(10)
            ),
            (None, bytes(600), transaction_position(12, 0), Some(11)),
            (
                digest("live-12-0"),
                bytes(701),
                transaction_position(12, 1),
                Some(11)
            ),
        ]
    );

    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        [
            "get_service_info",
            "get_service_info",
            "list_transactions",
            "list_transactions",
            "subscribe_transactions"
        ]
    );
    // Both Lists start at `U`, the retry from the echoed position, and end at checkpoint 11.
    assert_eq!(state.transaction_requests.len(), 2);
    for request in &state.transaction_requests {
        assert_eq!(request.body.start_checkpoint, Some(11));
        assert_eq!(request.body.start_transaction_index, Some(3));
        assert_eq!(request.body.end_checkpoint, Some(12));
        assert_eq!(request.body.end_transaction_index, None);
        let options = request.body.options.as_ref().unwrap();
        assert_eq!(options.after, Some(bytes(resume)));
        assert_eq!(options.before, None);
    }
}

/// An event stream resumed from a position whose checkpoint the service has not reached waits for
/// it, then lists from the position's start fields with the resume cursor.
#[tokio::test]
async fn event_resume_waits_for_its_checkpoint_and_lists_from_its_position() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([
        Ok(service_info(6)),
        Ok(service_info(6)),
        Ok(service_info(7)),
    ]);
    server.push_event_lists([StreamScript::frames([
        Ok(event_positioned_list_frame(
            Some(event_at(7, 2, 1, "after-resume")),
            71,
            6,
            None,
        )),
        Ok(event_positioned_list_frame(
            None,
            72,
            7,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let mut config = fast_config();
    config.ledger_tip_poll_interval = Duration::from_millis(1);
    let frames = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_events_with_config(
                EventStreamRequest::new()
                    .with_start(EventStreamStart::Resume {
                        cursor: bytes(70),
                        position: event_position(7, 2, 1),
                    })
                    .with_delivery(Delivery::Poll),
                config,
            )
            .take(2)
            .map_ok(|frame| {
                (
                    frame.event.and_then(|event| event.event_type),
                    frame.cursor,
                    frame.position,
                )
            })
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for resumed events")
    .unwrap();
    assert_eq!(
        frames,
        [
            (
                Some("after-resume".to_owned()),
                bytes(71),
                event_position(7, 2, 2)
            ),
            (None, bytes(72), event_position(8, 0, 0)),
        ]
    );

    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        [
            "get_service_info",
            "get_service_info",
            "get_service_info",
            "list_events"
        ]
    );
    let request = &state.event_requests[0].body;
    assert_eq!(request.start_checkpoint, Some(7));
    assert_eq!(request.start_transaction_index, Some(2));
    assert_eq!(request.start_event_index, Some(1));
    assert_eq!(request.end_checkpoint, Some(8));
    assert_eq!(request.end_transaction_index, None);
    assert_eq!(request.end_event_index, None);
    let options = request.options.as_ref().unwrap();
    assert_eq!(options.after, Some(bytes(70)));
    assert_eq!(options.before, None);
}
