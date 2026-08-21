use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use futures::StreamExt;
use prost::bytes::Bytes;
use proto::ledger_service_server::LedgerService;
use proto::ledger_service_server::LedgerServiceServer;
use proto::subscription_service_server::SubscriptionService;
use proto::subscription_service_server::SubscriptionServiceServer;
use sui_rpc::Client;
use sui_rpc::client::ListConfig;
use sui_rpc::proto::sui::rpc::v2 as proto;
use tokio::sync::mpsc;
use tonic::codegen::BoxStream;
use tower::ServiceBuilder;

pub(crate) const STREAM_DROP_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Clone, Debug)]
pub(crate) struct HttpObservation {
    pub(crate) path: String,
}

#[derive(Clone)]
pub(crate) struct CapturedRequest<T> {
    pub(crate) body: T,
}

pub(crate) enum StreamScript<T> {
    DispatchError(tonic::Status),
    Frames(Vec<Result<T, tonic::Status>>),
    Channel(mpsc::UnboundedReceiver<Result<T, tonic::Status>>),
}

impl<T> StreamScript<T> {
    pub(crate) fn frames(frames: impl IntoIterator<Item = Result<T, tonic::Status>>) -> Self {
        Self::Frames(frames.into_iter().collect())
    }
}

#[derive(Default)]
pub(crate) struct ScriptState {
    pub(crate) service_infos: VecDeque<Result<proto::GetServiceInfoResponse, tonic::Status>>,
    pub(crate) list_checkpoints: VecDeque<StreamScript<proto::ListCheckpointsResponse>>,
    pub(crate) list_transactions: VecDeque<StreamScript<proto::ListTransactionsResponse>>,
    pub(crate) list_events: VecDeque<StreamScript<proto::ListEventsResponse>>,
    pub(crate) subscribe_checkpoints: VecDeque<StreamScript<proto::SubscribeCheckpointsResponse>>,
    pub(crate) subscribe_transactions: VecDeque<StreamScript<proto::SubscribeTransactionsResponse>>,
    pub(crate) subscribe_events: VecDeque<StreamScript<proto::SubscribeEventsResponse>>,
    pub(crate) response_delay: Duration,
    pub(crate) service_info_requests: Vec<CapturedRequest<proto::GetServiceInfoRequest>>,
    pub(crate) checkpoint_requests: Vec<CapturedRequest<proto::ListCheckpointsRequest>>,
    pub(crate) transaction_requests: Vec<CapturedRequest<proto::ListTransactionsRequest>>,
    pub(crate) event_requests: Vec<CapturedRequest<proto::ListEventsRequest>>,
    pub(crate) checkpoint_subscriptions: Vec<CapturedRequest<proto::SubscribeCheckpointsRequest>>,
    pub(crate) transaction_subscriptions: Vec<CapturedRequest<proto::SubscribeTransactionsRequest>>,
    pub(crate) event_subscriptions: Vec<CapturedRequest<proto::SubscribeEventsRequest>>,
    pub(crate) calls: Vec<&'static str>,
}

#[derive(Clone)]
pub(crate) struct ScriptedStreamServer {
    pub(crate) state: Arc<Mutex<ScriptState>>,
    pub(crate) call_tx: mpsc::UnboundedSender<&'static str>,
}

impl ScriptedStreamServer {
    pub(crate) fn new() -> (Self, mpsc::UnboundedReceiver<&'static str>) {
        let (call_tx, call_rx) = mpsc::unbounded_channel();
        (
            Self {
                state: Arc::new(Mutex::new(ScriptState::default())),
                call_tx,
            },
            call_rx,
        )
    }

    pub(crate) fn push_checkpoint_lists(
        &self,
        scripts: impl IntoIterator<Item = StreamScript<proto::ListCheckpointsResponse>>,
    ) {
        self.state.lock().unwrap().list_checkpoints.extend(scripts);
    }

    pub(crate) fn push_transaction_lists(
        &self,
        scripts: impl IntoIterator<Item = StreamScript<proto::ListTransactionsResponse>>,
    ) {
        self.state.lock().unwrap().list_transactions.extend(scripts);
    }

    pub(crate) fn push_event_lists(
        &self,
        scripts: impl IntoIterator<Item = StreamScript<proto::ListEventsResponse>>,
    ) {
        self.state.lock().unwrap().list_events.extend(scripts);
    }
}

pub(crate) fn scripted_response<T: Send + 'static>(
    script: StreamScript<T>,
) -> Result<tonic::Response<BoxStream<T>>, tonic::Status> {
    match script {
        StreamScript::DispatchError(status) => Err(status),
        StreamScript::Frames(frames) => Ok(tonic::Response::new(Box::pin(futures::stream::iter(
            frames,
        )))),
        StreamScript::Channel(receiver) => {
            let stream = futures::stream::unfold(receiver, |mut receiver| async move {
                receiver.recv().await.map(|item| (item, receiver))
            });
            Ok(tonic::Response::new(Box::pin(stream)))
        }
    }
}

#[tonic::async_trait]
impl LedgerService for ScriptedStreamServer {
    async fn get_service_info(
        &self,
        request: tonic::Request<proto::GetServiceInfoRequest>,
    ) -> Result<tonic::Response<proto::GetServiceInfoResponse>, tonic::Status> {
        let (script, response_delay) = {
            let mut state = self.state.lock().unwrap();
            state.calls.push("get_service_info");
            state.service_info_requests.push(CapturedRequest {
                body: request.into_inner(),
            });
            let script = state
                .service_infos
                .pop_front()
                .expect("missing get_service_info script");
            (script, state.response_delay)
        };
        let _ = self.call_tx.send("get_service_info");
        if !response_delay.is_zero() {
            tokio::time::sleep(response_delay).await;
        }
        script.map(tonic::Response::new)
    }

    async fn list_checkpoints(
        &self,
        request: tonic::Request<proto::ListCheckpointsRequest>,
    ) -> Result<tonic::Response<BoxStream<proto::ListCheckpointsResponse>>, tonic::Status> {
        let (script, response_delay) = {
            let mut state = self.state.lock().unwrap();
            state.calls.push("list_checkpoints");
            state.checkpoint_requests.push(CapturedRequest {
                body: request.into_inner(),
            });
            let script = state
                .list_checkpoints
                .pop_front()
                .expect("missing list_checkpoints script");
            (script, state.response_delay)
        };
        let _ = self.call_tx.send("list_checkpoints");
        if !response_delay.is_zero() {
            tokio::time::sleep(response_delay).await;
        }
        scripted_response(script)
    }

    async fn list_transactions(
        &self,
        request: tonic::Request<proto::ListTransactionsRequest>,
    ) -> Result<tonic::Response<BoxStream<proto::ListTransactionsResponse>>, tonic::Status> {
        let (script, response_delay) = {
            let mut state = self.state.lock().unwrap();
            state.calls.push("list_transactions");
            state.transaction_requests.push(CapturedRequest {
                body: request.into_inner(),
            });
            let script = state
                .list_transactions
                .pop_front()
                .expect("missing list_transactions script");
            (script, state.response_delay)
        };
        let _ = self.call_tx.send("list_transactions");
        if !response_delay.is_zero() {
            tokio::time::sleep(response_delay).await;
        }
        scripted_response(script)
    }

    async fn list_events(
        &self,
        request: tonic::Request<proto::ListEventsRequest>,
    ) -> Result<tonic::Response<BoxStream<proto::ListEventsResponse>>, tonic::Status> {
        let (script, response_delay) = {
            let mut state = self.state.lock().unwrap();
            state.calls.push("list_events");
            state.event_requests.push(CapturedRequest {
                body: request.into_inner(),
            });
            let script = state
                .list_events
                .pop_front()
                .expect("missing list_events script");
            (script, state.response_delay)
        };
        let _ = self.call_tx.send("list_events");
        if !response_delay.is_zero() {
            tokio::time::sleep(response_delay).await;
        }
        scripted_response(script)
    }
}

#[tonic::async_trait]
impl SubscriptionService for ScriptedStreamServer {
    async fn subscribe_checkpoints(
        &self,
        request: tonic::Request<proto::SubscribeCheckpointsRequest>,
    ) -> Result<tonic::Response<BoxStream<proto::SubscribeCheckpointsResponse>>, tonic::Status>
    {
        let (script, response_delay) = {
            let mut state = self.state.lock().unwrap();
            state.calls.push("subscribe_checkpoints");
            state.checkpoint_subscriptions.push(CapturedRequest {
                body: request.into_inner(),
            });
            let script = state
                .subscribe_checkpoints
                .pop_front()
                .expect("missing subscribe_checkpoints script");
            (script, state.response_delay)
        };
        let _ = self.call_tx.send("subscribe_checkpoints");
        if !response_delay.is_zero() {
            tokio::time::sleep(response_delay).await;
        }
        scripted_response(script)
    }

    async fn subscribe_transactions(
        &self,
        request: tonic::Request<proto::SubscribeTransactionsRequest>,
    ) -> Result<tonic::Response<BoxStream<proto::SubscribeTransactionsResponse>>, tonic::Status>
    {
        let (script, response_delay) = {
            let mut state = self.state.lock().unwrap();
            state.calls.push("subscribe_transactions");
            state.transaction_subscriptions.push(CapturedRequest {
                body: request.into_inner(),
            });
            let script = state
                .subscribe_transactions
                .pop_front()
                .expect("missing subscribe_transactions script");
            (script, state.response_delay)
        };
        let _ = self.call_tx.send("subscribe_transactions");
        if !response_delay.is_zero() {
            tokio::time::sleep(response_delay).await;
        }
        scripted_response(script)
    }

    async fn subscribe_events(
        &self,
        request: tonic::Request<proto::SubscribeEventsRequest>,
    ) -> Result<tonic::Response<BoxStream<proto::SubscribeEventsResponse>>, tonic::Status> {
        let (script, response_delay) = {
            let mut state = self.state.lock().unwrap();
            state.calls.push("subscribe_events");
            state.event_subscriptions.push(CapturedRequest {
                body: request.into_inner(),
            });
            let script = state
                .subscribe_events
                .pop_front()
                .expect("missing subscribe_events script");
            (script, state.response_delay)
        };
        let _ = self.call_tx.send("subscribe_events");
        if !response_delay.is_zero() {
            tokio::time::sleep(response_delay).await;
        }
        scripted_response(script)
    }
}

pub(crate) async fn spawn_server(server: ScriptedStreamServer) -> std::net::SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind scripted server listener");
    let address = listener.local_addr().expect("scripted server address");
    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(LedgerServiceServer::new(server.clone()))
            .add_service(SubscriptionServiceServer::new(server))
            .serve_with_incoming(tonic::transport::server::TcpIncoming::from(listener))
            .await
            .expect("scripted server failed");
    });
    tokio::task::yield_now().await;
    address
}

pub(crate) fn observed_client(
    address: std::net::SocketAddr,
) -> (Client, Arc<Mutex<Vec<HttpObservation>>>) {
    let observations = Arc::new(Mutex::new(Vec::new()));
    let layer_observations = observations.clone();
    let layer = ServiceBuilder::new().map_request(move |request: http::Request<_>| {
        layer_observations.lock().unwrap().push(HttpObservation {
            path: request.uri().path().to_owned(),
        });
        request
    });
    let client = Client::new(format!("http://{address}"))
        .expect("scripted client")
        .request_layer(layer);
    (client, observations)
}

pub(crate) async fn next_scripted_call(
    calls: &mut mpsc::UnboundedReceiver<&'static str>,
) -> &'static str {
    tokio::time::timeout(Duration::from_secs(2), calls.recv())
        .await
        .expect("timed out waiting for scripted RPC call")
        .expect("scripted RPC call channel closed")
}

pub(crate) fn bytes(value: u64) -> Bytes {
    Bytes::from(format!("c{value}"))
}

pub(crate) fn watermark(value: u64) -> proto::Watermark {
    let mut watermark = proto::Watermark::default();
    watermark.cursor = Some(bytes(value));
    watermark.checkpoint = Some(value);
    watermark
}
pub(crate) fn watermark_at(cursor: u64, checkpoint: u64) -> proto::Watermark {
    let mut watermark = watermark(cursor);
    watermark.checkpoint = Some(checkpoint);
    watermark
}

pub(crate) fn query_end(reason: proto::QueryEndReason) -> proto::QueryEnd {
    let mut end = proto::QueryEnd::default();
    end.reason = Some(reason as i32);
    end
}

pub(crate) fn checkpoint(value: u64) -> proto::Checkpoint {
    let mut checkpoint = proto::Checkpoint::default();
    checkpoint.sequence_number = Some(value);
    checkpoint
}

pub(crate) fn transaction(value: u64) -> proto::ExecutedTransaction {
    let mut transaction = proto::ExecutedTransaction::default();
    transaction.digest = Some(format!("tx-{value}"));
    transaction.checkpoint = Some(value);
    transaction.transaction_index = Some(0);
    transaction
}

pub(crate) fn event(value: u64) -> proto::Event {
    let mut event = proto::Event::default();
    event.event_type = Some(format!("event-{value}"));
    event.checkpoint = Some(value);
    event.transaction_index = Some(0);
    event.event_index = Some(0);
    event
}
pub(crate) fn checkpoint_list_frame(
    item: Option<u64>,
    cursor: u64,
    reason: Option<proto::QueryEndReason>,
) -> proto::ListCheckpointsResponse {
    let mut response = proto::ListCheckpointsResponse::default();
    response.checkpoint = item.map(checkpoint);
    response.watermark = Some(watermark(cursor));
    response.end = reason.map(query_end);
    response
}

pub(crate) fn transaction_list_frame(
    item: Option<u64>,
    cursor: u64,
    reason: Option<proto::QueryEndReason>,
) -> proto::ListTransactionsResponse {
    let mut response = proto::ListTransactionsResponse::default();
    response.transaction = item.map(transaction);
    response.watermark = Some(watermark(cursor));
    response.end = reason.map(query_end);
    response
}

pub(crate) fn event_list_frame(
    item: Option<u64>,
    cursor: u64,
    reason: Option<proto::QueryEndReason>,
) -> proto::ListEventsResponse {
    let mut response = proto::ListEventsResponse::default();
    response.event = item.map(event);
    response.watermark = Some(watermark(cursor));
    response.end = reason.map(query_end);
    response
}

pub(crate) fn event_positioned_list_frame(
    item: Option<proto::Event>,
    cursor: u64,
    checkpoint: u64,
    reason: Option<proto::QueryEndReason>,
) -> proto::ListEventsResponse {
    let mut response = proto::ListEventsResponse::default();
    response.event = item;
    response.watermark = Some(watermark_at(cursor, checkpoint));
    response.end = reason.map(query_end);
    response
}
pub(crate) fn bounded_checkpoint_scripts() -> Vec<StreamScript<proto::ListCheckpointsResponse>> {
    vec![
        StreamScript::frames([
            Ok(checkpoint_list_frame(Some(2), 2, None)),
            Ok(checkpoint_list_frame(
                Some(3),
                3,
                Some(proto::QueryEndReason::ItemLimit),
            )),
        ]),
        StreamScript::frames([
            Ok(checkpoint_list_frame(None, 5, None)),
            Ok(checkpoint_list_frame(Some(6), 6, None)),
            Ok(checkpoint_list_frame(Some(7), 7, None)),
            Ok(checkpoint_list_frame(
                None,
                7,
                Some(proto::QueryEndReason::LedgerTip),
            )),
        ]),
    ]
}

pub(crate) fn bounded_transaction_scripts() -> Vec<StreamScript<proto::ListTransactionsResponse>> {
    vec![
        StreamScript::frames([
            Ok(transaction_list_frame(Some(2), 2, None)),
            Ok(transaction_list_frame(
                Some(3),
                3,
                Some(proto::QueryEndReason::ItemLimit),
            )),
        ]),
        StreamScript::frames([
            Ok(transaction_list_frame(None, 5, None)),
            Ok(transaction_list_frame(Some(6), 6, None)),
            Ok(transaction_list_frame(Some(7), 7, None)),
            Ok(transaction_list_frame(
                None,
                7,
                Some(proto::QueryEndReason::LedgerTip),
            )),
        ]),
    ]
}

pub(crate) fn bounded_event_scripts() -> Vec<StreamScript<proto::ListEventsResponse>> {
    vec![
        StreamScript::frames([
            Ok(event_list_frame(Some(2), 2, None)),
            Ok(event_list_frame(
                Some(3),
                3,
                Some(proto::QueryEndReason::ItemLimit),
            )),
        ]),
        StreamScript::frames([
            Ok(event_list_frame(None, 5, None)),
            Ok(event_list_frame(Some(6), 6, None)),
            Ok(event_list_frame(Some(7), 7, None)),
            Ok(event_list_frame(
                None,
                7,
                Some(proto::QueryEndReason::LedgerTip),
            )),
        ]),
    ]
}
pub(crate) fn checkpoint_identity_mask() -> prost_types::FieldMask {
    prost_types::FieldMask {
        paths: vec!["sequence_number".to_owned()],
    }
}

pub(crate) fn transaction_identity_mask() -> prost_types::FieldMask {
    prost_types::FieldMask {
        paths: vec!["checkpoint".to_owned(), "transaction_index".to_owned()],
    }
}

pub(crate) fn event_identity_mask() -> prost_types::FieldMask {
    prost_types::FieldMask {
        paths: vec![
            "checkpoint".to_owned(),
            "transaction_index".to_owned(),
            "event_index".to_owned(),
        ],
    }
}

pub(crate) fn fast_list_config() -> ListConfig {
    let mut config = ListConfig::default();
    config.base_retry_delay = Duration::ZERO;
    config.max_retry_delay = Duration::ZERO;
    config.retry_jitter = Duration::ZERO;
    config
}

pub(crate) fn bounded_event_request() -> proto::ListEventsRequest {
    proto::ListEventsRequest::default()
        .with_read_mask(event_identity_mask())
        .with_end_checkpoint(10)
}
pub(crate) async fn first_list_event_error(
    client: &Client,
    request_body: proto::ListEventsRequest,
    config: ListConfig,
) -> tonic::Status {
    let stream = client.list_events_with_config(request_body, config);
    futures::pin_mut!(stream);
    let status = tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .expect("timed out waiting for terminal List error")
        .unwrap()
        .unwrap_err();
    assert!(
        tokio::time::timeout(Duration::from_secs(2), stream.next())
            .await
            .expect("timed out waiting for List termination")
            .is_none()
    );
    status
}
