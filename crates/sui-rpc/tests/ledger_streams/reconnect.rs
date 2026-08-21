use super::support::ScriptedStreamServer;
use super::support::StreamScript;
use super::support::bounded_event_request;
use super::support::bytes;
use super::support::event_list_frame;
use super::support::event_positioned_list_frame;
use super::support::fast_list_config;
use super::support::first_list_event_error;
use super::support::next_scripted_call;
use super::support::observed_client;
use super::support::spawn_server;
use futures::StreamExt;
use futures::TryStreamExt;
use std::time::Duration;
use sui_rpc::client::ListConfig;
use sui_rpc::proto::sui::rpc::v2 as proto;
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
