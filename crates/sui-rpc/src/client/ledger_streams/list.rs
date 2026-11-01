use std::pin::Pin;
use std::time::Duration;

use futures::StreamExt;
use prost::bytes::Bytes;
use tonic::Request;
use tonic::Status;
use tonic::codegen::BoxStream;

use super::super::Client;
use super::super::Result;
use super::adapter::ListScanDirection;
use super::adapter::Position;
use super::adapter::Progress;
use super::adapter::RecoveryGap;
use super::adapter::RpcFuture;
use super::adapter::SubscriptionAdapter;
use super::adapter::missing_cursor_position;
use super::adapter::set_end;
use super::adapter::set_start;
use super::observability::LedgerStreamEvent;
use super::observability::LedgerStreamObservability;
use super::observability::LedgerStreamOperation;
use super::observability::LedgerStreamStage;
use super::retry::FailurePhase;
use super::retry::RequestBounds;
use super::retry::RetryState;
use super::retry::backoff_delay;
use super::types::LedgerStreamConfig;
use crate::proto::sui::rpc::v2::QueryEnd;
use crate::proto::sui::rpc::v2::QueryEndReason;
use crate::proto::sui::rpc::v2::Watermark;

/// Permitted `CheckpointBound` and `CursorBound` endings, with or without prior progress.
#[derive(Clone, Copy)]
pub(super) struct ExpectedEnd {
    checkpoint_after_progress: bool,
    cursor_after_progress: bool,
    checkpoint_without_prior_progress: bool,
    cursor_without_prior_progress: bool,
}

impl ExpectedEnd {
    fn from_bounds(checkpoint: bool, cursor: bool) -> Self {
        Self {
            checkpoint_after_progress: checkpoint,
            cursor_after_progress: cursor,
            checkpoint_without_prior_progress: checkpoint,
            cursor_without_prior_progress: cursor,
        }
    }

    fn accepts(
        self,
        reason: QueryEndReason,
        received_prior_frame: bool,
        request_has_resume_cursor: bool,
    ) -> bool {
        let (checkpoint, cursor) = if received_prior_frame {
            (self.checkpoint_after_progress, self.cursor_after_progress)
        } else {
            // A resume frontier may immediately return CursorBound when nothing follows it.
            (
                self.checkpoint_without_prior_progress,
                self.cursor_without_prior_progress || request_has_resume_cursor,
            )
        };
        (checkpoint && reason == QueryEndReason::CheckpointBound)
            || (cursor && reason == QueryEndReason::CursorBound)
    }
}

fn validate_query_end_item(reason: QueryEndReason, item_present: bool) -> Result<()> {
    if reason == QueryEndReason::ItemLimit && !item_present {
        Err(Status::data_loss("ItemLimit QueryEnd is missing its item"))
    } else if reason != QueryEndReason::ItemLimit && item_present {
        Err(Status::data_loss(
            "non-ItemLimit QueryEnd unexpectedly contains an item",
        ))
    } else {
        Ok(())
    }
}

/// The boundary before the first checkpoint past `checkpoint_height`.
fn past_checkpoint_height(checkpoint_height: u64) -> Result<Position> {
    checkpoint_height
        .checked_add(1)
        .map(Position::checkpoint_start)
        .ok_or_else(|| {
            Status::out_of_range("checkpoint height cannot be converted to an exclusive end bound")
        })
}

/// Lists from the template's start through `checkpoint_height`.
pub(super) fn build_initial_list_request<A: SubscriptionAdapter>(
    template: &A::ListRequest,
    checkpoint_height: u64,
) -> Result<(A::ListRequest, ExpectedEnd)> {
    let mut request = template.clone();
    set_end::<A>(
        &mut request,
        past_checkpoint_height(checkpoint_height)?,
        None,
    )?;
    Ok((request, ExpectedEnd::from_bounds(true, false)))
}

/// Queries a single tip checkpoint to establish baseline watermark progress (used when starting in
/// Poll mode, or recovering baseline progress after an initial Subscribe connection failure).
pub(super) fn build_live_tip_baseline_list_request<A: SubscriptionAdapter>(
    template: &A::ListRequest,
    checkpoint_height: u64,
) -> Result<(A::ListRequest, ExpectedEnd)> {
    let mut request = template.clone();
    set_start::<A>(
        &mut request,
        Position::checkpoint_start(checkpoint_height),
        None,
    );
    set_end::<A>(
        &mut request,
        past_checkpoint_height(checkpoint_height)?,
        None,
    )?;
    Ok((request, ExpectedEnd::from_bounds(true, false)))
}

/// Lists from committed progress through `checkpoint_height`.
pub(super) fn build_polling_list_request<A: SubscriptionAdapter>(
    template: &A::ListRequest,
    committed_progress: &Progress<A::Cursor>,
    checkpoint_height: u64,
) -> Result<(A::ListRequest, ExpectedEnd)> {
    let mut request = template.clone();
    set_start::<A>(
        &mut request,
        committed_progress.position,
        A::cursor_bytes(&committed_progress.cursor),
    );
    set_end::<A>(
        &mut request,
        past_checkpoint_height(checkpoint_height)?,
        None,
    )?;
    Ok((request, ExpectedEnd::from_bounds(true, false)))
}

/// Lists exactly the interval `gap` names.
pub(super) fn build_recovery_list_request<A: SubscriptionAdapter>(
    template: A::ListRequest,
    gap: &RecoveryGap<A::Cursor>,
) -> Result<(A::ListRequest, ExpectedEnd)> {
    let mut request = template;
    set_start::<A>(&mut request, gap.start, A::cursor_bytes(&gap.after));
    let before = A::cursor_bytes(&gap.before);
    let ends_at_cursor = before.is_some();
    set_end::<A>(&mut request, gap.end, before)?;
    // The end fields and the `before` cursor name the same point, so either may end the scan.
    Ok((request, ExpectedEnd::from_bounds(true, ends_at_cursor)))
}

pub(super) fn determine_expected_end<A: SubscriptionAdapter>(
    request: &A::ListRequest,
    direction: ListScanDirection,
) -> ExpectedEnd {
    let options = A::options(request);
    let checkpoint_without_prior_progress = A::start_checkpoint(request).is_some()
        || A::end_checkpoint(request).is_some()
        || direction == ListScanDirection::Descending;
    let cursor_without_prior_progress =
        options.is_some_and(|options| options.after.is_some() || options.before.is_some());
    let (checkpoint_after_progress, cursor_after_progress) = match direction {
        ListScanDirection::Ascending => (
            A::end_checkpoint(request).is_some(),
            options.is_some_and(|options| options.before.is_some()),
        ),
        ListScanDirection::Descending => {
            (true, options.is_some_and(|options| options.after.is_some()))
        }
    };
    ExpectedEnd {
        checkpoint_after_progress,
        cursor_after_progress,
        checkpoint_without_prior_progress,
        cursor_without_prior_progress,
    }
}

enum RpcState<A: SubscriptionAdapter> {
    Idle,
    Dispatch {
        future: RpcFuture<A::ListResponse>,
        started_at: Option<tokio::time::Instant>,
    },
    Stream(BoxStream<A::ListResponse>),
    Sleep(Pin<Box<tokio::time::Sleep>>),
}

/// Raw I/O event produced by [`ListMachine::poll_rpc`].
pub(super) enum RpcEvent<R> {
    Frame(R),
    Status(Status, FailurePhase),
    Eof,
    Wake,
}

/// Driver action returned by [`ListMachine::process_event`].
pub(super) enum ListAction<R> {
    Frame {
        response: R,
        /// `None` only on a page's final frame when the request names its bound by a bare
        /// `options.after` or `options.before` cursor minted without a position.
        progress: Option<Progress<Bytes>>,
        complete: bool,
    },
    Continue,
    Terminal(Status),
}

/// How an ascending scan handles a `LedgerTip` end.
#[derive(Clone, Copy)]
pub(super) enum LedgerTipPolicy {
    /// Complete at the indexed tip.
    Complete,
    /// Retry from the served frontier until the fixed bound is indexed.
    WaitForExpectedBound,
}

pub(super) struct ListMachine<A: SubscriptionAdapter> {
    client: Client,
    payload: A::ListRequest,
    expected_end: ExpectedEnd,
    direction: ListScanDirection,
    ledger_tip_policy: LedgerTipPolicy,
    rpc_state: RpcState<A>,
    /// Position the active request resumed from, used to detect pagination that does not advance.
    request_resume_position: Option<Position>,
    /// Whether the active request carries an opaque cursor in its resume slot.
    request_has_resume_cursor: bool,
    /// Most recent frame progress, used to resume and to validate ordering.
    latest_progress: Option<Progress<Bytes>>,
    polling_attempt: u32,
    received_frame_in_request: bool,
    request_has_checkpoint_coverage: bool,
    retry: RetryState,
    pub(super) observability: LedgerStreamObservability,
    stage: LedgerStreamStage,
}

impl<A: SubscriptionAdapter> ListMachine<A> {
    pub(super) fn new(
        client: Client,
        payload: A::ListRequest,
        expected_end: ExpectedEnd,
        direction: ListScanDirection,
        ledger_tip_policy: LedgerTipPolicy,
        observability: LedgerStreamObservability,
        stage: LedgerStreamStage,
    ) -> Self {
        let mut machine = Self {
            client,
            payload,
            expected_end,
            direction,
            ledger_tip_policy,
            rpc_state: RpcState::Idle,
            request_resume_position: None,
            request_has_resume_cursor: false,
            latest_progress: None,
            polling_attempt: 0,
            received_frame_in_request: false,
            request_has_checkpoint_coverage: false,
            retry: RetryState::new(A::FAMILY, LedgerStreamOperation::List),
            observability,
            stage,
        };
        machine.start_dispatch();
        machine
    }

    fn start_dispatch(&mut self) {
        self.received_frame_in_request = false;
        self.request_has_checkpoint_coverage = false;
        self.request_has_resume_cursor = A::options(&self.payload)
            .is_some_and(|options| self.direction.has_resume_cursor(options));
        let request = Request::new(self.payload.clone());
        self.rpc_state = RpcState::Dispatch {
            future: A::dispatch_list(self.client.clone(), request),
            started_at: None,
        };
    }

    /// Points the next request at the most recent frame's resume point.
    fn resume_from_latest_progress(&mut self) -> Result<()> {
        if let Some(progress) = &self.latest_progress {
            self.direction
                .set_resume::<A>(&mut self.payload, progress)?;
            self.request_resume_position = Some(progress.position);
        }
        Ok(())
    }

    fn sleep(&mut self, delay: Duration) {
        self.rpc_state = RpcState::Sleep(Box::pin(tokio::time::sleep(delay)));
    }

    /// Dropping this future preserves `self.rpc_state`, so the next call resumes rather than
    /// restarts in-flight work.
    pub(super) async fn poll_rpc(&mut self) -> RpcEvent<A::ListResponse> {
        loop {
            match &mut self.rpc_state {
                RpcState::Idle => {
                    return RpcEvent::Status(
                        Status::internal("idle RPC state polled"),
                        FailurePhase::Dispatch,
                    );
                }
                RpcState::Dispatch { future, started_at } => {
                    // The timer starts on the first poll and survives cancellation by gap buffering.
                    if started_at.is_none() {
                        *started_at = self.observability.start_timer();
                    }
                    let rpc_started_at = started_at.as_ref().cloned();
                    let result = future.await;
                    self.observability.emit_rpc_response(
                        rpc_started_at,
                        A::FAMILY,
                        LedgerStreamOperation::List,
                        self.stage,
                        &result,
                    );
                    match result {
                        Ok(stream) => self.rpc_state = RpcState::Stream(stream),
                        Err(status) => {
                            self.rpc_state = RpcState::Idle;
                            return RpcEvent::Status(status, FailurePhase::Dispatch);
                        }
                    }
                }
                RpcState::Stream(stream) => match stream.next().await {
                    Some(Ok(response)) => return RpcEvent::Frame(response),
                    Some(Err(status)) => {
                        self.rpc_state = RpcState::Idle;
                        return RpcEvent::Status(status, FailurePhase::Body);
                    }
                    None => {
                        self.rpc_state = RpcState::Idle;
                        return RpcEvent::Eof;
                    }
                },
                RpcState::Sleep(sleep) => {
                    sleep.as_mut().await;
                    self.rpc_state = RpcState::Idle;
                    return RpcEvent::Wake;
                }
            }
        }
    }

    /// Advances validation and pagination state with one RPC event.
    pub(super) fn process_event(
        &mut self,
        event: RpcEvent<A::ListResponse>,
        config: &LedgerStreamConfig,
    ) -> ListAction<A::ListResponse> {
        match event {
            RpcEvent::Wake => {
                self.start_dispatch();
                ListAction::Continue
            }
            // Successful List streams end with QueryEnd; bare EOF has no completion boundary.
            RpcEvent::Eof => ListAction::Terminal(Status::data_loss(
                "List stream ended before its QueryEnd frame",
            )),
            RpcEvent::Status(status, phase) => match self.process_status(status, phase, config) {
                Ok(()) => ListAction::Continue,
                Err(status) => ListAction::Terminal(status),
            },
            RpcEvent::Frame(response) => {
                let (item_present, watermark, end) = A::extract_metadata(&response);
                match self.process_frame_metadata(item_present, watermark, end, config) {
                    Ok((progress, complete)) => ListAction::Frame {
                        response,
                        progress,
                        complete,
                    },
                    Err(status) => ListAction::Terminal(status),
                }
            }
        }
    }

    fn process_status(
        &mut self,
        status: Status,
        phase: FailurePhase,
        config: &LedgerStreamConfig,
    ) -> Result<()> {
        self.resume_from_latest_progress()?;
        // A request that resumes from a frame has bounds a server reported.
        let bounds = if self.request_resume_position.is_some() {
            RequestBounds::ServerReported
        } else {
            RequestBounds::CallerSupplied
        };
        if let Some(delay) = self.retry.retry_delay(
            &status,
            phase,
            bounds,
            config,
            &self.observability,
            self.stage,
        ) {
            self.sleep(delay);
            Ok(())
        } else {
            Err(status)
        }
    }

    fn process_frame_metadata(
        &mut self,
        item_present: bool,
        watermark: Option<&Watermark>,
        end: Option<&QueryEnd>,
        config: &LedgerStreamConfig,
    ) -> Result<(Option<Progress<Bytes>>, bool)> {
        let watermark =
            watermark.ok_or_else(|| Status::data_loss("List frame is missing its watermark"))?;
        let cursor = watermark
            .cursor
            .clone()
            .ok_or_else(|| Status::data_loss("List watermark is missing its cursor"))?;
        let end_reason = end
            .map(|end| {
                end.reason
                    .and_then(|reason| QueryEndReason::try_from(reason).ok())
                    .filter(|reason| *reason != QueryEndReason::Unknown)
                    .ok_or_else(|| {
                        Status::data_loss("List QueryEnd has a missing or unknown reason")
                    })
            })
            .transpose()?;

        let checkpoint = watermark.checkpoint;
        if checkpoint.is_none() && self.request_has_checkpoint_coverage {
            return Err(Status::data_loss(
                "List checkpoint coverage became unavailable",
            ));
        }
        if checkpoint.is_some() {
            self.request_has_checkpoint_coverage = true;
        }

        let mut frame_progress = match watermark.cursor_position.as_ref() {
            Some(position) => Some(Progress {
                position: A::GRANULARITY.position(position),
                cursor,
                checkpoint,
            }),
            // A server leaves the position unset only on a page's final frame when the request
            // names that point by an `after` or `before` cursor alone and the cursor was minted
            // without a position, by a server that predates cursor positions. A caller's cursor on
            // the far side of the scan (`before` ascending, `after` descending) stays bare on every
            // request. A near-side cursor is bare only on the caller's first request:
            // continuations pair it with position fields.
            None if !item_present
                && matches!(
                    end_reason,
                    Some(
                        QueryEndReason::LedgerTip
                            | QueryEndReason::CursorBound
                            | QueryEndReason::CheckpointBound
                    )
                )
                && A::options(&self.payload).is_some_and(|options| {
                    self.direction.has_far_cursor(options)
                        || (self.request_resume_position.is_none()
                            && self.direction.has_resume_cursor(options))
                }) =>
            {
                None
            }
            None => return Err(missing_cursor_position("List")),
        };
        let mut advanced = item_present;
        if let Some(next) = &mut frame_progress {
            if let Some(latest) = &mut self.latest_progress {
                next.inherit_checkpoint_coverage(latest);
                let (position_advanced, _) =
                    latest.advance(next, self.direction, item_present, "List")?;
                advanced |= position_advanced;
            } else {
                advanced = true;
                self.latest_progress = Some(next.clone());
            }
        }
        let received_prior_frame_in_request = self.received_frame_in_request;
        self.received_frame_in_request = true;
        if advanced {
            self.polling_attempt = 0;
            self.retry.reset(&self.observability);
        }

        let complete = match end_reason {
            None => false,
            Some(reason) => {
                validate_query_end_item(reason, item_present)?;
                if reason == QueryEndReason::CheckpointBound {
                    A::validate_checkpoint_bound(&self.payload, self.direction, checkpoint)?;
                }

                match reason {
                    QueryEndReason::ItemLimit | QueryEndReason::ScanLimit => {
                        // An ItemLimit item already advanced past the previous frame.
                        if reason == QueryEndReason::ScanLimit
                            && let (Some(progress), Some(resumed_from)) =
                                (&frame_progress, self.request_resume_position)
                            && !self.direction.beyond(progress.position, resumed_from)
                        {
                            return Err(Status::data_loss(
                                "ScanLimit QueryEnd did not advance its cursor_position",
                            ));
                        }
                        self.resume_from_latest_progress()?;
                        self.start_dispatch();
                        false
                    }
                    QueryEndReason::CheckpointBound | QueryEndReason::CursorBound => {
                        if !self.expected_end.accepts(
                            reason,
                            received_prior_frame_in_request,
                            self.request_has_resume_cursor,
                        ) {
                            return Err(Status::data_loss("List ended at an unexpected bound"));
                        }
                        self.rpc_state = RpcState::Idle;
                        true
                    }
                    // Servers end only ascending scans at the ledger tip.
                    QueryEndReason::LedgerTip
                        if self.direction == ListScanDirection::Descending =>
                    {
                        return Err(Status::data_loss("descending List reached LedgerTip"));
                    }
                    QueryEndReason::LedgerTip
                        if matches!(self.ledger_tip_policy, LedgerTipPolicy::Complete) =>
                    {
                        self.rpc_state = RpcState::Idle;
                        true
                    }
                    QueryEndReason::LedgerTip => {
                        self.resume_from_latest_progress()?;
                        let delay = backoff_delay(config, self.polling_attempt);
                        self.polling_attempt = self.polling_attempt.saturating_add(1);
                        self.sleep(delay);
                        false
                    }
                    QueryEndReason::Unknown => {
                        return Err(Status::data_loss("List QueryEnd has an unknown reason"));
                    }
                }
            }
        };

        Ok((frame_progress, complete))
    }
}

/// Drives a finite List operation by resuming the request cursor and yielding whole responses.
pub(super) struct ListDriver<A: SubscriptionAdapter> {
    machine: Option<ListMachine<A>>,
    config: LedgerStreamConfig,
    observability: LedgerStreamObservability,
    terminal: Option<Status>,
    done: bool,
}

impl<A: SubscriptionAdapter> ListDriver<A> {
    pub(super) fn new(client: Client, request: A::ListRequest, config: LedgerStreamConfig) -> Self {
        let observability = LedgerStreamObservability::new(config.observer());
        let payload = request;
        let direction = ListScanDirection::from_request::<A>(&payload);
        let terminal = direction.as_ref().err().cloned();
        let machine = direction.ok().map(|direction| {
            let expected_end = determine_expected_end::<A>(&payload, direction);
            ListMachine::new(
                client,
                payload,
                expected_end,
                direction,
                LedgerTipPolicy::Complete,
                observability.clone(),
                LedgerStreamStage::List,
            )
        });
        Self {
            machine,
            config,
            observability,
            terminal,
            done: false,
        }
    }

    pub(super) async fn next(&mut self) -> Option<Result<A::ListResponse>> {
        if self.done {
            return None;
        }
        if let Some(status) = self.terminal.take() {
            return Some(self.finish_terminal(status));
        }

        loop {
            let machine = self.machine.as_mut()?;
            let event = machine.poll_rpc().await;
            match machine.process_event(event, &self.config) {
                ListAction::Frame {
                    response, complete, ..
                } => {
                    if complete {
                        self.done = true;
                        self.machine = None;
                    }
                    return Some(Ok(response));
                }
                ListAction::Continue => {}
                ListAction::Terminal(status) => return Some(self.finish_terminal(status)),
            }
        }
    }

    fn finish_terminal(&mut self, status: Status) -> Result<A::ListResponse> {
        self.observability
            .emit(|| LedgerStreamEvent::TerminalError {
                family: A::FAMILY,
                status: status.clone(),
            });
        self.done = true;
        self.machine = None;
        Err(status)
    }
}
