use super::support::ScriptedStreamServer;
use super::support::StreamScript;
use super::support::bytes;
use super::support::event_identity_mask;
use super::support::event_list_frame;
use super::support::event_live_frame;
use super::support::observed_client;
use super::support::recording_config;
use super::support::service_info;
use super::support::spawn_server;
use futures::StreamExt;
use futures::TryStreamExt;
use std::time::Duration;
use std::time::Instant;
use sui_rpc::client::EventStreamRequest;
use sui_rpc::client::EventStreamStart;
use sui_rpc::client::LedgerStreamConfig;
use sui_rpc::client::LedgerStreamEvent;
use sui_rpc::client::LedgerStreamFamily;
use sui_rpc::client::LedgerStreamOperation;
use sui_rpc::client::LedgerStreamStage;
use sui_rpc::proto::sui::rpc::v2 as proto;
use tokio::sync::mpsc;

#[tokio::test(start_paused = true)]
async fn observer_reports_rpc_response_context_and_elapsed_time() {
    const REAL_TIME_WAIT_TIMEOUT: Duration = Duration::from_secs(2);

    async fn receive_call_without_advancing_time(
        calls: &mut mpsc::UnboundedReceiver<&'static str>,
        expected_call: &str,
    ) -> &'static str {
        let deadline = Instant::now() + REAL_TIME_WAIT_TIMEOUT;
        loop {
            match calls.try_recv() {
                Ok(call) => return call,
                Err(mpsc::error::TryRecvError::Empty) => {
                    assert!(
                        Instant::now() < deadline,
                        "timed out waiting for {expected_call} scripted RPC call"
                    );
                    tokio::task::yield_now().await;
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    panic!(
                        "scripted call channel closed while waiting for {expected_call} RPC call"
                    )
                }
            }
        }
    }

    async fn receive_observer_event_without_advancing_time(
        observer_events: &mut mpsc::UnboundedReceiver<LedgerStreamEvent>,
        expected_event: &str,
    ) -> LedgerStreamEvent {
        let deadline = Instant::now() + REAL_TIME_WAIT_TIMEOUT;
        loop {
            match observer_events.try_recv() {
                Ok(event) => return event,
                Err(mpsc::error::TryRecvError::Empty) => {
                    assert!(
                        Instant::now() < deadline,
                        "timed out waiting for {expected_event} observer event"
                    );
                    tokio::task::yield_now().await;
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    panic!("observer event channel closed while waiting for {expected_event} event")
                }
            }
        }
    }

    let (server, mut calls) = ScriptedStreamServer::new();
    server.set_response_delay(Duration::from_secs(1));
    server.push_service_infos([Ok(service_info(0))]);
    server.push_event_lists([StreamScript::frames([Ok(event_list_frame(
        None,
        0,
        Some(proto::QueryEndReason::CheckpointBound),
    ))])]);
    let (live_tx, live_rx) = mpsc::unbounded_channel();
    server.push_event_subscriptions([StreamScript::Channel(live_rx)]);
    let address = spawn_server(server).await;
    let (client, observations) = observed_client(address);
    let (config, mut observer_events) = recording_config(LedgerStreamConfig::default());
    let collector = tokio::spawn(async move {
        client
            .stream_events_with_config(
                EventStreamRequest::new()
                    .with_read_mask(event_identity_mask())
                    .with_start(EventStreamStart::Checkpoint(0)),
                config,
            )
            .take(3)
            .try_collect::<Vec<_>>()
            .await
    });

    assert_eq!(
        receive_call_without_advancing_time(&mut calls, "GetServiceInfo").await,
        "get_service_info"
    );
    tokio::task::yield_now().await;
    tokio::time::advance(Duration::from_secs(1)).await;
    match receive_observer_event_without_advancing_time(
        &mut observer_events,
        "GetServiceInfo RpcResponse",
    )
    .await
    {
        LedgerStreamEvent::RpcResponse {
            family,
            operation,
            stage,
            code,
            elapsed,
            ..
        } => {
            assert_eq!(family, LedgerStreamFamily::Event);
            assert_eq!(operation, LedgerStreamOperation::GetServiceInfo);
            assert_eq!(stage, LedgerStreamStage::InitialReplay);
            assert_eq!(code, tonic::Code::Ok);
            assert_eq!(elapsed, Duration::from_secs(1));
        }
        event => panic!("unexpected observer event: {event:?}"),
    }

    assert_eq!(
        receive_call_without_advancing_time(&mut calls, "ListEvents").await,
        "list_events"
    );
    tokio::task::yield_now().await;
    tokio::time::advance(Duration::from_secs(1)).await;
    match receive_observer_event_without_advancing_time(
        &mut observer_events,
        "ListEvents RpcResponse",
    )
    .await
    {
        LedgerStreamEvent::RpcResponse {
            family,
            operation,
            stage,
            code,
            elapsed,
            ..
        } => {
            assert_eq!(family, LedgerStreamFamily::Event);
            assert_eq!(operation, LedgerStreamOperation::List);
            assert_eq!(stage, LedgerStreamStage::InitialReplay);
            assert_eq!(code, tonic::Code::Ok);
            assert_eq!(elapsed, Duration::from_secs(1));
        }
        event => panic!("unexpected observer event: {event:?}"),
    }

    assert_eq!(
        receive_call_without_advancing_time(&mut calls, "SubscribeEvents").await,
        "subscribe_events"
    );
    tokio::task::yield_now().await;
    tokio::time::advance(Duration::from_secs(1)).await;
    match receive_observer_event_without_advancing_time(
        &mut observer_events,
        "SubscribeEvents RpcResponse",
    )
    .await
    {
        LedgerStreamEvent::RpcResponse {
            family,
            operation,
            stage,
            code,
            elapsed,
            ..
        } => {
            assert_eq!(family, LedgerStreamFamily::Event);
            assert_eq!(operation, LedgerStreamOperation::Subscribe);
            assert_eq!(stage, LedgerStreamStage::LiveSubscription);
            assert_eq!(code, tonic::Code::Ok);
            assert_eq!(elapsed, Duration::from_secs(1));
        }
        event => panic!("unexpected observer event: {event:?}"),
    }

    live_tx.send(Ok(event_live_frame(None, 0))).unwrap();
    live_tx.send(Ok(event_live_frame(Some(1), 1))).unwrap();
    live_tx.send(Ok(event_live_frame(None, 2))).unwrap();
    let collector_deadline = Instant::now() + REAL_TIME_WAIT_TIMEOUT;
    while !collector.is_finished() {
        assert!(
            Instant::now() < collector_deadline,
            "timed out waiting for RPC response context collector"
        );
        tokio::task::yield_now().await;
    }
    let frames = collector.await.unwrap().unwrap();
    drop(live_tx);
    assert_eq!(frames.len(), 3);
    assert!(frames[0].event.is_none());
    assert_eq!(frames[0].cursor, bytes(0));
    assert_eq!(
        frames[1].event.as_ref().unwrap().event_type.as_deref(),
        Some("event-1")
    );
    assert_eq!(frames[1].cursor, bytes(1));
    assert!(frames[2].event.is_none());
    assert_eq!(frames[2].cursor, bytes(2));
    assert!(observer_events.try_recv().is_err());

    let observations = observations.lock().unwrap();
    assert_eq!(
        observations
            .iter()
            .map(|observation| observation.path.rsplit('/').next().unwrap())
            .collect::<Vec<_>>(),
        ["GetServiceInfo", "ListEvents", "SubscribeEvents"]
    );
}

#[tokio::test(start_paused = true)]
async fn observer_reports_independent_retry_episodes() {
    const REAL_TIME_WAIT_TIMEOUT: Duration = Duration::from_secs(2);

    async fn receive_call_without_advancing_time(
        calls: &mut mpsc::UnboundedReceiver<&'static str>,
        expected_call: &str,
    ) -> &'static str {
        let deadline = Instant::now() + REAL_TIME_WAIT_TIMEOUT;
        loop {
            match calls.try_recv() {
                Ok(call) => return call,
                Err(mpsc::error::TryRecvError::Empty) => {
                    assert!(
                        Instant::now() < deadline,
                        "timed out waiting for {expected_call} scripted RPC call"
                    );
                    tokio::task::yield_now().await;
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    panic!(
                        "scripted call channel closed while waiting for {expected_call} RPC call"
                    )
                }
            }
        }
    }

    async fn receive_observer_event_without_advancing_time(
        observer_events: &mut mpsc::UnboundedReceiver<LedgerStreamEvent>,
        operation: LedgerStreamOperation,
        expected_event: &str,
    ) -> LedgerStreamEvent {
        let deadline = Instant::now() + REAL_TIME_WAIT_TIMEOUT;
        loop {
            match observer_events.try_recv() {
                Ok(event) => return event,
                Err(mpsc::error::TryRecvError::Empty) => {
                    assert!(
                        Instant::now() < deadline,
                        "timed out waiting for {expected_event} observer event for {operation:?}"
                    );
                    tokio::task::yield_now().await;
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    panic!(
                        "observer event channel closed while waiting for {expected_event} event for {operation:?}"
                    )
                }
            }
        }
    }

    let (server, mut calls) = ScriptedStreamServer::new();
    server.push_service_infos([
        Err(tonic::Status::unavailable("service info failed")),
        Ok(service_info(0)),
        Ok(service_info(0)),
    ]);
    server.push_event_lists([
        StreamScript::DispatchError(tonic::Status::unavailable("list failed")),
        StreamScript::frames([Ok(event_list_frame(
            None,
            0,
            Some(proto::QueryEndReason::CheckpointBound),
        ))]),
    ]);
    let (live_tx, live_rx) = mpsc::unbounded_channel();
    server.push_event_subscriptions([
        StreamScript::DispatchError(tonic::Status::unavailable("subscribe failed")),
        StreamScript::Channel(live_rx),
    ]);
    let address = spawn_server(server).await;
    let (client, _observations) = observed_client(address);
    let mut config = LedgerStreamConfig::default();
    config.base_retry_delay = Duration::from_secs(1);
    config.max_retry_delay = Duration::from_secs(1);
    config.retry_jitter = Duration::ZERO;
    let (config, mut observer_events) = recording_config(config);
    let collector = tokio::spawn(async move {
        client
            .stream_events_with_config(
                EventStreamRequest::new()
                    .with_read_mask(event_identity_mask())
                    .with_start(EventStreamStart::Checkpoint(0)),
                config,
            )
            .take(2)
            .try_collect::<Vec<_>>()
            .await
    });

    for (call, operation, stage, message) in [
        (
            "get_service_info",
            LedgerStreamOperation::GetServiceInfo,
            LedgerStreamStage::InitialReplay,
            "service info failed",
        ),
        (
            "list_events",
            LedgerStreamOperation::List,
            LedgerStreamStage::InitialReplay,
            "list failed",
        ),
        (
            "subscribe_events",
            LedgerStreamOperation::Subscribe,
            LedgerStreamStage::LiveSubscription,
            "subscribe failed",
        ),
    ] {
        assert_eq!(
            receive_call_without_advancing_time(&mut calls, call).await,
            call
        );
        match receive_observer_event_without_advancing_time(
            &mut observer_events,
            operation,
            "failed RpcResponse",
        )
        .await
        {
            LedgerStreamEvent::RpcResponse {
                family,
                operation: observed_operation,
                stage: observed_stage,
                code,
                elapsed,
                ..
            } => {
                assert_eq!(family, LedgerStreamFamily::Event);
                assert_eq!(observed_operation, operation);
                assert_eq!(observed_stage, stage);
                assert_eq!(code, tonic::Code::Unavailable);
                assert_eq!(elapsed, Duration::ZERO);
            }
            event => panic!("unexpected observer event: {event:?}"),
        }
        match receive_observer_event_without_advancing_time(
            &mut observer_events,
            operation,
            "RetryScheduled",
        )
        .await
        {
            LedgerStreamEvent::RetryScheduled {
                family,
                operation: observed_operation,
                stage: observed_stage,
                status,
                consecutive_failures,
                delay,
                ..
            } => {
                assert_eq!(family, LedgerStreamFamily::Event);
                assert_eq!(observed_operation, operation);
                assert_eq!(observed_stage, stage);
                assert_eq!(status.code(), tonic::Code::Unavailable);
                assert_eq!(status.message(), message);
                assert_eq!(consecutive_failures, 1);
                assert_eq!(delay, Duration::from_secs(1));
            }
            event => panic!("unexpected observer event: {event:?}"),
        }
        tokio::task::yield_now().await;
        assert!(calls.try_recv().is_err());

        tokio::time::advance(Duration::from_secs(1)).await;
        if operation == LedgerStreamOperation::Subscribe {
            assert_eq!(
                receive_call_without_advancing_time(&mut calls, "get_service_info").await,
                "get_service_info"
            );
            match receive_observer_event_without_advancing_time(
                &mut observer_events,
                LedgerStreamOperation::GetServiceInfo,
                "recovery-tip RpcResponse",
            )
            .await
            {
                LedgerStreamEvent::RpcResponse {
                    family,
                    operation: observed_operation,
                    stage,
                    code,
                    elapsed,
                    ..
                } => {
                    assert_eq!(family, LedgerStreamFamily::Event);
                    assert_eq!(observed_operation, LedgerStreamOperation::GetServiceInfo);
                    assert_eq!(stage, LedgerStreamStage::GapRecovery);
                    assert_eq!(code, tonic::Code::Ok);
                    assert_eq!(elapsed, Duration::ZERO);
                }
                event => panic!("unexpected observer event: {event:?}"),
            }
            assert_eq!(
                receive_call_without_advancing_time(&mut calls, call).await,
                call
            );
        } else {
            assert_eq!(
                receive_call_without_advancing_time(&mut calls, call).await,
                call
            );
        }
        match receive_observer_event_without_advancing_time(
            &mut observer_events,
            operation,
            "successful RpcResponse",
        )
        .await
        {
            LedgerStreamEvent::RpcResponse {
                family,
                operation: observed_operation,
                stage: observed_stage,
                code,
                elapsed,
                ..
            } => {
                assert_eq!(family, LedgerStreamFamily::Event);
                assert_eq!(observed_operation, operation);
                assert_eq!(observed_stage, stage);
                assert_eq!(code, tonic::Code::Ok);
                assert_eq!(elapsed, Duration::ZERO);
            }
            event => panic!("unexpected observer event: {event:?}"),
        }
        if operation == LedgerStreamOperation::Subscribe {
            live_tx.send(Ok(event_live_frame(None, 0))).unwrap();
        }
        match receive_observer_event_without_advancing_time(
            &mut observer_events,
            operation,
            "RetryRecovered",
        )
        .await
        {
            LedgerStreamEvent::RetryRecovered {
                family,
                operation: observed_operation,
                started_in,
                consecutive_failures,
                elapsed,
                ..
            } => {
                assert_eq!(family, LedgerStreamFamily::Event);
                assert_eq!(observed_operation, operation);
                assert_eq!(started_in, stage);
                assert_eq!(consecutive_failures, 1);
                assert_eq!(elapsed, Duration::from_secs(1));
            }
            event => panic!("unexpected observer event: {event:?}"),
        }
        if operation == LedgerStreamOperation::Subscribe {
            live_tx.send(Ok(event_live_frame(Some(1), 1))).unwrap();
        }
    }

    let collector_deadline = Instant::now() + REAL_TIME_WAIT_TIMEOUT;
    while !collector.is_finished() {
        assert!(
            Instant::now() < collector_deadline,
            "timed out waiting for independent retry episode collector"
        );
        tokio::task::yield_now().await;
    }
    let frames = collector.await.unwrap().unwrap();
    drop(live_tx);
    assert_eq!(frames.len(), 2);
    assert!(frames[0].event.is_none());
    assert_eq!(frames[0].cursor, bytes(0));
    assert_eq!(
        frames[1].event.as_ref().unwrap().event_type.as_deref(),
        Some("event-1")
    );
    assert_eq!(frames[1].cursor, bytes(1));
    assert!(observer_events.try_recv().is_err());
}
