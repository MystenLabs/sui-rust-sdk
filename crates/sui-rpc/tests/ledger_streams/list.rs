use super::support::STREAM_DROP_TIMEOUT;
use super::support::ScriptedStreamServer;
use super::support::StreamScript;
use super::support::bounded_checkpoint_scripts;
use super::support::bounded_event_request;
use super::support::bounded_event_scripts;
use super::support::bounded_transaction_scripts;
use super::support::bytes;
use super::support::checkpoint_identity_mask;
use super::support::checkpoint_list_frame;
use super::support::event_identity_mask;
use super::support::event_list_frame;
use super::support::event_positioned_list_frame;
use super::support::fast_list_config;
use super::support::observed_client;
use super::support::service_info;
use super::support::spawn_server;
use super::support::transaction_identity_mask;
use super::support::transaction_list_frame;
use futures::StreamExt;
use futures::TryStreamExt;
use std::time::Duration;
use sui_rpc::client::CheckpointStreamRequest;
use sui_rpc::client::CheckpointStreamStart;
use sui_rpc::client::EventStreamRequest;
use sui_rpc::client::EventStreamStart;
use sui_rpc::client::ListConfig;
use sui_rpc::client::TransactionStreamRequest;
use sui_rpc::client::TransactionStreamStart;
use sui_rpc::proto::sui::rpc::v2 as proto;
use tokio::sync::mpsc;

#[tokio::test]
async fn list_observer_events_receive_list_events() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        5,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);
    let (observer_tx, mut observer_events) = mpsc::unbounded_channel();
    let config = fast_list_config().with_observer(move |event| {
        observer_tx.send(event).unwrap();
    });

    client
        .list_events_with_config(bounded_event_request().with_end_checkpoint(6), config)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    match observer_events.try_recv().unwrap() {
        sui_rpc::client::ListEvent::RpcResponse { family, code, .. } => {
            assert_eq!(family, sui_rpc::client::LedgerStreamFamily::Event);
            assert_eq!(code, tonic::Code::Ok);
        }
        event => panic!("unexpected observer event: {event:?}"),
    }
}

#[tokio::test]
async fn lists_continue_pagination_then_stop_at_ledger_tip_for_all_families() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_checkpoint_lists(bounded_checkpoint_scripts());
    server.push_transaction_lists(bounded_transaction_scripts());
    server.push_event_lists(bounded_event_scripts());
    let address = spawn_server(server.clone()).await;
    let (client, observations) = observed_client(address);

    let options = proto::QueryOptions::default()
        .with_limit(2)
        .with_ordering(proto::Ordering::Ascending);
    let checkpoint_request = proto::ListCheckpointsRequest::default()
        .with_read_mask(prost_types::FieldMask {
            paths: vec!["digest".to_owned(), "sequence_number".to_owned()],
        })
        .with_filter(proto::TransactionFilter::default())
        .with_start_checkpoint(2)
        .with_end_checkpoint(8)
        .with_options(options.clone());
    let checkpoint_responses = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .list_checkpoints_with_config(checkpoint_request, fast_list_config())
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for paginated checkpoints")
    .unwrap();
    assert_eq!(
        checkpoint_responses
            .iter()
            .filter_map(|response| response.checkpoint.as_ref())
            .count(),
        4
    );
    assert!(
        checkpoint_responses
            .iter()
            .filter_map(|response| response.checkpoint.as_ref())
            .all(|checkpoint| checkpoint.sequence_number.is_some())
    );

    let transaction_request = proto::ListTransactionsRequest::default()
        .with_read_mask(prost_types::FieldMask {
            paths: vec![
                "digest".to_owned(),
                "checkpoint".to_owned(),
                "transaction_index".to_owned(),
            ],
        })
        .with_filter(proto::TransactionFilter::default())
        .with_start_checkpoint(2)
        .with_end_checkpoint(8)
        .with_options(options.clone());
    let transaction_responses = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .list_transactions_with_config(transaction_request, fast_list_config())
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for paginated transactions")
    .unwrap();
    assert_eq!(
        transaction_responses
            .iter()
            .filter_map(|response| response.transaction.as_ref())
            .map(|transaction| transaction.digest.as_deref().unwrap())
            .collect::<Vec<_>>(),
        ["tx-2", "tx-3", "tx-6", "tx-7"]
    );
    assert!(
        transaction_responses
            .iter()
            .filter_map(|response| response.transaction.as_ref())
            .all(|transaction| {
                transaction.checkpoint.is_some() && transaction.transaction_index.is_some()
            })
    );

    let event_request = proto::ListEventsRequest::default()
        .with_read_mask(prost_types::FieldMask {
            paths: vec![
                "event_type".to_owned(),
                "checkpoint".to_owned(),
                "transaction_index".to_owned(),
                "event_index".to_owned(),
            ],
        })
        .with_filter(proto::EventFilter::default())
        .with_start_checkpoint(2)
        .with_end_checkpoint(8)
        .with_options(options);
    let event_responses = tokio::time::timeout(
        Duration::from_secs(2),
        client
            .list_events_with_config(event_request, fast_list_config())
            .try_collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for paginated events")
    .unwrap();
    assert_eq!(
        event_responses
            .iter()
            .filter_map(|response| response.event.as_ref())
            .map(|event| event.event_type.as_deref().unwrap())
            .collect::<Vec<_>>(),
        ["event-2", "event-3", "event-6", "event-7"]
    );
    assert!(
        event_responses
            .iter()
            .filter_map(|response| response.event.as_ref())
            .all(|event| {
                event.checkpoint.is_some()
                    && event.transaction_index.is_some()
                    && event.event_index.is_some()
            })
    );

    for reasons in [
        checkpoint_responses
            .iter()
            .map(|response| response.end.as_ref().and_then(|end| end.reason))
            .collect::<Vec<_>>(),
        transaction_responses
            .iter()
            .map(|response| response.end.as_ref().and_then(|end| end.reason))
            .collect::<Vec<_>>(),
        event_responses
            .iter()
            .map(|response| response.end.as_ref().and_then(|end| end.reason))
            .collect::<Vec<_>>(),
    ] {
        assert_eq!(
            reasons,
            [
                None,
                Some(proto::QueryEndReason::ItemLimit as i32),
                None,
                None,
                None,
                Some(proto::QueryEndReason::LedgerTip as i32),
            ]
        );
    }

    let state = server.state.lock().unwrap();
    assert_eq!(state.checkpoint_requests.len(), 2);
    assert_eq!(state.transaction_requests.len(), 2);
    assert_eq!(state.event_requests.len(), 2);
    assert_eq!(state.calls.len(), 6);
    assert!(state.calls.iter().all(|call| call.starts_with("list_")));
    for options in state
        .checkpoint_requests
        .iter()
        .map(|request| &request.body.options)
        .chain(
            state
                .transaction_requests
                .iter()
                .map(|request| &request.body.options),
        )
        .chain(
            state
                .event_requests
                .iter()
                .map(|request| &request.body.options),
        )
    {
        let options = options.as_ref().unwrap();
        assert_eq!(options.limit, Some(2));
        assert_eq!(options.ordering, Some(proto::Ordering::Ascending as i32));
    }
    assert_eq!(
        state.checkpoint_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .after,
        Some(bytes(3))
    );
    assert_eq!(
        state.transaction_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .after,
        Some(bytes(3))
    );
    assert_eq!(
        state.event_requests[1].body.options.as_ref().unwrap().after,
        Some(bytes(3))
    );
    drop(state);

    let observations = observations.lock().unwrap();
    assert_eq!(observations.len(), 6);
    assert!(
        observations
            .iter()
            .all(|observation| observation.path.contains("List"))
    );
}

#[tokio::test]
async fn default_lists_insert_only_the_required_resume_bound() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_checkpoint_lists([
        StreamScript::frames([Ok(checkpoint_list_frame(
            Some(1),
            1,
            Some(proto::QueryEndReason::ItemLimit),
        ))]),
        StreamScript::frames([Ok(checkpoint_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::LedgerTip),
        ))]),
    ]);
    server.push_transaction_lists([
        StreamScript::frames([Ok(transaction_list_frame(
            Some(1),
            1,
            Some(proto::QueryEndReason::ItemLimit),
        ))]),
        StreamScript::frames([Ok(transaction_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::LedgerTip),
        ))]),
    ]);
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            Some(1),
            1,
            Some(proto::QueryEndReason::ItemLimit),
        ))]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::LedgerTip),
        ))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    assert_eq!(
        client
            .list_checkpoints(
                proto::ListCheckpointsRequest::default().with_read_mask(checkpoint_identity_mask()),
            )
            .try_collect::<Vec<_>>()
            .await
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        client
            .list_transactions(
                proto::ListTransactionsRequest::default()
                    .with_read_mask(transaction_identity_mask()),
            )
            .try_collect::<Vec<_>>()
            .await
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        client
            .list_events(proto::ListEventsRequest::default().with_read_mask(event_identity_mask()),)
            .try_collect::<Vec<_>>()
            .await
            .unwrap()
            .len(),
        2
    );

    let state = server.state.lock().unwrap();
    for (first, second) in [
        (
            &state.checkpoint_requests[0].body.options,
            &state.checkpoint_requests[1].body.options,
        ),
        (
            &state.transaction_requests[0].body.options,
            &state.transaction_requests[1].body.options,
        ),
        (
            &state.event_requests[0].body.options,
            &state.event_requests[1].body.options,
        ),
    ] {
        assert!(first.is_none());
        let continuation = second.as_ref().unwrap();
        assert_eq!(continuation.after, Some(bytes(1)));
        assert!(continuation.before.is_none());
        assert!(continuation.limit.is_none());
        assert!(continuation.ordering.is_none());
    }
}

#[tokio::test]
async fn sparse_bounded_streams_yield_progress_and_the_final_watermark() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_checkpoint_lists([StreamScript::frames([
        Ok(checkpoint_list_frame(None, 2, None)),
        Ok(checkpoint_list_frame(Some(3), 3, None)),
        Ok(checkpoint_list_frame(
            None,
            4,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    server.push_transaction_lists([StreamScript::frames([
        Ok(transaction_list_frame(None, 2, None)),
        Ok(transaction_list_frame(Some(3), 3, None)),
        Ok(transaction_list_frame(
            None,
            5,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    server.push_event_lists([StreamScript::frames([
        Ok(event_list_frame(None, 2, None)),
        Ok(event_list_frame(Some(3), 3, None)),
        Ok(event_list_frame(
            None,
            5,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);

    let checkpoint_frames = client
        .list_checkpoints(
            proto::ListCheckpointsRequest::default()
                .with_read_mask(checkpoint_identity_mask())
                .with_end_checkpoint(5),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        checkpoint_frames
            .iter()
            .map(|frame| {
                frame
                    .checkpoint
                    .as_ref()
                    .and_then(|checkpoint| checkpoint.sequence_number)
            })
            .collect::<Vec<_>>(),
        [None, Some(3), None]
    );
    assert_eq!(
        checkpoint_frames
            .iter()
            .map(|frame| frame.watermark.as_ref().unwrap().checkpoint.unwrap())
            .collect::<Vec<_>>(),
        [2, 3, 4]
    );
    assert_eq!(
        checkpoint_frames
            .last()
            .unwrap()
            .end
            .as_ref()
            .unwrap()
            .reason,
        Some(proto::QueryEndReason::CheckpointBound as i32)
    );

    let transaction_frames = client
        .list_transactions(
            proto::ListTransactionsRequest::default()
                .with_read_mask(transaction_identity_mask())
                .with_end_checkpoint(6),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        transaction_frames
            .iter()
            .map(|frame| {
                frame
                    .transaction
                    .as_ref()
                    .and_then(|transaction| transaction.digest.as_deref())
            })
            .collect::<Vec<_>>(),
        [None, Some("tx-3"), None]
    );
    assert_eq!(
        transaction_frames
            .iter()
            .map(|frame| frame.watermark.as_ref().unwrap().cursor.clone().unwrap())
            .collect::<Vec<_>>(),
        [bytes(2), bytes(3), bytes(5)]
    );
    assert_eq!(
        transaction_frames
            .last()
            .unwrap()
            .end
            .as_ref()
            .unwrap()
            .reason,
        Some(proto::QueryEndReason::CheckpointBound as i32)
    );

    let event_frames = client
        .list_events(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_end_checkpoint(6),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        event_frames
            .iter()
            .map(|frame| {
                frame
                    .event
                    .as_ref()
                    .and_then(|event| event.event_type.as_deref())
            })
            .collect::<Vec<_>>(),
        [None, Some("event-3"), None]
    );
    assert_eq!(
        event_frames
            .iter()
            .map(|frame| frame.watermark.as_ref().unwrap().cursor.clone().unwrap())
            .collect::<Vec<_>>(),
        [bytes(2), bytes(3), bytes(5)]
    );
    assert_eq!(
        event_frames.last().unwrap().end.as_ref().unwrap().reason,
        Some(proto::QueryEndReason::CheckpointBound as i32)
    );
}

#[tokio::test]
async fn stream_position_round_trips_through_bytes() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_service_infos([2, 4, 2, 4, 2, 4].into_iter().map(service_info).map(Ok));
    server.push_checkpoint_lists([
        StreamScript::frames([
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
            Ok(transaction_list_frame(Some(2), 2, None)),
            Ok(transaction_list_frame(
                None,
                3,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([
            Ok(transaction_list_frame(Some(3), 3, None)),
            Ok(transaction_list_frame(Some(4), 4, None)),
            Ok(transaction_list_frame(
                None,
                5,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(2), 2, None)),
            Ok(event_list_frame(
                None,
                3,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(3), 3, None)),
            Ok(event_list_frame(Some(4), 4, None)),
            Ok(event_list_frame(
                None,
                5,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let checkpoint_request = CheckpointStreamRequest::new()
        .with_read_mask(checkpoint_identity_mask())
        .with_start(CheckpointStreamStart::Checkpoint(2));
    let checkpoint_frames = client
        .stream_checkpoints(checkpoint_request)
        .take(1)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        checkpoint_frames[0]
            .checkpoint
            .as_ref()
            .unwrap()
            .sequence_number,
        Some(2)
    );
    assert_eq!(checkpoint_frames[0].cursor, 2);
    assert_eq!(checkpoint_frames.len(), 1);

    let checkpoint_resume = CheckpointStreamRequest::new()
        .with_read_mask(checkpoint_identity_mask())
        .with_start(CheckpointStreamStart::Checkpoint(
            checkpoint_frames[0].cursor.checked_add(1).unwrap(),
        ));
    let checkpoints = client
        .stream_checkpoints(checkpoint_resume)
        .take(2)
        .try_filter_map(|frame| async move {
            Ok(frame
                .checkpoint
                .map(|checkpoint| checkpoint.sequence_number.unwrap()))
        })
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(checkpoints, [3, 4]);

    let transaction_request = TransactionStreamRequest::new()
        .with_read_mask(transaction_identity_mask())
        .with_start(TransactionStreamStart::Checkpoint(2));
    let transaction_frames = client
        .stream_transactions(transaction_request)
        .take(1)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        transaction_frames[0]
            .transaction
            .as_ref()
            .unwrap()
            .digest
            .as_deref(),
        Some("tx-2")
    );
    assert_eq!(transaction_frames[0].cursor, bytes(2));
    assert_eq!(transaction_frames[0].covered_checkpoint, Some(2));

    let persisted = transaction_frames[0].cursor.clone();
    let transaction_resume = TransactionStreamRequest::new()
        .with_read_mask(transaction_identity_mask())
        .with_start(TransactionStreamStart::Resume(persisted));
    let transactions = client
        .stream_transactions(transaction_resume)
        .take(2)
        .try_filter_map(|frame| async move {
            Ok(frame
                .transaction
                .map(|transaction| transaction.digest.unwrap()))
        })
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(transactions, ["tx-3", "tx-4"]);

    let event_request = EventStreamRequest::new()
        .with_read_mask(event_identity_mask())
        .with_start(EventStreamStart::Checkpoint(2));
    let event_frames = client
        .stream_events(event_request)
        .take(1)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        event_frames[0]
            .event
            .as_ref()
            .unwrap()
            .event_type
            .as_deref(),
        Some("event-2")
    );
    assert_eq!(event_frames[0].cursor, bytes(2));
    assert_eq!(event_frames[0].covered_checkpoint, Some(2));

    let event_resume = EventStreamRequest::new()
        .with_read_mask(event_identity_mask())
        .with_start(EventStreamStart::Resume(event_frames[0].cursor.clone()));
    let events = client
        .stream_events(event_resume)
        .take(2)
        .try_filter_map(
            |frame| async move { Ok(frame.event.map(|event| event.event_type.unwrap())) },
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(events, ["event-3", "event-4"]);

    let state = server.state.lock().unwrap();

    assert_eq!(state.checkpoint_requests[1].body.start_checkpoint, Some(3));
    assert_eq!(
        state.transaction_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .after,
        Some(bytes(2))
    );
    assert_eq!(
        state.event_requests[1].body.options.as_ref().unwrap().after,
        Some(bytes(2))
    );
}

#[tokio::test]
async fn list_accepts_checkpoint_cursor_and_intersection_bounds() {
    let (server, _calls) = ScriptedStreamServer::new();
    let mut checkpoint_bounded_end =
        event_list_frame(None, 10, Some(proto::QueryEndReason::CheckpointBound));
    checkpoint_bounded_end
        .watermark
        .as_mut()
        .unwrap()
        .checkpoint = Some(9);
    let mut intersection_bounded_end =
        event_list_frame(None, 30, Some(proto::QueryEndReason::CheckpointBound));
    intersection_bounded_end
        .watermark
        .as_mut()
        .unwrap()
        .checkpoint = Some(29);
    server.push_event_lists([
        StreamScript::frames([Ok(checkpoint_bounded_end)]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            20,
            Some(proto::QueryEndReason::CursorBound),
        ))]),
        StreamScript::frames([Ok(intersection_bounded_end)]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let requests = [
        proto::ListEventsRequest::default()
            .with_read_mask(event_identity_mask())
            .with_start_checkpoint(2)
            .with_end_checkpoint(10)
            .with_options(
                proto::QueryOptions::default()
                    .with_after(bytes(1))
                    .with_limit(7),
            ),
        proto::ListEventsRequest::default()
            .with_read_mask(event_identity_mask())
            .with_options(proto::QueryOptions::default().with_before(bytes(20))),
        proto::ListEventsRequest::default()
            .with_read_mask(event_identity_mask())
            .with_end_checkpoint(30)
            .with_options(proto::QueryOptions::default().with_before(bytes(30))),
    ];
    for request_body in requests {
        let frames = client
            .list_events_with_config(request_body, fast_list_config())
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
        assert_eq!(frames.len(), 1);
        assert!(frames[0].event.is_none());
    }

    let state = server.state.lock().unwrap();
    assert_eq!(state.event_requests.len(), 3);
    assert!(state.calls.iter().all(|call| *call == "list_events"));
}

#[tokio::test]
async fn opaque_checkpoint_bound_accepts_server_reported_coverage() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_transaction_lists([StreamScript::frames([Ok(transaction_list_frame(
        None,
        10,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        10,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let tx_responses = client
        .list_transactions(
            proto::ListTransactionsRequest::default()
                .with_read_mask(transaction_identity_mask())
                .with_end_checkpoint(10),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(tx_responses.len(), 1);
    assert_eq!(
        tx_responses[0].watermark.as_ref().unwrap().checkpoint,
        Some(10)
    );

    let event_responses = client
        .list_events(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_end_checkpoint(10),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(event_responses.len(), 1);
    assert_eq!(
        event_responses[0].watermark.as_ref().unwrap().checkpoint,
        Some(10)
    );

    assert_eq!(
        server.state.lock().unwrap().calls,
        ["list_transactions", "list_events"]
    );
}

#[tokio::test]
async fn descending_event_checkpoint_bound_accepts_server_reported_opaque_coverage() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        5,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);
    let responses = client
        .list_events(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_start_checkpoint(4)
                .with_options(
                    proto::QueryOptions::default().with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(responses.len(), 1);
    assert_eq!(responses[0].watermark.as_ref().unwrap().checkpoint, Some(5));
    assert_eq!(server.state.lock().unwrap().calls, ["list_events"]);
}

#[tokio::test]
async fn list_carries_checkpoint_coverage_across_continuation_requests() {
    let (server, _calls) = ScriptedStreamServer::new();
    let mut continued_item = event_list_frame(Some(6), 6, None);
    continued_item.watermark.as_mut().unwrap().checkpoint = None;
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Err(tonic::Status::unavailable("partial response")),
        ]),
        StreamScript::frames([
            Ok(continued_item),
            Ok(event_list_frame(
                None,
                7,
                Some(proto::QueryEndReason::CheckpointBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let frames = client
        .list_events_with_config(
            bounded_event_request().with_end_checkpoint(8),
            fast_list_config(),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        frames
            .iter()
            .map(|frame| frame.watermark.as_ref().unwrap().cursor.clone().unwrap())
            .collect::<Vec<_>>(),
        [bytes(5), bytes(6), bytes(7)]
    );
    assert_eq!(
        frames
            .iter()
            .map(|frame| frame.watermark.as_ref().unwrap().checkpoint)
            .collect::<Vec<_>>(),
        [Some(5), None, Some(7)]
    );
    let state = server.state.lock().unwrap();
    assert_eq!(state.event_requests.len(), 2);
    assert_eq!(
        state.event_requests[1].body.options.as_ref().unwrap().after,
        Some(bytes(5))
    );
}

#[tokio::test]
async fn list_accepts_missing_coverage_before_the_request_establishes_it() {
    let (server, _calls) = ScriptedStreamServer::new();
    let mut first = event_list_frame(Some(0), 0, None);
    first.watermark.as_mut().unwrap().checkpoint = None;
    let mut second = event_list_frame(Some(1), 1, None);
    second.watermark.as_mut().unwrap().checkpoint = None;
    server.push_event_lists([StreamScript::frames([
        Ok(first),
        Ok(second),
        Ok(event_list_frame(
            None,
            1,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);

    let frames = client
        .list_events(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_start_checkpoint(0)
                .with_end_checkpoint(2),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        frames
            .iter()
            .filter_map(|frame| frame.event.as_ref())
            .map(|event| event.event_type.as_deref().unwrap())
            .collect::<Vec<_>>(),
        ["event-0", "event-1"]
    );
    assert_eq!(frames[0].watermark.as_ref().unwrap().checkpoint, None);
    assert_eq!(frames[1].watermark.as_ref().unwrap().checkpoint, None);
}

#[tokio::test]
async fn list_accepts_empty_continuation_without_checkpoint_coverage() {
    let (server, _calls) = ScriptedStreamServer::new();
    let mut empty_terminal =
        event_list_frame(None, 5, Some(proto::QueryEndReason::CheckpointBound));
    empty_terminal.watermark.as_mut().unwrap().checkpoint = None;
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(5), 5, None)),
            Err(tonic::Status::unavailable("partial response")),
        ]),
        StreamScript::frames([Ok(empty_terminal)]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let frames = client
        .list_events_with_config(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_end_checkpoint(6),
            fast_list_config(),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(frames.len(), 2);
    assert_eq!(
        frames[0].event.as_ref().unwrap().event_type.as_deref(),
        Some("event-5")
    );
    assert!(frames[1].event.is_none());
    assert_eq!(frames[1].watermark.as_ref().unwrap().checkpoint, None);
    assert_eq!(
        frames[1].end.as_ref().unwrap().reason,
        Some(proto::QueryEndReason::CheckpointBound as i32)
    );
    assert_eq!(server.state.lock().unwrap().event_requests.len(), 2);
}

#[tokio::test]
async fn scan_limit_frontiers_redispatch_immediately_when_the_cursor_advances() {
    let (server, mut calls) = ScriptedStreamServer::new();
    let (repeated_scan_tx, repeated_scan_rx) = mpsc::unbounded_channel();
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            2,
            Some(proto::QueryEndReason::ScanLimit),
        ))]),
        StreamScript::Channel(repeated_scan_rx),
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
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);
    let mut config = ListConfig::default();
    config.base_retry_delay = Duration::from_secs(60);
    config.max_retry_delay = Duration::from_secs(60);
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

    assert_eq!(calls.recv().await, Some("list_events"));
    assert_eq!(calls.recv().await, Some("list_events"));
    repeated_scan_tx
        .send(Ok(event_list_frame(
            None,
            4,
            Some(proto::QueryEndReason::ScanLimit),
        )))
        .unwrap();
    assert_eq!(
        tokio::time::timeout(STREAM_DROP_TIMEOUT, calls.recv())
            .await
            .expect("ScanLimit continuation was delayed"),
        Some("list_events")
    );
    assert_eq!(collector.await.unwrap().unwrap(), ["event-5"]);
}

#[tokio::test]
async fn descending_streams_are_list_only_and_normalize_genesis_for_all_families() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_checkpoint_lists([StreamScript::frames([
        Ok(checkpoint_list_frame(Some(9), 9, None)),
        Ok(checkpoint_list_frame(Some(7), 7, None)),
        Ok(checkpoint_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    server.push_transaction_lists([StreamScript::frames([
        Ok(transaction_list_frame(Some(9), 9, None)),
        Ok(transaction_list_frame(Some(7), 7, None)),
        Ok(transaction_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    server.push_event_lists([StreamScript::frames([
        Ok(event_list_frame(Some(9), 9, None)),
        Ok(event_list_frame(Some(7), 7, None)),
        Ok(event_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        )),
    ])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let checkpoint_ids = client
        .list_checkpoints(
            proto::ListCheckpointsRequest::default()
                .with_read_mask(checkpoint_identity_mask())
                .with_options(
                    proto::QueryOptions::default().with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_filter_map(|frame| async move {
            Ok(frame
                .checkpoint
                .and_then(|checkpoint| checkpoint.sequence_number))
        })
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    let transaction_ids = client
        .list_transactions(
            proto::ListTransactionsRequest::default()
                .with_read_mask(transaction_identity_mask())
                .with_options(
                    proto::QueryOptions::default().with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_filter_map(|frame| async move {
            Ok(frame
                .transaction
                .and_then(|transaction| transaction.checkpoint))
        })
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    let event_ids = client
        .list_events(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_options(
                    proto::QueryOptions::default().with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_filter_map(|frame| async move { Ok(frame.event.and_then(|event| event.checkpoint)) })
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(checkpoint_ids, [9, 7]);
    assert_eq!(transaction_ids, [9, 7]);
    assert_eq!(event_ids, [9, 7]);

    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        ["list_checkpoints", "list_transactions", "list_events"]
    );
    assert!(state.checkpoint_subscriptions.is_empty());
    assert!(state.transaction_subscriptions.is_empty());
    assert!(state.event_subscriptions.is_empty());
    assert_eq!(state.checkpoint_requests[0].body.start_checkpoint, None);
    assert_eq!(state.transaction_requests[0].body.start_checkpoint, None);
    assert_eq!(state.event_requests[0].body.start_checkpoint, None);
    for options in [
        state.checkpoint_requests[0].body.options.as_ref().unwrap(),
        state.transaction_requests[0].body.options.as_ref().unwrap(),
        state.event_requests[0].body.options.as_ref().unwrap(),
    ] {
        assert_eq!(options.ordering, Some(proto::Ordering::Descending as i32));
        assert!(options.after.is_none());
        assert!(options.before.is_none());
    }
}

#[tokio::test]
async fn descending_item_and_scan_limits_resume_through_before_and_preserve_after() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_transaction_lists([
        StreamScript::frames([
            Ok(transaction_list_frame(Some(9), 9, None)),
            Ok(transaction_list_frame(
                Some(8),
                8,
                Some(proto::QueryEndReason::ItemLimit),
            )),
        ]),
        StreamScript::frames([
            Ok(transaction_list_frame(Some(7), 7, None)),
            Ok(transaction_list_frame(
                None,
                5,
                Some(proto::QueryEndReason::CursorBound),
            )),
        ]),
    ]);
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            None,
            8,
            Some(proto::QueryEndReason::ScanLimit),
        ))]),
        StreamScript::frames([
            Ok(event_list_frame(Some(7), 7, None)),
            Ok(event_list_frame(
                None,
                5,
                Some(proto::QueryEndReason::CursorBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let transaction_ids = client
        .list_transactions(
            proto::ListTransactionsRequest::default()
                .with_read_mask(transaction_identity_mask())
                .with_options(
                    proto::QueryOptions::default()
                        .with_after(bytes(5))
                        .with_before(bytes(10))
                        .with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_filter_map(|frame| async move {
            Ok(frame
                .transaction
                .and_then(|transaction| transaction.checkpoint))
        })
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    let event_ids = client
        .list_events(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_options(
                    proto::QueryOptions::default()
                        .with_after(bytes(5))
                        .with_before(bytes(10))
                        .with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_filter_map(|frame| async move { Ok(frame.event.and_then(|event| event.checkpoint)) })
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(transaction_ids, [9, 8, 7]);
    assert_eq!(event_ids, [7]);

    let state = server.state.lock().unwrap();
    assert_eq!(
        state.calls,
        [
            "list_transactions",
            "list_transactions",
            "list_events",
            "list_events"
        ]
    );
    assert!(state.transaction_subscriptions.is_empty());
    assert!(state.event_subscriptions.is_empty());
    for request in [
        &state.transaction_requests[0],
        &state.transaction_requests[1],
    ] {
        let options = request.body.options.as_ref().unwrap();
        assert_eq!(options.after, Some(bytes(5)));
        assert_eq!(options.ordering, Some(proto::Ordering::Descending as i32));
    }
    assert_eq!(
        state.transaction_requests[0]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(10))
    );
    assert_eq!(
        state.transaction_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(8))
    );
    for request in [&state.event_requests[0], &state.event_requests[1]] {
        let options = request.body.options.as_ref().unwrap();
        assert_eq!(options.after, Some(bytes(5)));
        assert_eq!(options.ordering, Some(proto::Ordering::Descending as i32));
    }
    assert_eq!(
        state.event_requests[0]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(10))
    );
    assert_eq!(
        state.event_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(8))
    );
}

#[tokio::test]
async fn descending_partial_response_retry_resumes_through_before() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(9), 9, None)),
            Err(tonic::Status::unavailable("partial response")),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(Some(8), 8, None)),
            Ok(event_list_frame(
                None,
                5,
                Some(proto::QueryEndReason::CursorBound),
            )),
        ]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let event_ids = client
        .list_events_with_config(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_options(
                    proto::QueryOptions::default()
                        .with_after(bytes(5))
                        .with_before(bytes(10))
                        .with_ordering(proto::Ordering::Descending),
                ),
            fast_list_config(),
        )
        .try_filter_map(|frame| async move { Ok(frame.event.and_then(|event| event.checkpoint)) })
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(event_ids, [9, 8]);
    let state = server.state.lock().unwrap();
    assert_eq!(state.calls, ["list_events", "list_events"]);
    let retry_options = state.event_requests[1].body.options.as_ref().unwrap();
    assert_eq!(retry_options.after, Some(bytes(5)));
    assert_eq!(retry_options.before, Some(bytes(9)));
    assert_eq!(
        retry_options.ordering,
        Some(proto::Ordering::Descending as i32)
    );
}

#[tokio::test]
async fn descending_accepts_low_cursor_completion_and_empty_ledger_tip() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_transaction_lists([StreamScript::frames([Ok(transaction_list_frame(
        None,
        5,
        Some(proto::QueryEndReason::CursorBound),
    ))])]);
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        9,
        Some(proto::QueryEndReason::LedgerTip),
    ))])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let transaction_frames = client
        .list_transactions(
            proto::ListTransactionsRequest::default()
                .with_read_mask(transaction_identity_mask())
                .with_options(
                    proto::QueryOptions::default()
                        .with_after(bytes(5))
                        .with_before(bytes(5))
                        .with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    let event_frames = client
        .list_events(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_options(
                    proto::QueryOptions::default().with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(transaction_frames.len(), 1);
    assert!(transaction_frames[0].transaction.is_none());
    assert_eq!(
        transaction_frames[0].end.as_ref().unwrap().reason,
        Some(proto::QueryEndReason::CursorBound as i32)
    );
    assert_eq!(event_frames.len(), 1);
    assert!(event_frames[0].event.is_none());
    assert_eq!(
        event_frames[0].watermark.as_ref().unwrap().cursor,
        Some(bytes(9))
    );
    let state = server.state.lock().unwrap();
    assert_eq!(state.calls, ["list_transactions", "list_events"]);
    assert!(state.transaction_subscriptions.is_empty());
    assert!(state.event_subscriptions.is_empty());
    assert_eq!(state.event_requests[0].body.start_checkpoint, None);
}

#[tokio::test]
async fn nonempty_fixed_streams_reject_entry_side_terminal_reasons() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_transaction_lists([StreamScript::frames([
        Ok(transaction_list_frame(Some(9), 9, None)),
        Ok(transaction_list_frame(
            None,
            5,
            Some(proto::QueryEndReason::CursorBound),
        )),
    ])]);
    server.push_event_lists([StreamScript::frames([
        Ok(event_list_frame(Some(1), 1, None)),
        Ok(event_list_frame(
            None,
            10,
            Some(proto::QueryEndReason::CursorBound),
        )),
    ])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let descending_status = client
        .list_transactions(
            proto::ListTransactionsRequest::default()
                .with_read_mask(transaction_identity_mask())
                .with_start_checkpoint(0)
                .with_options(
                    proto::QueryOptions::default()
                        .with_before(bytes(10))
                        .with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap_err();
    assert_eq!(descending_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        descending_status.message(),
        "List ended at an unexpected bound"
    );

    let ascending_status = client
        .list_events(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_end_checkpoint(10)
                .with_options(proto::QueryOptions::default().with_after(bytes(0))),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap_err();
    assert_eq!(ascending_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        ascending_status.message(),
        "List ended at an unexpected bound"
    );
}

#[tokio::test]
async fn empty_descending_stream_accepts_entry_side_cursor_bound() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        0,
        Some(proto::QueryEndReason::CursorBound),
    ))])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let frames = client
        .list_events(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_options(
                    proto::QueryOptions::default()
                        .with_before(bytes(0))
                        .with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(frames.len(), 1);
    assert!(frames[0].event.is_none());
    assert_eq!(
        frames[0].end.as_ref().unwrap().reason,
        Some(proto::QueryEndReason::CursorBound as i32)
    );
    let state = server.state.lock().unwrap();
    assert_eq!(state.event_requests[0].body.start_checkpoint, None);
    assert!(state.event_subscriptions.is_empty());
}

#[tokio::test]
async fn progress_only_fixed_streams_reject_entry_side_or_ledger_tip_termination() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([StreamScript::frames([
        Ok(event_list_frame(None, 9, None)),
        Ok(event_list_frame(
            None,
            5,
            Some(proto::QueryEndReason::CursorBound),
        )),
    ])]);
    server.push_transaction_lists([StreamScript::frames([
        Ok(transaction_list_frame(None, 9, None)),
        Ok(transaction_list_frame(
            None,
            9,
            Some(proto::QueryEndReason::LedgerTip),
        )),
    ])]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let cursor_status = client
        .list_events(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_start_checkpoint(0)
                .with_options(
                    proto::QueryOptions::default()
                        .with_before(bytes(10))
                        .with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap_err();
    assert_eq!(cursor_status.code(), tonic::Code::DataLoss);
    assert_eq!(cursor_status.message(), "List ended at an unexpected bound");

    let ledger_tip_status = client
        .list_transactions(
            proto::ListTransactionsRequest::default()
                .with_read_mask(transaction_identity_mask())
                .with_options(
                    proto::QueryOptions::default().with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap_err();
    assert_eq!(ledger_tip_status.code(), tonic::Code::DataLoss);
    assert_eq!(
        ledger_tip_status.message(),
        "descending List reached LedgerTip after prior scan progress"
    );
}

#[tokio::test]
async fn descending_retry_accepts_terminal_only_empty_intersection() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([
        StreamScript::frames([
            Ok(event_list_frame(Some(9), 9, None)),
            Err(tonic::Status::unavailable("partial response")),
        ]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            9,
            Some(proto::QueryEndReason::CursorBound),
        ))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let events = client
        .list_events_with_config(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_start_checkpoint(9)
                .with_options(
                    proto::QueryOptions::default()
                        .with_before(bytes(10))
                        .with_ordering(proto::Ordering::Descending),
                ),
            fast_list_config(),
        )
        .try_filter_map(|frame| async move { Ok(frame.event.and_then(|event| event.checkpoint)) })
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(events, [9]);
    let state = server.state.lock().unwrap();
    assert_eq!(state.event_requests.len(), 2);
    assert_eq!(
        state.event_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(9))
    );
}

#[tokio::test]
async fn descending_retry_rejects_terminal_only_ledger_tip_after_progress() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_transaction_lists([
        StreamScript::frames([
            Ok(transaction_list_frame(None, 9, None)),
            Err(tonic::Status::unavailable("partial response")),
        ]),
        StreamScript::frames([Ok(transaction_list_frame(
            None,
            9,
            Some(proto::QueryEndReason::LedgerTip),
        ))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let status = client
        .list_transactions_with_config(
            proto::ListTransactionsRequest::default()
                .with_read_mask(transaction_identity_mask())
                .with_options(
                    proto::QueryOptions::default().with_ordering(proto::Ordering::Descending),
                ),
            fast_list_config(),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap_err();

    assert_eq!(status.code(), tonic::Code::DataLoss);
    assert_eq!(
        status.message(),
        "descending List reached LedgerTip after prior scan progress"
    );
    let state = server.state.lock().unwrap();
    assert_eq!(state.transaction_requests.len(), 2);
    assert_eq!(
        state.transaction_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(9))
    );
}

#[tokio::test]
async fn descending_item_limit_accepts_terminal_only_resume_cursor_bound() {
    let (server, _calls) = ScriptedStreamServer::new();
    server.push_event_lists([
        StreamScript::frames([Ok(event_list_frame(
            Some(9),
            9,
            Some(proto::QueryEndReason::ItemLimit),
        ))]),
        StreamScript::frames([Ok(event_list_frame(
            None,
            9,
            Some(proto::QueryEndReason::CursorBound),
        ))]),
    ]);
    let address = spawn_server(server.clone()).await;
    let (client, _observations) = observed_client(address);

    let events = client
        .list_events(
            proto::ListEventsRequest::default()
                .with_read_mask(event_identity_mask())
                .with_start_checkpoint(9)
                .with_options(
                    proto::QueryOptions::default().with_ordering(proto::Ordering::Descending),
                ),
        )
        .try_filter_map(|frame| async move { Ok(frame.event.and_then(|event| event.checkpoint)) })
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(events, [9]);
    let state = server.state.lock().unwrap();
    assert_eq!(state.event_requests.len(), 2);
    assert_eq!(
        state.event_requests[1]
            .body
            .options
            .as_ref()
            .unwrap()
            .before,
        Some(bytes(9))
    );
}
