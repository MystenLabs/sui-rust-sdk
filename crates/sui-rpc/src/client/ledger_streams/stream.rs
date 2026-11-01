use std::time::Duration;

use futures::StreamExt;
use prost::bytes::Bytes;
use tonic::Request;
use tonic::Status;
use tonic::codegen::BoxStream;

use super::super::Client;
use super::super::Result;
use super::adapter::LedgerFrame;
use super::adapter::ListScanDirection;
use super::adapter::Position;
use super::adapter::Progress;
use super::adapter::RecoveryGap;
use super::adapter::SubscriptionAdapter;
use super::adapter::missing_cursor_position;
use super::adapter::set_start;
use super::list::ExpectedEnd;
use super::list::LedgerTipPolicy;
use super::list::ListAction;
use super::list::ListMachine;
use super::list::build_initial_list_request;
use super::list::build_live_tip_baseline_list_request;
use super::list::build_polling_list_request;
use super::list::build_recovery_list_request;
use super::observability::LedgerStreamEvent;
use super::observability::LedgerStreamObservability;
use super::observability::LedgerStreamOperation;
use super::observability::LedgerStreamStage;
use super::retry::FailurePhase;
use super::retry::RequestBounds;
use super::retry::RetryState;
use super::subscription::BufferedSubscriptionDrain;
use super::subscription::BufferedSubscriptionState;
use super::subscription::GapReplay;
use super::subscription::LiveSubscription;
use super::types::Delivery;
use super::types::LedgerStreamConfig;
use crate::proto::sui::rpc::v2::GetServiceInfoRequest;

enum Phase<A: SubscriptionAdapter> {
    /// Waits until the ledger reaches the start checkpoint, then lists history through its tip.
    ReadInitialTip,
    /// Lists through a tip, then follows the ledger with the stream's delivery.
    // `Phase` moves by value; each replay allocates its boxed machine once.
    Replay(Box<Replay<A>>),
    Subscribing(SubscribingPhase<A>),
    Polling(PollingPhase),
    Terminal(Status),
    Done,
}

pub(super) enum Start {
    Tip,
    Checkpoint(u64),
    /// At `position`, which the opaque `cursor` also names.
    After {
        cursor: Bytes,
        position: Position,
    },
}

struct Replay<A: SubscriptionAdapter> {
    list_machine: ListMachine<A>,
    output: ReplayOutput,
}

#[derive(Clone, Copy)]
enum ReplayOutput {
    Suppress,
    Yield,
}

enum SubscribingPhase<A: SubscriptionAdapter> {
    Attempt,
    RetryDelay(Duration),
    ReadRecoveryTip,
    GapReplay(Box<GapReplay<A>>),
    DrainGapBuffer(BufferedSubscriptionDrain<A>),
    Live(LiveSubscription<A>),
}

enum PollingPhase {
    Sleep,
    ReadTip,
}

enum Step<A: SubscriptionAdapter> {
    Next(Phase<A>),
    Yield(Phase<A>, A::Output),
}

fn terminal<A: SubscriptionAdapter>(status: Status) -> Step<A> {
    Step::Next(Phase::Terminal(status))
}

/// Keys a List response from its progress, which an ascending stream scan always carries.
fn list_frame<A: SubscriptionAdapter>(
    response: A::ListResponse,
    progress: Option<Progress<Bytes>>,
) -> Result<LedgerFrame<A::Item, A::Cursor>> {
    let Progress {
        position,
        cursor,
        checkpoint,
    } = progress.ok_or_else(|| missing_cursor_position("List"))?;
    LedgerFrame::new(
        A::into_list_item(response),
        Progress {
            position,
            cursor: A::list_cursor(cursor),
            checkpoint,
        },
        A::GRANULARITY,
    )
}

/// The List interval between `committed` progress and a new subscription's `first` frame, or
/// `None` when the subscription starts at or before committed progress.
fn recovery_gap<A: SubscriptionAdapter>(
    committed: &Progress<A::Cursor>,
    first: &LedgerFrame<A::Item, A::Cursor>,
) -> Option<RecoveryGap<A::Cursor>> {
    // An item frame's cursor names its item's key as `options.before`.
    let end = first.leading_boundary();
    (end > committed.position).then(|| RecoveryGap {
        start: committed.position,
        after: committed.cursor.clone(),
        end,
        before: first.progress.cursor.clone(),
    })
}

enum SubscriptionAttempt<A: SubscriptionAdapter> {
    Established {
        stream: BoxStream<A::SubscribeResponse>,
        first_frame: LedgerFrame<A::Item, A::Cursor>,
    },
    Failed {
        status: Status,
        failure_phase: FailurePhase,
    },
}

async fn dispatch_subscription<A: SubscriptionAdapter>(
    client: &Client,
    observability: &LedgerStreamObservability,
    request: Request<A::SubscribeRequest>,
    observability_stage: LedgerStreamStage,
) -> Result<BoxStream<A::SubscribeResponse>> {
    let future = A::dispatch_subscribe(client.clone(), request);
    let started_at = observability.start_timer();
    let result = future.await;
    observability.emit_rpc_response(
        started_at,
        A::FAMILY,
        LedgerStreamOperation::Subscribe,
        observability_stage,
        &result,
    );
    result
}

pub(super) struct Driver<A: SubscriptionAdapter> {
    client: Client,
    list_template: A::ListRequest,
    subscribe_payload: A::SubscribeRequest,
    item_required: bool,
    config: LedgerStreamConfig,
    observability: LedgerStreamObservability,
    service_info_retry: RetryState,
    subscription_retry: RetryState,
    delivery: Delivery,
    phase: Option<Phase<A>>,
    /// The position a stream started at a checkpoint begins at. Until progress is committed,
    /// frames at or before it deliver nothing.
    start_position: Option<Position>,
    /// Every match before `position` has been delivered, or precedes the stream's start.
    committed_progress: Option<Progress<A::Cursor>>,
}

impl<A: SubscriptionAdapter> Driver<A> {
    pub(super) fn new_stream(
        client: Client,
        subscribe_payload: A::SubscribeRequest,
        mut list_template: A::ListRequest,
        start: Start,
        delivery: Delivery,
        config: LedgerStreamConfig,
    ) -> Self {
        let observability = LedgerStreamObservability::new(config.observer());
        let item_required = A::item_required(&subscribe_payload);
        let mut start_position = None;
        let mut committed_progress = None;
        let requested_phase = match start {
            Start::Tip => match delivery {
                Delivery::Subscribe => Phase::Subscribing(SubscribingPhase::Attempt),
                Delivery::Poll => Phase::Polling(PollingPhase::ReadTip),
            },
            Start::Checkpoint(checkpoint) => {
                let position = Position::checkpoint_start(checkpoint);
                set_start::<A>(&mut list_template, position, None);
                start_position = Some(position);
                Phase::ReadInitialTip
            }
            Start::After { cursor, position } => {
                let position = A::GRANULARITY.normalize(position);
                set_start::<A>(&mut list_template, position, Some(cursor.clone()));
                committed_progress = Some(Progress {
                    position,
                    cursor: A::list_cursor(cursor),
                    checkpoint: None,
                });
                Phase::ReadInitialTip
            }
        };
        let phase = if config.ledger_tip_poll_interval == Duration::ZERO {
            Phase::Terminal(Status::invalid_argument(
                "ledger_tip_poll_interval must be greater than zero",
            ))
        } else {
            requested_phase
        };
        Self {
            client,
            list_template,
            subscribe_payload,
            item_required,
            config,
            observability,
            service_info_retry: RetryState::new(A::FAMILY, LedgerStreamOperation::GetServiceInfo),
            subscription_retry: RetryState::new(A::FAMILY, LedgerStreamOperation::Subscribe),
            delivery,
            phase: Some(phase),
            start_position,
            committed_progress,
        }
    }

    pub(super) async fn next(&mut self) -> Option<Result<A::Output>> {
        loop {
            let step = match self.phase.take().unwrap_or(Phase::Done) {
                Phase::Done => {
                    self.phase = Some(Phase::Done);
                    return None;
                }
                Phase::Terminal(status) => {
                    self.observability
                        .emit(|| LedgerStreamEvent::TerminalError {
                            family: A::FAMILY,
                            status: status.clone(),
                        });
                    self.phase = Some(Phase::Done);
                    return Some(Err(status));
                }
                Phase::ReadInitialTip => self.read_initial_tip().await,
                Phase::Replay(replay) => self.step_replay(replay).await,
                Phase::Subscribing(phase) => self.step_subscribing(phase).await,
                Phase::Polling(phase) => self.step_polling(phase).await,
            };
            match step {
                Step::Next(next) => self.phase = Some(next),
                Step::Yield(next, output) => {
                    self.phase = Some(next);
                    return Some(Ok(output));
                }
            }
        }
    }

    /// A replay phase over an internally built ascending request.
    fn replay(
        &self,
        payload: A::ListRequest,
        expected_end: ExpectedEnd,
        stage: LedgerStreamStage,
        output: ReplayOutput,
    ) -> Phase<A> {
        Phase::Replay(Box::new(Replay {
            list_machine: self.list_machine(payload, expected_end, stage),
            output,
        }))
    }

    /// An ascending List machine for an internally built request, paged by `list_page_limit`.
    fn list_machine(
        &self,
        mut payload: A::ListRequest,
        expected_end: ExpectedEnd,
        stage: LedgerStreamStage,
    ) -> ListMachine<A> {
        A::options_mut(&mut payload).limit = self.config.list_page_limit;
        ListMachine::new(
            self.client.clone(),
            payload,
            expected_end,
            ListScanDirection::Ascending,
            LedgerTipPolicy::WaitForExpectedBound,
            self.observability.clone(),
            stage,
        )
    }

    /// Delivers `frame` unless it lies behind committed progress, then continues with `next`.
    fn deliver(&mut self, next: Phase<A>, frame: LedgerFrame<A::Item, A::Cursor>) -> Step<A> {
        match self.accept_frame(frame) {
            Some(output) => Step::Yield(next, output),
            None => Step::Next(next),
        }
    }

    async fn read_initial_tip(&mut self) -> Step<A> {
        match self
            .read_service_checkpoint_height(LedgerStreamStage::InitialReplay)
            .await
        {
            Err(status) => terminal(status),
            // Wait for `start_checkpoint`; otherwise `[start_checkpoint, checkpoint_height + 1)`
            // is invalid.
            Ok(checkpoint_height)
                if A::start_checkpoint(&self.list_template)
                    .is_some_and(|start_checkpoint| checkpoint_height < start_checkpoint) =>
            {
                tokio::time::sleep(self.config.ledger_tip_poll_interval).await;
                Step::Next(Phase::ReadInitialTip)
            }
            Ok(checkpoint_height) => {
                match build_initial_list_request::<A>(&self.list_template, checkpoint_height) {
                    Ok((payload, expected_end)) => Step::Next(self.replay(
                        payload,
                        expected_end,
                        LedgerStreamStage::InitialReplay,
                        ReplayOutput::Yield,
                    )),
                    Err(status) => terminal(status),
                }
            }
        }
    }

    async fn step_replay(&mut self, mut replay: Box<Replay<A>>) -> Step<A> {
        let rpc_event = replay.list_machine.poll_rpc().await;
        match replay.list_machine.process_event(rpc_event, &self.config) {
            ListAction::Frame {
                response,
                progress,
                complete,
            } => {
                let frame = match list_frame::<A>(response, progress) {
                    Ok(frame) => frame,
                    Err(status) => return terminal(status),
                };
                let output = replay.output;
                let next = if complete {
                    self.finish_replay()
                } else {
                    Phase::Replay(replay)
                };
                match (self.accept_frame(frame), output) {
                    (Some(frame_output), ReplayOutput::Yield) => Step::Yield(next, frame_output),
                    _ => Step::Next(next),
                }
            }
            ListAction::Continue => Step::Next(Phase::Replay(replay)),
            ListAction::Terminal(status) => terminal(status),
        }
    }

    /// Follows the ledger with the stream's delivery once a replay reaches its tip.
    fn finish_replay(&self) -> Phase<A> {
        match self.delivery {
            Delivery::Subscribe => Phase::Subscribing(SubscribingPhase::Attempt),
            Delivery::Poll => Phase::Polling(PollingPhase::Sleep),
        }
    }

    /// The replay from committed progress through `checkpoint_height`, or `None` when committed
    /// progress already passes it. With nothing committed, lists only the tip checkpoint, without
    /// yielding its frames, to establish a resume point.
    fn catch_up(
        &self,
        checkpoint_height: u64,
        stage: LedgerStreamStage,
    ) -> Result<Option<Phase<A>>> {
        match &self.committed_progress {
            None => {
                let (payload, expected_end) = build_live_tip_baseline_list_request::<A>(
                    &self.list_template,
                    checkpoint_height,
                )?;
                Ok(Some(self.replay(
                    payload,
                    expected_end,
                    stage,
                    ReplayOutput::Suppress,
                )))
            }
            // The position lies at or past the start of checkpoint `checkpoint_height + 1`.
            Some(committed) if committed.position.checkpoint > checkpoint_height => Ok(None),
            Some(committed) => {
                let (payload, expected_end) = build_polling_list_request::<A>(
                    &self.list_template,
                    committed,
                    checkpoint_height,
                )?;
                Ok(Some(self.replay(
                    payload,
                    expected_end,
                    stage,
                    ReplayOutput::Yield,
                )))
            }
        }
    }

    async fn step_subscribing(&mut self, phase: SubscribingPhase<A>) -> Step<A> {
        match phase {
            SubscribingPhase::Attempt => {
                let observability_stage = if self.committed_progress.is_some() {
                    LedgerStreamStage::LiveSubscription
                } else {
                    LedgerStreamStage::LiveTipStartup
                };
                match self.subscription_attempt(observability_stage).await {
                    Err(status) => terminal(status),
                    Ok(SubscriptionAttempt::Failed {
                        status,
                        failure_phase,
                    }) => self.schedule_connect_retry(status, failure_phase, observability_stage),
                    Ok(SubscriptionAttempt::Established {
                        stream,
                        first_frame,
                    }) => {
                        self.subscription_retry.reset(&self.observability);
                        let gap = self
                            .committed_progress
                            .as_ref()
                            .and_then(|committed| recovery_gap::<A>(committed, &first_frame));
                        match gap {
                            Some(gap) => self.enter_gap_replay(gap, stream, first_frame),
                            None => {
                                let live = LiveSubscription {
                                    stream,
                                    furthest: first_frame.progress.clone(),
                                };
                                self.deliver(
                                    Phase::Subscribing(SubscribingPhase::Live(live)),
                                    first_frame,
                                )
                            }
                        }
                    }
                }
            }
            SubscribingPhase::RetryDelay(delay) => {
                tokio::time::sleep(delay).await;
                Step::Next(Phase::Subscribing(SubscribingPhase::ReadRecoveryTip))
            }
            SubscribingPhase::ReadRecoveryTip => match self
                .read_service_checkpoint_height(LedgerStreamStage::GapRecovery)
                .await
            {
                Err(status) => terminal(status),
                Ok(checkpoint_height) => {
                    match self.catch_up(checkpoint_height, LedgerStreamStage::GapRecovery) {
                        Err(status) => terminal(status),
                        Ok(None) => Step::Next(Phase::Subscribing(SubscribingPhase::Attempt)),
                        Ok(Some(replay)) => {
                            self.observability
                                .emit(|| LedgerStreamEvent::GapRecoveryStarted {
                                    family: A::FAMILY,
                                });
                            Step::Next(replay)
                        }
                    }
                }
            },
            SubscribingPhase::GapReplay(mut gap) => {
                let rpc_event =
                    if let BufferedSubscriptionState::Active(live) = &mut gap.subscription_state {
                        tokio::select! {
                            event = gap.list_machine.poll_rpc() => Some(event),
                            result = live.stream.next() => {
                                if gap.buffer_subscription_result(
                                    result,
                                    self.item_required,
                                    &self.config,
                                ) {
                                    self.subscription_retry.reset(&self.observability);
                                }
                                None
                            }
                        }
                    } else {
                        Some(gap.list_machine.poll_rpc().await)
                    };
                let Some(rpc_event) = rpc_event else {
                    return Step::Next(Phase::Subscribing(SubscribingPhase::GapReplay(gap)));
                };
                match gap.list_machine.process_event(rpc_event, &self.config) {
                    ListAction::Frame {
                        response,
                        progress,
                        complete,
                    } => {
                        let frame = match list_frame::<A>(response, progress) {
                            Ok(frame) => frame,
                            Err(status) => return terminal(status),
                        };
                        let next = if complete {
                            SubscribingPhase::DrainGapBuffer(
                                (*gap).into_buffered_subscription_drain(),
                            )
                        } else {
                            SubscribingPhase::GapReplay(gap)
                        };
                        self.deliver(Phase::Subscribing(next), frame)
                    }
                    ListAction::Continue => {
                        Step::Next(Phase::Subscribing(SubscribingPhase::GapReplay(gap)))
                    }
                    ListAction::Terminal(status) => terminal(status),
                }
            }
            SubscribingPhase::DrainGapBuffer(mut delivery) => {
                if let Some(frame) = delivery.buffered_subscription_frames.pop_front() {
                    self.deliver(
                        Phase::Subscribing(SubscribingPhase::DrainGapBuffer(delivery)),
                        frame,
                    )
                } else {
                    match delivery.subscription_state {
                        BufferedSubscriptionState::Failed(status) => self.schedule_connect_retry(
                            status,
                            FailurePhase::Body,
                            LedgerStreamStage::GapRecovery,
                        ),
                        BufferedSubscriptionState::DroppedAtBufferLimit => {
                            Step::Next(Phase::Subscribing(SubscribingPhase::Attempt))
                        }
                        BufferedSubscriptionState::Active(live) => {
                            Step::Next(Phase::Subscribing(SubscribingPhase::Live(live)))
                        }
                    }
                }
            }
            SubscribingPhase::Live(mut live) => match live.stream.next().await {
                Some(Ok(response)) => match A::parse_live(response, self.item_required) {
                    Err(status) => terminal(status),
                    Ok(frame) => match live.furthest.advance(
                        &frame.progress,
                        ListScanDirection::Ascending,
                        frame.item.is_some(),
                        "subscription",
                    ) {
                        Err(status) => terminal(status),
                        Ok((position_advanced, coverage_advanced)) => {
                            if position_advanced || coverage_advanced {
                                self.subscription_retry.reset(&self.observability);
                            }
                            // A subscription that started behind committed progress repeats
                            // frames until it passes that progress.
                            self.deliver(Phase::Subscribing(SubscribingPhase::Live(live)), frame)
                        }
                    },
                },
                Some(Err(status)) => {
                    self.emit_subscription_interruption(
                        &status,
                        LedgerStreamStage::LiveSubscription,
                    );
                    self.schedule_connect_retry(
                        status,
                        FailurePhase::Body,
                        LedgerStreamStage::LiveSubscription,
                    )
                }
                None => {
                    let status = Status::unavailable("subscription stream ended unexpectedly");
                    self.emit_subscription_interruption(
                        &status,
                        LedgerStreamStage::LiveSubscription,
                    );
                    self.schedule_connect_retry(
                        status,
                        FailurePhase::Body,
                        LedgerStreamStage::LiveSubscription,
                    )
                }
            },
        }
    }

    async fn step_polling(&mut self, phase: PollingPhase) -> Step<A> {
        match phase {
            PollingPhase::Sleep => {
                tokio::time::sleep(self.config.ledger_tip_poll_interval).await;
                Step::Next(Phase::Polling(PollingPhase::ReadTip))
            }
            PollingPhase::ReadTip => {
                let stage = if self.committed_progress.is_some() {
                    LedgerStreamStage::PollingTail
                } else {
                    LedgerStreamStage::PollingBaseline
                };
                match self.read_service_checkpoint_height(stage).await {
                    Err(status) => terminal(status),
                    Ok(checkpoint_height) => match self.catch_up(checkpoint_height, stage) {
                        Err(status) => terminal(status),
                        Ok(None) => Step::Next(Phase::Polling(PollingPhase::Sleep)),
                        Ok(Some(replay)) => Step::Next(replay),
                    },
                }
            }
        }
    }

    async fn read_service_checkpoint_height(
        &mut self,
        observability_stage: LedgerStreamStage,
    ) -> Result<u64> {
        loop {
            let request = Request::new(GetServiceInfoRequest::default());
            let mut client = self.client.ledger_client();
            let future = client.get_service_info(request);
            let started_at = self.observability.start_timer();
            let result = future.await;
            self.observability.emit_rpc_response(
                started_at,
                A::FAMILY,
                LedgerStreamOperation::GetServiceInfo,
                observability_stage,
                &result,
            );
            match result {
                Ok(response) => {
                    let checkpoint_height =
                        response.into_inner().checkpoint_height.ok_or_else(|| {
                            Status::data_loss(
                                "GetServiceInfo response is missing checkpoint_height",
                            )
                        })?;
                    self.service_info_retry.reset(&self.observability);
                    return Ok(checkpoint_height);
                }
                Err(status) => {
                    let Some(delay) = self.service_info_retry.retry_delay(
                        &status,
                        // Unary dispatch and body share this await. Remote statuses retain dispatch
                        // classification; local tonic transport sources allow reset-code retry.
                        FailurePhase::Dispatch,
                        RequestBounds::CallerSupplied,
                        &self.config,
                        &self.observability,
                        observability_stage,
                    ) else {
                        return Err(status);
                    };
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    fn emit_subscription_interruption(
        &self,
        status: &Status,
        observability_stage: LedgerStreamStage,
    ) {
        self.observability
            .emit(|| LedgerStreamEvent::SubscriptionStreamInterrupted {
                family: A::FAMILY,
                stage: observability_stage,
                status: status.clone(),
            });
    }

    fn enter_gap_replay(
        &mut self,
        gap: RecoveryGap<A::Cursor>,
        new_subscription_stream: BoxStream<A::SubscribeResponse>,
        new_subscription_frame: LedgerFrame<A::Item, A::Cursor>,
    ) -> Step<A> {
        // List fills the interval before this frame before buffered live data is delivered.
        let (payload, expected_end) =
            match build_recovery_list_request::<A>(self.list_template.clone(), &gap) {
                Ok(request) => request,
                Err(status) => return terminal(status),
            };
        self.observability
            .emit(|| LedgerStreamEvent::GapRecoveryStarted { family: A::FAMILY });
        let new_live_subscription = LiveSubscription {
            stream: new_subscription_stream,
            furthest: new_subscription_frame.progress.clone(),
        };
        let recovery_list_machine =
            self.list_machine(payload, expected_end, LedgerStreamStage::GapRecovery);
        Step::Next(Phase::Subscribing(SubscribingPhase::GapReplay(Box::new(
            GapReplay::new(
                recovery_list_machine,
                new_live_subscription,
                new_subscription_frame,
                &self.config,
            ),
        ))))
    }

    async fn subscription_attempt(
        &mut self,
        observability_stage: LedgerStreamStage,
    ) -> Result<SubscriptionAttempt<A>> {
        let request = Request::new(self.subscribe_payload.clone());
        let mut stream = match dispatch_subscription::<A>(
            &self.client,
            &self.observability,
            request,
            observability_stage,
        )
        .await
        {
            Ok(stream) => stream,
            Err(status) => {
                return Ok(SubscriptionAttempt::Failed {
                    status,
                    failure_phase: FailurePhase::Dispatch,
                });
            }
        };
        let first_frame = match stream.next().await {
            Some(Ok(response)) => A::parse_live(response, self.item_required)?,
            Some(Err(status)) => {
                self.emit_subscription_interruption(&status, observability_stage);
                return Ok(SubscriptionAttempt::Failed {
                    status,
                    failure_phase: FailurePhase::Body,
                });
            }
            None => {
                let status = Status::unavailable("subscription stream ended unexpectedly");
                self.emit_subscription_interruption(&status, observability_stage);
                return Ok(SubscriptionAttempt::Failed {
                    status,
                    failure_phase: FailurePhase::Body,
                });
            }
        };
        Ok(SubscriptionAttempt::Established {
            stream,
            first_frame,
        })
    }

    fn schedule_connect_retry(
        &mut self,
        status: Status,
        failure_phase: FailurePhase,
        observability_stage: LedgerStreamStage,
    ) -> Step<A> {
        match self.subscription_retry.retry_delay(
            &status,
            failure_phase,
            RequestBounds::CallerSupplied,
            &self.config,
            &self.observability,
            observability_stage,
        ) {
            Some(delay) => Step::Next(Phase::Subscribing(SubscribingPhase::RetryDelay(delay))),
            None => terminal(status),
        }
    }

    /// Commits `frame` and returns its output when it lies past committed progress, or past the
    /// stream's start while nothing is committed. An item frame's position is its item's key with
    /// the finest index incremented, so the item lies before that point exactly when the frame's
    /// position does not pass it.
    fn accept_frame(&mut self, frame: LedgerFrame<A::Item, A::Cursor>) -> Option<A::Output> {
        let LedgerFrame { item, progress } = frame;
        let floor = self
            .committed_progress
            .as_ref()
            .map(|committed| committed.position)
            .or(self.start_position);
        if floor.is_some_and(|floor| progress.position <= floor) {
            return None;
        }
        self.committed_progress = Some(progress.clone());
        Some(A::into_output(item.map(|item| item.payload), progress))
    }
}
