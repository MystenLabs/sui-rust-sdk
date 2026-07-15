//! Client-side watchdog for response bodies.
//!
//! Every RPC made through a [`Client`](super::Client) is multiplexed over a
//! single HTTP/2 connection whose connection-level receive window is shared
//! by all in-flight responses. Response data counts against that window until
//! the application polls it out of the stream, so a streaming response that
//! is held without being polled pins up to a full stream window of the shared
//! budget. Enough stalled streams starve the connection, and then every RPC
//! on the channel hangs indefinitely while TCP and HTTP/2 keepalives stay
//! healthy. This converts into a permanent hang whenever the task that should
//! be polling a stream is itself blocked on another call on the same starved
//! channel: every party waits for connection window while holding connection
//! window.
//!
//! The watchdog breaks both the starvation and the deadlock. Each response
//! body is moved into a spawned task and bridged back to the caller through a
//! bounded channel. The task's outermost await is an idle timer, so it stays
//! responsive even when the caller has parked the response and nothing is
//! polling it. If a whole idle period passes without a frame being delivered
//! to the caller, the task drops the inner body, which resets the stream
//! (RST_STREAM is a control frame and is deliverable even when the connection
//! window is exhausted) and releases every byte of window the stream had
//! pinned. The caller observes a `DeadlineExceeded` status on its next poll.
//!
//! A timer raced against the caller's own polls would not be enough: a parked
//! stream is never polled, so a poll-driven timer freezes exactly when it is
//! needed. The spawned task gives the timer its own polling root.
//!
//! The watchdog task also enforces the request's `grpc-timeout` deadline (set
//! with [`tonic::Request::set_timeout`]) across the response body. tonic's
//! own channel machinery only enforces that header until response headers
//! arrive, which on a starved connection is precisely the part that still
//! works -- headers are not flow-controlled -- so without this the deadline
//! never bounds the actual hang. With it, `set_timeout` is a total per-call
//! deadline: the same value is enforced locally end to end and communicated
//! to the server through the standard header.

use std::future::Future;
use std::ops::ControlFlow;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;

use bytes::Bytes;
use http_body::Body as _;
use http_body::Frame;
use tonic::Status;
use tonic::body::Body;
use tower::Layer;
use tower::Service;

/// Default maximum time between response-body progress events before the
/// watchdog resets the stream. Comfortably above the fullnode's subscription
/// watermark cadence (~5s), so a healthy subscription always makes progress
/// well within it.
pub(super) const DEFAULT_BODY_IDLE_TIMEOUT: Duration = Duration::from_secs(30);

/// Per-request override for the client's idle-body watchdog.
///
/// Insert it into a request's extensions to raise, lower, or disable the
/// watchdog timeout for that call only:
///
/// ```
/// use std::time::Duration;
/// use sui_rpc::client::BodyIdleTimeout;
/// use sui_rpc::proto::sui::rpc::v2::SubscribeCheckpointsRequest;
///
/// let mut request = tonic::Request::new(SubscribeCheckpointsRequest::default());
/// request
///     .extensions_mut()
///     .insert(BodyIdleTimeout::new(Duration::from_secs(120)));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct BodyIdleTimeout(Option<Duration>);

impl BodyIdleTimeout {
    /// Override the watchdog's idle timeout for this request.
    pub fn new(timeout: Duration) -> Self {
        Self(Some(timeout))
    }

    /// Disable the watchdog for this request. The response body can then
    /// stall indefinitely; only use this when the caller enforces its own
    /// bound on the call.
    pub fn disabled() -> Self {
        Self(None)
    }
}

/// [`Layer`] that applies [`Watchdog`] to the client's transport.
#[derive(Clone)]
pub(super) struct WatchdogLayer {
    idle_timeout: Option<Duration>,
}

impl WatchdogLayer {
    pub(super) fn new(idle_timeout: Option<Duration>) -> Self {
        Self { idle_timeout }
    }
}

impl<S> Layer<S> for WatchdogLayer {
    type Service = Watchdog<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Watchdog {
            inner,
            idle_timeout: self.idle_timeout,
        }
    }
}

/// Service that moves each response body into a watchdog task (see the
/// module documentation).
pub(super) struct Watchdog<S> {
    inner: S,
    idle_timeout: Option<Duration>,
}

impl<S> Service<http::Request<Body>> for Watchdog<S>
where
    S: Service<http::Request<Body>, Response = http::Response<Body>>,
    S::Future: Send + 'static,
    S::Error: Send + 'static,
{
    type Response = http::Response<Body>;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: http::Request<Body>) -> Self::Future {
        let idle_timeout = match request.extensions().get::<BodyIdleTimeout>() {
            Some(BodyIdleTimeout(override_timeout)) => *override_timeout,
            None => self.idle_timeout,
        };
        // The deadline starts at request time, so the budget the header
        // communicates to the server is the same one enforced locally.
        let deadline = parse_grpc_timeout(request.headers())
            .map(|timeout| tokio::time::Instant::now() + timeout);

        let response = self.inner.call(request);
        Box::pin(async move {
            let response = response.await?;
            if idle_timeout.is_none() && deadline.is_none() {
                return Ok(response);
            }
            Ok(response.map(|body| Body::new(WatchdogBody::spawn(body, idle_timeout, deadline))))
        })
    }
}

/// Parse the standard `grpc-timeout` request header (1-8 ASCII digits
/// followed by a unit, per the gRPC-over-HTTP/2 spec). Malformed values are
/// ignored, matching tonic's own leniency, since failing the call over an
/// unenforceable header would be worse than not enforcing it.
fn parse_grpc_timeout(headers: &http::HeaderMap) -> Option<Duration> {
    let value = headers.get("grpc-timeout")?.to_str().ok()?;
    let (magnitude, unit) = value.split_at(value.len().checked_sub(1)?);
    if magnitude.is_empty() || magnitude.len() > 8 || !magnitude.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let magnitude: u64 = magnitude.parse().ok()?;
    match unit {
        "H" => Some(Duration::from_secs(magnitude * 60 * 60)),
        "M" => Some(Duration::from_secs(magnitude * 60)),
        "S" => Some(Duration::from_secs(magnitude)),
        "m" => Some(Duration::from_millis(magnitude)),
        "u" => Some(Duration::from_micros(magnitude)),
        "n" => Some(Duration::from_nanos(magnitude)),
        _ => None,
    }
}

/// The caller-facing half of the bridge: receives frames from the watchdog
/// task over a bounded channel and aborts the task if dropped, so dropping a
/// response still resets the underlying stream promptly.
struct WatchdogBody {
    rx: tokio::sync::mpsc::Receiver<Result<Frame<Bytes>, Status>>,
    /// `Some` until the task's completion has been observed. Consulted when
    /// the channel closes without a terminal status so that a panic in the
    /// watchdog task surfaces as an error instead of a clean end-of-stream.
    task: Option<tokio::task::JoinHandle<()>>,
}

impl WatchdogBody {
    fn spawn(
        body: Body,
        idle_timeout: Option<Duration>,
        deadline: Option<tokio::time::Instant>,
    ) -> Self {
        // Capacity 1: the bridge only needs to decouple the two polling
        // roots, and the tightest bound keeps the transport's own
        // flow-control backpressure intact.
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let task = tokio::spawn(drive(body, tx, idle_timeout, deadline));
        Self {
            rx,
            task: Some(task),
        }
    }
}

impl Drop for WatchdogBody {
    fn drop(&mut self) {
        if let Some(task) = &self.task {
            task.abort();
        }
    }
}

impl http_body::Body for WatchdogBody {
    type Data = Bytes;
    type Error = Status;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Status>>> {
        let this = &mut *self;
        match this.rx.poll_recv(cx) {
            Poll::Ready(Some(item)) => Poll::Ready(Some(item)),
            Poll::Ready(None) => {
                let Some(task) = &mut this.task else {
                    return Poll::Ready(None);
                };
                match Pin::new(task).poll(cx) {
                    Poll::Ready(result) => {
                        this.task = None;
                        match result {
                            Err(e) if e.is_panic() => Poll::Ready(Some(Err(Status::internal(
                                "client idle-body watchdog task panicked",
                            )))),
                            // A clean exit, or cancellation via our own
                            // abort-on-drop (not observable here since we
                            // are being polled, not dropped).
                            Ok(()) | Err(_) => Poll::Ready(None),
                        }
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// The watchdog task: pump frames from the transport to the bridge channel,
/// bounding each unit of progress with the idle timer and the whole body
/// with the request deadline. On expiry, drop the body first (resetting the
/// stream and releasing its pinned flow-control window in real time), then
/// deliver the terminal status at the consumer's pace.
async fn drive(
    body: Body,
    tx: tokio::sync::mpsc::Sender<Result<Frame<Bytes>, Status>>,
    idle_timeout: Option<Duration>,
    deadline: Option<tokio::time::Instant>,
) {
    let pump = pump(body, tx.clone(), idle_timeout);
    let cut = match deadline {
        Some(deadline) => tokio::time::timeout_at(deadline, pump)
            .await
            .unwrap_or_else(|_| {
                Some(Status::deadline_exceeded(
                    "grpc-timeout deadline exceeded before the response body completed; \
                     stream reset by the client",
                ))
            }),
        None => pump.await,
    };
    if let Some(status) = cut {
        let _ = tx.send(Err(status)).await;
    }
}

/// Returns `Some(status)` if the stream was cut by the watchdog, and `None`
/// on natural end-of-stream, transport error, or consumer hang-up. The inner
/// body is dropped on return in all cases -- including cancellation by the
/// deadline in [`drive`], which drops this future and the body it owns.
async fn pump(
    mut body: Body,
    tx: tokio::sync::mpsc::Sender<Result<Frame<Bytes>, Status>>,
    idle_timeout: Option<Duration>,
) -> Option<Status> {
    loop {
        // One unit of progress: wait for bridge capacity, then receive a
        // frame from the transport and hand it over through the permit.
        // Reserving before polling the body means a frame is only taken from
        // the transport once it is immediately deliverable: nothing sits in
        // limbo between the transport and the bridge, and the frame's
        // receive-window capacity is not released (inviting the server to
        // send more) until the consumer can actually accept it. The timer
        // covers both awaits, so it fires both when the consumer has parked
        // the response without polling it (reserve never resolves) and when
        // the transport delivers nothing (a starved or dead connection).
        let step = async {
            let Ok(permit) = tx.reserve().await else {
                // Consumer dropped the response.
                return ControlFlow::Break(());
            };
            match std::future::poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await {
                Some(Ok(frame)) => {
                    permit.send(Ok(frame));
                    ControlFlow::Continue(())
                }
                Some(Err(status)) => {
                    permit.send(Err(status));
                    ControlFlow::Break(())
                }
                None => ControlFlow::Break(()),
            }
        };

        let flow = match idle_timeout {
            Some(idle_timeout) => match tokio::time::timeout(idle_timeout, step).await {
                Ok(flow) => flow,
                Err(_) => {
                    return Some(Status::deadline_exceeded(format!(
                        "no response-body progress within {idle_timeout:?}; \
                         stream reset by the client's idle-body watchdog"
                    )));
                }
            },
            None => step.await,
        };
        match flow {
            ControlFlow::Continue(()) => {}
            ControlFlow::Break(()) => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    /// Test body driven by an unbounded channel, holding an optional beacon
    /// whose `Drop` proves when the watchdog task has dropped the body.
    struct TestBody {
        rx: tokio::sync::mpsc::UnboundedReceiver<Result<Frame<Bytes>, Status>>,
        _beacon: Option<DropBeacon>,
    }

    struct DropBeacon(Arc<AtomicBool>);
    impl Drop for DropBeacon {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    impl http_body::Body for TestBody {
        type Data = Bytes;
        type Error = Status;

        fn poll_frame(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Bytes>, Status>>> {
            self.rx.poll_recv(cx)
        }
    }

    #[allow(clippy::type_complexity)]
    fn test_body() -> (
        tokio::sync::mpsc::UnboundedSender<Result<Frame<Bytes>, Status>>,
        Arc<AtomicBool>,
        Body,
    ) {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let dropped = Arc::new(AtomicBool::new(false));
        let body = Body::new(TestBody {
            rx,
            _beacon: Some(DropBeacon(dropped.clone())),
        });
        (tx, dropped, body)
    }

    async fn next_frame(body: &mut WatchdogBody) -> Option<Result<Frame<Bytes>, Status>> {
        std::future::poll_fn(|cx| Pin::new(&mut *body).poll_frame(cx)).await
    }

    fn data_frame(bytes: &'static [u8]) -> Frame<Bytes> {
        Frame::data(Bytes::from_static(bytes))
    }

    /// `start_paused = true` makes `tokio::time` virtual: sleeps advance only
    /// when the runtime is otherwise idle, so these tests run instantly and
    /// deterministically.
    #[tokio::test(start_paused = true)]
    async fn frames_and_trailers_pass_through() {
        let (tx, _dropped, body) = test_body();
        let mut watched = WatchdogBody::spawn(body, Some(Duration::from_secs(30)), None);

        tx.send(Ok(data_frame(b"hello"))).unwrap();
        let frame = next_frame(&mut watched).await.unwrap().unwrap();
        assert_eq!(frame.into_data().unwrap(), Bytes::from_static(b"hello"));

        let mut trailers = http::HeaderMap::new();
        trailers.insert("grpc-status", "0".parse().unwrap());
        tx.send(Ok(Frame::trailers(trailers.clone()))).unwrap();
        let frame = next_frame(&mut watched).await.unwrap().unwrap();
        assert_eq!(frame.into_trailers().unwrap(), trailers);

        drop(tx);
        assert!(next_frame(&mut watched).await.is_none());
    }

    #[tokio::test(start_paused = true)]
    async fn slow_but_live_stream_is_not_cut() {
        let (tx, _dropped, body) = test_body();
        let mut watched = WatchdogBody::spawn(body, Some(Duration::from_secs(30)), None);

        for _ in 0..5 {
            tokio::time::sleep(Duration::from_secs(20)).await;
            tx.send(Ok(data_frame(b"tick"))).unwrap();
            let frame = next_frame(&mut watched).await.unwrap().unwrap();
            assert_eq!(frame.into_data().unwrap(), Bytes::from_static(b"tick"));
        }

        drop(tx);
        assert!(next_frame(&mut watched).await.is_none());
    }

    #[tokio::test(start_paused = true)]
    async fn idle_transport_is_cut_and_body_dropped() {
        let (_tx, dropped, body) = test_body();
        let mut watched = WatchdogBody::spawn(body, Some(Duration::from_secs(30)), None);

        let status = next_frame(&mut watched).await.unwrap().unwrap_err();
        assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
        assert!(dropped.load(Ordering::SeqCst), "inner body was not dropped");
        assert!(next_frame(&mut watched).await.is_none());
    }

    /// The core starvation defense: when the consumer parks the response
    /// without polling it, the watchdog must still fire and drop the inner
    /// body (releasing its pinned flow-control window) in wall-clock time,
    /// not on the consumer's next poll.
    #[tokio::test(start_paused = true)]
    async fn parked_consumer_is_cut_without_being_polled() {
        let (tx, dropped, body) = test_body();
        let mut watched = WatchdogBody::spawn(body, Some(Duration::from_secs(30)), None);

        // The consumer takes one frame, then parks the response entirely.
        tx.send(Ok(data_frame(b"first"))).unwrap();
        assert!(next_frame(&mut watched).await.unwrap().is_ok());

        // The transport keeps delivering: one frame sits in the bridge
        // channel and the pump waits for bridge capacity before taking the
        // next one.
        tx.send(Ok(data_frame(b"buffered"))).unwrap();
        tx.send(Ok(data_frame(b"blocked"))).unwrap();

        // Without any consumer poll, the idle timer must drop the body.
        tokio::time::sleep(Duration::from_secs(60)).await;
        assert!(
            dropped.load(Ordering::SeqCst),
            "inner body was not dropped while the consumer was parked"
        );

        // When the consumer returns it drains the buffered frame, then
        // observes the watchdog cut.
        let frame = next_frame(&mut watched).await.unwrap().unwrap();
        assert_eq!(frame.into_data().unwrap(), Bytes::from_static(b"buffered"));
        let status = next_frame(&mut watched).await.unwrap().unwrap_err();
        assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
    }

    /// Reserve-before-pull: the pump takes a frame from the transport only
    /// once the bridge can accept it, so a parked consumer leaves at most one
    /// frame buffered in the bridge and everything else in the transport,
    /// where it remains subject to HTTP/2 flow control.
    #[tokio::test(start_paused = true)]
    async fn parked_consumer_does_not_double_buffer() {
        use std::sync::atomic::AtomicU64;

        struct CountingBody {
            rx: tokio::sync::mpsc::UnboundedReceiver<Result<Frame<Bytes>, Status>>,
            pulled: Arc<AtomicU64>,
        }
        impl http_body::Body for CountingBody {
            type Data = Bytes;
            type Error = Status;

            fn poll_frame(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Option<Result<Frame<Bytes>, Status>>> {
                let poll = self.rx.poll_recv(cx);
                if matches!(poll, Poll::Ready(Some(_))) {
                    self.pulled.fetch_add(1, Ordering::SeqCst);
                }
                poll
            }
        }

        let pulled = Arc::new(AtomicU64::new(0));
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let body = Body::new(CountingBody {
            rx,
            pulled: pulled.clone(),
        });
        let mut watched = WatchdogBody::spawn(body, Some(Duration::from_secs(30)), None);

        // The transport has plenty of frames available.
        for _ in 0..5 {
            tx.send(Ok(data_frame(b"tick"))).unwrap();
        }

        // The consumer takes one frame and parks. The pump may refill the
        // one-slot bridge, but must not pull frames it cannot deliver.
        assert!(next_frame(&mut watched).await.unwrap().is_ok());
        tokio::time::sleep(Duration::from_secs(60)).await;
        assert_eq!(
            pulled.load(Ordering::SeqCst),
            2,
            "the pump pulled frames from the transport that it could not deliver"
        );

        // The consumer still sees the buffered frame and then the cut.
        assert!(next_frame(&mut watched).await.unwrap().is_ok());
        let status = next_frame(&mut watched).await.unwrap().unwrap_err();
        assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
    }

    #[tokio::test(start_paused = true)]
    async fn transport_error_passes_through() {
        let (tx, _dropped, body) = test_body();
        let mut watched = WatchdogBody::spawn(body, Some(Duration::from_secs(30)), None);

        tx.send(Err(Status::unavailable("connection reset")))
            .unwrap();
        let status = next_frame(&mut watched).await.unwrap().unwrap_err();
        assert_eq!(status.code(), tonic::Code::Unavailable);
        assert!(next_frame(&mut watched).await.is_none());
    }

    #[tokio::test(start_paused = true)]
    async fn dropping_response_aborts_task_and_drops_body() {
        let (tx, dropped, body) = test_body();
        let watched = WatchdogBody::spawn(body, Some(Duration::from_secs(30)), None);
        tx.send(Ok(data_frame(b"unread"))).unwrap();

        drop(watched);
        for _ in 0..10 {
            tokio::task::yield_now().await;
            if dropped.load(Ordering::SeqCst) {
                break;
            }
        }
        assert!(
            dropped.load(Ordering::SeqCst),
            "inner body was not dropped after the response was dropped"
        );
    }

    /// A panic in the watchdog task must surface as an error instead of
    /// silently closing the bridge, which the consumer could not distinguish
    /// from a clean end-of-stream.
    #[tokio::test(start_paused = true)]
    async fn task_panic_surfaces_as_internal() {
        struct PanicBody;
        impl http_body::Body for PanicBody {
            type Data = Bytes;
            type Error = Status;

            fn poll_frame(
                self: Pin<&mut Self>,
                _cx: &mut Context<'_>,
            ) -> Poll<Option<Result<Frame<Bytes>, Status>>> {
                panic!("boom from inner body");
            }
        }

        let mut watched =
            WatchdogBody::spawn(Body::new(PanicBody), Some(Duration::from_secs(30)), None);
        let status = next_frame(&mut watched).await.unwrap().unwrap_err();
        assert_eq!(status.code(), tonic::Code::Internal);
    }

    /// The request deadline bounds the whole body even when every individual
    /// frame arrives well within the idle timeout.
    #[tokio::test(start_paused = true)]
    async fn deadline_cuts_a_live_stream() {
        let (tx, dropped, body) = test_body();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
        let mut watched =
            WatchdogBody::spawn(body, Some(Duration::from_secs(3600)), Some(deadline));

        for _ in 0..4 {
            tokio::time::sleep(Duration::from_secs(5)).await;
            tx.send(Ok(data_frame(b"tick"))).unwrap();
            assert!(next_frame(&mut watched).await.unwrap().is_ok());
        }

        // Past the deadline the body must be dropped and the consumer must
        // observe DeadlineExceeded, even though frames kept flowing.
        tokio::time::sleep(Duration::from_secs(15)).await;
        assert!(
            dropped.load(Ordering::SeqCst),
            "inner body was not dropped at the deadline"
        );
        let status = next_frame(&mut watched).await.unwrap().unwrap_err();
        assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
    }

    /// The deadline is enforced even when the idle watchdog is disabled.
    #[tokio::test(start_paused = true)]
    async fn deadline_applies_without_idle_timeout() {
        let (_tx, dropped, body) = test_body();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
        let mut watched = WatchdogBody::spawn(body, None, Some(deadline));

        let status = next_frame(&mut watched).await.unwrap().unwrap_err();
        assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
        assert!(dropped.load(Ordering::SeqCst), "inner body was not dropped");
    }

    /// End-to-end through the service: the `grpc-timeout` header (what
    /// `tonic::Request::set_timeout` produces) bounds the response body, and
    /// a `BodyIdleTimeout::disabled()` extension does not interfere with it.
    #[tokio::test(start_paused = true)]
    async fn service_honors_grpc_timeout_header() {
        let (_tx, dropped, body) = test_body();
        let mut body = Some(body);
        let mut service = WatchdogLayer::new(None).layer(tower::service_fn(
            move |_request: http::Request<Body>| {
                let body = body.take().unwrap();
                async move { Ok::<_, Status>(http::Response::new(body)) }
            },
        ));

        let mut request = http::Request::new(Body::empty());
        request
            .headers_mut()
            .insert("grpc-timeout", "30S".parse().unwrap());
        request.extensions_mut().insert(BodyIdleTimeout::disabled());

        let response = service.call(request).await.unwrap();
        let mut body = response.into_body();
        let item = std::future::poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await;
        let status = item.unwrap().unwrap_err();
        assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
        assert!(dropped.load(Ordering::SeqCst), "inner body was not dropped");
    }

    /// With neither an idle timeout nor a deadline, the response body must be
    /// passed through without spawning a bridge.
    #[tokio::test(start_paused = true)]
    async fn service_passes_body_through_when_unbounded() {
        let (tx, _dropped, body) = test_body();
        let mut body = Some(body);
        let mut service = WatchdogLayer::new(None).layer(tower::service_fn(
            move |_request: http::Request<Body>| {
                let body = body.take().unwrap();
                async move { Ok::<_, Status>(http::Response::new(body)) }
            },
        ));

        let response = service
            .call(http::Request::new(Body::empty()))
            .await
            .unwrap();
        let mut body = response.into_body();

        // Nothing cuts the stream no matter how long it idles.
        tokio::time::sleep(Duration::from_secs(3600)).await;
        tx.send(Ok(data_frame(b"still here"))).unwrap();
        let item = std::future::poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await;
        assert!(item.unwrap().is_ok());
    }

    #[test]
    fn grpc_timeout_parsing() {
        fn parse(value: &str) -> Option<Duration> {
            let mut headers = http::HeaderMap::new();
            headers.insert("grpc-timeout", value.parse().unwrap());
            parse_grpc_timeout(&headers)
        }

        assert_eq!(parse("2H"), Some(Duration::from_secs(2 * 60 * 60)));
        assert_eq!(parse("3M"), Some(Duration::from_secs(180)));
        assert_eq!(parse("30S"), Some(Duration::from_secs(30)));
        assert_eq!(parse("500m"), Some(Duration::from_millis(500)));
        assert_eq!(parse("250u"), Some(Duration::from_micros(250)));
        assert_eq!(parse("82n"), Some(Duration::from_nanos(82)));
        assert_eq!(parse("99999999S"), Some(Duration::from_secs(99_999_999)));

        // Malformed values are ignored rather than failing the call.
        assert_eq!(parse(""), None);
        assert_eq!(parse("S"), None);
        assert_eq!(parse("30"), None);
        assert_eq!(parse("30f"), None);
        assert_eq!(parse("-30S"), None);
        assert_eq!(parse("+30S"), None);
        assert_eq!(parse("3.5S"), None);
        assert_eq!(parse("123456789S"), None);
        assert_eq!(
            parse_grpc_timeout(&http::HeaderMap::new()),
            None,
            "absent header"
        );
    }
}
