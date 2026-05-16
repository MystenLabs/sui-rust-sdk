//! Committee ratchet driver.
//!
//! [`ratchet_to_checkpoint`] is the entry point. Given a [`Client`] and an
//! [`EpochCache`], it advances the cache forward until the cache covers the
//! requested target checkpoint sequence number — by:
//!
//! 1. Asking the server when the cache's current epoch ends via the v2
//!    `LedgerService.GetEpoch` RPC (reading just the `last_checkpoint`
//!    field).
//! 2. If the current epoch has already ended on the network and the
//!    target falls past it, fetching the epoch's last checkpoint summary
//!    + signature via `LedgerService.GetCheckpoint`.
//! 3. BLS-verifying that summary's aggregate signature against the
//!    cache's current committee.
//! 4. Extracting `next_epoch_committee` from the summary's
//!    `end_of_epoch_data` and applying it as a ratchet update.
//! 5. Looping until the target is covered.
//!
//! Each iteration is one round-trip pair plus one BLS verification. The
//! cache rejects out-of-order or non-consecutive ratchet updates, so a
//! malicious server cannot rewind history or skip an epoch.

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
use super::error::LightClientError;

/// Advance `cache` forward so that it covers `target_seq`.
///
/// On return, `cache.committee_for_checkpoint(target_seq)` is guaranteed
/// to return the committee that signed the checkpoint at `target_seq`
/// (subject to the server returning truthful epoch boundaries; the cache
/// independently BLS-verifies each end-of-epoch summary it accepts).
///
/// If `target_seq` already falls within an epoch the cache knows about
/// (either a completed one or the current one, as far as the network has
/// progressed) this is a single `GetEpoch` round-trip with no state
/// change.
pub async fn ratchet_to_checkpoint(
    client: &mut Client,
    cache: &mut EpochCache,
    target_seq: u64,
) -> Result<(), LightClientError> {
    loop {
        // Targets that fall inside a completed epoch are already covered.
        if target_seq < cache.current_epoch_start() {
            return Ok(());
        }

        // Ask the server where the current cached epoch ends.
        let request = GetEpochRequest::new(cache.current_epoch())
            .with_read_mask(FieldMask::from_paths(["last_checkpoint"]));
        let response = client
            .ledger_client()
            .get_epoch(request)
            .await?
            .into_inner();
        let epoch = response.epoch.ok_or_else(|| {
            LightClientError::Proto(crate::proto::TryFromProtoError::missing("epoch"))
        })?;

        let Some(last_of_epoch) = epoch.last_checkpoint else {
            // The epoch is still ongoing on the server side; the current
            // committee covers `target_seq`.
            return Ok(());
        };

        if last_of_epoch >= target_seq {
            // Target falls within the current epoch; no advance needed.
            return Ok(());
        }

        // Need to ratchet forward. Fetch the end-of-epoch summary +
        // signature, verify, and advance.
        advance_one_epoch(client, cache, last_of_epoch).await?;
    }
}

/// Fetch the end-of-epoch summary for `end_of_epoch_seq`, BLS-verify its
/// aggregate signature against `cache`'s current committee, extract the
/// next epoch's committee from `end_of_epoch_data`, and apply the
/// ratchet update.
async fn advance_one_epoch(
    client: &mut Client,
    cache: &mut EpochCache,
    end_of_epoch_seq: u64,
) -> Result<(), LightClientError> {
    let request = GetCheckpointRequest::by_sequence_number(end_of_epoch_seq)
        .with_read_mask(FieldMask::from_paths(["summary", "signature"]));
    let response = client
        .ledger_client()
        .get_checkpoint(request)
        .await?
        .into_inner();

    let checkpoint = response.checkpoint.ok_or_else(|| {
        LightClientError::Proto(crate::proto::TryFromProtoError::missing("checkpoint"))
    })?;
    let summary_proto = checkpoint.summary.as_ref().ok_or_else(|| {
        LightClientError::Proto(crate::proto::TryFromProtoError::missing("summary"))
    })?;
    let signature_proto = checkpoint.signature.as_ref().ok_or_else(|| {
        LightClientError::Proto(crate::proto::TryFromProtoError::missing("signature"))
    })?;

    let summary: CheckpointSummary = summary_proto.try_into()?;
    let signature: ValidatorAggregatedSignature = signature_proto.try_into()?;

    apply_verified_end_of_epoch(cache, end_of_epoch_seq, &summary, &signature)
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
    cache.apply_ratchet_update(end_of_epoch_seq, next_committee)?;
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
