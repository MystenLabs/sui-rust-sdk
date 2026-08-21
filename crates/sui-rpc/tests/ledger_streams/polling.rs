use super::support::ScriptedStreamServer;
use super::support::StreamScript;
use super::support::bytes;
use super::support::checkpoint_identity_mask;
use super::support::checkpoint_list_frame;
use super::support::event_identity_mask;
use super::support::event_list_frame;
use super::support::event_live_frame;
use super::support::event_positioned_list_frame;
use super::support::fast_config;
use super::support::fast_list_config;
use super::support::next_scripted_call;
use super::support::observed_client;
use super::support::recording_config;
use super::support::service_info;
use super::support::spawn_ledger_only_server;
use super::support::spawn_server;
use super::support::transaction_identity_mask;
use super::support::transaction_list_frame;
use futures::StreamExt;
use futures::TryStreamExt;
use std::time::Duration;
use sui_rpc::client::CheckpointStreamRequest;
use sui_rpc::client::CheckpointStreamStart;
use sui_rpc::client::Delivery;
use sui_rpc::client::EventStreamRequest;
use sui_rpc::client::EventStreamStart;
use sui_rpc::client::LedgerStreamEvent;
use sui_rpc::client::LedgerStreamFamily;
use sui_rpc::client::LedgerStreamOperation;
use sui_rpc::client::LedgerStreamStage;
use sui_rpc::client::TransactionStreamRequest;
use sui_rpc::client::TransactionStreamStart;
use sui_rpc::proto::sui::rpc::v2 as proto;
use tokio::sync::mpsc;

#[tokio::test]
async fn poll_suppresses_baseline_for_all_families() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos(
        [5, 6, 5, 6, 5, 6]
            .into_iter()
            .map(|height| Ok(service_info(height))),
    );
    server.push_checkpoint_lists([
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(5), 5, None)),
            Ok(checkpoint_list_frame(
                None,
                5,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(6), 6, None)),
            Ok(checkpoint_list_frame(
                None,
                6,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    server.push_transaction_lists([
        StreamScript::frames([
            Ok(transaction_list_frame(Some(5), 5, None)),
            Ok(transaction_list_frame(
                None,
                5,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([
            Ok(transaction_list_frame(Some(6), 6, None)),
            Ok(transaction_list_frame(
                None,
                6,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Ok(event_list_frame(
                None,
                5,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(6), 6, None)),
            Ok(event_list_frame(
                None,
                6,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    let address = spawn_ledger_only_server(server.clone()).await;
    let (client, observations) = observed_client(address);
    let mut config = fast_config();
    config.ledger_tip_poll_interval = Duration::from_millis(1);
    let (config, mut observer_events) = recording_config(config);

    let checkpoint_ids = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_checkpoints_with_config(
                CheckpointStreamRequest::new()
                    .with_read_mask(prost_types::FieldMask {
                        paths: vec!["sequence_number".to_owned()],
                    })
                    .with_filter(proto::TransactionFilter::default())
                    .with_delivery(Delivery::Poll),
                config.clone(),
            )
            .try_filter_map(|frame| async move {
                Ok(frame
                    .checkpoint
                    .and_then(|checkpoint| checkpoint.sequence_number))
            })
            .take(1)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for polled checkpoint")
    .unwrap();
    assert_eq!(checkpoint_ids, [6]);

    let transaction_ids = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_transactions_with_config(
                TransactionStreamRequest::new()
                    .with_read_mask(transaction_identity_mask())
                    .with_filter(proto::TransactionFilter::default())
                    .with_delivery(Delivery::Poll),
                config.clone(),
            )
            .try_filter_map(|frame| async move {
                Ok(frame
                    .transaction
                    .and_then(|transaction| transaction.checkpoint))
            })
            .take(1)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for polled transaction")
    .unwrap();
    assert_eq!(transaction_ids, [6]);

    let event_ids = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .stream_events_with_config(
                EventStreamRequest::new()
                    .with_read_mask(event_identity_mask())
                    .with_filter(proto::EventFilter::default())
                    .with_delivery(Delivery::Poll),
                config,
            )
            .try_filter_map(
                |frame| async move { Ok(frame.event.and_then(|event| event.checkpoint)) },
            )
            .take(1)
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for polled event")
    .unwrap();
    assert_eq!(event_ids, [6]);

    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        [
            "get_service_info",
            "list_checkpoints",
            "get_service_info",
            "list_checkpoints",
            "get_service_info",
            "list_transactions",
            "get_service_info",
            "list_transactions",
            "get_service_info",
            "list_events",
            "get_service_info",
            "list_events",
        ]
    );
    assert_eq!(state.checkpoint_requests[0].body.start_checkpoint, Some(5));
    assert_eq!(state.checkpoint_requests[0].body.end_checkpoint, Some(6));
    assert_eq!(state.transaction_requests[0].body.start_checkpoint, Some(5));
    assert_eq!(state.transaction_requests[0].body.end_checkpoint, Some(6));
    assert_eq!(state.event_requests[0].body.start_checkpoint, Some(5));
    assert_eq!(state.event_requests[0].body.end_checkpoint, Some(6));
    assert_eq!(
        state.checkpoint_requests[0].body.read_mask,
        Some(prost_types::FieldMask {
            paths: vec!["sequence_number".to_owned()],
        })
    );
    assert_eq!(
        state.checkpoint_requests[0].body.filter,
        Some(proto::TransactionFilter::default())
    );
    assert_eq!(
        state.transaction_requests[0].body.read_mask,
        Some(prost_types::FieldMask {
            paths: vec!["checkpoint".to_owned(), "transaction_index".to_owned()],
        })
    );
    assert_eq!(
        state.transaction_requests[0].body.filter,
        Some(proto::TransactionFilter::default())
    );
    assert_eq!(
        state.event_requests[0].body.read_mask,
        Some(prost_types::FieldMask {
            paths: vec![
                "checkpoint".to_owned(),
                "transaction_index".to_owned(),
                "event_index".to_owned(),
            ],
        })
    );
    assert_eq!(
        state.event_requests[0].body.filter,
        Some(proto::EventFilter::default())
    );
    for options in [
        state.checkpoint_requests[0].body.options.as_ref().unwrap(),
        state.transaction_requests[0].body.options.as_ref().unwrap(),
        state.event_requests[0].body.options.as_ref().unwrap(),
    ] {
        assert!(options.limit.is_none());
        assert!(options.ordering.is_none());
    }
    drop(state);

    let observations = observations.lock().unwrap();
    let rpc_names = observations
        .iter()
        .map(|observation| observation.path.rsplit('/').next().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        rpc_names,
        [
            "GetServiceInfo",
            "ListCheckpoints",
            "GetServiceInfo",
            "ListCheckpoints",
            "GetServiceInfo",
            "ListTransactions",
            "GetServiceInfo",
            "ListTransactions",
            "GetServiceInfo",
            "ListEvents",
            "GetServiceInfo",
            "ListEvents",
        ]
    );
    assert!(
        observations
            .iter()
            .all(|observation| !observation.path.contains("Subscribe"))
    );
    drop(observations);

    let mut events = Vec::new();
    while let Ok(event) = observer_events.try_recv() {
        events.push(event);
    }
    let families = [
        LedgerStreamFamily::Checkpoint,
        LedgerStreamFamily::Transaction,
        LedgerStreamFamily::Event,
    ];
    assert_eq!(events.len(), families.len() * 4);
    for (family_events, family) in events.chunks_exact(4).zip(families) {
        assert!(matches!(
            &family_events[0],
            LedgerStreamEvent::RpcResponse {
                family: observed_family,
                operation: LedgerStreamOperation::GetServiceInfo,
                stage: LedgerStreamStage::PollingBaseline,
                code: tonic::Code::Ok,
                ..
            } if *observed_family == family
        ));
        assert!(matches!(
            &family_events[1],
            LedgerStreamEvent::RpcResponse {
                family: observed_family,
                operation: LedgerStreamOperation::List,
                stage: LedgerStreamStage::PollingBaseline,
                code: tonic::Code::Ok,
                ..
            } if *observed_family == family
        ));
        assert!(matches!(
            &family_events[2],
            LedgerStreamEvent::RpcResponse {
                family: observed_family,
                operation: LedgerStreamOperation::GetServiceInfo,
                stage: LedgerStreamStage::PollingTail,
                code: tonic::Code::Ok,
                ..
            } if *observed_family == family
        ));
        assert!(matches!(
            &family_events[3],
            LedgerStreamEvent::RpcResponse {
                family: observed_family,
                operation: LedgerStreamOperation::List,
                stage: LedgerStreamStage::PollingTail,
                code: tonic::Code::Ok,
                ..
            } if *observed_family == family
        ));
    }
}

#[tokio::test]
async fn tip_poll_and_tip_subscribe_share_the_frame_contract() {
    let (subscribe_server, _calls) = ScriptedStreamServer::new();
    subscribe_server.push_event_subscriptions([StreamScript::frames([
        Ok(event_live_frame(None, 5)),
        Ok(event_live_frame(Some(6), 6)),
    ])]);
    let address = spawn_server(subscribe_server).await;
    let (subscribe_client, _observations) = observed_client(address);
    let subscribe_frames = subscribe_client
        .stream_events(EventStreamRequest::new().with_read_mask(event_identity_mask()))
        .take(2)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    let (poll_server, _calls) = ScriptedStreamServer::new();
    poll_server.push_service_infos([
        Ok(service_info(5)),
        Ok(service_info(6)),
        Ok(service_info(7)),
    ]);
    poll_server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Ok(event_list_frame(
                None,
                5,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(6), 6, None)),
            Ok(event_list_frame(
                None,
                6,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            7,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
    ]);
    let address = spawn_ledger_only_server(poll_server).await;
    let (poll_client, _observations) = observed_client(address);
    let mut config = fast_config();
    config.ledger_tip_poll_interval = Duration::from_millis(1);
    let poll_frames = poll_client
        .stream_events_with_config(
            EventStreamRequest::new()
                .with_read_mask(event_identity_mask())
                .with_delivery(Delivery::Poll),
            config,
        )
        .take(2)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    // Both deliveries use the same resumable frame contract, but Poll suppresses its tip baseline,
    // so frame ordering may differ.
    for frames in [&subscribe_frames, &poll_frames] {
        assert_eq!(frames.len(), 2);
        for frame in frames.iter() {
            assert!(!frame.cursor.is_empty());
        }
        let item_checkpoints: Vec<_> = frames
            .iter()
            .filter_map(|frame| frame.event.as_ref())
            .map(|event| event.checkpoint)
            .collect();
        assert_eq!(item_checkpoints, [Some(6)]);
    }
}

#[tokio::test]
async fn tip_poll_rejects_maximum_service_checkpoint_before_list() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(u64::MAX))]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let stream = client.stream_checkpoints(
        CheckpointStreamRequest::new()
            .with_read_mask(checkpoint_identity_mask())
            .with_delivery(Delivery::Poll),
    );
    futures::pin_mut!(stream);
    let status = tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .expect("timed out waiting for live-tip overflow")
        .unwrap()
        .unwrap_err();
    assert_eq!(status.code(), tonic::Code::OutOfRange);
    assert_eq!(
        status.message(),
        "checkpoint height cannot be converted to an exclusive end bound"
    );
    assert_eq!(server.state.lock().unwrap().calls, ["get_service_info"]);
}

#[tokio::test]
async fn in_body_unimplemented_is_terminal() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_subscriptions([StreamScript::frames([Err(tonic::Status::unimplemented(
        "subscription body unavailable",
    ))])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let stream =
        client.stream_events(EventStreamRequest::new().with_read_mask(event_identity_mask()));
    futures::pin_mut!(stream);
    let status = tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .expect("timed out waiting for in-body Unimplemented")
        .unwrap()
        .unwrap_err();
    assert_eq!(status.code(), tonic::Code::Unimplemented);
    assert_eq!(server.state.lock().unwrap().calls, ["subscribe_events"]);
}

#[tokio::test]
async fn future_lower_bound_polls_service_info_before_listing() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([
        Ok(service_info(5)),
        Ok(service_info(99)),
        Ok(service_info(100)),
    ]);
    server.push_event_lists([StreamScript::frames([
        Ok(event_list_frame(Some(100), 100, None)),
        Ok(event_list_frame(
            None,
            100,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    let (live_tx, live_rx) = mpsc::unbounded_channel();
    live_tx.send(Ok(event_live_frame(None, 100))).unwrap();
    live_tx.send(Ok(event_live_frame(Some(101), 101))).unwrap();
    server.push_event_subscriptions([StreamScript::Channel(live_rx)]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let mut config = fast_config();
    config.ledger_tip_poll_interval = Duration::from_millis(10);

    let body = EventStreamRequest::new()
        .with_read_mask(event_identity_mask())
        .with_start(EventStreamStart::Checkpoint(100));
    let collector = tokio::spawn(async move {
        client
            .stream_events_with_config(body, config)
            .try_filter_map(|frame| async move {
                Ok(frame.event.map(|event| event.checkpoint.unwrap()))
            })
            .take(2)
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(server.state.lock().unwrap().event_requests.len(), 0);
    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(server.state.lock().unwrap().event_requests.len(), 0);
    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    assert_eq!(next_scripted_call(&mut calls).await, "subscribe_events");
    assert_eq!(collector.await.unwrap().unwrap(), [100, 101]);

    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        [
            "get_service_info",
            "get_service_info",
            "get_service_info",
            "list_events",
            "subscribe_events"
        ]
    );
    assert_eq!(state.event_requests.len(), 1);
    assert_eq!(state.event_requests[0].body.start_checkpoint, Some(100));
    assert_eq!(state.event_requests[0].body.end_checkpoint, Some(101));
}

#[tokio::test]
async fn list_page_limit_applies_to_internal_lists_only() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos(
        [2, 2, 2, 4, 2, 2, 2, 4, 2, 2, 2, 4]
            .into_iter()
            .map(|height| Ok(service_info(height))),
    );
    server.push_checkpoint_lists([
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(1), 1, None)),
            Ok(checkpoint_list_frame(Some(2), 2, None)),
            Ok(checkpoint_list_frame(
                None,
                2,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(3), 3, None)),
            Ok(checkpoint_list_frame(Some(4), 4, None)),
            Ok(checkpoint_list_frame(
                None,
                4,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    server.push_transaction_lists([
        StreamScript::frames([
            Ok(transaction_list_frame(Some(1), 1, None)),
            Ok(transaction_list_frame(Some(2), 2, None)),
            Ok(transaction_list_frame(
                None,
                2,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([
            Ok(transaction_list_frame(Some(3), 3, None)),
            Ok(transaction_list_frame(Some(4), 4, None)),
            Ok(transaction_list_frame(
                None,
                4,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(1), 1, None)),
            Ok(event_list_frame(Some(2), 2, None)),
            Ok(event_list_frame(
                None,
                2,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(3), 3, None)),
            Ok(event_list_frame(Some(4), 4, None)),
            Ok(event_list_frame(
                None,
                4,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            4,
            Some(proto::QueryEndReason::LedgerTip),
        ))]),
    ]);
    let address = spawn_ledger_only_server(server.clone()).await;
    let (client, observations) = observed_client(address);
    let mut config = fast_config();
    config.ledger_tip_poll_interval = Duration::from_millis(10);
    config.list_page_limit = Some(2);

    let checkpoint_request = CheckpointStreamRequest::new()
        .with_read_mask(checkpoint_identity_mask())
        .with_start(CheckpointStreamStart::Checkpoint(1))
        .with_delivery(Delivery::Poll);
    let checkpoint_client = client.clone();
    let checkpoint_config = config.clone();
    let checkpoints = tokio::spawn(async move {
        checkpoint_client
            .stream_checkpoints_with_config(checkpoint_request, checkpoint_config)
            .try_filter_map(|frame| async move {
                Ok(frame
                    .checkpoint
                    .map(|checkpoint| checkpoint.sequence_number.unwrap()))
            })
            .take(4)
            .try_collect::<Vec<_>>()
            .await
    });
    assert_eq!(checkpoints.await.unwrap().unwrap(), [1, 2, 3, 4]);

    let transaction_request = TransactionStreamRequest::new()
        .with_read_mask(transaction_identity_mask())
        .with_start(TransactionStreamStart::Checkpoint(1))
        .with_delivery(Delivery::Poll);
    let transaction_client = client.clone();
    let transaction_config = config.clone();
    let transactions = tokio::spawn(async move {
        transaction_client
            .stream_transactions_with_config(transaction_request, transaction_config)
            .try_filter_map(|frame| async move {
                Ok(frame
                    .transaction
                    .map(|transaction| transaction.checkpoint.unwrap()))
            })
            .take(4)
            .try_collect::<Vec<_>>()
            .await
    });
    assert_eq!(transactions.await.unwrap().unwrap(), [1, 2, 3, 4]);

    let event_request = EventStreamRequest::new()
        .with_read_mask(event_identity_mask())
        .with_start(EventStreamStart::Checkpoint(1))
        .with_delivery(Delivery::Poll);
    let event_client = client.clone();
    let event_config = config.clone();
    let events = tokio::spawn(async move {
        event_client
            .stream_events_with_config(event_request, event_config)
            .try_filter_map(|frame| async move {
                Ok(frame.event.map(|event| event.checkpoint.unwrap()))
            })
            .take(4)
            .try_collect::<Vec<_>>()
            .await
    });
    assert_eq!(events.await.unwrap().unwrap(), [1, 2, 3, 4]);

    client
        .list_events_with_config(
            proto::ListEventsRequest::default()
                .with_options(proto::QueryOptions::default().with_limit(7)),
            fast_list_config(),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    let state = server.state.lock().unwrap();
    assert_eq!(state.checkpoint_requests.len(), 2);
    assert_eq!(state.transaction_requests.len(), 2);
    assert_eq!(state.event_requests.len(), 3);
    assert_eq!(state.service_info_requests.len(), 12);
    for request in [
        &state.checkpoint_requests[0].body,
        &state.checkpoint_requests[1].body,
    ] {
        assert_eq!(request.options.as_ref().unwrap().limit, Some(2));
    }
    for request in [
        &state.transaction_requests[0].body,
        &state.transaction_requests[1].body,
    ] {
        assert_eq!(request.options.as_ref().unwrap().limit, Some(2));
    }
    for request in [&state.event_requests[0].body, &state.event_requests[1].body] {
        assert_eq!(request.options.as_ref().unwrap().limit, Some(2));
    }
    assert_eq!(
        state.event_requests[2].body.options.as_ref().unwrap().limit,
        Some(7)
    );
    assert_eq!(state.checkpoint_requests[1].body.start_checkpoint, Some(3));
    assert_eq!(state.checkpoint_requests[1].body.end_checkpoint, Some(5));
    assert_eq!(
        state.transaction_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .after,
        Some(bytes(2))
    );
    assert_eq!(state.transaction_requests[1].body.end_checkpoint, Some(5));
    assert_eq!(
        state.event_requests[1].body.options.as_ref().unwrap().after,
        Some(bytes(2))
    );
    assert_eq!(state.event_requests[1].body.end_checkpoint, Some(5));
    assert!(state.checkpoint_subscriptions.is_empty());
    assert!(state.transaction_subscriptions.is_empty());
    assert!(state.event_subscriptions.is_empty());
    assert!(
        observations
            .lock()
            .unwrap()
            .iter()
            .all(|observation| !observation.path.contains("Subscribe"))
    );
}

#[tokio::test]
async fn dispatch_unimplemented_is_terminal_for_ledger_streams() {
    let (checkpoint_server, _calls) = ScriptedStreamServer::new();
    checkpoint_server.push_service_infos([Ok(service_info(2))]);
    checkpoint_server.push_checkpoint_lists([StreamScript::frames([
        Ok(checkpoint_list_frame(Some(2), 2, None)),
        Ok(checkpoint_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    checkpoint_server.push_checkpoint_subscriptions([StreamScript::DispatchError(
        tonic::Status::unimplemented("checkpoint subscription unavailable"),
    )]);
    let address = spawn_server(checkpoint_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let (config, mut checkpoint_events) = recording_config(fast_config());
    let stream = client.stream_checkpoints_with_config(
        CheckpointStreamRequest::new()
            .with_read_mask(checkpoint_identity_mask())
            .with_start(CheckpointStreamStart::Checkpoint(0)),
        config,
    );
    futures::pin_mut!(stream);
    assert_eq!(
        stream
            .next()
            .await
            .unwrap()
            .unwrap()
            .checkpoint
            .unwrap()
            .sequence_number,
        Some(2)
    );
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::Unimplemented);
    assert_eq!(status.message(), "checkpoint subscription unavailable");
    assert!(stream.next().await.is_none());
    {
        let state = checkpoint_server.state.lock().unwrap();
        assert_eq!(
            state.calls,
            [
                "get_service_info",
                "list_checkpoints",
                "subscribe_checkpoints"
            ]
        );
        assert_eq!(state.service_info_requests.len(), 1);
        assert_eq!(state.checkpoint_requests.len(), 1);
    }
    let mut events = Vec::new();
    while let Ok(event) = checkpoint_events.try_recv() {
        events.push(event);
    }
    assert_eq!(events.len(), 4);
    assert!(matches!(
        &events[0],
        LedgerStreamEvent::RpcResponse {
            family: LedgerStreamFamily::Checkpoint,
            operation: LedgerStreamOperation::GetServiceInfo,
            stage: LedgerStreamStage::InitialReplay,
            code: tonic::Code::Ok,
            ..
        }
    ));
    assert!(matches!(
        &events[1],
        LedgerStreamEvent::RpcResponse {
            family: LedgerStreamFamily::Checkpoint,
            operation: LedgerStreamOperation::List,
            stage: LedgerStreamStage::InitialReplay,
            code: tonic::Code::Ok,
            ..
        }
    ));
    assert!(matches!(
        &events[2],
        LedgerStreamEvent::RpcResponse {
            family: LedgerStreamFamily::Checkpoint,
            operation: LedgerStreamOperation::Subscribe,
            stage: LedgerStreamStage::LiveSubscription,
            code: tonic::Code::Unimplemented,
            ..
        }
    ));
    assert!(matches!(
        &events[3],
        LedgerStreamEvent::TerminalError {
            family: LedgerStreamFamily::Checkpoint,
            status,
            ..
        } if status.code() == tonic::Code::Unimplemented
            && status.message() == "checkpoint subscription unavailable"
    ));

    let (event_server, _calls) = ScriptedStreamServer::new();
    event_server.push_event_subscriptions([StreamScript::DispatchError(
        tonic::Status::unimplemented("event subscription unavailable"),
    )]);
    let address = spawn_server(event_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let (config, mut event_events) = recording_config(fast_config());
    let stream = client.stream_events_with_config(
        EventStreamRequest::new().with_read_mask(event_identity_mask()),
        config,
    );
    futures::pin_mut!(stream);
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::Unimplemented);
    assert_eq!(status.message(), "event subscription unavailable");
    assert!(stream.next().await.is_none());
    {
        let state = event_server.state.lock().unwrap();
        assert_eq!(state.calls, ["subscribe_events"]);
        assert!(state.service_info_requests.is_empty());
        assert!(state.event_requests.is_empty());
    }
    let mut events = Vec::new();
    while let Ok(event) = event_events.try_recv() {
        events.push(event);
    }
    assert_eq!(events.len(), 2);
    assert!(matches!(
        &events[0],
        LedgerStreamEvent::RpcResponse {
            family: LedgerStreamFamily::Event,
            operation: LedgerStreamOperation::Subscribe,
            stage: LedgerStreamStage::LiveTipStartup,
            code: tonic::Code::Unimplemented,
            ..
        }
    ));
    assert!(matches!(
        &events[1],
        LedgerStreamEvent::TerminalError {
            family: LedgerStreamFamily::Event,
            status,
            ..
        } if status.code() == tonic::Code::Unimplemented
            && status.message() == "event subscription unavailable"
    ));

    let (permission_server, _calls) = ScriptedStreamServer::new();
    permission_server.push_service_infos([Ok(service_info(2))]);
    permission_server.push_checkpoint_lists([StreamScript::frames([
        Ok(checkpoint_list_frame(Some(2), 2, None)),
        Ok(checkpoint_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    permission_server.push_checkpoint_subscriptions([StreamScript::DispatchError(
        tonic::Status::permission_denied("subscription denied"),
    )]);
    let address = spawn_server(permission_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let (config, mut permission_events) = recording_config(fast_config());
    let stream = client.stream_checkpoints_with_config(
        CheckpointStreamRequest::new()
            .with_read_mask(checkpoint_identity_mask())
            .with_start(CheckpointStreamStart::Checkpoint(0)),
        config,
    );
    futures::pin_mut!(stream);
    assert_eq!(
        stream
            .next()
            .await
            .unwrap()
            .unwrap()
            .checkpoint
            .unwrap()
            .sequence_number,
        Some(2)
    );
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::PermissionDenied);
    assert_eq!(
        permission_server.state.lock().unwrap().calls,
        [
            "get_service_info",
            "list_checkpoints",
            "subscribe_checkpoints"
        ]
    );
    let mut events = Vec::new();
    while let Ok(event) = permission_events.try_recv() {
        events.push(event);
    }
    assert_eq!(events.len(), 4);
    assert!(matches!(
        &events[0],
        LedgerStreamEvent::RpcResponse {
            family: LedgerStreamFamily::Checkpoint,
            operation: LedgerStreamOperation::GetServiceInfo,
            stage: LedgerStreamStage::InitialReplay,
            code: tonic::Code::Ok,
            ..
        }
    ));
    assert!(matches!(
        &events[1],
        LedgerStreamEvent::RpcResponse {
            family: LedgerStreamFamily::Checkpoint,
            operation: LedgerStreamOperation::List,
            stage: LedgerStreamStage::InitialReplay,
            code: tonic::Code::Ok,
            ..
        }
    ));
    assert!(matches!(
        &events[2],
        LedgerStreamEvent::RpcResponse {
            family: LedgerStreamFamily::Checkpoint,
            operation: LedgerStreamOperation::Subscribe,
            stage: LedgerStreamStage::LiveSubscription,
            code: tonic::Code::PermissionDenied,
            ..
        }
    ));
    assert!(matches!(
        &events[3],
        LedgerStreamEvent::TerminalError {
            family: LedgerStreamFamily::Checkpoint,
            status,
            ..
        } if status.code() == tonic::Code::PermissionDenied
            && status.message() == "subscription denied"
    ));
}

#[tokio::test]
async fn service_info_retry_and_validation() {
    let (retry_server, _calls) = ScriptedStreamServer::new();
    retry_server.push_service_infos([
        Err(tonic::Status::unavailable("service-info first")),
        Err(tonic::Status::aborted("service-info second")),
        Ok(service_info(2)),
    ]);
    retry_server.push_checkpoint_lists([
        StreamScript::DispatchError(tonic::Status::unavailable("List first")),
        StreamScript::DispatchError(tonic::Status::aborted("List second")),
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(2), 2, None)),
            Ok(checkpoint_list_frame(
                None,
                2,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    let address = spawn_server(retry_server.clone()).await;
    let (client, observations) = observed_client(address);
    let stream = client.stream_checkpoints_with_config(
        CheckpointStreamRequest::new()
            .with_read_mask(checkpoint_identity_mask())
            .with_start(CheckpointStreamStart::Checkpoint(0)),
        fast_config(),
    );
    futures::pin_mut!(stream);
    let frame = stream.next().await.unwrap().unwrap();
    assert_eq!(frame.checkpoint.unwrap().sequence_number, Some(2));
    {
        let state = retry_server.state.lock().unwrap();
        assert_eq!(state.service_info_requests.len(), 3);
        assert_eq!(state.checkpoint_requests.len(), 3);
    }
    let get_service_info_observations = observations
        .lock()
        .unwrap()
        .iter()
        .filter(|observation| observation.path.contains("GetServiceInfo"))
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(get_service_info_observations.len(), 3);

    let (missing_server, _calls) = ScriptedStreamServer::new();
    missing_server.push_service_infos([Ok(proto::GetServiceInfoResponse::default())]);
    let address = spawn_server(missing_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let stream = client.stream_events_with_config(
        EventStreamRequest::new()
            .with_read_mask(event_identity_mask())
            .with_start(EventStreamStart::Checkpoint(0)),
        fast_config(),
    );
    futures::pin_mut!(stream);
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(
        status.message(),
        "GetServiceInfo response is missing checkpoint_height"
    );
    assert_eq!(
        missing_server.state.lock().unwrap().calls,
        ["get_service_info"]
    );

    let (overflow_server, _calls) = ScriptedStreamServer::new();
    overflow_server.push_service_infos([Ok(service_info(u64::MAX))]);
    let address = spawn_server(overflow_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let stream = client.stream_events_with_config(
        EventStreamRequest::new()
            .with_read_mask(event_identity_mask())
            .with_start(EventStreamStart::Checkpoint(0)),
        fast_config(),
    );
    futures::pin_mut!(stream);
    let status = stream.next().await.unwrap().unwrap_err();
    assert_eq!(status.code(), tonic::Code::OutOfRange);
    assert_eq!(
        status.message(),
        "checkpoint height cannot be converted to an exclusive end bound"
    );
    assert_eq!(
        overflow_server.state.lock().unwrap().calls,
        ["get_service_info"]
    );

    async fn assert_zero_interval<T: std::fmt::Debug>(
        stream: impl futures::Stream<Item = Result<T, tonic::Status>>,
    ) {
        futures::pin_mut!(stream);
        let status = stream.next().await.unwrap().unwrap_err();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert_eq!(
            status.message(),
            "ledger_tip_poll_interval must be greater than zero"
        );
    }

    let (invalid_config_server, _calls) = ScriptedStreamServer::new();
    let address = spawn_server(invalid_config_server.clone()).await;
    let (client, _observations) = observed_client(address);
    let mut config = fast_config();
    config.ledger_tip_poll_interval = Duration::ZERO;
    assert_zero_interval(
        client.stream_transactions_with_config(
            TransactionStreamRequest::new()
                .with_read_mask(transaction_identity_mask())
                .with_start(TransactionStreamStart::Checkpoint(0)),
            config.clone(),
        ),
    )
    .await;
    assert_zero_interval(client.stream_checkpoints_with_config(
        CheckpointStreamRequest::new().with_read_mask(checkpoint_identity_mask()),
        config,
    ))
    .await;
    assert!(invalid_config_server.state.lock().unwrap().calls.is_empty());
}

#[tokio::test]
async fn dropping_during_tip_poll_sleep_prevents_later_rpc() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(2)), Ok(service_info(3))]);
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(2), 2, None)),
            Ok(event_list_frame(
                None,
                2,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let collector = tokio::spawn(async move {
        let stream = client.stream_events_with_config(
            EventStreamRequest::new()
                .with_read_mask(event_identity_mask())
                .with_delivery(Delivery::Poll),
            {
                let mut config = fast_config();
                config.ledger_tip_poll_interval = Duration::from_millis(50);
                config
            },
        );
        futures::pin_mut!(stream);
        assert!(stream.next().await.unwrap().is_ok());
        stream.next().await
    });
    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    tokio::task::yield_now().await;
    collector.abort();
    assert!(collector.await.unwrap_err().is_cancelled());
    tokio::time::sleep(Duration::from_millis(100)).await;
    let state = server.state.lock().unwrap();
    assert_eq!(state.service_info_requests.len(), 2);
    assert_eq!(state.event_requests.len(), 2);
    assert!(state.event_subscriptions.is_empty());
}

#[tokio::test]
async fn ledger_tip_polling_redispatches_until_the_fixed_bound() {
    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(9))]);
    let (bound_tx, bound_rx) = mpsc::unbounded_channel();
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            4,
            Some(proto::QueryEndReason::LedgerTip),
        ))]),
        StreamScript::Channel(bound_rx),
    ]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);
    let collector = tokio::spawn(async move {
        client
            .stream_events_with_config(
                EventStreamRequest::new()
                    .with_read_mask(event_identity_mask())
                    .with_start(EventStreamStart::Checkpoint(0)),
                fast_config(),
            )
            .take(2)
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(next_scripted_call(&mut calls).await, "get_service_info");
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    assert_eq!(next_scripted_call(&mut calls).await, "list_events");
    assert!(!collector.is_finished());
    bound_tx
        .send(Ok(event_positioned_list_frame(
            None,
            10,
            9,
            Some(proto::QueryEndReason::CheckpointBound),
        )))
        .unwrap();
    let frames = collector.await.unwrap().unwrap();
    assert!(frames.iter().all(|frame| frame.event.is_none()));
    assert_eq!(
        frames
            .into_iter()
            .map(|frame| frame.cursor)
            .collect::<Vec<_>>(),
        [bytes(4), bytes(10)]
    );
}

#[tokio::test]
async fn dispatch_internal_stays_terminal() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_subscriptions([
        StreamScript::DispatchError(tonic::Status::internal("dispatch failed")),
        StreamScript::frames([Ok(event_live_frame(None, 30))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let stream = client.stream_events_with_config(
        EventStreamRequest::new().with_read_mask(event_identity_mask()),
        fast_config(),
    );
    futures::pin_mut!(stream);

    let status = tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .expect("timed out waiting for terminal dispatch failure")
        .unwrap()
        .unwrap_err();

    assert_eq!(status.code(), tonic::Code::Internal);
    assert_eq!(status.message(), "dispatch failed");
    assert!(stream.next().await.is_none());
    let state = server.state.lock().unwrap();
    assert_eq!(state.calls, ["subscribe_events"]);
    assert!(state.service_info_requests.is_empty());
    assert!(state.event_requests.is_empty());
}

#[tokio::test]
async fn persisted_positions_resume_with_poll_delivery() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([Ok(service_info(2)), Ok(service_info(4))]);
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(2), 2, None)),
            Ok(event_list_frame(
                None,
                2,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(3), 3, None)),
            Ok(event_list_frame(Some(4), 4, None)),
            Ok(event_list_frame(
                None,
                4,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, observations) = observed_client(address);

    let first_frames = client
        .stream_events(
            EventStreamRequest::new()
                .with_read_mask(event_identity_mask())
                .with_start(EventStreamStart::Checkpoint(2))
                .with_delivery(Delivery::Poll),
        )
        .take(1)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(first_frames.len(), 1);
    assert_eq!(first_frames[0].event.as_ref().unwrap().checkpoint, Some(2));
    assert_eq!(first_frames[0].cursor, bytes(2));

    let resume_request = EventStreamRequest::new()
        .with_read_mask(event_identity_mask())
        .with_start(EventStreamStart::Resume(first_frames[0].cursor.clone()))
        .with_delivery(Delivery::Poll);
    let resumed = client
        .stream_events(resume_request)
        .take(2)
        .try_filter_map(|frame| async move { Ok(frame.event.and_then(|event| event.checkpoint)) })
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(resumed, [3, 4]);

    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        [
            "get_service_info",
            "list_events",
            "get_service_info",
            "list_events"
        ]
    );
    assert_eq!(
        state.event_requests[1].body.options.as_ref().unwrap().after,
        Some(bytes(2))
    );
    assert!(state.event_subscriptions.is_empty());
    drop(state);
    assert!(
        observations
            .lock()
            .unwrap()
            .iter()
            .all(|observation| !observation.path.contains("Subscribe"))
    );
}
