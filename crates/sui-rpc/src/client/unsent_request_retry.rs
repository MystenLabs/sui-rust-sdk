//! Transparent retry of RPCs the server never processed, following gRPC's
//! transparent retry design (gRFC A6, "Transparent Retries").
//!
//! A server or load balancer that drains an HTTP/2 connection sends GOAWAY
//! carrying the id of the last stream it will process. Streams the client
//! opened above that id, and streams it tries to open once the GOAWAY has
//! arrived, fail without the server having processed them (RFC 9113 §6.8);
//! the same holds for a stream the server resets with REFUSED_STREAM (RFC
//! 9113 §8.7). Load balancers drain connections routinely (on a maximum
//! connection age or request count, or during deploys), so a long-lived
//! client hits this regularly. tonic reports these failures as `Internal` or
//! `Unavailable` and does not resend the request (hyperium/tonic#2185); the
//! next request reconnects, but the failed one reaches the caller.
//!
//! [`UnsentRequestRetry`] resends such requests the way grpc-go and grpc-java
//! do. A6 distinguishes two cases, which are retried differently:
//!
//! - The request never left the client: hyper cancelled it before writing it
//!   because its connection closed. This costs the server nothing, so it is
//!   retried a few times.
//! - The request reached the server, which rejected it unprocessed: a
//!   GOAWAY whose last stream id is below the request's stream, or a
//!   REFUSED_STREAM reset. This puts load on the wire, so it is retried once
//!   per call.
//!
//! As in grpc-java, only a GOAWAY with the NO_ERROR code counts: a GOAWAY
//! carrying an error code may report a wrong last stream id (Netty servers do
//! so for HPACK failures, netty/netty#10670), so a request it fails may have
//! been processed. A request the server may have processed is never resent,
//! so this retry is safe for every RPC, including transaction execution.
//!
//! Also as in A6, the call's deadline (the `grpc-timeout` header set by
//! [`tonic::Request::set_timeout`]) covers all attempts: it runs from the
//! first attempt, and each resend carries the remaining time in its
//! `grpc-timeout` header. The client's response-headers timeout is enforced
//! by tonic inside the channel and so applies to each attempt.

use std::error::Error as StdError;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;

use futures::future::BoxFuture;
use http_body_util::BodyExt;
use http_body_util::Full;
use tokio::time::Instant;
use tonic::body::Body;
use tower::Service;
use tower::ServiceExt;

use super::BoxError;
use super::watchdog::parse_grpc_timeout;

/// Retries per call of a request that never left the client. One is enough
/// for a drained connection: the retry after it reconnects. grpc-java allows
/// up to 1000, but its reconnects back off exponentially; tonic reconnects
/// immediately, so a connection that keeps closing before the request is
/// written would turn a large bound into a burst of back-to-back connections.
const MAX_NOT_SENT_RETRIES: usize = 3;

/// Sends each request on the channel, resending it when it fails before the
/// server processed it.
///
/// Request bodies are buffered so they can be resent. Every `sui.rpc` RPC
/// sends a single request message, so no request is streamed.
#[derive(Clone)]
pub(super) struct UnsentRequestRetry {
    channel: tonic::transport::Channel,
}

impl UnsentRequestRetry {
    pub(super) fn new(channel: tonic::transport::Channel) -> Self {
        Self { channel }
    }
}

impl Service<http::Request<Body>> for UnsentRequestRetry {
    type Response = http::Response<Body>;
    type Error = BoxError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // The channel's readiness is awaited for every attempt in `call`.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: http::Request<Body>) -> Self::Future {
        let call_deadline =
            parse_grpc_timeout(request.headers()).map(|timeout| Instant::now() + timeout);
        let attempts = send_until_processed(self.channel.clone(), request, call_deadline);
        Box::pin(async move {
            let Some(call_deadline) = call_deadline else {
                return attempts.await;
            };
            tokio::time::timeout_at(call_deadline, attempts)
                .await
                .unwrap_or_else(|_| {
                    Err(tonic::Status::deadline_exceeded(
                        "timeout expired before response headers were received",
                    )
                    .into())
                })
        })
    }
}

async fn send_until_processed(
    channel: tonic::transport::Channel,
    request: http::Request<Body>,
    call_deadline: Option<Instant>,
) -> Result<http::Response<Body>, BoxError> {
    let (mut parts, body) = request.into_parts();
    let body = body.collect().await?.to_bytes();

    let mut refused_retried = false;
    let mut not_sent_retries = 0;
    loop {
        let request = http::Request::from_parts(parts.clone(), Body::new(Full::new(body.clone())));
        let error = match channel.clone().oneshot(request).await {
            Ok(response) => return Ok(response),
            Err(error) => error,
        };
        match unprocessed_failure(&error) {
            Some(UnprocessedFailure::NotSent) if not_sent_retries < MAX_NOT_SENT_RETRIES => {
                not_sent_retries += 1;
            }
            Some(UnprocessedFailure::Refused) if !refused_retried => {
                refused_retried = true;
            }
            _ => return Err(error.into()),
        }

        if let Some(call_deadline) = call_deadline {
            let remaining = call_deadline.saturating_duration_since(Instant::now());
            parts.headers.insert(
                "grpc-timeout",
                http::HeaderValue::from_str(&format_grpc_timeout(remaining))
                    .expect("grpc-timeout values are ASCII"),
            );
        }
    }
}

/// How a request failed without the server processing it.
#[derive(Debug, PartialEq, Eq)]
enum UnprocessedFailure {
    /// hyper cancelled the request before writing it to the connection.
    NotSent,
    /// The server rejected the request unprocessed: a NO_ERROR GOAWAY that
    /// excluded its stream, or a REFUSED_STREAM reset.
    Refused,
}

/// Classify `error` by the first h2 or hyper error in its source chain.
///
/// hyper does not expose the h2 error kind through its own API, so this
/// inspects the `h2::Error` in hyper's source chain, which needs the same h2
/// version as hyper. h2 reports a remote GOAWAY on a stream only for streams
/// above the GOAWAY's last stream id and for streams opened after it.
fn unprocessed_failure(error: &(dyn StdError + 'static)) -> Option<UnprocessedFailure> {
    let mut source = Some(error);
    while let Some(error) = source {
        if let Some(h2_error) = error.downcast_ref::<h2::Error>() {
            let refused = h2_error.is_remote()
                && match h2_error.reason() {
                    Some(h2::Reason::NO_ERROR) => h2_error.is_go_away(),
                    Some(h2::Reason::REFUSED_STREAM) => h2_error.is_reset(),
                    _ => false,
                };
            return refused.then_some(UnprocessedFailure::Refused);
        }
        if let Some(hyper_error) = error.downcast_ref::<hyper::Error>()
            && hyper_error.is_canceled()
        {
            return Some(UnprocessedFailure::NotSent);
        }
        source = error.source();
    }
    None
}

/// Format `duration` as a `grpc-timeout` header value: at most 8 digits and
/// a unit, using the most precise unit that fits, as tonic does.
fn format_grpc_timeout(duration: Duration) -> String {
    const MAX_VALUE: u128 = 99_999_999;
    [
        (duration.as_micros(), 'u'),
        (duration.as_millis(), 'm'),
        (u128::from(duration.as_secs()), 'S'),
        (u128::from(duration.as_secs() / 60), 'M'),
    ]
    .into_iter()
    .find(|(value, _)| *value <= MAX_VALUE)
    .map(|(value, unit)| format!("{value}{unit}"))
    .unwrap_or_else(|| format!("{}H", (duration.as_secs() / 3600).min(99_999_999)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grpc_timeout_formatting_round_trips() {
        for duration in [
            Duration::ZERO,
            Duration::from_micros(1),
            Duration::from_millis(1500),
            Duration::from_secs(99),
            Duration::from_secs(100),
            Duration::from_secs(60 * 60 * 24 * 30),
        ] {
            let mut headers = http::HeaderMap::new();
            headers.insert(
                "grpc-timeout",
                format_grpc_timeout(duration).parse().unwrap(),
            );
            let parsed = parse_grpc_timeout(&headers).unwrap();
            assert!(parsed <= duration, "{duration:?} formatted as {parsed:?}");
            assert!(
                duration - parsed < Duration::from_secs(60),
                "{duration:?} lost precision: {parsed:?}"
            );
        }
    }
}
