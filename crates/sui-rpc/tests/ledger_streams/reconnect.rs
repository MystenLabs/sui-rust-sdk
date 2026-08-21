use super::support::STREAM_DROP_TIMEOUT;
use super::support::ScriptedStreamServer;
use super::support::StreamScript;
use super::support::bounded_event_request;
use super::support::bytes;
use super::support::checkpoint_identity_mask;
use super::support::checkpoint_list_frame;
use super::support::checkpoint_live_frame;
use super::support::event;
use super::support::event_at;
use super::support::event_identity_mask;
use super::support::event_list_frame;
use super::support::event_live_frame;
use super::support::event_positioned_list_frame;
use super::support::event_positioned_live_frame;
use super::support::fast_config;
use super::support::fast_list_config;
use super::support::first_event_error;
use super::support::first_list_event_error;
use super::support::next_scripted_call;
use super::support::observed_client;
use super::support::recording_config;
use super::support::service_info;
use super::support::spawn_server;
use super::support::transaction_at;
use super::support::transaction_identity_mask;
use super::support::transaction_list_frame;
use super::support::transaction_live_frame;
use super::support::transaction_positioned_list_frame;
use super::support::transaction_positioned_live_frame;
use futures::StreamExt;
use futures::TryStreamExt;
use std::num::NonZeroUsize;
use std::time::Duration;
use sui_rpc::client::CheckpointStreamRequest;
use sui_rpc::client::CheckpointStreamStart;
use sui_rpc::client::EventStreamRequest;
use sui_rpc::client::EventStreamStart;
use sui_rpc::client::LedgerStreamConfig;
use sui_rpc::client::LedgerStreamEvent;
use sui_rpc::client::LedgerStreamFamily;
use sui_rpc::client::LedgerStreamOperation;
use sui_rpc::client::LedgerStreamStage;
use sui_rpc::client::ListConfig;
use sui_rpc::client::TransactionStreamRequest;
use sui_rpc::client::TransactionStreamStart;
use sui_rpc::proto::sui::rpc::v2 as proto;
use tokio::sync::mpsc;

#[tokio::test]
async fn lagging_new_subscriptions_catch_up_without_reconnect_churn_for_all_families() {
    let (checkpoint_server, _calls) = ScriptedStreamServer::new();
    checkpoint_server.push_service_infos([Ok(service_info(10))]);
    checkpoint_server.push_checkpoint_subscriptions([StreamScript::frames([
        Ok(checkpoint_live_frame(None, 8)),
        Ok(checkpoint_live_frame(None, 9)),
        Ok(checkpoint_live_frame(None, 10)),
        Ok(checkpoint_live_frame(Some(11), 11)),
    ])]);
    checkpoint_server.push_checkpoint_lists([StreamScript::frames([Ok(checkpoint_list_frame(
        None,
        10,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    let address = spawn_server(checkpoint_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let checkpoint_frames = client
        .stream_checkpoints_with_config(
            CheckpointStreamRequest::new()
                .with_read_mask(checkpoint_identity_mask())
                .with_filter(proto::TransactionFilter::default())
                .with_start(CheckpointStreamStart::Checkpoint(0)),
            fast_config(),
        )
        .take(2)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert!(checkpoint_frames[0].checkpoint.is_none());
    assert_eq!(checkpoint_frames[0].cursor, 10);
    assert_eq!(
        checkpoint_frames[1]
            .checkpoint
            .as_ref()
            .unwrap()
            .sequence_number,
        Some(11)
    );
    {
        let state = checkpoint_server.state.lock().unwrap();
        assert_eq!(state.checkpoint_subscriptions.len(), 1);
        assert_eq!(state.checkpoint_requests.len(), 1);
    }

    let (transaction_server, _calls) = ScriptedStreamServer::new();
    transaction_server.push_service_infos([Ok(service_info(10))]);
    transaction_server.push_transaction_subscriptions([StreamScript::frames([
        Ok(transaction_live_frame(None, 8)),
        Ok(transaction_live_frame(None, 9)),
        Ok(transaction_live_frame(None, 10)),
        Ok(transaction_live_frame(Some(11), 11)),
    ])]);
    transaction_server.push_transaction_lists([StreamScript::frames([Ok(
        transaction_list_frame(None, 10, Some(proto::QueryEndReason::CheckpointBound)),
    )])]);
    let address = spawn_server(transaction_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let transaction_frames = client
        .stream_transactions_with_config(
            TransactionStreamRequest::new()
                .with_read_mask(transaction_identity_mask())
                .with_start(TransactionStreamStart::Checkpoint(0)),
            fast_config(),
        )
        .take(2)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert!(transaction_frames[0].transaction.is_none());
    assert_eq!(transaction_frames[0].cursor, bytes(10));
    assert_eq!(
        transaction_frames[1]
            .transaction
            .as_ref()
            .unwrap()
            .digest
            .as_deref(),
        Some("tx-11")
    );
    {
        let state = transaction_server.state.lock().unwrap();
        assert_eq!(state.transaction_subscriptions.len(), 1);
        assert_eq!(state.transaction_requests.len(), 1);
    }

    let (event_server, _calls) = ScriptedStreamServer::new();
    event_server.push_service_infos([Ok(service_info(10))]);
    event_server.push_event_subscriptions([StreamScript::frames([
        Ok(event_live_frame(None, 8)),
        Ok(event_live_frame(None, 9)),
        Ok(event_live_frame(None, 10)),
        Ok(event_live_frame(Some(11), 11)),
    ])]);
    event_server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        10,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    let address = spawn_server(event_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let event_frames = client
        .stream_events_with_config(
            EventStreamRequest::new()
                .with_read_mask(event_identity_mask())
                .with_start(EventStreamStart::Checkpoint(0)),
            fast_config(),
        )
        .take(2)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert!(event_frames[0].event.is_none());
    assert_eq!(event_frames[0].cursor, bytes(10));
    assert_eq!(
        event_frames[1]
            .event
            .as_ref()
            .unwrap()
            .event_type
            .as_deref(),
        Some("event-11")
    );
    let state = event_server.state.lock().unwrap();
    assert_eq!(state.event_subscriptions.len(), 1);
    assert_eq!(state.event_requests.len(), 1);
}

#[tokio::test]
async fn lagging_opaque_subscription_replays_unknown_cursor_order_before_live_data() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10))]);
    let mut unknown_cursor_order_progress = event_live_frame(None, 9);
    unknown_cursor_order_progress
        .watermark
        .as_mut()
        .unwrap()
        .checkpoint = Some(10);
    server.push_event_subscriptions([StreamScript::frames([
        Ok(event_live_frame(None, 8)),
        Ok(unknown_cursor_order_progress),
        Ok(event_live_frame(Some(12), 12)),
    ])]);
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::frames([Ok(event_positioned_list_frame(
            None,
            9,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let frames = client
        .stream_events_with_config(
            EventStreamRequest::new()
                .with_read_mask(event_identity_mask())
                .with_start(EventStreamStart::Checkpoint(0)),
            fast_config(),
        )
        .take(3)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert!(frames[0].event.is_none());
    assert_eq!(frames[0].cursor, bytes(10));
    assert_eq!(frames[0].covered_checkpoint, Some(10));
    assert!(frames[1].event.is_none());
    assert_eq!(frames[1].cursor, bytes(9));
    assert_eq!(frames[1].covered_checkpoint, Some(10));
    assert_eq!(
        frames[2].event.as_ref().unwrap().event_type.as_deref(),
        Some("event-12")
    );
    let state = server.state.lock().unwrap();
    assert_eq!(state.event_subscriptions.len(), 1);
    assert_eq!(state.event_requests.len(), 2);
    assert_eq!(
        state.event_requests[1].body.options.as_ref().unwrap().after,
        Some(bytes(10))
    );
    assert_eq!(
        state.event_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        None,
    );
    assert_eq!(state.event_requests[1].body.end_checkpoint, Some(11));
}

#[tokio::test]
async fn same_checkpoint_transaction_reconnect_replays_immediately_and_deduplicates_positions() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10))]);
    let (subscription_tx, subscription_rx) = mpsc::unbounded_channel();
    let (recovery_tx, recovery_rx) = mpsc::unbounded_channel();
    server.push_transaction_subscriptions([StreamScript::Channel(subscription_rx)]);
    server.push_transaction_lists([
        StreamScript::frames([Ok(transaction_positioned_list_frame(
            None,
            100,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::Channel(recovery_rx),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let body = TransactionStreamRequest::new()
        .with_read_mask(prost_types::FieldMask {
            paths: vec![
                "digest".to_owned(),
                "checkpoint".to_owned(),
                "transaction_index".to_owned(),
            ],
        })
        .with_start(TransactionStreamStart::Resume(bytes(50)));
    let collector = tokio::spawn(async move {
        let mut config = fast_config();
        config.list_page_limit = Some(7);
        client
            .stream_transactions_with_config(body, config)
            .take(5)
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "list_transactions");
    assert_eq!(
        next_scripted_call(&mut calls).await,
        "subscribe_transactions"
    );
    subscription_tx
        .send(Ok(transaction_positioned_live_frame(
            Some(transaction_at(10, 1, "retained-and-listed")),
            900,
            10,
        )))
        .unwrap();
    assert_eq!(
        next_scripted_call(&mut calls).await,
        "list_transactions",
        "same-checkpoint recovery must not wait for another subscription frame"
    );
    subscription_tx
        .send(Ok(transaction_positioned_live_frame(
            Some(transaction_at(10, 2, "retained-only")),
            901,
            10,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(transaction_positioned_list_frame(
            Some(transaction_at(10, 0, "list-only")),
            800,
            10,
            None,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(transaction_positioned_list_frame(
            Some(transaction_at(10, 1, "retained-and-listed")),
            900,
            10,
            None,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(transaction_positioned_list_frame(
            None,
            900,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        )))
        .unwrap();
    subscription_tx
        .send(Ok(transaction_positioned_live_frame(
            Some(transaction_at(11, 0, "live-after-drain")),
            1_000,
            11,
        )))
        .unwrap();

    let frames = tokio::time::timeout(Duration::from_secs(2), collector)
        .await
        .expect("same-checkpoint transaction recovery stalled")
        .unwrap()
        .unwrap();
    let transactions = frames
        .iter()
        .filter_map(|frame| frame.transaction.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(
        transactions
            .iter()
            .map(|transaction| transaction.digest.as_deref().unwrap())
            .collect::<Vec<_>>(),
        [
            "list-only",
            "retained-and-listed",
            "retained-only",
            "live-after-drain",
        ]
    );
    assert!(
        transactions
            .iter()
            .all(|transaction| transaction.checkpoint.is_some()
                && transaction.transaction_index.is_some())
    );
    assert_eq!(frames.last().unwrap().cursor, bytes(1_000));

    let state = server.state.lock().unwrap();
    assert_eq!(state.transaction_subscriptions.len(), 1);
    assert_eq!(state.transaction_requests.len(), 2);
    assert_eq!(
        state.transaction_requests[0]
            .body
            .options
            .as_ref()
            .unwrap()
            .after,
        Some(bytes(50))
    );
    assert_eq!(
        state.transaction_requests[0]
            .body
            .options
            .as_ref()
            .unwrap()
            .limit,
        Some(7)
    );
    let recovery_request = &state.transaction_requests[1].body;
    assert_eq!(recovery_request.end_checkpoint, Some(11));
    assert_eq!(
        recovery_request.options.as_ref().unwrap().after,
        Some(bytes(100))
    );
    assert_eq!(recovery_request.options.as_ref().unwrap().before, None);
    assert_eq!(recovery_request.options.as_ref().unwrap().limit, Some(7));
    assert!(
        recovery_request
            .options
            .as_ref()
            .unwrap()
            .ordering
            .is_none()
    );
    assert_eq!(
        recovery_request.read_mask.as_ref().unwrap().paths,
        ["digest", "checkpoint", "transaction_index"]
    );
    assert_eq!(
        state.transaction_subscriptions[0]
            .body
            .read_mask
            .as_ref()
            .unwrap()
            .paths,
        ["digest", "checkpoint", "transaction_index"]
    );
}

#[tokio::test]
async fn same_checkpoint_progress_only_reconnect_replays_immediately() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10))]);
    let (subscription_tx, subscription_rx) = mpsc::unbounded_channel();
    let (recovery_tx, recovery_rx) = mpsc::unbounded_channel();
    server.push_transaction_subscriptions([StreamScript::Channel(subscription_rx)]);
    server.push_transaction_lists([
        StreamScript::frames([Ok(transaction_positioned_list_frame(
            None,
            100,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::Channel(recovery_rx),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let collector = tokio::spawn(async move {
        client
            .stream_transactions_with_config(
                TransactionStreamRequest::new()
                    .with_read_mask(transaction_identity_mask())
                    .with_start(TransactionStreamStart::Checkpoint(0)),
                fast_config(),
            )
            .take(4)
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "list_transactions");
    assert_eq!(
        next_scripted_call(&mut calls).await,
        "subscribe_transactions"
    );
    subscription_tx
        .send(Ok(transaction_positioned_live_frame(None, 900, 10)))
        .unwrap();
    assert_eq!(
        next_scripted_call(&mut calls).await,
        "list_transactions",
        "same-checkpoint recovery must not wait for an item-bearing subscription frame"
    );

    recovery_tx
        .send(Ok(transaction_positioned_list_frame(
            Some(transaction_at(10, 0, "replayed")),
            800,
            10,
            None,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(transaction_positioned_list_frame(
            None,
            900,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        )))
        .unwrap();
    subscription_tx
        .send(Ok(transaction_positioned_live_frame(
            Some(transaction_at(11, 0, "live")),
            1_000,
            11,
        )))
        .unwrap();

    let frames = tokio::time::timeout(Duration::from_secs(2), collector)
        .await
        .expect("progress-only same-checkpoint recovery stalled")
        .unwrap()
        .unwrap();
    assert_eq!(
        frames
            .iter()
            .filter_map(|frame| frame.transaction.as_ref())
            .map(|transaction| transaction.digest.as_deref().unwrap())
            .collect::<Vec<_>>(),
        ["replayed", "live"]
    );
    assert_eq!(frames.last().unwrap().cursor, bytes(1_000));
}

#[tokio::test]
async fn same_checkpoint_event_reconnect_replays_immediately_and_deduplicates_positions() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10))]);
    let (subscription_tx, subscription_rx) = mpsc::unbounded_channel();
    let (recovery_tx, recovery_rx) = mpsc::unbounded_channel();
    server.push_event_subscriptions([StreamScript::Channel(subscription_rx)]);
    server.push_event_lists([
        StreamScript::frames([Ok(event_positioned_list_frame(
            None,
            100,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::Channel(recovery_rx),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let body = EventStreamRequest::new()
        .with_read_mask(prost_types::FieldMask {
            paths: vec![
                "event_type".to_owned(),
                "checkpoint".to_owned(),
                "transaction_index".to_owned(),
                "event_index".to_owned(),
            ],
        })
        .with_start(EventStreamStart::Checkpoint(0));
    let collector = tokio::spawn(async move {
        client
            .stream_events_with_config(body, fast_config())
            .take(5)
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    assert_eq!(next_scripted_call(&mut calls).await, "subscribe_events");
    subscription_tx
        .send(Ok(event_positioned_live_frame(
            Some(event_at(10, 1, 0, "retained-and-listed")),
            900,
            10,
        )))
        .unwrap();
    assert_eq!(
        next_scripted_call(&mut calls).await,
        "list_events",
        "same-checkpoint recovery must not wait for another subscription frame"
    );
    subscription_tx
        .send(Ok(event_positioned_live_frame(
            Some(event_at(10, 1, 1, "retained-only")),
            901,
            10,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(event_positioned_list_frame(
            Some(event_at(10, 0, 0, "list-only")),
            800,
            10,
            None,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(event_positioned_list_frame(
            Some(event_at(10, 1, 0, "retained-and-listed")),
            900,
            10,
            None,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(event_positioned_list_frame(
            None,
            900,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        )))
        .unwrap();
    subscription_tx
        .send(Ok(event_positioned_live_frame(
            Some(event_at(11, 0, 0, "live-after-drain")),
            1_000,
            11,
        )))
        .unwrap();

    let frames = tokio::time::timeout(Duration::from_secs(2), collector)
        .await
        .expect("same-checkpoint event recovery stalled")
        .unwrap()
        .unwrap();
    let events = frames
        .iter()
        .filter_map(|frame| frame.event.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(
        events
            .iter()
            .map(|event| event.event_type.as_deref().unwrap())
            .collect::<Vec<_>>(),
        [
            "list-only",
            "retained-and-listed",
            "retained-only",
            "live-after-drain",
        ]
    );
    assert!(events.iter().all(|event| {
        event.checkpoint.is_some()
            && event.transaction_index.is_some()
            && event.event_index.is_some()
    }));
    assert_eq!(frames.last().unwrap().cursor, bytes(1_000));

    let state = server.state.lock().unwrap();
    assert_eq!(state.event_subscriptions.len(), 1);
    assert_eq!(state.event_requests.len(), 2);
    let recovery_request = &state.event_requests[1].body;
    assert_eq!(recovery_request.end_checkpoint, Some(11));
    assert_eq!(
        recovery_request.options.as_ref().unwrap().after,
        Some(bytes(100))
    );
    assert_eq!(recovery_request.options.as_ref().unwrap().before, None);
    assert_eq!(
        recovery_request.read_mask.as_ref().unwrap().paths,
        [
            "event_type",
            "checkpoint",
            "transaction_index",
            "event_index"
        ]
    );
    assert_eq!(
        state.event_subscriptions[0]
            .body
            .read_mask
            .as_ref()
            .unwrap()
            .paths,
        [
            "event_type",
            "checkpoint",
            "transaction_index",
            "event_index"
        ]
    );
}

#[tokio::test]
async fn filtered_opaque_bootstrap_and_subscription_gap_release_boundary_once() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(5)), Ok(service_info(5))]);
    server.push_transaction_subscriptions([StreamScript::frames([Ok(transaction_live_frame(
        Some(7),
        7,
    ))])]);
    server.push_transaction_lists([
        StreamScript::frames([Ok(transaction_list_frame(
            None,
            5,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::frames([
            Ok(transaction_list_frame(Some(6), 6, None)),
            Ok(transaction_list_frame(
                None,
                7,
                Some(proto::QueryEndReason::CursorBound),
            )),
        ]),
    ]);
    server.push_event_subscriptions([StreamScript::frames([Ok(event_live_frame(Some(7), 7))])]);
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            5,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::frames([
            Ok(event_list_frame(Some(6), 6, None)),
            Ok(event_list_frame(
                None,
                7,
                Some(proto::QueryEndReason::CursorBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let transaction_request = TransactionStreamRequest::new()
        .with_read_mask(prost_types::FieldMask {
            paths: vec![
                "digest".to_owned(),
                "checkpoint".to_owned(),
                "transaction_index".to_owned(),
            ],
        })
        .with_filter(proto::TransactionFilter::default())
        .with_start(TransactionStreamStart::Checkpoint(0));
    let transactions = client
        .stream_transactions_with_config(transaction_request, fast_config())
        .try_filter_map(|frame| async move {
            Ok(frame
                .transaction
                .map(|transaction| transaction.digest.unwrap()))
        })
        .take(2)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(transactions, ["tx-6", "tx-7"]);

    let event_request = EventStreamRequest::new()
        .with_read_mask(prost_types::FieldMask {
            paths: vec![
                "event_type".to_owned(),
                "checkpoint".to_owned(),
                "transaction_index".to_owned(),
                "event_index".to_owned(),
            ],
        })
        .with_filter(proto::EventFilter::default())
        .with_start(EventStreamStart::Checkpoint(0));
    let events = client
        .stream_events_with_config(event_request, fast_config())
        .try_filter_map(
            |frame| async move { Ok(frame.event.map(|event| event.event_type.unwrap())) },
        )
        .take(2)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(events, ["event-6", "event-7"]);

    let state = server.state.lock().unwrap();
    assert_eq!(state.transaction_requests[0].body.end_checkpoint, Some(6));
    assert_eq!(
        state.transaction_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .after,
        Some(bytes(5))
    );
    assert_eq!(
        state.transaction_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(7))
    );
    assert_eq!(state.event_requests[0].body.end_checkpoint, Some(6));
    assert_eq!(
        state.event_requests[1].body.options.as_ref().unwrap().after,
        Some(bytes(5))
    );
    assert_eq!(
        state.event_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(7))
    );
    assert_eq!(state.transaction_subscriptions.len(), 1);
    assert_eq!(state.event_subscriptions.len(), 1);
}

#[tokio::test]
async fn dropped_event_stream_replays_index_lag_before_buffered_live_data() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10)), Ok(service_info(12))]);
    server.push_event_subscriptions([
        StreamScript::frames([
            Ok(event_live_frame(None, 10)),
            Ok(event_live_frame(Some(11), 11)),
            Ok(event_live_frame(None, 12)),
            Err(tonic::Status::unavailable("dropped live stream")),
        ]),
        StreamScript::frames([
            Ok(event_live_frame(None, 16)),
            Ok(event_live_frame(Some(17), 17)),
        ]),
    ]);
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            12,
            Some(proto::QueryEndReason::LedgerTip),
        ))]),
        StreamScript::frames([
            Ok(event_list_frame(Some(14), 14, None)),
            Ok(event_list_frame(Some(15), 15, None)),
            Ok(event_list_frame(
                None,
                16,
                Some(proto::QueryEndReason::CursorBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let events = client
        .stream_events_with_config(
            EventStreamRequest::new()
                .with_read_mask(event_identity_mask())
                .with_start(EventStreamStart::Checkpoint(0)),
            fast_config(),
        )
        .try_filter_map(
            |frame| async move { Ok(frame.event.map(|event| event.event_type.unwrap())) },
        )
        .take(4)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(events, ["event-11", "event-14", "event-15", "event-17"]);

    let state = server.state.lock().unwrap();
    assert_eq!(
        state.event_requests[1].body.options.as_ref().unwrap().after,
        Some(bytes(12))
    );
    assert_eq!(
        state.event_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(16))
    );
    assert_eq!(
        state.event_requests[2].body.options.as_ref().unwrap().after,
        Some(bytes(12))
    );
    assert_eq!(
        state.event_requests[2]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(16))
    );
}

#[tokio::test]
async fn replay_defers_and_coalesces_progress_until_the_fixed_gap_completes() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(0))]);
    let (live_tx, live_rx) = mpsc::unbounded_channel();
    let (gap_tx, gap_rx) = mpsc::unbounded_channel();
    server.push_event_subscriptions([
        StreamScript::Channel(live_rx),
        StreamScript::frames([Ok(event_live_frame(Some(7), 7))]),
    ]);
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::Channel(gap_rx),
        StreamScript::frames([
            Ok(event_list_frame(Some(6), 6, None)),
            Ok(event_list_frame(
                None,
                7,
                Some(proto::QueryEndReason::CursorBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let mut config = fast_config();
    config.max_buffered_live_items = NonZeroUsize::new(2).unwrap();
    let (config, mut observer_events) = recording_config(config);
    let collector = tokio::spawn(async move {
        client
            .stream_events_with_config(
                EventStreamRequest::new()
                    .with_read_mask(event_identity_mask())
                    .with_start(EventStreamStart::Checkpoint(0)),
                config,
            )
            .take(7)
            .map_ok(|frame| {
                (
                    frame.event.map(|event| event.event_type.unwrap()),
                    frame.cursor,
                )
            })
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(calls.recv().await, Some("get_service_info"));
    assert_eq!(calls.recv().await, Some("list_events"));
    assert_eq!(calls.recv().await, Some("subscribe_events"));
    live_tx.send(Ok(event_live_frame(Some(2), 2))).unwrap();
    assert_eq!(calls.recv().await, Some("list_events"));
    live_tx.send(Ok(event_live_frame(None, 3))).unwrap();
    live_tx.send(Ok(event_live_frame(None, 4))).unwrap();
    live_tx.send(Ok(event_live_frame(Some(5), 5))).unwrap();
    tokio::time::timeout(STREAM_DROP_TIMEOUT, live_tx.closed())
        .await
        .expect("item cap did not drop the pinned subscription");
    let mut semantic_events = Vec::new();
    while let Ok(event) = observer_events.try_recv() {
        if !matches!(&event, LedgerStreamEvent::RpcResponse { .. }) {
            semantic_events.push(event);
        }
    }
    assert_eq!(semantic_events.len(), 2);
    assert!(matches!(
        &semantic_events[0],
        LedgerStreamEvent::GapRecoveryStarted {
            family: LedgerStreamFamily::Event,
            ..
        }
    ));
    assert!(matches!(
        &semantic_events[1],
        LedgerStreamEvent::SubscriptionBufferLimitReached {
            family: LedgerStreamFamily::Event,
            buffered_items: 2,
            limit: 2,
            ..
        }
    ));
    gap_tx.send(Ok(event_list_frame(Some(1), 1, None))).unwrap();
    gap_tx
        .send(Ok(event_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::CursorBound),
        )))
        .unwrap();

    let events = collector.await.unwrap().unwrap();
    assert_eq!(
        events,
        [
            (None, bytes(0)),
            (Some("event-1".to_owned()), bytes(1)),
            (Some("event-2".to_owned()), bytes(2)),
            (None, bytes(4)),
            (Some("event-5".to_owned()), bytes(5)),
            (Some("event-6".to_owned()), bytes(6)),
            (Some("event-7".to_owned()), bytes(7)),
        ]
    );
    let state = server.state.lock().unwrap();
    assert_eq!(
        state.event_requests[2].body.options.as_ref().unwrap().after,
        Some(bytes(5))
    );
    assert_eq!(
        state.event_requests[2]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(7))
    );
    drop(state);
}

#[tokio::test]
async fn live_failure_during_replay_is_deferred_until_valid_buffer_is_delivered() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(0)), Ok(service_info(2))]);
    let (live_tx, live_rx) = mpsc::unbounded_channel();
    let (gap_tx, gap_rx) = mpsc::unbounded_channel();
    server.push_event_subscriptions([
        StreamScript::Channel(live_rx),
        StreamScript::frames([Ok(event_live_frame(Some(5), 5))]),
    ]);
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::Channel(gap_rx),
        StreamScript::frames([
            Ok(event_list_frame(Some(4), 4, None)),
            Ok(event_list_frame(
                None,
                5,
                Some(proto::QueryEndReason::CursorBound),
            )),
        ]),
    ]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);
    let (config, mut observer_events) = recording_config(fast_config());
    let collector = tokio::spawn(async move {
        client
            .stream_events_with_config(
                EventStreamRequest::new()
                    .with_read_mask(event_identity_mask())
                    .with_start(EventStreamStart::Checkpoint(0)),
                config,
            )
            .try_filter_map(|frame| async move {
                Ok(frame.event.map(|event| event.event_type.unwrap()))
            })
            .take(5)
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(calls.recv().await, Some("get_service_info"));
    assert_eq!(calls.recv().await, Some("list_events"));
    assert_eq!(calls.recv().await, Some("subscribe_events"));
    live_tx.send(Ok(event_live_frame(Some(2), 2))).unwrap();
    assert_eq!(calls.recv().await, Some("list_events"));

    match observer_events.recv().await.unwrap() {
        LedgerStreamEvent::RpcResponse {
            family: LedgerStreamFamily::Event,
            operation: LedgerStreamOperation::GetServiceInfo,
            stage: LedgerStreamStage::InitialReplay,
            code: tonic::Code::Ok,
            ..
        } => {}
        event => panic!("unexpected observer event: {event:?}"),
    }
    match observer_events.recv().await.unwrap() {
        LedgerStreamEvent::RpcResponse {
            family: LedgerStreamFamily::Event,
            operation: LedgerStreamOperation::List,
            stage: LedgerStreamStage::InitialReplay,
            code: tonic::Code::Ok,
            ..
        } => {}
        event => panic!("unexpected observer event: {event:?}"),
    }
    match observer_events.recv().await.unwrap() {
        LedgerStreamEvent::RpcResponse {
            family: LedgerStreamFamily::Event,
            operation: LedgerStreamOperation::Subscribe,
            stage: LedgerStreamStage::LiveSubscription,
            code: tonic::Code::Ok,
            ..
        } => {}
        event => panic!("unexpected observer event: {event:?}"),
    }
    match observer_events.recv().await.unwrap() {
        LedgerStreamEvent::GapRecoveryStarted {
            family: LedgerStreamFamily::Event,
            ..
        } => {}
        event => panic!("unexpected observer event: {event:?}"),
    }
    match observer_events.recv().await.unwrap() {
        LedgerStreamEvent::RpcResponse {
            family: LedgerStreamFamily::Event,
            operation: LedgerStreamOperation::List,
            stage: LedgerStreamStage::GapRecovery,
            code: tonic::Code::Ok,
            ..
        } => {}
        event => panic!("unexpected observer event: {event:?}"),
    }
    assert!(observer_events.try_recv().is_err());

    live_tx.send(Ok(event_live_frame(Some(3), 3))).unwrap();
    live_tx
        .send(Err(tonic::Status::unavailable("pinned stream failed")))
        .unwrap();
    match observer_events.recv().await.unwrap() {
        LedgerStreamEvent::SubscriptionStreamInterrupted {
            family: LedgerStreamFamily::Event,
            stage: LedgerStreamStage::GapRecovery,
            status,
            ..
        } => {
            assert_eq!(status.code(), tonic::Code::Unavailable);
            assert_eq!(status.message(), "pinned stream failed");
        }
        event => panic!("unexpected observer event: {event:?}"),
    }
    assert!(observer_events.try_recv().is_err());
    tokio::time::timeout(STREAM_DROP_TIMEOUT, live_tx.closed())
        .await
        .expect("failed pinned stream was not dropped");
    gap_tx.send(Ok(event_list_frame(Some(1), 1, None))).unwrap();
    gap_tx
        .send(Ok(event_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::CursorBound),
        )))
        .unwrap();

    assert_eq!(
        collector.await.unwrap().unwrap(),
        ["event-1", "event-2", "event-3", "event-4", "event-5"]
    );
    match observer_events.recv().await.unwrap() {
        LedgerStreamEvent::RetryScheduled {
            family: LedgerStreamFamily::Event,
            operation: LedgerStreamOperation::Subscribe,
            stage: LedgerStreamStage::GapRecovery,
            status,
            consecutive_failures: 1,
            delay: Duration::ZERO,
            ..
        } => {
            assert_eq!(status.code(), tonic::Code::Unavailable);
            assert_eq!(status.message(), "pinned stream failed");
        }
        event => panic!("unexpected observer event: {event:?}"),
    }
}

#[tokio::test]
async fn transient_retries_and_in_body_resume_preserve_the_safe_watermark() {
    let (unlimited_server, _calls) = ScriptedStreamServer::new();
    unlimited_server.push_event_lists(
        (0..6)
            .map(|attempt| {
                StreamScript::DispatchError(tonic::Status::unavailable(format!(
                    "unlimited-{attempt}"
                )))
            })
            .chain([StreamScript::frames([
                Ok(event_list_frame(Some(9), 9, None)),
                Ok(event_positioned_list_frame(
                    None,
                    10,
                    9,
                    Some(proto::QueryEndReason::CheckpointBound),
                )),
            ])]),
    );
    let address = spawn_server(unlimited_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let events = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .list_events_with_config(bounded_event_request(), fast_list_config())
            .try_filter_map(|frame| async move {
                Ok(frame.event.map(|event| event.event_type.unwrap()))
            })
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for repeated transient List retries")
    .unwrap();
    assert_eq!(events, ["event-9"]);
    assert_eq!(
        unlimited_server.state.lock().unwrap().event_requests.len(),
        7
    );

    let (resume_server, _calls) = ScriptedStreamServer::new();
    resume_server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(2), 2, None)),
            Ok(event_list_frame(None, 4, None)),
            Err(tonic::Status::unavailable("body failed")),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Ok(event_positioned_list_frame(
                None,
                10,
                9,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    let address = spawn_server(resume_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let events = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .list_events_with_config(bounded_event_request(), fast_list_config())
            .try_filter_map(|frame| async move {
                Ok(frame.event.map(|event| event.event_type.unwrap()))
            })
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for in-body List resume")
    .unwrap();
    assert_eq!(events, ["event-2", "event-5"]);
    let state = resume_server.state.lock().unwrap();
    assert_eq!(
        state.event_requests[1].body.options.as_ref().unwrap().after,
        Some(bytes(4))
    );
}

#[tokio::test]
async fn transient_retry_backoff_is_exponential_and_capped() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_event_lists(
        (0..4)
            .map(|attempt| {
                StreamScript::DispatchError(tonic::Status::unavailable(format!(
                    "attempt-{attempt}"
                )))
            })
            .chain([StreamScript::frames([
                Ok(event_list_frame(Some(9), 9, None)),
                Ok(event_positioned_list_frame(
                    None,
                    10,
                    9,
                    Some(proto::QueryEndReason::CheckpointBound),
                )),
            ])]),
    );
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);
    let mut config = ListConfig::default();
    config.base_retry_delay = Duration::from_millis(50);
    config.max_retry_delay = Duration::from_millis(150);
    config.retry_jitter = Duration::ZERO;
    let collector = tokio::spawn(async move {
        client
            .list_events_with_config(bounded_event_request(), config)
            .try_filter_map(|frame| async move {
                Ok(frame.event.map(|event| event.event_type.unwrap()))
            })
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    for expected_delay in [50, 100, 150].map(Duration::from_millis) {
        let started = std::time::Instant::now();
        assert_eq!(next_scripted_call(&mut calls).await, "list_events");
        let elapsed = started.elapsed();
        assert!(
            elapsed >= expected_delay,
            "retry dispatched after {elapsed:?}, before {expected_delay:?}"
        );
    }
    let capped_retry_started = std::time::Instant::now();
    assert_eq!(
        tokio::time::timeout(Duration::from_millis(300), next_scripted_call(&mut calls),)
            .await
            .expect("retry exceeded twice the configured backoff cap"),
        "list_events"
    );
    let capped_retry_elapsed = capped_retry_started.elapsed();
    assert!(
        capped_retry_elapsed >= Duration::from_millis(150),
        "capped retry dispatched after {capped_retry_elapsed:?}, before its configured delay"
    );

    let events = tokio::time::timeout(Duration::from_secs(2), collector)
        .await
        .expect("timed out waiting for capped transient retries")
        .unwrap()
        .unwrap();
    assert_eq!(events, ["event-9"]);
}

#[tokio::test]
async fn list_and_subscription_failures_use_independent_retry_states() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10)), Ok(service_info(11))]);
    let (second_subscription_tx, second_subscription_rx) = mpsc::unbounded_channel();
    let (third_tx, third_rx) = mpsc::unbounded_channel();
    server.push_event_subscriptions([
        StreamScript::Channel(second_subscription_rx),
        StreamScript::Channel(third_rx),
    ]);
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::frames([
            Ok(event_list_frame(Some(11), 11, None)),
            Err(tonic::Status::unavailable("List failed after progress")),
        ]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            11,
            Some(proto::QueryEndReason::CursorBound),
        ))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let mut config = LedgerStreamConfig::default();
    config.base_retry_delay = Duration::from_millis(200);
    config.max_retry_delay = Duration::from_millis(800);
    config.retry_jitter = Duration::ZERO;
    let collector = tokio::spawn(async move {
        client
            .stream_events_with_config(
                EventStreamRequest::new()
                    .with_read_mask(event_identity_mask())
                    .with_start(EventStreamStart::Checkpoint(0)),
                config,
            )
            .take(4)
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    assert_eq!(next_scripted_call(&mut calls).await, "subscribe_events");
    second_subscription_tx
        .send(Ok(event_live_frame(None, 12)))
        .unwrap();
    second_subscription_tx
        .send(Err(tonic::Status::unavailable(
            "subscription failed during replay",
        )))
        .unwrap();
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    let list_retry_started = std::time::Instant::now();
    assert_eq!(
        tokio::time::timeout(Duration::from_millis(350), next_scripted_call(&mut calls),)
            .await
            .expect("List retry exceeded its independent base delay window"),
        "list_events"
    );
    let list_retry_elapsed = list_retry_started.elapsed();
    assert!(
        list_retry_elapsed >= Duration::from_millis(200),
        "List retry dispatched after {list_retry_elapsed:?}, before its base delay"
    );
    let subscription_recovery_started = std::time::Instant::now();
    assert_eq!(
        tokio::time::timeout(Duration::from_millis(350), next_scripted_call(&mut calls),)
            .await
            .expect("Subscribe recovery exceeded its independent base delay window"),
        "get_service_info"
    );
    let subscription_recovery_elapsed = subscription_recovery_started.elapsed();
    assert!(
        subscription_recovery_elapsed >= Duration::from_millis(200),
        "Subscribe recovery dispatched after {subscription_recovery_elapsed:?}, before its base delay"
    );
    assert_eq!(next_scripted_call(&mut calls).await, "subscribe_events");

    collector.abort();
    assert!(collector.await.unwrap_err().is_cancelled());
    tokio::time::timeout(STREAM_DROP_TIMEOUT, third_tx.closed())
        .await
        .expect("third subscription was not dropped");
    let state = server.state.lock().unwrap();
    assert_eq!(state.event_requests.len(), 3);
    assert_eq!(state.event_subscriptions.len(), 2);
}

#[tokio::test]
async fn reconnection_requires_checkpoint_coverage_in_its_initial_frame() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(5)), Ok(service_info(5))]);
    let mut invalid_reconnection_boundary = event_live_frame(None, 6);
    invalid_reconnection_boundary
        .watermark
        .as_mut()
        .unwrap()
        .checkpoint = None;
    server.push_event_subscriptions([
        StreamScript::frames([Ok(event_live_frame(None, 5))]),
        StreamScript::frames([Ok(invalid_reconnection_boundary)]),
    ]);
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
    assert_eq!(state.event_subscriptions.len(), 2);
    assert_eq!(state.event_requests.len(), 1);
}

#[tokio::test]
async fn non_retryable_status_is_forwarded_without_reconnect() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([StreamScript::DispatchError(
        tonic::Status::invalid_argument("bad filter"),
    )]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let status = first_list_event_error(&client, bounded_event_request(), fast_list_config()).await;
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
    assert_eq!(status.message(), "bad filter");
    assert_eq!(server.state.lock().unwrap().event_requests.len(), 1);
}

#[tokio::test]
async fn checkpoint_gap_rejects_out_of_order_buffered_live_frames() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10))]);
    let (live_tx, live_rx) = mpsc::unbounded_channel();
    let (gap_tx, gap_rx) = mpsc::unbounded_channel();
    server.push_checkpoint_subscriptions([StreamScript::Channel(live_rx)]);
    server.push_checkpoint_lists([
        StreamScript::frames([Ok(checkpoint_list_frame(
            None,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::Channel(gap_rx),
    ]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);
    let collector = tokio::spawn(async move {
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
        assert_eq!(progress.cursor, 10);
        let gap_progress = stream.next().await.unwrap().unwrap();
        assert!(gap_progress.checkpoint.is_none());
        assert_eq!(gap_progress.cursor, 15);
        stream.next().await.unwrap().unwrap_err()
    });

    assert_eq!(calls.recv().await, Some("get_service_info"));
    assert_eq!(calls.recv().await, Some("list_checkpoints"));
    assert_eq!(calls.recv().await, Some("subscribe_checkpoints"));
    live_tx.send(Ok(checkpoint_live_frame(None, 15))).unwrap();
    assert_eq!(calls.recv().await, Some("list_checkpoints"));
    live_tx
        .send(Ok(checkpoint_live_frame(Some(14), 14)))
        .unwrap();
    tokio::time::timeout(STREAM_DROP_TIMEOUT, live_tx.closed())
        .await
        .expect("regressing live stream was not dropped");
    gap_tx
        .send(Ok(checkpoint_list_frame(
            None,
            15,
            Some(proto::QueryEndReason::CheckpointBound),
        )))
        .unwrap();

    let status = collector.await.unwrap();
    assert_eq!(status.code(), tonic::Code::DataLoss);
}

#[tokio::test]
async fn dropping_live_facade_drops_the_tonic_stream_without_reconnecting() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(0))]);
    let (live_tx, live_rx) = mpsc::unbounded_channel();
    server.push_event_subscriptions([StreamScript::Channel(live_rx)]);
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        0,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let mut stream = Box::pin(
        client.stream_events_with_config(
            EventStreamRequest::new()
                .with_read_mask(event_identity_mask())
                .with_start(EventStreamStart::Checkpoint(0)),
            fast_config(),
        ),
    );
    let orchestration = tokio::spawn(async move {
        assert_eq!(calls.recv().await, Some("get_service_info"));
        assert_eq!(calls.recv().await, Some("list_events"));
        assert_eq!(calls.recv().await, Some("subscribe_events"));
        live_tx.send(Ok(event_live_frame(None, 0))).unwrap();
        live_tx.send(Ok(event_live_frame(Some(1), 1))).unwrap();
        live_tx
    });
    let progress = stream.next().await.unwrap().unwrap();
    assert!(progress.event.is_none());
    assert_eq!(progress.cursor, bytes(0));
    let item = stream.next().await.unwrap().unwrap();
    assert_eq!(item.event.unwrap().event_type.as_deref(), Some("event-1"));
    let live_tx = orchestration.await.unwrap();
    drop(stream);
    tokio::time::timeout(STREAM_DROP_TIMEOUT, live_tx.closed())
        .await
        .expect("dropping facade did not drop tonic response");
    tokio::task::yield_now().await;
    assert_eq!(server.state.lock().unwrap().calls.len(), 3);
}

#[tokio::test]
async fn dropping_facade_during_retry_sleep_prevents_later_rpc() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_event_lists([
        StreamScript::DispatchError(tonic::Status::unavailable("sleep")),
        StreamScript::frames([Ok(event_list_frame(
            None,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let mut config = ListConfig::default();
    config.base_retry_delay = Duration::from_millis(50);
    config.max_retry_delay = Duration::from_millis(50);
    config.retry_jitter = Duration::ZERO;
    let collector = tokio::spawn(async move {
        let stream = client.list_events_with_config(bounded_event_request(), config);
        futures::pin_mut!(stream);
        stream.next().await
    });
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    collector.abort();
    let _ = collector.await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(server.state.lock().unwrap().event_requests.len(), 1);
}

#[tokio::test]
async fn clean_subscription_eof_uses_the_transient_recovery_policy() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(0)), Ok(service_info(0))]);
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        0,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    server.push_event_subscriptions([
        StreamScript::frames([]),
        StreamScript::DispatchError(tonic::Status::permission_denied(
            "subscription retry denied",
        )),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let config = fast_config();
    let body = EventStreamRequest::new()
        .with_read_mask(event_identity_mask())
        .with_start(EventStreamStart::Resume(bytes(0)));
    let status = first_event_error(&client, body, config).await;
    assert_eq!(status.code(), tonic::Code::PermissionDenied);
    assert_eq!(server.state.lock().unwrap().event_subscriptions.len(), 2);
}

#[tokio::test]
async fn gap_recovery_without_list_coverage_still_plans_replay_on_reconnect() {
    let (server, mut calls) = ScriptedStreamServer::new();
    let (first_gap_tx, first_gap_rx) = mpsc::unbounded_channel();
    let (second_subscription_tx, second_subscription_rx) = mpsc::unbounded_channel();
    let (third_subscription_tx, third_subscription_rx) = mpsc::unbounded_channel();
    server.push_service_infos([Ok(service_info(0)), Ok(service_info(0))]);
    server.push_event_subscriptions([
        StreamScript::frames([
            Ok(event_live_frame(None, 0)),
            Err(tonic::Status::unavailable("initial subscription failed")),
        ]),
        StreamScript::Channel(second_subscription_rx),
        StreamScript::Channel(third_subscription_rx),
    ]);
    let mut first_gap_item = event_positioned_list_frame(Some(event(1)), 1, 0, None);
    first_gap_item.watermark.as_mut().unwrap().checkpoint = None;
    let mut first_gap_second_item = event_positioned_list_frame(Some(event(2)), 2, 0, None);
    first_gap_second_item.watermark.as_mut().unwrap().checkpoint = None;
    let mut first_gap_end =
        event_positioned_list_frame(None, 3, 0, Some(proto::QueryEndReason::CursorBound));
    first_gap_end.watermark.as_mut().unwrap().checkpoint = None;
    let mut second_gap_item = event_positioned_list_frame(Some(event(4)), 4, 0, None);
    second_gap_item.watermark.as_mut().unwrap().checkpoint = None;
    let mut second_gap_end =
        event_positioned_list_frame(None, 5, 0, Some(proto::QueryEndReason::CursorBound));
    second_gap_end.watermark.as_mut().unwrap().checkpoint = None;
    server.push_event_lists([
        StreamScript::Channel(first_gap_rx),
        StreamScript::frames([Ok(second_gap_item), Ok(second_gap_end)]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let collector = tokio::spawn(async move {
        client
            .stream_events_with_config(
                EventStreamRequest::new().with_read_mask(event_identity_mask()),
                fast_config(),
            )
            .try_filter_map(|frame| async move {
                Ok(frame.event.map(|event| event.event_type.unwrap()))
            })
            .take(3)
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(next_scripted_call(&mut calls).await, "subscribe_events");
    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "subscribe_events");
    second_subscription_tx
        .send(Ok(event_live_frame(Some(2), 3)))
        .unwrap();
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    first_gap_tx.send(Ok(first_gap_item)).unwrap();
    second_subscription_tx
        .send(Err(tonic::Status::unavailable(
            "subscription failed during gap replay",
        )))
        .unwrap();
    tokio::time::timeout(STREAM_DROP_TIMEOUT, second_subscription_tx.closed())
        .await
        .expect("failed gap subscription was not dropped");
    first_gap_tx.send(Ok(first_gap_second_item)).unwrap();
    first_gap_tx.send(Ok(first_gap_end)).unwrap();

    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "subscribe_events");
    third_subscription_tx
        .send(Ok(event_live_frame(Some(5), 5)))
        .unwrap();
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    assert_eq!(
        tokio::time::timeout(Duration::from_secs(2), collector)
            .await
            .expect("stream stalled instead of replaying the next cursor gap")
            .unwrap()
            .unwrap(),
        ["event-1", "event-2", "event-4"]
    );
}

#[tokio::test]
async fn server_internal_mid_stream_is_terminal() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_subscriptions([
        StreamScript::frames([
            Ok(event_live_frame(None, 10)),
            Ok(event_live_frame(Some(11), 11)),
            Err(tonic::Status::internal("server bug")),
        ]),
        StreamScript::frames([Ok(event_live_frame(None, 11))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let stream = client.stream_events_with_config(
        EventStreamRequest::new().with_read_mask(event_identity_mask()),
        fast_config(),
    );
    futures::pin_mut!(stream);

    assert!(stream.next().await.unwrap().is_ok());
    assert!(stream.next().await.unwrap().is_ok());
    let status = tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .expect("timed out waiting for terminal Internal")
        .unwrap()
        .unwrap_err();

    assert_eq!(status.code(), tonic::Code::Internal);
    assert_eq!(status.message(), "server bug");
    assert!(stream.next().await.is_none());
    assert_eq!(server.state.lock().unwrap().calls, ["subscribe_events"]);
}

#[tokio::test]
async fn live_stream_cancelled_and_unknown_interruptions_reconnect() {
    for interruption in [
        tonic::Status::cancelled("server cancelled response body"),
        tonic::Status::unknown("h2 response body failure"),
    ] {
        let (server, _calls) = ScriptedStreamServer::new();
        server.push_service_infos([Ok(service_info(21))]);
        server.push_event_subscriptions([
            StreamScript::frames([
                Ok(event_live_frame(None, 20)),
                Ok(event_live_frame(Some(21), 21)),
                Err(interruption),
            ]),
            StreamScript::frames([
                Ok(event_live_frame(None, 21)),
                Ok(event_live_frame(Some(22), 22)),
            ]),
        ]);
        let address = spawn_server(server.clone()).await;
        let (client, _observations) = observed_client(address);

        let events = tokio::time::timeout(
            Duration::from_secs(2),
            client
                .stream_events_with_config(
                    EventStreamRequest::new().with_read_mask(event_identity_mask()),
                    fast_config(),
                )
                .try_filter_map(|frame| async move {
                    Ok(frame.event.map(|event| event.event_type.unwrap()))
                })
                .take(2)
                .try_collect::<Vec<_>>(),
        )
        .await
        .expect("timed out waiting for a body interruption retry")
        .unwrap();

        assert_eq!(events, ["event-21", "event-22"]);
        assert_eq!(
            server.state.lock().unwrap().calls,
            ["subscribe_events", "get_service_info", "subscribe_events"]
        );
    }
}

#[tokio::test]
async fn checkpoint_tip_start_uses_list_while_initial_subscriptions_fail_before_first_frame() {
    let (server, _calls) = ScriptedStreamServer::new();
    let (_stalled_subscription_tx, stalled_subscription_rx) = mpsc::unbounded_channel();
    server.push_checkpoint_subscriptions([
        StreamScript::frames([Err(tonic::Status::deadline_exceeded("idle watchdog"))]),
        StreamScript::frames([Err(tonic::Status::deadline_exceeded("idle watchdog"))]),
        StreamScript::Channel(stalled_subscription_rx),
    ]);
    server.push_service_infos([Ok(service_info(0)), Ok(service_info(2))]);
    server.push_checkpoint_lists([
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(0), 0, None)),
            Ok(checkpoint_list_frame(
                None,
                0,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(1), 1, None)),
            Ok(checkpoint_list_frame(Some(2), 2, None)),
            Ok(checkpoint_list_frame(
                None,
                2,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let (config, mut stream_events) = recording_config(fast_config());

    let checkpoints = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_checkpoints_with_config(
                CheckpointStreamRequest::new().with_read_mask(checkpoint_identity_mask()),
                config,
            )
            .try_filter_map(|frame| async move {
                Ok(frame
                    .checkpoint
                    .and_then(|checkpoint| checkpoint.sequence_number))
            })
            .take(2)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("pre-frame subscription failures starved live-tip List recovery")
    .unwrap();

    assert_eq!(checkpoints, [1, 2]);
    let events: Vec<_> = std::iter::from_fn(|| stream_events.try_recv().ok()).collect();
    assert_eq!(
        events
            .iter()
            .filter(|event| matches!(
                event,
                LedgerStreamEvent::SubscriptionStreamInterrupted { status, .. }
                    if status.code() == tonic::Code::DeadlineExceeded
            ))
            .count(),
        2,
        "unexpected subscription interruption history: {events:#?}"
    );
    assert_eq!(
        server.state.lock().unwrap().calls,
        [
            "subscribe_checkpoints",
            "get_service_info",
            "list_checkpoints",
            "subscribe_checkpoints",
            "get_service_info",
            "list_checkpoints",
        ]
    );
}

#[tokio::test]
async fn checkpoint_reconnect_uses_list_when_replacement_fails_before_first_frame() {
    let (server, _calls) = ScriptedStreamServer::new();
    let (_stalled_subscription_tx, stalled_subscription_rx) = mpsc::unbounded_channel();
    server.push_checkpoint_subscriptions([
        StreamScript::frames([
            Ok(checkpoint_live_frame(Some(0), 0)),
            Err(tonic::Status::deadline_exceeded("idle watchdog")),
        ]),
        StreamScript::frames([Err(tonic::Status::deadline_exceeded("idle watchdog"))]),
        StreamScript::Channel(stalled_subscription_rx),
    ]);
    server.push_service_infos([Ok(service_info(0)), Ok(service_info(2))]);
    server.push_checkpoint_lists([StreamScript::frames([
        Ok(checkpoint_list_frame(Some(1), 1, None)),
        Ok(checkpoint_list_frame(Some(2), 2, None)),
        Ok(checkpoint_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let (config, mut stream_events) = recording_config(fast_config());

    let checkpoints = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_checkpoints_with_config(
                CheckpointStreamRequest::new().with_read_mask(checkpoint_identity_mask()),
                config,
            )
            .try_filter_map(|frame| async move {
                Ok(frame
                    .checkpoint
                    .and_then(|checkpoint| checkpoint.sequence_number))
            })
            .take(3)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("repeated pre-frame subscription failures starved List recovery")
    .unwrap();

    assert_eq!(checkpoints, [0, 1, 2]);
    let events: Vec<_> = std::iter::from_fn(|| stream_events.try_recv().ok()).collect();
    assert!(
        events.iter().any(|event| matches!(
            event,
            LedgerStreamEvent::GapRecoveryStarted {
                family: LedgerStreamFamily::Checkpoint,
                ..
            }
        )),
        "checkpoint List recovery did not start: {events:#?}"
    );
    assert_eq!(
        server.state.lock().unwrap().calls,
        [
            "subscribe_checkpoints",
            "get_service_info",
            "subscribe_checkpoints",
            "get_service_info",
            "list_checkpoints",
        ]
    );
}

#[tokio::test]
async fn list_body_unknown_interruption_retries() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Err(tonic::Status::unknown(
                "error reading a body from connection",
            )),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(6), 6, None)),
            Ok(event_list_frame(
                None,
                7,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let event_ids = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .list_events_with_config(
                bounded_event_request().with_end_checkpoint(8),
                fast_list_config(),
            )
            .try_filter_map(
                |frame| async move { Ok(frame.event.and_then(|event| event.checkpoint)) },
            )
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for an Internal List body retry")
    .unwrap();

    assert_eq!(event_ids, [5, 6]);
    let state = server.state.lock().unwrap();
    assert_eq!(state.calls, ["list_events", "list_events"]);
    assert_eq!(
        state.event_requests[1].body.options.as_ref().unwrap().after,
        Some(bytes(5))
    );
}

#[tokio::test]
async fn live_item_with_repeated_cursor_is_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_subscriptions([StreamScript::frames([
        Ok(event_live_frame(Some(1), 1)),
        Ok(event_live_frame(Some(2), 1)),
    ])]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);

    let stream = client.stream_events_with_config(
        EventStreamRequest::new().with_read_mask(event_identity_mask()),
        fast_config(),
    );
    futures::pin_mut!(stream);
    let item = stream.next().await.unwrap().unwrap();
    assert_eq!(
        item.event.as_ref().unwrap().event_type.as_deref(),
        Some("event-1")
    );
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(status.message(), "subscription item repeated its cursor");
    assert!(stream.next().await.is_none());
}

#[tokio::test]
async fn muted_subscription_item_with_repeated_cursor_is_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10))]);
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        10,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    server.push_event_subscriptions([StreamScript::frames([
        Ok(event_live_frame(None, 8)),
        Ok(event_live_frame(Some(9), 8)),
    ])]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);

    let stream = client.stream_events_with_config(
        EventStreamRequest::new()
            .with_read_mask(event_identity_mask())
            .with_start(EventStreamStart::Checkpoint(0)),
        fast_config(),
    );
    futures::pin_mut!(stream);
    let history = stream.next().await.unwrap().unwrap();
    assert!(history.event.is_none());
    assert_eq!(history.cursor, bytes(10));
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(status.message(), "subscription item repeated its cursor");
    assert!(stream.next().await.is_none());
}

#[tokio::test]
async fn gap_buffered_item_with_repeated_cursor_is_terminal_data_loss() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(0))]);
    let (live_tx, live_rx) = mpsc::unbounded_channel();
    let (gap_tx, gap_rx) = mpsc::unbounded_channel();
    server.push_event_subscriptions([StreamScript::Channel(live_rx)]);
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::Channel(gap_rx),
    ]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);
    let collector = tokio::spawn(async move {
        let stream = client.stream_events_with_config(
            EventStreamRequest::new()
                .with_read_mask(event_identity_mask())
                .with_start(EventStreamStart::Checkpoint(0)),
            fast_config(),
        );
        futures::pin_mut!(stream);
        let mut frames = Vec::new();
        while let Some(result) = stream.next().await {
            match result {
                Ok(frame) => frames.push((
                    frame.event.map(|event| event.event_type.unwrap()),
                    frame.cursor,
                )),
                Err(status) => {
                    let ended = stream.next().await.is_none();
                    return (frames, status, ended);
                }
            }
        }
        panic!("stream ended without reporting repeated subscription cursor");
    });

    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    assert_eq!(next_scripted_call(&mut calls).await, "subscribe_events");
    live_tx.send(Ok(event_live_frame(Some(2), 2))).unwrap();
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    live_tx.send(Ok(event_live_frame(Some(3), 2))).unwrap();
    tokio::time::timeout(STREAM_DROP_TIMEOUT, live_tx.closed())
        .await
        .expect("malformed pinned subscription was not dropped");
    gap_tx.send(Ok(event_list_frame(Some(1), 1, None))).unwrap();
    gap_tx
        .send(Ok(event_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::CursorBound),
        )))
        .unwrap();

    let (frames, status, ended) = collector.await.unwrap();
    assert_eq!(
        frames,
        [
            (None, bytes(0)),
            (Some("event-1".to_owned()), bytes(1)),
            (Some("event-2".to_owned()), bytes(2)),
        ]
    );
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(status.message(), "subscription item repeated its cursor");
    assert!(ended);
}

#[tokio::test]
async fn equal_cursor_coverage_growth_updates_recovery_planning() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(21))]);
    server.push_event_subscriptions([
        StreamScript::frames([
            Ok(event_positioned_live_frame(
                Some(event_at(20, 0, 0, "event-20")),
                10,
                20,
            )),
            Ok(event_positioned_live_frame(None, 10, 21)),
            Err(tonic::Status::unavailable("live stream interrupted")),
        ]),
        StreamScript::frames([
            Ok(event_positioned_live_frame(None, 10, 21)),
            Ok(event_positioned_live_frame(
                Some(event_at(21, 0, 0, "event-21")),
                11,
                21,
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let (config, mut observer_events) = recording_config(fast_config());

    let frames = client
        .stream_events_with_config(
            EventStreamRequest::new().with_read_mask(event_identity_mask()),
            config,
        )
        .take(2)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(frames[0].cursor, bytes(10));
    assert_eq!(frames[0].covered_checkpoint, Some(20));
    assert_eq!(frames[1].cursor, bytes(11));
    assert_eq!(frames[1].covered_checkpoint, Some(21));
    while let Ok(event) = observer_events.try_recv() {
        assert!(!matches!(
            event,
            LedgerStreamEvent::GapRecoveryStarted { .. }
        ));
    }
    assert!(server.state.lock().unwrap().event_requests.is_empty());
}

#[tokio::test]
async fn same_cursor_reconnect_preserves_committed_checkpoint_coverage() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10)), Ok(service_info(10))]);
    server.push_event_subscriptions([
        StreamScript::frames([
            Ok(event_positioned_live_frame(
                Some(event_at(8, 0, 0, "already-delivered")),
                100,
                10,
            )),
            Err(tonic::Status::unavailable("first interruption")),
        ]),
        StreamScript::frames([
            Ok(event_positioned_live_frame(None, 100, 5)),
            Ok(event_positioned_live_frame(None, 100, 6)),
            Err(tonic::Status::unavailable("second interruption")),
        ]),
        StreamScript::frames([
            Ok(event_positioned_live_frame(None, 200, 8)),
            Ok(event_positioned_live_frame(
                Some(event_at(8, 0, 0, "already-delivered")),
                201,
                9,
            )),
            Ok(event_positioned_live_frame(None, 100, 10)),
            Ok(event_positioned_live_frame(
                Some(event_at(11, 0, 0, "after-committed-frontier")),
                300,
                11,
            )),
        ]),
    ]);
    server.push_event_lists([StreamScript::frames([Ok(event_positioned_list_frame(
        None,
        100,
        6,
        Some(proto::QueryEndReason::CursorBound),
    ))])]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);

    let frames = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_events_with_config(
                EventStreamRequest::new().with_read_mask(event_identity_mask()),
                fast_config(),
            )
            .take(2)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("muted subscription did not reach the committed frontier")
    .unwrap();

    assert_eq!(
        frames
            .iter()
            .map(|frame| frame.cursor.clone())
            .collect::<Vec<_>>(),
        [bytes(100), bytes(300)]
    );
    assert_eq!(
        frames
            .iter()
            .filter_map(|frame| frame.event.as_ref())
            .map(|event| event.event_type.as_deref().unwrap())
            .collect::<Vec<_>>(),
        ["already-delivered", "after-committed-frontier"]
    );
}
