use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde_with::DeserializeAs;
use serde_with::SerializeAs;

use crate::types::{ObjectId, ObjectReference};

use super::Argument;

mod transaction {
    use super::*;
    use crate::types::{
        transaction::{GasPayment, Transaction, TransactionExpiration, TransactionKind},
        Address,
    };

    #[derive(serde_derive::Serialize)]
    struct ReadableTransactionRef<'a> {
        #[serde(flatten)]
        kind: &'a TransactionKind,
        sender: &'a Address,
        gas_payment: &'a GasPayment,
        expiration: &'a TransactionExpiration,
    }

    #[derive(serde_derive::Deserialize)]
    struct ReadableTransaction {
        #[serde(flatten)]
        kind: TransactionKind,
        sender: Address,
        gas_payment: GasPayment,
        expiration: TransactionExpiration,
    }

    #[derive(serde_derive::Serialize)]
    struct BinaryTransactionRef<'a> {
        kind: &'a TransactionKind,
        sender: &'a Address,
        gas_payment: &'a GasPayment,
        expiration: &'a TransactionExpiration,
    }

    #[derive(serde_derive::Deserialize)]
    struct BinaryTransaction {
        kind: TransactionKind,
        sender: Address,
        gas_payment: GasPayment,
        expiration: TransactionExpiration,
    }

    #[derive(serde_derive::Serialize)]
    enum BinaryTransactionDataRef<'a> {
        V1(BinaryTransactionRef<'a>),
    }

    #[derive(serde_derive::Deserialize)]
    enum BinaryTransactionData {
        V1(BinaryTransaction),
    }

    impl Serialize for Transaction {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = ReadableTransactionRef {
                    kind: &self.kind,
                    sender: &self.sender,
                    gas_payment: &self.gas_payment,
                    expiration: &self.expiration,
                };
                readable.serialize(serializer)
            } else {
                let binary = BinaryTransactionDataRef::V1(BinaryTransactionRef {
                    kind: &self.kind,
                    sender: &self.sender,
                    gas_payment: &self.gas_payment,
                    expiration: &self.expiration,
                });
                binary.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for Transaction {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let ReadableTransaction {
                    kind,
                    sender,
                    gas_payment,
                    expiration,
                } = Deserialize::deserialize(deserializer)?;

                Ok(Transaction {
                    kind,
                    sender,
                    gas_payment,
                    expiration,
                })
            } else {
                let BinaryTransactionData::V1(BinaryTransaction {
                    kind,
                    sender,
                    gas_payment,
                    expiration,
                }) = Deserialize::deserialize(deserializer)?;

                Ok(Transaction {
                    kind,
                    sender,
                    gas_payment,
                    expiration,
                })
            }
        }
    }
}

mod transaction_kind {
    use super::*;
    use crate::types::transaction::{
        AuthenticatorStateUpdate, ChangeEpoch, ConsensusCommitPrologue, ConsensusCommitPrologueV2,
        EndOfEpochTransactionKind, GenesisTransaction, ProgrammableTransaction,
        RandomnessStateUpdate, TransactionKind,
    };

    #[derive(serde_derive::Serialize)]
    #[serde(tag = "kind", rename_all = "kebab-case")]
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
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(tag = "kind", rename_all = "kebab-case")]
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
    }

    impl Serialize for TransactionKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self {
                    TransactionKind::ProgrammableTransaction(k) => {
                        ReadableTransactionKindRef::ProgrammableTransaction(k)
                    }
                    TransactionKind::ChangeEpoch(k) => ReadableTransactionKindRef::ChangeEpoch(k),
                    TransactionKind::Genesis(k) => ReadableTransactionKindRef::Genesis(k),
                    TransactionKind::ConsensusCommitPrologue(k) => {
                        ReadableTransactionKindRef::ConsensusCommitPrologue(k)
                    }
                    TransactionKind::AuthenticatorStateUpdate(k) => {
                        ReadableTransactionKindRef::AuthenticatorStateUpdate(k)
                    }
                    TransactionKind::EndOfEpoch(commands) => {
                        ReadableTransactionKindRef::EndOfEpoch { commands }
                    }
                    TransactionKind::RandomnessStateUpdate(k) => {
                        ReadableTransactionKindRef::RandomnessStateUpdate(k)
                    }
                    TransactionKind::ConsensusCommitPrologueV2(k) => {
                        ReadableTransactionKindRef::ConsensusCommitPrologueV2(k)
                    }
                };
                readable.serialize(serializer)
            } else {
                let binary = match self {
                    TransactionKind::ProgrammableTransaction(k) => {
                        BinaryTransactionKindRef::ProgrammableTransaction(k)
                    }
                    TransactionKind::ChangeEpoch(k) => BinaryTransactionKindRef::ChangeEpoch(k),
                    TransactionKind::Genesis(k) => BinaryTransactionKindRef::Genesis(k),
                    TransactionKind::ConsensusCommitPrologue(k) => {
                        BinaryTransactionKindRef::ConsensusCommitPrologue(k)
                    }
                    TransactionKind::AuthenticatorStateUpdate(k) => {
                        BinaryTransactionKindRef::AuthenticatorStateUpdate(k)
                    }
                    TransactionKind::EndOfEpoch(k) => BinaryTransactionKindRef::EndOfEpoch(k),
                    TransactionKind::RandomnessStateUpdate(k) => {
                        BinaryTransactionKindRef::RandomnessStateUpdate(k)
                    }
                    TransactionKind::ConsensusCommitPrologueV2(k) => {
                        BinaryTransactionKindRef::ConsensusCommitPrologueV2(k)
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
                        TransactionKind::ProgrammableTransaction(k)
                    }
                    ReadableTransactionKind::ChangeEpoch(k) => TransactionKind::ChangeEpoch(k),
                    ReadableTransactionKind::Genesis(k) => TransactionKind::Genesis(k),
                    ReadableTransactionKind::ConsensusCommitPrologue(k) => {
                        TransactionKind::ConsensusCommitPrologue(k)
                    }
                    ReadableTransactionKind::AuthenticatorStateUpdate(k) => {
                        TransactionKind::AuthenticatorStateUpdate(k)
                    }
                    ReadableTransactionKind::EndOfEpoch { commands } => {
                        TransactionKind::EndOfEpoch(commands)
                    }
                    ReadableTransactionKind::RandomnessStateUpdate(k) => {
                        TransactionKind::RandomnessStateUpdate(k)
                    }
                    ReadableTransactionKind::ConsensusCommitPrologueV2(k) => {
                        TransactionKind::ConsensusCommitPrologueV2(k)
                    }
                })
            } else {
                BinaryTransactionKind::deserialize(deserializer).map(|binary| match binary {
                    BinaryTransactionKind::ProgrammableTransaction(k) => {
                        TransactionKind::ProgrammableTransaction(k)
                    }
                    BinaryTransactionKind::ChangeEpoch(k) => TransactionKind::ChangeEpoch(k),
                    BinaryTransactionKind::Genesis(k) => TransactionKind::Genesis(k),
                    BinaryTransactionKind::ConsensusCommitPrologue(k) => {
                        TransactionKind::ConsensusCommitPrologue(k)
                    }
                    BinaryTransactionKind::AuthenticatorStateUpdate(k) => {
                        TransactionKind::AuthenticatorStateUpdate(k)
                    }
                    BinaryTransactionKind::EndOfEpoch(k) => TransactionKind::EndOfEpoch(k),
                    BinaryTransactionKind::RandomnessStateUpdate(k) => {
                        TransactionKind::RandomnessStateUpdate(k)
                    }
                    BinaryTransactionKind::ConsensusCommitPrologueV2(k) => {
                        TransactionKind::ConsensusCommitPrologueV2(k)
                    }
                })
            }
        }
    }
}

mod end_of_epoch {
    use super::*;
    use crate::types::transaction::{
        AuthenticatorStateExpire, ChangeEpoch, EndOfEpochTransactionKind,
    };

    #[derive(serde_derive::Serialize)]
    #[serde(tag = "kind", rename_all = "kebab-case")]
    enum ReadableEndOfEpochTransactionKindRef<'a> {
        ChangeEpoch(&'a ChangeEpoch),
        AuthenticatorStateCreate,
        AuthenticatorStateExpire(&'a AuthenticatorStateExpire),
        RandomnessStateCreate,
        DenyListStateCreate,
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(tag = "kind", rename_all = "kebab-case")]
    enum ReadableEndOfEpochTransactionKind {
        ChangeEpoch(ChangeEpoch),
        AuthenticatorStateCreate,
        AuthenticatorStateExpire(AuthenticatorStateExpire),
        RandomnessStateCreate,
        DenyListStateCreate,
    }

    #[derive(serde_derive::Serialize)]
    enum BinaryEndOfEpochTransactionKindRef<'a> {
        ChangeEpoch(&'a ChangeEpoch),
        AuthenticatorStateCreate,
        AuthenticatorStateExpire(&'a AuthenticatorStateExpire),
        RandomnessStateCreate,
        DenyListStateCreate,
    }

    #[derive(serde_derive::Deserialize)]
    enum BinaryEndOfEpochTransactionKind {
        ChangeEpoch(ChangeEpoch),
        AuthenticatorStateCreate,
        AuthenticatorStateExpire(AuthenticatorStateExpire),
        RandomnessStateCreate,
        DenyListStateCreate,
    }

    impl Serialize for EndOfEpochTransactionKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                let readable = match self {
                    EndOfEpochTransactionKind::ChangeEpoch(k) => {
                        ReadableEndOfEpochTransactionKindRef::ChangeEpoch(k)
                    }
                    EndOfEpochTransactionKind::AuthenticatorStateCreate => {
                        ReadableEndOfEpochTransactionKindRef::AuthenticatorStateCreate
                    }
                    EndOfEpochTransactionKind::AuthenticatorStateExpire(k) => {
                        ReadableEndOfEpochTransactionKindRef::AuthenticatorStateExpire(k)
                    }
                    EndOfEpochTransactionKind::RandomnessStateCreate => {
                        ReadableEndOfEpochTransactionKindRef::RandomnessStateCreate
                    }
                    EndOfEpochTransactionKind::DenyListStateCreate => {
                        ReadableEndOfEpochTransactionKindRef::DenyListStateCreate
                    }
                };
                readable.serialize(serializer)
            } else {
                let binary = match self {
                    EndOfEpochTransactionKind::ChangeEpoch(k) => {
                        BinaryEndOfEpochTransactionKindRef::ChangeEpoch(k)
                    }
                    EndOfEpochTransactionKind::AuthenticatorStateCreate => {
                        BinaryEndOfEpochTransactionKindRef::AuthenticatorStateCreate
                    }
                    EndOfEpochTransactionKind::AuthenticatorStateExpire(k) => {
                        BinaryEndOfEpochTransactionKindRef::AuthenticatorStateExpire(k)
                    }
                    EndOfEpochTransactionKind::RandomnessStateCreate => {
                        BinaryEndOfEpochTransactionKindRef::RandomnessStateCreate
                    }
                    EndOfEpochTransactionKind::DenyListStateCreate => {
                        BinaryEndOfEpochTransactionKindRef::DenyListStateCreate
                    }
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
                        ReadableEndOfEpochTransactionKind::ChangeEpoch(k) => {
                            EndOfEpochTransactionKind::ChangeEpoch(k)
                        }
                        ReadableEndOfEpochTransactionKind::AuthenticatorStateCreate => {
                            EndOfEpochTransactionKind::AuthenticatorStateCreate
                        }
                        ReadableEndOfEpochTransactionKind::AuthenticatorStateExpire(k) => {
                            EndOfEpochTransactionKind::AuthenticatorStateExpire(k)
                        }
                        ReadableEndOfEpochTransactionKind::RandomnessStateCreate => {
                            EndOfEpochTransactionKind::RandomnessStateCreate
                        }
                        ReadableEndOfEpochTransactionKind::DenyListStateCreate => {
                            EndOfEpochTransactionKind::DenyListStateCreate
                        }
                    }
                })
            } else {
                BinaryEndOfEpochTransactionKind::deserialize(deserializer).map(
                    |binary| match binary {
                        BinaryEndOfEpochTransactionKind::ChangeEpoch(k) => {
                            EndOfEpochTransactionKind::ChangeEpoch(k)
                        }
                        BinaryEndOfEpochTransactionKind::AuthenticatorStateCreate => {
                            EndOfEpochTransactionKind::AuthenticatorStateCreate
                        }
                        BinaryEndOfEpochTransactionKind::AuthenticatorStateExpire(k) => {
                            EndOfEpochTransactionKind::AuthenticatorStateExpire(k)
                        }
                        BinaryEndOfEpochTransactionKind::RandomnessStateCreate => {
                            EndOfEpochTransactionKind::RandomnessStateCreate
                        }
                        BinaryEndOfEpochTransactionKind::DenyListStateCreate => {
                            EndOfEpochTransactionKind::DenyListStateCreate
                        }
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
    #[serde(tag = "type", rename_all = "kebab-case")]
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
    #[serde(untagged)]
    enum ReadableArgument {
        GasCoin(GasCoin),
        Input {
            input: u16,
        },
        Result {
            result: u16,
            #[serde(skip_serializing_if = "Option::is_none")]
            subresult: Option<u16>,
        },
    }

    struct GasCoin;

    impl std::fmt::Display for GasCoin {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("gas-coin")
        }
    }

    impl std::str::FromStr for GasCoin {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s == "gas-coin" {
                Ok(Self)
            } else {
                Err("expected 'gas-coin'")
            }
        }
    }

    impl Serialize for GasCoin {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serde_with::DisplayFromStr::serialize_as(self, serializer)
        }
    }

    impl<'de> Deserialize<'de> for GasCoin {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            serde_with::DisplayFromStr::deserialize_as(deserializer)
        }
    }

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    pub enum BinaryArgument {
        GasCoin,
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
                    Argument::GasCoin => ReadableArgument::GasCoin(GasCoin),
                    Argument::Input(input) => ReadableArgument::Input { input },
                    Argument::Result(result) => ReadableArgument::Result {
                        result,
                        subresult: None,
                    },
                    Argument::NestedResult(result, subresult) => ReadableArgument::Result {
                        result,
                        subresult: Some(subresult),
                    },
                };
                readable.serialize(serializer)
            } else {
                let binary = match *self {
                    Argument::GasCoin => BinaryArgument::GasCoin,
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
                    ReadableArgument::GasCoin(_) => Argument::GasCoin,
                    ReadableArgument::Input { input } => Argument::Input(input),
                    ReadableArgument::Result {
                        result,
                        subresult: None,
                    } => Argument::Result(result),
                    ReadableArgument::Result {
                        result,
                        subresult: Some(sub),
                    } => Argument::NestedResult(result, sub),
                })
            } else {
                BinaryArgument::deserialize(deserializer).map(|binary| match binary {
                    BinaryArgument::GasCoin => Argument::GasCoin,
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

    use crate::types::transaction::{
        Command, MakeMoveVector, MergeCoins, MoveCall, Publish, SplitCoins, TransferObjects,
        Upgrade,
    };

    #[derive(serde_derive::Serialize)]
    #[serde(tag = "command", rename_all = "kebab-case")]
    pub enum ReadableCommandRef<'a> {
        MoveCall(&'a MoveCall),
        TransferObjects(&'a TransferObjects),
        SplitCoins(&'a SplitCoins),
        MergeCoins(&'a MergeCoins),
        Publish(&'a Publish),
        MakeMoveVector(&'a MakeMoveVector),
        Upgrade(&'a Upgrade),
    }

    #[derive(serde_derive::Deserialize)]
    #[serde(tag = "command", rename_all = "kebab-case")]
    pub enum ReadableCommand {
        MoveCall(MoveCall),
        TransferObjects(TransferObjects),
        SplitCoins(SplitCoins),
        MergeCoins(MergeCoins),
        Publish(Publish),
        MakeMoveVector(MakeMoveVector),
        Upgrade(Upgrade),
    }

    #[derive(serde_derive::Serialize)]
    pub enum BinaryCommandRef<'a> {
        MoveCall(&'a MoveCall),
        TransferObjects(&'a TransferObjects),
        SplitCoins(&'a SplitCoins),
        MergeCoins(&'a MergeCoins),
        Publish(&'a Publish),
        MakeMoveVector(&'a MakeMoveVector),
        Upgrade(&'a Upgrade),
    }

    #[derive(serde_derive::Deserialize)]
    pub enum BinaryCommand {
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

#[cfg(test)]
mod test {
    use base64ct::{Base64, Encoding};

    use crate::types::{
        transaction::{Argument, InputArgument, Transaction},
        ObjectDigest, ObjectId, ObjectReference,
    };

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn argument() {
        let test_cases = [
            (Argument::GasCoin, serde_json::json!("gas-coin")),
            (Argument::Input(1), serde_json::json!({"input": 1})),
            (Argument::Result(2), serde_json::json!({"result": 2})),
            (
                Argument::NestedResult(3, 4),
                serde_json::json!({"result": 3, "subresult": 4}),
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
                  "type": "immutable-or-owned",
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
