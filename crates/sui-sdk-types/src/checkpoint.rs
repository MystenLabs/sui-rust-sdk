use super::Digest;
use super::GasCostSummary;
use super::Object;
use super::SignedTransaction;
use super::TransactionEffects;
use super::TransactionEvents;
use super::UserSignature;
use super::ValidatorAggregatedSignature;
use super::ValidatorCommitteeMember;

pub type CheckpointSequenceNumber = u64;
pub type CheckpointTimestamp = u64;
pub type EpochId = u64;
pub type StakeUnit = u64;
pub type ProtocolVersion = u64;

/// A commitment made by a checkpoint.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// ; CheckpointCommitment is an enum and each variant is prefixed with its index
/// ecmh-live-object-set = %x00 digest
/// checkpoint-artifacts-digest = %x01 digest
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub enum CheckpointCommitment {
    /// An Elliptic Curve Multiset Hash attesting to the set of Objects that comprise the live
    /// state of the Sui blockchain.
    EcmhLiveObjectSet { digest: Digest },

    /// Digest of the checkpoint artifacts.
    CheckpointArtifactsDigest { digest: Digest },
}

/// Data, which when included in a [`CheckpointSummary`], signals the end of an `Epoch`.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// end-of-epoch-data = (vector validator-committee-member) ; next_epoch_committee
///                     u64                                 ; next_epoch_protocol_version
///                     (vector checkpoint-commitment)      ; epoch_commitments
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct EndOfEpochData {
    /// The set of Validators that will be in the ValidatorCommittee for the next epoch.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=1).lift()))]
    pub next_epoch_committee: Vec<ValidatorCommitteeMember>,

    /// The protocol version that is in effect during the next epoch.
    pub next_epoch_protocol_version: ProtocolVersion,

    /// Commitments to epoch specific state (e.g. live object set)
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=1).lift()))]
    pub epoch_commitments: Vec<CheckpointCommitment>,
}

/// A header for a Checkpoint on the Sui blockchain.
///
/// On the Sui network, checkpoints define the history of the blockchain. They are quite similar to
/// the concept of blocks used by other blockchains like Bitcoin or Ethereum. The Sui blockchain,
/// however, forms checkpoints after transaction execution has already happened to provide a
/// certified history of the chain, instead of being formed before execution.
///
/// Checkpoints commit to a variety of state including but not limited to:
/// - The hash of the previous checkpoint.
/// - The set of transaction digests, their corresponding effects digests, as well as the set of
///   user signatures which authorized its execution.
/// - The object's produced by a transaction.
/// - The set of live objects that make up the current state of the chain.
/// - On epoch transitions, the next validator committee.
///
/// `CheckpointSummary`s themselves don't directly include all of the above information but they
/// are the top-level type by which all the above are committed to transitively via cryptographic
/// hashes included in the summary. `CheckpointSummary`s are signed and certified by a quorum of
/// the validator committee in a given epoch in order to allow verification of the chain's state.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// checkpoint-summary = u64                            ; epoch
///                      u64                            ; sequence_number
///                      u64                            ; network_total_transactions
///                      digest                         ; content_digest
///                      (option digest)                ; previous_digest
///                      gas-cost-summary               ; epoch_rolling_gas_cost_summary
///                      u64                            ; timestamp_ms
///                      (vector checkpoint-commitment) ; checkpoint_commitments
///                      (option end-of-epoch-data)     ; end_of_epoch_data
///                      bytes                          ; version_specific_data
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct CheckpointSummary {
    /// Epoch that this checkpoint belongs to.
    pub epoch: EpochId,

    /// The height of this checkpoint.
    pub sequence_number: CheckpointSequenceNumber,

    /// Total number of transactions committed since genesis, including those in this
    /// checkpoint.
    pub network_total_transactions: u64,

    /// The hash of the [`CheckpointContents`] for this checkpoint.
    pub content_digest: Digest,

    /// The hash of the previous `CheckpointSummary`.
    ///
    /// This will be only be `None` for the first, or genesis checkpoint.
    pub previous_digest: Option<Digest>,

    /// The running total gas costs of all transactions included in the current epoch so far
    /// until this checkpoint.
    pub epoch_rolling_gas_cost_summary: GasCostSummary,

    /// Timestamp of the checkpoint - number of milliseconds from the Unix epoch
    /// Checkpoint timestamps are monotonic, but not strongly monotonic - subsequent
    /// checkpoints can have same timestamp if they originate from the same underlining consensus commit
    pub timestamp_ms: CheckpointTimestamp,

    /// Commitments to checkpoint-specific state.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=1).lift()))]
    pub checkpoint_commitments: Vec<CheckpointCommitment>,

    /// Extra data only present in the final checkpoint of an epoch.
    pub end_of_epoch_data: Option<EndOfEpochData>,

    /// `CheckpointSummary` is not an evolvable structure - it must be readable by any version of
    /// the code. Therefore, in order to allow extensions to be added to `CheckpointSummary`, we
    /// allow opaque data to be added to checkpoints which can be deserialized based on the current
    /// protocol version.
    pub version_specific_data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct SignedCheckpointSummary {
    pub checkpoint: CheckpointSummary,
    pub signature: ValidatorAggregatedSignature,
}

/// The committed to contents of a checkpoint.
///
/// `CheckpointContents` contains a list of digests of Transactions, their effects, and the user
/// signatures that authorized their execution included in a checkpoint.
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// checkpoint-contents = %x00 checkpoint-contents-v1 ; variant 0
///
/// checkpoint-contents-v1 = (vector (digest digest)) ; vector of transaction and effect digests
///                          (vector (vector bcs-user-signature)) ; set of user signatures for each
///                                                               ; transaction. MUST be the same
///                                                               ; length as the vector of digests
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct CheckpointContents(
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    Vec<CheckpointTransactionInfo>,
);

impl CheckpointContents {
    pub fn new(transactions: Vec<CheckpointTransactionInfo>) -> Self {
        Self(transactions)
    }

    pub fn transactions(&self) -> &[CheckpointTransactionInfo] {
        &self.0
    }

    pub fn into_v1(self) -> Vec<CheckpointTransactionInfo> {
        self.0
    }
}

/// Transaction information committed to in a checkpoint
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct CheckpointTransactionInfo {
    pub transaction: Digest,
    pub effects: Digest,
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=2).lift()))]
    pub signatures: Vec<UserSignature>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct CheckpointData {
    pub checkpoint_summary: SignedCheckpointSummary,
    pub checkpoint_contents: CheckpointContents,
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=1).lift()))]
    pub transactions: Vec<CheckpointTransaction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct CheckpointTransaction {
    /// The input Transaction
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<crate::_serde::SignedTransactionWithIntentMessage>")
    )]
    pub transaction: SignedTransaction,
    /// The effects produced by executing this transaction
    pub effects: TransactionEffects,
    /// The events, if any, emitted by this transaciton during execution
    pub events: Option<TransactionEvents>,
    /// The state of all inputs to this transaction as they were prior to execution.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=1).lift()))]
    pub input_objects: Vec<Object>,
    /// The state of all output objects created or mutated by this transaction.
    #[cfg_attr(feature = "proptest", any(proptest::collection::size_range(0..=1).lift()))]
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

    impl Serialize for CheckpointContents {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            use serde::ser::SerializeSeq;
            use serde::ser::SerializeTupleVariant;

            #[derive(serde_derive::Serialize)]
            struct Digests<'a> {
                transaction: &'a Digest,
                effects: &'a Digest,
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

    #[derive(serde_derive::Deserialize)]
    struct ExecutionDigests {
        transaction: Digest,
        effects: Digest,
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

    #[cfg(test)]
    mod test {
        use super::*;
        use base64ct::Base64;
        use base64ct::Encoding;

        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen_test::wasm_bindgen_test as test;

        // TODO: Include a checkpoint with the artifacts digest
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
