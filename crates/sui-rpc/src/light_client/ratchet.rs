//! Committee ratchet driver.
//!
//! [`ratchet_to_checkpoint`] is the entry point. Given a [`Client`] and an
//! [`EpochCache`], it advances the cache forward until the cache covers the
//! requested target checkpoint sequence number — in three phases:
//!
//! 1. **Discover** the set of epochs the cache needs to advance through by
//!    walking `LedgerService.GetEpoch` calls sequentially. The discovery
//!    phase reads only `["last_checkpoint"]` (no signature verification)
//!    and terminates as soon as it finds an epoch that either is still
//!    ongoing on the server side or whose last checkpoint covers the
//!    target.
//! 2. **Fetch** the end-of-epoch checkpoint summaries for the discovered
//!    epochs in parallel via `LedgerService.GetCheckpoint`. The
//!    concurrency is capped by [`RatchetConfig::concurrency`]. Each
//!    summary comes back with its aggregate signature.
//! 3. **Apply** each fetched summary sequentially: BLS-verify the
//!    signature against the cache's current committee, extract the next
//!    epoch's committee from the summary's `end_of_epoch_data`, and apply
//!    the ratchet update. Sequencing here is load-bearing — each summary
//!    is verified against the committee that the *previous* summary just
//!    installed.
//!
//! Splitting discovery from fetch lets a fresh ratchet across N epochs
//! incur roughly `N + N/concurrency` round trips of latency rather than
//! `2N` serial round trips. Discovery stays cheap; the heavy
//! summary-fetch round trips are pipelined.

use sui_crypto::bls12381::ValidatorCommitteeSignatureVerifier;
use sui_sdk_types::CheckpointSummary;
use sui_sdk_types::ValidatorAggregatedSignature;
use sui_sdk_types::ValidatorCommittee;

use crate::Client;
use crate::field::FieldMask;
use crate::field::FieldMaskUtil;
use crate::proto::sui::rpc::v2::GetCheckpointRequest;
use crate::proto::sui::rpc::v2::GetEpochRequest;

use super::EpochCache;
use super::RatchetConfig;
use super::error::LightClientError;
use super::retry;

/// Advance `cache` forward so that it covers `target_seq`, with the
/// default [`RatchetConfig`].
pub async fn ratchet_to_checkpoint(
    client: &mut Client,
    cache: &mut EpochCache,
    target_seq: u64,
) -> Result<(), LightClientError> {
    ratchet_to_checkpoint_with_config(client, cache, target_seq, &RatchetConfig::default()).await
}

/// Advance `cache` forward so that it covers `target_seq`, applying the
/// concurrency knobs in `config` to the parallel summary fetch.
///
/// On return, the cache holds the committee for the epoch that contains
/// `target_seq` (subject to the server returning truthful epoch
/// boundaries; the cache independently BLS-verifies each end-of-epoch
/// summary it accepts).
pub async fn ratchet_to_checkpoint_with_config(
    client: &mut Client,
    cache: &mut EpochCache,
    target_seq: u64,
    config: &RatchetConfig,
) -> Result<(), LightClientError> {
    let to_advance =
        discover_epochs_to_advance(client, cache.current_epoch(), target_seq, config).await?;
    if to_advance.is_empty() {
        return Ok(());
    }

    let fetched = fetch_end_of_epoch_summaries(client, &to_advance, config).await?;

    for (end_seq, summary, signature) in fetched {
        apply_verified_end_of_epoch(cache, end_seq, &summary, &signature)?;
    }
    Ok(())
}

/// One discovered epoch to ratchet through: the epoch's index, and the
/// sequence number of its last checkpoint (the one whose summary needs
/// fetching to install the next epoch's committee).
#[derive(Debug, Clone, Copy)]
struct EpochToAdvance {
    epoch: u64,
    end_of_epoch_seq: u64,
}

/// Walk `GetEpoch` calls forward from `start_epoch` to determine which
/// epochs need ratcheting through to cover `target_seq`.
///
/// Returns the discovered epochs in ascending order. An empty result
/// means the cache's current epoch already covers `target_seq` (either
/// because it is still ongoing on the server side, or because it has
/// already ended at a checkpoint at or after `target_seq`).
///
/// The walk is capped at `max_gap` discovered epochs. Hitting the cap
/// is a strong signal that the server is misbehaving — either it
/// genuinely thinks the chain is that far ahead of the cache (in
/// which case the client should bootstrap from a more recent trust
/// anchor instead of ratcheting forever) or it is malicious. Either
/// way, returning [`LightClientError::RatchetGapTooLarge`] is more
/// useful than continuing to issue `GetEpoch` calls.
async fn discover_epochs_to_advance(
    client: &mut Client,
    start_epoch: u64,
    target_seq: u64,
    config: &RatchetConfig,
) -> Result<Vec<EpochToAdvance>, LightClientError> {
    let mut to_advance = Vec::new();
    let mut epoch_number = start_epoch;
    loop {
        if to_advance.len() as u64 >= config.max_ratchet_gap {
            return Err(LightClientError::RatchetGapTooLarge {
                current: start_epoch,
                target: epoch_number,
                max: config.max_ratchet_gap,
            });
        }

        let response = {
            let mut attempt: u32 = 0;
            loop {
                let request = GetEpochRequest::new(epoch_number)
                    .with_read_mask(FieldMask::from_paths(["last_checkpoint"]));
                match client.ledger_client().get_epoch(request).await {
                    Ok(resp) => break resp.into_inner(),
                    Err(status) => {
                        retry::step(config, LightClientError::Rpc(status), &mut attempt).await?;
                    }
                }
            }
        };

        let epoch = response.epoch.ok_or_else(|| {
            LightClientError::Proto(crate::proto::TryFromProtoError::missing("epoch"))
        })?;

        let Some(last_of_epoch) = epoch.last_checkpoint else {
            // Epoch is still ongoing on the server side; everything
            // beyond it is unreachable.
            break;
        };

        if last_of_epoch >= target_seq {
            // This epoch covers the target; no further advance needed.
            break;
        }

        to_advance.push(EpochToAdvance {
            epoch: epoch_number,
            end_of_epoch_seq: last_of_epoch,
        });
        epoch_number += 1;
    }
    Ok(to_advance)
}

/// Fetch the end-of-epoch checkpoint summaries for `to_advance` in
/// parallel, returning them in ascending epoch order.
///
/// Each future fetches `["summary.bcs", "signature"]` and BCS-decodes
/// the summary directly, skipping the proto-to-SDK conversion layer.
async fn fetch_end_of_epoch_summaries(
    client: &mut Client,
    to_advance: &[EpochToAdvance],
    config: &RatchetConfig,
) -> Result<Vec<(u64, CheckpointSummary, ValidatorAggregatedSignature)>, LightClientError> {
    use futures::stream::StreamExt;
    use futures::stream::TryStreamExt;

    // Build a per-task `LedgerServiceClient` so the futures can run
    // concurrently without borrowing `client` mutably across awaits.
    let ledger_clients: Vec<_> = (0..to_advance.len())
        .map(|_| client.ledger_client())
        .collect();

    let concurrency = config.concurrency.max(1);
    let futures =
        to_advance
            .iter()
            .copied()
            .zip(ledger_clients)
            .map(|(item, mut ledger)| async move {
                // Inline manual retry: `LedgerServiceClient<BoxedChannel>`
                // isn't `Clone` (its `BoxService` backend isn't), so we
                // can't drop it into a `FnMut`-callable closure for
                // `with_backoff`. Each future owns its own ledger and
                // retries against it sequentially.
                let response = {
                    let mut attempt: u32 = 0;
                    loop {
                        let request =
                            GetCheckpointRequest::by_sequence_number(item.end_of_epoch_seq)
                                .with_read_mask(FieldMask::from_paths([
                                    "summary.bcs",
                                    "signature",
                                ]));
                        match ledger.get_checkpoint(request).await {
                            Ok(resp) => break resp.into_inner(),
                            Err(status) => {
                                retry::step(config, LightClientError::Rpc(status), &mut attempt)
                                    .await?;
                            }
                        }
                    }
                };
                let checkpoint = response.checkpoint.ok_or_else(|| {
                    LightClientError::Proto(crate::proto::TryFromProtoError::missing("checkpoint"))
                })?;
                let summary_bcs = checkpoint
                    .summary
                    .as_ref()
                    .and_then(|s| s.bcs.as_ref())
                    .ok_or_else(|| {
                        LightClientError::Proto(crate::proto::TryFromProtoError::missing(
                            "summary.bcs",
                        ))
                    })?;
                let signature_proto = checkpoint.signature.as_ref().ok_or_else(|| {
                    LightClientError::Proto(crate::proto::TryFromProtoError::missing("signature"))
                })?;
                let summary: CheckpointSummary = summary_bcs.deserialize()?;
                let signature: ValidatorAggregatedSignature = signature_proto.try_into()?;
                Ok::<_, LightClientError>((item.epoch, item.end_of_epoch_seq, summary, signature))
            });

    let mut fetched: Vec<_> = futures::stream::iter(futures)
        .buffer_unordered(concurrency)
        .try_collect()
        .await?;

    // `buffer_unordered` may return results out of order; the apply
    // phase requires ascending epoch order to feed the state machine
    // correctly.
    fetched.sort_by_key(|(epoch, _, _, _)| *epoch);
    Ok(fetched
        .into_iter()
        .map(|(_, end_seq, summary, signature)| (end_seq, summary, signature))
        .collect())
}

/// Verify `summary`'s BLS aggregate signature against `cache`'s current
/// committee, then extract the next committee from
/// `summary.end_of_epoch_data` and apply it as a ratchet update.
///
/// Exposed for testing the pure (non-RPC) half of the ratchet step.
pub(crate) fn apply_verified_end_of_epoch(
    cache: &mut EpochCache,
    end_of_epoch_seq: u64,
    summary: &CheckpointSummary,
    signature: &ValidatorAggregatedSignature,
) -> Result<(), LightClientError> {
    let verifier = ValidatorCommitteeSignatureVerifier::new(cache.current_committee().clone())?;
    verifier.verify_checkpoint_summary(summary, signature)?;

    let next_committee = extract_next_epoch_committee(cache, summary, end_of_epoch_seq)?;
    cache.apply_ratchet_update(next_committee)?;
    Ok(())
}

/// Build the next epoch's committee from `summary.end_of_epoch_data`,
/// pairing it with the cache's next epoch number.
///
/// Returns [`LightClientError::MissingEndOfEpochData`] if `summary` has no
/// `end_of_epoch_data` (i.e. it was not in fact the last checkpoint of an
/// epoch, despite the server having advertised it as such).
fn extract_next_epoch_committee(
    cache: &EpochCache,
    summary: &CheckpointSummary,
    checkpoint: u64,
) -> Result<ValidatorCommittee, LightClientError> {
    let end_of_epoch_data = summary
        .end_of_epoch_data
        .as_ref()
        .ok_or(LightClientError::MissingEndOfEpochData { checkpoint })?;

    Ok(ValidatorCommittee {
        epoch: cache.current_epoch() + 1,
        members: end_of_epoch_data.next_epoch_committee.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use sui_sdk_types::CheckpointCommitment;
    use sui_sdk_types::Digest;
    use sui_sdk_types::EndOfEpochData;
    use sui_sdk_types::GasCostSummary;
    use sui_sdk_types::ValidatorCommitteeMember;

    fn committee(epoch: u64) -> ValidatorCommittee {
        ValidatorCommittee {
            epoch,
            members: Vec::new(),
        }
    }

    fn make_end_of_epoch_summary(
        epoch: u64,
        members: Vec<ValidatorCommitteeMember>,
    ) -> CheckpointSummary {
        CheckpointSummary {
            epoch,
            sequence_number: 99,
            network_total_transactions: 0,
            content_digest: Digest::ZERO,
            previous_digest: None,
            epoch_rolling_gas_cost_summary: GasCostSummary::default(),
            timestamp_ms: 0,
            checkpoint_commitments: Vec::<CheckpointCommitment>::new(),
            end_of_epoch_data: Some(EndOfEpochData {
                next_epoch_committee: members,
                next_epoch_protocol_version: 1,
                epoch_commitments: Vec::new(),
            }),
            version_specific_data: Vec::new(),
        }
    }

    /// `extract_next_epoch_committee` pairs the cache's `current_epoch + 1`
    /// with the members carried in the summary's `end_of_epoch_data`.
    #[test]
    fn extract_next_epoch_committee_uses_summary_members_and_advances_epoch() {
        let cache = EpochCache::new(committee(7));
        // Members content doesn't matter for this test; just check the
        // count and the epoch number.
        let members: Vec<ValidatorCommitteeMember> = Vec::new();
        let summary = make_end_of_epoch_summary(7, members);

        let next = extract_next_epoch_committee(&cache, &summary, 99).unwrap();
        assert_eq!(next.epoch, 8);
        assert!(next.members.is_empty());
    }

    /// A summary with no `end_of_epoch_data` is rejected with the right
    /// error and the named checkpoint sequence number.
    #[test]
    fn extract_next_epoch_committee_requires_end_of_epoch_data() {
        let cache = EpochCache::new(committee(0));
        let summary = CheckpointSummary {
            epoch: 0,
            sequence_number: 42,
            network_total_transactions: 0,
            content_digest: Digest::ZERO,
            previous_digest: None,
            epoch_rolling_gas_cost_summary: GasCostSummary::default(),
            timestamp_ms: 0,
            checkpoint_commitments: Vec::new(),
            end_of_epoch_data: None,
            version_specific_data: Vec::new(),
        };

        let err = extract_next_epoch_committee(&cache, &summary, 42).unwrap_err();
        assert!(
            matches!(
                err,
                LightClientError::MissingEndOfEpochData { checkpoint: 42 }
            ),
            "got {err:?}"
        );
    }
}
