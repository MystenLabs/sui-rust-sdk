use std::sync::Arc;
use std::time::Duration;

use tonic::Code;
use tonic::Status;

use super::super::Result;

/// The ledger protocol family associated with an observability event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum LedgerStreamFamily {
    Checkpoint,
    Transaction,
    Event,
}

/// The RPC operation associated with an observability event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum LedgerStreamOperation {
    List,
    GetServiceInfo,
    Subscribe,
}

/// List operations use `List`; stream operations use the remaining stages.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum LedgerStreamStage {
    List,
    InitialReplay,
    LiveTipStartup,
    PollingBaseline,
    PollingTail,
    GapRecovery,
    LiveSubscription,
}

/// Observability events emitted by ledger streams.
///
/// Label metrics by family, operation, stage, and status code. Record counts and durations as
/// values; keep status messages and metadata in logs.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum LedgerStreamEvent {
    /// A generated RPC future resolved.
    ///
    /// For List and Subscribe, `elapsed` measures dispatch-to-response-headers latency, not
    /// response-body processing or subscription lifetime.
    #[non_exhaustive]
    RpcResponse {
        family: LedgerStreamFamily,
        operation: LedgerStreamOperation,
        stage: LedgerStreamStage,
        code: Code,
        elapsed: Duration,
    },
    /// A classified transient failure scheduled another attempt.
    #[non_exhaustive]
    RetryScheduled {
        family: LedgerStreamFamily,
        operation: LedgerStreamOperation,
        stage: LedgerStreamStage,
        status: Status,
        consecutive_failures: u32,
        delay: Duration,
    },
    /// Operation progress ended a period of consecutive transient failures.
    #[non_exhaustive]
    RetryRecovered {
        family: LedgerStreamFamily,
        operation: LedgerStreamOperation,
        started_in: LedgerStreamStage,
        consecutive_failures: u32,
        elapsed: Duration,
    },
    /// An established Subscribe response body returned an error or ended unexpectedly.
    #[non_exhaustive]
    SubscriptionStreamInterrupted {
        family: LedgerStreamFamily,
        stage: LedgerStreamStage,
        status: Status,
    },
    /// The stream began a bounded List replay for a subscription gap.
    #[non_exhaustive]
    GapRecoveryStarted { family: LedgerStreamFamily },
    /// List gap buffering reached its configured item-bearing frame limit.
    #[non_exhaustive]
    SubscriptionBufferLimitReached {
        family: LedgerStreamFamily,
        buffered_items: usize,
        limit: usize,
    },
    /// A non-retryable terminal error that will be yielded as the next stream item.
    #[non_exhaustive]
    TerminalError {
        family: LedgerStreamFamily,
        status: Status,
    },
}

/// Observability events emitted by finite List operations.
///
/// Label metrics by family and status code. Record counts and durations as values; keep status
/// messages and metadata in logs.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ListEvent {
    /// A generated List RPC future resolved.
    ///
    /// `elapsed` measures dispatch-to-response-headers latency, not response-body processing.
    #[non_exhaustive]
    RpcResponse {
        family: LedgerStreamFamily,
        code: Code,
        elapsed: Duration,
    },
    /// A classified transient failure scheduled another attempt.
    #[non_exhaustive]
    RetryScheduled {
        family: LedgerStreamFamily,
        status: Status,
        consecutive_failures: u32,
        delay: Duration,
    },
    /// Operation progress ended a period of consecutive transient failures.
    #[non_exhaustive]
    RetryRecovered {
        family: LedgerStreamFamily,
        consecutive_failures: u32,
        elapsed: Duration,
    },
    /// A non-retryable terminal error that will be yielded as the next stream item.
    #[non_exhaustive]
    TerminalError {
        family: LedgerStreamFamily,
        status: Status,
    },
}

impl ListEvent {
    pub(super) fn from_stream_event(event: LedgerStreamEvent) -> Option<Self> {
        match event {
            LedgerStreamEvent::RpcResponse {
                family,
                operation: LedgerStreamOperation::List,
                code,
                elapsed,
                ..
            } => Some(Self::RpcResponse {
                family,
                code,
                elapsed,
            }),
            LedgerStreamEvent::RetryScheduled {
                family,
                operation: LedgerStreamOperation::List,
                status,
                consecutive_failures,
                delay,
                ..
            } => Some(Self::RetryScheduled {
                family,
                status,
                consecutive_failures,
                delay,
            }),
            LedgerStreamEvent::RetryRecovered {
                family,
                operation: LedgerStreamOperation::List,
                consecutive_failures,
                elapsed,
                ..
            } => Some(Self::RetryRecovered {
                family,
                consecutive_failures,
                elapsed,
            }),
            LedgerStreamEvent::TerminalError { family, status } => {
                Some(Self::TerminalError { family, status })
            }
            _ => None,
        }
    }
}

pub(super) type ListObserver = Arc<dyn Fn(ListEvent) + Send + Sync + 'static>;

pub(super) type LedgerStreamObserver = Arc<dyn Fn(LedgerStreamEvent) + Send + Sync + 'static>;

#[derive(Clone, Default)]
pub(super) struct LedgerStreamObservability {
    observer: Option<LedgerStreamObserver>,
}

impl LedgerStreamObservability {
    pub(super) fn new(observer: Option<LedgerStreamObserver>) -> Self {
        Self { observer }
    }

    pub(super) fn emit(&self, event: impl FnOnce() -> LedgerStreamEvent) {
        if let Some(observer) = &self.observer {
            observer(event());
        }
    }

    pub(super) fn start_timer(&self) -> Option<tokio::time::Instant> {
        self.observer.as_ref().map(|_| tokio::time::Instant::now())
    }

    pub(super) fn emit_rpc_response<T>(
        &self,
        started_at: Option<tokio::time::Instant>,
        family: LedgerStreamFamily,
        operation: LedgerStreamOperation,
        stage: LedgerStreamStage,
        result: &Result<T>,
    ) {
        let Some(started_at) = started_at else {
            return;
        };
        let code = result
            .as_ref()
            .map_or_else(|status| status.code(), |_| Code::Ok);
        self.emit(|| LedgerStreamEvent::RpcResponse {
            family,
            operation,
            stage,
            code,
            elapsed: started_at.elapsed(),
        });
    }
}
