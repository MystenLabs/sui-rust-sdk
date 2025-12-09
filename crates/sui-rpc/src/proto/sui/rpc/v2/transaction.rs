use super::*;
use crate::field::FieldMaskTree;
use crate::merge::Merge;
use crate::proto::TryFromProtoError;
use tap::Pipe;

//
// Transaction
//

impl From<sui_sdk_types::Transaction> for Transaction {
    fn from(value: sui_sdk_types::Transaction) -> Self {
        Self::merge_from(value, &FieldMaskTree::new_wildcard())
    }
}

impl Merge<sui_sdk_types::Transaction> for Transaction {
    fn merge(&mut self, source: sui_sdk_types::Transaction, mask: &FieldMaskTree) {
        if mask.contains(Self::BCS_FIELD.name) {
            let mut bcs = Bcs::serialize(&source).unwrap();
            bcs.name = Some("TransactionData".to_owned());
            self.bcs = Some(bcs);
        }

        if mask.contains(Self::DIGEST_FIELD.name) {
            self.digest = Some(source.digest().to_string());
        }

        if mask.contains(Self::VERSION_FIELD.name) {
            self.version = Some(1);
        }

        if mask.contains(Self::KIND_FIELD.name) {
            self.kind = Some(source.kind.into());
        }

        if mask.contains(Self::SENDER_FIELD.name) {
            self.sender = Some(source.sender.to_string());
        }

        if mask.contains(Self::GAS_PAYMENT_FIELD.name) {
            self.gas_payment = Some(source.gas_payment.into());
        }

        if mask.contains(Self::EXPIRATION_FIELD.name) {
            self.expiration = Some(source.expiration.into());
        }
    }
}

impl Merge<&Transaction> for Transaction {
    fn merge(&mut self, source: &Transaction, mask: &FieldMaskTree) {
        let Transaction {
            bcs,
            digest,
            version,
            kind,
            sender,
            gas_payment,
            expiration,
        } = source;

        if mask.contains(Self::BCS_FIELD.name) {
            self.bcs = bcs.clone();
        }

        if mask.contains(Self::DIGEST_FIELD.name) {
            self.digest = digest.clone();
        }

        if mask.contains(Self::VERSION_FIELD.name) {
            self.version = *version;
        }

        if mask.contains(Self::KIND_FIELD.name) {
            self.kind = kind.clone();
        }

        if mask.contains(Self::SENDER_FIELD.name) {
            self.sender = sender.clone();
        }

        if mask.contains(Self::GAS_PAYMENT_FIELD.name) {
            self.gas_payment = gas_payment.clone();
        }

        if mask.contains(Self::EXPIRATION_FIELD.name) {
            self.expiration = expiration.clone();
        }
    }
}

impl TryFrom<&Transaction> for sui_sdk_types::Transaction {
    type Error = TryFromProtoError;

    fn try_from(value: &Transaction) -> Result<Self, Self::Error> {
        if let Some(bcs) = &value.bcs {
            return bcs
                .deserialize()
                .map_err(|e| TryFromProtoError::invalid(Transaction::BCS_FIELD, e));
        }

        match value.version {
            Some(1) => {}
            v => {
                return Err(TryFromProtoError::invalid(
                    Transaction::VERSION_FIELD,
                    format!("unknown Transaction version {v:?}"),
                ));
            }
        }

        let kind = value
            .kind
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("kind"))?
            .try_into()?;

        let sender = value
            .sender
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("sender"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(Transaction::SENDER_FIELD, e))?;

        let gas_payment = value
            .gas_payment
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("gas_payment"))?
            .try_into()?;

        let expiration = value
            .expiration
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("expiration"))?
            .try_into()?;

        Ok(Self {
            kind,
            sender,
            gas_payment,
            expiration,
        })
    }
}

//
// GasPayment
//

impl From<sui_sdk_types::GasPayment> for GasPayment {
    fn from(value: sui_sdk_types::GasPayment) -> Self {
        Self {
            objects: value.objects.into_iter().map(Into::into).collect(),
            owner: Some(value.owner.to_string()),
            price: Some(value.price),
            budget: Some(value.budget),
        }
    }
}

impl TryFrom<&GasPayment> for sui_sdk_types::GasPayment {
    type Error = TryFromProtoError;

    fn try_from(value: &GasPayment) -> Result<Self, Self::Error> {
        let objects = value
            .objects
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let owner = value
            .owner
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("owner"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(GasPayment::OWNER_FIELD, e))?;
        let price = value
            .price
            .ok_or_else(|| TryFromProtoError::missing("price"))?;
        let budget = value
            .budget
            .ok_or_else(|| TryFromProtoError::missing("budget"))?;
        Ok(Self {
            objects,
            owner,
            price,
            budget,
        })
    }
}

//
// TransactionExpiration
//

impl From<sui_sdk_types::TransactionExpiration> for TransactionExpiration {
    fn from(value: sui_sdk_types::TransactionExpiration) -> Self {
        use sui_sdk_types::TransactionExpiration::*;
        use transaction_expiration::TransactionExpirationKind;

        let mut message = Self::default();

        let kind = match value {
            None => TransactionExpirationKind::None,
            Epoch(epoch) => {
                message.epoch = Some(epoch);
                TransactionExpirationKind::Epoch
            }
            ValidDuring {
                min_epoch,
                max_epoch,
                min_timestamp_seconds: _,
                max_timestamp_seconds: _,
                chain,
                nonce,
            } => {
                message.epoch = max_epoch;
                message.min_epoch = min_epoch;
                message.set_chain(chain);
                message.set_nonce(nonce);
                TransactionExpirationKind::ValidDuring
            }
            _ => TransactionExpirationKind::Unknown,
        };

        message.set_kind(kind);
        message
    }
}

impl TryFrom<&TransactionExpiration> for sui_sdk_types::TransactionExpiration {
    type Error = TryFromProtoError;

    fn try_from(value: &TransactionExpiration) -> Result<Self, Self::Error> {
        use transaction_expiration::TransactionExpirationKind;

        match value.kind() {
            TransactionExpirationKind::Unknown => {
                return Err(TryFromProtoError::invalid(
                    TransactionExpiration::KIND_FIELD,
                    "unknown TransactionExpirationKind",
                ));
            }
            TransactionExpirationKind::None => Self::None,
            TransactionExpirationKind::Epoch => Self::Epoch(value.epoch()),
            TransactionExpirationKind::ValidDuring => Self::ValidDuring {
                min_epoch: value.min_epoch_opt(),
                max_epoch: value.epoch_opt(),
                min_timestamp_seconds: None,
                max_timestamp_seconds: None,
                chain: value
                    .chain_opt()
                    .ok_or_else(|| TryFromProtoError::missing("chain"))?
                    .parse()
                    .map_err(|e| TryFromProtoError::invalid("chain", e))?,
                nonce: value
                    .nonce_opt()
                    .ok_or_else(|| TryFromProtoError::missing("nonce"))?,
            },
        }
        .pipe(Ok)
    }
}

//
// TransactionKind
//

impl From<ProgrammableTransaction> for TransactionKind {
    fn from(value: ProgrammableTransaction) -> Self {
        Self::default()
            .with_programmable_transaction(value)
            .with_kind(transaction_kind::Kind::ProgrammableTransaction)
    }
}

impl From<sui_sdk_types::TransactionKind> for TransactionKind {
    fn from(value: sui_sdk_types::TransactionKind) -> Self {
        use sui_sdk_types::TransactionKind as K;
        use transaction_kind::Kind;

        let message = Self::default();

        match value {
            K::ProgrammableTransaction(ptb) => message
                .with_programmable_transaction(ptb)
                .with_kind(Kind::ProgrammableTransaction),
            // K::ProgrammableSystemTransaction(ptb) => message
            //     .with_programmable_transaction(ptb)
            //     .with_kind(Kind::ProgrammableSystemTransaction),
            K::ChangeEpoch(change_epoch) => message
                .with_change_epoch(change_epoch)
                .with_kind(Kind::ChangeEpoch),
            K::Genesis(genesis) => message.with_genesis(genesis).with_kind(Kind::Genesis),
            K::ConsensusCommitPrologue(prologue) => message
                .with_consensus_commit_prologue(prologue)
                .with_kind(Kind::ConsensusCommitPrologueV1),
            K::AuthenticatorStateUpdate(update) => message
                .with_authenticator_state_update(update)
                .with_kind(Kind::AuthenticatorStateUpdate),
            K::EndOfEpoch(transactions) => message
                .with_end_of_epoch(EndOfEpochTransaction {
                    transactions: transactions.into_iter().map(Into::into).collect(),
                })
                .with_kind(Kind::EndOfEpoch),
            K::RandomnessStateUpdate(update) => message
                .with_randomness_state_update(update)
                .with_kind(Kind::RandomnessStateUpdate),
            K::ConsensusCommitPrologueV2(prologue) => message
                .with_consensus_commit_prologue(prologue)
                .with_kind(Kind::ConsensusCommitPrologueV2),
            K::ConsensusCommitPrologueV3(prologue) => message
                .with_consensus_commit_prologue(prologue)
                .with_kind(Kind::ConsensusCommitPrologueV3),
            K::ConsensusCommitPrologueV4(prologue) => message
                .with_consensus_commit_prologue(prologue)
                .with_kind(Kind::ConsensusCommitPrologueV4),
            _ => message,
        }
    }
}

impl TryFrom<&TransactionKind> for sui_sdk_types::TransactionKind {
    type Error = TryFromProtoError;

    fn try_from(value: &TransactionKind) -> Result<Self, Self::Error> {
        use transaction_kind::Kind;

        match value.kind() {
            Kind::Unknown => {
                return Err(TryFromProtoError::invalid(
                    "kind",
                    "unknown TransactionKind",
                ));
            }
            Kind::ProgrammableTransaction => {
                Self::ProgrammableTransaction(value.programmable_transaction().try_into()?)
            }
            Kind::ChangeEpoch => Self::ChangeEpoch(value.change_epoch().try_into()?),
            Kind::Genesis => Self::Genesis(value.genesis().try_into()?),
            Kind::ConsensusCommitPrologueV1 => {
                Self::ConsensusCommitPrologue(value.consensus_commit_prologue().try_into()?)
            }
            Kind::AuthenticatorStateUpdate => {
                Self::AuthenticatorStateUpdate(value.authenticator_state_update().try_into()?)
            }
            Kind::EndOfEpoch => Self::EndOfEpoch(
                value
                    .end_of_epoch()
                    .transactions()
                    .iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            ),
            Kind::RandomnessStateUpdate => {
                Self::RandomnessStateUpdate(value.randomness_state_update().try_into()?)
            }
            Kind::ConsensusCommitPrologueV2 => {
                Self::ConsensusCommitPrologueV2(value.consensus_commit_prologue().try_into()?)
            }
            Kind::ConsensusCommitPrologueV3 => {
                Self::ConsensusCommitPrologueV3(value.consensus_commit_prologue().try_into()?)
            }
            Kind::ConsensusCommitPrologueV4 => {
                Self::ConsensusCommitPrologueV4(value.consensus_commit_prologue().try_into()?)
            }
        }
        .pipe(Ok)
    }
}

//
// ConsensusCommitPrologue
//

impl From<sui_sdk_types::ConsensusCommitPrologue> for ConsensusCommitPrologue {
    fn from(value: sui_sdk_types::ConsensusCommitPrologue) -> Self {
        Self {
            epoch: Some(value.epoch),
            round: Some(value.round),
            commit_timestamp: Some(crate::proto::timestamp_ms_to_proto(
                value.commit_timestamp_ms,
            )),
            consensus_commit_digest: None,
            sub_dag_index: None,
            consensus_determined_version_assignments: None,
            additional_state_digest: None,
        }
    }
}

impl TryFrom<&ConsensusCommitPrologue> for sui_sdk_types::ConsensusCommitPrologue {
    type Error = TryFromProtoError;

    fn try_from(value: &ConsensusCommitPrologue) -> Result<Self, Self::Error> {
        let epoch = value
            .epoch
            .ok_or_else(|| TryFromProtoError::missing("epoch"))?;
        let round = value
            .round
            .ok_or_else(|| TryFromProtoError::missing("round"))?;
        let commit_timestamp_ms = value
            .commit_timestamp
            .ok_or_else(|| TryFromProtoError::missing("commit_timestamp"))?
            .pipe(crate::proto::proto_to_timestamp_ms)?;

        Ok(Self {
            epoch,
            round,
            commit_timestamp_ms,
        })
    }
}

impl From<sui_sdk_types::ConsensusCommitPrologueV2> for ConsensusCommitPrologue {
    fn from(value: sui_sdk_types::ConsensusCommitPrologueV2) -> Self {
        Self {
            epoch: Some(value.epoch),
            round: Some(value.round),
            commit_timestamp: Some(crate::proto::timestamp_ms_to_proto(
                value.commit_timestamp_ms,
            )),
            consensus_commit_digest: Some(value.consensus_commit_digest.to_string()),
            sub_dag_index: None,
            consensus_determined_version_assignments: None,
            additional_state_digest: None,
        }
    }
}

impl TryFrom<&ConsensusCommitPrologue> for sui_sdk_types::ConsensusCommitPrologueV2 {
    type Error = TryFromProtoError;

    fn try_from(value: &ConsensusCommitPrologue) -> Result<Self, Self::Error> {
        let epoch = value
            .epoch
            .ok_or_else(|| TryFromProtoError::missing("epoch"))?;
        let round = value
            .round
            .ok_or_else(|| TryFromProtoError::missing("round"))?;
        let commit_timestamp_ms = value
            .commit_timestamp
            .ok_or_else(|| TryFromProtoError::missing("commit_timestamp"))?
            .pipe(crate::proto::proto_to_timestamp_ms)?;

        let consensus_commit_digest = value
            .consensus_commit_digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("consensus_commit_digest"))?
            .parse()
            .map_err(|e| {
                TryFromProtoError::invalid(
                    ConsensusCommitPrologue::CONSENSUS_COMMIT_DIGEST_FIELD,
                    e,
                )
            })?;

        Ok(Self {
            epoch,
            round,
            commit_timestamp_ms,
            consensus_commit_digest,
        })
    }
}

impl From<sui_sdk_types::ConsensusCommitPrologueV3> for ConsensusCommitPrologue {
    fn from(value: sui_sdk_types::ConsensusCommitPrologueV3) -> Self {
        Self {
            epoch: Some(value.epoch),
            round: Some(value.round),
            commit_timestamp: Some(crate::proto::timestamp_ms_to_proto(
                value.commit_timestamp_ms,
            )),
            consensus_commit_digest: Some(value.consensus_commit_digest.to_string()),
            sub_dag_index: value.sub_dag_index,
            consensus_determined_version_assignments: Some(
                value.consensus_determined_version_assignments.into(),
            ),
            additional_state_digest: None,
        }
    }
}

impl TryFrom<&ConsensusCommitPrologue> for sui_sdk_types::ConsensusCommitPrologueV3 {
    type Error = TryFromProtoError;

    fn try_from(value: &ConsensusCommitPrologue) -> Result<Self, Self::Error> {
        let epoch = value
            .epoch
            .ok_or_else(|| TryFromProtoError::missing("epoch"))?;
        let round = value
            .round
            .ok_or_else(|| TryFromProtoError::missing("round"))?;
        let commit_timestamp_ms = value
            .commit_timestamp
            .ok_or_else(|| TryFromProtoError::missing("commit_timestamp"))?
            .pipe(crate::proto::proto_to_timestamp_ms)?;

        let consensus_commit_digest = value
            .consensus_commit_digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("consensus_commit_digest"))?
            .parse()
            .map_err(|e| {
                TryFromProtoError::invalid(
                    ConsensusCommitPrologue::CONSENSUS_COMMIT_DIGEST_FIELD,
                    e,
                )
            })?;

        let consensus_determined_version_assignments = value
            .consensus_determined_version_assignments
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("consensus_determined_version_assignments"))?
            .try_into()?;

        Ok(Self {
            epoch,
            round,
            commit_timestamp_ms,
            sub_dag_index: value.sub_dag_index,
            consensus_commit_digest,
            consensus_determined_version_assignments,
        })
    }
}

impl From<sui_sdk_types::ConsensusCommitPrologueV4> for ConsensusCommitPrologue {
    fn from(
        sui_sdk_types::ConsensusCommitPrologueV4 {
            epoch,
            round,
            sub_dag_index,
            commit_timestamp_ms,
            consensus_commit_digest,
            consensus_determined_version_assignments,
            additional_state_digest,
        }: sui_sdk_types::ConsensusCommitPrologueV4,
    ) -> Self {
        Self {
            epoch: Some(epoch),
            round: Some(round),
            commit_timestamp: Some(crate::proto::timestamp_ms_to_proto(commit_timestamp_ms)),
            consensus_commit_digest: Some(consensus_commit_digest.to_string()),
            sub_dag_index,
            consensus_determined_version_assignments: Some(
                consensus_determined_version_assignments.into(),
            ),
            additional_state_digest: Some(additional_state_digest.to_string()),
        }
    }
}

impl TryFrom<&ConsensusCommitPrologue> for sui_sdk_types::ConsensusCommitPrologueV4 {
    type Error = TryFromProtoError;

    fn try_from(value: &ConsensusCommitPrologue) -> Result<Self, Self::Error> {
        let epoch = value
            .epoch
            .ok_or_else(|| TryFromProtoError::missing("epoch"))?;
        let round = value
            .round
            .ok_or_else(|| TryFromProtoError::missing("round"))?;
        let commit_timestamp_ms = value
            .commit_timestamp
            .ok_or_else(|| TryFromProtoError::missing("commit_timestamp"))?
            .pipe(crate::proto::proto_to_timestamp_ms)?;

        let consensus_commit_digest = value
            .consensus_commit_digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("consensus_commit_digest"))?
            .parse()
            .map_err(|e| {
                TryFromProtoError::invalid(
                    ConsensusCommitPrologue::CONSENSUS_COMMIT_DIGEST_FIELD,
                    e,
                )
            })?;

        let consensus_determined_version_assignments = value
            .consensus_determined_version_assignments
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("consensus_determined_version_assignments"))?
            .try_into()?;

        let additional_state_digest = value
            .additional_state_digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("additional_state_digest"))?
            .parse()
            .map_err(|e| {
                TryFromProtoError::invalid(
                    ConsensusCommitPrologue::ADDITIONAL_STATE_DIGEST_FIELD,
                    e,
                )
            })?;

        Ok(Self {
            epoch,
            round,
            commit_timestamp_ms,
            sub_dag_index: value.sub_dag_index,
            consensus_commit_digest,
            consensus_determined_version_assignments,
            additional_state_digest,
        })
    }
}

//
// ConsensusDeterminedVersionAssignments
//

impl From<sui_sdk_types::ConsensusDeterminedVersionAssignments>
    for ConsensusDeterminedVersionAssignments
{
    fn from(value: sui_sdk_types::ConsensusDeterminedVersionAssignments) -> Self {
        use sui_sdk_types::ConsensusDeterminedVersionAssignments::*;

        let mut message = Self::default();

        let version = match value {
            CanceledTransactions {
                canceled_transactions,
            } => {
                message.canceled_transactions =
                    canceled_transactions.into_iter().map(Into::into).collect();
                1
            }
            CanceledTransactionsV2 {
                canceled_transactions,
            } => {
                message.canceled_transactions =
                    canceled_transactions.into_iter().map(Into::into).collect();
                2
            }
            _ => return Self::default(),
        };

        message.version = Some(version);
        message
    }
}

impl TryFrom<&ConsensusDeterminedVersionAssignments>
    for sui_sdk_types::ConsensusDeterminedVersionAssignments
{
    type Error = TryFromProtoError;

    fn try_from(value: &ConsensusDeterminedVersionAssignments) -> Result<Self, Self::Error> {
        match value.version() {
            1 => Self::CanceledTransactions {
                canceled_transactions: value
                    .canceled_transactions
                    .iter()
                    .map(TryFrom::try_from)
                    .collect::<Result<_, _>>()?,
            },
            2 => Self::CanceledTransactionsV2 {
                canceled_transactions: value
                    .canceled_transactions
                    .iter()
                    .map(TryFrom::try_from)
                    .collect::<Result<_, _>>()?,
            },
            _ => {
                return Err(TryFromProtoError::invalid(
                    ConsensusDeterminedVersionAssignments::VERSION_FIELD,
                    "unknown ConsensusDeterminedVersionAssignments version",
                ));
            }
        }
        .pipe(Ok)
    }
}

//
// CanceledTransaction
//

impl From<sui_sdk_types::CanceledTransaction> for CanceledTransaction {
    fn from(value: sui_sdk_types::CanceledTransaction) -> Self {
        Self {
            digest: Some(value.digest.to_string()),
            version_assignments: value
                .version_assignments
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl TryFrom<&CanceledTransaction> for sui_sdk_types::CanceledTransaction {
    type Error = TryFromProtoError;

    fn try_from(value: &CanceledTransaction) -> Result<Self, Self::Error> {
        let digest = value
            .digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("digest"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(CanceledTransaction::DIGEST_FIELD, e))?;

        let version_assignments = value
            .version_assignments
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            digest,
            version_assignments,
        })
    }
}

impl From<sui_sdk_types::CanceledTransactionV2> for CanceledTransaction {
    fn from(value: sui_sdk_types::CanceledTransactionV2) -> Self {
        Self {
            digest: Some(value.digest.to_string()),
            version_assignments: value
                .version_assignments
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl TryFrom<&CanceledTransaction> for sui_sdk_types::CanceledTransactionV2 {
    type Error = TryFromProtoError;

    fn try_from(value: &CanceledTransaction) -> Result<Self, Self::Error> {
        let digest = value
            .digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("digest"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(CanceledTransaction::DIGEST_FIELD, e))?;

        let version_assignments = value
            .version_assignments
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            digest,
            version_assignments,
        })
    }
}

//
// VersionAssignment
//

impl From<sui_sdk_types::VersionAssignment> for VersionAssignment {
    fn from(value: sui_sdk_types::VersionAssignment) -> Self {
        Self {
            object_id: Some(value.object_id.to_string()),
            start_version: None,
            version: Some(value.version),
        }
    }
}

impl TryFrom<&VersionAssignment> for sui_sdk_types::VersionAssignment {
    type Error = TryFromProtoError;

    fn try_from(value: &VersionAssignment) -> Result<Self, Self::Error> {
        let object_id = value
            .object_id
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("object_id"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(VersionAssignment::OBJECT_ID_FIELD, e))?;
        let version = value
            .version
            .ok_or_else(|| TryFromProtoError::missing("version"))?;

        Ok(Self { object_id, version })
    }
}

impl From<sui_sdk_types::VersionAssignmentV2> for VersionAssignment {
    fn from(value: sui_sdk_types::VersionAssignmentV2) -> Self {
        Self {
            object_id: Some(value.object_id.to_string()),
            start_version: Some(value.start_version),
            version: Some(value.version),
        }
    }
}

impl TryFrom<&VersionAssignment> for sui_sdk_types::VersionAssignmentV2 {
    type Error = TryFromProtoError;

    fn try_from(value: &VersionAssignment) -> Result<Self, Self::Error> {
        let object_id = value
            .object_id
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("object_id"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(VersionAssignment::OBJECT_ID_FIELD, e))?;
        let start_version = value
            .start_version
            .ok_or_else(|| TryFromProtoError::missing(VersionAssignment::START_VERSION_FIELD))?;
        let version = value
            .version
            .ok_or_else(|| TryFromProtoError::missing("version"))?;

        Ok(Self {
            object_id,
            start_version,
            version,
        })
    }
}

//
// GenesisTransaction
//

impl From<sui_sdk_types::GenesisTransaction> for GenesisTransaction {
    fn from(value: sui_sdk_types::GenesisTransaction) -> Self {
        Self {
            objects: value.objects.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<&GenesisTransaction> for sui_sdk_types::GenesisTransaction {
    type Error = TryFromProtoError;

    fn try_from(value: &GenesisTransaction) -> Result<Self, Self::Error> {
        let objects = value
            .objects
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        Ok(Self { objects })
    }
}

//
// RandomnessStateUpdate
//

impl From<sui_sdk_types::RandomnessStateUpdate> for RandomnessStateUpdate {
    fn from(value: sui_sdk_types::RandomnessStateUpdate) -> Self {
        Self {
            epoch: Some(value.epoch),
            randomness_round: Some(value.randomness_round),
            random_bytes: Some(value.random_bytes.into()),
            randomness_object_initial_shared_version: Some(
                value.randomness_obj_initial_shared_version,
            ),
        }
    }
}

impl TryFrom<&RandomnessStateUpdate> for sui_sdk_types::RandomnessStateUpdate {
    type Error = TryFromProtoError;

    fn try_from(
        RandomnessStateUpdate {
            epoch,
            randomness_round,
            random_bytes,
            randomness_object_initial_shared_version,
        }: &RandomnessStateUpdate,
    ) -> Result<Self, Self::Error> {
        let epoch = epoch.ok_or_else(|| TryFromProtoError::missing("epoch"))?;
        let randomness_round =
            randomness_round.ok_or_else(|| TryFromProtoError::missing("randomness_round"))?;
        let random_bytes = random_bytes
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("random_bytes"))?
            .to_vec();
        let randomness_obj_initial_shared_version = randomness_object_initial_shared_version
            .ok_or_else(|| {
                TryFromProtoError::missing("randomness_object_initial_shared_version")
            })?;
        Ok(Self {
            epoch,
            randomness_round,
            random_bytes,
            randomness_obj_initial_shared_version,
        })
    }
}

//
// AuthenticatorStateUpdate
//

impl From<sui_sdk_types::AuthenticatorStateUpdate> for AuthenticatorStateUpdate {
    fn from(value: sui_sdk_types::AuthenticatorStateUpdate) -> Self {
        Self {
            epoch: Some(value.epoch),
            round: Some(value.round),
            new_active_jwks: value.new_active_jwks.into_iter().map(Into::into).collect(),
            authenticator_object_initial_shared_version: Some(
                value.authenticator_obj_initial_shared_version,
            ),
        }
    }
}

impl TryFrom<&AuthenticatorStateUpdate> for sui_sdk_types::AuthenticatorStateUpdate {
    type Error = TryFromProtoError;

    fn try_from(
        AuthenticatorStateUpdate {
            epoch,
            round,
            new_active_jwks,
            authenticator_object_initial_shared_version,
        }: &AuthenticatorStateUpdate,
    ) -> Result<Self, Self::Error> {
        let epoch = epoch.ok_or_else(|| TryFromProtoError::missing("epoch"))?;
        let round = round.ok_or_else(|| TryFromProtoError::missing("round"))?;
        let authenticator_obj_initial_shared_version = authenticator_object_initial_shared_version
            .ok_or_else(|| {
                TryFromProtoError::missing("authenticator_object_initial_shared_version")
            })?;
        Ok(Self {
            epoch,
            round,
            new_active_jwks: new_active_jwks
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            authenticator_obj_initial_shared_version,
        })
    }
}

//
// Jwk
//

impl From<sui_sdk_types::Jwk> for Jwk {
    fn from(sui_sdk_types::Jwk { kty, e, n, alg }: sui_sdk_types::Jwk) -> Self {
        Self {
            kty: Some(kty),
            e: Some(e),
            n: Some(n),
            alg: Some(alg),
        }
    }
}

impl TryFrom<&Jwk> for sui_sdk_types::Jwk {
    type Error = TryFromProtoError;

    fn try_from(Jwk { kty, e, n, alg }: &Jwk) -> Result<Self, Self::Error> {
        let kty = kty
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("kty"))?
            .into();
        let e = e
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("e"))?
            .into();
        let n = n
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("n"))?
            .into();
        let alg = alg
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("alg"))?
            .into();
        Ok(Self { kty, e, n, alg })
    }
}

//
// JwkId
//

impl From<sui_sdk_types::JwkId> for JwkId {
    fn from(sui_sdk_types::JwkId { iss, kid }: sui_sdk_types::JwkId) -> Self {
        Self {
            iss: Some(iss),
            kid: Some(kid),
        }
    }
}

impl From<&sui_sdk_types::JwkId> for JwkId {
    fn from(value: &sui_sdk_types::JwkId) -> Self {
        Self {
            iss: Some(value.iss.clone()),
            kid: Some(value.kid.clone()),
        }
    }
}

impl TryFrom<&JwkId> for sui_sdk_types::JwkId {
    type Error = TryFromProtoError;

    fn try_from(JwkId { iss, kid }: &JwkId) -> Result<Self, Self::Error> {
        let iss = iss
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("iss"))?
            .into();
        let kid = kid
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("kid"))?
            .into();
        Ok(Self { iss, kid })
    }
}

//
// ActiveJwk
//

impl From<sui_sdk_types::ActiveJwk> for ActiveJwk {
    fn from(value: sui_sdk_types::ActiveJwk) -> Self {
        Self {
            id: Some(value.jwk_id.into()),
            jwk: Some(value.jwk.into()),
            epoch: Some(value.epoch),
        }
    }
}

impl TryFrom<&ActiveJwk> for sui_sdk_types::ActiveJwk {
    type Error = TryFromProtoError;

    fn try_from(value: &ActiveJwk) -> Result<Self, Self::Error> {
        let jwk_id = value
            .id
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("id"))?
            .try_into()?;

        let jwk = value
            .jwk
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("jwk"))?
            .try_into()?;

        let epoch = value
            .epoch
            .ok_or_else(|| TryFromProtoError::missing("epoch"))?;

        Ok(Self { jwk_id, jwk, epoch })
    }
}

//
// ChangeEpoch
//

impl From<sui_sdk_types::ChangeEpoch> for ChangeEpoch {
    fn from(value: sui_sdk_types::ChangeEpoch) -> Self {
        Self {
            epoch: Some(value.epoch),
            protocol_version: Some(value.protocol_version),
            storage_charge: Some(value.storage_charge),
            computation_charge: Some(value.computation_charge),
            storage_rebate: Some(value.storage_rebate),
            non_refundable_storage_fee: Some(value.non_refundable_storage_fee),
            epoch_start_timestamp: Some(crate::proto::timestamp_ms_to_proto(
                value.epoch_start_timestamp_ms,
            )),
            system_packages: value.system_packages.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<&ChangeEpoch> for sui_sdk_types::ChangeEpoch {
    type Error = TryFromProtoError;

    fn try_from(
        ChangeEpoch {
            epoch,
            protocol_version,
            storage_charge,
            computation_charge,
            storage_rebate,
            non_refundable_storage_fee,
            epoch_start_timestamp,
            system_packages,
        }: &ChangeEpoch,
    ) -> Result<Self, Self::Error> {
        let epoch = epoch.ok_or_else(|| TryFromProtoError::missing("epoch"))?;
        let protocol_version =
            protocol_version.ok_or_else(|| TryFromProtoError::missing("protocol_version"))?;
        let storage_charge =
            storage_charge.ok_or_else(|| TryFromProtoError::missing("storage_charge"))?;
        let computation_charge =
            computation_charge.ok_or_else(|| TryFromProtoError::missing("computation_charge"))?;
        let storage_rebate =
            storage_rebate.ok_or_else(|| TryFromProtoError::missing("storage_rebate"))?;
        let non_refundable_storage_fee = non_refundable_storage_fee
            .ok_or_else(|| TryFromProtoError::missing("non_refundable_storage_fee"))?;
        let epoch_start_timestamp_ms = epoch_start_timestamp
            .ok_or_else(|| TryFromProtoError::missing("epoch_start_timestamp_ms"))?
            .pipe(crate::proto::proto_to_timestamp_ms)?;

        Ok(Self {
            epoch,
            protocol_version,
            storage_charge,
            computation_charge,
            storage_rebate,
            non_refundable_storage_fee,
            epoch_start_timestamp_ms,
            system_packages: system_packages
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

//
// SystemPackage
//

impl From<sui_sdk_types::SystemPackage> for SystemPackage {
    fn from(value: sui_sdk_types::SystemPackage) -> Self {
        Self {
            version: Some(value.version),
            modules: value.modules.into_iter().map(Into::into).collect(),
            dependencies: value.dependencies.iter().map(ToString::to_string).collect(),
        }
    }
}

impl TryFrom<&SystemPackage> for sui_sdk_types::SystemPackage {
    type Error = TryFromProtoError;

    fn try_from(value: &SystemPackage) -> Result<Self, Self::Error> {
        Ok(Self {
            version: value
                .version
                .ok_or_else(|| TryFromProtoError::missing("version"))?,
            modules: value.modules.iter().map(|bytes| bytes.to_vec()).collect(),
            dependencies: value
                .dependencies
                .iter()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()
                .map_err(|e| TryFromProtoError::invalid(SystemPackage::DEPENDENCIES_FIELD, e))?,
        })
    }
}

//
// EndOfEpochTransactionkind
//

impl From<sui_sdk_types::EndOfEpochTransactionKind> for EndOfEpochTransactionKind {
    fn from(value: sui_sdk_types::EndOfEpochTransactionKind) -> Self {
        use end_of_epoch_transaction_kind::Kind;
        use sui_sdk_types::EndOfEpochTransactionKind as K;

        let message = Self::default();

        match value {
            K::ChangeEpoch(change_epoch) => message
                .with_change_epoch(change_epoch)
                .with_kind(Kind::ChangeEpoch),
            K::AuthenticatorStateCreate => message.with_kind(Kind::AuthenticatorStateCreate),
            K::AuthenticatorStateExpire(expire) => message
                .with_authenticator_state_expire(expire)
                .with_kind(Kind::AuthenticatorStateExpire),
            K::RandomnessStateCreate => message.with_kind(Kind::RandomnessStateCreate),
            K::DenyListStateCreate => message.with_kind(Kind::DenyListStateCreate),
            K::BridgeStateCreate { chain_id } => message
                .with_bridge_chain_id(chain_id)
                .with_kind(Kind::BridgeStateCreate),
            K::BridgeCommitteeInit {
                bridge_object_version,
            } => message
                .with_bridge_object_version(bridge_object_version)
                .with_kind(Kind::BridgeCommitteeInit),
            K::StoreExecutionTimeObservations(observations) => message
                .with_execution_time_observations(observations)
                .with_kind(Kind::StoreExecutionTimeObservations),
            K::AccumulatorRootCreate => message.with_kind(Kind::AccumulatorRootCreate),
            K::CoinRegistryCreate => message.with_kind(Kind::CoinRegistryCreate),
            K::DisplayRegistryCreate => message.with_kind(Kind::DisplayRegistryCreate),
            _ => message,
        }
    }
}

impl TryFrom<&EndOfEpochTransactionKind> for sui_sdk_types::EndOfEpochTransactionKind {
    type Error = TryFromProtoError;

    fn try_from(value: &EndOfEpochTransactionKind) -> Result<Self, Self::Error> {
        use end_of_epoch_transaction_kind::Kind;

        match value.kind() {
            Kind::Unknown => {
                return Err(TryFromProtoError::invalid(
                    EndOfEpochTransactionKind::KIND_FIELD,
                    "unknown EndOfEpochTransactionKind",
                ));
            }
            Kind::ChangeEpoch => Self::ChangeEpoch(value.change_epoch().try_into()?),
            Kind::AuthenticatorStateCreate => Self::AuthenticatorStateCreate,
            Kind::AuthenticatorStateExpire => {
                Self::AuthenticatorStateExpire(value.authenticator_state_expire().try_into()?)
            }
            Kind::RandomnessStateCreate => Self::RandomnessStateCreate,
            Kind::DenyListStateCreate => Self::DenyListStateCreate,
            Kind::BridgeStateCreate => Self::BridgeStateCreate {
                chain_id: value.bridge_chain_id().parse().map_err(|e| {
                    TryFromProtoError::invalid(EndOfEpochTransactionKind::BRIDGE_CHAIN_ID_FIELD, e)
                })?,
            },
            Kind::BridgeCommitteeInit => Self::BridgeCommitteeInit {
                bridge_object_version: value.bridge_object_version(),
            },
            Kind::StoreExecutionTimeObservations => Self::StoreExecutionTimeObservations(
                value.execution_time_observations().try_into()?,
            ),
            Kind::AccumulatorRootCreate => Self::AccumulatorRootCreate,
            Kind::CoinRegistryCreate => Self::CoinRegistryCreate,
            Kind::DisplayRegistryCreate => Self::DisplayRegistryCreate,
        }
        .pipe(Ok)
    }
}

//
// AuthenticatorStateExpire
//

impl From<sui_sdk_types::AuthenticatorStateExpire> for AuthenticatorStateExpire {
    fn from(value: sui_sdk_types::AuthenticatorStateExpire) -> Self {
        Self {
            min_epoch: Some(value.min_epoch),
            authenticator_object_initial_shared_version: Some(
                value.authenticator_object_initial_shared_version,
            ),
        }
    }
}

impl TryFrom<&AuthenticatorStateExpire> for sui_sdk_types::AuthenticatorStateExpire {
    type Error = TryFromProtoError;

    fn try_from(
        AuthenticatorStateExpire {
            min_epoch,
            authenticator_object_initial_shared_version,
        }: &AuthenticatorStateExpire,
    ) -> Result<Self, Self::Error> {
        let min_epoch = min_epoch.ok_or_else(|| TryFromProtoError::missing("min_epoch"))?;
        let authenticator_object_initial_shared_version =
            authenticator_object_initial_shared_version.ok_or_else(|| {
                TryFromProtoError::missing("authenticator_object_initial_shared_version")
            })?;
        Ok(Self {
            min_epoch,
            authenticator_object_initial_shared_version,
        })
    }
}

// ExecutionTimeObservations

impl From<sui_sdk_types::ExecutionTimeObservations> for ExecutionTimeObservations {
    fn from(value: sui_sdk_types::ExecutionTimeObservations) -> Self {
        match value {
            sui_sdk_types::ExecutionTimeObservations::V1(vec) => Self {
                version: Some(1),
                observations: vec.into_iter().map(Into::into).collect(),
            },
            _ => Self::default(),
        }
    }
}

impl TryFrom<&ExecutionTimeObservations> for sui_sdk_types::ExecutionTimeObservations {
    type Error = TryFromProtoError;

    fn try_from(value: &ExecutionTimeObservations) -> Result<Self, Self::Error> {
        Ok(Self::V1(
            value
                .observations
                .iter()
                .map(|observation| observation.try_into())
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl
    From<(
        sui_sdk_types::ExecutionTimeObservationKey,
        Vec<sui_sdk_types::ValidatorExecutionTimeObservation>,
    )> for ExecutionTimeObservation
{
    fn from(
        value: (
            sui_sdk_types::ExecutionTimeObservationKey,
            Vec<sui_sdk_types::ValidatorExecutionTimeObservation>,
        ),
    ) -> Self {
        use execution_time_observation::ExecutionTimeObservationKind;
        use sui_sdk_types::ExecutionTimeObservationKey;

        let mut message = Self::default();

        let kind = match value.0 {
            ExecutionTimeObservationKey::MoveEntryPoint {
                package,
                module,
                function,
                type_arguments,
            } => {
                message.move_entry_point = Some(MoveCall {
                    package: Some(package.to_string()),
                    module: Some(module),
                    function: Some(function),
                    type_arguments: type_arguments
                        .into_iter()
                        .map(|ty| ty.to_string())
                        .collect(),
                    arguments: Vec::new(),
                });
                ExecutionTimeObservationKind::MoveEntryPoint
            }
            ExecutionTimeObservationKey::TransferObjects => {
                ExecutionTimeObservationKind::TransferObjects
            }
            ExecutionTimeObservationKey::SplitCoins => ExecutionTimeObservationKind::SplitCoins,
            ExecutionTimeObservationKey::MergeCoins => ExecutionTimeObservationKind::MergeCoins,
            ExecutionTimeObservationKey::Publish => ExecutionTimeObservationKind::Publish,
            ExecutionTimeObservationKey::MakeMoveVec => {
                ExecutionTimeObservationKind::MakeMoveVector
            }
            ExecutionTimeObservationKey::Upgrade => ExecutionTimeObservationKind::Upgrade,

            _ => ExecutionTimeObservationKind::Unknown,
        };

        message.validator_observations = value.1.into_iter().map(Into::into).collect();
        message.set_kind(kind);
        message
    }
}

impl TryFrom<&ExecutionTimeObservation>
    for (
        sui_sdk_types::ExecutionTimeObservationKey,
        Vec<sui_sdk_types::ValidatorExecutionTimeObservation>,
    )
{
    type Error = TryFromProtoError;

    fn try_from(value: &ExecutionTimeObservation) -> Result<Self, Self::Error> {
        use execution_time_observation::ExecutionTimeObservationKind;
        use sui_sdk_types::ExecutionTimeObservationKey;

        let key = match value.kind() {
            ExecutionTimeObservationKind::Unknown => {
                return Err(TryFromProtoError::invalid(
                    ExecutionTimeObservation::KIND_FIELD,
                    "unknown ExecutionTimeObservationKind",
                ));
            }
            ExecutionTimeObservationKind::MoveEntryPoint => {
                let move_call = value
                    .move_entry_point
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("move_entry_point"))?;
                ExecutionTimeObservationKey::MoveEntryPoint {
                    package: move_call
                        .package()
                        .parse()
                        .map_err(|e| TryFromProtoError::invalid(MoveCall::PACKAGE_FIELD, e))?,
                    module: move_call.module().to_owned(),
                    function: move_call.function().to_owned(),
                    type_arguments: move_call
                        .type_arguments
                        .iter()
                        .map(|t| {
                            t.parse().map_err(|e| {
                                TryFromProtoError::invalid(MoveCall::TYPE_ARGUMENTS_FIELD, e)
                            })
                        })
                        .collect::<Result<_, _>>()?,
                }
            }
            ExecutionTimeObservationKind::TransferObjects => {
                ExecutionTimeObservationKey::TransferObjects
            }
            ExecutionTimeObservationKind::SplitCoins => ExecutionTimeObservationKey::SplitCoins,
            ExecutionTimeObservationKind::MergeCoins => ExecutionTimeObservationKey::MergeCoins,
            ExecutionTimeObservationKind::Publish => ExecutionTimeObservationKey::Publish,
            ExecutionTimeObservationKind::MakeMoveVector => {
                ExecutionTimeObservationKey::MakeMoveVec
            }
            ExecutionTimeObservationKind::Upgrade => ExecutionTimeObservationKey::Upgrade,
        };

        let observations = value
            .validator_observations
            .iter()
            .map(sui_sdk_types::ValidatorExecutionTimeObservation::try_from)
            .collect::<Result<_, _>>()?;

        Ok((key, observations))
    }
}

// ValidatorExecutionTimeObservation

impl From<sui_sdk_types::ValidatorExecutionTimeObservation> for ValidatorExecutionTimeObservation {
    fn from(value: sui_sdk_types::ValidatorExecutionTimeObservation) -> Self {
        Self {
            validator: Some(value.validator.as_bytes().to_vec().into()),
            duration: Some(prost_types::Duration {
                seconds: value.duration.as_secs() as i64,
                nanos: value.duration.subsec_nanos() as i32,
            }),
        }
    }
}

impl TryFrom<&ValidatorExecutionTimeObservation>
    for sui_sdk_types::ValidatorExecutionTimeObservation
{
    type Error = TryFromProtoError;

    fn try_from(value: &ValidatorExecutionTimeObservation) -> Result<Self, Self::Error> {
        Ok(Self {
            validator: value
                .validator
                .as_ref()
                .ok_or_else(|| TryFromProtoError::missing("validator"))?
                .as_ref()
                .pipe(sui_sdk_types::Bls12381PublicKey::from_bytes)
                .map_err(|e| {
                    TryFromProtoError::invalid(
                        ValidatorExecutionTimeObservation::VALIDATOR_FIELD,
                        e,
                    )
                })?,
            duration: value
                .duration
                .ok_or_else(|| TryFromProtoError::missing("duration"))?
                .try_into()
                .map_err(|e| {
                    TryFromProtoError::invalid(ValidatorExecutionTimeObservation::DURATION_FIELD, e)
                })?,
        })
    }
}

//
// ProgrammableTransaction
//

impl From<sui_sdk_types::ProgrammableTransaction> for ProgrammableTransaction {
    fn from(value: sui_sdk_types::ProgrammableTransaction) -> Self {
        Self {
            inputs: value.inputs.into_iter().map(Into::into).collect(),
            commands: value.commands.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<&ProgrammableTransaction> for sui_sdk_types::ProgrammableTransaction {
    type Error = TryFromProtoError;

    fn try_from(value: &ProgrammableTransaction) -> Result<Self, Self::Error> {
        Ok(Self {
            inputs: value
                .inputs
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            commands: value
                .commands
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

//
// Input
//

impl From<sui_sdk_types::Input> for Input {
    fn from(value: sui_sdk_types::Input) -> Self {
        use input::InputKind;
        use sui_sdk_types::Input::*;

        let mut message = Self::default();

        let kind = match value {
            Pure(value) => {
                message.pure = Some(value.into());
                InputKind::Pure
            }
            ImmutableOrOwned(reference) => {
                message.object_id = Some(reference.object_id().to_string());
                message.version = Some(reference.version());
                message.digest = Some(reference.digest().to_string());
                InputKind::ImmutableOrOwned
            }
            Shared(shared_input) => {
                message.object_id = Some(shared_input.object_id().to_string());
                message.version = Some(shared_input.version());
                message.mutable = Some(shared_input.mutability().is_mutable());
                message.set_mutability(shared_input.mutability().into());

                InputKind::Shared
            }
            Receiving(reference) => {
                message.object_id = Some(reference.object_id().to_string());
                message.version = Some(reference.version());
                message.digest = Some(reference.digest().to_string());
                InputKind::Receiving
            }
            FundsWithdrawal(funds_withdrawal) => {
                message.set_funds_withdrawal(funds_withdrawal);
                InputKind::FundsWithdrawal
            }
            _ => InputKind::Unknown,
        };

        message.set_kind(kind);
        message
    }
}

impl TryFrom<&Input> for sui_sdk_types::Input {
    type Error = TryFromProtoError;

    fn try_from(value: &Input) -> Result<Self, Self::Error> {
        use input::InputKind;

        match value.kind() {
            InputKind::Unknown => {
                return Err(TryFromProtoError::invalid(
                    Input::KIND_FIELD,
                    "unknown InputKind",
                ));
            }
            InputKind::Pure => Self::Pure(
                value
                    .pure
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("pure"))?
                    .to_vec(),
            ),
            InputKind::ImmutableOrOwned => {
                let object_id = value
                    .object_id
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("object_id"))?
                    .parse()
                    .map_err(|e| TryFromProtoError::invalid(Input::OBJECT_ID_FIELD, e))?;
                let version = value
                    .version
                    .ok_or_else(|| TryFromProtoError::missing("version"))?;
                let digest = value
                    .digest
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("digest"))?
                    .parse()
                    .map_err(|e| TryFromProtoError::invalid(Input::DIGEST_FIELD, e))?;
                let reference = sui_sdk_types::ObjectReference::new(object_id, version, digest);
                Self::ImmutableOrOwned(reference)
            }
            InputKind::Shared => {
                let object_id = value
                    .object_id
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("object_id"))?
                    .parse()
                    .map_err(|e| TryFromProtoError::invalid(Input::OBJECT_ID_FIELD, e))?;
                let initial_shared_version = value
                    .version
                    .ok_or_else(|| TryFromProtoError::missing("version"))?;

                let shared_input = match value.mutability() {
                    input::Mutability::Unknown => {
                        let mutable = value
                            .mutable
                            .ok_or_else(|| TryFromProtoError::missing("mutable"))?;
                        sui_sdk_types::SharedInput::new(object_id, initial_shared_version, mutable)
                    }
                    input::Mutability::Immutable => sui_sdk_types::SharedInput::new(
                        object_id,
                        initial_shared_version,
                        sui_sdk_types::Mutability::Immutable,
                    ),
                    input::Mutability::Mutable => sui_sdk_types::SharedInput::new(
                        object_id,
                        initial_shared_version,
                        sui_sdk_types::Mutability::Mutable,
                    ),
                    input::Mutability::NonExclusiveWrite => sui_sdk_types::SharedInput::new(
                        object_id,
                        initial_shared_version,
                        sui_sdk_types::Mutability::NonExclusiveWrite,
                    ),
                };

                Self::Shared(shared_input)
            }
            InputKind::Receiving => {
                let object_id = value
                    .object_id
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("object_id"))?
                    .parse()
                    .map_err(|e| TryFromProtoError::invalid(Input::OBJECT_ID_FIELD, e))?;
                let version = value
                    .version
                    .ok_or_else(|| TryFromProtoError::missing("version"))?;
                let digest = value
                    .digest
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("digest"))?
                    .parse()
                    .map_err(|e| TryFromProtoError::invalid(Input::DIGEST_FIELD, e))?;
                let reference = sui_sdk_types::ObjectReference::new(object_id, version, digest);
                Self::Receiving(reference)
            }
            InputKind::FundsWithdrawal => {
                let funds_withdrawal = value
                    .funds_withdrawal_opt()
                    .ok_or_else(|| TryFromProtoError::missing("funds_withdrawal"))?
                    .try_into()?;
                Self::FundsWithdrawal(funds_withdrawal)
            }
        }
        .pipe(Ok)
    }
}

impl From<sui_sdk_types::Mutability> for input::Mutability {
    fn from(value: sui_sdk_types::Mutability) -> Self {
        match value {
            sui_sdk_types::Mutability::Immutable => Self::Immutable,
            sui_sdk_types::Mutability::Mutable => Self::Mutable,
            sui_sdk_types::Mutability::NonExclusiveWrite => Self::NonExclusiveWrite,
        }
    }
}

//
// FundsWithdrawal
//

impl From<sui_sdk_types::FundsWithdrawal> for FundsWithdrawal {
    fn from(value: sui_sdk_types::FundsWithdrawal) -> Self {
        let mut message = Self::default();
        message.set_coin_type(value.coin_type());
        message.set_source(value.source().into());
        message.amount = value.amount();
        message
    }
}

impl TryFrom<&FundsWithdrawal> for sui_sdk_types::FundsWithdrawal {
    type Error = TryFromProtoError;

    fn try_from(value: &FundsWithdrawal) -> Result<Self, Self::Error> {
        use funds_withdrawal::Source;

        let amount = value
            .amount_opt()
            .ok_or_else(|| TryFromProtoError::missing("amount"))?;
        let coin_type = value
            .coin_type_opt()
            .ok_or_else(|| TryFromProtoError::missing("coin_type"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid("coin_type", e))?;
        let source = match value.source() {
            Source::Unknown => return Err(TryFromProtoError::invalid("source", "unknown source")),
            Source::Sender => sui_sdk_types::WithdrawFrom::Sender,
            Source::Sponsor => sui_sdk_types::WithdrawFrom::Sponsor,
        };

        Ok(Self::new(amount, coin_type, source))
    }
}

impl From<sui_sdk_types::WithdrawFrom> for funds_withdrawal::Source {
    fn from(value: sui_sdk_types::WithdrawFrom) -> Self {
        match value {
            sui_sdk_types::WithdrawFrom::Sender => Self::Sender,
            sui_sdk_types::WithdrawFrom::Sponsor => Self::Sponsor,
            _ => Self::Unknown,
        }
    }
}

//
// Argument
//

impl Argument {
    pub fn gas() -> Self {
        Self {
            kind: Some(argument::ArgumentKind::Gas.into()),
            input: None,
            result: None,
            subresult: None,
        }
    }

    pub fn new_input(input: u16) -> Self {
        Self {
            kind: Some(argument::ArgumentKind::Input.into()),
            input: Some(input.into()),
            result: None,
            subresult: None,
        }
    }

    pub fn new_result(command: u16) -> Self {
        Self {
            kind: Some(argument::ArgumentKind::Result.into()),
            input: None,
            result: Some(command.into()),
            subresult: None,
        }
    }

    pub fn nested_result(command: u16, subresult: u16) -> Self {
        Self {
            kind: Some(argument::ArgumentKind::Result.into()),
            input: None,
            result: Some(command.into()),
            subresult: Some(subresult.into()),
        }
    }
}

impl From<sui_sdk_types::Argument> for Argument {
    fn from(value: sui_sdk_types::Argument) -> Self {
        use argument::ArgumentKind;
        use sui_sdk_types::Argument::*;

        let mut message = Self::default();

        let kind = match value {
            Gas => ArgumentKind::Gas,
            Input(input) => {
                message.input = Some(input.into());
                ArgumentKind::Input
            }
            Result(result) => {
                message.result = Some(result.into());
                ArgumentKind::Result
            }
            NestedResult(result, subresult) => {
                message.result = Some(result.into());
                message.subresult = Some(subresult.into());
                ArgumentKind::Result
            }
        };

        message.set_kind(kind);
        message
    }
}

impl TryFrom<&Argument> for sui_sdk_types::Argument {
    type Error = TryFromProtoError;

    fn try_from(value: &Argument) -> Result<Self, Self::Error> {
        use argument::ArgumentKind;

        match value.kind() {
            ArgumentKind::Unknown => {
                return Err(TryFromProtoError::invalid(
                    Argument::KIND_FIELD,
                    "unknown ArgumentKind",
                ));
            }
            ArgumentKind::Gas => Self::Gas,
            ArgumentKind::Input => {
                let input = value
                    .input
                    .ok_or_else(|| TryFromProtoError::missing("input"))?
                    .try_into()
                    .map_err(|e| TryFromProtoError::invalid(Argument::INPUT_FIELD, e))?;
                Self::Input(input)
            }
            ArgumentKind::Result => {
                let result = value
                    .result
                    .ok_or_else(|| TryFromProtoError::missing("result"))?
                    .try_into()
                    .map_err(|e| TryFromProtoError::invalid(Argument::RESULT_FIELD, e))?;

                if let Some(subresult) = value.subresult {
                    Self::NestedResult(
                        result,
                        subresult.try_into().map_err(|e| {
                            TryFromProtoError::invalid(Argument::SUBRESULT_FIELD, e)
                        })?,
                    )
                } else {
                    Self::Result(result)
                }
            }
        }
        .pipe(Ok)
    }
}

//
// Command
//

impl From<command::Command> for Command {
    fn from(value: command::Command) -> Self {
        Self {
            command: Some(value),
        }
    }
}

impl From<MoveCall> for command::Command {
    fn from(value: MoveCall) -> Self {
        Self::MoveCall(value)
    }
}

impl From<MoveCall> for Command {
    fn from(value: MoveCall) -> Self {
        command::Command::from(value).into()
    }
}

impl From<TransferObjects> for command::Command {
    fn from(value: TransferObjects) -> Self {
        Self::TransferObjects(value)
    }
}

impl From<TransferObjects> for Command {
    fn from(value: TransferObjects) -> Self {
        command::Command::from(value).into()
    }
}

impl From<sui_sdk_types::Command> for Command {
    fn from(value: sui_sdk_types::Command) -> Self {
        use command::Command;
        use sui_sdk_types::Command::*;

        let command = match value {
            MoveCall(move_call) => Command::MoveCall(move_call.into()),
            TransferObjects(transfer_objects) => Command::TransferObjects(transfer_objects.into()),
            SplitCoins(split_coins) => Command::SplitCoins(split_coins.into()),
            MergeCoins(merge_coins) => Command::MergeCoins(merge_coins.into()),
            Publish(publish) => Command::Publish(publish.into()),
            MakeMoveVector(make_move_vector) => Command::MakeMoveVector(make_move_vector.into()),
            Upgrade(upgrade) => Command::Upgrade(upgrade.into()),

            //
            _ => return Self::default(),
        };

        Self {
            command: Some(command),
        }
    }
}

impl TryFrom<&Command> for sui_sdk_types::Command {
    type Error = TryFromProtoError;

    fn try_from(value: &Command) -> Result<Self, Self::Error> {
        use command::Command;

        match value
            .command
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("command"))?
        {
            Command::MoveCall(move_call) => Self::MoveCall(move_call.try_into()?),
            Command::TransferObjects(transfer_objects) => {
                Self::TransferObjects(transfer_objects.try_into()?)
            }
            Command::SplitCoins(split_coins) => Self::SplitCoins(split_coins.try_into()?),
            Command::MergeCoins(merge_coins) => Self::MergeCoins(merge_coins.try_into()?),
            Command::Publish(publish) => Self::Publish(publish.try_into()?),
            Command::MakeMoveVector(make_move_vector) => {
                Self::MakeMoveVector(make_move_vector.try_into()?)
            }
            Command::Upgrade(upgrade) => Self::Upgrade(upgrade.try_into()?),
        }
        .pipe(Ok)
    }
}

//
// MoveCall
//

impl From<sui_sdk_types::MoveCall> for MoveCall {
    fn from(value: sui_sdk_types::MoveCall) -> Self {
        Self {
            package: Some(value.package.to_string()),
            module: Some(value.module.to_string()),
            function: Some(value.function.to_string()),
            type_arguments: value
                .type_arguments
                .iter()
                .map(ToString::to_string)
                .collect(),
            arguments: value.arguments.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<&MoveCall> for sui_sdk_types::MoveCall {
    type Error = TryFromProtoError;

    fn try_from(value: &MoveCall) -> Result<Self, Self::Error> {
        let package = value
            .package
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("package"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(MoveCall::PACKAGE_FIELD, e))?;

        let module = value
            .module
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("module"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(MoveCall::MODULE_FIELD, e))?;

        let function = value
            .function
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("function"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(MoveCall::FUNCTION_FIELD, e))?;

        let type_arguments = value
            .type_arguments
            .iter()
            .map(|t| {
                t.parse()
                    .map_err(|e| TryFromProtoError::invalid(MoveCall::TYPE_ARGUMENTS_FIELD, e))
            })
            .collect::<Result<_, _>>()?;
        let arguments = value
            .arguments
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            package,
            module,
            function,
            type_arguments,
            arguments,
        })
    }
}

//
// TransferObjects
//

impl From<sui_sdk_types::TransferObjects> for TransferObjects {
    fn from(value: sui_sdk_types::TransferObjects) -> Self {
        Self {
            objects: value.objects.into_iter().map(Into::into).collect(),
            address: Some(value.address.into()),
        }
    }
}

impl TryFrom<&TransferObjects> for sui_sdk_types::TransferObjects {
    type Error = TryFromProtoError;

    fn try_from(value: &TransferObjects) -> Result<Self, Self::Error> {
        let objects = value
            .objects
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let address = value
            .address
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("address"))?
            .try_into()?;

        Ok(Self { objects, address })
    }
}

//
// SplitCoins
//

impl From<sui_sdk_types::SplitCoins> for SplitCoins {
    fn from(value: sui_sdk_types::SplitCoins) -> Self {
        Self {
            coin: Some(value.coin.into()),
            amounts: value.amounts.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<&SplitCoins> for sui_sdk_types::SplitCoins {
    type Error = TryFromProtoError;

    fn try_from(value: &SplitCoins) -> Result<Self, Self::Error> {
        let coin = value
            .coin
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("coin"))?
            .try_into()?;

        let amounts = value
            .amounts
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        Ok(Self { coin, amounts })
    }
}

//
// MergeCoins
//

impl From<sui_sdk_types::MergeCoins> for MergeCoins {
    fn from(value: sui_sdk_types::MergeCoins) -> Self {
        Self {
            coin: Some(value.coin.into()),
            coins_to_merge: value.coins_to_merge.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<&MergeCoins> for sui_sdk_types::MergeCoins {
    type Error = TryFromProtoError;

    fn try_from(value: &MergeCoins) -> Result<Self, Self::Error> {
        let coin = value
            .coin
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("coin"))?
            .try_into()?;

        let coins_to_merge = value
            .coins_to_merge
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            coin,
            coins_to_merge,
        })
    }
}

//
// Publish
//

impl From<sui_sdk_types::Publish> for Publish {
    fn from(value: sui_sdk_types::Publish) -> Self {
        Self {
            modules: value.modules.into_iter().map(Into::into).collect(),
            dependencies: value.dependencies.iter().map(ToString::to_string).collect(),
        }
    }
}

impl TryFrom<&Publish> for sui_sdk_types::Publish {
    type Error = TryFromProtoError;

    fn try_from(value: &Publish) -> Result<Self, Self::Error> {
        let modules = value.modules.iter().map(|bytes| bytes.to_vec()).collect();

        let dependencies = value
            .dependencies
            .iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .map_err(|e| TryFromProtoError::invalid(Publish::DEPENDENCIES_FIELD, e))?;

        Ok(Self {
            modules,
            dependencies,
        })
    }
}

//
// MakeMoveVector
//

impl From<sui_sdk_types::MakeMoveVector> for MakeMoveVector {
    fn from(value: sui_sdk_types::MakeMoveVector) -> Self {
        Self {
            element_type: value.type_.map(|t| t.to_string()),
            elements: value.elements.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<&MakeMoveVector> for sui_sdk_types::MakeMoveVector {
    type Error = TryFromProtoError;

    fn try_from(value: &MakeMoveVector) -> Result<Self, Self::Error> {
        let element_type = value
            .element_type
            .as_ref()
            .map(|t| {
                t.parse()
                    .map_err(|e| TryFromProtoError::invalid(MakeMoveVector::ELEMENT_TYPE_FIELD, e))
            })
            .transpose()?;

        let elements = value
            .elements
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            type_: element_type,
            elements,
        })
    }
}

//
// Upgrade
//

impl From<sui_sdk_types::Upgrade> for Upgrade {
    fn from(value: sui_sdk_types::Upgrade) -> Self {
        Self {
            modules: value.modules.into_iter().map(Into::into).collect(),
            dependencies: value.dependencies.iter().map(ToString::to_string).collect(),
            package: Some(value.package.to_string()),
            ticket: Some(value.ticket.into()),
        }
    }
}

impl TryFrom<&Upgrade> for sui_sdk_types::Upgrade {
    type Error = TryFromProtoError;

    fn try_from(value: &Upgrade) -> Result<Self, Self::Error> {
        let modules = value.modules.iter().map(|bytes| bytes.to_vec()).collect();

        let dependencies = value
            .dependencies
            .iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .map_err(|e| TryFromProtoError::invalid(Upgrade::DEPENDENCIES_FIELD, e))?;

        let package = value
            .package
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("package"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(Upgrade::PACKAGE_FIELD, e))?;

        let ticket = value
            .ticket
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("ticket"))?
            .try_into()?;

        Ok(Self {
            modules,
            dependencies,
            package,
            ticket,
        })
    }
}
