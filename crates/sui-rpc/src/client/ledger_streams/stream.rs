use std::time::Duration;

use futures::StreamExt;
use tonic::Request;
use tonic::Status;
use tonic::codegen::BoxStream;

use super::super::Client;
use super::super::Result;
use super::adapter::CursorDomain;
use super::adapter::ListResponseParts;
use super::adapter::ListScanDirection;
use super::adapter::LiveFrame;
use super::adapter::Progress;
use super::adapter::ProgressAdvance;
use super::adapter::Recovery;
use super::adapter::RecoveryGap;
use super::adapter::SubscriptionAdapter;
use super::adapter::validate_caller_read_mask;
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
use super::retry::RetryState;
use super::subscription::BufferedSubscriptionDrain;
use super::subscription::BufferedSubscriptionState;
use super::subscription::GapReplay;
use super::subscription::LiveSubscription;
use super::types::Delivery;
use super::types::LedgerStreamConfig;
use crate::proto::sui::rpc::v2::GetServiceInfoRequest;
use crate::proto::sui::rpc::v2::Watermark;
use prost::bytes::Bytes;

enum Phase<A: SubscriptionAdapter> {
    Prologue(ProloguePhase<A>),
    Subscribing(SubscribingPhase<A>),
    Polling(PollingPhase<A>),
    Terminal(Status),
    Done,
}

pub(super) enum Start {
    Tip,
    Checkpoint(u64),
    After(Bytes),
}

enum ProloguePhase<A: SubscriptionAdapter> {
    ReadInitialTip,
    Replay {
        // `Phase` moves by value; prologue replay allocates this boxed machine once.
        list_machine: Box<ListMachine<A>>,
        target_checkpoint: u64,
    },
}

enum SubscribingPhase<A: SubscriptionAdapter> {
    Connect(ConnectMachine<A>),
    MuteUntilCommitted(LiveSubscription<A>),
    GapReplay(Box<GapReplay<A>>),
    DrainGapBuffer(BufferedSubscriptionDrain<A>),
    Live(LiveSubscription<A>),
}

struct ConnectMachine<A: SubscriptionAdapter> {
    context: ConnectContext,
    phase: ConnectPhase<A>,
}

#[derive(Clone, Copy)]
enum ConnectContext {
    LiveTipStartup,
    Resume,
}

enum ConnectPhase<A: SubscriptionAdapter> {
    Attempt,
    RetryDelay {
        delay: Duration,
    },
    ReadRecoveryTip,
    ReplayToRecoveryTip {
        list_machine: Box<ListMachine<A>>,
        target_checkpoint: u64,
        output: ReplayOutput,
    },
}

enum PollingPhase<A: SubscriptionAdapter> {
    ReadBaselineTip,
    Replay {
        list_machine: Box<ListMachine<A>>,
        target_checkpoint: u64,
        output: ReplayOutput,
    },
    Sleep,
    ReadTip,
}

#[derive(Clone, Copy)]
enum ReplayOutput {
    Suppress,
    Yield,
}

struct ReplayFrame<I, P> {
    item: Option<I>,
    watermark: Watermark,
    progress: Option<P>,
}

fn parse_replay_frame<A: SubscriptionAdapter>(
    response: A::ListResponse,
    progress: Option<Progress<A::Cursor>>,
) -> Result<ReplayFrame<A::Item, Progress<A::Cursor>>> {
    let ListResponseParts {
        item, watermark, ..
    } = A::split_list(response)?;
    let watermark =
        watermark.ok_or_else(|| Status::data_loss("List frame is missing its watermark"))?;
    Ok(ReplayFrame {
        item,
        watermark,
        progress,
    })
}

enum ReplayStep<A: SubscriptionAdapter> {
    Yield(A::Output),
    Complete,
    Continue,
    Terminal(Status),
}

enum SubscriptionAttempt<A: SubscriptionAdapter> {
    Established {
        stream: BoxStream<A::SubscribeResponse>,
        first_frame: LiveFrame<A::Item, Progress<A::Cursor>>,
    },
    Failed {
        status: Status,
        failure_phase: FailurePhase,
    },
}

enum SubscribingStep<A: SubscriptionAdapter> {
    Next(SubscribingPhase<A>),
    Yield(SubscribingPhase<A>, A::Output),
    Terminal(Status),
}

enum PollingStep<A: SubscriptionAdapter> {
    Next(PollingPhase<A>),
    Yield(PollingPhase<A>, A::Output),
    Terminal(Status),
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
    committed_progress: Option<Progress<A::Cursor>>,
    committed_item_position: Option<A::ItemPosition>,
    last_covered_checkpoint_height: Option<u64>,
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
        let requested_phase = match start {
            Start::Tip => match delivery {
                Delivery::Subscribe => {
                    Phase::Subscribing(SubscribingPhase::Connect(ConnectMachine {
                        context: ConnectContext::LiveTipStartup,
                        phase: ConnectPhase::Attempt,
                    }))
                }
                Delivery::Poll => Phase::Polling(PollingPhase::ReadBaselineTip),
            },
            Start::Checkpoint(checkpoint) => {
                A::set_start_checkpoint(&mut list_template, Some(checkpoint));
                Phase::Prologue(ProloguePhase::ReadInitialTip)
            }
            Start::After(cursor) => {
                A::options_mut(&mut list_template).after = Some(cursor);
                Phase::Prologue(ProloguePhase::ReadInitialTip)
            }
        };
        let phase = if config.ledger_tip_poll_interval == Duration::ZERO {
            Phase::Terminal(Status::invalid_argument(
                "ledger_tip_poll_interval must be greater than zero",
            ))
        } else if let Err(status) =
            validate_caller_read_mask::<A>(A::list_read_mask(&list_template))
        {
            Phase::Terminal(status)
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
            committed_progress: None,
            committed_item_position: None,
            last_covered_checkpoint_height: None,
        }
    }

    pub(super) async fn next(&mut self) -> Option<Result<A::Output>> {
        loop {
            let phase = self.phase.take().unwrap_or(Phase::Done);
            match phase {
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
                Phase::Prologue(phase) => {
                    let (next_phase, output) = self.step_prologue(phase).await;
                    self.phase = Some(next_phase);
                    if let Some(output) = output {
                        return Some(Ok(output));
                    }
                }
                Phase::Subscribing(phase) => match self.step_subscribing(phase).await {
                    SubscribingStep::Next(next) => {
                        self.phase = Some(Phase::Subscribing(next));
                    }
                    SubscribingStep::Yield(next, output) => {
                        self.phase = Some(Phase::Subscribing(next));
                        return Some(Ok(output));
                    }
                    SubscribingStep::Terminal(status) => {
                        self.phase = Some(Phase::Terminal(status));
                    }
                },
                Phase::Polling(phase) => match self.step_polling(phase).await {
                    PollingStep::Next(next) => {
                        self.phase = Some(Phase::Polling(next));
                    }
                    PollingStep::Yield(next, output) => {
                        self.phase = Some(Phase::Polling(next));
                        return Some(Ok(output));
                    }
                    PollingStep::Terminal(status) => {
                        self.phase = Some(Phase::Terminal(status));
                    }
                },
            }
        }
    }

    async fn step_prologue(&mut self, phase: ProloguePhase<A>) -> (Phase<A>, Option<A::Output>) {
        match phase {
            ProloguePhase::ReadInitialTip => match self
                .read_service_checkpoint_height(LedgerStreamStage::InitialReplay)
                .await
            {
                Err(status) => (Phase::Terminal(status), None),
                // Wait for `start_checkpoint`; otherwise `[start_checkpoint, checkpoint_height + 1)`
                // is invalid.
                Ok(checkpoint_height)
                    if A::start_checkpoint(&self.list_template)
                        .is_some_and(|start_checkpoint| checkpoint_height < start_checkpoint) =>
                {
                    tokio::time::sleep(self.config.ledger_tip_poll_interval).await;
                    (Phase::Prologue(ProloguePhase::ReadInitialTip), None)
                }
                Ok(checkpoint_height) => {
                    match build_initial_list_request::<A>(
                        &self.list_template,
                        checkpoint_height,
                        self.config.list_page_limit,
                    ) {
                        Ok((payload, expected_end)) => (
                            Phase::Prologue(ProloguePhase::Replay {
                                list_machine: Box::new(ListMachine::new(
                                    self.client.clone(),
                                    payload,
                                    expected_end,
                                    ListScanDirection::Ascending,
                                    LedgerTipPolicy::WaitForExpectedBound,
                                    self.observability.clone(),
                                    LedgerStreamStage::InitialReplay,
                                )),
                                target_checkpoint: checkpoint_height,
                            }),
                            None,
                        ),
                        Err(status) => (Phase::Terminal(status), None),
                    }
                }
            },
            ProloguePhase::Replay {
                mut list_machine,
                target_checkpoint,
            } => {
                let rpc_event = list_machine.poll_rpc().await;
                let action = list_machine.process_event(rpc_event, &self.config);
                let complete = matches!(&action, ListAction::Frame { complete: true, .. });
                match self.process_replay_frame(action, target_checkpoint, ReplayOutput::Yield) {
                    ReplayStep::Yield(output) => {
                        let next = if complete {
                            self.finish_prologue_replay(target_checkpoint)
                        } else {
                            Phase::Prologue(ProloguePhase::Replay {
                                list_machine,
                                target_checkpoint,
                            })
                        };
                        (next, Some(output))
                    }
                    ReplayStep::Complete => (self.finish_prologue_replay(target_checkpoint), None),
                    ReplayStep::Continue => (
                        Phase::Prologue(ProloguePhase::Replay {
                            list_machine,
                            target_checkpoint,
                        }),
                        None,
                    ),
                    ReplayStep::Terminal(status) => (Phase::Terminal(status), None),
                }
            }
        }
    }

    async fn step_subscribing(&mut self, phase: SubscribingPhase<A>) -> SubscribingStep<A> {
        match phase {
            SubscribingPhase::Connect(machine) => self.step_connect(machine).await,
            SubscribingPhase::MuteUntilCommitted(mut live) => {
                // This subscription began behind committed progress. Keep its frames hidden
                // until it reaches that point; a later frame bounds the required List replay.
                match live.stream.next().await {
                    Some(Ok(response)) => match A::parse_live(response, self.item_required) {
                        Err(status) => SubscribingStep::Terminal(status),
                        Ok(frame) => match live
                            .last_seen
                            .classify_consecutive_subscription_progress(
                                &frame.progress,
                                frame.item.is_some(),
                            ) {
                            Err(status) => SubscribingStep::Terminal(status),
                            Ok(ProgressAdvance::Unchanged) => {
                                SubscribingStep::Next(SubscribingPhase::MuteUntilCommitted(live))
                            }
                            Ok(
                                ProgressAdvance::CheckpointCoverageAdvanced
                                | ProgressAdvance::CursorAdvanced,
                            ) => {
                                live.last_seen = frame.progress.clone();
                                self.subscription_retry.reset(&self.observability);
                                let Some(last_committed_progress) = self.committed_progress.clone()
                                else {
                                    return SubscribingStep::Terminal(Status::internal(
                                        "muted subscription has no committed progress",
                                    ));
                                };
                                match last_committed_progress.plan_recovery(
                                    &frame.progress,
                                    self.last_covered_checkpoint_height,
                                ) {
                                    Err(status) => SubscribingStep::Terminal(status),
                                    Ok(Recovery::Live) => {
                                        SubscribingStep::Next(SubscribingPhase::Live(live))
                                    }
                                    Ok(Recovery::MuteUntilCommitted) => SubscribingStep::Next(
                                        SubscribingPhase::MuteUntilCommitted(live),
                                    ),
                                    Ok(Recovery::Replay(gap)) => {
                                        self.enter_gap_replay(gap, live.stream, frame)
                                    }
                                }
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
                            ConnectContext::Resume,
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
                            ConnectContext::Resume,
                        )
                    }
                }
            }
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
                    return SubscribingStep::Next(SubscribingPhase::GapReplay(gap));
                };
                match gap.list_machine.process_event(rpc_event, &self.config) {
                    ListAction::Frame {
                        response,
                        progress,
                        complete,
                    } => {
                        let frame = match parse_replay_frame::<A>(response, progress) {
                            Ok(frame) => frame,
                            Err(status) => return SubscribingStep::Terminal(status),
                        };
                        let Some(progress) = frame.progress else {
                            return SubscribingStep::Next(if complete {
                                SubscribingPhase::DrainGapBuffer(
                                    (*gap).into_buffered_subscription_drain(),
                                )
                            } else {
                                SubscribingPhase::GapReplay(gap)
                            });
                        };
                        if frame.item.is_none()
                            && (gap.has_deferred_progress(&progress)
                                || self.progress_is_committed(&progress))
                        {
                            return SubscribingStep::Next(if complete {
                                SubscribingPhase::DrainGapBuffer(
                                    (*gap).into_buffered_subscription_drain(),
                                )
                            } else {
                                SubscribingPhase::GapReplay(gap)
                            });
                        }
                        let mut item = frame.item;
                        if item
                            .as_ref()
                            .is_some_and(|item| gap.replayed_item_was_already_emitted(item))
                        {
                            item = None;
                        } else if let Some(item) = &item {
                            gap.record_replayed_item(item);
                        }
                        self.commit_progress(progress.clone());
                        let next = if complete {
                            SubscribingPhase::DrainGapBuffer(
                                (*gap).into_buffered_subscription_drain(),
                            )
                        } else {
                            SubscribingPhase::GapReplay(gap)
                        };
                        SubscribingStep::Yield(next, self.make_output(item, progress))
                    }
                    ListAction::Continue => SubscribingStep::Next(SubscribingPhase::GapReplay(gap)),
                    ListAction::Terminal(status) => SubscribingStep::Terminal(status),
                }
            }
            SubscribingPhase::DrainGapBuffer(mut delivery) => {
                if let Some(frame) = delivery.buffered_subscription_frames.pop_front() {
                    let cursor_advanced = self
                        .committed_progress
                        .as_ref()
                        .is_none_or(|committed| !committed.same_position(&frame.progress));
                    let checkpoint_coverage_advanced = self
                        .committed_progress
                        .as_ref()
                        .and_then(|committed| committed.checkpoint)
                        .zip(frame.progress.checkpoint)
                        .is_some_and(|(committed, next)| next > committed);
                    let duplicate = frame.item.as_ref().is_some_and(|item| {
                        delivery
                            .replay_item_frontier
                            .as_ref()
                            .is_some_and(|frontier| A::item_position(item) <= frontier)
                    });
                    let item = if duplicate { None } else { frame.item };
                    let next = SubscribingPhase::DrainGapBuffer(delivery);
                    if duplicate {
                        SubscribingStep::Next(next)
                    } else {
                        let yield_frame = item.is_some() || cursor_advanced;
                        if yield_frame || checkpoint_coverage_advanced {
                            self.commit_progress(frame.progress.clone());
                        }
                        if yield_frame {
                            SubscribingStep::Yield(next, self.make_output(item, frame.progress))
                        } else {
                            SubscribingStep::Next(next)
                        }
                    }
                } else {
                    match delivery.subscription_state {
                        BufferedSubscriptionState::Failed(status) => self.schedule_connect_retry(
                            status,
                            FailurePhase::Body,
                            LedgerStreamStage::GapRecovery,
                            ConnectContext::Resume,
                        ),
                        BufferedSubscriptionState::DroppedAtBufferLimit => {
                            SubscribingStep::Next(SubscribingPhase::Connect(ConnectMachine {
                                context: ConnectContext::Resume,
                                phase: ConnectPhase::Attempt,
                            }))
                        }
                        BufferedSubscriptionState::Active(live) => {
                            SubscribingStep::Next(SubscribingPhase::Live(live))
                        }
                    }
                }
            }
            SubscribingPhase::Live(mut live) => match live.stream.next().await {
                Some(Ok(response)) => match A::parse_live(response, self.item_required) {
                    Err(status) => SubscribingStep::Terminal(status),
                    Ok(frame) => match live.last_seen.classify_consecutive_subscription_progress(
                        &frame.progress,
                        frame.item.is_some(),
                    ) {
                        Err(status) => SubscribingStep::Terminal(status),
                        Ok(ProgressAdvance::Unchanged) => {
                            SubscribingStep::Next(SubscribingPhase::Live(live))
                        }
                        Ok(ProgressAdvance::CheckpointCoverageAdvanced) => {
                            live.last_seen = frame.progress.clone();
                            self.commit_progress(frame.progress);
                            self.subscription_retry.reset(&self.observability);
                            SubscribingStep::Next(SubscribingPhase::Live(live))
                        }
                        Ok(ProgressAdvance::CursorAdvanced) => {
                            let LiveFrame { item, progress } = frame;
                            live.last_seen = progress.clone();
                            self.commit_progress(progress.clone());
                            self.subscription_retry.reset(&self.observability);
                            SubscribingStep::Yield(
                                SubscribingPhase::Live(live),
                                self.make_output(item, progress),
                            )
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
                        ConnectContext::Resume,
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
                        ConnectContext::Resume,
                    )
                }
            },
        }
    }

    async fn step_polling(&mut self, phase: PollingPhase<A>) -> PollingStep<A> {
        match phase {
            PollingPhase::ReadBaselineTip => match self
                .read_service_checkpoint_height(LedgerStreamStage::PollingBaseline)
                .await
            {
                Err(status) => PollingStep::Terminal(status),
                Ok(checkpoint_height) => {
                    match build_live_tip_baseline_list_request::<A>(
                        &self.list_template,
                        checkpoint_height,
                        self.config.list_page_limit,
                    ) {
                        Ok((payload, expected_end)) => PollingStep::Next(PollingPhase::Replay {
                            list_machine: Box::new(ListMachine::new(
                                self.client.clone(),
                                payload,
                                expected_end,
                                ListScanDirection::Ascending,
                                LedgerTipPolicy::WaitForExpectedBound,
                                self.observability.clone(),
                                LedgerStreamStage::PollingBaseline,
                            )),
                            target_checkpoint: checkpoint_height,
                            output: ReplayOutput::Suppress,
                        }),
                        Err(status) => PollingStep::Terminal(status),
                    }
                }
            },
            PollingPhase::Replay {
                mut list_machine,
                target_checkpoint,
                output,
            } => {
                let rpc_event = list_machine.poll_rpc().await;
                let action = list_machine.process_event(rpc_event, &self.config);
                let complete = matches!(&action, ListAction::Frame { complete: true, .. });
                match self.process_replay_frame(action, target_checkpoint, output) {
                    ReplayStep::Yield(frame_output) => {
                        if complete {
                            match self.finish_polling_replay(target_checkpoint) {
                                Ok(next) => PollingStep::Yield(next, frame_output),
                                Err(status) => PollingStep::Terminal(status),
                            }
                        } else {
                            PollingStep::Yield(
                                PollingPhase::Replay {
                                    list_machine,
                                    target_checkpoint,
                                    output,
                                },
                                frame_output,
                            )
                        }
                    }
                    ReplayStep::Complete => match self.finish_polling_replay(target_checkpoint) {
                        Ok(next) => PollingStep::Next(next),
                        Err(status) => PollingStep::Terminal(status),
                    },
                    ReplayStep::Continue if complete => {
                        match self.finish_polling_replay(target_checkpoint) {
                            Ok(next) => PollingStep::Next(next),
                            Err(status) => PollingStep::Terminal(status),
                        }
                    }
                    ReplayStep::Continue => PollingStep::Next(PollingPhase::Replay {
                        list_machine,
                        target_checkpoint,
                        output,
                    }),
                    ReplayStep::Terminal(status) => PollingStep::Terminal(status),
                }
            }
            PollingPhase::Sleep => {
                tokio::time::sleep(self.config.ledger_tip_poll_interval).await;
                PollingStep::Next(PollingPhase::ReadTip)
            }
            PollingPhase::ReadTip => match self
                .read_service_checkpoint_height(LedgerStreamStage::PollingTail)
                .await
            {
                Err(status) => PollingStep::Terminal(status),
                Ok(checkpoint_height)
                    if self
                        .last_covered_checkpoint_height
                        .is_some_and(|covered| checkpoint_height <= covered) =>
                {
                    PollingStep::Next(PollingPhase::Sleep)
                }
                Ok(checkpoint_height) => {
                    let Some(committed_progress) = self.committed_progress.as_ref() else {
                        return PollingStep::Terminal(Status::data_loss(
                            "subscription recovery has no committed progress marker",
                        ));
                    };
                    match build_polling_list_request::<A>(
                        &self.list_template,
                        committed_progress,
                        checkpoint_height,
                        self.config.list_page_limit,
                    ) {
                        Ok((payload, expected_end)) => PollingStep::Next(PollingPhase::Replay {
                            list_machine: Box::new(ListMachine::new(
                                self.client.clone(),
                                payload,
                                expected_end,
                                ListScanDirection::Ascending,
                                LedgerTipPolicy::WaitForExpectedBound,
                                self.observability.clone(),
                                LedgerStreamStage::PollingTail,
                            )),
                            target_checkpoint: checkpoint_height,
                            output: ReplayOutput::Yield,
                        }),
                        Err(status) => PollingStep::Terminal(status),
                    }
                }
            },
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

    fn process_replay_frame(
        &mut self,
        action: ListAction<A::ListResponse, Progress<A::Cursor>>,
        target_checkpoint: u64,
        output: ReplayOutput,
    ) -> ReplayStep<A> {
        match action {
            ListAction::Frame {
                response,
                progress,
                complete,
            } => {
                let frame = match parse_replay_frame::<A>(response, progress) {
                    Ok(frame) => frame,
                    Err(status) => return ReplayStep::Terminal(status),
                };
                let Some(progress) = frame.progress else {
                    if !complete {
                        return ReplayStep::Continue;
                    }
                    let Some(cursor) = frame.watermark.cursor else {
                        return ReplayStep::Terminal(Status::data_loss(
                            "subscription recovery has no committed progress marker",
                        ));
                    };
                    let Some(progress) = A::Cursor::position(&cursor, Some(target_checkpoint))
                    else {
                        return ReplayStep::Terminal(Status::internal(
                            "checkpoint-bound watermark names no position",
                        ));
                    };
                    let duplicate = self.progress_is_committed(&progress);
                    self.commit_progress(progress.clone());
                    return if duplicate || matches!(output, ReplayOutput::Suppress) {
                        ReplayStep::Complete
                    } else {
                        ReplayStep::Yield(self.make_output(None, progress))
                    };
                };
                let duplicate = frame.item.is_none() && self.progress_is_committed(&progress);
                self.commit_progress(progress.clone());
                if duplicate || matches!(output, ReplayOutput::Suppress) {
                    self.commit_item_position(&frame.item);
                    if complete {
                        ReplayStep::Complete
                    } else {
                        ReplayStep::Continue
                    }
                } else {
                    ReplayStep::Yield(self.make_output(frame.item, progress))
                }
            }
            ListAction::Continue => ReplayStep::Continue,
            ListAction::Terminal(status) => ReplayStep::Terminal(status),
        }
    }

    fn finish_prologue_replay(&mut self, target_checkpoint: u64) -> Phase<A> {
        self.last_covered_checkpoint_height = Some(
            self.last_covered_checkpoint_height
                .map_or(target_checkpoint, |covered| covered.max(target_checkpoint)),
        );
        if self.committed_progress.is_none() {
            return Phase::Terminal(Status::data_loss(
                "subscription recovery has no committed progress marker",
            ));
        }
        match self.delivery {
            Delivery::Subscribe => Phase::Subscribing(SubscribingPhase::Connect(ConnectMachine {
                context: ConnectContext::Resume,
                phase: ConnectPhase::Attempt,
            })),
            Delivery::Poll => Phase::Polling(PollingPhase::Sleep),
        }
    }

    fn finish_polling_replay(&mut self, target_checkpoint: u64) -> Result<PollingPhase<A>> {
        self.last_covered_checkpoint_height = Some(
            self.last_covered_checkpoint_height
                .map_or(target_checkpoint, |covered| covered.max(target_checkpoint)),
        );
        if self.committed_progress.is_none() {
            return Err(Status::data_loss(
                "subscription recovery has no committed progress marker",
            ));
        }
        Ok(PollingPhase::Sleep)
    }

    fn finish_subscription_recovery_replay(&mut self, target_checkpoint: u64) -> ConnectMachine<A> {
        self.last_covered_checkpoint_height = Some(
            self.last_covered_checkpoint_height
                .map_or(target_checkpoint, |covered| covered.max(target_checkpoint)),
        );
        ConnectMachine {
            context: ConnectContext::Resume,
            phase: ConnectPhase::Attempt,
        }
    }

    fn enter_gap_replay(
        &mut self,
        gap: RecoveryGap,
        new_subscription_stream: BoxStream<A::SubscribeResponse>,
        new_subscription_frame: LiveFrame<A::Item, Progress<A::Cursor>>,
    ) -> SubscribingStep<A> {
        // List fills the interval through this frame before buffered live data is delivered.
        let recovery_list_template = A::list_request_from_subscribe(&self.subscribe_payload);
        let (payload, expected_end) = match build_recovery_list_request::<A>(
            recovery_list_template,
            &gap,
            self.config.list_page_limit,
        ) {
            Ok(request) => request,
            Err(status) => return SubscribingStep::Terminal(status),
        };
        self.observability
            .emit(|| LedgerStreamEvent::GapRecoveryStarted { family: A::FAMILY });
        let new_live_subscription = LiveSubscription {
            stream: new_subscription_stream,
            last_seen: new_subscription_frame.progress.clone(),
        };
        let recovery_list_machine = ListMachine::new(
            self.client.clone(),
            payload,
            expected_end,
            ListScanDirection::Ascending,
            LedgerTipPolicy::WaitForExpectedBound,
            self.observability.clone(),
            LedgerStreamStage::GapRecovery,
        );
        SubscribingStep::Next(SubscribingPhase::GapReplay(Box::new(GapReplay::new(
            recovery_list_machine,
            new_live_subscription,
            new_subscription_frame,
            self.committed_item_position.clone(),
            &self.config,
            gap.replays_boundary_item(),
        ))))
    }

    async fn step_connect(&mut self, machine: ConnectMachine<A>) -> SubscribingStep<A> {
        let ConnectMachine { context, phase } = machine;
        match phase {
            ConnectPhase::Attempt => {
                let observability_stage = match context {
                    ConnectContext::LiveTipStartup => LedgerStreamStage::LiveTipStartup,
                    ConnectContext::Resume => LedgerStreamStage::LiveSubscription,
                };
                match self.subscription_attempt(observability_stage).await {
                    Err(status) => SubscribingStep::Terminal(status),
                    Ok(SubscriptionAttempt::Failed {
                        status,
                        failure_phase,
                    }) => self.schedule_connect_retry(
                        status,
                        failure_phase,
                        observability_stage,
                        context,
                    ),
                    Ok(SubscriptionAttempt::Established {
                        stream,
                        first_frame,
                    }) => {
                        self.subscription_retry.reset(&self.observability);
                        match context {
                            ConnectContext::LiveTipStartup => {
                                let LiveFrame { item, progress } = first_frame;
                                let live = LiveSubscription {
                                    stream,
                                    last_seen: progress.clone(),
                                };
                                self.commit_progress(progress.clone());
                                SubscribingStep::Yield(
                                    SubscribingPhase::Live(live),
                                    self.make_output(item, progress),
                                )
                            }
                            ConnectContext::Resume => {
                                let Some(last_committed_progress) = self.committed_progress.clone()
                                else {
                                    return SubscribingStep::Terminal(Status::data_loss(
                                        "subscription recovery has no committed progress marker",
                                    ));
                                };
                                match last_committed_progress.plan_recovery(
                                    &first_frame.progress,
                                    self.last_covered_checkpoint_height,
                                ) {
                                    Err(status) => SubscribingStep::Terminal(status),
                                    Ok(Recovery::Live) => SubscribingStep::Next(
                                        SubscribingPhase::Live(LiveSubscription {
                                            stream,
                                            last_seen: first_frame.progress,
                                        }),
                                    ),
                                    Ok(Recovery::MuteUntilCommitted) => SubscribingStep::Next(
                                        SubscribingPhase::MuteUntilCommitted(LiveSubscription {
                                            stream,
                                            last_seen: first_frame.progress,
                                        }),
                                    ),
                                    Ok(Recovery::Replay(gap)) => {
                                        self.enter_gap_replay(gap, stream, first_frame)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ConnectPhase::RetryDelay { delay } => {
                tokio::time::sleep(delay).await;
                SubscribingStep::Next(SubscribingPhase::Connect(ConnectMachine {
                    context,
                    phase: ConnectPhase::ReadRecoveryTip,
                }))
            }
            ConnectPhase::ReadRecoveryTip => {
                let committed_progress = self.committed_progress.clone();
                match self
                    .read_service_checkpoint_height(LedgerStreamStage::GapRecovery)
                    .await
                {
                    Err(status) => SubscribingStep::Terminal(status),
                    Ok(checkpoint_height)
                        if committed_progress
                            .as_ref()
                            .and_then(|progress| progress.checkpoint)
                            .is_some_and(|committed| checkpoint_height <= committed) =>
                    {
                        SubscribingStep::Next(SubscribingPhase::Connect(ConnectMachine {
                            context: ConnectContext::Resume,
                            phase: ConnectPhase::Attempt,
                        }))
                    }
                    Ok(checkpoint_height) => {
                        let (request, output) =
                            if let Some(committed_progress) = &committed_progress {
                                (
                                    build_polling_list_request::<A>(
                                        &self.list_template,
                                        committed_progress,
                                        checkpoint_height,
                                        self.config.list_page_limit,
                                    ),
                                    ReplayOutput::Yield,
                                )
                            } else {
                                (
                                    build_live_tip_baseline_list_request::<A>(
                                        &self.list_template,
                                        checkpoint_height,
                                        self.config.list_page_limit,
                                    ),
                                    ReplayOutput::Suppress,
                                )
                            };
                        match request {
                            Err(status) => SubscribingStep::Terminal(status),
                            Ok((payload, expected_end)) => {
                                self.observability
                                    .emit(|| LedgerStreamEvent::GapRecoveryStarted {
                                        family: A::FAMILY,
                                    });
                                SubscribingStep::Next(SubscribingPhase::Connect(ConnectMachine {
                                    context,
                                    phase: ConnectPhase::ReplayToRecoveryTip {
                                        list_machine: Box::new(ListMachine::new(
                                            self.client.clone(),
                                            payload,
                                            expected_end,
                                            ListScanDirection::Ascending,
                                            LedgerTipPolicy::WaitForExpectedBound,
                                            self.observability.clone(),
                                            LedgerStreamStage::GapRecovery,
                                        )),
                                        target_checkpoint: checkpoint_height,
                                        output,
                                    },
                                }))
                            }
                        }
                    }
                }
            }
            ConnectPhase::ReplayToRecoveryTip {
                mut list_machine,
                target_checkpoint,
                output,
            } => {
                let rpc_event = list_machine.poll_rpc().await;
                let action = list_machine.process_event(rpc_event, &self.config);
                let complete = matches!(&action, ListAction::Frame { complete: true, .. });
                match self.process_replay_frame(action, target_checkpoint, output) {
                    ReplayStep::Yield(frame_output) => {
                        let next = if complete {
                            self.finish_subscription_recovery_replay(target_checkpoint)
                        } else {
                            ConnectMachine {
                                context,
                                phase: ConnectPhase::ReplayToRecoveryTip {
                                    list_machine,
                                    target_checkpoint,
                                    output,
                                },
                            }
                        };
                        SubscribingStep::Yield(SubscribingPhase::Connect(next), frame_output)
                    }
                    ReplayStep::Complete => SubscribingStep::Next(SubscribingPhase::Connect(
                        self.finish_subscription_recovery_replay(target_checkpoint),
                    )),
                    ReplayStep::Continue if complete => {
                        SubscribingStep::Next(SubscribingPhase::Connect(
                            self.finish_subscription_recovery_replay(target_checkpoint),
                        ))
                    }
                    ReplayStep::Continue => {
                        SubscribingStep::Next(SubscribingPhase::Connect(ConnectMachine {
                            context,
                            phase: ConnectPhase::ReplayToRecoveryTip {
                                list_machine,
                                target_checkpoint,
                                output,
                            },
                        }))
                    }
                    ReplayStep::Terminal(status) => SubscribingStep::Terminal(status),
                }
            }
        }
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
        if !first_frame.progress.has_checkpoint_coverage() {
            return Err(Status::data_loss(
                "subscription initial frame is missing checkpoint coverage",
            ));
        }
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
        context: ConnectContext,
    ) -> SubscribingStep<A> {
        let Some(delay) = self.subscription_retry.retry_delay(
            &status,
            failure_phase,
            &self.config,
            &self.observability,
            observability_stage,
        ) else {
            return SubscribingStep::Terminal(status);
        };
        SubscribingStep::Next(SubscribingPhase::Connect(ConnectMachine {
            context,
            phase: ConnectPhase::RetryDelay { delay },
        }))
    }

    fn progress_is_committed(&self, progress: &Progress<A::Cursor>) -> bool {
        self.committed_progress
            .as_ref()
            .is_some_and(|committed| committed.same_position(progress))
            || self.committed_progress.is_none()
                && A::request_resume_position(&self.list_template)
                    .as_ref()
                    .is_some_and(|resume| resume.same_position(progress))
    }

    fn commit_item_position(&mut self, item: &Option<A::Item>) {
        if let Some(item) = item {
            self.committed_item_position = Some(A::item_position(item).clone());
        }
    }

    fn make_output(&mut self, item: Option<A::Item>, progress: Progress<A::Cursor>) -> A::Output {
        self.commit_item_position(&item);
        A::into_output(item, progress)
    }

    fn commit_progress(&mut self, mut progress: Progress<A::Cursor>) {
        if let Some(committed) = &self.committed_progress {
            // Preserve known checkpoint coverage when a later watermark omits it.
            progress.inherit_checkpoint_coverage(committed);
        }
        self.committed_progress = Some(progress);
    }
}
