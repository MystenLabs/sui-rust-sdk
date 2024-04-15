use super::CheckpointContentsDigest;
use super::CheckpointDigest;
use super::Digest;
use super::GasCostSummary;
use super::Object;
use super::SignedTransaction;
use super::TransactionDigest;
use super::TransactionEffects;
use super::TransactionEffectsDigest;
use super::TransactionEvents;
use super::UserSignature;
use super::ValidatorAggregatedSignature;
use super::ValidatorCommitteeMember;

pub type CheckpointSequenceNumber = u64;
pub type CheckpointTimestamp = u64;
pub type EpochId = u64;
pub type StakeUnit = u64;
pub type ProtocolVersion = u64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckpointCommitment {
    EcmhLiveObjectSet { digest: Digest },
    // Other commitment types (e.g. merkle roots) go here.
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
    pub next_epoch_committee: Vec<ValidatorCommitteeMember>,

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

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct SignedCheckpointSummary {
    pub checkpoint: CheckpointSummary,
    pub signature: ValidatorAggregatedSignature,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointContents(Vec<CheckpointTransactionInfo>);

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct CheckpointTransactionInfo {
    pub transaction: TransactionDigest,
    pub effects: TransactionEffectsDigest,
    pub signatures: Vec<UserSignature>,
}

#[derive(Clone, Debug)]
pub struct CheckpointData {
    pub checkpoint_summary: SignedCheckpointSummary,
    pub checkpoint_contents: CheckpointContents,
    pub transactions: Vec<CheckpointTransaction>,
}

#[derive(Clone, Debug)]
pub struct CheckpointTransaction {
    /// The input Transaction
    pub transaction: SignedTransaction,
    /// The effects produced by executing this transaction
    pub effects: TransactionEffects,
    /// The events, if any, emitted by this transaciton during execution
    pub events: Option<TransactionEvents>,
    /// The state of all inputs to this transaction as they were prior to execution.
    pub input_objects: Vec<Object>,
    /// The state of all output objects created or mutated by this transaction.
    pub output_objects: Vec<Object>,
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

    impl Serialize for CheckpointContents {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            use serde::ser::SerializeSeq;
            use serde::ser::SerializeTupleVariant;

            if serializer.is_human_readable() {
                serializer.serialize_newtype_struct("CheckpointContents", &self.0)
            } else {
                #[derive(serde_derive::Serialize)]
                struct Digests<'a> {
                    transaction: &'a TransactionDigest,
                    effects: &'a TransactionEffectsDigest,
                }

                struct DigestSeq<'a>(&'a CheckpointContents);
                impl Serialize for DigestSeq<'_> {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                    {
                        let mut seq = serializer.serialize_seq(Some(self.0 .0.len()))?;
                        for txn in &self.0 .0 {
                            let digests = Digests {
                                transaction: &txn.transaction,
                                effects: &txn.effects,
                            };
                            seq.serialize_element(&digests)?;
                        }
                        seq.end()
                    }
                }

                struct SignatureSeq<'a>(&'a CheckpointContents);
                impl Serialize for SignatureSeq<'_> {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                    {
                        let mut seq = serializer.serialize_seq(Some(self.0 .0.len()))?;
                        for txn in &self.0 .0 {
                            seq.serialize_element(&txn.signatures)?;
                        }
                        seq.end()
                    }
                }

                let mut s = serializer.serialize_tuple_variant("CheckpointContents", 0, "V1", 2)?;
                s.serialize_field(&DigestSeq(self))?;
                s.serialize_field(&SignatureSeq(self))?;
                s.end()
            }
        }
    }

    #[derive(serde_derive::Deserialize)]
    struct ExecutionDigests {
        transaction: TransactionDigest,
        effects: TransactionEffectsDigest,
    }

    #[derive(serde_derive::Deserialize)]
    struct BinaryContentsV1 {
        digests: Vec<ExecutionDigests>,
        signatures: Vec<Vec<UserSignature>>,
    }

    #[derive(serde_derive::Deserialize)]
    enum BinaryContents {
        V1(BinaryContentsV1),
    }

    impl<'de> Deserialize<'de> for CheckpointContents {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let contents: Vec<CheckpointTransactionInfo> =
                    Deserialize::deserialize(deserializer)?;
                Ok(Self(contents))
            } else {
                let BinaryContents::V1(BinaryContentsV1 {
                    digests,
                    signatures,
                }) = Deserialize::deserialize(deserializer)?;

                if digests.len() != signatures.len() {
                    return Err(serde::de::Error::custom(
                        "must have same number of signatures as transactions",
                    ));
                }

                Ok(Self(
                    digests
                        .into_iter()
                        .zip(signatures)
                        .map(
                            |(
                                ExecutionDigests {
                                    transaction,
                                    effects,
                                },
                                signatures,
                            )| CheckpointTransactionInfo {
                                transaction,
                                effects,
                                signatures,
                            },
                        )
                        .collect(),
                ))
            }
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    enum ReadableCommitment {
        EcmhLiveObjectSet { digest: Digest },
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum BinaryCommitment {
        EcmhLiveObjectSet { digest: Digest },
    }

    impl Serialize for CheckpointCommitment {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match *self {
                    CheckpointCommitment::EcmhLiveObjectSet { digest } => {
                        ReadableCommitment::EcmhLiveObjectSet { digest }
                    }
                };
                readable.serialize(serializer)
            } else {
                let binary = match *self {
                    CheckpointCommitment::EcmhLiveObjectSet { digest } => {
                        BinaryCommitment::EcmhLiveObjectSet { digest }
                    }
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for CheckpointCommitment {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                Ok(match ReadableCommitment::deserialize(deserializer)? {
                    ReadableCommitment::EcmhLiveObjectSet { digest } => {
                        Self::EcmhLiveObjectSet { digest }
                    }
                })
            } else {
                Ok(match BinaryCommitment::deserialize(deserializer)? {
                    BinaryCommitment::EcmhLiveObjectSet { digest } => {
                        Self::EcmhLiveObjectSet { digest }
                    }
                })
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use base64ct::Base64;
        use base64ct::Encoding;

        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen_test::wasm_bindgen_test as test;

        #[test]
        fn signed_checkpoint_fixture() {
            const FIXTURES: &[&str] = &[
                "CgAAAAAAAAAUAAAAAAAAABUAAAAAAAAAIJ6CIMG/6Un4MKNM8h+R9r8bQ6dNTk0WZxBMUQH1XFQBASCWUVucdQkje+4YbXVpvQZcg74nndL1NK7ccj1dDR04agAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACwAAAAAAAAAAAAAKAAAAAAAAAKOonlp6Vf8dJEjQYa/VyigZruaZwSwu3u/ZZVCsdrS1iaGPIAERZcNnfM75tOh10hI6MAAAAQAAAAAAAAAQAAAAAAA=",
                "AgAAAAAAAAAFAAAAAAAAAAYAAAAAAAAAIINaPEm+WRQV2vGcPR9fe6fYhxl48GpqB+DqDYQqRHkuASBe+6BDLHSRCMiWqBkvVMqWXPWUsZnpc2gbOVdre3vnowAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAwAAAAAAAAAAAQFgqGJldzxWMt2CZow1QiLmDf0RdLE6udu0bVdc1xaExX37NByF27rDH5C1DF+mkpLdA6YZnXMvuUw+zoWo71qe2DTdIDU4AcNaSUE3OoEHceuT+fBa6dMib3yDkkhmOZLyECcAAAAAAAAkAAAAAAAAAAAAAgAAAAAAAACvljn+1LWFSpu3PGx4BlIlVZq7blFK+fV7SOPEU0z9nz7lgkv8a12EA9R0tGm8hEYSOjAAAAEAAAAAAAAAEAAAAAAA",
            ];

            for fixture in FIXTURES {
                let bcs = Base64::decode_vec(fixture).unwrap();

                let checkpoint: SignedCheckpointSummary = bcs::from_bytes(&bcs).unwrap();
                let bytes = bcs::to_bytes(&checkpoint).unwrap();
                assert_eq!(bcs, bytes);
                let json = serde_json::to_string_pretty(&checkpoint).unwrap();
                println!("{json}");
            }
        }

        #[test]
        fn contents_fixture() {
            let fixture ="AAEgp6oAB8Qadn8+FqtdqeDIp8ViQNOZpMKs44MN0N5y7zIgqn5dKR1+8poL0pLNwRo/2knMnodwMTEDhqYL03kdewQBAWEAgpORkfH6ewjfFQYZJhmjkYq0/B3Set4mLJX/G0wUPb/V4H41gJipYu4I6ToyixnEuPQWxHKLckhNn+0UmI+pAJ9GegzEh0q2HWABmFMpFoPw0229dCfzWNOhHW5bes4H";

            let bcs = Base64::decode_vec(fixture).unwrap();

            let contents: CheckpointContents = bcs::from_bytes(&bcs).unwrap();
            let bytes = bcs::to_bytes(&contents).unwrap();
            assert_eq!(bcs, bytes);
            let json = serde_json::to_string_pretty(&contents).unwrap();
            println!("{json}");
        }
    }
}
