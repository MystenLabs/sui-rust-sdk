use std::pin::Pin;
use std::time::Duration;

use futures::StreamExt;
use prost::bytes::Bytes;
use tonic::Request;
use tonic::Status;
use tonic::codegen::BoxStream;

use super::super::Client;
use super::super::Result;
use super::adapter::CursorDomain;
use super::adapter::CursorGapUpper;
use super::adapter::ListScanDirection;
use super::adapter::Progress;
use super::adapter::RecoveryGap;
use super::adapter::RpcFuture;
use super::adapter::SubscriptionAdapter;
use super::observability::LedgerStreamEvent;
use super::observability::LedgerStreamObservability;
use super::observability::LedgerStreamOperation;
use super::observability::LedgerStreamStage;
use super::retry::FailurePhase;
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
        request_started_from_resume_bound: bool,
    ) -> bool {
        let (checkpoint, cursor) = if received_prior_frame {
            (self.checkpoint_after_progress, self.cursor_after_progress)
        } else {
            // A resume frontier may immediately return CursorBound when nothing follows it.
            (
                self.checkpoint_without_prior_progress,
                self.cursor_without_prior_progress || request_started_from_resume_bound,
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

fn to_exclusive_end_bound(checkpoint_height: u64) -> Result<u64> {
    checkpoint_height.checked_add(1).ok_or_else(|| {
        Status::out_of_range("checkpoint height cannot be converted to an exclusive end bound")
    })
}

/// Sets the page size on an internal request; caller-built `list_*` requests bypass this path.
fn apply_internal_list_page_limit<A: SubscriptionAdapter>(
    request: &mut A::ListRequest,
    list_page_limit: Option<u32>,
) {
    A::options_mut(request).limit = list_page_limit;
}

pub(super) fn build_initial_list_request<A: SubscriptionAdapter>(
    template: &A::ListRequest,
    checkpoint_height: u64,
    list_page_limit: Option<u32>,
) -> Result<(A::ListRequest, ExpectedEnd)> {
    let mut request = template.clone();
    apply_internal_list_page_limit::<A>(&mut request, list_page_limit);
    A::set_end_checkpoint(
        &mut request,
        Some(to_exclusive_end_bound(checkpoint_height)?),
    );
    Ok((request, ExpectedEnd::from_bounds(true, false)))
}

/// Queries a single tip checkpoint to establish baseline watermark progress (used when starting in
/// Poll mode, or recovering baseline progress after an initial Subscribe connection failure).
pub(super) fn build_live_tip_baseline_list_request<A: SubscriptionAdapter>(
    template: &A::ListRequest,
    checkpoint_height: u64,
    list_page_limit: Option<u32>,
) -> Result<(A::ListRequest, ExpectedEnd)> {
    let mut request = template.clone();
    apply_internal_list_page_limit::<A>(&mut request, list_page_limit);
    A::set_start_checkpoint(&mut request, Some(checkpoint_height));
    A::set_end_checkpoint(
        &mut request,
        Some(to_exclusive_end_bound(checkpoint_height)?),
    );
    let options = A::options_mut(&mut request);
    options.after = None;
    options.before = None;
    Ok((request, ExpectedEnd::from_bounds(true, false)))
}

pub(super) fn build_polling_list_request<A: SubscriptionAdapter>(
    template: &A::ListRequest,
    committed_progress: &Progress<A::Cursor>,
    checkpoint_height: u64,
    list_page_limit: Option<u32>,
) -> Result<(A::ListRequest, ExpectedEnd)> {
    let mut request = template.clone();
    apply_internal_list_page_limit::<A>(&mut request, list_page_limit);
    A::set_ascending_resume(&mut request, committed_progress)?;
    A::options_mut(&mut request).before = None;
    A::set_end_checkpoint(
        &mut request,
        Some(to_exclusive_end_bound(checkpoint_height)?),
    );
    Ok((request, ExpectedEnd::from_bounds(true, false)))
}

pub(super) fn build_recovery_list_request<A: SubscriptionAdapter>(
    template: A::ListRequest,
    gap: &RecoveryGap,
    list_page_limit: Option<u32>,
) -> Result<(A::ListRequest, ExpectedEnd)> {
    let mut request = template;
    apply_internal_list_page_limit::<A>(&mut request, list_page_limit);
    match gap {
        RecoveryGap::Checkpoints {
            start_checkpoint,
            end_checkpoint,
        } => {
            A::set_start_checkpoint(&mut request, Some(*start_checkpoint));
            A::set_end_checkpoint(&mut request, Some(*end_checkpoint));
            let options = A::options_mut(&mut request);
            options.after = None;
            options.before = None;
            Ok((request, ExpectedEnd::from_bounds(true, false)))
        }
        RecoveryGap::Cursors {
            after,
            upper: CursorGapUpper::Before(before),
        } => {
            let options = A::options_mut(&mut request);
            options.after = Some(after.clone());
            options.before = Some(before.clone());
            Ok((request, ExpectedEnd::from_bounds(false, true)))
        }
        RecoveryGap::Cursors {
            after,
            upper: CursorGapUpper::EndOfCheckpoint(checkpoint),
        } => {
            let options = A::options_mut(&mut request);
            options.after = Some(after.clone());
            options.before = None;
            A::set_end_checkpoint(&mut request, Some(to_exclusive_end_bound(*checkpoint)?));
            Ok((request, ExpectedEnd::from_bounds(true, false)))
        }
    }
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

fn set_resume_bound<A: SubscriptionAdapter>(
    direction: ListScanDirection,
    request: &mut A::ListRequest,
    cursor: Bytes,
) {
    direction.set_resume_bound(A::options_mut(request), cursor);
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
pub(super) enum ListAction<R, P> {
    Frame {
        response: R,
        progress: Option<P>,
        complete: bool,
    },
    Continue,
    Terminal(Status),
}

/// How an ascending scan handles `LedgerTip` before its requested bound.
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
    /// Snapshot of the starting bound sent in the active request, used to detect bound violations.
    request_start_resume_bound: Option<Bytes>,
    /// Most recent raw wire cursor received, used to populate `after`/`before` when reconnecting.
    latest_cursor: Option<Bytes>,
    /// Most recent progress metadata, used to validate monotonic checkpoint coverage.
    latest_progress: Option<Progress<A::Cursor>>,
    polling_attempt: u32,
    received_frame_in_request: bool,
    request_has_checkpoint_coverage: bool,
    received_any_frame: bool,
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
            request_start_resume_bound: None,
            latest_cursor: None,
            latest_progress: None,
            polling_attempt: 0,
            received_frame_in_request: false,
            request_has_checkpoint_coverage: false,
            received_any_frame: false,
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
        self.request_start_resume_bound = A::options(&self.payload)
            .and_then(|options| self.direction.resume_bound(options).clone());
        let request = Request::new(self.payload.clone());
        self.rpc_state = RpcState::Dispatch {
            future: A::dispatch_list(self.client.clone(), request),
            started_at: None,
        };
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
    ) -> ListAction<A::ListResponse, Progress<A::Cursor>> {
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
        if let Some(cursor) = self.latest_cursor.clone() {
            set_resume_bound::<A>(self.direction, &mut self.payload, cursor);
        }
        if let Some(delay) =
            self.retry
                .retry_delay(&status, phase, config, &self.observability, self.stage)
        {
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
    ) -> Result<(Option<Progress<A::Cursor>>, bool)> {
        let watermark =
            watermark.ok_or_else(|| Status::data_loss("List frame is missing its watermark"))?;
        let cursor = watermark
            .cursor
            .as_ref()
            .cloned()
            .ok_or_else(|| Status::data_loss("List watermark is missing its cursor"))?;

        let checkpoint = watermark.checkpoint;
        if checkpoint.is_none() && self.request_has_checkpoint_coverage {
            return Err(Status::data_loss(
                "List checkpoint coverage became unavailable",
            ));
        }
        if checkpoint.is_some() {
            self.request_has_checkpoint_coverage = true;
        }
        let received_prior_frame_in_request = self.received_frame_in_request;
        let received_prior_frame_ever = self.received_any_frame;
        if item_present
            && end.is_none()
            && !received_prior_frame_in_request
            && self.request_start_resume_bound.as_ref() == Some(&cursor)
        {
            return Err(Status::data_loss(
                "List item cursor equals its exclusive request resume bound",
            ));
        }
        let mut frame_progress = A::Cursor::position(&cursor, checkpoint);
        if frame_progress.is_none() && item_present {
            return Err(Status::data_loss(
                "List item watermark names no resumable position",
            ));
        }
        if let (Some(previous), Some(next)) = (&self.latest_progress, &mut frame_progress) {
            next.inherit_checkpoint_coverage(previous);
            previous.validate_list_successor(next, self.direction)?;
        }
        self.received_frame_in_request = true;
        self.received_any_frame = true;
        let cursor_changed = self.latest_cursor.as_ref() != Some(&cursor);
        if item_present && !cursor_changed {
            return Err(Status::data_loss("List item repeated its cursor"));
        }
        if cursor_changed {
            self.latest_cursor = Some(cursor.clone());
            self.polling_attempt = 0;
            self.retry.reset(&self.observability);
        }
        if let Some(progress) = &frame_progress {
            self.latest_progress = Some(progress.clone());
        }

        let complete = match end {
            None => false,
            Some(end) => {
                let reason = end
                    .reason
                    .and_then(|reason| QueryEndReason::try_from(reason).ok())
                    .filter(|reason| *reason != QueryEndReason::Unknown)
                    .ok_or_else(|| {
                        Status::data_loss("List QueryEnd has a missing or unknown reason")
                    })?;
                validate_query_end_item(reason, item_present)?;
                if reason == QueryEndReason::CheckpointBound {
                    A::validate_checkpoint_bound(&self.payload, self.direction, checkpoint)?;
                }

                match reason {
                    QueryEndReason::ItemLimit => {
                        if self.request_start_resume_bound.as_ref() == Some(&cursor) {
                            return Err(Status::data_loss(
                                "ItemLimit QueryEnd did not advance its cursor",
                            ));
                        }
                        set_resume_bound::<A>(self.direction, &mut self.payload, cursor.clone());
                        self.start_dispatch();
                        false
                    }
                    QueryEndReason::ScanLimit => {
                        if self.request_start_resume_bound.as_ref() == Some(&cursor) {
                            return Err(Status::data_loss(
                                "ScanLimit QueryEnd did not advance its cursor",
                            ));
                        }
                        set_resume_bound::<A>(self.direction, &mut self.payload, cursor.clone());
                        self.start_dispatch();
                        false
                    }
                    QueryEndReason::CheckpointBound | QueryEndReason::CursorBound => {
                        if !self.expected_end.accepts(
                            reason,
                            received_prior_frame_in_request,
                            self.request_start_resume_bound.is_some(),
                        ) {
                            return Err(Status::data_loss("List ended at an unexpected bound"));
                        }
                        self.rpc_state = RpcState::Idle;
                        true
                    }
                    // A descending interval entirely beyond the indexed tip ends at LedgerTip.
                    QueryEndReason::LedgerTip
                        if self.direction == ListScanDirection::Descending
                            && !received_prior_frame_ever =>
                    {
                        self.rpc_state = RpcState::Idle;
                        true
                    }
                    // After progress, a descending scan cannot legitimately reach LedgerTip.
                    QueryEndReason::LedgerTip
                        if self.direction == ListScanDirection::Descending =>
                    {
                        return Err(Status::data_loss(
                            "descending List reached LedgerTip after prior scan progress",
                        ));
                    }
                    QueryEndReason::LedgerTip
                        if matches!(self.ledger_tip_policy, LedgerTipPolicy::Complete) =>
                    {
                        self.rpc_state = RpcState::Idle;
                        true
                    }
                    QueryEndReason::LedgerTip => {
                        set_resume_bound::<A>(self.direction, &mut self.payload, cursor.clone());
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
