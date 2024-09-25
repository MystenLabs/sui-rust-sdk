use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde_with::DeserializeAs;
use serde_with::SerializeAs;

use crate::types::ObjectId;
use crate::types::ObjectReference;

use super::Argument;

mod transaction {
    use super::*;
    use crate::types::transaction::GasPayment;
    use crate::types::transaction::Transaction;
    use crate::types::transaction::TransactionExpiration;
    use crate::types::transaction::TransactionKind;
    use crate::types::Address;

    #[derive(serde_derive::Serialize)]
    #[serde(tag = "version")]
    #[serde(rename = "Transaction")]
    enum TransactionDataRef<'a> {
        #[serde(rename = "1")]
        V1(TransactionV1Ref<'a>),
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(tag = "version")]
    #[serde(rename = "Transaction")]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    enum TransactionData {
        #[serde(rename = "1")]
        V1(TransactionV1),
    }

    #[derive(serde_derive::Serialize)]
    #[serde(rename = "Transaction")]
    enum BinaryTransactionDataRef<'a> {
        #[serde(rename = "1")]
        V1(TransactionV1Ref<'a>),
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(rename = "Transaction")]
    enum BinaryTransactionData {
        #[serde(rename = "1")]
        V1(TransactionV1),
    }

    #[derive(serde_derive::Serialize)]
    #[serde(rename = "TransactionV1")]
    struct TransactionV1Ref<'a> {
        kind: &'a TransactionKind,
        sender: &'a Address,
        gas_payment: &'a GasPayment,
        expiration: &'a TransactionExpiration,
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(rename = "TransactionV1")]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    struct TransactionV1 {
        kind: TransactionKind,
        sender: Address,
        gas_payment: GasPayment,
        expiration: TransactionExpiration,
    }

    impl Serialize for Transaction {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let transaction = TransactionV1Ref {
                kind: &self.kind,
                sender: &self.sender,
                gas_payment: &self.gas_payment,
                expiration: &self.expiration,
            };

            if serializer.is_human_readable() {
                TransactionDataRef::V1(transaction).serialize(serializer)
            } else {
                BinaryTransactionDataRef::V1(transaction).serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for Transaction {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let TransactionV1 {
                kind,
                sender,
                gas_payment,
                expiration,
            } = if deserializer.is_human_readable() {
                let TransactionData::V1(transaction) = Deserialize::deserialize(deserializer)?;
                transaction
            } else {
                let BinaryTransactionData::V1(transaction) =
                    Deserialize::deserialize(deserializer)?;
                transaction
            };

            Ok(Transaction {
                kind,
                sender,
                gas_payment,
                expiration,
            })
        }
    }

    #[cfg(feature = "schemars")]
    impl schemars::JsonSchema for Transaction {
        fn schema_name() -> String {
            TransactionData::schema_name()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            TransactionData::json_schema(gen)
        }
    }
}

mod transaction_kind {
    use super::*;
    use crate::types::transaction::AuthenticatorStateUpdate;
    use crate::types::transaction::ChangeEpoch;
    use crate::types::transaction::ConsensusCommitPrologue;
    use crate::types::transaction::ConsensusCommitPrologueV2;
    use crate::types::transaction::ConsensusCommitPrologueV3;
    use crate::types::transaction::EndOfEpochTransactionKind;
    use crate::types::transaction::GenesisTransaction;
    use crate::types::transaction::ProgrammableTransaction;
    use crate::types::transaction::RandomnessStateUpdate;
    use crate::types::transaction::TransactionKind;

    #[derive(serde_derive::Serialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum ReadableTransactionKindRef<'a> {
        ProgrammableTransaction(&'a ProgrammableTransaction),
        ChangeEpoch(&'a ChangeEpoch),
        Genesis(&'a GenesisTransaction),
        ConsensusCommitPrologue(&'a ConsensusCommitPrologue),
        AuthenticatorStateUpdate(&'a AuthenticatorStateUpdate),
        EndOfEpoch {
            commands: &'a Vec<EndOfEpochTransactionKind>,
        },
        RandomnessStateUpdate(&'a RandomnessStateUpdate),
        ConsensusCommitPrologueV2(&'a ConsensusCommitPrologueV2),
        ConsensusCommitPrologueV3(&'a ConsensusCommitPrologueV3),
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    #[serde(rename = "TransactionKind")]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    enum ReadableTransactionKind {
        ProgrammableTransaction(ProgrammableTransaction),
        ChangeEpoch(ChangeEpoch),
        Genesis(GenesisTransaction),
        ConsensusCommitPrologue(ConsensusCommitPrologue),
        AuthenticatorStateUpdate(AuthenticatorStateUpdate),
        EndOfEpoch {
            commands: Vec<EndOfEpochTransactionKind>,
        },
        RandomnessStateUpdate(RandomnessStateUpdate),
        ConsensusCommitPrologueV2(ConsensusCommitPrologueV2),
        ConsensusCommitPrologueV3(ConsensusCommitPrologueV3),
    }

    #[cfg(feature = "schemars")]
    impl schemars::JsonSchema for TransactionKind {
        fn schema_name() -> String {
            ReadableTransactionKind::schema_name()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            ReadableTransactionKind::json_schema(gen)
        }
    }

    #[derive(serde_derive::Serialize)]
    enum BinaryTransactionKindRef<'a> {
        ProgrammableTransaction(&'a ProgrammableTransaction),
        ChangeEpoch(&'a ChangeEpoch),
        Genesis(&'a GenesisTransaction),
        ConsensusCommitPrologue(&'a ConsensusCommitPrologue),
        AuthenticatorStateUpdate(&'a AuthenticatorStateUpdate),
        EndOfEpoch(&'a Vec<EndOfEpochTransactionKind>),
        RandomnessStateUpdate(&'a RandomnessStateUpdate),
        ConsensusCommitPrologueV2(&'a ConsensusCommitPrologueV2),
        ConsensusCommitPrologueV3(&'a ConsensusCommitPrologueV3),
    }
    #[derive(serde_derive::Deserialize)]
    enum BinaryTransactionKind {
        ProgrammableTransaction(ProgrammableTransaction),
        ChangeEpoch(ChangeEpoch),
        Genesis(GenesisTransaction),
        ConsensusCommitPrologue(ConsensusCommitPrologue),
        AuthenticatorStateUpdate(AuthenticatorStateUpdate),
        EndOfEpoch(Vec<EndOfEpochTransactionKind>),
        RandomnessStateUpdate(RandomnessStateUpdate),
        ConsensusCommitPrologueV2(ConsensusCommitPrologueV2),
        ConsensusCommitPrologueV3(ConsensusCommitPrologueV3),
    }

    impl Serialize for TransactionKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self {
                    Self::ProgrammableTransaction(k) => {
                        ReadableTransactionKindRef::ProgrammableTransaction(k)
                    }
                    Self::ChangeEpoch(k) => ReadableTransactionKindRef::ChangeEpoch(k),
                    Self::Genesis(k) => ReadableTransactionKindRef::Genesis(k),
                    Self::ConsensusCommitPrologue(k) => {
                        ReadableTransactionKindRef::ConsensusCommitPrologue(k)
                    }
                    Self::AuthenticatorStateUpdate(k) => {
                        ReadableTransactionKindRef::AuthenticatorStateUpdate(k)
                    }
                    Self::EndOfEpoch(commands) => {
                        ReadableTransactionKindRef::EndOfEpoch { commands }
                    }
                    Self::RandomnessStateUpdate(k) => {
                        ReadableTransactionKindRef::RandomnessStateUpdate(k)
                    }
                    Self::ConsensusCommitPrologueV2(k) => {
                        ReadableTransactionKindRef::ConsensusCommitPrologueV2(k)
                    }
                    Self::ConsensusCommitPrologueV3(k) => {
                        ReadableTransactionKindRef::ConsensusCommitPrologueV3(k)
                    }
                };
                readable.serialize(serializer)
            } else {
                let binary = match self {
                    Self::ProgrammableTransaction(k) => {
                        BinaryTransactionKindRef::ProgrammableTransaction(k)
                    }
                    Self::ChangeEpoch(k) => BinaryTransactionKindRef::ChangeEpoch(k),
                    Self::Genesis(k) => BinaryTransactionKindRef::Genesis(k),
                    Self::ConsensusCommitPrologue(k) => {
                        BinaryTransactionKindRef::ConsensusCommitPrologue(k)
                    }
                    Self::AuthenticatorStateUpdate(k) => {
                        BinaryTransactionKindRef::AuthenticatorStateUpdate(k)
                    }
                    Self::EndOfEpoch(k) => BinaryTransactionKindRef::EndOfEpoch(k),
                    Self::RandomnessStateUpdate(k) => {
                        BinaryTransactionKindRef::RandomnessStateUpdate(k)
                    }
                    Self::ConsensusCommitPrologueV2(k) => {
                        BinaryTransactionKindRef::ConsensusCommitPrologueV2(k)
                    }
                    Self::ConsensusCommitPrologueV3(k) => {
                        BinaryTransactionKindRef::ConsensusCommitPrologueV3(k)
                    }
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for TransactionKind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableTransactionKind::deserialize(deserializer).map(|readable| match readable {
                    ReadableTransactionKind::ProgrammableTransaction(k) => {
                        Self::ProgrammableTransaction(k)
                    }
                    ReadableTransactionKind::ChangeEpoch(k) => Self::ChangeEpoch(k),
                    ReadableTransactionKind::Genesis(k) => Self::Genesis(k),
                    ReadableTransactionKind::ConsensusCommitPrologue(k) => {
                        Self::ConsensusCommitPrologue(k)
                    }
                    ReadableTransactionKind::AuthenticatorStateUpdate(k) => {
                        Self::AuthenticatorStateUpdate(k)
                    }
                    ReadableTransactionKind::EndOfEpoch { commands } => Self::EndOfEpoch(commands),
                    ReadableTransactionKind::RandomnessStateUpdate(k) => {
                        Self::RandomnessStateUpdate(k)
                    }
                    ReadableTransactionKind::ConsensusCommitPrologueV2(k) => {
                        Self::ConsensusCommitPrologueV2(k)
                    }
                    ReadableTransactionKind::ConsensusCommitPrologueV3(k) => {
                        Self::ConsensusCommitPrologueV3(k)
                    }
                })
            } else {
                BinaryTransactionKind::deserialize(deserializer).map(|binary| match binary {
                    BinaryTransactionKind::ProgrammableTransaction(k) => {
                        Self::ProgrammableTransaction(k)
                    }
                    BinaryTransactionKind::ChangeEpoch(k) => Self::ChangeEpoch(k),
                    BinaryTransactionKind::Genesis(k) => Self::Genesis(k),
                    BinaryTransactionKind::ConsensusCommitPrologue(k) => {
                        Self::ConsensusCommitPrologue(k)
                    }
                    BinaryTransactionKind::AuthenticatorStateUpdate(k) => {
                        Self::AuthenticatorStateUpdate(k)
                    }
                    BinaryTransactionKind::EndOfEpoch(k) => Self::EndOfEpoch(k),
                    BinaryTransactionKind::RandomnessStateUpdate(k) => {
                        Self::RandomnessStateUpdate(k)
                    }
                    BinaryTransactionKind::ConsensusCommitPrologueV2(k) => {
                        Self::ConsensusCommitPrologueV2(k)
                    }
                    BinaryTransactionKind::ConsensusCommitPrologueV3(k) => {
                        Self::ConsensusCommitPrologueV3(k)
                    }
                })
            }
        }
    }
}

mod end_of_epoch {
    use super::*;
    use crate::types::transaction::AuthenticatorStateExpire;
    use crate::types::transaction::ChangeEpoch;
    use crate::types::transaction::EndOfEpochTransactionKind;
    use crate::types::CheckpointDigest;

    #[derive(serde_derive::Serialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum ReadableEndOfEpochTransactionKindRef<'a> {
        ChangeEpoch(&'a ChangeEpoch),
        AuthenticatorStateCreate,
        AuthenticatorStateExpire(&'a AuthenticatorStateExpire),
        RandomnessStateCreate,
        DenyListStateCreate,
        BridgeStateCreate {
            chain_id: &'a CheckpointDigest,
        },
        BridgeCommitteeInit {
            #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
            bridge_object_version: u64,
        },
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum ReadableEndOfEpochTransactionKind {
        ChangeEpoch(ChangeEpoch),
        AuthenticatorStateCreate,
        AuthenticatorStateExpire(AuthenticatorStateExpire),
        RandomnessStateCreate,
        DenyListStateCreate,
        BridgeStateCreate {
            chain_id: CheckpointDigest,
        },
        BridgeCommitteeInit {
            #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
            bridge_object_version: u64,
        },
    }

    #[derive(serde_derive::Serialize)]
    enum BinaryEndOfEpochTransactionKindRef<'a> {
        ChangeEpoch(&'a ChangeEpoch),
        AuthenticatorStateCreate,
        AuthenticatorStateExpire(&'a AuthenticatorStateExpire),
        RandomnessStateCreate,
        DenyListStateCreate,
        BridgeStateCreate { chain_id: &'a CheckpointDigest },
        BridgeCommitteeInit { bridge_object_version: u64 },
    }

    #[derive(serde_derive::Deserialize)]
    enum BinaryEndOfEpochTransactionKind {
        ChangeEpoch(ChangeEpoch),
        AuthenticatorStateCreate,
        AuthenticatorStateExpire(AuthenticatorStateExpire),
        RandomnessStateCreate,
        DenyListStateCreate,
        BridgeStateCreate { chain_id: CheckpointDigest },
        BridgeCommitteeInit { bridge_object_version: u64 },
    }

    impl Serialize for EndOfEpochTransactionKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self {
                    Self::ChangeEpoch(k) => ReadableEndOfEpochTransactionKindRef::ChangeEpoch(k),
                    Self::AuthenticatorStateCreate => {
                        ReadableEndOfEpochTransactionKindRef::AuthenticatorStateCreate
                    }
                    Self::AuthenticatorStateExpire(k) => {
                        ReadableEndOfEpochTransactionKindRef::AuthenticatorStateExpire(k)
                    }
                    Self::RandomnessStateCreate => {
                        ReadableEndOfEpochTransactionKindRef::RandomnessStateCreate
                    }
                    Self::DenyListStateCreate => {
                        ReadableEndOfEpochTransactionKindRef::DenyListStateCreate
                    }
                    Self::BridgeStateCreate { chain_id } => {
                        ReadableEndOfEpochTransactionKindRef::BridgeStateCreate { chain_id }
                    }
                    Self::BridgeCommitteeInit {
                        bridge_object_version,
                    } => ReadableEndOfEpochTransactionKindRef::BridgeCommitteeInit {
                        bridge_object_version: *bridge_object_version,
                    },
                };
                readable.serialize(serializer)
            } else {
                let binary = match self {
                    Self::ChangeEpoch(k) => BinaryEndOfEpochTransactionKindRef::ChangeEpoch(k),
                    Self::AuthenticatorStateCreate => {
                        BinaryEndOfEpochTransactionKindRef::AuthenticatorStateCreate
                    }
                    Self::AuthenticatorStateExpire(k) => {
                        BinaryEndOfEpochTransactionKindRef::AuthenticatorStateExpire(k)
                    }
                    Self::RandomnessStateCreate => {
                        BinaryEndOfEpochTransactionKindRef::RandomnessStateCreate
                    }
                    Self::DenyListStateCreate => {
                        BinaryEndOfEpochTransactionKindRef::DenyListStateCreate
                    }
                    Self::BridgeStateCreate { chain_id } => {
                        BinaryEndOfEpochTransactionKindRef::BridgeStateCreate { chain_id }
                    }
                    Self::BridgeCommitteeInit {
                        bridge_object_version,
                    } => BinaryEndOfEpochTransactionKindRef::BridgeCommitteeInit {
                        bridge_object_version: *bridge_object_version,
                    },
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for EndOfEpochTransactionKind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableEndOfEpochTransactionKind::deserialize(deserializer).map(|readable| {
                    match readable {
                        ReadableEndOfEpochTransactionKind::ChangeEpoch(k) => Self::ChangeEpoch(k),
                        ReadableEndOfEpochTransactionKind::AuthenticatorStateCreate => {
                            Self::AuthenticatorStateCreate
                        }
                        ReadableEndOfEpochTransactionKind::AuthenticatorStateExpire(k) => {
                            Self::AuthenticatorStateExpire(k)
                        }
                        ReadableEndOfEpochTransactionKind::RandomnessStateCreate => {
                            Self::RandomnessStateCreate
                        }
                        ReadableEndOfEpochTransactionKind::DenyListStateCreate => {
                            Self::DenyListStateCreate
                        }
                        ReadableEndOfEpochTransactionKind::BridgeStateCreate { chain_id } => {
                            Self::BridgeStateCreate { chain_id }
                        }
                        ReadableEndOfEpochTransactionKind::BridgeCommitteeInit {
                            bridge_object_version,
                        } => Self::BridgeCommitteeInit {
                            bridge_object_version,
                        },
                    }
                })
            } else {
                BinaryEndOfEpochTransactionKind::deserialize(deserializer).map(
                    |binary| match binary {
                        BinaryEndOfEpochTransactionKind::ChangeEpoch(k) => Self::ChangeEpoch(k),
                        BinaryEndOfEpochTransactionKind::AuthenticatorStateCreate => {
                            Self::AuthenticatorStateCreate
                        }
                        BinaryEndOfEpochTransactionKind::AuthenticatorStateExpire(k) => {
                            Self::AuthenticatorStateExpire(k)
                        }
                        BinaryEndOfEpochTransactionKind::RandomnessStateCreate => {
                            Self::RandomnessStateCreate
                        }
                        BinaryEndOfEpochTransactionKind::DenyListStateCreate => {
                            Self::DenyListStateCreate
                        }
                        BinaryEndOfEpochTransactionKind::BridgeStateCreate { chain_id } => {
                            Self::BridgeStateCreate { chain_id }
                        }
                        BinaryEndOfEpochTransactionKind::BridgeCommitteeInit {
                            bridge_object_version,
                        } => Self::BridgeCommitteeInit {
                            bridge_object_version,
                        },
                    },
                )
            }
        }
    }
}

mod version_assignments {
    use super::*;
    use crate::types::transaction::CancelledTransaction;
    use crate::types::transaction::ConsensusDeterminedVersionAssignments;

    #[derive(serde_derive::Serialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum ReadableConsensusDeterminedVersionAssignmentsRef<'a> {
        CancelledTransactions {
            cancelled_transactions: &'a Vec<CancelledTransaction>,
        },
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum ReadableConsensusDeterminedVersionAssignments {
        CancelledTransactions {
            cancelled_transactions: Vec<CancelledTransaction>,
        },
    }

    #[derive(serde_derive::Serialize)]
    enum BinaryConsensusDeterminedVersionAssignmentsRef<'a> {
        CancelledTransactions {
            cancelled_transactions: &'a Vec<CancelledTransaction>,
        },
    }

    #[derive(serde_derive::Deserialize)]
    enum BinaryConsensusDeterminedVersionAssignments {
        CancelledTransactions {
            cancelled_transactions: Vec<CancelledTransaction>,
        },
    }

    impl Serialize for ConsensusDeterminedVersionAssignments {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self {
                    Self::CancelledTransactions {
                        cancelled_transactions,
                    } => ReadableConsensusDeterminedVersionAssignmentsRef::CancelledTransactions {
                        cancelled_transactions,
                    },
                };
                readable.serialize(serializer)
            } else {
                let binary = match self {
                    Self::CancelledTransactions {
                        cancelled_transactions,
                    } => BinaryConsensusDeterminedVersionAssignmentsRef::CancelledTransactions {
                        cancelled_transactions,
                    },
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for ConsensusDeterminedVersionAssignments {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableConsensusDeterminedVersionAssignments::deserialize(deserializer).map(
                    |readable| match readable {
                        ReadableConsensusDeterminedVersionAssignments::CancelledTransactions {
                            cancelled_transactions,
                        } => Self::CancelledTransactions {
                            cancelled_transactions,
                        },
                    },
                )
            } else {
                BinaryConsensusDeterminedVersionAssignments::deserialize(deserializer).map(
                    |binary| match binary {
                        BinaryConsensusDeterminedVersionAssignments::CancelledTransactions {
                            cancelled_transactions,
                        } => Self::CancelledTransactions {
                            cancelled_transactions,
                        },
                    },
                )
            }
        }
    }
}

mod input_argument {
    use crate::types::transaction::InputArgument;

    use super::*;

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    enum ReadableInputArgument {
        Pure {
            #[serde(with = "::serde_with::As::<crate::_serde::Base64Encoded>")]
            value: Vec<u8>,
        },
        ImmutableOrOwned(ObjectReference),
        Shared {
            object_id: ObjectId,
            #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
            initial_shared_version: u64,
            mutable: bool,
        },
        Receiving(ObjectReference),
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum CallArg {
        Pure(#[serde(with = "::serde_with::As::<::serde_with::Bytes>")] Vec<u8>),
        Object(ObjectArg),
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum ObjectArg {
        ImmutableOrOwned(ObjectReference),
        Shared {
            object_id: ObjectId,
            initial_shared_version: u64,
            mutable: bool,
        },
        Receiving(ObjectReference),
    }

    impl Serialize for InputArgument {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self.clone() {
                    InputArgument::Pure { value } => ReadableInputArgument::Pure { value },
                    InputArgument::ImmutableOrOwned(object_ref) => {
                        ReadableInputArgument::ImmutableOrOwned(object_ref)
                    }
                    InputArgument::Shared {
                        object_id,
                        initial_shared_version,
                        mutable,
                    } => ReadableInputArgument::Shared {
                        object_id,
                        initial_shared_version,
                        mutable,
                    },
                    InputArgument::Receiving(object_ref) => {
                        ReadableInputArgument::Receiving(object_ref)
                    }
                };
                readable.serialize(serializer)
            } else {
                let binary = match self.clone() {
                    InputArgument::Pure { value } => CallArg::Pure(value),
                    InputArgument::ImmutableOrOwned(object_ref) => {
                        CallArg::Object(ObjectArg::ImmutableOrOwned(object_ref))
                    }
                    InputArgument::Shared {
                        object_id,
                        initial_shared_version,
                        mutable,
                    } => CallArg::Object(ObjectArg::Shared {
                        object_id,
                        initial_shared_version,
                        mutable,
                    }),
                    InputArgument::Receiving(object_ref) => {
                        CallArg::Object(ObjectArg::Receiving(object_ref))
                    }
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for InputArgument {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableInputArgument::deserialize(deserializer).map(|readable| match readable {
                    ReadableInputArgument::Pure { value } => InputArgument::Pure { value },
                    ReadableInputArgument::ImmutableOrOwned(object_ref) => {
                        InputArgument::ImmutableOrOwned(object_ref)
                    }
                    ReadableInputArgument::Shared {
                        object_id,
                        initial_shared_version,
                        mutable,
                    } => InputArgument::Shared {
                        object_id,
                        initial_shared_version,
                        mutable,
                    },
                    ReadableInputArgument::Receiving(object_ref) => {
                        InputArgument::Receiving(object_ref)
                    }
                })
            } else {
                CallArg::deserialize(deserializer).map(|binary| match binary {
                    CallArg::Pure(value) => InputArgument::Pure { value },
                    CallArg::Object(ObjectArg::ImmutableOrOwned(object_ref)) => {
                        InputArgument::ImmutableOrOwned(object_ref)
                    }
                    CallArg::Object(ObjectArg::Shared {
                        object_id,
                        initial_shared_version,
                        mutable,
                    }) => InputArgument::Shared {
                        object_id,
                        initial_shared_version,
                        mutable,
                    },
                    CallArg::Object(ObjectArg::Receiving(object_ref)) => {
                        InputArgument::Receiving(object_ref)
                    }
                })
            }
        }
    }
}

mod argument {
    use super::*;

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(rename = "Argument", untagged, rename_all = "lowercase")]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    enum ReadableArgument {
        /// # Gas
        Gas(Gas),
        /// # Input
        Input { input: u16 },
        /// # Result
        Result { result: u16 },
        /// # NestedResult
        NestedResult { result: (u16, u16) },
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum Gas {
        Gas,
    }

    #[cfg(feature = "schemars")]
    impl schemars::JsonSchema for Gas {
        fn schema_name() -> std::string::String {
            "GasArgument".to_owned()
        }

        fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                instance_type: Some(schemars::schema::InstanceType::String.into()),
                enum_values: Some(vec!["gas".into()]),
                ..Default::default()
            })
        }

        fn is_referenceable() -> bool {
            false
        }
    }

    #[cfg(feature = "schemars")]
    impl schemars::JsonSchema for Argument {
        fn schema_name() -> String {
            ReadableArgument::schema_name()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            ReadableArgument::json_schema(gen)
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    enum BinaryArgument {
        Gas,
        Input(u16),
        Result(u16),
        NestedResult(u16, u16),
    }

    impl Serialize for Argument {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match *self {
                    Argument::Gas => ReadableArgument::Gas(Gas::Gas),
                    Argument::Input(input) => ReadableArgument::Input { input },
                    Argument::Result(result) => ReadableArgument::Result { result },
                    Argument::NestedResult(result, subresult) => ReadableArgument::NestedResult {
                        result: (result, subresult),
                    },
                };
                readable.serialize(serializer)
            } else {
                let binary = match *self {
                    Argument::Gas => BinaryArgument::Gas,
                    Argument::Input(input) => BinaryArgument::Input(input),
                    Argument::Result(result) => BinaryArgument::Result(result),
                    Argument::NestedResult(result, subresult) => {
                        BinaryArgument::NestedResult(result, subresult)
                    }
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for Argument {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableArgument::deserialize(deserializer).map(|readable| match readable {
                    ReadableArgument::Gas(_) => Argument::Gas,
                    ReadableArgument::Input { input } => Argument::Input(input),
                    ReadableArgument::Result { result } => Argument::Result(result),
                    ReadableArgument::NestedResult {
                        result: (result, subresult),
                    } => Argument::NestedResult(result, subresult),
                })
            } else {
                BinaryArgument::deserialize(deserializer).map(|binary| match binary {
                    BinaryArgument::Gas => Argument::Gas,
                    BinaryArgument::Input(input) => Argument::Input(input),
                    BinaryArgument::Result(result) => Argument::Result(result),
                    BinaryArgument::NestedResult(result, subresult) => {
                        Argument::NestedResult(result, subresult)
                    }
                })
            }
        }
    }
}

mod command {
    use super::*;

    use crate::types::transaction::Command;
    use crate::types::transaction::MakeMoveVector;
    use crate::types::transaction::MergeCoins;
    use crate::types::transaction::MoveCall;
    use crate::types::transaction::Publish;
    use crate::types::transaction::SplitCoins;
    use crate::types::transaction::TransferObjects;
    use crate::types::transaction::Upgrade;

    #[derive(serde_derive::Serialize)]
    #[serde(tag = "command", rename_all = "snake_case")]
    enum ReadableCommandRef<'a> {
        MoveCall(&'a MoveCall),
        TransferObjects(&'a TransferObjects),
        SplitCoins(&'a SplitCoins),
        MergeCoins(&'a MergeCoins),
        Publish(&'a Publish),
        MakeMoveVector(&'a MakeMoveVector),
        Upgrade(&'a Upgrade),
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(tag = "command", rename_all = "snake_case")]
    enum ReadableCommand {
        MoveCall(MoveCall),
        TransferObjects(TransferObjects),
        SplitCoins(SplitCoins),
        MergeCoins(MergeCoins),
        Publish(Publish),
        MakeMoveVector(MakeMoveVector),
        Upgrade(Upgrade),
    }

    #[derive(serde_derive::Serialize)]
    enum BinaryCommandRef<'a> {
        MoveCall(&'a MoveCall),
        TransferObjects(&'a TransferObjects),
        SplitCoins(&'a SplitCoins),
        MergeCoins(&'a MergeCoins),
        Publish(&'a Publish),
        MakeMoveVector(&'a MakeMoveVector),
        Upgrade(&'a Upgrade),
    }

    #[derive(serde_derive::Deserialize)]
    enum BinaryCommand {
        MoveCall(MoveCall),
        TransferObjects(TransferObjects),
        SplitCoins(SplitCoins),
        MergeCoins(MergeCoins),
        Publish(Publish),
        MakeMoveVector(MakeMoveVector),
        Upgrade(Upgrade),
    }

    impl Serialize for Command {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self {
                    Command::MoveCall(c) => ReadableCommandRef::MoveCall(c),
                    Command::TransferObjects(c) => ReadableCommandRef::TransferObjects(c),
                    Command::SplitCoins(c) => ReadableCommandRef::SplitCoins(c),
                    Command::MergeCoins(c) => ReadableCommandRef::MergeCoins(c),
                    Command::Publish(c) => ReadableCommandRef::Publish(c),
                    Command::MakeMoveVector(c) => ReadableCommandRef::MakeMoveVector(c),
                    Command::Upgrade(c) => ReadableCommandRef::Upgrade(c),
                };
                readable.serialize(serializer)
            } else {
                let binary = match self {
                    Command::MoveCall(c) => BinaryCommandRef::MoveCall(c),
                    Command::TransferObjects(c) => BinaryCommandRef::TransferObjects(c),
                    Command::SplitCoins(c) => BinaryCommandRef::SplitCoins(c),
                    Command::MergeCoins(c) => BinaryCommandRef::MergeCoins(c),
                    Command::Publish(c) => BinaryCommandRef::Publish(c),
                    Command::MakeMoveVector(c) => BinaryCommandRef::MakeMoveVector(c),
                    Command::Upgrade(c) => BinaryCommandRef::Upgrade(c),
                };
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for Command {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableCommand::deserialize(deserializer).map(|readable| match readable {
                    ReadableCommand::MoveCall(c) => Command::MoveCall(c),
                    ReadableCommand::TransferObjects(c) => Command::TransferObjects(c),
                    ReadableCommand::SplitCoins(c) => Command::SplitCoins(c),
                    ReadableCommand::MergeCoins(c) => Command::MergeCoins(c),
                    ReadableCommand::Publish(c) => Command::Publish(c),
                    ReadableCommand::MakeMoveVector(c) => Command::MakeMoveVector(c),
                    ReadableCommand::Upgrade(c) => Command::Upgrade(c),
                })
            } else {
                BinaryCommand::deserialize(deserializer).map(|binary| match binary {
                    BinaryCommand::MoveCall(c) => Command::MoveCall(c),
                    BinaryCommand::TransferObjects(c) => Command::TransferObjects(c),
                    BinaryCommand::SplitCoins(c) => Command::SplitCoins(c),
                    BinaryCommand::MergeCoins(c) => Command::MergeCoins(c),
                    BinaryCommand::Publish(c) => Command::Publish(c),
                    BinaryCommand::MakeMoveVector(c) => Command::MakeMoveVector(c),
                    BinaryCommand::Upgrade(c) => Command::Upgrade(c),
                })
            }
        }
    }
}

pub(crate) use signed_transaction::SignedTransactionWithIntentMessage;

mod signed_transaction {
    use serde::ser::SerializeSeq;

    use super::*;
    use crate::types::transaction::SignedTransaction;
    use crate::types::transaction::Transaction;
    use crate::types::UserSignature;

    /// serde implementation that serializes a transaction prefixed with the signing intent. See
    /// [struct Intent] for more info.
    ///
    /// So we need to serialize Transaction as (0, 0, 0, Transaction)
    struct IntentMessageWrappedTransaction;

    impl SerializeAs<Transaction> for IntentMessageWrappedTransaction {
        fn serialize_as<S>(transaction: &Transaction, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            use serde::ser::SerializeTuple;

            let mut s = serializer.serialize_tuple(4)?;
            s.serialize_element(&0u8)?;
            s.serialize_element(&0u8)?;
            s.serialize_element(&0u8)?;
            s.serialize_element(transaction)?;
            s.end()
        }
    }

    impl<'de> DeserializeAs<'de, Transaction> for IntentMessageWrappedTransaction {
        fn deserialize_as<D>(deserializer: D) -> Result<Transaction, D::Error>
        where
            D: Deserializer<'de>,
        {
            let (scope, version, app, transaction): (u8, u8, u8, Transaction) =
                Deserialize::deserialize(deserializer)?;
            match (scope, version, app) {
                (0, 0, 0) => {}
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "invalid intent message ({scope}, {version}, {app})"
                    )))
                }
            }

            Ok(transaction)
        }
    }

    pub(crate) struct SignedTransactionWithIntentMessage;

    #[derive(serde_derive::Serialize)]
    struct BinarySignedTransactionWithIntentMessageRef<'a> {
        #[serde(with = "::serde_with::As::<IntentMessageWrappedTransaction>")]
        transaction: &'a Transaction,
        signatures: &'a Vec<UserSignature>,
    }

    #[derive(serde_derive::Deserialize)]
    struct BinarySignedTransactionWithIntentMessage {
        #[serde(with = "::serde_with::As::<IntentMessageWrappedTransaction>")]
        transaction: Transaction,
        signatures: Vec<UserSignature>,
    }

    impl SerializeAs<SignedTransaction> for SignedTransactionWithIntentMessage {
        fn serialize_as<S>(
            transaction: &SignedTransaction,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                transaction.serialize(serializer)
            } else {
                let SignedTransaction {
                    transaction,
                    signatures,
                } = transaction;
                let binary = BinarySignedTransactionWithIntentMessageRef {
                    transaction,
                    signatures,
                };

                let mut s = serializer.serialize_seq(Some(1))?;
                s.serialize_element(&binary)?;
                s.end()
            }
        }
    }

    impl<'de> DeserializeAs<'de, SignedTransaction> for SignedTransactionWithIntentMessage {
        fn deserialize_as<D>(deserializer: D) -> Result<SignedTransaction, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                SignedTransaction::deserialize(deserializer)
            } else {
                struct V;
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = SignedTransaction;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("expected a sequence with length 1")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        if seq.size_hint().is_some_and(|size| size != 1) {
                            return Err(serde::de::Error::custom(
                                "expected a sequence with length 1",
                            ));
                        }

                        let BinarySignedTransactionWithIntentMessage {
                            transaction,
                            signatures,
                        } = seq.next_element()?.ok_or_else(|| {
                            serde::de::Error::custom("expected a sequence with length 1")
                        })?;
                        Ok(SignedTransaction {
                            transaction,
                            signatures,
                        })
                    }
                }

                deserializer.deserialize_seq(V)
            }
        }
    }
}

mod transaction_expiration {
    use crate::types::{EpochId, TransactionExpiration};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(rename = "TransactionExpiration")]
    #[serde(rename_all = "lowercase")]
    enum ReadableTransactionExpiration {
        /// Validators wont sign a transaction unless the expiration Epoch
        /// is greater than or equal to the current epoch
        Epoch(
            #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))] EpochId,
        ),
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    pub enum BinaryTransactionExpiration {
        /// The transaction has no expiration
        None,
        /// Validators wont sign a transaction unless the expiration Epoch
        /// is greater than or equal to the current epoch
        Epoch(EpochId),
    }

    impl Serialize for TransactionExpiration {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                match *self {
                    Self::None => None,
                    Self::Epoch(epoch) => Some(ReadableTransactionExpiration::Epoch(epoch)),
                }
                .serialize(serializer)
            } else {
                match *self {
                    Self::None => BinaryTransactionExpiration::None,
                    Self::Epoch(epoch) => BinaryTransactionExpiration::Epoch(epoch),
                }
                .serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for TransactionExpiration {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                Option::<ReadableTransactionExpiration>::deserialize(deserializer).map(|readable| {
                    match readable {
                        None => Self::None,
                        Some(ReadableTransactionExpiration::Epoch(epoch)) => Self::Epoch(epoch),
                    }
                })
            } else {
                BinaryTransactionExpiration::deserialize(deserializer).map(|binary| match binary {
                    BinaryTransactionExpiration::None => Self::None,
                    BinaryTransactionExpiration::Epoch(epoch) => Self::Epoch(epoch),
                })
            }
        }
    }

    #[cfg(feature = "schemars")]
    impl schemars::JsonSchema for TransactionExpiration {
        fn schema_name() -> String {
            "TransactionExpiration".into()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            use schemars::schema::{Schema, SchemaObject};
            schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                    one_of: Some(vec![
                        schemars::_private::metadata::add_description(
                            schemars::_private::new_externally_tagged_enum(
                                "epoch",
                                gen.subschema_for::<crate::_schemars::U64>(),
                            ),
                            "Validators wont sign a transaction unless the expiration Epoch is greater than or equal to the current epoch",
                        ),
                        Schema::Object(SchemaObject {
                            instance_type: Some(schemars::schema::InstanceType::Null.into()),
                            ..SchemaObject::default()
                        }),
                    ]),
                    ..Default::default()
                })),
                ..Default::default()
            })
        }
    }
}

#[cfg(test)]
mod test {
    use base64ct::Base64;
    use base64ct::Encoding;

    use crate::types::transaction::Argument;
    use crate::types::transaction::InputArgument;
    use crate::types::transaction::Transaction;
    use crate::types::ObjectDigest;
    use crate::types::ObjectId;
    use crate::types::ObjectReference;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn argument() {
        let test_cases = [
            (Argument::Gas, serde_json::json!("gas")),
            (Argument::Input(1), serde_json::json!({"input": 1})),
            (Argument::Result(2), serde_json::json!({"result": 2})),
            (
                Argument::NestedResult(3, 4),
                serde_json::json!({"result": [3, 4]}),
            ),
        ];

        for (case, expected) in test_cases {
            let actual = serde_json::to_value(case).unwrap();
            assert_eq!(actual, expected);
            println!("{actual}");

            let deser = serde_json::from_value(expected).unwrap();
            assert_eq!(case, deser);
        }
    }

    #[test]
    fn input_argument() {
        let test_cases = [
            (
                InputArgument::Pure {
                    value: vec![1, 2, 3, 4],
                },
                serde_json::json!({
                  "type": "pure",
                  "value": "AQIDBA=="
                }),
            ),
            (
                InputArgument::ImmutableOrOwned(ObjectReference::new(
                    ObjectId::ZERO,
                    1,
                    ObjectDigest::ZERO,
                )),
                serde_json::json!({
                  "type": "immutable_or_owned",
                  "object_id": "0x0000000000000000000000000000000000000000000000000000000000000000",
                  "version": "1",
                  "digest": "11111111111111111111111111111111"
                }),
            ),
            (
                InputArgument::Shared {
                    object_id: ObjectId::ZERO,
                    initial_shared_version: 1,
                    mutable: true,
                },
                serde_json::json!({
                  "type": "shared",
                  "object_id": "0x0000000000000000000000000000000000000000000000000000000000000000",
                  "initial_shared_version": "1",
                  "mutable": true
                }),
            ),
            (
                InputArgument::Receiving(ObjectReference::new(
                    ObjectId::ZERO,
                    1,
                    ObjectDigest::ZERO,
                )),
                serde_json::json!({
                  "type": "receiving",
                  "object_id": "0x0000000000000000000000000000000000000000000000000000000000000000",
                  "version": "1",
                  "digest": "11111111111111111111111111111111"
                }),
            ),
        ];

        for (case, expected) in test_cases {
            let actual = serde_json::to_value(&case).unwrap();
            assert_eq!(actual, expected);
            println!("{actual}");

            let deser = serde_json::from_value(expected).unwrap();
            assert_eq!(case, deser);
        }
    }

    #[test]
    fn transaction_fixtures() {
        const GENESIS_TRANSACTION: &str = include_str!("fixtures/genesis-transaction");
        const CONSENSUS_PROLOGUE: &str = "AAMAAAAAAAAAAAIAAAAAAAAAtkjHeocBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAA==";
        const EPOCH_CHANGE: &str = "AAUCAmkBAAAAAAAAmSrgAQAAAAAAagEAAAAAAAApAAAAAAAAALAQCoNLLwAAnNn0sywGAABsVBEfSC0AAKQnlhd1AAAAzve+vo4BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAA=";
        const AUTHENTICATOR_STATE_UPDATE: &str =
            include_str!("fixtures/authenticator_state_update");
        const PTB: &str = "AAADAQFEBbUNeR/TNGdU6Bcaqra8LtJsLEbv3QM8FLMK5QesMyx96QEAAAAAAQAIVsakAAAAAAABALyyokbZ/8ynfWQer6UyP1DpeCnPU1NC7AyFNJSaTztnQF40BQAAAAAgffPXh5XuG6TWjHk6qC5w9k2a+41oTWfm0sC1FOYRqsEBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAN7pB2Nsb2JfdjIMY2FuY2VsX29yZGVyAgcAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgNzdWkDU1VJAAddSzAlBmRcN/8TO5jEtQpa4UhBZZc41tcz1Z0NIXqTvwRjb2luBENPSU4AAwEAAAEBAAECAPgh00g/x3Jeuvqlo9Ejc9SZAb384UhPIZ2qcGajDfd9ASXQjpFOD6mfycbzwD1wc+IOkCXQ8rHQo/Vi5SDOGMR/Jl40BQAAAAAgV7P1E0IMKon5uI82R/0arWLt+dc1ng/4VwKDqpTCxHT4IdNIP8dyXrr6paPRI3PUmQG9/OFITyGdqnBmow33fe4CAAAAAAAAAMqaOwAAAAAA";
        const WORMHOLE_PYTH_TRANSACTION: &str = include_str!("fixtures/wormhole-pyth-transaction");

        for fixture in [
            GENESIS_TRANSACTION,
            CONSENSUS_PROLOGUE,
            EPOCH_CHANGE,
            AUTHENTICATOR_STATE_UPDATE,
            PTB,
            WORMHOLE_PYTH_TRANSACTION,
        ] {
            let fixture = Base64::decode_vec(fixture.trim()).unwrap();
            let tx: Transaction = bcs::from_bytes(&fixture).unwrap();
            assert_eq!(bcs::to_bytes(&tx).unwrap(), fixture);

            let json = serde_json::to_string_pretty(&tx).unwrap();
            println!("{json}");
            assert_eq!(tx, serde_json::from_str(&json).unwrap());
        }
    }
}
