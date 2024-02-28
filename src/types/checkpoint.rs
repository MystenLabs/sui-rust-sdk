use super::Bls12381PublicKey;
use super::CheckpointContentsDigest;
use super::CheckpointDigest;
use super::Digest;
use super::GasCostSummary;

pub type CheckpointSequenceNumber = u64;
pub type CheckpointTimestamp = u64;
pub type EpochId = u64;
pub type StakeUnit = u64;
pub type ProtocolVersion = u64;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
//TODO maybe do a tag?
pub enum CheckpointCommitment {
    EcmhLiveObjectSet(Digest),
    // Other commitment types (e.g. merkle roots) go here.
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct CommitteeMember {
    pub public_key: Bls12381PublicKey,
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub stake: StakeUnit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct EndOfEpochData {
    /// next_epoch_committee is `Some` if and only if the current checkpoint is
    /// the last checkpoint of an epoch.
    /// Therefore next_epoch_committee can be used to pick the last checkpoint of an epoch,
    /// which is often useful to get epoch level summary stats like total gas cost of an epoch,
    /// or the total number of transactions from genesis to the end of an epoch.
    /// The committee is stored as a vector of validator pub key and stake pairs. The vector
    /// should be sorted based on the Committee data structure.
    pub next_epoch_committee: Vec<CommitteeMember>,

    /// The protocol version that is in effect during the epoch that starts immediately after this
    /// checkpoint.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub next_epoch_protocol_version: ProtocolVersion,

    /// Commitments to epoch specific state (e.g. live object set)
    pub epoch_commitments: Vec<CheckpointCommitment>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointSummary {
    pub epoch: EpochId,
    pub sequence_number: CheckpointSequenceNumber,
    /// Total number of transactions committed since genesis, including those in this
    /// checkpoint.
    pub network_total_transactions: u64,
    pub content_digest: CheckpointContentsDigest,
    pub previous_digest: Option<CheckpointDigest>,
    /// The running total gas costs of all transactions included in the current epoch so far
    /// until this checkpoint.
    pub epoch_rolling_gas_cost_summary: GasCostSummary,

    /// Timestamp of the checkpoint - number of milliseconds from the Unix epoch
    /// Checkpoint timestamps are monotonic, but not strongly monotonic - subsequent
    /// checkpoints can have same timestamp if they originate from the same underlining consensus commit
    pub timestamp_ms: CheckpointTimestamp,

    /// Commitments to checkpoint-specific state (e.g. txns in checkpoint, objects read/written in
    /// checkpoint).
    pub checkpoint_commitments: Vec<CheckpointCommitment>,

    /// Present only on the final checkpoint of the epoch.
    pub end_of_epoch_data: Option<EndOfEpochData>,

    /// CheckpointSummary is not an evolvable structure - it must be readable by any version of the
    /// code. Therefore, in order to allow extensions to be added to CheckpointSummary, we allow
    /// opaque data to be added to checkpoints which can be deserialized based on the current
    /// protocol version.
    pub version_specific_data: Vec<u8>,
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
mod serialization {
    use super::*;

    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;

    #[derive(serde_derive::Serialize)]
    struct ReadableCheckpointSummaryRef<'a> {
        #[serde(with = "crate::_serde::ReadableDisplay")]
        epoch: &'a EpochId,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        sequence_number: &'a CheckpointSequenceNumber,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        network_total_transactions: &'a u64,
        content_digest: &'a CheckpointContentsDigest,
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_digest: &'a Option<CheckpointDigest>,
        epoch_rolling_gas_cost_summary: &'a GasCostSummary,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        timestamp_ms: &'a CheckpointTimestamp,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        checkpoint_commitments: &'a Vec<CheckpointCommitment>,
        #[serde(skip_serializing_if = "Option::is_none")]
        end_of_epoch_data: &'a Option<EndOfEpochData>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[serde(with = "::serde_with::As::<crate::_serde::Base64Encoded>")]
        version_specific_data: &'a Vec<u8>,
    }

    #[derive(serde_derive::Deserialize)]
    struct ReadableCheckpointSummary {
        #[serde(with = "crate::_serde::ReadableDisplay")]
        epoch: EpochId,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        sequence_number: CheckpointSequenceNumber,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        network_total_transactions: u64,
        content_digest: CheckpointContentsDigest,
        #[serde(default)]
        previous_digest: Option<CheckpointDigest>,
        epoch_rolling_gas_cost_summary: GasCostSummary,
        #[serde(with = "crate::_serde::ReadableDisplay")]
        timestamp_ms: CheckpointTimestamp,
        #[serde(default)]
        checkpoint_commitments: Vec<CheckpointCommitment>,
        #[serde(default)]
        end_of_epoch_data: Option<EndOfEpochData>,
        #[serde(default)]
        #[serde(with = "::serde_with::As::<crate::_serde::Base64Encoded>")]
        version_specific_data: Vec<u8>,
    }

    #[derive(serde_derive::Serialize)]
    struct BinaryCheckpointSummaryRef<'a> {
        epoch: &'a EpochId,
        sequence_number: &'a CheckpointSequenceNumber,
        network_total_transactions: &'a u64,
        content_digest: &'a CheckpointContentsDigest,
        previous_digest: &'a Option<CheckpointDigest>,
        epoch_rolling_gas_cost_summary: &'a GasCostSummary,
        timestamp_ms: &'a CheckpointTimestamp,
        checkpoint_commitments: &'a Vec<CheckpointCommitment>,
        end_of_epoch_data: &'a Option<EndOfEpochData>,
        version_specific_data: &'a Vec<u8>,
    }

    #[derive(serde_derive::Deserialize)]
    struct BinaryCheckpointSummary {
        epoch: EpochId,
        sequence_number: CheckpointSequenceNumber,
        network_total_transactions: u64,
        content_digest: CheckpointContentsDigest,
        previous_digest: Option<CheckpointDigest>,
        epoch_rolling_gas_cost_summary: GasCostSummary,
        timestamp_ms: CheckpointTimestamp,
        checkpoint_commitments: Vec<CheckpointCommitment>,
        end_of_epoch_data: Option<EndOfEpochData>,
        version_specific_data: Vec<u8>,
    }

    impl Serialize for CheckpointSummary {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let Self {
                epoch,
                sequence_number,
                network_total_transactions,
                content_digest,
                previous_digest,
                epoch_rolling_gas_cost_summary,
                timestamp_ms,
                checkpoint_commitments,
                end_of_epoch_data,
                version_specific_data,
            } = self;

            if serializer.is_human_readable() {
                let readable = ReadableCheckpointSummaryRef {
                    epoch,
                    sequence_number,
                    network_total_transactions,
                    content_digest,
                    previous_digest,
                    epoch_rolling_gas_cost_summary,
                    timestamp_ms,
                    checkpoint_commitments,
                    end_of_epoch_data,
                    version_specific_data,
                };
                readable.serialize(serializer)
            } else {
                let binary = BinaryCheckpointSummaryRef {
                    epoch,
                    sequence_number,
                    network_total_transactions,
                    content_digest,
                    previous_digest,
                    epoch_rolling_gas_cost_summary,
                    timestamp_ms,
                    checkpoint_commitments,
                    end_of_epoch_data,
                    version_specific_data,
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for CheckpointSummary {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let ReadableCheckpointSummary {
                    epoch,
                    sequence_number,
                    network_total_transactions,
                    content_digest,
                    previous_digest,
                    epoch_rolling_gas_cost_summary,
                    timestamp_ms,
                    checkpoint_commitments,
                    end_of_epoch_data,
                    version_specific_data,
                } = Deserialize::deserialize(deserializer)?;
                Ok(Self {
                    epoch,
                    sequence_number,
                    network_total_transactions,
                    content_digest,
                    previous_digest,
                    epoch_rolling_gas_cost_summary,
                    timestamp_ms,
                    checkpoint_commitments,
                    end_of_epoch_data,
                    version_specific_data,
                })
            } else {
                let BinaryCheckpointSummary {
                    epoch,
                    sequence_number,
                    network_total_transactions,
                    content_digest,
                    previous_digest,
                    epoch_rolling_gas_cost_summary,
                    timestamp_ms,
                    checkpoint_commitments,
                    end_of_epoch_data,
                    version_specific_data,
                } = Deserialize::deserialize(deserializer)?;
                Ok(Self {
                    epoch,
                    sequence_number,
                    network_total_transactions,
                    content_digest,
                    previous_digest,
                    epoch_rolling_gas_cost_summary,
                    timestamp_ms,
                    checkpoint_commitments,
                    end_of_epoch_data,
                    version_specific_data,
                })
            }
        }
    }
}
