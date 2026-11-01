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
use super::support::event_position;
use super::support::event_positioned_list_frame;
use super::support::event_positioned_live_frame;
use super::support::fast_config;
use super::support::fast_list_config;
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
use super::support::transaction_position;
use super::support::transaction_positioned_list_frame;
use super::support::transaction_positioned_live_frame;
use super::support::with_position;
use futures::StreamExt;
use futures::TryStreamExt;
use prost::bytes::Bytes;
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
use sui_rpc::client::TransactionStreamFrame;
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

/// A subscription whose first item lies in the checkpoint committed progress reached replays the
/// gap before that item at once, without waiting for another subscription frame.
#[tokio::test]
async fn same_checkpoint_transaction_reconnect_replays_the_gap_before_the_first_live_item() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(9))]);
    let (subscription_tx, subscription_rx) = mpsc::unbounded_channel();
    let (recovery_tx, recovery_rx) = mpsc::unbounded_channel();
    server.push_transaction_subscriptions([StreamScript::Channel(subscription_rx)]);
    server.push_transaction_lists([
        // The resumed scan ends at the start of checkpoint 10.
        StreamScript::frames([Ok(transaction_positioned_list_frame(
            None,
            100,
            9,
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
        .with_start(TransactionStreamStart::Resume {
            cursor: bytes(50),
            position: transaction_position(9, 3),
        });
    let collector = tokio::spawn(async move {
        let mut config = fast_config();
        config.list_page_limit = Some(7);
        client
            .stream_transactions_with_config(body, config)
            .take(5)
            .map_ok(|frame| {
                (
                    frame.transaction.and_then(|transaction| transaction.digest),
                    frame.cursor,
                )
            })
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
            Some(transaction_at(10, 1, "first-live")),
            900,
            9,
        )))
        .unwrap();
    assert_eq!(
        next_scripted_call(&mut calls).await,
        "list_transactions",
        "same-checkpoint recovery must not wait for another subscription frame"
    );
    subscription_tx
        .send(Ok(transaction_positioned_live_frame(
            Some(transaction_at(10, 2, "buffered-live")),
            901,
            9,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(transaction_positioned_list_frame(
            Some(transaction_at(10, 0, "list-only")),
            800,
            9,
            None,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(with_position(
            transaction_positioned_list_frame(
                None,
                900,
                9,
                Some(proto::QueryEndReason::CursorBound),
            ),
            transaction_position(10, 1),
        )))
        .unwrap();
    subscription_tx
        .send(Ok(transaction_positioned_live_frame(
            Some(transaction_at(11, 0, "live-after-drain")),
            1_000,
            10,
        )))
        .unwrap();

    let frames = tokio::time::timeout(Duration::from_secs(2), collector)
        .await
        .expect("same-checkpoint transaction recovery stalled")
        .unwrap()
        .unwrap();
    let digest = |value: &str| Some(value.to_owned());
    // The replay's end names the position transaction `list-only` reached, so it is not yielded.
    assert_eq!(
        frames,
        [
            (None, bytes(100)),
            (digest("list-only"), bytes(800)),
            (digest("first-live"), bytes(900)),
            (digest("buffered-live"), bytes(901)),
            (digest("live-after-drain"), bytes(1_000)),
        ]
    );

    let state = server.state.lock().unwrap();
    assert_eq!(state.transaction_subscriptions.len(), 1);
    assert_eq!(state.transaction_requests.len(), 2);
    let initial_request = &state.transaction_requests[0].body;
    assert_eq!(initial_request.start_checkpoint, Some(9));
    assert_eq!(initial_request.start_transaction_index, Some(3));
    assert_eq!(initial_request.end_checkpoint, Some(10));
    assert_eq!(initial_request.end_transaction_index, None);
    let initial_options = initial_request.options.as_ref().unwrap();
    assert_eq!(initial_options.after, Some(bytes(50)));
    assert_eq!(initial_options.before, None);
    assert_eq!(initial_options.limit, Some(7));
    let recovery_request = &state.transaction_requests[1].body;
    assert_eq!(recovery_request.start_checkpoint, Some(10));
    assert_eq!(recovery_request.start_transaction_index, None);
    assert_eq!(recovery_request.end_checkpoint, Some(11));
    assert_eq!(recovery_request.end_transaction_index, Some(1));
    let recovery_options = recovery_request.options.as_ref().unwrap();
    assert_eq!(recovery_options.after, Some(bytes(100)));
    assert_eq!(recovery_options.before, Some(bytes(900)));
    assert_eq!(recovery_options.limit, Some(7));
    assert!(recovery_options.ordering.is_none());
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

/// A progress-only first frame inside the checkpoint committed progress reached also bounds a gap
/// replay, which starts at once.
#[tokio::test]
async fn same_checkpoint_progress_only_reconnect_replays_immediately() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(9))]);
    let (subscription_tx, subscription_rx) = mpsc::unbounded_channel();
    let (recovery_tx, recovery_rx) = mpsc::unbounded_channel();
    server.push_transaction_subscriptions([StreamScript::Channel(subscription_rx)]);
    server.push_transaction_lists([
        // The initial scan ends at the start of checkpoint 10.
        StreamScript::frames([Ok(transaction_positioned_list_frame(
            None,
            100,
            9,
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
            .take(3)
            .map_ok(|frame| {
                (
                    frame.transaction.and_then(|transaction| transaction.digest),
                    frame.cursor,
                )
            })
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
        .send(Ok(with_position(
            transaction_positioned_live_frame(None, 900, 9),
            transaction_position(10, 1),
        )))
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
            9,
            None,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(with_position(
            transaction_positioned_list_frame(
                None,
                900,
                9,
                Some(proto::QueryEndReason::CursorBound),
            ),
            transaction_position(10, 1),
        )))
        .unwrap();
    subscription_tx
        .send(Ok(transaction_positioned_live_frame(
            Some(transaction_at(11, 0, "live")),
            1_000,
            10,
        )))
        .unwrap();

    let frames = tokio::time::timeout(Duration::from_secs(2), collector)
        .await
        .expect("progress-only same-checkpoint recovery stalled")
        .unwrap()
        .unwrap();
    // The replay's end and the buffered first frame both name `(10, 1)`, the position the
    // replayed transaction reached, so neither is yielded.
    assert_eq!(
        frames,
        [
            (None, bytes(100)),
            (Some("replayed".to_owned()), bytes(800)),
            (Some("live".to_owned()), bytes(1_000)),
        ]
    );
    let state = server.state.lock().unwrap();
    let recovery_request = &state.transaction_requests[1].body;
    assert_eq!(recovery_request.start_checkpoint, Some(10));
    assert_eq!(recovery_request.start_transaction_index, None);
    assert_eq!(recovery_request.end_checkpoint, Some(11));
    assert_eq!(recovery_request.end_transaction_index, Some(1));
    let options = recovery_request.options.as_ref().unwrap();
    assert_eq!(options.after, Some(bytes(100)));
    assert_eq!(options.before, Some(bytes(900)));
}

#[tokio::test]
async fn same_checkpoint_event_reconnect_replays_the_gap_before_the_first_live_item() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(9))]);
    let (subscription_tx, subscription_rx) = mpsc::unbounded_channel();
    let (recovery_tx, recovery_rx) = mpsc::unbounded_channel();
    server.push_event_subscriptions([StreamScript::Channel(subscription_rx)]);
    server.push_event_lists([
        // The initial scan ends at the start of checkpoint 10.
        StreamScript::frames([Ok(event_positioned_list_frame(
            None,
            100,
            9,
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
            .take(6)
            .map_ok(|frame| {
                (
                    frame.event.map(|event| event.event_type.unwrap()),
                    frame.cursor,
                )
            })
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    assert_eq!(next_scripted_call(&mut calls).await, "subscribe_events");
    subscription_tx
        .send(Ok(event_positioned_live_frame(
            Some(event_at(10, 1, 0, "first-live")),
            900,
            9,
        )))
        .unwrap();
    assert_eq!(
        next_scripted_call(&mut calls).await,
        "list_events",
        "same-checkpoint recovery must not wait for another subscription frame"
    );
    subscription_tx
        .send(Ok(event_positioned_live_frame(
            Some(event_at(10, 1, 1, "buffered-live")),
            901,
            9,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(event_positioned_list_frame(
            Some(event_at(10, 0, 0, "list-only")),
            800,
            9,
            None,
        )))
        .unwrap();
    recovery_tx
        .send(Ok(with_position(
            event_positioned_list_frame(None, 900, 9, Some(proto::QueryEndReason::CursorBound)),
            event_position(10, 1, 0),
        )))
        .unwrap();
    subscription_tx
        .send(Ok(event_positioned_live_frame(
            Some(event_at(11, 0, 0, "live-after-drain")),
            1_000,
            10,
        )))
        .unwrap();

    let frames = tokio::time::timeout(Duration::from_secs(2), collector)
        .await
        .expect("same-checkpoint event recovery stalled")
        .unwrap()
        .unwrap();
    let event_type = |value: &str| Some(value.to_owned());
    // The replay's end names `(10, 1, 0)`, past the `(10, 0, 1)` that event `list-only` reached,
    // so it is yielded as progress before the buffered live items.
    assert_eq!(
        frames,
        [
            (None, bytes(100)),
            (event_type("list-only"), bytes(800)),
            (None, bytes(900)),
            (event_type("first-live"), bytes(900)),
            (event_type("buffered-live"), bytes(901)),
            (event_type("live-after-drain"), bytes(1_000)),
        ]
    );

    let state = server.state.lock().unwrap();
    assert_eq!(state.event_subscriptions.len(), 1);
    assert_eq!(state.event_requests.len(), 2);
    let recovery_request = &state.event_requests[1].body;
    assert_eq!(recovery_request.start_checkpoint, Some(10));
    assert_eq!(recovery_request.start_transaction_index, None);
    assert_eq!(recovery_request.start_event_index, None);
    assert_eq!(recovery_request.end_checkpoint, Some(11));
    assert_eq!(recovery_request.end_transaction_index, Some(1));
    assert_eq!(recovery_request.end_event_index, None);
    assert_eq!(
        recovery_request.options.as_ref().unwrap().after,
        Some(bytes(100))
    );
    assert_eq!(
        recovery_request.options.as_ref().unwrap().before,
        Some(bytes(900))
    );
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
    // Each gap ends at the start of checkpoint 7, before the first live item.
    server.push_transaction_lists([
        StreamScript::frames([Ok(transaction_list_frame(
            None,
            5,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
        StreamScript::frames([
            Ok(transaction_list_frame(Some(6), 6, None)),
            Ok(transaction_positioned_list_frame(
                None,
                7,
                6,
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
            Ok(event_positioned_list_frame(
                None,
                7,
                6,
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
    let transaction_gap = &state.transaction_requests[1].body;
    assert_eq!(transaction_gap.start_checkpoint, Some(6));
    assert_eq!(transaction_gap.start_transaction_index, None);
    assert_eq!(transaction_gap.end_checkpoint, Some(7));
    assert_eq!(transaction_gap.end_transaction_index, None);
    let options = transaction_gap.options.as_ref().unwrap();
    assert_eq!(options.after, Some(bytes(5)));
    assert_eq!(options.before, Some(bytes(7)));
    assert_eq!(state.event_requests[0].body.end_checkpoint, Some(6));
    let event_gap = &state.event_requests[1].body;
    assert_eq!(event_gap.start_checkpoint, Some(6));
    assert_eq!(event_gap.start_transaction_index, None);
    assert_eq!(event_gap.start_event_index, None);
    assert_eq!(event_gap.end_checkpoint, Some(7));
    assert_eq!(event_gap.end_transaction_index, None);
    assert_eq!(event_gap.end_event_index, None);
    let options = event_gap.options.as_ref().unwrap();
    assert_eq!(options.after, Some(bytes(5)));
    assert_eq!(options.before, Some(bytes(7)));
    assert_eq!(state.transaction_subscriptions.len(), 1);
    assert_eq!(state.event_subscriptions.len(), 1);
}

/// The replacement subscription starts ahead of the replica serving the gap replay. That replica
/// first answers at its indexed tip, echoing the replay's start, and then catches up.
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

    // Both gap requests list from the start of checkpoint 13 to the start of checkpoint 17: the
    // retry after `LedgerTip` resumes from the echoed start.
    let state = server.state.lock().unwrap();
    assert_eq!(state.event_requests.len(), 3);
    for request in &state.event_requests[1..] {
        let request = &request.body;
        assert_eq!(request.start_checkpoint, Some(13));
        assert_eq!(request.start_transaction_index, None);
        assert_eq!(request.start_event_index, None);
        assert_eq!(request.end_checkpoint, Some(17));
        assert_eq!(request.end_transaction_index, None);
        assert_eq!(request.end_event_index, None);
        let options = request.options.as_ref().unwrap();
        assert_eq!(options.after, Some(bytes(12)));
        assert_eq!(options.before, Some(bytes(16)));
    }
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
            Ok(event_positioned_list_frame(
                None,
                7,
                6,
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
            .take(9)
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
    // The gap ends before event `(2, 0, 0)`, the first live frame's item.
    gap_tx
        .send(Ok(event_positioned_list_frame(
            None,
            2,
            1,
            Some(proto::QueryEndReason::CursorBound),
        )))
        .unwrap();

    let events = collector.await.unwrap().unwrap();
    // Each replay ends with a progress frame at the start of the checkpoint holding its first
    // live item; the progress-only live frames 3 and 4 coalesce into one.
    assert_eq!(
        events,
        [
            (None, bytes(0)),
            (Some("event-1".to_owned()), bytes(1)),
            (None, bytes(2)),
            (Some("event-2".to_owned()), bytes(2)),
            (None, bytes(4)),
            (Some("event-5".to_owned()), bytes(5)),
            (Some("event-6".to_owned()), bytes(6)),
            (None, bytes(7)),
            (Some("event-7".to_owned()), bytes(7)),
        ]
    );
    let state = server.state.lock().unwrap();
    // The subscription dropped at the buffer limit resumes after event 5, its last buffered item.
    let options = state.event_requests[2].body.options.as_ref().unwrap();
    assert_eq!(options.after, Some(bytes(5)));
    assert_eq!(options.before, Some(bytes(7)));
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
            Ok(event_positioned_list_frame(
                None,
                5,
                4,
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
    // The gap ends before event `(2, 0, 0)`, the first live frame's item.
    gap_tx
        .send(Ok(event_positioned_list_frame(
            None,
            2,
            1,
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
    let mut unlimited_config = fast_list_config();
    unlimited_config.max_consecutive_retries = None;
    let events = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .list_events_with_config(bounded_event_request(), unlimited_config)
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
async fn list_yields_the_failure_after_the_default_consecutive_retries() {
    let retries = ListConfig::DEFAULT_MAX_CONSECUTIVE_RETRIES;
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists(
        (0..=retries)
            .map(|attempt| {
                StreamScript::DispatchError(tonic::Status::unavailable(format!(
                    "attempt-{attempt}"
                )))
            })
            .chain([StreamScript::frames([Ok(event_positioned_list_frame(
                None,
                10,
                9,
                Some(proto::QueryEndReason::CheckpointBound),
            ))])]),
    );
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let status = client
        .list_events_with_config(bounded_event_request(), fast_list_config())
        .try_collect::<Vec<_>>()
        .await
        .unwrap_err();
    assert_eq!(status.code(), tonic::Code::Unavailable);
    assert_eq!(status.message(), format!("attempt-{retries}"));
    assert_eq!(
        server.state.lock().unwrap().event_requests.len(),
        retries as usize + 1
    );
}

#[tokio::test]
async fn list_progress_restarts_the_consecutive_retry_count() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(2), 2, None)),
            Err(tonic::Status::unavailable("body failed after 2")),
        ]),
        StreamScript::DispatchError(tonic::Status::unavailable("dispatch failed after 2")),
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Err(tonic::Status::unavailable("body failed after 5")),
        ]),
        StreamScript::DispatchError(tonic::Status::unavailable("dispatch failed after 5")),
        StreamScript::frames([Ok(event_positioned_list_frame(
            None,
            10,
            9,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let mut config = fast_list_config();
    config.max_consecutive_retries = Some(2);

    let events = client
        .list_events_with_config(bounded_event_request(), config)
        .try_filter_map(
            |frame| async move { Ok(frame.event.map(|event| event.event_type.unwrap())) },
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(events, ["event-2", "event-5"]);
    assert_eq!(server.state.lock().unwrap().event_requests.len(), 5);
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
    config.max_consecutive_retries = None;
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
        // The gap ends at the first live frame's position, the start of checkpoint 13.
        StreamScript::frames([Ok(event_list_frame(
            None,
            12,
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
    assert_eq!(
        status.message(),
        "subscription checkpoint coverage regressed"
    );
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
    let body = EventStreamRequest::new()
        .with_read_mask(event_identity_mask())
        .with_start(EventStreamStart::Resume {
            cursor: bytes(0),
            position: event_position(0, 0, 1),
        });
    let stream = client.stream_events_with_config(body, fast_config());
    futures::pin_mut!(stream);
    // The resumed scan ends at the start of checkpoint 1, past the resume position.
    let progress = stream.next().await.unwrap().unwrap();
    assert!(progress.event.is_none());
    assert_eq!(progress.cursor, bytes(0));
    let status = tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .expect("timed out waiting for the terminal Subscribe error")
        .unwrap()
        .unwrap_err();
    assert_eq!(status.code(), tonic::Code::PermissionDenied);
    assert!(stream.next().await.is_none());
    assert_eq!(server.state.lock().unwrap().event_subscriptions.len(), 2);
}

/// Gap replay List frames that report no checkpoint coverage still commit progress, so a reconnect
/// after that replay plans the next gap from it.
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
    let without_coverage = |mut frame: proto::ListEventsResponse| {
        frame.watermark.as_mut().unwrap().checkpoint = None;
        frame
    };
    // Each gap ends at the start of the checkpoint holding its subscription's first item.
    let first_gap_item = without_coverage(event_positioned_list_frame(Some(event(1)), 1, 0, None));
    let first_gap_end = without_coverage(event_positioned_list_frame(
        None,
        2,
        1,
        Some(proto::QueryEndReason::CursorBound),
    ));
    let second_gap_item = without_coverage(event_positioned_list_frame(Some(event(4)), 4, 0, None));
    let second_gap_end = without_coverage(event_positioned_list_frame(
        None,
        5,
        4,
        Some(proto::QueryEndReason::CursorBound),
    ));
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
        .send(Ok(event_live_frame(Some(2), 2)))
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
            .expect("stream stalled instead of replaying the next gap")
            .unwrap()
            .unwrap(),
        ["event-1", "event-2", "event-4"]
    );
    // The second gap starts after event 2, the buffered item the first gap delivered.
    let state = server.state.lock().unwrap();
    let second_gap = &state.event_requests[1].body;
    assert_eq!(second_gap.start_checkpoint, Some(2));
    assert_eq!(second_gap.start_transaction_index, Some(0));
    assert_eq!(second_gap.start_event_index, Some(1));
    assert_eq!(second_gap.end_checkpoint, Some(5));
    assert_eq!(second_gap.end_transaction_index, None);
    let options = second_gap.options.as_ref().unwrap();
    assert_eq!(options.after, Some(bytes(2)));
    assert_eq!(options.before, Some(bytes(5)));
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
                // The subscription reaches the start of checkpoint 22, past the tip, so the
                // reconnect needs no List.
                Ok(event_live_frame(None, 21)),
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
async fn live_item_before_the_previous_position_is_terminal_data_loss() {
    let (server, _calls) = ScriptedStreamServer::new();
    // The second frame repeats event `(1, 0, 0)`, which lies before the first frame's position.
    server.push_event_subscriptions([StreamScript::frames([
        Ok(event_live_frame(Some(1), 1)),
        Ok(event_positioned_live_frame(Some(event(1)), 2, 1)),
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
    assert_eq!(
        status.message(),
        "subscription item did not advance its cursor_position"
    );
    assert!(stream.next().await.is_none());
}

#[tokio::test]
async fn gap_buffered_item_before_the_previous_position_is_terminal_data_loss() {
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
        panic!("stream ended without reporting the repeated subscription item");
    });

    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    assert_eq!(next_scripted_call(&mut calls).await, "subscribe_events");
    live_tx.send(Ok(event_live_frame(Some(2), 2))).unwrap();
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    live_tx
        .send(Ok(event_positioned_live_frame(Some(event(2)), 3, 2)))
        .unwrap();
    tokio::time::timeout(STREAM_DROP_TIMEOUT, live_tx.closed())
        .await
        .expect("malformed pinned subscription was not dropped");
    gap_tx.send(Ok(event_list_frame(Some(1), 1, None))).unwrap();
    // The gap ends before event `(2, 0, 0)`, the first live frame's item.
    gap_tx
        .send(Ok(event_positioned_list_frame(
            None,
            2,
            1,
            Some(proto::QueryEndReason::CursorBound),
        )))
        .unwrap();

    let (frames, status, ended) = collector.await.unwrap();
    assert_eq!(
        frames,
        [
            (None, bytes(0)),
            (Some("event-1".to_owned()), bytes(1)),
            (None, bytes(2)),
            (Some("event-2".to_owned()), bytes(2)),
        ]
    );
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(
        status.message(),
        "subscription item did not advance its cursor_position"
    );
    assert!(ended);
}

/// An item-free frame that advances committed progress past the last item moves the reconnect
/// boundary with it: a replacement subscription starting at that position needs no replay.
#[tokio::test]
async fn item_free_progress_advances_the_reconnect_boundary() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(21))]);
    server.push_event_subscriptions([
        StreamScript::frames([
            Ok(event_positioned_live_frame(
                Some(event_at(20, 0, 0, "event-20")),
                10,
                20,
            )),
            // Checkpoint 21 holds no match, so the subscription reaches the start of 22.
            Ok(event_positioned_live_frame(None, 11, 21)),
            Err(tonic::Status::unavailable("live stream interrupted")),
        ]),
        StreamScript::frames([
            Ok(event_positioned_live_frame(None, 12, 21)),
            Ok(event_positioned_live_frame(
                Some(event_at(22, 0, 0, "event-22")),
                13,
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
        .take(3)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        frames
            .iter()
            .map(|frame| (
                frame
                    .event
                    .as_ref()
                    .map(|event| event.event_type.clone().unwrap()),
                frame.cursor.clone(),
                frame.position,
                frame.covered_checkpoint,
            ))
            .collect::<Vec<_>>(),
        [
            (
                Some("event-20".to_owned()),
                bytes(10),
                event_position(20, 0, 1),
                Some(20)
            ),
            (None, bytes(11), event_position(22, 0, 0), Some(21)),
            (
                Some("event-22".to_owned()),
                bytes(13),
                event_position(22, 0, 1),
                Some(21)
            ),
        ]
    );
    while let Ok(event) = observer_events.try_recv() {
        assert!(!matches!(
            event,
            LedgerStreamEvent::GapRecoveryStarted { .. }
        ));
    }
    assert!(server.state.lock().unwrap().event_requests.is_empty());
}

/// Replacement subscriptions that start behind committed progress are hidden until they pass it,
/// and their hidden frames never move committed progress, so a later reconnect still needs no
/// List.
#[tokio::test]
async fn lagging_reconnects_hide_repeated_frames_without_moving_committed_progress() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10)), Ok(service_info(10))]);
    server.push_event_subscriptions([
        StreamScript::frames([
            Ok(event_positioned_live_frame(
                Some(event_at(10, 0, 0, "already-delivered")),
                100,
                10,
            )),
            // Committed progress reaches the start of checkpoint 11, past the tip.
            Ok(event_positioned_live_frame(None, 101, 10)),
            Err(tonic::Status::unavailable("first interruption")),
        ]),
        StreamScript::frames([
            Ok(event_positioned_live_frame(None, 200, 5)),
            Ok(event_positioned_live_frame(None, 201, 6)),
            Err(tonic::Status::unavailable("second interruption")),
        ]),
        StreamScript::frames([
            Ok(event_positioned_live_frame(None, 300, 8)),
            Ok(event_positioned_live_frame(
                Some(event_at(10, 0, 0, "already-delivered")),
                301,
                9,
            )),
            Ok(event_positioned_live_frame(
                Some(event_at(11, 0, 0, "after-committed-position")),
                302,
                10,
            )),
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
            .take(3)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("lagging subscription did not pass the committed position")
    .unwrap();

    assert_eq!(
        frames
            .iter()
            .map(|frame| (
                frame
                    .event
                    .as_ref()
                    .map(|event| event.event_type.clone().unwrap()),
                frame.cursor.clone(),
            ))
            .collect::<Vec<_>>(),
        [
            (Some("already-delivered".to_owned()), bytes(100)),
            (None, bytes(101)),
            (Some("after-committed-position".to_owned()), bytes(302)),
        ]
    );
    let state = server.state.lock().unwrap();
    assert!(state.event_requests.is_empty());
    assert_eq!(state.event_subscriptions.len(), 3);
}

/// A live stream that loses its subscription partway through a checkpoint and reconnects to a
/// replica one checkpoint behind yields nothing at or before the last transaction it delivered:
/// no transaction twice, and no cursor a restart would replay from.
///
/// Frames follow a fullnode's subscription: a transaction in checkpoint `n` claims `n - 1`, and
/// a progress-only frame at the start of checkpoint `n + 1` claims `n`.
///
/// - The first subscription starts at the start of checkpoint 11 (cursor 400), delivers its
///   transactions 0 and 1 (cursors 401 and 402, positions `(11, 1)` and `(11, 2)`), and fails.
/// - The replacement replica has indexed only checkpoint 10. Its subscription starts at `(11, 0)`,
///   behind committed position `(11, 2)`, so it needs no List, and then sends all of checkpoint
///   11; only what lies past `(11, 2)` is yielded.
#[tokio::test]
async fn reconnect_to_a_replica_behind_the_last_transaction_yields_nothing_at_or_before_it() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10))]);
    server.push_transaction_subscriptions([
        StreamScript::frames([
            Ok(transaction_positioned_live_frame(None, 400, 10)),
            Ok(transaction_positioned_live_frame(
                Some(transaction_at(11, 0, "tx-11-0")),
                401,
                10,
            )),
            Ok(transaction_positioned_live_frame(
                Some(transaction_at(11, 1, "tx-11-1")),
                402,
                10,
            )),
            Err(tonic::Status::unavailable("replica restarted")),
        ]),
        StreamScript::frames([
            Ok(transaction_positioned_live_frame(None, 400, 10)),
            Ok(transaction_positioned_live_frame(
                Some(transaction_at(11, 0, "tx-11-0")),
                401,
                10,
            )),
            Ok(transaction_positioned_live_frame(
                Some(transaction_at(11, 1, "tx-11-1")),
                402,
                10,
            )),
            Ok(transaction_positioned_live_frame(
                Some(transaction_at(11, 2, "tx-11-2")),
                403,
                10,
            )),
            Ok(transaction_positioned_live_frame(None, 500, 11)),
            Ok(transaction_positioned_live_frame(
                Some(transaction_at(12, 0, "tx-12-0")),
                501,
                11,
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let frames = tokio::time::timeout(
        Duration::from_secs(5),
        client
            .stream_transactions_with_config(
                TransactionStreamRequest::new().with_read_mask(transaction_identity_mask()),
                fast_config(),
            )
            .take(6)
            .map_ok(|frame| {
                (
                    frame.transaction.and_then(|transaction| transaction.digest),
                    frame.cursor,
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
            (None, bytes(400), Some(10)),
            (digest("tx-11-0"), bytes(401), Some(10)),
            (digest("tx-11-1"), bytes(402), Some(10)),
            (digest("tx-11-2"), bytes(403), Some(10)),
            (None, bytes(500), Some(11)),
            (digest("tx-12-0"), bytes(501), Some(11)),
        ]
    );
    let state = server.state.lock().unwrap();
    assert!(state.transaction_requests.is_empty());
    assert_eq!(
        state.calls,
        [
            "subscribe_transactions",
            "get_service_info",
            "subscribe_transactions"
        ]
    );
}

/// Collects `(digest, cursor)` for the first `count` transaction frames of `stream`.
async fn collect_transaction_frames(
    stream: impl futures::Stream<Item = Result<TransactionStreamFrame, tonic::Status>>,
    count: usize,
) -> Vec<(Option<String>, Bytes)> {
    tokio::time::timeout(
        Duration::from_secs(2),
        stream
            .take(count)
            .map_ok(|frame| {
                (
                    frame.transaction.and_then(|transaction| transaction.digest),
                    frame.cursor,
                )
            })
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("transaction stream stalled")
    .unwrap()
}

/// Two servers name one boundary with different tuples, `(10, 3)` and `(11, 0)`, and no
/// transaction is repeated or lost across the reconnect.
///
/// Server A ends checkpoint 10, which holds three transactions, at `(10, 3)` and claims coverage
/// of checkpoint 10. Positions are only ordered, so `(10, 3)` does not pass the start of checkpoint
/// 11: with the tip at checkpoint 10, the reconnect lists `[(10, 3), 11)`, which holds no
/// transaction, before resubscribing. Server B's first frame names the boundary that List reached
/// as `(11, 0)`, so it needs no gap replay, and checkpoint 11 is delivered once.
#[tokio::test]
async fn reconnect_across_two_tuples_for_one_boundary_delivers_each_transaction_once() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(10))]);
    server.push_transaction_subscriptions([
        StreamScript::frames([
            Ok(transaction_positioned_live_frame(
                Some(transaction_at(10, 2, "tx-10-2")),
                302,
                9,
            )),
            Ok(with_position(
                transaction_positioned_live_frame(None, 303, 10),
                transaction_position(10, 3),
            )),
            Err(tonic::Status::unavailable("server A interrupted")),
        ]),
        StreamScript::frames([
            // Server B's first frame names the start of checkpoint 11.
            Ok(transaction_positioned_live_frame(None, 500, 10)),
            Ok(transaction_positioned_live_frame(
                Some(transaction_at(11, 0, "tx-11-0")),
                501,
                10,
            )),
            Ok(transaction_positioned_live_frame(
                Some(transaction_at(11, 1, "tx-11-1")),
                502,
                10,
            )),
        ]),
    ]);
    server.push_transaction_lists([StreamScript::frames([Ok(
        transaction_positioned_list_frame(
            None,
            500,
            10,
            Some(proto::QueryEndReason::CheckpointBound),
        ),
    )])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let frames = collect_transaction_frames(
        client.stream_transactions_with_config(
            TransactionStreamRequest::new().with_read_mask(transaction_identity_mask()),
            fast_config(),
        ),
        4,
    )
    .await;
    let digest = |value: &str| Some(value.to_owned());
    assert_eq!(
        frames,
        [
            (digest("tx-10-2"), bytes(302)),
            (None, bytes(500)),
            (digest("tx-11-0"), bytes(501)),
            (digest("tx-11-1"), bytes(502)),
        ]
    );
    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        [
            "subscribe_transactions",
            "get_service_info",
            "list_transactions",
            "subscribe_transactions"
        ]
    );
    let catch_up = &state.transaction_requests[0].body;
    assert_eq!(catch_up.start_checkpoint, Some(10));
    assert_eq!(catch_up.start_transaction_index, Some(3));
    assert_eq!(catch_up.end_checkpoint, Some(11));
    assert_eq!(catch_up.end_transaction_index, None);
    let options = catch_up.options.as_ref().unwrap();
    assert_eq!(options.after, Some(bytes(302)));
    assert_eq!(options.before, None);
}
