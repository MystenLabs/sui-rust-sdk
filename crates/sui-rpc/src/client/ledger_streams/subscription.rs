use std::collections::VecDeque;

use tonic::Status;
use tonic::codegen::BoxStream;

use super::super::Result;
use super::adapter::LiveFrame;
use super::adapter::Progress;
use super::adapter::ProgressAdvance;
use super::adapter::SubscriptionAdapter;
use super::list::ListMachine;
use super::observability::LedgerStreamEvent;
use super::observability::LedgerStreamObservability;
use super::observability::LedgerStreamStage;
use super::types::LedgerStreamConfig;

pub(super) struct LiveSubscription<A: SubscriptionAdapter> {
    pub(super) stream: BoxStream<A::SubscribeResponse>,
    pub(super) last_seen: Progress<A::Cursor>,
}

pub(super) enum BufferedSubscriptionState<A: SubscriptionAdapter> {
    Active(LiveSubscription<A>),
    Failed(Status),
    DroppedAtBufferLimit,
}

pub(super) struct GapReplay<A: SubscriptionAdapter> {
    pub(super) list_machine: ListMachine<A>,
    pub(super) subscription_state: BufferedSubscriptionState<A>,
    /// Item frames retain payloads; adjacent progress-only frames coalesce once per contiguous run.
    buffered_subscription_frames: VecDeque<LiveFrame<A::Item, Progress<A::Cursor>>>,
    /// Number of payload-bearing frames currently buffered, bounded by `max_buffered_live_items`
    /// (empty progress-only frames do not count toward this limit).
    buffered_subscription_items: usize,
    replay_item_frontier: Option<A::ItemPosition>,
    observability: LedgerStreamObservability,
}

impl<A: SubscriptionAdapter> GapReplay<A> {
    pub(super) fn new(
        list_machine: ListMachine<A>,
        live: LiveSubscription<A>,
        first: LiveFrame<A::Item, Progress<A::Cursor>>,
        committed_item_position: Option<A::ItemPosition>,
        config: &LedgerStreamConfig,
        replays_boundary_item: bool,
    ) -> Self {
        let observability = list_machine.observability.clone();
        let mut gap = Self {
            list_machine,
            subscription_state: BufferedSubscriptionState::Active(live),
            buffered_subscription_frames: VecDeque::new(),
            buffered_subscription_items: 0,
            observability,
            replay_item_frontier: committed_item_position,
        };
        if !replays_boundary_item {
            gap.retain_frame(first, Some(config));
        }
        gap
    }

    fn retain_frame(
        &mut self,
        frame: LiveFrame<A::Item, Progress<A::Cursor>>,
        config: Option<&LedgerStreamConfig>,
    ) {
        if frame.item.is_none()
            && let Some(deferred_tail) = self.buffered_subscription_frames.back_mut()
            && deferred_tail.item.is_none()
        {
            deferred_tail.progress = frame.progress;
            return;
        }

        if frame.item.is_some() {
            self.buffered_subscription_items = self.buffered_subscription_items.saturating_add(1);
        }
        self.buffered_subscription_frames.push_back(frame);
        if let Some(config) = config
            && self.buffered_subscription_items >= config.max_buffered_live_items.get()
            && matches!(
                &self.subscription_state,
                BufferedSubscriptionState::Active(_)
            )
        {
            self.subscription_state = BufferedSubscriptionState::DroppedAtBufferLimit;
            let buffered_items = self.buffered_subscription_items;
            let limit = config.max_buffered_live_items.get();
            self.observability
                .emit(|| LedgerStreamEvent::SubscriptionBufferLimitReached {
                    family: A::FAMILY,
                    buffered_items,
                    limit,
                });
        }
    }

    pub(super) fn has_deferred_progress(&self, progress: &Progress<A::Cursor>) -> bool {
        self.buffered_subscription_frames
            .iter()
            .any(|frame| frame.progress.same_position(progress))
    }

    pub(super) fn replayed_item_was_already_emitted(&self, item: &A::Item) -> bool {
        self.replay_item_frontier
            .as_ref()
            .is_some_and(|frontier| A::item_position(item) <= frontier)
    }

    pub(super) fn record_replayed_item(&mut self, item: &A::Item) {
        let position = A::item_position(item);
        if self
            .replay_item_frontier
            .as_ref()
            .is_none_or(|frontier| position > frontier)
        {
            self.replay_item_frontier = Some(position.clone());
        }
    }

    /// Returns whether a valid buffered frame advanced subscription progress, controlling retry
    /// backoff reset.
    pub(super) fn buffer_subscription_result(
        &mut self,
        result: Option<Result<A::SubscribeResponse>>,
        item_required: bool,
        config: &LedgerStreamConfig,
    ) -> bool {
        let BufferedSubscriptionState::Active(live) = &mut self.subscription_state else {
            return false;
        };
        let frame = match result {
            Some(Ok(response)) => match A::parse_live(response, item_required) {
                Ok(frame) => frame,
                Err(status) => {
                    self.subscription_state = BufferedSubscriptionState::Failed(status);
                    return false;
                }
            },
            Some(Err(status)) => {
                self.observability
                    .emit(|| LedgerStreamEvent::SubscriptionStreamInterrupted {
                        family: A::FAMILY,
                        stage: LedgerStreamStage::GapRecovery,
                        status: status.clone(),
                    });
                self.subscription_state = BufferedSubscriptionState::Failed(status);
                return false;
            }
            None => {
                let status = Status::unavailable("subscription stream ended unexpectedly");
                self.observability
                    .emit(|| LedgerStreamEvent::SubscriptionStreamInterrupted {
                        family: A::FAMILY,
                        stage: LedgerStreamStage::GapRecovery,
                        status: status.clone(),
                    });
                self.subscription_state = BufferedSubscriptionState::Failed(status);
                return false;
            }
        };

        match live
            .last_seen
            .classify_consecutive_subscription_progress(&frame.progress, frame.item.is_some())
        {
            Err(status) => {
                self.subscription_state = BufferedSubscriptionState::Failed(status);
                false
            }
            Ok(ProgressAdvance::Unchanged) => false,
            Ok(ProgressAdvance::CheckpointCoverageAdvanced | ProgressAdvance::CursorAdvanced) => {
                live.last_seen = frame.progress.clone();
                self.retain_frame(frame, Some(config));
                true
            }
        }
    }

    pub(super) fn into_buffered_subscription_drain(self) -> BufferedSubscriptionDrain<A> {
        BufferedSubscriptionDrain {
            subscription_state: self.subscription_state,
            buffered_subscription_frames: self.buffered_subscription_frames,
            replay_item_frontier: self.replay_item_frontier,
        }
    }
}

pub(super) struct BufferedSubscriptionDrain<A: SubscriptionAdapter> {
    pub(super) subscription_state: BufferedSubscriptionState<A>,
    pub(super) buffered_subscription_frames: VecDeque<LiveFrame<A::Item, Progress<A::Cursor>>>,
    pub(super) replay_item_frontier: Option<A::ItemPosition>,
}
