mod _field_impls {
    #![allow(clippy::wrong_self_convention)]
    use super::*;
    use crate::field::MessageFields;
    use crate::field::MessageField;
    impl Argument {
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 1i32,
            message_fields: None,
        };
        pub const INPUT_FIELD: &'static MessageField = &MessageField {
            name: "input",
            json_name: "input",
            number: 2i32,
            message_fields: None,
        };
        pub const RESULT_FIELD: &'static MessageField = &MessageField {
            name: "result",
            json_name: "result",
            number: 3i32,
            message_fields: None,
        };
        pub const SUBRESULT_FIELD: &'static MessageField = &MessageField {
            name: "subresult",
            json_name: "subresult",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for Argument {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::INPUT_FIELD,
            Self::RESULT_FIELD,
            Self::SUBRESULT_FIELD,
        ];
    }
    impl Argument {
        pub fn path_builder() -> ArgumentFieldPathBuilder {
            ArgumentFieldPathBuilder::new()
        }
    }
    pub struct ArgumentFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ArgumentFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kind(mut self) -> String {
            self.path.push(Argument::KIND_FIELD.name);
            self.finish()
        }
        pub fn input(mut self) -> String {
            self.path.push(Argument::INPUT_FIELD.name);
            self.finish()
        }
        pub fn result(mut self) -> String {
            self.path.push(Argument::RESULT_FIELD.name);
            self.finish()
        }
        pub fn subresult(mut self) -> String {
            self.path.push(Argument::SUBRESULT_FIELD.name);
            self.finish()
        }
    }
    impl BalanceChange {
        pub const ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "address",
            json_name: "address",
            number: 1i32,
            message_fields: None,
        };
        pub const COIN_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "coin_type",
            json_name: "coinType",
            number: 2i32,
            message_fields: None,
        };
        pub const AMOUNT_FIELD: &'static MessageField = &MessageField {
            name: "amount",
            json_name: "amount",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for BalanceChange {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ADDRESS_FIELD,
            Self::COIN_TYPE_FIELD,
            Self::AMOUNT_FIELD,
        ];
    }
    impl BalanceChange {
        pub fn path_builder() -> BalanceChangeFieldPathBuilder {
            BalanceChangeFieldPathBuilder::new()
        }
    }
    pub struct BalanceChangeFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl BalanceChangeFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn address(mut self) -> String {
            self.path.push(BalanceChange::ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn coin_type(mut self) -> String {
            self.path.push(BalanceChange::COIN_TYPE_FIELD.name);
            self.finish()
        }
        pub fn amount(mut self) -> String {
            self.path.push(BalanceChange::AMOUNT_FIELD.name);
            self.finish()
        }
    }
    impl Bcs {
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 1i32,
            message_fields: None,
        };
        pub const VALUE_FIELD: &'static MessageField = &MessageField {
            name: "value",
            json_name: "value",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for Bcs {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::NAME_FIELD,
            Self::VALUE_FIELD,
        ];
    }
    impl Bcs {
        pub fn path_builder() -> BcsFieldPathBuilder {
            BcsFieldPathBuilder::new()
        }
    }
    pub struct BcsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl BcsFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn name(mut self) -> String {
            self.path.push(Bcs::NAME_FIELD.name);
            self.finish()
        }
        pub fn value(mut self) -> String {
            self.path.push(Bcs::VALUE_FIELD.name);
            self.finish()
        }
    }
    impl Checkpoint {
        pub const SEQUENCE_NUMBER_FIELD: &'static MessageField = &MessageField {
            name: "sequence_number",
            json_name: "sequenceNumber",
            number: 1i32,
            message_fields: None,
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 2i32,
            message_fields: None,
        };
        pub const SUMMARY_FIELD: &'static MessageField = &MessageField {
            name: "summary",
            json_name: "summary",
            number: 3i32,
            message_fields: Some(CheckpointSummary::FIELDS),
        };
        pub const SIGNATURE_FIELD: &'static MessageField = &MessageField {
            name: "signature",
            json_name: "signature",
            number: 4i32,
            message_fields: Some(ValidatorAggregatedSignature::FIELDS),
        };
        pub const CONTENTS_FIELD: &'static MessageField = &MessageField {
            name: "contents",
            json_name: "contents",
            number: 5i32,
            message_fields: Some(CheckpointContents::FIELDS),
        };
        pub const TRANSACTIONS_FIELD: &'static MessageField = &MessageField {
            name: "transactions",
            json_name: "transactions",
            number: 6i32,
            message_fields: Some(ExecutedTransaction::FIELDS),
        };
        pub const OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "objects",
            json_name: "objects",
            number: 7i32,
            message_fields: Some(ObjectSet::FIELDS),
        };
    }
    impl MessageFields for Checkpoint {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::SEQUENCE_NUMBER_FIELD,
            Self::DIGEST_FIELD,
            Self::SUMMARY_FIELD,
            Self::SIGNATURE_FIELD,
            Self::CONTENTS_FIELD,
            Self::TRANSACTIONS_FIELD,
            Self::OBJECTS_FIELD,
        ];
    }
    impl Checkpoint {
        pub fn path_builder() -> CheckpointFieldPathBuilder {
            CheckpointFieldPathBuilder::new()
        }
    }
    pub struct CheckpointFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CheckpointFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn sequence_number(mut self) -> String {
            self.path.push(Checkpoint::SEQUENCE_NUMBER_FIELD.name);
            self.finish()
        }
        pub fn digest(mut self) -> String {
            self.path.push(Checkpoint::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn summary(mut self) -> CheckpointSummaryFieldPathBuilder {
            self.path.push(Checkpoint::SUMMARY_FIELD.name);
            CheckpointSummaryFieldPathBuilder::new_with_base(self.path)
        }
        pub fn signature(mut self) -> ValidatorAggregatedSignatureFieldPathBuilder {
            self.path.push(Checkpoint::SIGNATURE_FIELD.name);
            ValidatorAggregatedSignatureFieldPathBuilder::new_with_base(self.path)
        }
        pub fn contents(mut self) -> CheckpointContentsFieldPathBuilder {
            self.path.push(Checkpoint::CONTENTS_FIELD.name);
            CheckpointContentsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn transactions(mut self) -> ExecutedTransactionFieldPathBuilder {
            self.path.push(Checkpoint::TRANSACTIONS_FIELD.name);
            ExecutedTransactionFieldPathBuilder::new_with_base(self.path)
        }
        pub fn objects(mut self) -> ObjectSetFieldPathBuilder {
            self.path.push(Checkpoint::OBJECTS_FIELD.name);
            ObjectSetFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl CheckpointContents {
        pub const BCS_FIELD: &'static MessageField = &MessageField {
            name: "bcs",
            json_name: "bcs",
            number: 1i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 2i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 3i32,
            message_fields: None,
        };
        pub const TRANSACTIONS_FIELD: &'static MessageField = &MessageField {
            name: "transactions",
            json_name: "transactions",
            number: 4i32,
            message_fields: Some(CheckpointedTransactionInfo::FIELDS),
        };
    }
    impl MessageFields for CheckpointContents {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::BCS_FIELD,
            Self::DIGEST_FIELD,
            Self::VERSION_FIELD,
            Self::TRANSACTIONS_FIELD,
        ];
    }
    impl CheckpointContents {
        pub fn path_builder() -> CheckpointContentsFieldPathBuilder {
            CheckpointContentsFieldPathBuilder::new()
        }
    }
    pub struct CheckpointContentsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CheckpointContentsFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn bcs(mut self) -> BcsFieldPathBuilder {
            self.path.push(CheckpointContents::BCS_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn digest(mut self) -> String {
            self.path.push(CheckpointContents::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(CheckpointContents::VERSION_FIELD.name);
            self.finish()
        }
        pub fn transactions(mut self) -> CheckpointedTransactionInfoFieldPathBuilder {
            self.path.push(CheckpointContents::TRANSACTIONS_FIELD.name);
            CheckpointedTransactionInfoFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl CheckpointedTransactionInfo {
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 1i32,
            message_fields: None,
        };
        pub const EFFECTS_FIELD: &'static MessageField = &MessageField {
            name: "effects",
            json_name: "effects",
            number: 2i32,
            message_fields: None,
        };
        pub const SIGNATURES_FIELD: &'static MessageField = &MessageField {
            name: "signatures",
            json_name: "signatures",
            number: 3i32,
            message_fields: Some(UserSignature::FIELDS),
        };
        pub const ALIAS_CONFIG_VERSIONS_FIELD: &'static MessageField = &MessageField {
            name: "alias_config_versions",
            json_name: "aliasConfigVersions",
            number: 4i32,
            message_fields: Some(AliasConfigVersion::FIELDS),
        };
    }
    impl MessageFields for CheckpointedTransactionInfo {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TRANSACTION_FIELD,
            Self::EFFECTS_FIELD,
            Self::SIGNATURES_FIELD,
            Self::ALIAS_CONFIG_VERSIONS_FIELD,
        ];
    }
    impl CheckpointedTransactionInfo {
        pub fn path_builder() -> CheckpointedTransactionInfoFieldPathBuilder {
            CheckpointedTransactionInfoFieldPathBuilder::new()
        }
    }
    pub struct CheckpointedTransactionInfoFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CheckpointedTransactionInfoFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn transaction(mut self) -> String {
            self.path.push(CheckpointedTransactionInfo::TRANSACTION_FIELD.name);
            self.finish()
        }
        pub fn effects(mut self) -> String {
            self.path.push(CheckpointedTransactionInfo::EFFECTS_FIELD.name);
            self.finish()
        }
        pub fn signatures(mut self) -> UserSignatureFieldPathBuilder {
            self.path.push(CheckpointedTransactionInfo::SIGNATURES_FIELD.name);
            UserSignatureFieldPathBuilder::new_with_base(self.path)
        }
        pub fn alias_config_versions(mut self) -> AliasConfigVersionFieldPathBuilder {
            self.path
                .push(CheckpointedTransactionInfo::ALIAS_CONFIG_VERSIONS_FIELD.name);
            AliasConfigVersionFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl AliasConfigVersion {
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for AliasConfigVersion {
        const FIELDS: &'static [&'static MessageField] = &[Self::VERSION_FIELD];
    }
    impl AliasConfigVersion {
        pub fn path_builder() -> AliasConfigVersionFieldPathBuilder {
            AliasConfigVersionFieldPathBuilder::new()
        }
    }
    pub struct AliasConfigVersionFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl AliasConfigVersionFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn version(mut self) -> String {
            self.path.push(AliasConfigVersion::VERSION_FIELD.name);
            self.finish()
        }
    }
    impl CheckpointSummary {
        pub const BCS_FIELD: &'static MessageField = &MessageField {
            name: "bcs",
            json_name: "bcs",
            number: 1i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 2i32,
            message_fields: None,
        };
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 3i32,
            message_fields: None,
        };
        pub const SEQUENCE_NUMBER_FIELD: &'static MessageField = &MessageField {
            name: "sequence_number",
            json_name: "sequenceNumber",
            number: 4i32,
            message_fields: None,
        };
        pub const TOTAL_NETWORK_TRANSACTIONS_FIELD: &'static MessageField = &MessageField {
            name: "total_network_transactions",
            json_name: "totalNetworkTransactions",
            number: 5i32,
            message_fields: None,
        };
        pub const CONTENT_DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "content_digest",
            json_name: "contentDigest",
            number: 6i32,
            message_fields: None,
        };
        pub const PREVIOUS_DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "previous_digest",
            json_name: "previousDigest",
            number: 7i32,
            message_fields: None,
        };
        pub const EPOCH_ROLLING_GAS_COST_SUMMARY_FIELD: &'static MessageField = &MessageField {
            name: "epoch_rolling_gas_cost_summary",
            json_name: "epochRollingGasCostSummary",
            number: 8i32,
            message_fields: Some(GasCostSummary::FIELDS),
        };
        pub const TIMESTAMP_FIELD: &'static MessageField = &MessageField {
            name: "timestamp",
            json_name: "timestamp",
            number: 9i32,
            message_fields: None,
        };
        pub const COMMITMENTS_FIELD: &'static MessageField = &MessageField {
            name: "commitments",
            json_name: "commitments",
            number: 10i32,
            message_fields: Some(CheckpointCommitment::FIELDS),
        };
        pub const END_OF_EPOCH_DATA_FIELD: &'static MessageField = &MessageField {
            name: "end_of_epoch_data",
            json_name: "endOfEpochData",
            number: 11i32,
            message_fields: Some(EndOfEpochData::FIELDS),
        };
        pub const VERSION_SPECIFIC_DATA_FIELD: &'static MessageField = &MessageField {
            name: "version_specific_data",
            json_name: "versionSpecificData",
            number: 12i32,
            message_fields: None,
        };
    }
    impl MessageFields for CheckpointSummary {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::BCS_FIELD,
            Self::DIGEST_FIELD,
            Self::EPOCH_FIELD,
            Self::SEQUENCE_NUMBER_FIELD,
            Self::TOTAL_NETWORK_TRANSACTIONS_FIELD,
            Self::CONTENT_DIGEST_FIELD,
            Self::PREVIOUS_DIGEST_FIELD,
            Self::EPOCH_ROLLING_GAS_COST_SUMMARY_FIELD,
            Self::TIMESTAMP_FIELD,
            Self::COMMITMENTS_FIELD,
            Self::END_OF_EPOCH_DATA_FIELD,
            Self::VERSION_SPECIFIC_DATA_FIELD,
        ];
    }
    impl CheckpointSummary {
        pub fn path_builder() -> CheckpointSummaryFieldPathBuilder {
            CheckpointSummaryFieldPathBuilder::new()
        }
    }
    pub struct CheckpointSummaryFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CheckpointSummaryFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn bcs(mut self) -> BcsFieldPathBuilder {
            self.path.push(CheckpointSummary::BCS_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn digest(mut self) -> String {
            self.path.push(CheckpointSummary::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn epoch(mut self) -> String {
            self.path.push(CheckpointSummary::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn sequence_number(mut self) -> String {
            self.path.push(CheckpointSummary::SEQUENCE_NUMBER_FIELD.name);
            self.finish()
        }
        pub fn total_network_transactions(mut self) -> String {
            self.path.push(CheckpointSummary::TOTAL_NETWORK_TRANSACTIONS_FIELD.name);
            self.finish()
        }
        pub fn content_digest(mut self) -> String {
            self.path.push(CheckpointSummary::CONTENT_DIGEST_FIELD.name);
            self.finish()
        }
        pub fn previous_digest(mut self) -> String {
            self.path.push(CheckpointSummary::PREVIOUS_DIGEST_FIELD.name);
            self.finish()
        }
        pub fn epoch_rolling_gas_cost_summary(
            mut self,
        ) -> GasCostSummaryFieldPathBuilder {
            self.path.push(CheckpointSummary::EPOCH_ROLLING_GAS_COST_SUMMARY_FIELD.name);
            GasCostSummaryFieldPathBuilder::new_with_base(self.path)
        }
        pub fn timestamp(mut self) -> String {
            self.path.push(CheckpointSummary::TIMESTAMP_FIELD.name);
            self.finish()
        }
        pub fn commitments(mut self) -> CheckpointCommitmentFieldPathBuilder {
            self.path.push(CheckpointSummary::COMMITMENTS_FIELD.name);
            CheckpointCommitmentFieldPathBuilder::new_with_base(self.path)
        }
        pub fn end_of_epoch_data(mut self) -> EndOfEpochDataFieldPathBuilder {
            self.path.push(CheckpointSummary::END_OF_EPOCH_DATA_FIELD.name);
            EndOfEpochDataFieldPathBuilder::new_with_base(self.path)
        }
        pub fn version_specific_data(mut self) -> String {
            self.path.push(CheckpointSummary::VERSION_SPECIFIC_DATA_FIELD.name);
            self.finish()
        }
    }
    impl EndOfEpochData {
        pub const NEXT_EPOCH_COMMITTEE_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_committee",
            json_name: "nextEpochCommittee",
            number: 1i32,
            message_fields: Some(ValidatorCommitteeMember::FIELDS),
        };
        pub const NEXT_EPOCH_PROTOCOL_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_protocol_version",
            json_name: "nextEpochProtocolVersion",
            number: 2i32,
            message_fields: None,
        };
        pub const EPOCH_COMMITMENTS_FIELD: &'static MessageField = &MessageField {
            name: "epoch_commitments",
            json_name: "epochCommitments",
            number: 3i32,
            message_fields: Some(CheckpointCommitment::FIELDS),
        };
    }
    impl MessageFields for EndOfEpochData {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::NEXT_EPOCH_COMMITTEE_FIELD,
            Self::NEXT_EPOCH_PROTOCOL_VERSION_FIELD,
            Self::EPOCH_COMMITMENTS_FIELD,
        ];
    }
    impl EndOfEpochData {
        pub fn path_builder() -> EndOfEpochDataFieldPathBuilder {
            EndOfEpochDataFieldPathBuilder::new()
        }
    }
    pub struct EndOfEpochDataFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EndOfEpochDataFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn next_epoch_committee(
            mut self,
        ) -> ValidatorCommitteeMemberFieldPathBuilder {
            self.path.push(EndOfEpochData::NEXT_EPOCH_COMMITTEE_FIELD.name);
            ValidatorCommitteeMemberFieldPathBuilder::new_with_base(self.path)
        }
        pub fn next_epoch_protocol_version(mut self) -> String {
            self.path.push(EndOfEpochData::NEXT_EPOCH_PROTOCOL_VERSION_FIELD.name);
            self.finish()
        }
        pub fn epoch_commitments(mut self) -> CheckpointCommitmentFieldPathBuilder {
            self.path.push(EndOfEpochData::EPOCH_COMMITMENTS_FIELD.name);
            CheckpointCommitmentFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl CheckpointCommitment {
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 1i32,
            message_fields: None,
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for CheckpointCommitment {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::DIGEST_FIELD,
        ];
    }
    impl CheckpointCommitment {
        pub fn path_builder() -> CheckpointCommitmentFieldPathBuilder {
            CheckpointCommitmentFieldPathBuilder::new()
        }
    }
    pub struct CheckpointCommitmentFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CheckpointCommitmentFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kind(mut self) -> String {
            self.path.push(CheckpointCommitment::KIND_FIELD.name);
            self.finish()
        }
        pub fn digest(mut self) -> String {
            self.path.push(CheckpointCommitment::DIGEST_FIELD.name);
            self.finish()
        }
    }
    impl TransactionEffects {
        pub const BCS_FIELD: &'static MessageField = &MessageField {
            name: "bcs",
            json_name: "bcs",
            number: 1i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 2i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 3i32,
            message_fields: None,
        };
        pub const STATUS_FIELD: &'static MessageField = &MessageField {
            name: "status",
            json_name: "status",
            number: 4i32,
            message_fields: Some(ExecutionStatus::FIELDS),
        };
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 5i32,
            message_fields: None,
        };
        pub const GAS_USED_FIELD: &'static MessageField = &MessageField {
            name: "gas_used",
            json_name: "gasUsed",
            number: 6i32,
            message_fields: Some(GasCostSummary::FIELDS),
        };
        pub const TRANSACTION_DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "transaction_digest",
            json_name: "transactionDigest",
            number: 7i32,
            message_fields: None,
        };
        pub const GAS_OBJECT_FIELD: &'static MessageField = &MessageField {
            name: "gas_object",
            json_name: "gasObject",
            number: 8i32,
            message_fields: Some(ChangedObject::FIELDS),
        };
        pub const EVENTS_DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "events_digest",
            json_name: "eventsDigest",
            number: 9i32,
            message_fields: None,
        };
        pub const DEPENDENCIES_FIELD: &'static MessageField = &MessageField {
            name: "dependencies",
            json_name: "dependencies",
            number: 10i32,
            message_fields: None,
        };
        pub const LAMPORT_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "lamport_version",
            json_name: "lamportVersion",
            number: 11i32,
            message_fields: None,
        };
        pub const CHANGED_OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "changed_objects",
            json_name: "changedObjects",
            number: 12i32,
            message_fields: Some(ChangedObject::FIELDS),
        };
        pub const UNCHANGED_CONSENSUS_OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "unchanged_consensus_objects",
            json_name: "unchangedConsensusObjects",
            number: 13i32,
            message_fields: Some(UnchangedConsensusObject::FIELDS),
        };
        pub const AUXILIARY_DATA_DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "auxiliary_data_digest",
            json_name: "auxiliaryDataDigest",
            number: 14i32,
            message_fields: None,
        };
        pub const UNCHANGED_LOADED_RUNTIME_OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "unchanged_loaded_runtime_objects",
            json_name: "unchangedLoadedRuntimeObjects",
            number: 15i32,
            message_fields: Some(ObjectReference::FIELDS),
        };
    }
    impl MessageFields for TransactionEffects {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::BCS_FIELD,
            Self::DIGEST_FIELD,
            Self::VERSION_FIELD,
            Self::STATUS_FIELD,
            Self::EPOCH_FIELD,
            Self::GAS_USED_FIELD,
            Self::TRANSACTION_DIGEST_FIELD,
            Self::GAS_OBJECT_FIELD,
            Self::EVENTS_DIGEST_FIELD,
            Self::DEPENDENCIES_FIELD,
            Self::LAMPORT_VERSION_FIELD,
            Self::CHANGED_OBJECTS_FIELD,
            Self::UNCHANGED_CONSENSUS_OBJECTS_FIELD,
            Self::AUXILIARY_DATA_DIGEST_FIELD,
            Self::UNCHANGED_LOADED_RUNTIME_OBJECTS_FIELD,
        ];
    }
    impl TransactionEffects {
        pub fn path_builder() -> TransactionEffectsFieldPathBuilder {
            TransactionEffectsFieldPathBuilder::new()
        }
    }
    pub struct TransactionEffectsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TransactionEffectsFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn bcs(mut self) -> BcsFieldPathBuilder {
            self.path.push(TransactionEffects::BCS_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn digest(mut self) -> String {
            self.path.push(TransactionEffects::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(TransactionEffects::VERSION_FIELD.name);
            self.finish()
        }
        pub fn status(mut self) -> ExecutionStatusFieldPathBuilder {
            self.path.push(TransactionEffects::STATUS_FIELD.name);
            ExecutionStatusFieldPathBuilder::new_with_base(self.path)
        }
        pub fn epoch(mut self) -> String {
            self.path.push(TransactionEffects::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn gas_used(mut self) -> GasCostSummaryFieldPathBuilder {
            self.path.push(TransactionEffects::GAS_USED_FIELD.name);
            GasCostSummaryFieldPathBuilder::new_with_base(self.path)
        }
        pub fn transaction_digest(mut self) -> String {
            self.path.push(TransactionEffects::TRANSACTION_DIGEST_FIELD.name);
            self.finish()
        }
        pub fn gas_object(mut self) -> ChangedObjectFieldPathBuilder {
            self.path.push(TransactionEffects::GAS_OBJECT_FIELD.name);
            ChangedObjectFieldPathBuilder::new_with_base(self.path)
        }
        pub fn events_digest(mut self) -> String {
            self.path.push(TransactionEffects::EVENTS_DIGEST_FIELD.name);
            self.finish()
        }
        pub fn dependencies(mut self) -> String {
            self.path.push(TransactionEffects::DEPENDENCIES_FIELD.name);
            self.finish()
        }
        pub fn lamport_version(mut self) -> String {
            self.path.push(TransactionEffects::LAMPORT_VERSION_FIELD.name);
            self.finish()
        }
        pub fn changed_objects(mut self) -> ChangedObjectFieldPathBuilder {
            self.path.push(TransactionEffects::CHANGED_OBJECTS_FIELD.name);
            ChangedObjectFieldPathBuilder::new_with_base(self.path)
        }
        pub fn unchanged_consensus_objects(
            mut self,
        ) -> UnchangedConsensusObjectFieldPathBuilder {
            self.path.push(TransactionEffects::UNCHANGED_CONSENSUS_OBJECTS_FIELD.name);
            UnchangedConsensusObjectFieldPathBuilder::new_with_base(self.path)
        }
        pub fn auxiliary_data_digest(mut self) -> String {
            self.path.push(TransactionEffects::AUXILIARY_DATA_DIGEST_FIELD.name);
            self.finish()
        }
        pub fn unchanged_loaded_runtime_objects(
            mut self,
        ) -> ObjectReferenceFieldPathBuilder {
            self.path
                .push(TransactionEffects::UNCHANGED_LOADED_RUNTIME_OBJECTS_FIELD.name);
            ObjectReferenceFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ChangedObject {
        pub const OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "object_id",
            json_name: "objectId",
            number: 1i32,
            message_fields: None,
        };
        pub const INPUT_STATE_FIELD: &'static MessageField = &MessageField {
            name: "input_state",
            json_name: "inputState",
            number: 2i32,
            message_fields: None,
        };
        pub const INPUT_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "input_version",
            json_name: "inputVersion",
            number: 3i32,
            message_fields: None,
        };
        pub const INPUT_DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "input_digest",
            json_name: "inputDigest",
            number: 4i32,
            message_fields: None,
        };
        pub const INPUT_OWNER_FIELD: &'static MessageField = &MessageField {
            name: "input_owner",
            json_name: "inputOwner",
            number: 5i32,
            message_fields: Some(Owner::FIELDS),
        };
        pub const OUTPUT_STATE_FIELD: &'static MessageField = &MessageField {
            name: "output_state",
            json_name: "outputState",
            number: 6i32,
            message_fields: None,
        };
        pub const OUTPUT_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "output_version",
            json_name: "outputVersion",
            number: 7i32,
            message_fields: None,
        };
        pub const OUTPUT_DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "output_digest",
            json_name: "outputDigest",
            number: 8i32,
            message_fields: None,
        };
        pub const OUTPUT_OWNER_FIELD: &'static MessageField = &MessageField {
            name: "output_owner",
            json_name: "outputOwner",
            number: 9i32,
            message_fields: Some(Owner::FIELDS),
        };
        pub const ACCUMULATOR_WRITE_FIELD: &'static MessageField = &MessageField {
            name: "accumulator_write",
            json_name: "accumulatorWrite",
            number: 12i32,
            message_fields: Some(AccumulatorWrite::FIELDS),
        };
        pub const ID_OPERATION_FIELD: &'static MessageField = &MessageField {
            name: "id_operation",
            json_name: "idOperation",
            number: 10i32,
            message_fields: None,
        };
        pub const OBJECT_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "object_type",
            json_name: "objectType",
            number: 11i32,
            message_fields: None,
        };
    }
    impl MessageFields for ChangedObject {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECT_ID_FIELD,
            Self::INPUT_STATE_FIELD,
            Self::INPUT_VERSION_FIELD,
            Self::INPUT_DIGEST_FIELD,
            Self::INPUT_OWNER_FIELD,
            Self::OUTPUT_STATE_FIELD,
            Self::OUTPUT_VERSION_FIELD,
            Self::OUTPUT_DIGEST_FIELD,
            Self::OUTPUT_OWNER_FIELD,
            Self::ACCUMULATOR_WRITE_FIELD,
            Self::ID_OPERATION_FIELD,
            Self::OBJECT_TYPE_FIELD,
        ];
    }
    impl ChangedObject {
        pub fn path_builder() -> ChangedObjectFieldPathBuilder {
            ChangedObjectFieldPathBuilder::new()
        }
    }
    pub struct ChangedObjectFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ChangedObjectFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn object_id(mut self) -> String {
            self.path.push(ChangedObject::OBJECT_ID_FIELD.name);
            self.finish()
        }
        pub fn input_state(mut self) -> String {
            self.path.push(ChangedObject::INPUT_STATE_FIELD.name);
            self.finish()
        }
        pub fn input_version(mut self) -> String {
            self.path.push(ChangedObject::INPUT_VERSION_FIELD.name);
            self.finish()
        }
        pub fn input_digest(mut self) -> String {
            self.path.push(ChangedObject::INPUT_DIGEST_FIELD.name);
            self.finish()
        }
        pub fn input_owner(mut self) -> OwnerFieldPathBuilder {
            self.path.push(ChangedObject::INPUT_OWNER_FIELD.name);
            OwnerFieldPathBuilder::new_with_base(self.path)
        }
        pub fn output_state(mut self) -> String {
            self.path.push(ChangedObject::OUTPUT_STATE_FIELD.name);
            self.finish()
        }
        pub fn output_version(mut self) -> String {
            self.path.push(ChangedObject::OUTPUT_VERSION_FIELD.name);
            self.finish()
        }
        pub fn output_digest(mut self) -> String {
            self.path.push(ChangedObject::OUTPUT_DIGEST_FIELD.name);
            self.finish()
        }
        pub fn output_owner(mut self) -> OwnerFieldPathBuilder {
            self.path.push(ChangedObject::OUTPUT_OWNER_FIELD.name);
            OwnerFieldPathBuilder::new_with_base(self.path)
        }
        pub fn accumulator_write(mut self) -> AccumulatorWriteFieldPathBuilder {
            self.path.push(ChangedObject::ACCUMULATOR_WRITE_FIELD.name);
            AccumulatorWriteFieldPathBuilder::new_with_base(self.path)
        }
        pub fn id_operation(mut self) -> String {
            self.path.push(ChangedObject::ID_OPERATION_FIELD.name);
            self.finish()
        }
        pub fn object_type(mut self) -> String {
            self.path.push(ChangedObject::OBJECT_TYPE_FIELD.name);
            self.finish()
        }
    }
    impl AccumulatorWrite {
        pub const ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "address",
            json_name: "address",
            number: 1i32,
            message_fields: None,
        };
        pub const ACCUMULATOR_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "accumulator_type",
            json_name: "accumulatorType",
            number: 2i32,
            message_fields: None,
        };
        pub const OPERATION_FIELD: &'static MessageField = &MessageField {
            name: "operation",
            json_name: "operation",
            number: 3i32,
            message_fields: None,
        };
        pub const VALUE_FIELD: &'static MessageField = &MessageField {
            name: "value",
            json_name: "value",
            number: 5i32,
            message_fields: None,
        };
    }
    impl MessageFields for AccumulatorWrite {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ADDRESS_FIELD,
            Self::ACCUMULATOR_TYPE_FIELD,
            Self::OPERATION_FIELD,
            Self::VALUE_FIELD,
        ];
    }
    impl AccumulatorWrite {
        pub fn path_builder() -> AccumulatorWriteFieldPathBuilder {
            AccumulatorWriteFieldPathBuilder::new()
        }
    }
    pub struct AccumulatorWriteFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl AccumulatorWriteFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn address(mut self) -> String {
            self.path.push(AccumulatorWrite::ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn accumulator_type(mut self) -> String {
            self.path.push(AccumulatorWrite::ACCUMULATOR_TYPE_FIELD.name);
            self.finish()
        }
        pub fn operation(mut self) -> String {
            self.path.push(AccumulatorWrite::OPERATION_FIELD.name);
            self.finish()
        }
        pub fn value(mut self) -> String {
            self.path.push(AccumulatorWrite::VALUE_FIELD.name);
            self.finish()
        }
    }
    impl UnchangedConsensusObject {
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 1i32,
            message_fields: None,
        };
        pub const OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "object_id",
            json_name: "objectId",
            number: 2i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 3i32,
            message_fields: None,
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 4i32,
            message_fields: None,
        };
        pub const OBJECT_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "object_type",
            json_name: "objectType",
            number: 5i32,
            message_fields: None,
        };
    }
    impl MessageFields for UnchangedConsensusObject {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::OBJECT_ID_FIELD,
            Self::VERSION_FIELD,
            Self::DIGEST_FIELD,
            Self::OBJECT_TYPE_FIELD,
        ];
    }
    impl UnchangedConsensusObject {
        pub fn path_builder() -> UnchangedConsensusObjectFieldPathBuilder {
            UnchangedConsensusObjectFieldPathBuilder::new()
        }
    }
    pub struct UnchangedConsensusObjectFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl UnchangedConsensusObjectFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kind(mut self) -> String {
            self.path.push(UnchangedConsensusObject::KIND_FIELD.name);
            self.finish()
        }
        pub fn object_id(mut self) -> String {
            self.path.push(UnchangedConsensusObject::OBJECT_ID_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(UnchangedConsensusObject::VERSION_FIELD.name);
            self.finish()
        }
        pub fn digest(mut self) -> String {
            self.path.push(UnchangedConsensusObject::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn object_type(mut self) -> String {
            self.path.push(UnchangedConsensusObject::OBJECT_TYPE_FIELD.name);
            self.finish()
        }
    }
    impl Epoch {
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 1i32,
            message_fields: None,
        };
        pub const COMMITTEE_FIELD: &'static MessageField = &MessageField {
            name: "committee",
            json_name: "committee",
            number: 2i32,
            message_fields: Some(ValidatorCommittee::FIELDS),
        };
        pub const SYSTEM_STATE_FIELD: &'static MessageField = &MessageField {
            name: "system_state",
            json_name: "systemState",
            number: 3i32,
            message_fields: Some(SystemState::FIELDS),
        };
        pub const FIRST_CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "first_checkpoint",
            json_name: "firstCheckpoint",
            number: 4i32,
            message_fields: None,
        };
        pub const LAST_CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "last_checkpoint",
            json_name: "lastCheckpoint",
            number: 5i32,
            message_fields: None,
        };
        pub const START_FIELD: &'static MessageField = &MessageField {
            name: "start",
            json_name: "start",
            number: 6i32,
            message_fields: None,
        };
        pub const END_FIELD: &'static MessageField = &MessageField {
            name: "end",
            json_name: "end",
            number: 7i32,
            message_fields: None,
        };
        pub const REFERENCE_GAS_PRICE_FIELD: &'static MessageField = &MessageField {
            name: "reference_gas_price",
            json_name: "referenceGasPrice",
            number: 8i32,
            message_fields: None,
        };
        pub const PROTOCOL_CONFIG_FIELD: &'static MessageField = &MessageField {
            name: "protocol_config",
            json_name: "protocolConfig",
            number: 9i32,
            message_fields: Some(ProtocolConfig::FIELDS),
        };
    }
    impl MessageFields for Epoch {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EPOCH_FIELD,
            Self::COMMITTEE_FIELD,
            Self::SYSTEM_STATE_FIELD,
            Self::FIRST_CHECKPOINT_FIELD,
            Self::LAST_CHECKPOINT_FIELD,
            Self::START_FIELD,
            Self::END_FIELD,
            Self::REFERENCE_GAS_PRICE_FIELD,
            Self::PROTOCOL_CONFIG_FIELD,
        ];
    }
    impl Epoch {
        pub fn path_builder() -> EpochFieldPathBuilder {
            EpochFieldPathBuilder::new()
        }
    }
    pub struct EpochFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EpochFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn epoch(mut self) -> String {
            self.path.push(Epoch::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn committee(mut self) -> ValidatorCommitteeFieldPathBuilder {
            self.path.push(Epoch::COMMITTEE_FIELD.name);
            ValidatorCommitteeFieldPathBuilder::new_with_base(self.path)
        }
        pub fn system_state(mut self) -> SystemStateFieldPathBuilder {
            self.path.push(Epoch::SYSTEM_STATE_FIELD.name);
            SystemStateFieldPathBuilder::new_with_base(self.path)
        }
        pub fn first_checkpoint(mut self) -> String {
            self.path.push(Epoch::FIRST_CHECKPOINT_FIELD.name);
            self.finish()
        }
        pub fn last_checkpoint(mut self) -> String {
            self.path.push(Epoch::LAST_CHECKPOINT_FIELD.name);
            self.finish()
        }
        pub fn start(mut self) -> String {
            self.path.push(Epoch::START_FIELD.name);
            self.finish()
        }
        pub fn end(mut self) -> String {
            self.path.push(Epoch::END_FIELD.name);
            self.finish()
        }
        pub fn reference_gas_price(mut self) -> String {
            self.path.push(Epoch::REFERENCE_GAS_PRICE_FIELD.name);
            self.finish()
        }
        pub fn protocol_config(mut self) -> ProtocolConfigFieldPathBuilder {
            self.path.push(Epoch::PROTOCOL_CONFIG_FIELD.name);
            ProtocolConfigFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl TransactionEvents {
        pub const BCS_FIELD: &'static MessageField = &MessageField {
            name: "bcs",
            json_name: "bcs",
            number: 1i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 2i32,
            message_fields: None,
        };
        pub const EVENTS_FIELD: &'static MessageField = &MessageField {
            name: "events",
            json_name: "events",
            number: 3i32,
            message_fields: Some(Event::FIELDS),
        };
    }
    impl MessageFields for TransactionEvents {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::BCS_FIELD,
            Self::DIGEST_FIELD,
            Self::EVENTS_FIELD,
        ];
    }
    impl TransactionEvents {
        pub fn path_builder() -> TransactionEventsFieldPathBuilder {
            TransactionEventsFieldPathBuilder::new()
        }
    }
    pub struct TransactionEventsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TransactionEventsFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn bcs(mut self) -> BcsFieldPathBuilder {
            self.path.push(TransactionEvents::BCS_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn digest(mut self) -> String {
            self.path.push(TransactionEvents::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn events(mut self) -> EventFieldPathBuilder {
            self.path.push(TransactionEvents::EVENTS_FIELD.name);
            EventFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl Event {
        pub const PACKAGE_ID_FIELD: &'static MessageField = &MessageField {
            name: "package_id",
            json_name: "packageId",
            number: 1i32,
            message_fields: None,
        };
        pub const MODULE_FIELD: &'static MessageField = &MessageField {
            name: "module",
            json_name: "module",
            number: 2i32,
            message_fields: None,
        };
        pub const SENDER_FIELD: &'static MessageField = &MessageField {
            name: "sender",
            json_name: "sender",
            number: 3i32,
            message_fields: None,
        };
        pub const EVENT_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "event_type",
            json_name: "eventType",
            number: 4i32,
            message_fields: None,
        };
        pub const CONTENTS_FIELD: &'static MessageField = &MessageField {
            name: "contents",
            json_name: "contents",
            number: 5i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const JSON_FIELD: &'static MessageField = &MessageField {
            name: "json",
            json_name: "json",
            number: 6i32,
            message_fields: None,
        };
    }
    impl MessageFields for Event {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PACKAGE_ID_FIELD,
            Self::MODULE_FIELD,
            Self::SENDER_FIELD,
            Self::EVENT_TYPE_FIELD,
            Self::CONTENTS_FIELD,
            Self::JSON_FIELD,
        ];
    }
    impl Event {
        pub fn path_builder() -> EventFieldPathBuilder {
            EventFieldPathBuilder::new()
        }
    }
    pub struct EventFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EventFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn package_id(mut self) -> String {
            self.path.push(Event::PACKAGE_ID_FIELD.name);
            self.finish()
        }
        pub fn module(mut self) -> String {
            self.path.push(Event::MODULE_FIELD.name);
            self.finish()
        }
        pub fn sender(mut self) -> String {
            self.path.push(Event::SENDER_FIELD.name);
            self.finish()
        }
        pub fn event_type(mut self) -> String {
            self.path.push(Event::EVENT_TYPE_FIELD.name);
            self.finish()
        }
        pub fn contents(mut self) -> BcsFieldPathBuilder {
            self.path.push(Event::CONTENTS_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn json(mut self) -> String {
            self.path.push(Event::JSON_FIELD.name);
            self.finish()
        }
    }
    impl ExecutedTransaction {
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 1i32,
            message_fields: None,
        };
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 2i32,
            message_fields: Some(Transaction::FIELDS),
        };
        pub const SIGNATURES_FIELD: &'static MessageField = &MessageField {
            name: "signatures",
            json_name: "signatures",
            number: 3i32,
            message_fields: Some(UserSignature::FIELDS),
        };
        pub const EFFECTS_FIELD: &'static MessageField = &MessageField {
            name: "effects",
            json_name: "effects",
            number: 4i32,
            message_fields: Some(TransactionEffects::FIELDS),
        };
        pub const EVENTS_FIELD: &'static MessageField = &MessageField {
            name: "events",
            json_name: "events",
            number: 5i32,
            message_fields: Some(TransactionEvents::FIELDS),
        };
        pub const CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "checkpoint",
            json_name: "checkpoint",
            number: 6i32,
            message_fields: None,
        };
        pub const TIMESTAMP_FIELD: &'static MessageField = &MessageField {
            name: "timestamp",
            json_name: "timestamp",
            number: 7i32,
            message_fields: None,
        };
        pub const BALANCE_CHANGES_FIELD: &'static MessageField = &MessageField {
            name: "balance_changes",
            json_name: "balanceChanges",
            number: 8i32,
            message_fields: Some(BalanceChange::FIELDS),
        };
        pub const OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "objects",
            json_name: "objects",
            number: 9i32,
            message_fields: Some(ObjectSet::FIELDS),
        };
    }
    impl MessageFields for ExecutedTransaction {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::DIGEST_FIELD,
            Self::TRANSACTION_FIELD,
            Self::SIGNATURES_FIELD,
            Self::EFFECTS_FIELD,
            Self::EVENTS_FIELD,
            Self::CHECKPOINT_FIELD,
            Self::TIMESTAMP_FIELD,
            Self::BALANCE_CHANGES_FIELD,
            Self::OBJECTS_FIELD,
        ];
    }
    impl ExecutedTransaction {
        pub fn path_builder() -> ExecutedTransactionFieldPathBuilder {
            ExecutedTransactionFieldPathBuilder::new()
        }
    }
    pub struct ExecutedTransactionFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ExecutedTransactionFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn digest(mut self) -> String {
            self.path.push(ExecutedTransaction::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn transaction(mut self) -> TransactionFieldPathBuilder {
            self.path.push(ExecutedTransaction::TRANSACTION_FIELD.name);
            TransactionFieldPathBuilder::new_with_base(self.path)
        }
        pub fn signatures(mut self) -> UserSignatureFieldPathBuilder {
            self.path.push(ExecutedTransaction::SIGNATURES_FIELD.name);
            UserSignatureFieldPathBuilder::new_with_base(self.path)
        }
        pub fn effects(mut self) -> TransactionEffectsFieldPathBuilder {
            self.path.push(ExecutedTransaction::EFFECTS_FIELD.name);
            TransactionEffectsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn events(mut self) -> TransactionEventsFieldPathBuilder {
            self.path.push(ExecutedTransaction::EVENTS_FIELD.name);
            TransactionEventsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn checkpoint(mut self) -> String {
            self.path.push(ExecutedTransaction::CHECKPOINT_FIELD.name);
            self.finish()
        }
        pub fn timestamp(mut self) -> String {
            self.path.push(ExecutedTransaction::TIMESTAMP_FIELD.name);
            self.finish()
        }
        pub fn balance_changes(mut self) -> BalanceChangeFieldPathBuilder {
            self.path.push(ExecutedTransaction::BALANCE_CHANGES_FIELD.name);
            BalanceChangeFieldPathBuilder::new_with_base(self.path)
        }
        pub fn objects(mut self) -> ObjectSetFieldPathBuilder {
            self.path.push(ExecutedTransaction::OBJECTS_FIELD.name);
            ObjectSetFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ExecutionStatus {
        pub const SUCCESS_FIELD: &'static MessageField = &MessageField {
            name: "success",
            json_name: "success",
            number: 1i32,
            message_fields: None,
        };
        pub const ERROR_FIELD: &'static MessageField = &MessageField {
            name: "error",
            json_name: "error",
            number: 2i32,
            message_fields: Some(ExecutionError::FIELDS),
        };
    }
    impl MessageFields for ExecutionStatus {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::SUCCESS_FIELD,
            Self::ERROR_FIELD,
        ];
    }
    impl ExecutionStatus {
        pub fn path_builder() -> ExecutionStatusFieldPathBuilder {
            ExecutionStatusFieldPathBuilder::new()
        }
    }
    pub struct ExecutionStatusFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ExecutionStatusFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn success(mut self) -> String {
            self.path.push(ExecutionStatus::SUCCESS_FIELD.name);
            self.finish()
        }
        pub fn error(mut self) -> ExecutionErrorFieldPathBuilder {
            self.path.push(ExecutionStatus::ERROR_FIELD.name);
            ExecutionErrorFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ExecutionError {
        pub const DESCRIPTION_FIELD: &'static MessageField = &MessageField {
            name: "description",
            json_name: "description",
            number: 1i32,
            message_fields: None,
        };
        pub const COMMAND_FIELD: &'static MessageField = &MessageField {
            name: "command",
            json_name: "command",
            number: 2i32,
            message_fields: None,
        };
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 3i32,
            message_fields: None,
        };
        pub const ABORT_FIELD: &'static MessageField = &MessageField {
            name: "abort",
            json_name: "abort",
            number: 4i32,
            message_fields: Some(MoveAbort::FIELDS),
        };
        pub const SIZE_ERROR_FIELD: &'static MessageField = &MessageField {
            name: "size_error",
            json_name: "sizeError",
            number: 5i32,
            message_fields: Some(SizeError::FIELDS),
        };
        pub const COMMAND_ARGUMENT_ERROR_FIELD: &'static MessageField = &MessageField {
            name: "command_argument_error",
            json_name: "commandArgumentError",
            number: 6i32,
            message_fields: Some(CommandArgumentError::FIELDS),
        };
        pub const TYPE_ARGUMENT_ERROR_FIELD: &'static MessageField = &MessageField {
            name: "type_argument_error",
            json_name: "typeArgumentError",
            number: 7i32,
            message_fields: Some(TypeArgumentError::FIELDS),
        };
        pub const PACKAGE_UPGRADE_ERROR_FIELD: &'static MessageField = &MessageField {
            name: "package_upgrade_error",
            json_name: "packageUpgradeError",
            number: 8i32,
            message_fields: Some(PackageUpgradeError::FIELDS),
        };
        pub const INDEX_ERROR_FIELD: &'static MessageField = &MessageField {
            name: "index_error",
            json_name: "indexError",
            number: 9i32,
            message_fields: Some(IndexError::FIELDS),
        };
        pub const OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "object_id",
            json_name: "objectId",
            number: 10i32,
            message_fields: None,
        };
        pub const COIN_DENY_LIST_ERROR_FIELD: &'static MessageField = &MessageField {
            name: "coin_deny_list_error",
            json_name: "coinDenyListError",
            number: 11i32,
            message_fields: Some(CoinDenyListError::FIELDS),
        };
        pub const CONGESTED_OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "congested_objects",
            json_name: "congestedObjects",
            number: 12i32,
            message_fields: Some(CongestedObjects::FIELDS),
        };
    }
    impl MessageFields for ExecutionError {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::DESCRIPTION_FIELD,
            Self::COMMAND_FIELD,
            Self::KIND_FIELD,
            Self::ABORT_FIELD,
            Self::SIZE_ERROR_FIELD,
            Self::COMMAND_ARGUMENT_ERROR_FIELD,
            Self::TYPE_ARGUMENT_ERROR_FIELD,
            Self::PACKAGE_UPGRADE_ERROR_FIELD,
            Self::INDEX_ERROR_FIELD,
            Self::OBJECT_ID_FIELD,
            Self::COIN_DENY_LIST_ERROR_FIELD,
            Self::CONGESTED_OBJECTS_FIELD,
        ];
    }
    impl ExecutionError {
        pub fn path_builder() -> ExecutionErrorFieldPathBuilder {
            ExecutionErrorFieldPathBuilder::new()
        }
    }
    pub struct ExecutionErrorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ExecutionErrorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn description(mut self) -> String {
            self.path.push(ExecutionError::DESCRIPTION_FIELD.name);
            self.finish()
        }
        pub fn command(mut self) -> String {
            self.path.push(ExecutionError::COMMAND_FIELD.name);
            self.finish()
        }
        pub fn kind(mut self) -> String {
            self.path.push(ExecutionError::KIND_FIELD.name);
            self.finish()
        }
        pub fn abort(mut self) -> MoveAbortFieldPathBuilder {
            self.path.push(ExecutionError::ABORT_FIELD.name);
            MoveAbortFieldPathBuilder::new_with_base(self.path)
        }
        pub fn size_error(mut self) -> SizeErrorFieldPathBuilder {
            self.path.push(ExecutionError::SIZE_ERROR_FIELD.name);
            SizeErrorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn command_argument_error(mut self) -> CommandArgumentErrorFieldPathBuilder {
            self.path.push(ExecutionError::COMMAND_ARGUMENT_ERROR_FIELD.name);
            CommandArgumentErrorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn type_argument_error(mut self) -> TypeArgumentErrorFieldPathBuilder {
            self.path.push(ExecutionError::TYPE_ARGUMENT_ERROR_FIELD.name);
            TypeArgumentErrorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn package_upgrade_error(mut self) -> PackageUpgradeErrorFieldPathBuilder {
            self.path.push(ExecutionError::PACKAGE_UPGRADE_ERROR_FIELD.name);
            PackageUpgradeErrorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn index_error(mut self) -> IndexErrorFieldPathBuilder {
            self.path.push(ExecutionError::INDEX_ERROR_FIELD.name);
            IndexErrorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn object_id(mut self) -> String {
            self.path.push(ExecutionError::OBJECT_ID_FIELD.name);
            self.finish()
        }
        pub fn coin_deny_list_error(mut self) -> CoinDenyListErrorFieldPathBuilder {
            self.path.push(ExecutionError::COIN_DENY_LIST_ERROR_FIELD.name);
            CoinDenyListErrorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn congested_objects(mut self) -> CongestedObjectsFieldPathBuilder {
            self.path.push(ExecutionError::CONGESTED_OBJECTS_FIELD.name);
            CongestedObjectsFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MoveAbort {
        pub const ABORT_CODE_FIELD: &'static MessageField = &MessageField {
            name: "abort_code",
            json_name: "abortCode",
            number: 1i32,
            message_fields: None,
        };
        pub const LOCATION_FIELD: &'static MessageField = &MessageField {
            name: "location",
            json_name: "location",
            number: 2i32,
            message_fields: Some(MoveLocation::FIELDS),
        };
        pub const CLEVER_ERROR_FIELD: &'static MessageField = &MessageField {
            name: "clever_error",
            json_name: "cleverError",
            number: 3i32,
            message_fields: Some(CleverError::FIELDS),
        };
    }
    impl MessageFields for MoveAbort {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ABORT_CODE_FIELD,
            Self::LOCATION_FIELD,
            Self::CLEVER_ERROR_FIELD,
        ];
    }
    impl MoveAbort {
        pub fn path_builder() -> MoveAbortFieldPathBuilder {
            MoveAbortFieldPathBuilder::new()
        }
    }
    pub struct MoveAbortFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MoveAbortFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn abort_code(mut self) -> String {
            self.path.push(MoveAbort::ABORT_CODE_FIELD.name);
            self.finish()
        }
        pub fn location(mut self) -> MoveLocationFieldPathBuilder {
            self.path.push(MoveAbort::LOCATION_FIELD.name);
            MoveLocationFieldPathBuilder::new_with_base(self.path)
        }
        pub fn clever_error(mut self) -> CleverErrorFieldPathBuilder {
            self.path.push(MoveAbort::CLEVER_ERROR_FIELD.name);
            CleverErrorFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MoveLocation {
        pub const PACKAGE_FIELD: &'static MessageField = &MessageField {
            name: "package",
            json_name: "package",
            number: 1i32,
            message_fields: None,
        };
        pub const MODULE_FIELD: &'static MessageField = &MessageField {
            name: "module",
            json_name: "module",
            number: 2i32,
            message_fields: None,
        };
        pub const FUNCTION_FIELD: &'static MessageField = &MessageField {
            name: "function",
            json_name: "function",
            number: 3i32,
            message_fields: None,
        };
        pub const INSTRUCTION_FIELD: &'static MessageField = &MessageField {
            name: "instruction",
            json_name: "instruction",
            number: 4i32,
            message_fields: None,
        };
        pub const FUNCTION_NAME_FIELD: &'static MessageField = &MessageField {
            name: "function_name",
            json_name: "functionName",
            number: 5i32,
            message_fields: None,
        };
    }
    impl MessageFields for MoveLocation {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PACKAGE_FIELD,
            Self::MODULE_FIELD,
            Self::FUNCTION_FIELD,
            Self::INSTRUCTION_FIELD,
            Self::FUNCTION_NAME_FIELD,
        ];
    }
    impl MoveLocation {
        pub fn path_builder() -> MoveLocationFieldPathBuilder {
            MoveLocationFieldPathBuilder::new()
        }
    }
    pub struct MoveLocationFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MoveLocationFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn package(mut self) -> String {
            self.path.push(MoveLocation::PACKAGE_FIELD.name);
            self.finish()
        }
        pub fn module(mut self) -> String {
            self.path.push(MoveLocation::MODULE_FIELD.name);
            self.finish()
        }
        pub fn function(mut self) -> String {
            self.path.push(MoveLocation::FUNCTION_FIELD.name);
            self.finish()
        }
        pub fn instruction(mut self) -> String {
            self.path.push(MoveLocation::INSTRUCTION_FIELD.name);
            self.finish()
        }
        pub fn function_name(mut self) -> String {
            self.path.push(MoveLocation::FUNCTION_NAME_FIELD.name);
            self.finish()
        }
    }
    impl CleverError {
        pub const ERROR_CODE_FIELD: &'static MessageField = &MessageField {
            name: "error_code",
            json_name: "errorCode",
            number: 1i32,
            message_fields: None,
        };
        pub const LINE_NUMBER_FIELD: &'static MessageField = &MessageField {
            name: "line_number",
            json_name: "lineNumber",
            number: 2i32,
            message_fields: None,
        };
        pub const CONSTANT_NAME_FIELD: &'static MessageField = &MessageField {
            name: "constant_name",
            json_name: "constantName",
            number: 3i32,
            message_fields: None,
        };
        pub const CONSTANT_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "constant_type",
            json_name: "constantType",
            number: 4i32,
            message_fields: None,
        };
        pub const RENDERED_FIELD: &'static MessageField = &MessageField {
            name: "rendered",
            json_name: "rendered",
            number: 5i32,
            message_fields: None,
        };
        pub const RAW_FIELD: &'static MessageField = &MessageField {
            name: "raw",
            json_name: "raw",
            number: 6i32,
            message_fields: None,
        };
    }
    impl MessageFields for CleverError {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ERROR_CODE_FIELD,
            Self::LINE_NUMBER_FIELD,
            Self::CONSTANT_NAME_FIELD,
            Self::CONSTANT_TYPE_FIELD,
            Self::RENDERED_FIELD,
            Self::RAW_FIELD,
        ];
    }
    impl CleverError {
        pub fn path_builder() -> CleverErrorFieldPathBuilder {
            CleverErrorFieldPathBuilder::new()
        }
    }
    pub struct CleverErrorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CleverErrorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn error_code(mut self) -> String {
            self.path.push(CleverError::ERROR_CODE_FIELD.name);
            self.finish()
        }
        pub fn line_number(mut self) -> String {
            self.path.push(CleverError::LINE_NUMBER_FIELD.name);
            self.finish()
        }
        pub fn constant_name(mut self) -> String {
            self.path.push(CleverError::CONSTANT_NAME_FIELD.name);
            self.finish()
        }
        pub fn constant_type(mut self) -> String {
            self.path.push(CleverError::CONSTANT_TYPE_FIELD.name);
            self.finish()
        }
        pub fn rendered(mut self) -> String {
            self.path.push(CleverError::RENDERED_FIELD.name);
            self.finish()
        }
        pub fn raw(mut self) -> String {
            self.path.push(CleverError::RAW_FIELD.name);
            self.finish()
        }
    }
    impl SizeError {
        pub const SIZE_FIELD: &'static MessageField = &MessageField {
            name: "size",
            json_name: "size",
            number: 1i32,
            message_fields: None,
        };
        pub const MAX_SIZE_FIELD: &'static MessageField = &MessageField {
            name: "max_size",
            json_name: "maxSize",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for SizeError {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::SIZE_FIELD,
            Self::MAX_SIZE_FIELD,
        ];
    }
    impl SizeError {
        pub fn path_builder() -> SizeErrorFieldPathBuilder {
            SizeErrorFieldPathBuilder::new()
        }
    }
    pub struct SizeErrorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SizeErrorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn size(mut self) -> String {
            self.path.push(SizeError::SIZE_FIELD.name);
            self.finish()
        }
        pub fn max_size(mut self) -> String {
            self.path.push(SizeError::MAX_SIZE_FIELD.name);
            self.finish()
        }
    }
    impl IndexError {
        pub const INDEX_FIELD: &'static MessageField = &MessageField {
            name: "index",
            json_name: "index",
            number: 1i32,
            message_fields: None,
        };
        pub const SUBRESULT_FIELD: &'static MessageField = &MessageField {
            name: "subresult",
            json_name: "subresult",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for IndexError {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::INDEX_FIELD,
            Self::SUBRESULT_FIELD,
        ];
    }
    impl IndexError {
        pub fn path_builder() -> IndexErrorFieldPathBuilder {
            IndexErrorFieldPathBuilder::new()
        }
    }
    pub struct IndexErrorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl IndexErrorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn index(mut self) -> String {
            self.path.push(IndexError::INDEX_FIELD.name);
            self.finish()
        }
        pub fn subresult(mut self) -> String {
            self.path.push(IndexError::SUBRESULT_FIELD.name);
            self.finish()
        }
    }
    impl CoinDenyListError {
        pub const ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "address",
            json_name: "address",
            number: 1i32,
            message_fields: None,
        };
        pub const COIN_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "coin_type",
            json_name: "coinType",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for CoinDenyListError {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ADDRESS_FIELD,
            Self::COIN_TYPE_FIELD,
        ];
    }
    impl CoinDenyListError {
        pub fn path_builder() -> CoinDenyListErrorFieldPathBuilder {
            CoinDenyListErrorFieldPathBuilder::new()
        }
    }
    pub struct CoinDenyListErrorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CoinDenyListErrorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn address(mut self) -> String {
            self.path.push(CoinDenyListError::ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn coin_type(mut self) -> String {
            self.path.push(CoinDenyListError::COIN_TYPE_FIELD.name);
            self.finish()
        }
    }
    impl CongestedObjects {
        pub const OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "objects",
            json_name: "objects",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for CongestedObjects {
        const FIELDS: &'static [&'static MessageField] = &[Self::OBJECTS_FIELD];
    }
    impl CongestedObjects {
        pub fn path_builder() -> CongestedObjectsFieldPathBuilder {
            CongestedObjectsFieldPathBuilder::new()
        }
    }
    pub struct CongestedObjectsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CongestedObjectsFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn objects(mut self) -> String {
            self.path.push(CongestedObjects::OBJECTS_FIELD.name);
            self.finish()
        }
    }
    impl CommandArgumentError {
        pub const ARGUMENT_FIELD: &'static MessageField = &MessageField {
            name: "argument",
            json_name: "argument",
            number: 1i32,
            message_fields: None,
        };
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 2i32,
            message_fields: None,
        };
        pub const INDEX_ERROR_FIELD: &'static MessageField = &MessageField {
            name: "index_error",
            json_name: "indexError",
            number: 3i32,
            message_fields: Some(IndexError::FIELDS),
        };
    }
    impl MessageFields for CommandArgumentError {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ARGUMENT_FIELD,
            Self::KIND_FIELD,
            Self::INDEX_ERROR_FIELD,
        ];
    }
    impl CommandArgumentError {
        pub fn path_builder() -> CommandArgumentErrorFieldPathBuilder {
            CommandArgumentErrorFieldPathBuilder::new()
        }
    }
    pub struct CommandArgumentErrorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CommandArgumentErrorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn argument(mut self) -> String {
            self.path.push(CommandArgumentError::ARGUMENT_FIELD.name);
            self.finish()
        }
        pub fn kind(mut self) -> String {
            self.path.push(CommandArgumentError::KIND_FIELD.name);
            self.finish()
        }
        pub fn index_error(mut self) -> IndexErrorFieldPathBuilder {
            self.path.push(CommandArgumentError::INDEX_ERROR_FIELD.name);
            IndexErrorFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl PackageUpgradeError {
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 1i32,
            message_fields: None,
        };
        pub const PACKAGE_ID_FIELD: &'static MessageField = &MessageField {
            name: "package_id",
            json_name: "packageId",
            number: 2i32,
            message_fields: None,
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 3i32,
            message_fields: None,
        };
        pub const POLICY_FIELD: &'static MessageField = &MessageField {
            name: "policy",
            json_name: "policy",
            number: 4i32,
            message_fields: None,
        };
        pub const TICKET_ID_FIELD: &'static MessageField = &MessageField {
            name: "ticket_id",
            json_name: "ticketId",
            number: 5i32,
            message_fields: None,
        };
    }
    impl MessageFields for PackageUpgradeError {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::PACKAGE_ID_FIELD,
            Self::DIGEST_FIELD,
            Self::POLICY_FIELD,
            Self::TICKET_ID_FIELD,
        ];
    }
    impl PackageUpgradeError {
        pub fn path_builder() -> PackageUpgradeErrorFieldPathBuilder {
            PackageUpgradeErrorFieldPathBuilder::new()
        }
    }
    pub struct PackageUpgradeErrorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl PackageUpgradeErrorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kind(mut self) -> String {
            self.path.push(PackageUpgradeError::KIND_FIELD.name);
            self.finish()
        }
        pub fn package_id(mut self) -> String {
            self.path.push(PackageUpgradeError::PACKAGE_ID_FIELD.name);
            self.finish()
        }
        pub fn digest(mut self) -> String {
            self.path.push(PackageUpgradeError::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn policy(mut self) -> String {
            self.path.push(PackageUpgradeError::POLICY_FIELD.name);
            self.finish()
        }
        pub fn ticket_id(mut self) -> String {
            self.path.push(PackageUpgradeError::TICKET_ID_FIELD.name);
            self.finish()
        }
    }
    impl TypeArgumentError {
        pub const TYPE_ARGUMENT_FIELD: &'static MessageField = &MessageField {
            name: "type_argument",
            json_name: "typeArgument",
            number: 1i32,
            message_fields: None,
        };
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for TypeArgumentError {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TYPE_ARGUMENT_FIELD,
            Self::KIND_FIELD,
        ];
    }
    impl TypeArgumentError {
        pub fn path_builder() -> TypeArgumentErrorFieldPathBuilder {
            TypeArgumentErrorFieldPathBuilder::new()
        }
    }
    pub struct TypeArgumentErrorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TypeArgumentErrorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn type_argument(mut self) -> String {
            self.path.push(TypeArgumentError::TYPE_ARGUMENT_FIELD.name);
            self.finish()
        }
        pub fn kind(mut self) -> String {
            self.path.push(TypeArgumentError::KIND_FIELD.name);
            self.finish()
        }
    }
    impl GasCostSummary {
        pub const COMPUTATION_COST_FIELD: &'static MessageField = &MessageField {
            name: "computation_cost",
            json_name: "computationCost",
            number: 1i32,
            message_fields: None,
        };
        pub const STORAGE_COST_FIELD: &'static MessageField = &MessageField {
            name: "storage_cost",
            json_name: "storageCost",
            number: 2i32,
            message_fields: None,
        };
        pub const STORAGE_REBATE_FIELD: &'static MessageField = &MessageField {
            name: "storage_rebate",
            json_name: "storageRebate",
            number: 3i32,
            message_fields: None,
        };
        pub const NON_REFUNDABLE_STORAGE_FEE_FIELD: &'static MessageField = &MessageField {
            name: "non_refundable_storage_fee",
            json_name: "nonRefundableStorageFee",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for GasCostSummary {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::COMPUTATION_COST_FIELD,
            Self::STORAGE_COST_FIELD,
            Self::STORAGE_REBATE_FIELD,
            Self::NON_REFUNDABLE_STORAGE_FEE_FIELD,
        ];
    }
    impl GasCostSummary {
        pub fn path_builder() -> GasCostSummaryFieldPathBuilder {
            GasCostSummaryFieldPathBuilder::new()
        }
    }
    pub struct GasCostSummaryFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GasCostSummaryFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn computation_cost(mut self) -> String {
            self.path.push(GasCostSummary::COMPUTATION_COST_FIELD.name);
            self.finish()
        }
        pub fn storage_cost(mut self) -> String {
            self.path.push(GasCostSummary::STORAGE_COST_FIELD.name);
            self.finish()
        }
        pub fn storage_rebate(mut self) -> String {
            self.path.push(GasCostSummary::STORAGE_REBATE_FIELD.name);
            self.finish()
        }
        pub fn non_refundable_storage_fee(mut self) -> String {
            self.path.push(GasCostSummary::NON_REFUNDABLE_STORAGE_FEE_FIELD.name);
            self.finish()
        }
    }
    impl Input {
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 1i32,
            message_fields: None,
        };
        pub const PURE_FIELD: &'static MessageField = &MessageField {
            name: "pure",
            json_name: "pure",
            number: 2i32,
            message_fields: None,
        };
        pub const OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "object_id",
            json_name: "objectId",
            number: 3i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 4i32,
            message_fields: None,
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 5i32,
            message_fields: None,
        };
        pub const MUTABLE_FIELD: &'static MessageField = &MessageField {
            name: "mutable",
            json_name: "mutable",
            number: 6i32,
            message_fields: None,
        };
        pub const MUTABILITY_FIELD: &'static MessageField = &MessageField {
            name: "mutability",
            json_name: "mutability",
            number: 7i32,
            message_fields: None,
        };
        pub const FUNDS_WITHDRAWAL_FIELD: &'static MessageField = &MessageField {
            name: "funds_withdrawal",
            json_name: "fundsWithdrawal",
            number: 8i32,
            message_fields: Some(FundsWithdrawal::FIELDS),
        };
        pub const LITERAL_FIELD: &'static MessageField = &MessageField {
            name: "literal",
            json_name: "literal",
            number: 1000i32,
            message_fields: None,
        };
    }
    impl MessageFields for Input {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::PURE_FIELD,
            Self::OBJECT_ID_FIELD,
            Self::VERSION_FIELD,
            Self::DIGEST_FIELD,
            Self::MUTABLE_FIELD,
            Self::MUTABILITY_FIELD,
            Self::FUNDS_WITHDRAWAL_FIELD,
            Self::LITERAL_FIELD,
        ];
    }
    impl Input {
        pub fn path_builder() -> InputFieldPathBuilder {
            InputFieldPathBuilder::new()
        }
    }
    pub struct InputFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl InputFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kind(mut self) -> String {
            self.path.push(Input::KIND_FIELD.name);
            self.finish()
        }
        pub fn pure(mut self) -> String {
            self.path.push(Input::PURE_FIELD.name);
            self.finish()
        }
        pub fn object_id(mut self) -> String {
            self.path.push(Input::OBJECT_ID_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(Input::VERSION_FIELD.name);
            self.finish()
        }
        pub fn digest(mut self) -> String {
            self.path.push(Input::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn mutable(mut self) -> String {
            self.path.push(Input::MUTABLE_FIELD.name);
            self.finish()
        }
        pub fn mutability(mut self) -> String {
            self.path.push(Input::MUTABILITY_FIELD.name);
            self.finish()
        }
        pub fn funds_withdrawal(mut self) -> FundsWithdrawalFieldPathBuilder {
            self.path.push(Input::FUNDS_WITHDRAWAL_FIELD.name);
            FundsWithdrawalFieldPathBuilder::new_with_base(self.path)
        }
        pub fn literal(mut self) -> String {
            self.path.push(Input::LITERAL_FIELD.name);
            self.finish()
        }
    }
    impl FundsWithdrawal {
        pub const AMOUNT_FIELD: &'static MessageField = &MessageField {
            name: "amount",
            json_name: "amount",
            number: 1i32,
            message_fields: None,
        };
        pub const COIN_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "coin_type",
            json_name: "coinType",
            number: 2i32,
            message_fields: None,
        };
        pub const SOURCE_FIELD: &'static MessageField = &MessageField {
            name: "source",
            json_name: "source",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for FundsWithdrawal {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::AMOUNT_FIELD,
            Self::COIN_TYPE_FIELD,
            Self::SOURCE_FIELD,
        ];
    }
    impl FundsWithdrawal {
        pub fn path_builder() -> FundsWithdrawalFieldPathBuilder {
            FundsWithdrawalFieldPathBuilder::new()
        }
    }
    pub struct FundsWithdrawalFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl FundsWithdrawalFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn amount(mut self) -> String {
            self.path.push(FundsWithdrawal::AMOUNT_FIELD.name);
            self.finish()
        }
        pub fn coin_type(mut self) -> String {
            self.path.push(FundsWithdrawal::COIN_TYPE_FIELD.name);
            self.finish()
        }
        pub fn source(mut self) -> String {
            self.path.push(FundsWithdrawal::SOURCE_FIELD.name);
            self.finish()
        }
    }
    impl JwkId {
        pub const ISS_FIELD: &'static MessageField = &MessageField {
            name: "iss",
            json_name: "iss",
            number: 1i32,
            message_fields: None,
        };
        pub const KID_FIELD: &'static MessageField = &MessageField {
            name: "kid",
            json_name: "kid",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for JwkId {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ISS_FIELD,
            Self::KID_FIELD,
        ];
    }
    impl JwkId {
        pub fn path_builder() -> JwkIdFieldPathBuilder {
            JwkIdFieldPathBuilder::new()
        }
    }
    pub struct JwkIdFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl JwkIdFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn iss(mut self) -> String {
            self.path.push(JwkId::ISS_FIELD.name);
            self.finish()
        }
        pub fn kid(mut self) -> String {
            self.path.push(JwkId::KID_FIELD.name);
            self.finish()
        }
    }
    impl Jwk {
        pub const KTY_FIELD: &'static MessageField = &MessageField {
            name: "kty",
            json_name: "kty",
            number: 1i32,
            message_fields: None,
        };
        pub const E_FIELD: &'static MessageField = &MessageField {
            name: "e",
            json_name: "e",
            number: 2i32,
            message_fields: None,
        };
        pub const N_FIELD: &'static MessageField = &MessageField {
            name: "n",
            json_name: "n",
            number: 3i32,
            message_fields: None,
        };
        pub const ALG_FIELD: &'static MessageField = &MessageField {
            name: "alg",
            json_name: "alg",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for Jwk {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KTY_FIELD,
            Self::E_FIELD,
            Self::N_FIELD,
            Self::ALG_FIELD,
        ];
    }
    impl Jwk {
        pub fn path_builder() -> JwkFieldPathBuilder {
            JwkFieldPathBuilder::new()
        }
    }
    pub struct JwkFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl JwkFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kty(mut self) -> String {
            self.path.push(Jwk::KTY_FIELD.name);
            self.finish()
        }
        pub fn e(mut self) -> String {
            self.path.push(Jwk::E_FIELD.name);
            self.finish()
        }
        pub fn n(mut self) -> String {
            self.path.push(Jwk::N_FIELD.name);
            self.finish()
        }
        pub fn alg(mut self) -> String {
            self.path.push(Jwk::ALG_FIELD.name);
            self.finish()
        }
    }
    impl GetServiceInfoRequest {}
    impl MessageFields for GetServiceInfoRequest {
        const FIELDS: &'static [&'static MessageField] = &[];
    }
    impl GetServiceInfoRequest {
        pub fn path_builder() -> GetServiceInfoRequestFieldPathBuilder {
            GetServiceInfoRequestFieldPathBuilder::new()
        }
    }
    pub struct GetServiceInfoRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetServiceInfoRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
    }
    impl GetServiceInfoResponse {
        pub const CHAIN_ID_FIELD: &'static MessageField = &MessageField {
            name: "chain_id",
            json_name: "chainId",
            number: 1i32,
            message_fields: None,
        };
        pub const CHAIN_FIELD: &'static MessageField = &MessageField {
            name: "chain",
            json_name: "chain",
            number: 2i32,
            message_fields: None,
        };
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 3i32,
            message_fields: None,
        };
        pub const CHECKPOINT_HEIGHT_FIELD: &'static MessageField = &MessageField {
            name: "checkpoint_height",
            json_name: "checkpointHeight",
            number: 4i32,
            message_fields: None,
        };
        pub const TIMESTAMP_FIELD: &'static MessageField = &MessageField {
            name: "timestamp",
            json_name: "timestamp",
            number: 5i32,
            message_fields: None,
        };
        pub const LOWEST_AVAILABLE_CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "lowest_available_checkpoint",
            json_name: "lowestAvailableCheckpoint",
            number: 6i32,
            message_fields: None,
        };
        pub const LOWEST_AVAILABLE_CHECKPOINT_OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "lowest_available_checkpoint_objects",
            json_name: "lowestAvailableCheckpointObjects",
            number: 7i32,
            message_fields: None,
        };
        pub const SERVER_FIELD: &'static MessageField = &MessageField {
            name: "server",
            json_name: "server",
            number: 8i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetServiceInfoResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CHAIN_ID_FIELD,
            Self::CHAIN_FIELD,
            Self::EPOCH_FIELD,
            Self::CHECKPOINT_HEIGHT_FIELD,
            Self::TIMESTAMP_FIELD,
            Self::LOWEST_AVAILABLE_CHECKPOINT_FIELD,
            Self::LOWEST_AVAILABLE_CHECKPOINT_OBJECTS_FIELD,
            Self::SERVER_FIELD,
        ];
    }
    impl GetServiceInfoResponse {
        pub fn path_builder() -> GetServiceInfoResponseFieldPathBuilder {
            GetServiceInfoResponseFieldPathBuilder::new()
        }
    }
    pub struct GetServiceInfoResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetServiceInfoResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn chain_id(mut self) -> String {
            self.path.push(GetServiceInfoResponse::CHAIN_ID_FIELD.name);
            self.finish()
        }
        pub fn chain(mut self) -> String {
            self.path.push(GetServiceInfoResponse::CHAIN_FIELD.name);
            self.finish()
        }
        pub fn epoch(mut self) -> String {
            self.path.push(GetServiceInfoResponse::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn checkpoint_height(mut self) -> String {
            self.path.push(GetServiceInfoResponse::CHECKPOINT_HEIGHT_FIELD.name);
            self.finish()
        }
        pub fn timestamp(mut self) -> String {
            self.path.push(GetServiceInfoResponse::TIMESTAMP_FIELD.name);
            self.finish()
        }
        pub fn lowest_available_checkpoint(mut self) -> String {
            self.path
                .push(GetServiceInfoResponse::LOWEST_AVAILABLE_CHECKPOINT_FIELD.name);
            self.finish()
        }
        pub fn lowest_available_checkpoint_objects(mut self) -> String {
            self.path
                .push(
                    GetServiceInfoResponse::LOWEST_AVAILABLE_CHECKPOINT_OBJECTS_FIELD
                        .name,
                );
            self.finish()
        }
        pub fn server(mut self) -> String {
            self.path.push(GetServiceInfoResponse::SERVER_FIELD.name);
            self.finish()
        }
    }
    impl GetObjectRequest {
        pub const OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "object_id",
            json_name: "objectId",
            number: 1i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 2i32,
            message_fields: None,
        };
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetObjectRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECT_ID_FIELD,
            Self::VERSION_FIELD,
            Self::READ_MASK_FIELD,
        ];
    }
    impl GetObjectRequest {
        pub fn path_builder() -> GetObjectRequestFieldPathBuilder {
            GetObjectRequestFieldPathBuilder::new()
        }
    }
    pub struct GetObjectRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetObjectRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn object_id(mut self) -> String {
            self.path.push(GetObjectRequest::OBJECT_ID_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(GetObjectRequest::VERSION_FIELD.name);
            self.finish()
        }
        pub fn read_mask(mut self) -> String {
            self.path.push(GetObjectRequest::READ_MASK_FIELD.name);
            self.finish()
        }
    }
    impl GetObjectResponse {
        pub const OBJECT_FIELD: &'static MessageField = &MessageField {
            name: "object",
            json_name: "object",
            number: 1i32,
            message_fields: Some(Object::FIELDS),
        };
    }
    impl MessageFields for GetObjectResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::OBJECT_FIELD];
    }
    impl GetObjectResponse {
        pub fn path_builder() -> GetObjectResponseFieldPathBuilder {
            GetObjectResponseFieldPathBuilder::new()
        }
    }
    pub struct GetObjectResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetObjectResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn object(mut self) -> ObjectFieldPathBuilder {
            self.path.push(GetObjectResponse::OBJECT_FIELD.name);
            ObjectFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl BatchGetObjectsRequest {
        pub const REQUESTS_FIELD: &'static MessageField = &MessageField {
            name: "requests",
            json_name: "requests",
            number: 1i32,
            message_fields: Some(GetObjectRequest::FIELDS),
        };
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for BatchGetObjectsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::REQUESTS_FIELD,
            Self::READ_MASK_FIELD,
        ];
    }
    impl BatchGetObjectsRequest {
        pub fn path_builder() -> BatchGetObjectsRequestFieldPathBuilder {
            BatchGetObjectsRequestFieldPathBuilder::new()
        }
    }
    pub struct BatchGetObjectsRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl BatchGetObjectsRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn requests(mut self) -> GetObjectRequestFieldPathBuilder {
            self.path.push(BatchGetObjectsRequest::REQUESTS_FIELD.name);
            GetObjectRequestFieldPathBuilder::new_with_base(self.path)
        }
        pub fn read_mask(mut self) -> String {
            self.path.push(BatchGetObjectsRequest::READ_MASK_FIELD.name);
            self.finish()
        }
    }
    impl BatchGetObjectsResponse {
        pub const OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "objects",
            json_name: "objects",
            number: 1i32,
            message_fields: Some(GetObjectResult::FIELDS),
        };
    }
    impl MessageFields for BatchGetObjectsResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::OBJECTS_FIELD];
    }
    impl BatchGetObjectsResponse {
        pub fn path_builder() -> BatchGetObjectsResponseFieldPathBuilder {
            BatchGetObjectsResponseFieldPathBuilder::new()
        }
    }
    pub struct BatchGetObjectsResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl BatchGetObjectsResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn objects(mut self) -> GetObjectResultFieldPathBuilder {
            self.path.push(BatchGetObjectsResponse::OBJECTS_FIELD.name);
            GetObjectResultFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl GetObjectResult {
        pub const OBJECT_FIELD: &'static MessageField = &MessageField {
            name: "object",
            json_name: "object",
            number: 1i32,
            message_fields: Some(Object::FIELDS),
        };
        pub const ERROR_FIELD: &'static MessageField = &MessageField {
            name: "error",
            json_name: "error",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetObjectResult {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECT_FIELD,
            Self::ERROR_FIELD,
        ];
    }
    impl GetObjectResult {
        pub fn path_builder() -> GetObjectResultFieldPathBuilder {
            GetObjectResultFieldPathBuilder::new()
        }
    }
    pub struct GetObjectResultFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetObjectResultFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn object(mut self) -> ObjectFieldPathBuilder {
            self.path.push(GetObjectResult::OBJECT_FIELD.name);
            ObjectFieldPathBuilder::new_with_base(self.path)
        }
        pub fn error(mut self) -> String {
            self.path.push(GetObjectResult::ERROR_FIELD.name);
            self.finish()
        }
    }
    impl GetTransactionRequest {
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 1i32,
            message_fields: None,
        };
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetTransactionRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::DIGEST_FIELD,
            Self::READ_MASK_FIELD,
        ];
    }
    impl GetTransactionRequest {
        pub fn path_builder() -> GetTransactionRequestFieldPathBuilder {
            GetTransactionRequestFieldPathBuilder::new()
        }
    }
    pub struct GetTransactionRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetTransactionRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn digest(mut self) -> String {
            self.path.push(GetTransactionRequest::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn read_mask(mut self) -> String {
            self.path.push(GetTransactionRequest::READ_MASK_FIELD.name);
            self.finish()
        }
    }
    impl GetTransactionResponse {
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 1i32,
            message_fields: Some(ExecutedTransaction::FIELDS),
        };
    }
    impl MessageFields for GetTransactionResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::TRANSACTION_FIELD];
    }
    impl GetTransactionResponse {
        pub fn path_builder() -> GetTransactionResponseFieldPathBuilder {
            GetTransactionResponseFieldPathBuilder::new()
        }
    }
    pub struct GetTransactionResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetTransactionResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn transaction(mut self) -> ExecutedTransactionFieldPathBuilder {
            self.path.push(GetTransactionResponse::TRANSACTION_FIELD.name);
            ExecutedTransactionFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl BatchGetTransactionsRequest {
        pub const DIGESTS_FIELD: &'static MessageField = &MessageField {
            name: "digests",
            json_name: "digests",
            number: 1i32,
            message_fields: None,
        };
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for BatchGetTransactionsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::DIGESTS_FIELD,
            Self::READ_MASK_FIELD,
        ];
    }
    impl BatchGetTransactionsRequest {
        pub fn path_builder() -> BatchGetTransactionsRequestFieldPathBuilder {
            BatchGetTransactionsRequestFieldPathBuilder::new()
        }
    }
    pub struct BatchGetTransactionsRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl BatchGetTransactionsRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn digests(mut self) -> String {
            self.path.push(BatchGetTransactionsRequest::DIGESTS_FIELD.name);
            self.finish()
        }
        pub fn read_mask(mut self) -> String {
            self.path.push(BatchGetTransactionsRequest::READ_MASK_FIELD.name);
            self.finish()
        }
    }
    impl BatchGetTransactionsResponse {
        pub const TRANSACTIONS_FIELD: &'static MessageField = &MessageField {
            name: "transactions",
            json_name: "transactions",
            number: 1i32,
            message_fields: Some(GetTransactionResult::FIELDS),
        };
    }
    impl MessageFields for BatchGetTransactionsResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::TRANSACTIONS_FIELD];
    }
    impl BatchGetTransactionsResponse {
        pub fn path_builder() -> BatchGetTransactionsResponseFieldPathBuilder {
            BatchGetTransactionsResponseFieldPathBuilder::new()
        }
    }
    pub struct BatchGetTransactionsResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl BatchGetTransactionsResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn transactions(mut self) -> GetTransactionResultFieldPathBuilder {
            self.path.push(BatchGetTransactionsResponse::TRANSACTIONS_FIELD.name);
            GetTransactionResultFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl GetTransactionResult {
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 1i32,
            message_fields: Some(ExecutedTransaction::FIELDS),
        };
        pub const ERROR_FIELD: &'static MessageField = &MessageField {
            name: "error",
            json_name: "error",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetTransactionResult {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TRANSACTION_FIELD,
            Self::ERROR_FIELD,
        ];
    }
    impl GetTransactionResult {
        pub fn path_builder() -> GetTransactionResultFieldPathBuilder {
            GetTransactionResultFieldPathBuilder::new()
        }
    }
    pub struct GetTransactionResultFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetTransactionResultFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn transaction(mut self) -> ExecutedTransactionFieldPathBuilder {
            self.path.push(GetTransactionResult::TRANSACTION_FIELD.name);
            ExecutedTransactionFieldPathBuilder::new_with_base(self.path)
        }
        pub fn error(mut self) -> String {
            self.path.push(GetTransactionResult::ERROR_FIELD.name);
            self.finish()
        }
    }
    impl GetCheckpointRequest {
        pub const SEQUENCE_NUMBER_FIELD: &'static MessageField = &MessageField {
            name: "sequence_number",
            json_name: "sequenceNumber",
            number: 1i32,
            message_fields: None,
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 2i32,
            message_fields: None,
        };
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetCheckpointRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::SEQUENCE_NUMBER_FIELD,
            Self::DIGEST_FIELD,
            Self::READ_MASK_FIELD,
        ];
    }
    impl GetCheckpointRequest {
        pub fn path_builder() -> GetCheckpointRequestFieldPathBuilder {
            GetCheckpointRequestFieldPathBuilder::new()
        }
    }
    pub struct GetCheckpointRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetCheckpointRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn sequence_number(mut self) -> String {
            self.path.push(GetCheckpointRequest::SEQUENCE_NUMBER_FIELD.name);
            self.finish()
        }
        pub fn digest(mut self) -> String {
            self.path.push(GetCheckpointRequest::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn read_mask(mut self) -> String {
            self.path.push(GetCheckpointRequest::READ_MASK_FIELD.name);
            self.finish()
        }
    }
    impl GetCheckpointResponse {
        pub const CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "checkpoint",
            json_name: "checkpoint",
            number: 1i32,
            message_fields: Some(Checkpoint::FIELDS),
        };
    }
    impl MessageFields for GetCheckpointResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::CHECKPOINT_FIELD];
    }
    impl GetCheckpointResponse {
        pub fn path_builder() -> GetCheckpointResponseFieldPathBuilder {
            GetCheckpointResponseFieldPathBuilder::new()
        }
    }
    pub struct GetCheckpointResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetCheckpointResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn checkpoint(mut self) -> CheckpointFieldPathBuilder {
            self.path.push(GetCheckpointResponse::CHECKPOINT_FIELD.name);
            CheckpointFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl GetEpochRequest {
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 1i32,
            message_fields: None,
        };
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetEpochRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EPOCH_FIELD,
            Self::READ_MASK_FIELD,
        ];
    }
    impl GetEpochRequest {
        pub fn path_builder() -> GetEpochRequestFieldPathBuilder {
            GetEpochRequestFieldPathBuilder::new()
        }
    }
    pub struct GetEpochRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetEpochRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn epoch(mut self) -> String {
            self.path.push(GetEpochRequest::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn read_mask(mut self) -> String {
            self.path.push(GetEpochRequest::READ_MASK_FIELD.name);
            self.finish()
        }
    }
    impl GetEpochResponse {
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 1i32,
            message_fields: Some(Epoch::FIELDS),
        };
    }
    impl MessageFields for GetEpochResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::EPOCH_FIELD];
    }
    impl GetEpochResponse {
        pub fn path_builder() -> GetEpochResponseFieldPathBuilder {
            GetEpochResponseFieldPathBuilder::new()
        }
    }
    pub struct GetEpochResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetEpochResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn epoch(mut self) -> EpochFieldPathBuilder {
            self.path.push(GetEpochResponse::EPOCH_FIELD.name);
            EpochFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl Package {
        pub const STORAGE_ID_FIELD: &'static MessageField = &MessageField {
            name: "storage_id",
            json_name: "storageId",
            number: 1i32,
            message_fields: None,
        };
        pub const ORIGINAL_ID_FIELD: &'static MessageField = &MessageField {
            name: "original_id",
            json_name: "originalId",
            number: 2i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 3i32,
            message_fields: None,
        };
        pub const MODULES_FIELD: &'static MessageField = &MessageField {
            name: "modules",
            json_name: "modules",
            number: 4i32,
            message_fields: Some(Module::FIELDS),
        };
        pub const TYPE_ORIGINS_FIELD: &'static MessageField = &MessageField {
            name: "type_origins",
            json_name: "typeOrigins",
            number: 5i32,
            message_fields: Some(TypeOrigin::FIELDS),
        };
        pub const LINKAGE_FIELD: &'static MessageField = &MessageField {
            name: "linkage",
            json_name: "linkage",
            number: 6i32,
            message_fields: Some(Linkage::FIELDS),
        };
    }
    impl MessageFields for Package {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::STORAGE_ID_FIELD,
            Self::ORIGINAL_ID_FIELD,
            Self::VERSION_FIELD,
            Self::MODULES_FIELD,
            Self::TYPE_ORIGINS_FIELD,
            Self::LINKAGE_FIELD,
        ];
    }
    impl Package {
        pub fn path_builder() -> PackageFieldPathBuilder {
            PackageFieldPathBuilder::new()
        }
    }
    pub struct PackageFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl PackageFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn storage_id(mut self) -> String {
            self.path.push(Package::STORAGE_ID_FIELD.name);
            self.finish()
        }
        pub fn original_id(mut self) -> String {
            self.path.push(Package::ORIGINAL_ID_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(Package::VERSION_FIELD.name);
            self.finish()
        }
        pub fn modules(mut self) -> ModuleFieldPathBuilder {
            self.path.push(Package::MODULES_FIELD.name);
            ModuleFieldPathBuilder::new_with_base(self.path)
        }
        pub fn type_origins(mut self) -> TypeOriginFieldPathBuilder {
            self.path.push(Package::TYPE_ORIGINS_FIELD.name);
            TypeOriginFieldPathBuilder::new_with_base(self.path)
        }
        pub fn linkage(mut self) -> LinkageFieldPathBuilder {
            self.path.push(Package::LINKAGE_FIELD.name);
            LinkageFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl Module {
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 1i32,
            message_fields: None,
        };
        pub const CONTENTS_FIELD: &'static MessageField = &MessageField {
            name: "contents",
            json_name: "contents",
            number: 2i32,
            message_fields: None,
        };
        pub const DATATYPES_FIELD: &'static MessageField = &MessageField {
            name: "datatypes",
            json_name: "datatypes",
            number: 3i32,
            message_fields: Some(DatatypeDescriptor::FIELDS),
        };
        pub const FUNCTIONS_FIELD: &'static MessageField = &MessageField {
            name: "functions",
            json_name: "functions",
            number: 4i32,
            message_fields: Some(FunctionDescriptor::FIELDS),
        };
    }
    impl MessageFields for Module {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::NAME_FIELD,
            Self::CONTENTS_FIELD,
            Self::DATATYPES_FIELD,
            Self::FUNCTIONS_FIELD,
        ];
    }
    impl Module {
        pub fn path_builder() -> ModuleFieldPathBuilder {
            ModuleFieldPathBuilder::new()
        }
    }
    pub struct ModuleFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ModuleFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn name(mut self) -> String {
            self.path.push(Module::NAME_FIELD.name);
            self.finish()
        }
        pub fn contents(mut self) -> String {
            self.path.push(Module::CONTENTS_FIELD.name);
            self.finish()
        }
        pub fn datatypes(mut self) -> DatatypeDescriptorFieldPathBuilder {
            self.path.push(Module::DATATYPES_FIELD.name);
            DatatypeDescriptorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn functions(mut self) -> FunctionDescriptorFieldPathBuilder {
            self.path.push(Module::FUNCTIONS_FIELD.name);
            FunctionDescriptorFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl DatatypeDescriptor {
        pub const TYPE_NAME_FIELD: &'static MessageField = &MessageField {
            name: "type_name",
            json_name: "typeName",
            number: 1i32,
            message_fields: None,
        };
        pub const DEFINING_ID_FIELD: &'static MessageField = &MessageField {
            name: "defining_id",
            json_name: "definingId",
            number: 2i32,
            message_fields: None,
        };
        pub const MODULE_FIELD: &'static MessageField = &MessageField {
            name: "module",
            json_name: "module",
            number: 3i32,
            message_fields: None,
        };
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 4i32,
            message_fields: None,
        };
        pub const ABILITIES_FIELD: &'static MessageField = &MessageField {
            name: "abilities",
            json_name: "abilities",
            number: 5i32,
            message_fields: None,
        };
        pub const TYPE_PARAMETERS_FIELD: &'static MessageField = &MessageField {
            name: "type_parameters",
            json_name: "typeParameters",
            number: 6i32,
            message_fields: Some(TypeParameter::FIELDS),
        };
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 7i32,
            message_fields: None,
        };
        pub const FIELDS_FIELD: &'static MessageField = &MessageField {
            name: "fields",
            json_name: "fields",
            number: 8i32,
            message_fields: Some(FieldDescriptor::FIELDS),
        };
        pub const VARIANTS_FIELD: &'static MessageField = &MessageField {
            name: "variants",
            json_name: "variants",
            number: 9i32,
            message_fields: Some(VariantDescriptor::FIELDS),
        };
    }
    impl MessageFields for DatatypeDescriptor {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TYPE_NAME_FIELD,
            Self::DEFINING_ID_FIELD,
            Self::MODULE_FIELD,
            Self::NAME_FIELD,
            Self::ABILITIES_FIELD,
            Self::TYPE_PARAMETERS_FIELD,
            Self::KIND_FIELD,
            Self::FIELDS_FIELD,
            Self::VARIANTS_FIELD,
        ];
    }
    impl DatatypeDescriptor {
        pub fn path_builder() -> DatatypeDescriptorFieldPathBuilder {
            DatatypeDescriptorFieldPathBuilder::new()
        }
    }
    pub struct DatatypeDescriptorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl DatatypeDescriptorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn type_name(mut self) -> String {
            self.path.push(DatatypeDescriptor::TYPE_NAME_FIELD.name);
            self.finish()
        }
        pub fn defining_id(mut self) -> String {
            self.path.push(DatatypeDescriptor::DEFINING_ID_FIELD.name);
            self.finish()
        }
        pub fn module(mut self) -> String {
            self.path.push(DatatypeDescriptor::MODULE_FIELD.name);
            self.finish()
        }
        pub fn name(mut self) -> String {
            self.path.push(DatatypeDescriptor::NAME_FIELD.name);
            self.finish()
        }
        pub fn abilities(mut self) -> String {
            self.path.push(DatatypeDescriptor::ABILITIES_FIELD.name);
            self.finish()
        }
        pub fn type_parameters(mut self) -> TypeParameterFieldPathBuilder {
            self.path.push(DatatypeDescriptor::TYPE_PARAMETERS_FIELD.name);
            TypeParameterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn kind(mut self) -> String {
            self.path.push(DatatypeDescriptor::KIND_FIELD.name);
            self.finish()
        }
        pub fn fields(mut self) -> FieldDescriptorFieldPathBuilder {
            self.path.push(DatatypeDescriptor::FIELDS_FIELD.name);
            FieldDescriptorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn variants(mut self) -> VariantDescriptorFieldPathBuilder {
            self.path.push(DatatypeDescriptor::VARIANTS_FIELD.name);
            VariantDescriptorFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl TypeParameter {
        pub const CONSTRAINTS_FIELD: &'static MessageField = &MessageField {
            name: "constraints",
            json_name: "constraints",
            number: 1i32,
            message_fields: None,
        };
        pub const IS_PHANTOM_FIELD: &'static MessageField = &MessageField {
            name: "is_phantom",
            json_name: "isPhantom",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for TypeParameter {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CONSTRAINTS_FIELD,
            Self::IS_PHANTOM_FIELD,
        ];
    }
    impl TypeParameter {
        pub fn path_builder() -> TypeParameterFieldPathBuilder {
            TypeParameterFieldPathBuilder::new()
        }
    }
    pub struct TypeParameterFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TypeParameterFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn constraints(mut self) -> String {
            self.path.push(TypeParameter::CONSTRAINTS_FIELD.name);
            self.finish()
        }
        pub fn is_phantom(mut self) -> String {
            self.path.push(TypeParameter::IS_PHANTOM_FIELD.name);
            self.finish()
        }
    }
    impl FieldDescriptor {
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 1i32,
            message_fields: None,
        };
        pub const POSITION_FIELD: &'static MessageField = &MessageField {
            name: "position",
            json_name: "position",
            number: 2i32,
            message_fields: None,
        };
        pub const TYPE_FIELD: &'static MessageField = &MessageField {
            name: "type",
            json_name: "type",
            number: 3i32,
            message_fields: Some(OpenSignatureBody::FIELDS),
        };
    }
    impl MessageFields for FieldDescriptor {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::NAME_FIELD,
            Self::POSITION_FIELD,
            Self::TYPE_FIELD,
        ];
    }
    impl FieldDescriptor {
        pub fn path_builder() -> FieldDescriptorFieldPathBuilder {
            FieldDescriptorFieldPathBuilder::new()
        }
    }
    pub struct FieldDescriptorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl FieldDescriptorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn name(mut self) -> String {
            self.path.push(FieldDescriptor::NAME_FIELD.name);
            self.finish()
        }
        pub fn position(mut self) -> String {
            self.path.push(FieldDescriptor::POSITION_FIELD.name);
            self.finish()
        }
        pub fn r#type(mut self) -> OpenSignatureBodyFieldPathBuilder {
            self.path.push(FieldDescriptor::TYPE_FIELD.name);
            OpenSignatureBodyFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl VariantDescriptor {
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 1i32,
            message_fields: None,
        };
        pub const POSITION_FIELD: &'static MessageField = &MessageField {
            name: "position",
            json_name: "position",
            number: 2i32,
            message_fields: None,
        };
        pub const FIELDS_FIELD: &'static MessageField = &MessageField {
            name: "fields",
            json_name: "fields",
            number: 3i32,
            message_fields: Some(FieldDescriptor::FIELDS),
        };
    }
    impl MessageFields for VariantDescriptor {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::NAME_FIELD,
            Self::POSITION_FIELD,
            Self::FIELDS_FIELD,
        ];
    }
    impl VariantDescriptor {
        pub fn path_builder() -> VariantDescriptorFieldPathBuilder {
            VariantDescriptorFieldPathBuilder::new()
        }
    }
    pub struct VariantDescriptorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl VariantDescriptorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn name(mut self) -> String {
            self.path.push(VariantDescriptor::NAME_FIELD.name);
            self.finish()
        }
        pub fn position(mut self) -> String {
            self.path.push(VariantDescriptor::POSITION_FIELD.name);
            self.finish()
        }
        pub fn fields(mut self) -> FieldDescriptorFieldPathBuilder {
            self.path.push(VariantDescriptor::FIELDS_FIELD.name);
            FieldDescriptorFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl OpenSignatureBody {
        pub const TYPE_FIELD: &'static MessageField = &MessageField {
            name: "type",
            json_name: "type",
            number: 1i32,
            message_fields: None,
        };
        pub const TYPE_NAME_FIELD: &'static MessageField = &MessageField {
            name: "type_name",
            json_name: "typeName",
            number: 2i32,
            message_fields: None,
        };
        pub const TYPE_PARAMETER_INSTANTIATION_FIELD: &'static MessageField = &MessageField {
            name: "type_parameter_instantiation",
            json_name: "typeParameterInstantiation",
            number: 3i32,
            message_fields: None,
        };
        pub const TYPE_PARAMETER_FIELD: &'static MessageField = &MessageField {
            name: "type_parameter",
            json_name: "typeParameter",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for OpenSignatureBody {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TYPE_FIELD,
            Self::TYPE_NAME_FIELD,
            Self::TYPE_PARAMETER_INSTANTIATION_FIELD,
            Self::TYPE_PARAMETER_FIELD,
        ];
    }
    impl OpenSignatureBody {
        pub fn path_builder() -> OpenSignatureBodyFieldPathBuilder {
            OpenSignatureBodyFieldPathBuilder::new()
        }
    }
    pub struct OpenSignatureBodyFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl OpenSignatureBodyFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn r#type(mut self) -> String {
            self.path.push(OpenSignatureBody::TYPE_FIELD.name);
            self.finish()
        }
        pub fn type_name(mut self) -> String {
            self.path.push(OpenSignatureBody::TYPE_NAME_FIELD.name);
            self.finish()
        }
        pub fn type_parameter_instantiation(mut self) -> String {
            self.path.push(OpenSignatureBody::TYPE_PARAMETER_INSTANTIATION_FIELD.name);
            self.finish()
        }
        pub fn type_parameter(mut self) -> String {
            self.path.push(OpenSignatureBody::TYPE_PARAMETER_FIELD.name);
            self.finish()
        }
    }
    impl FunctionDescriptor {
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 1i32,
            message_fields: None,
        };
        pub const VISIBILITY_FIELD: &'static MessageField = &MessageField {
            name: "visibility",
            json_name: "visibility",
            number: 5i32,
            message_fields: None,
        };
        pub const IS_ENTRY_FIELD: &'static MessageField = &MessageField {
            name: "is_entry",
            json_name: "isEntry",
            number: 6i32,
            message_fields: None,
        };
        pub const TYPE_PARAMETERS_FIELD: &'static MessageField = &MessageField {
            name: "type_parameters",
            json_name: "typeParameters",
            number: 7i32,
            message_fields: Some(TypeParameter::FIELDS),
        };
        pub const PARAMETERS_FIELD: &'static MessageField = &MessageField {
            name: "parameters",
            json_name: "parameters",
            number: 8i32,
            message_fields: Some(OpenSignature::FIELDS),
        };
        pub const RETURNS_FIELD: &'static MessageField = &MessageField {
            name: "returns",
            json_name: "returns",
            number: 9i32,
            message_fields: Some(OpenSignature::FIELDS),
        };
    }
    impl MessageFields for FunctionDescriptor {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::NAME_FIELD,
            Self::VISIBILITY_FIELD,
            Self::IS_ENTRY_FIELD,
            Self::TYPE_PARAMETERS_FIELD,
            Self::PARAMETERS_FIELD,
            Self::RETURNS_FIELD,
        ];
    }
    impl FunctionDescriptor {
        pub fn path_builder() -> FunctionDescriptorFieldPathBuilder {
            FunctionDescriptorFieldPathBuilder::new()
        }
    }
    pub struct FunctionDescriptorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl FunctionDescriptorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn name(mut self) -> String {
            self.path.push(FunctionDescriptor::NAME_FIELD.name);
            self.finish()
        }
        pub fn visibility(mut self) -> String {
            self.path.push(FunctionDescriptor::VISIBILITY_FIELD.name);
            self.finish()
        }
        pub fn is_entry(mut self) -> String {
            self.path.push(FunctionDescriptor::IS_ENTRY_FIELD.name);
            self.finish()
        }
        pub fn type_parameters(mut self) -> TypeParameterFieldPathBuilder {
            self.path.push(FunctionDescriptor::TYPE_PARAMETERS_FIELD.name);
            TypeParameterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn parameters(mut self) -> OpenSignatureFieldPathBuilder {
            self.path.push(FunctionDescriptor::PARAMETERS_FIELD.name);
            OpenSignatureFieldPathBuilder::new_with_base(self.path)
        }
        pub fn returns(mut self) -> OpenSignatureFieldPathBuilder {
            self.path.push(FunctionDescriptor::RETURNS_FIELD.name);
            OpenSignatureFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl OpenSignature {
        pub const REFERENCE_FIELD: &'static MessageField = &MessageField {
            name: "reference",
            json_name: "reference",
            number: 1i32,
            message_fields: None,
        };
        pub const BODY_FIELD: &'static MessageField = &MessageField {
            name: "body",
            json_name: "body",
            number: 2i32,
            message_fields: Some(OpenSignatureBody::FIELDS),
        };
    }
    impl MessageFields for OpenSignature {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::REFERENCE_FIELD,
            Self::BODY_FIELD,
        ];
    }
    impl OpenSignature {
        pub fn path_builder() -> OpenSignatureFieldPathBuilder {
            OpenSignatureFieldPathBuilder::new()
        }
    }
    pub struct OpenSignatureFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl OpenSignatureFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn reference(mut self) -> String {
            self.path.push(OpenSignature::REFERENCE_FIELD.name);
            self.finish()
        }
        pub fn body(mut self) -> OpenSignatureBodyFieldPathBuilder {
            self.path.push(OpenSignature::BODY_FIELD.name);
            OpenSignatureBodyFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl TypeOrigin {
        pub const MODULE_NAME_FIELD: &'static MessageField = &MessageField {
            name: "module_name",
            json_name: "moduleName",
            number: 1i32,
            message_fields: None,
        };
        pub const DATATYPE_NAME_FIELD: &'static MessageField = &MessageField {
            name: "datatype_name",
            json_name: "datatypeName",
            number: 2i32,
            message_fields: None,
        };
        pub const PACKAGE_ID_FIELD: &'static MessageField = &MessageField {
            name: "package_id",
            json_name: "packageId",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for TypeOrigin {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::MODULE_NAME_FIELD,
            Self::DATATYPE_NAME_FIELD,
            Self::PACKAGE_ID_FIELD,
        ];
    }
    impl TypeOrigin {
        pub fn path_builder() -> TypeOriginFieldPathBuilder {
            TypeOriginFieldPathBuilder::new()
        }
    }
    pub struct TypeOriginFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TypeOriginFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn module_name(mut self) -> String {
            self.path.push(TypeOrigin::MODULE_NAME_FIELD.name);
            self.finish()
        }
        pub fn datatype_name(mut self) -> String {
            self.path.push(TypeOrigin::DATATYPE_NAME_FIELD.name);
            self.finish()
        }
        pub fn package_id(mut self) -> String {
            self.path.push(TypeOrigin::PACKAGE_ID_FIELD.name);
            self.finish()
        }
    }
    impl Linkage {
        pub const ORIGINAL_ID_FIELD: &'static MessageField = &MessageField {
            name: "original_id",
            json_name: "originalId",
            number: 1i32,
            message_fields: None,
        };
        pub const UPGRADED_ID_FIELD: &'static MessageField = &MessageField {
            name: "upgraded_id",
            json_name: "upgradedId",
            number: 2i32,
            message_fields: None,
        };
        pub const UPGRADED_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "upgraded_version",
            json_name: "upgradedVersion",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for Linkage {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ORIGINAL_ID_FIELD,
            Self::UPGRADED_ID_FIELD,
            Self::UPGRADED_VERSION_FIELD,
        ];
    }
    impl Linkage {
        pub fn path_builder() -> LinkageFieldPathBuilder {
            LinkageFieldPathBuilder::new()
        }
    }
    pub struct LinkageFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl LinkageFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn original_id(mut self) -> String {
            self.path.push(Linkage::ORIGINAL_ID_FIELD.name);
            self.finish()
        }
        pub fn upgraded_id(mut self) -> String {
            self.path.push(Linkage::UPGRADED_ID_FIELD.name);
            self.finish()
        }
        pub fn upgraded_version(mut self) -> String {
            self.path.push(Linkage::UPGRADED_VERSION_FIELD.name);
            self.finish()
        }
    }
    impl GetPackageRequest {
        pub const PACKAGE_ID_FIELD: &'static MessageField = &MessageField {
            name: "package_id",
            json_name: "packageId",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetPackageRequest {
        const FIELDS: &'static [&'static MessageField] = &[Self::PACKAGE_ID_FIELD];
    }
    impl GetPackageRequest {
        pub fn path_builder() -> GetPackageRequestFieldPathBuilder {
            GetPackageRequestFieldPathBuilder::new()
        }
    }
    pub struct GetPackageRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetPackageRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn package_id(mut self) -> String {
            self.path.push(GetPackageRequest::PACKAGE_ID_FIELD.name);
            self.finish()
        }
    }
    impl GetPackageResponse {
        pub const PACKAGE_FIELD: &'static MessageField = &MessageField {
            name: "package",
            json_name: "package",
            number: 1i32,
            message_fields: Some(Package::FIELDS),
        };
    }
    impl MessageFields for GetPackageResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::PACKAGE_FIELD];
    }
    impl GetPackageResponse {
        pub fn path_builder() -> GetPackageResponseFieldPathBuilder {
            GetPackageResponseFieldPathBuilder::new()
        }
    }
    pub struct GetPackageResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetPackageResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn package(mut self) -> PackageFieldPathBuilder {
            self.path.push(GetPackageResponse::PACKAGE_FIELD.name);
            PackageFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl GetDatatypeRequest {
        pub const PACKAGE_ID_FIELD: &'static MessageField = &MessageField {
            name: "package_id",
            json_name: "packageId",
            number: 1i32,
            message_fields: None,
        };
        pub const MODULE_NAME_FIELD: &'static MessageField = &MessageField {
            name: "module_name",
            json_name: "moduleName",
            number: 2i32,
            message_fields: None,
        };
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetDatatypeRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PACKAGE_ID_FIELD,
            Self::MODULE_NAME_FIELD,
            Self::NAME_FIELD,
        ];
    }
    impl GetDatatypeRequest {
        pub fn path_builder() -> GetDatatypeRequestFieldPathBuilder {
            GetDatatypeRequestFieldPathBuilder::new()
        }
    }
    pub struct GetDatatypeRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetDatatypeRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn package_id(mut self) -> String {
            self.path.push(GetDatatypeRequest::PACKAGE_ID_FIELD.name);
            self.finish()
        }
        pub fn module_name(mut self) -> String {
            self.path.push(GetDatatypeRequest::MODULE_NAME_FIELD.name);
            self.finish()
        }
        pub fn name(mut self) -> String {
            self.path.push(GetDatatypeRequest::NAME_FIELD.name);
            self.finish()
        }
    }
    impl GetDatatypeResponse {
        pub const DATATYPE_FIELD: &'static MessageField = &MessageField {
            name: "datatype",
            json_name: "datatype",
            number: 1i32,
            message_fields: Some(DatatypeDescriptor::FIELDS),
        };
    }
    impl MessageFields for GetDatatypeResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::DATATYPE_FIELD];
    }
    impl GetDatatypeResponse {
        pub fn path_builder() -> GetDatatypeResponseFieldPathBuilder {
            GetDatatypeResponseFieldPathBuilder::new()
        }
    }
    pub struct GetDatatypeResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetDatatypeResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn datatype(mut self) -> DatatypeDescriptorFieldPathBuilder {
            self.path.push(GetDatatypeResponse::DATATYPE_FIELD.name);
            DatatypeDescriptorFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl GetFunctionRequest {
        pub const PACKAGE_ID_FIELD: &'static MessageField = &MessageField {
            name: "package_id",
            json_name: "packageId",
            number: 1i32,
            message_fields: None,
        };
        pub const MODULE_NAME_FIELD: &'static MessageField = &MessageField {
            name: "module_name",
            json_name: "moduleName",
            number: 2i32,
            message_fields: None,
        };
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetFunctionRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PACKAGE_ID_FIELD,
            Self::MODULE_NAME_FIELD,
            Self::NAME_FIELD,
        ];
    }
    impl GetFunctionRequest {
        pub fn path_builder() -> GetFunctionRequestFieldPathBuilder {
            GetFunctionRequestFieldPathBuilder::new()
        }
    }
    pub struct GetFunctionRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetFunctionRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn package_id(mut self) -> String {
            self.path.push(GetFunctionRequest::PACKAGE_ID_FIELD.name);
            self.finish()
        }
        pub fn module_name(mut self) -> String {
            self.path.push(GetFunctionRequest::MODULE_NAME_FIELD.name);
            self.finish()
        }
        pub fn name(mut self) -> String {
            self.path.push(GetFunctionRequest::NAME_FIELD.name);
            self.finish()
        }
    }
    impl GetFunctionResponse {
        pub const FUNCTION_FIELD: &'static MessageField = &MessageField {
            name: "function",
            json_name: "function",
            number: 1i32,
            message_fields: Some(FunctionDescriptor::FIELDS),
        };
    }
    impl MessageFields for GetFunctionResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::FUNCTION_FIELD];
    }
    impl GetFunctionResponse {
        pub fn path_builder() -> GetFunctionResponseFieldPathBuilder {
            GetFunctionResponseFieldPathBuilder::new()
        }
    }
    pub struct GetFunctionResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetFunctionResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn function(mut self) -> FunctionDescriptorFieldPathBuilder {
            self.path.push(GetFunctionResponse::FUNCTION_FIELD.name);
            FunctionDescriptorFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ListPackageVersionsRequest {
        pub const PACKAGE_ID_FIELD: &'static MessageField = &MessageField {
            name: "package_id",
            json_name: "packageId",
            number: 1i32,
            message_fields: None,
        };
        pub const PAGE_SIZE_FIELD: &'static MessageField = &MessageField {
            name: "page_size",
            json_name: "pageSize",
            number: 2i32,
            message_fields: None,
        };
        pub const PAGE_TOKEN_FIELD: &'static MessageField = &MessageField {
            name: "page_token",
            json_name: "pageToken",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for ListPackageVersionsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PACKAGE_ID_FIELD,
            Self::PAGE_SIZE_FIELD,
            Self::PAGE_TOKEN_FIELD,
        ];
    }
    impl ListPackageVersionsRequest {
        pub fn path_builder() -> ListPackageVersionsRequestFieldPathBuilder {
            ListPackageVersionsRequestFieldPathBuilder::new()
        }
    }
    pub struct ListPackageVersionsRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListPackageVersionsRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn package_id(mut self) -> String {
            self.path.push(ListPackageVersionsRequest::PACKAGE_ID_FIELD.name);
            self.finish()
        }
        pub fn page_size(mut self) -> String {
            self.path.push(ListPackageVersionsRequest::PAGE_SIZE_FIELD.name);
            self.finish()
        }
        pub fn page_token(mut self) -> String {
            self.path.push(ListPackageVersionsRequest::PAGE_TOKEN_FIELD.name);
            self.finish()
        }
    }
    impl ListPackageVersionsResponse {
        pub const VERSIONS_FIELD: &'static MessageField = &MessageField {
            name: "versions",
            json_name: "versions",
            number: 1i32,
            message_fields: Some(PackageVersion::FIELDS),
        };
        pub const NEXT_PAGE_TOKEN_FIELD: &'static MessageField = &MessageField {
            name: "next_page_token",
            json_name: "nextPageToken",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for ListPackageVersionsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::VERSIONS_FIELD,
            Self::NEXT_PAGE_TOKEN_FIELD,
        ];
    }
    impl ListPackageVersionsResponse {
        pub fn path_builder() -> ListPackageVersionsResponseFieldPathBuilder {
            ListPackageVersionsResponseFieldPathBuilder::new()
        }
    }
    pub struct ListPackageVersionsResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListPackageVersionsResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn versions(mut self) -> PackageVersionFieldPathBuilder {
            self.path.push(ListPackageVersionsResponse::VERSIONS_FIELD.name);
            PackageVersionFieldPathBuilder::new_with_base(self.path)
        }
        pub fn next_page_token(mut self) -> String {
            self.path.push(ListPackageVersionsResponse::NEXT_PAGE_TOKEN_FIELD.name);
            self.finish()
        }
    }
    impl PackageVersion {
        pub const PACKAGE_ID_FIELD: &'static MessageField = &MessageField {
            name: "package_id",
            json_name: "packageId",
            number: 1i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for PackageVersion {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PACKAGE_ID_FIELD,
            Self::VERSION_FIELD,
        ];
    }
    impl PackageVersion {
        pub fn path_builder() -> PackageVersionFieldPathBuilder {
            PackageVersionFieldPathBuilder::new()
        }
    }
    pub struct PackageVersionFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl PackageVersionFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn package_id(mut self) -> String {
            self.path.push(PackageVersion::PACKAGE_ID_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(PackageVersion::VERSION_FIELD.name);
            self.finish()
        }
    }
    impl LookupNameRequest {
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for LookupNameRequest {
        const FIELDS: &'static [&'static MessageField] = &[Self::NAME_FIELD];
    }
    impl LookupNameRequest {
        pub fn path_builder() -> LookupNameRequestFieldPathBuilder {
            LookupNameRequestFieldPathBuilder::new()
        }
    }
    pub struct LookupNameRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl LookupNameRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn name(mut self) -> String {
            self.path.push(LookupNameRequest::NAME_FIELD.name);
            self.finish()
        }
    }
    impl LookupNameResponse {
        pub const RECORD_FIELD: &'static MessageField = &MessageField {
            name: "record",
            json_name: "record",
            number: 1i32,
            message_fields: Some(NameRecord::FIELDS),
        };
    }
    impl MessageFields for LookupNameResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::RECORD_FIELD];
    }
    impl LookupNameResponse {
        pub fn path_builder() -> LookupNameResponseFieldPathBuilder {
            LookupNameResponseFieldPathBuilder::new()
        }
    }
    pub struct LookupNameResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl LookupNameResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn record(mut self) -> NameRecordFieldPathBuilder {
            self.path.push(LookupNameResponse::RECORD_FIELD.name);
            NameRecordFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ReverseLookupNameRequest {
        pub const ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "address",
            json_name: "address",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for ReverseLookupNameRequest {
        const FIELDS: &'static [&'static MessageField] = &[Self::ADDRESS_FIELD];
    }
    impl ReverseLookupNameRequest {
        pub fn path_builder() -> ReverseLookupNameRequestFieldPathBuilder {
            ReverseLookupNameRequestFieldPathBuilder::new()
        }
    }
    pub struct ReverseLookupNameRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ReverseLookupNameRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn address(mut self) -> String {
            self.path.push(ReverseLookupNameRequest::ADDRESS_FIELD.name);
            self.finish()
        }
    }
    impl ReverseLookupNameResponse {
        pub const RECORD_FIELD: &'static MessageField = &MessageField {
            name: "record",
            json_name: "record",
            number: 1i32,
            message_fields: Some(NameRecord::FIELDS),
        };
    }
    impl MessageFields for ReverseLookupNameResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::RECORD_FIELD];
    }
    impl ReverseLookupNameResponse {
        pub fn path_builder() -> ReverseLookupNameResponseFieldPathBuilder {
            ReverseLookupNameResponseFieldPathBuilder::new()
        }
    }
    pub struct ReverseLookupNameResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ReverseLookupNameResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn record(mut self) -> NameRecordFieldPathBuilder {
            self.path.push(ReverseLookupNameResponse::RECORD_FIELD.name);
            NameRecordFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl NameRecord {
        pub const ID_FIELD: &'static MessageField = &MessageField {
            name: "id",
            json_name: "id",
            number: 1i32,
            message_fields: None,
        };
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 2i32,
            message_fields: None,
        };
        pub const REGISTRATION_NFT_ID_FIELD: &'static MessageField = &MessageField {
            name: "registration_nft_id",
            json_name: "registrationNftId",
            number: 3i32,
            message_fields: None,
        };
        pub const EXPIRATION_TIMESTAMP_FIELD: &'static MessageField = &MessageField {
            name: "expiration_timestamp",
            json_name: "expirationTimestamp",
            number: 4i32,
            message_fields: None,
        };
        pub const TARGET_ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "target_address",
            json_name: "targetAddress",
            number: 5i32,
            message_fields: None,
        };
        pub const DATA_FIELD: &'static MessageField = &MessageField {
            name: "data",
            json_name: "data",
            number: 6i32,
            message_fields: None,
        };
    }
    impl MessageFields for NameRecord {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ID_FIELD,
            Self::NAME_FIELD,
            Self::REGISTRATION_NFT_ID_FIELD,
            Self::EXPIRATION_TIMESTAMP_FIELD,
            Self::TARGET_ADDRESS_FIELD,
            Self::DATA_FIELD,
        ];
    }
    impl NameRecord {
        pub fn path_builder() -> NameRecordFieldPathBuilder {
            NameRecordFieldPathBuilder::new()
        }
    }
    pub struct NameRecordFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl NameRecordFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn id(mut self) -> String {
            self.path.push(NameRecord::ID_FIELD.name);
            self.finish()
        }
        pub fn name(mut self) -> String {
            self.path.push(NameRecord::NAME_FIELD.name);
            self.finish()
        }
        pub fn registration_nft_id(mut self) -> String {
            self.path.push(NameRecord::REGISTRATION_NFT_ID_FIELD.name);
            self.finish()
        }
        pub fn expiration_timestamp(mut self) -> String {
            self.path.push(NameRecord::EXPIRATION_TIMESTAMP_FIELD.name);
            self.finish()
        }
        pub fn target_address(mut self) -> String {
            self.path.push(NameRecord::TARGET_ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn data(mut self) -> String {
            self.path.push(NameRecord::DATA_FIELD.name);
            self.finish()
        }
    }
    impl Object {
        pub const BCS_FIELD: &'static MessageField = &MessageField {
            name: "bcs",
            json_name: "bcs",
            number: 1i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "object_id",
            json_name: "objectId",
            number: 2i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 3i32,
            message_fields: None,
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 4i32,
            message_fields: None,
        };
        pub const OWNER_FIELD: &'static MessageField = &MessageField {
            name: "owner",
            json_name: "owner",
            number: 5i32,
            message_fields: Some(Owner::FIELDS),
        };
        pub const OBJECT_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "object_type",
            json_name: "objectType",
            number: 6i32,
            message_fields: None,
        };
        pub const HAS_PUBLIC_TRANSFER_FIELD: &'static MessageField = &MessageField {
            name: "has_public_transfer",
            json_name: "hasPublicTransfer",
            number: 7i32,
            message_fields: None,
        };
        pub const CONTENTS_FIELD: &'static MessageField = &MessageField {
            name: "contents",
            json_name: "contents",
            number: 8i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const PACKAGE_FIELD: &'static MessageField = &MessageField {
            name: "package",
            json_name: "package",
            number: 9i32,
            message_fields: Some(Package::FIELDS),
        };
        pub const PREVIOUS_TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "previous_transaction",
            json_name: "previousTransaction",
            number: 10i32,
            message_fields: None,
        };
        pub const STORAGE_REBATE_FIELD: &'static MessageField = &MessageField {
            name: "storage_rebate",
            json_name: "storageRebate",
            number: 11i32,
            message_fields: None,
        };
        pub const JSON_FIELD: &'static MessageField = &MessageField {
            name: "json",
            json_name: "json",
            number: 100i32,
            message_fields: None,
        };
        pub const BALANCE_FIELD: &'static MessageField = &MessageField {
            name: "balance",
            json_name: "balance",
            number: 101i32,
            message_fields: None,
        };
    }
    impl MessageFields for Object {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::BCS_FIELD,
            Self::OBJECT_ID_FIELD,
            Self::VERSION_FIELD,
            Self::DIGEST_FIELD,
            Self::OWNER_FIELD,
            Self::OBJECT_TYPE_FIELD,
            Self::HAS_PUBLIC_TRANSFER_FIELD,
            Self::CONTENTS_FIELD,
            Self::PACKAGE_FIELD,
            Self::PREVIOUS_TRANSACTION_FIELD,
            Self::STORAGE_REBATE_FIELD,
            Self::JSON_FIELD,
            Self::BALANCE_FIELD,
        ];
    }
    impl Object {
        pub fn path_builder() -> ObjectFieldPathBuilder {
            ObjectFieldPathBuilder::new()
        }
    }
    pub struct ObjectFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ObjectFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn bcs(mut self) -> BcsFieldPathBuilder {
            self.path.push(Object::BCS_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn object_id(mut self) -> String {
            self.path.push(Object::OBJECT_ID_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(Object::VERSION_FIELD.name);
            self.finish()
        }
        pub fn digest(mut self) -> String {
            self.path.push(Object::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn owner(mut self) -> OwnerFieldPathBuilder {
            self.path.push(Object::OWNER_FIELD.name);
            OwnerFieldPathBuilder::new_with_base(self.path)
        }
        pub fn object_type(mut self) -> String {
            self.path.push(Object::OBJECT_TYPE_FIELD.name);
            self.finish()
        }
        pub fn has_public_transfer(mut self) -> String {
            self.path.push(Object::HAS_PUBLIC_TRANSFER_FIELD.name);
            self.finish()
        }
        pub fn contents(mut self) -> BcsFieldPathBuilder {
            self.path.push(Object::CONTENTS_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn package(mut self) -> PackageFieldPathBuilder {
            self.path.push(Object::PACKAGE_FIELD.name);
            PackageFieldPathBuilder::new_with_base(self.path)
        }
        pub fn previous_transaction(mut self) -> String {
            self.path.push(Object::PREVIOUS_TRANSACTION_FIELD.name);
            self.finish()
        }
        pub fn storage_rebate(mut self) -> String {
            self.path.push(Object::STORAGE_REBATE_FIELD.name);
            self.finish()
        }
        pub fn json(mut self) -> String {
            self.path.push(Object::JSON_FIELD.name);
            self.finish()
        }
        pub fn balance(mut self) -> String {
            self.path.push(Object::BALANCE_FIELD.name);
            self.finish()
        }
    }
    impl ObjectSet {
        pub const OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "objects",
            json_name: "objects",
            number: 1i32,
            message_fields: Some(Object::FIELDS),
        };
    }
    impl MessageFields for ObjectSet {
        const FIELDS: &'static [&'static MessageField] = &[Self::OBJECTS_FIELD];
    }
    impl ObjectSet {
        pub fn path_builder() -> ObjectSetFieldPathBuilder {
            ObjectSetFieldPathBuilder::new()
        }
    }
    pub struct ObjectSetFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ObjectSetFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn objects(mut self) -> ObjectFieldPathBuilder {
            self.path.push(ObjectSet::OBJECTS_FIELD.name);
            ObjectFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ObjectReference {
        pub const OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "object_id",
            json_name: "objectId",
            number: 1i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 2i32,
            message_fields: None,
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for ObjectReference {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECT_ID_FIELD,
            Self::VERSION_FIELD,
            Self::DIGEST_FIELD,
        ];
    }
    impl ObjectReference {
        pub fn path_builder() -> ObjectReferenceFieldPathBuilder {
            ObjectReferenceFieldPathBuilder::new()
        }
    }
    pub struct ObjectReferenceFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ObjectReferenceFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn object_id(mut self) -> String {
            self.path.push(ObjectReference::OBJECT_ID_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(ObjectReference::VERSION_FIELD.name);
            self.finish()
        }
        pub fn digest(mut self) -> String {
            self.path.push(ObjectReference::DIGEST_FIELD.name);
            self.finish()
        }
    }
    impl Owner {
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 1i32,
            message_fields: None,
        };
        pub const ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "address",
            json_name: "address",
            number: 2i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for Owner {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::ADDRESS_FIELD,
            Self::VERSION_FIELD,
        ];
    }
    impl Owner {
        pub fn path_builder() -> OwnerFieldPathBuilder {
            OwnerFieldPathBuilder::new()
        }
    }
    pub struct OwnerFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl OwnerFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kind(mut self) -> String {
            self.path.push(Owner::KIND_FIELD.name);
            self.finish()
        }
        pub fn address(mut self) -> String {
            self.path.push(Owner::ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(Owner::VERSION_FIELD.name);
            self.finish()
        }
    }
    impl ProtocolConfig {
        pub const PROTOCOL_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "protocol_version",
            json_name: "protocolVersion",
            number: 1i32,
            message_fields: None,
        };
        pub const FEATURE_FLAGS_FIELD: &'static MessageField = &MessageField {
            name: "feature_flags",
            json_name: "featureFlags",
            number: 2i32,
            message_fields: None,
        };
        pub const ATTRIBUTES_FIELD: &'static MessageField = &MessageField {
            name: "attributes",
            json_name: "attributes",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for ProtocolConfig {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PROTOCOL_VERSION_FIELD,
            Self::FEATURE_FLAGS_FIELD,
            Self::ATTRIBUTES_FIELD,
        ];
    }
    impl ProtocolConfig {
        pub fn path_builder() -> ProtocolConfigFieldPathBuilder {
            ProtocolConfigFieldPathBuilder::new()
        }
    }
    pub struct ProtocolConfigFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ProtocolConfigFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn protocol_version(mut self) -> String {
            self.path.push(ProtocolConfig::PROTOCOL_VERSION_FIELD.name);
            self.finish()
        }
        pub fn feature_flags(mut self) -> String {
            self.path.push(ProtocolConfig::FEATURE_FLAGS_FIELD.name);
            self.finish()
        }
        pub fn attributes(mut self) -> String {
            self.path.push(ProtocolConfig::ATTRIBUTES_FIELD.name);
            self.finish()
        }
    }
    impl UserSignature {
        pub const BCS_FIELD: &'static MessageField = &MessageField {
            name: "bcs",
            json_name: "bcs",
            number: 1i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const SCHEME_FIELD: &'static MessageField = &MessageField {
            name: "scheme",
            json_name: "scheme",
            number: 2i32,
            message_fields: None,
        };
        pub const SIMPLE_FIELD: &'static MessageField = &MessageField {
            name: "simple",
            json_name: "simple",
            number: 3i32,
            message_fields: Some(SimpleSignature::FIELDS),
        };
        pub const MULTISIG_FIELD: &'static MessageField = &MessageField {
            name: "multisig",
            json_name: "multisig",
            number: 4i32,
            message_fields: Some(MultisigAggregatedSignature::FIELDS),
        };
        pub const ZKLOGIN_FIELD: &'static MessageField = &MessageField {
            name: "zklogin",
            json_name: "zklogin",
            number: 5i32,
            message_fields: Some(ZkLoginAuthenticator::FIELDS),
        };
        pub const PASSKEY_FIELD: &'static MessageField = &MessageField {
            name: "passkey",
            json_name: "passkey",
            number: 6i32,
            message_fields: Some(PasskeyAuthenticator::FIELDS),
        };
    }
    impl MessageFields for UserSignature {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::BCS_FIELD,
            Self::SCHEME_FIELD,
            Self::SIMPLE_FIELD,
            Self::MULTISIG_FIELD,
            Self::ZKLOGIN_FIELD,
            Self::PASSKEY_FIELD,
        ];
    }
    impl UserSignature {
        pub fn path_builder() -> UserSignatureFieldPathBuilder {
            UserSignatureFieldPathBuilder::new()
        }
    }
    pub struct UserSignatureFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl UserSignatureFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn bcs(mut self) -> BcsFieldPathBuilder {
            self.path.push(UserSignature::BCS_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn scheme(mut self) -> String {
            self.path.push(UserSignature::SCHEME_FIELD.name);
            self.finish()
        }
        pub fn simple(mut self) -> SimpleSignatureFieldPathBuilder {
            self.path.push(UserSignature::SIMPLE_FIELD.name);
            SimpleSignatureFieldPathBuilder::new_with_base(self.path)
        }
        pub fn multisig(mut self) -> MultisigAggregatedSignatureFieldPathBuilder {
            self.path.push(UserSignature::MULTISIG_FIELD.name);
            MultisigAggregatedSignatureFieldPathBuilder::new_with_base(self.path)
        }
        pub fn zklogin(mut self) -> ZkLoginAuthenticatorFieldPathBuilder {
            self.path.push(UserSignature::ZKLOGIN_FIELD.name);
            ZkLoginAuthenticatorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn passkey(mut self) -> PasskeyAuthenticatorFieldPathBuilder {
            self.path.push(UserSignature::PASSKEY_FIELD.name);
            PasskeyAuthenticatorFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl SimpleSignature {
        pub const SCHEME_FIELD: &'static MessageField = &MessageField {
            name: "scheme",
            json_name: "scheme",
            number: 1i32,
            message_fields: None,
        };
        pub const SIGNATURE_FIELD: &'static MessageField = &MessageField {
            name: "signature",
            json_name: "signature",
            number: 2i32,
            message_fields: None,
        };
        pub const PUBLIC_KEY_FIELD: &'static MessageField = &MessageField {
            name: "public_key",
            json_name: "publicKey",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for SimpleSignature {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::SCHEME_FIELD,
            Self::SIGNATURE_FIELD,
            Self::PUBLIC_KEY_FIELD,
        ];
    }
    impl SimpleSignature {
        pub fn path_builder() -> SimpleSignatureFieldPathBuilder {
            SimpleSignatureFieldPathBuilder::new()
        }
    }
    pub struct SimpleSignatureFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SimpleSignatureFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn scheme(mut self) -> String {
            self.path.push(SimpleSignature::SCHEME_FIELD.name);
            self.finish()
        }
        pub fn signature(mut self) -> String {
            self.path.push(SimpleSignature::SIGNATURE_FIELD.name);
            self.finish()
        }
        pub fn public_key(mut self) -> String {
            self.path.push(SimpleSignature::PUBLIC_KEY_FIELD.name);
            self.finish()
        }
    }
    impl ZkLoginPublicIdentifier {
        pub const ISS_FIELD: &'static MessageField = &MessageField {
            name: "iss",
            json_name: "iss",
            number: 1i32,
            message_fields: None,
        };
        pub const ADDRESS_SEED_FIELD: &'static MessageField = &MessageField {
            name: "address_seed",
            json_name: "addressSeed",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for ZkLoginPublicIdentifier {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ISS_FIELD,
            Self::ADDRESS_SEED_FIELD,
        ];
    }
    impl ZkLoginPublicIdentifier {
        pub fn path_builder() -> ZkLoginPublicIdentifierFieldPathBuilder {
            ZkLoginPublicIdentifierFieldPathBuilder::new()
        }
    }
    pub struct ZkLoginPublicIdentifierFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ZkLoginPublicIdentifierFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn iss(mut self) -> String {
            self.path.push(ZkLoginPublicIdentifier::ISS_FIELD.name);
            self.finish()
        }
        pub fn address_seed(mut self) -> String {
            self.path.push(ZkLoginPublicIdentifier::ADDRESS_SEED_FIELD.name);
            self.finish()
        }
    }
    impl MultisigMemberPublicKey {
        pub const SCHEME_FIELD: &'static MessageField = &MessageField {
            name: "scheme",
            json_name: "scheme",
            number: 1i32,
            message_fields: None,
        };
        pub const PUBLIC_KEY_FIELD: &'static MessageField = &MessageField {
            name: "public_key",
            json_name: "publicKey",
            number: 2i32,
            message_fields: None,
        };
        pub const ZKLOGIN_FIELD: &'static MessageField = &MessageField {
            name: "zklogin",
            json_name: "zklogin",
            number: 3i32,
            message_fields: Some(ZkLoginPublicIdentifier::FIELDS),
        };
    }
    impl MessageFields for MultisigMemberPublicKey {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::SCHEME_FIELD,
            Self::PUBLIC_KEY_FIELD,
            Self::ZKLOGIN_FIELD,
        ];
    }
    impl MultisigMemberPublicKey {
        pub fn path_builder() -> MultisigMemberPublicKeyFieldPathBuilder {
            MultisigMemberPublicKeyFieldPathBuilder::new()
        }
    }
    pub struct MultisigMemberPublicKeyFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MultisigMemberPublicKeyFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn scheme(mut self) -> String {
            self.path.push(MultisigMemberPublicKey::SCHEME_FIELD.name);
            self.finish()
        }
        pub fn public_key(mut self) -> String {
            self.path.push(MultisigMemberPublicKey::PUBLIC_KEY_FIELD.name);
            self.finish()
        }
        pub fn zklogin(mut self) -> ZkLoginPublicIdentifierFieldPathBuilder {
            self.path.push(MultisigMemberPublicKey::ZKLOGIN_FIELD.name);
            ZkLoginPublicIdentifierFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MultisigMember {
        pub const PUBLIC_KEY_FIELD: &'static MessageField = &MessageField {
            name: "public_key",
            json_name: "publicKey",
            number: 1i32,
            message_fields: Some(MultisigMemberPublicKey::FIELDS),
        };
        pub const WEIGHT_FIELD: &'static MessageField = &MessageField {
            name: "weight",
            json_name: "weight",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for MultisigMember {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PUBLIC_KEY_FIELD,
            Self::WEIGHT_FIELD,
        ];
    }
    impl MultisigMember {
        pub fn path_builder() -> MultisigMemberFieldPathBuilder {
            MultisigMemberFieldPathBuilder::new()
        }
    }
    pub struct MultisigMemberFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MultisigMemberFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn public_key(mut self) -> MultisigMemberPublicKeyFieldPathBuilder {
            self.path.push(MultisigMember::PUBLIC_KEY_FIELD.name);
            MultisigMemberPublicKeyFieldPathBuilder::new_with_base(self.path)
        }
        pub fn weight(mut self) -> String {
            self.path.push(MultisigMember::WEIGHT_FIELD.name);
            self.finish()
        }
    }
    impl MultisigCommittee {
        pub const MEMBERS_FIELD: &'static MessageField = &MessageField {
            name: "members",
            json_name: "members",
            number: 1i32,
            message_fields: Some(MultisigMember::FIELDS),
        };
        pub const THRESHOLD_FIELD: &'static MessageField = &MessageField {
            name: "threshold",
            json_name: "threshold",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for MultisigCommittee {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::MEMBERS_FIELD,
            Self::THRESHOLD_FIELD,
        ];
    }
    impl MultisigCommittee {
        pub fn path_builder() -> MultisigCommitteeFieldPathBuilder {
            MultisigCommitteeFieldPathBuilder::new()
        }
    }
    pub struct MultisigCommitteeFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MultisigCommitteeFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn members(mut self) -> MultisigMemberFieldPathBuilder {
            self.path.push(MultisigCommittee::MEMBERS_FIELD.name);
            MultisigMemberFieldPathBuilder::new_with_base(self.path)
        }
        pub fn threshold(mut self) -> String {
            self.path.push(MultisigCommittee::THRESHOLD_FIELD.name);
            self.finish()
        }
    }
    impl MultisigAggregatedSignature {
        pub const SIGNATURES_FIELD: &'static MessageField = &MessageField {
            name: "signatures",
            json_name: "signatures",
            number: 1i32,
            message_fields: Some(MultisigMemberSignature::FIELDS),
        };
        pub const BITMAP_FIELD: &'static MessageField = &MessageField {
            name: "bitmap",
            json_name: "bitmap",
            number: 2i32,
            message_fields: None,
        };
        pub const LEGACY_BITMAP_FIELD: &'static MessageField = &MessageField {
            name: "legacy_bitmap",
            json_name: "legacyBitmap",
            number: 3i32,
            message_fields: None,
        };
        pub const COMMITTEE_FIELD: &'static MessageField = &MessageField {
            name: "committee",
            json_name: "committee",
            number: 4i32,
            message_fields: Some(MultisigCommittee::FIELDS),
        };
    }
    impl MessageFields for MultisigAggregatedSignature {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::SIGNATURES_FIELD,
            Self::BITMAP_FIELD,
            Self::LEGACY_BITMAP_FIELD,
            Self::COMMITTEE_FIELD,
        ];
    }
    impl MultisigAggregatedSignature {
        pub fn path_builder() -> MultisigAggregatedSignatureFieldPathBuilder {
            MultisigAggregatedSignatureFieldPathBuilder::new()
        }
    }
    pub struct MultisigAggregatedSignatureFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MultisigAggregatedSignatureFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn signatures(mut self) -> MultisigMemberSignatureFieldPathBuilder {
            self.path.push(MultisigAggregatedSignature::SIGNATURES_FIELD.name);
            MultisigMemberSignatureFieldPathBuilder::new_with_base(self.path)
        }
        pub fn bitmap(mut self) -> String {
            self.path.push(MultisigAggregatedSignature::BITMAP_FIELD.name);
            self.finish()
        }
        pub fn legacy_bitmap(mut self) -> String {
            self.path.push(MultisigAggregatedSignature::LEGACY_BITMAP_FIELD.name);
            self.finish()
        }
        pub fn committee(mut self) -> MultisigCommitteeFieldPathBuilder {
            self.path.push(MultisigAggregatedSignature::COMMITTEE_FIELD.name);
            MultisigCommitteeFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MultisigMemberSignature {
        pub const SCHEME_FIELD: &'static MessageField = &MessageField {
            name: "scheme",
            json_name: "scheme",
            number: 1i32,
            message_fields: None,
        };
        pub const SIGNATURE_FIELD: &'static MessageField = &MessageField {
            name: "signature",
            json_name: "signature",
            number: 2i32,
            message_fields: None,
        };
        pub const ZKLOGIN_FIELD: &'static MessageField = &MessageField {
            name: "zklogin",
            json_name: "zklogin",
            number: 3i32,
            message_fields: Some(ZkLoginAuthenticator::FIELDS),
        };
        pub const PASSKEY_FIELD: &'static MessageField = &MessageField {
            name: "passkey",
            json_name: "passkey",
            number: 4i32,
            message_fields: Some(PasskeyAuthenticator::FIELDS),
        };
    }
    impl MessageFields for MultisigMemberSignature {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::SCHEME_FIELD,
            Self::SIGNATURE_FIELD,
            Self::ZKLOGIN_FIELD,
            Self::PASSKEY_FIELD,
        ];
    }
    impl MultisigMemberSignature {
        pub fn path_builder() -> MultisigMemberSignatureFieldPathBuilder {
            MultisigMemberSignatureFieldPathBuilder::new()
        }
    }
    pub struct MultisigMemberSignatureFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MultisigMemberSignatureFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn scheme(mut self) -> String {
            self.path.push(MultisigMemberSignature::SCHEME_FIELD.name);
            self.finish()
        }
        pub fn signature(mut self) -> String {
            self.path.push(MultisigMemberSignature::SIGNATURE_FIELD.name);
            self.finish()
        }
        pub fn zklogin(mut self) -> ZkLoginAuthenticatorFieldPathBuilder {
            self.path.push(MultisigMemberSignature::ZKLOGIN_FIELD.name);
            ZkLoginAuthenticatorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn passkey(mut self) -> PasskeyAuthenticatorFieldPathBuilder {
            self.path.push(MultisigMemberSignature::PASSKEY_FIELD.name);
            PasskeyAuthenticatorFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ZkLoginAuthenticator {
        pub const INPUTS_FIELD: &'static MessageField = &MessageField {
            name: "inputs",
            json_name: "inputs",
            number: 1i32,
            message_fields: Some(ZkLoginInputs::FIELDS),
        };
        pub const MAX_EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "max_epoch",
            json_name: "maxEpoch",
            number: 2i32,
            message_fields: None,
        };
        pub const SIGNATURE_FIELD: &'static MessageField = &MessageField {
            name: "signature",
            json_name: "signature",
            number: 3i32,
            message_fields: Some(SimpleSignature::FIELDS),
        };
        pub const PUBLIC_IDENTIFIER_FIELD: &'static MessageField = &MessageField {
            name: "public_identifier",
            json_name: "publicIdentifier",
            number: 4i32,
            message_fields: Some(ZkLoginPublicIdentifier::FIELDS),
        };
        pub const JWK_ID_FIELD: &'static MessageField = &MessageField {
            name: "jwk_id",
            json_name: "jwkId",
            number: 5i32,
            message_fields: Some(JwkId::FIELDS),
        };
    }
    impl MessageFields for ZkLoginAuthenticator {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::INPUTS_FIELD,
            Self::MAX_EPOCH_FIELD,
            Self::SIGNATURE_FIELD,
            Self::PUBLIC_IDENTIFIER_FIELD,
            Self::JWK_ID_FIELD,
        ];
    }
    impl ZkLoginAuthenticator {
        pub fn path_builder() -> ZkLoginAuthenticatorFieldPathBuilder {
            ZkLoginAuthenticatorFieldPathBuilder::new()
        }
    }
    pub struct ZkLoginAuthenticatorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ZkLoginAuthenticatorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn inputs(mut self) -> ZkLoginInputsFieldPathBuilder {
            self.path.push(ZkLoginAuthenticator::INPUTS_FIELD.name);
            ZkLoginInputsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn max_epoch(mut self) -> String {
            self.path.push(ZkLoginAuthenticator::MAX_EPOCH_FIELD.name);
            self.finish()
        }
        pub fn signature(mut self) -> SimpleSignatureFieldPathBuilder {
            self.path.push(ZkLoginAuthenticator::SIGNATURE_FIELD.name);
            SimpleSignatureFieldPathBuilder::new_with_base(self.path)
        }
        pub fn public_identifier(mut self) -> ZkLoginPublicIdentifierFieldPathBuilder {
            self.path.push(ZkLoginAuthenticator::PUBLIC_IDENTIFIER_FIELD.name);
            ZkLoginPublicIdentifierFieldPathBuilder::new_with_base(self.path)
        }
        pub fn jwk_id(mut self) -> JwkIdFieldPathBuilder {
            self.path.push(ZkLoginAuthenticator::JWK_ID_FIELD.name);
            JwkIdFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ZkLoginInputs {
        pub const PROOF_POINTS_FIELD: &'static MessageField = &MessageField {
            name: "proof_points",
            json_name: "proofPoints",
            number: 1i32,
            message_fields: Some(ZkLoginProof::FIELDS),
        };
        pub const ISS_BASE64_DETAILS_FIELD: &'static MessageField = &MessageField {
            name: "iss_base64_details",
            json_name: "issBase64Details",
            number: 2i32,
            message_fields: Some(ZkLoginClaim::FIELDS),
        };
        pub const HEADER_BASE64_FIELD: &'static MessageField = &MessageField {
            name: "header_base64",
            json_name: "headerBase64",
            number: 3i32,
            message_fields: None,
        };
        pub const ADDRESS_SEED_FIELD: &'static MessageField = &MessageField {
            name: "address_seed",
            json_name: "addressSeed",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for ZkLoginInputs {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PROOF_POINTS_FIELD,
            Self::ISS_BASE64_DETAILS_FIELD,
            Self::HEADER_BASE64_FIELD,
            Self::ADDRESS_SEED_FIELD,
        ];
    }
    impl ZkLoginInputs {
        pub fn path_builder() -> ZkLoginInputsFieldPathBuilder {
            ZkLoginInputsFieldPathBuilder::new()
        }
    }
    pub struct ZkLoginInputsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ZkLoginInputsFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn proof_points(mut self) -> ZkLoginProofFieldPathBuilder {
            self.path.push(ZkLoginInputs::PROOF_POINTS_FIELD.name);
            ZkLoginProofFieldPathBuilder::new_with_base(self.path)
        }
        pub fn iss_base64_details(mut self) -> ZkLoginClaimFieldPathBuilder {
            self.path.push(ZkLoginInputs::ISS_BASE64_DETAILS_FIELD.name);
            ZkLoginClaimFieldPathBuilder::new_with_base(self.path)
        }
        pub fn header_base64(mut self) -> String {
            self.path.push(ZkLoginInputs::HEADER_BASE64_FIELD.name);
            self.finish()
        }
        pub fn address_seed(mut self) -> String {
            self.path.push(ZkLoginInputs::ADDRESS_SEED_FIELD.name);
            self.finish()
        }
    }
    impl ZkLoginProof {
        pub const A_FIELD: &'static MessageField = &MessageField {
            name: "a",
            json_name: "a",
            number: 1i32,
            message_fields: Some(CircomG1::FIELDS),
        };
        pub const B_FIELD: &'static MessageField = &MessageField {
            name: "b",
            json_name: "b",
            number: 2i32,
            message_fields: Some(CircomG2::FIELDS),
        };
        pub const C_FIELD: &'static MessageField = &MessageField {
            name: "c",
            json_name: "c",
            number: 3i32,
            message_fields: Some(CircomG1::FIELDS),
        };
    }
    impl MessageFields for ZkLoginProof {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::A_FIELD,
            Self::B_FIELD,
            Self::C_FIELD,
        ];
    }
    impl ZkLoginProof {
        pub fn path_builder() -> ZkLoginProofFieldPathBuilder {
            ZkLoginProofFieldPathBuilder::new()
        }
    }
    pub struct ZkLoginProofFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ZkLoginProofFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn a(mut self) -> CircomG1FieldPathBuilder {
            self.path.push(ZkLoginProof::A_FIELD.name);
            CircomG1FieldPathBuilder::new_with_base(self.path)
        }
        pub fn b(mut self) -> CircomG2FieldPathBuilder {
            self.path.push(ZkLoginProof::B_FIELD.name);
            CircomG2FieldPathBuilder::new_with_base(self.path)
        }
        pub fn c(mut self) -> CircomG1FieldPathBuilder {
            self.path.push(ZkLoginProof::C_FIELD.name);
            CircomG1FieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ZkLoginClaim {
        pub const VALUE_FIELD: &'static MessageField = &MessageField {
            name: "value",
            json_name: "value",
            number: 1i32,
            message_fields: None,
        };
        pub const INDEX_MOD_4_FIELD: &'static MessageField = &MessageField {
            name: "index_mod_4",
            json_name: "indexMod4",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for ZkLoginClaim {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::VALUE_FIELD,
            Self::INDEX_MOD_4_FIELD,
        ];
    }
    impl ZkLoginClaim {
        pub fn path_builder() -> ZkLoginClaimFieldPathBuilder {
            ZkLoginClaimFieldPathBuilder::new()
        }
    }
    pub struct ZkLoginClaimFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ZkLoginClaimFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn value(mut self) -> String {
            self.path.push(ZkLoginClaim::VALUE_FIELD.name);
            self.finish()
        }
        pub fn index_mod_4(mut self) -> String {
            self.path.push(ZkLoginClaim::INDEX_MOD_4_FIELD.name);
            self.finish()
        }
    }
    impl CircomG1 {
        pub const E0_FIELD: &'static MessageField = &MessageField {
            name: "e0",
            json_name: "e0",
            number: 1i32,
            message_fields: None,
        };
        pub const E1_FIELD: &'static MessageField = &MessageField {
            name: "e1",
            json_name: "e1",
            number: 2i32,
            message_fields: None,
        };
        pub const E2_FIELD: &'static MessageField = &MessageField {
            name: "e2",
            json_name: "e2",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for CircomG1 {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::E0_FIELD,
            Self::E1_FIELD,
            Self::E2_FIELD,
        ];
    }
    impl CircomG1 {
        pub fn path_builder() -> CircomG1FieldPathBuilder {
            CircomG1FieldPathBuilder::new()
        }
    }
    pub struct CircomG1FieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CircomG1FieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn e0(mut self) -> String {
            self.path.push(CircomG1::E0_FIELD.name);
            self.finish()
        }
        pub fn e1(mut self) -> String {
            self.path.push(CircomG1::E1_FIELD.name);
            self.finish()
        }
        pub fn e2(mut self) -> String {
            self.path.push(CircomG1::E2_FIELD.name);
            self.finish()
        }
    }
    impl CircomG2 {
        pub const E00_FIELD: &'static MessageField = &MessageField {
            name: "e00",
            json_name: "e00",
            number: 1i32,
            message_fields: None,
        };
        pub const E01_FIELD: &'static MessageField = &MessageField {
            name: "e01",
            json_name: "e01",
            number: 2i32,
            message_fields: None,
        };
        pub const E10_FIELD: &'static MessageField = &MessageField {
            name: "e10",
            json_name: "e10",
            number: 3i32,
            message_fields: None,
        };
        pub const E11_FIELD: &'static MessageField = &MessageField {
            name: "e11",
            json_name: "e11",
            number: 4i32,
            message_fields: None,
        };
        pub const E20_FIELD: &'static MessageField = &MessageField {
            name: "e20",
            json_name: "e20",
            number: 5i32,
            message_fields: None,
        };
        pub const E21_FIELD: &'static MessageField = &MessageField {
            name: "e21",
            json_name: "e21",
            number: 6i32,
            message_fields: None,
        };
    }
    impl MessageFields for CircomG2 {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::E00_FIELD,
            Self::E01_FIELD,
            Self::E10_FIELD,
            Self::E11_FIELD,
            Self::E20_FIELD,
            Self::E21_FIELD,
        ];
    }
    impl CircomG2 {
        pub fn path_builder() -> CircomG2FieldPathBuilder {
            CircomG2FieldPathBuilder::new()
        }
    }
    pub struct CircomG2FieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CircomG2FieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn e00(mut self) -> String {
            self.path.push(CircomG2::E00_FIELD.name);
            self.finish()
        }
        pub fn e01(mut self) -> String {
            self.path.push(CircomG2::E01_FIELD.name);
            self.finish()
        }
        pub fn e10(mut self) -> String {
            self.path.push(CircomG2::E10_FIELD.name);
            self.finish()
        }
        pub fn e11(mut self) -> String {
            self.path.push(CircomG2::E11_FIELD.name);
            self.finish()
        }
        pub fn e20(mut self) -> String {
            self.path.push(CircomG2::E20_FIELD.name);
            self.finish()
        }
        pub fn e21(mut self) -> String {
            self.path.push(CircomG2::E21_FIELD.name);
            self.finish()
        }
    }
    impl PasskeyAuthenticator {
        pub const AUTHENTICATOR_DATA_FIELD: &'static MessageField = &MessageField {
            name: "authenticator_data",
            json_name: "authenticatorData",
            number: 1i32,
            message_fields: None,
        };
        pub const CLIENT_DATA_JSON_FIELD: &'static MessageField = &MessageField {
            name: "client_data_json",
            json_name: "clientDataJson",
            number: 2i32,
            message_fields: None,
        };
        pub const SIGNATURE_FIELD: &'static MessageField = &MessageField {
            name: "signature",
            json_name: "signature",
            number: 3i32,
            message_fields: Some(SimpleSignature::FIELDS),
        };
    }
    impl MessageFields for PasskeyAuthenticator {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::AUTHENTICATOR_DATA_FIELD,
            Self::CLIENT_DATA_JSON_FIELD,
            Self::SIGNATURE_FIELD,
        ];
    }
    impl PasskeyAuthenticator {
        pub fn path_builder() -> PasskeyAuthenticatorFieldPathBuilder {
            PasskeyAuthenticatorFieldPathBuilder::new()
        }
    }
    pub struct PasskeyAuthenticatorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl PasskeyAuthenticatorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn authenticator_data(mut self) -> String {
            self.path.push(PasskeyAuthenticator::AUTHENTICATOR_DATA_FIELD.name);
            self.finish()
        }
        pub fn client_data_json(mut self) -> String {
            self.path.push(PasskeyAuthenticator::CLIENT_DATA_JSON_FIELD.name);
            self.finish()
        }
        pub fn signature(mut self) -> SimpleSignatureFieldPathBuilder {
            self.path.push(PasskeyAuthenticator::SIGNATURE_FIELD.name);
            SimpleSignatureFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ValidatorCommittee {
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 1i32,
            message_fields: None,
        };
        pub const MEMBERS_FIELD: &'static MessageField = &MessageField {
            name: "members",
            json_name: "members",
            number: 2i32,
            message_fields: Some(ValidatorCommitteeMember::FIELDS),
        };
    }
    impl MessageFields for ValidatorCommittee {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EPOCH_FIELD,
            Self::MEMBERS_FIELD,
        ];
    }
    impl ValidatorCommittee {
        pub fn path_builder() -> ValidatorCommitteeFieldPathBuilder {
            ValidatorCommitteeFieldPathBuilder::new()
        }
    }
    pub struct ValidatorCommitteeFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ValidatorCommitteeFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn epoch(mut self) -> String {
            self.path.push(ValidatorCommittee::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn members(mut self) -> ValidatorCommitteeMemberFieldPathBuilder {
            self.path.push(ValidatorCommittee::MEMBERS_FIELD.name);
            ValidatorCommitteeMemberFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ValidatorCommitteeMember {
        pub const PUBLIC_KEY_FIELD: &'static MessageField = &MessageField {
            name: "public_key",
            json_name: "publicKey",
            number: 1i32,
            message_fields: None,
        };
        pub const WEIGHT_FIELD: &'static MessageField = &MessageField {
            name: "weight",
            json_name: "weight",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for ValidatorCommitteeMember {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PUBLIC_KEY_FIELD,
            Self::WEIGHT_FIELD,
        ];
    }
    impl ValidatorCommitteeMember {
        pub fn path_builder() -> ValidatorCommitteeMemberFieldPathBuilder {
            ValidatorCommitteeMemberFieldPathBuilder::new()
        }
    }
    pub struct ValidatorCommitteeMemberFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ValidatorCommitteeMemberFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn public_key(mut self) -> String {
            self.path.push(ValidatorCommitteeMember::PUBLIC_KEY_FIELD.name);
            self.finish()
        }
        pub fn weight(mut self) -> String {
            self.path.push(ValidatorCommitteeMember::WEIGHT_FIELD.name);
            self.finish()
        }
    }
    impl ValidatorAggregatedSignature {
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 1i32,
            message_fields: None,
        };
        pub const SIGNATURE_FIELD: &'static MessageField = &MessageField {
            name: "signature",
            json_name: "signature",
            number: 2i32,
            message_fields: None,
        };
        pub const BITMAP_FIELD: &'static MessageField = &MessageField {
            name: "bitmap",
            json_name: "bitmap",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for ValidatorAggregatedSignature {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EPOCH_FIELD,
            Self::SIGNATURE_FIELD,
            Self::BITMAP_FIELD,
        ];
    }
    impl ValidatorAggregatedSignature {
        pub fn path_builder() -> ValidatorAggregatedSignatureFieldPathBuilder {
            ValidatorAggregatedSignatureFieldPathBuilder::new()
        }
    }
    pub struct ValidatorAggregatedSignatureFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ValidatorAggregatedSignatureFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn epoch(mut self) -> String {
            self.path.push(ValidatorAggregatedSignature::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn signature(mut self) -> String {
            self.path.push(ValidatorAggregatedSignature::SIGNATURE_FIELD.name);
            self.finish()
        }
        pub fn bitmap(mut self) -> String {
            self.path.push(ValidatorAggregatedSignature::BITMAP_FIELD.name);
            self.finish()
        }
    }
    impl VerifySignatureRequest {
        pub const MESSAGE_FIELD: &'static MessageField = &MessageField {
            name: "message",
            json_name: "message",
            number: 1i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const SIGNATURE_FIELD: &'static MessageField = &MessageField {
            name: "signature",
            json_name: "signature",
            number: 2i32,
            message_fields: Some(UserSignature::FIELDS),
        };
        pub const ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "address",
            json_name: "address",
            number: 3i32,
            message_fields: None,
        };
        pub const JWKS_FIELD: &'static MessageField = &MessageField {
            name: "jwks",
            json_name: "jwks",
            number: 4i32,
            message_fields: Some(ActiveJwk::FIELDS),
        };
    }
    impl MessageFields for VerifySignatureRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::MESSAGE_FIELD,
            Self::SIGNATURE_FIELD,
            Self::ADDRESS_FIELD,
            Self::JWKS_FIELD,
        ];
    }
    impl VerifySignatureRequest {
        pub fn path_builder() -> VerifySignatureRequestFieldPathBuilder {
            VerifySignatureRequestFieldPathBuilder::new()
        }
    }
    pub struct VerifySignatureRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl VerifySignatureRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn message(mut self) -> BcsFieldPathBuilder {
            self.path.push(VerifySignatureRequest::MESSAGE_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn signature(mut self) -> UserSignatureFieldPathBuilder {
            self.path.push(VerifySignatureRequest::SIGNATURE_FIELD.name);
            UserSignatureFieldPathBuilder::new_with_base(self.path)
        }
        pub fn address(mut self) -> String {
            self.path.push(VerifySignatureRequest::ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn jwks(mut self) -> ActiveJwkFieldPathBuilder {
            self.path.push(VerifySignatureRequest::JWKS_FIELD.name);
            ActiveJwkFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl VerifySignatureResponse {
        pub const IS_VALID_FIELD: &'static MessageField = &MessageField {
            name: "is_valid",
            json_name: "isValid",
            number: 1i32,
            message_fields: None,
        };
        pub const REASON_FIELD: &'static MessageField = &MessageField {
            name: "reason",
            json_name: "reason",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for VerifySignatureResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::IS_VALID_FIELD,
            Self::REASON_FIELD,
        ];
    }
    impl VerifySignatureResponse {
        pub fn path_builder() -> VerifySignatureResponseFieldPathBuilder {
            VerifySignatureResponseFieldPathBuilder::new()
        }
    }
    pub struct VerifySignatureResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl VerifySignatureResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn is_valid(mut self) -> String {
            self.path.push(VerifySignatureResponse::IS_VALID_FIELD.name);
            self.finish()
        }
        pub fn reason(mut self) -> String {
            self.path.push(VerifySignatureResponse::REASON_FIELD.name);
            self.finish()
        }
    }
    impl GetCoinInfoRequest {
        pub const COIN_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "coin_type",
            json_name: "coinType",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetCoinInfoRequest {
        const FIELDS: &'static [&'static MessageField] = &[Self::COIN_TYPE_FIELD];
    }
    impl GetCoinInfoRequest {
        pub fn path_builder() -> GetCoinInfoRequestFieldPathBuilder {
            GetCoinInfoRequestFieldPathBuilder::new()
        }
    }
    pub struct GetCoinInfoRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetCoinInfoRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn coin_type(mut self) -> String {
            self.path.push(GetCoinInfoRequest::COIN_TYPE_FIELD.name);
            self.finish()
        }
    }
    impl GetCoinInfoResponse {
        pub const COIN_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "coin_type",
            json_name: "coinType",
            number: 1i32,
            message_fields: None,
        };
        pub const METADATA_FIELD: &'static MessageField = &MessageField {
            name: "metadata",
            json_name: "metadata",
            number: 2i32,
            message_fields: Some(CoinMetadata::FIELDS),
        };
        pub const TREASURY_FIELD: &'static MessageField = &MessageField {
            name: "treasury",
            json_name: "treasury",
            number: 3i32,
            message_fields: Some(CoinTreasury::FIELDS),
        };
        pub const REGULATED_METADATA_FIELD: &'static MessageField = &MessageField {
            name: "regulated_metadata",
            json_name: "regulatedMetadata",
            number: 4i32,
            message_fields: Some(RegulatedCoinMetadata::FIELDS),
        };
    }
    impl MessageFields for GetCoinInfoResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::COIN_TYPE_FIELD,
            Self::METADATA_FIELD,
            Self::TREASURY_FIELD,
            Self::REGULATED_METADATA_FIELD,
        ];
    }
    impl GetCoinInfoResponse {
        pub fn path_builder() -> GetCoinInfoResponseFieldPathBuilder {
            GetCoinInfoResponseFieldPathBuilder::new()
        }
    }
    pub struct GetCoinInfoResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetCoinInfoResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn coin_type(mut self) -> String {
            self.path.push(GetCoinInfoResponse::COIN_TYPE_FIELD.name);
            self.finish()
        }
        pub fn metadata(mut self) -> CoinMetadataFieldPathBuilder {
            self.path.push(GetCoinInfoResponse::METADATA_FIELD.name);
            CoinMetadataFieldPathBuilder::new_with_base(self.path)
        }
        pub fn treasury(mut self) -> CoinTreasuryFieldPathBuilder {
            self.path.push(GetCoinInfoResponse::TREASURY_FIELD.name);
            CoinTreasuryFieldPathBuilder::new_with_base(self.path)
        }
        pub fn regulated_metadata(mut self) -> RegulatedCoinMetadataFieldPathBuilder {
            self.path.push(GetCoinInfoResponse::REGULATED_METADATA_FIELD.name);
            RegulatedCoinMetadataFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl CoinMetadata {
        pub const ID_FIELD: &'static MessageField = &MessageField {
            name: "id",
            json_name: "id",
            number: 1i32,
            message_fields: None,
        };
        pub const DECIMALS_FIELD: &'static MessageField = &MessageField {
            name: "decimals",
            json_name: "decimals",
            number: 2i32,
            message_fields: None,
        };
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 3i32,
            message_fields: None,
        };
        pub const SYMBOL_FIELD: &'static MessageField = &MessageField {
            name: "symbol",
            json_name: "symbol",
            number: 4i32,
            message_fields: None,
        };
        pub const DESCRIPTION_FIELD: &'static MessageField = &MessageField {
            name: "description",
            json_name: "description",
            number: 5i32,
            message_fields: None,
        };
        pub const ICON_URL_FIELD: &'static MessageField = &MessageField {
            name: "icon_url",
            json_name: "iconUrl",
            number: 6i32,
            message_fields: None,
        };
        pub const METADATA_CAP_ID_FIELD: &'static MessageField = &MessageField {
            name: "metadata_cap_id",
            json_name: "metadataCapId",
            number: 7i32,
            message_fields: None,
        };
        pub const METADATA_CAP_STATE_FIELD: &'static MessageField = &MessageField {
            name: "metadata_cap_state",
            json_name: "metadataCapState",
            number: 8i32,
            message_fields: None,
        };
    }
    impl MessageFields for CoinMetadata {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ID_FIELD,
            Self::DECIMALS_FIELD,
            Self::NAME_FIELD,
            Self::SYMBOL_FIELD,
            Self::DESCRIPTION_FIELD,
            Self::ICON_URL_FIELD,
            Self::METADATA_CAP_ID_FIELD,
            Self::METADATA_CAP_STATE_FIELD,
        ];
    }
    impl CoinMetadata {
        pub fn path_builder() -> CoinMetadataFieldPathBuilder {
            CoinMetadataFieldPathBuilder::new()
        }
    }
    pub struct CoinMetadataFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CoinMetadataFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn id(mut self) -> String {
            self.path.push(CoinMetadata::ID_FIELD.name);
            self.finish()
        }
        pub fn decimals(mut self) -> String {
            self.path.push(CoinMetadata::DECIMALS_FIELD.name);
            self.finish()
        }
        pub fn name(mut self) -> String {
            self.path.push(CoinMetadata::NAME_FIELD.name);
            self.finish()
        }
        pub fn symbol(mut self) -> String {
            self.path.push(CoinMetadata::SYMBOL_FIELD.name);
            self.finish()
        }
        pub fn description(mut self) -> String {
            self.path.push(CoinMetadata::DESCRIPTION_FIELD.name);
            self.finish()
        }
        pub fn icon_url(mut self) -> String {
            self.path.push(CoinMetadata::ICON_URL_FIELD.name);
            self.finish()
        }
        pub fn metadata_cap_id(mut self) -> String {
            self.path.push(CoinMetadata::METADATA_CAP_ID_FIELD.name);
            self.finish()
        }
        pub fn metadata_cap_state(mut self) -> String {
            self.path.push(CoinMetadata::METADATA_CAP_STATE_FIELD.name);
            self.finish()
        }
    }
    impl CoinTreasury {
        pub const ID_FIELD: &'static MessageField = &MessageField {
            name: "id",
            json_name: "id",
            number: 1i32,
            message_fields: None,
        };
        pub const TOTAL_SUPPLY_FIELD: &'static MessageField = &MessageField {
            name: "total_supply",
            json_name: "totalSupply",
            number: 2i32,
            message_fields: None,
        };
        pub const SUPPLY_STATE_FIELD: &'static MessageField = &MessageField {
            name: "supply_state",
            json_name: "supplyState",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for CoinTreasury {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ID_FIELD,
            Self::TOTAL_SUPPLY_FIELD,
            Self::SUPPLY_STATE_FIELD,
        ];
    }
    impl CoinTreasury {
        pub fn path_builder() -> CoinTreasuryFieldPathBuilder {
            CoinTreasuryFieldPathBuilder::new()
        }
    }
    pub struct CoinTreasuryFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CoinTreasuryFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn id(mut self) -> String {
            self.path.push(CoinTreasury::ID_FIELD.name);
            self.finish()
        }
        pub fn total_supply(mut self) -> String {
            self.path.push(CoinTreasury::TOTAL_SUPPLY_FIELD.name);
            self.finish()
        }
        pub fn supply_state(mut self) -> String {
            self.path.push(CoinTreasury::SUPPLY_STATE_FIELD.name);
            self.finish()
        }
    }
    impl RegulatedCoinMetadata {
        pub const ID_FIELD: &'static MessageField = &MessageField {
            name: "id",
            json_name: "id",
            number: 1i32,
            message_fields: None,
        };
        pub const COIN_METADATA_OBJECT_FIELD: &'static MessageField = &MessageField {
            name: "coin_metadata_object",
            json_name: "coinMetadataObject",
            number: 2i32,
            message_fields: None,
        };
        pub const DENY_CAP_OBJECT_FIELD: &'static MessageField = &MessageField {
            name: "deny_cap_object",
            json_name: "denyCapObject",
            number: 3i32,
            message_fields: None,
        };
        pub const ALLOW_GLOBAL_PAUSE_FIELD: &'static MessageField = &MessageField {
            name: "allow_global_pause",
            json_name: "allowGlobalPause",
            number: 4i32,
            message_fields: None,
        };
        pub const VARIANT_FIELD: &'static MessageField = &MessageField {
            name: "variant",
            json_name: "variant",
            number: 5i32,
            message_fields: None,
        };
        pub const COIN_REGULATED_STATE_FIELD: &'static MessageField = &MessageField {
            name: "coin_regulated_state",
            json_name: "coinRegulatedState",
            number: 6i32,
            message_fields: None,
        };
    }
    impl MessageFields for RegulatedCoinMetadata {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ID_FIELD,
            Self::COIN_METADATA_OBJECT_FIELD,
            Self::DENY_CAP_OBJECT_FIELD,
            Self::ALLOW_GLOBAL_PAUSE_FIELD,
            Self::VARIANT_FIELD,
            Self::COIN_REGULATED_STATE_FIELD,
        ];
    }
    impl RegulatedCoinMetadata {
        pub fn path_builder() -> RegulatedCoinMetadataFieldPathBuilder {
            RegulatedCoinMetadataFieldPathBuilder::new()
        }
    }
    pub struct RegulatedCoinMetadataFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl RegulatedCoinMetadataFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn id(mut self) -> String {
            self.path.push(RegulatedCoinMetadata::ID_FIELD.name);
            self.finish()
        }
        pub fn coin_metadata_object(mut self) -> String {
            self.path.push(RegulatedCoinMetadata::COIN_METADATA_OBJECT_FIELD.name);
            self.finish()
        }
        pub fn deny_cap_object(mut self) -> String {
            self.path.push(RegulatedCoinMetadata::DENY_CAP_OBJECT_FIELD.name);
            self.finish()
        }
        pub fn allow_global_pause(mut self) -> String {
            self.path.push(RegulatedCoinMetadata::ALLOW_GLOBAL_PAUSE_FIELD.name);
            self.finish()
        }
        pub fn variant(mut self) -> String {
            self.path.push(RegulatedCoinMetadata::VARIANT_FIELD.name);
            self.finish()
        }
        pub fn coin_regulated_state(mut self) -> String {
            self.path.push(RegulatedCoinMetadata::COIN_REGULATED_STATE_FIELD.name);
            self.finish()
        }
    }
    impl GetBalanceRequest {
        pub const OWNER_FIELD: &'static MessageField = &MessageField {
            name: "owner",
            json_name: "owner",
            number: 1i32,
            message_fields: None,
        };
        pub const COIN_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "coin_type",
            json_name: "coinType",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetBalanceRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OWNER_FIELD,
            Self::COIN_TYPE_FIELD,
        ];
    }
    impl GetBalanceRequest {
        pub fn path_builder() -> GetBalanceRequestFieldPathBuilder {
            GetBalanceRequestFieldPathBuilder::new()
        }
    }
    pub struct GetBalanceRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetBalanceRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn owner(mut self) -> String {
            self.path.push(GetBalanceRequest::OWNER_FIELD.name);
            self.finish()
        }
        pub fn coin_type(mut self) -> String {
            self.path.push(GetBalanceRequest::COIN_TYPE_FIELD.name);
            self.finish()
        }
    }
    impl GetBalanceResponse {
        pub const BALANCE_FIELD: &'static MessageField = &MessageField {
            name: "balance",
            json_name: "balance",
            number: 1i32,
            message_fields: Some(Balance::FIELDS),
        };
    }
    impl MessageFields for GetBalanceResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::BALANCE_FIELD];
    }
    impl GetBalanceResponse {
        pub fn path_builder() -> GetBalanceResponseFieldPathBuilder {
            GetBalanceResponseFieldPathBuilder::new()
        }
    }
    pub struct GetBalanceResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetBalanceResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn balance(mut self) -> BalanceFieldPathBuilder {
            self.path.push(GetBalanceResponse::BALANCE_FIELD.name);
            BalanceFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ListBalancesRequest {
        pub const OWNER_FIELD: &'static MessageField = &MessageField {
            name: "owner",
            json_name: "owner",
            number: 1i32,
            message_fields: None,
        };
        pub const PAGE_SIZE_FIELD: &'static MessageField = &MessageField {
            name: "page_size",
            json_name: "pageSize",
            number: 2i32,
            message_fields: None,
        };
        pub const PAGE_TOKEN_FIELD: &'static MessageField = &MessageField {
            name: "page_token",
            json_name: "pageToken",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for ListBalancesRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OWNER_FIELD,
            Self::PAGE_SIZE_FIELD,
            Self::PAGE_TOKEN_FIELD,
        ];
    }
    impl ListBalancesRequest {
        pub fn path_builder() -> ListBalancesRequestFieldPathBuilder {
            ListBalancesRequestFieldPathBuilder::new()
        }
    }
    pub struct ListBalancesRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListBalancesRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn owner(mut self) -> String {
            self.path.push(ListBalancesRequest::OWNER_FIELD.name);
            self.finish()
        }
        pub fn page_size(mut self) -> String {
            self.path.push(ListBalancesRequest::PAGE_SIZE_FIELD.name);
            self.finish()
        }
        pub fn page_token(mut self) -> String {
            self.path.push(ListBalancesRequest::PAGE_TOKEN_FIELD.name);
            self.finish()
        }
    }
    impl ListBalancesResponse {
        pub const BALANCES_FIELD: &'static MessageField = &MessageField {
            name: "balances",
            json_name: "balances",
            number: 1i32,
            message_fields: Some(Balance::FIELDS),
        };
        pub const NEXT_PAGE_TOKEN_FIELD: &'static MessageField = &MessageField {
            name: "next_page_token",
            json_name: "nextPageToken",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for ListBalancesResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::BALANCES_FIELD,
            Self::NEXT_PAGE_TOKEN_FIELD,
        ];
    }
    impl ListBalancesResponse {
        pub fn path_builder() -> ListBalancesResponseFieldPathBuilder {
            ListBalancesResponseFieldPathBuilder::new()
        }
    }
    pub struct ListBalancesResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListBalancesResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn balances(mut self) -> BalanceFieldPathBuilder {
            self.path.push(ListBalancesResponse::BALANCES_FIELD.name);
            BalanceFieldPathBuilder::new_with_base(self.path)
        }
        pub fn next_page_token(mut self) -> String {
            self.path.push(ListBalancesResponse::NEXT_PAGE_TOKEN_FIELD.name);
            self.finish()
        }
    }
    impl Balance {
        pub const COIN_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "coin_type",
            json_name: "coinType",
            number: 1i32,
            message_fields: None,
        };
        pub const BALANCE_FIELD: &'static MessageField = &MessageField {
            name: "balance",
            json_name: "balance",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for Balance {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::COIN_TYPE_FIELD,
            Self::BALANCE_FIELD,
        ];
    }
    impl Balance {
        pub fn path_builder() -> BalanceFieldPathBuilder {
            BalanceFieldPathBuilder::new()
        }
    }
    pub struct BalanceFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl BalanceFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn coin_type(mut self) -> String {
            self.path.push(Balance::COIN_TYPE_FIELD.name);
            self.finish()
        }
        pub fn balance(mut self) -> String {
            self.path.push(Balance::BALANCE_FIELD.name);
            self.finish()
        }
    }
    impl ListDynamicFieldsRequest {
        pub const PARENT_FIELD: &'static MessageField = &MessageField {
            name: "parent",
            json_name: "parent",
            number: 1i32,
            message_fields: None,
        };
        pub const PAGE_SIZE_FIELD: &'static MessageField = &MessageField {
            name: "page_size",
            json_name: "pageSize",
            number: 2i32,
            message_fields: None,
        };
        pub const PAGE_TOKEN_FIELD: &'static MessageField = &MessageField {
            name: "page_token",
            json_name: "pageToken",
            number: 3i32,
            message_fields: None,
        };
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for ListDynamicFieldsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PARENT_FIELD,
            Self::PAGE_SIZE_FIELD,
            Self::PAGE_TOKEN_FIELD,
            Self::READ_MASK_FIELD,
        ];
    }
    impl ListDynamicFieldsRequest {
        pub fn path_builder() -> ListDynamicFieldsRequestFieldPathBuilder {
            ListDynamicFieldsRequestFieldPathBuilder::new()
        }
    }
    pub struct ListDynamicFieldsRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListDynamicFieldsRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn parent(mut self) -> String {
            self.path.push(ListDynamicFieldsRequest::PARENT_FIELD.name);
            self.finish()
        }
        pub fn page_size(mut self) -> String {
            self.path.push(ListDynamicFieldsRequest::PAGE_SIZE_FIELD.name);
            self.finish()
        }
        pub fn page_token(mut self) -> String {
            self.path.push(ListDynamicFieldsRequest::PAGE_TOKEN_FIELD.name);
            self.finish()
        }
        pub fn read_mask(mut self) -> String {
            self.path.push(ListDynamicFieldsRequest::READ_MASK_FIELD.name);
            self.finish()
        }
    }
    impl ListDynamicFieldsResponse {
        pub const DYNAMIC_FIELDS_FIELD: &'static MessageField = &MessageField {
            name: "dynamic_fields",
            json_name: "dynamicFields",
            number: 1i32,
            message_fields: Some(DynamicField::FIELDS),
        };
        pub const NEXT_PAGE_TOKEN_FIELD: &'static MessageField = &MessageField {
            name: "next_page_token",
            json_name: "nextPageToken",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for ListDynamicFieldsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::DYNAMIC_FIELDS_FIELD,
            Self::NEXT_PAGE_TOKEN_FIELD,
        ];
    }
    impl ListDynamicFieldsResponse {
        pub fn path_builder() -> ListDynamicFieldsResponseFieldPathBuilder {
            ListDynamicFieldsResponseFieldPathBuilder::new()
        }
    }
    pub struct ListDynamicFieldsResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListDynamicFieldsResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn dynamic_fields(mut self) -> DynamicFieldFieldPathBuilder {
            self.path.push(ListDynamicFieldsResponse::DYNAMIC_FIELDS_FIELD.name);
            DynamicFieldFieldPathBuilder::new_with_base(self.path)
        }
        pub fn next_page_token(mut self) -> String {
            self.path.push(ListDynamicFieldsResponse::NEXT_PAGE_TOKEN_FIELD.name);
            self.finish()
        }
    }
    impl DynamicField {
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 1i32,
            message_fields: None,
        };
        pub const PARENT_FIELD: &'static MessageField = &MessageField {
            name: "parent",
            json_name: "parent",
            number: 2i32,
            message_fields: None,
        };
        pub const FIELD_ID_FIELD: &'static MessageField = &MessageField {
            name: "field_id",
            json_name: "fieldId",
            number: 3i32,
            message_fields: None,
        };
        pub const FIELD_OBJECT_FIELD: &'static MessageField = &MessageField {
            name: "field_object",
            json_name: "fieldObject",
            number: 4i32,
            message_fields: Some(Object::FIELDS),
        };
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 5i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const VALUE_FIELD: &'static MessageField = &MessageField {
            name: "value",
            json_name: "value",
            number: 6i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const VALUE_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "value_type",
            json_name: "valueType",
            number: 7i32,
            message_fields: None,
        };
        pub const CHILD_ID_FIELD: &'static MessageField = &MessageField {
            name: "child_id",
            json_name: "childId",
            number: 8i32,
            message_fields: None,
        };
        pub const CHILD_OBJECT_FIELD: &'static MessageField = &MessageField {
            name: "child_object",
            json_name: "childObject",
            number: 9i32,
            message_fields: Some(Object::FIELDS),
        };
    }
    impl MessageFields for DynamicField {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::PARENT_FIELD,
            Self::FIELD_ID_FIELD,
            Self::FIELD_OBJECT_FIELD,
            Self::NAME_FIELD,
            Self::VALUE_FIELD,
            Self::VALUE_TYPE_FIELD,
            Self::CHILD_ID_FIELD,
            Self::CHILD_OBJECT_FIELD,
        ];
    }
    impl DynamicField {
        pub fn path_builder() -> DynamicFieldFieldPathBuilder {
            DynamicFieldFieldPathBuilder::new()
        }
    }
    pub struct DynamicFieldFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl DynamicFieldFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kind(mut self) -> String {
            self.path.push(DynamicField::KIND_FIELD.name);
            self.finish()
        }
        pub fn parent(mut self) -> String {
            self.path.push(DynamicField::PARENT_FIELD.name);
            self.finish()
        }
        pub fn field_id(mut self) -> String {
            self.path.push(DynamicField::FIELD_ID_FIELD.name);
            self.finish()
        }
        pub fn field_object(mut self) -> ObjectFieldPathBuilder {
            self.path.push(DynamicField::FIELD_OBJECT_FIELD.name);
            ObjectFieldPathBuilder::new_with_base(self.path)
        }
        pub fn name(mut self) -> BcsFieldPathBuilder {
            self.path.push(DynamicField::NAME_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn value(mut self) -> BcsFieldPathBuilder {
            self.path.push(DynamicField::VALUE_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn value_type(mut self) -> String {
            self.path.push(DynamicField::VALUE_TYPE_FIELD.name);
            self.finish()
        }
        pub fn child_id(mut self) -> String {
            self.path.push(DynamicField::CHILD_ID_FIELD.name);
            self.finish()
        }
        pub fn child_object(mut self) -> ObjectFieldPathBuilder {
            self.path.push(DynamicField::CHILD_OBJECT_FIELD.name);
            ObjectFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ListOwnedObjectsRequest {
        pub const OWNER_FIELD: &'static MessageField = &MessageField {
            name: "owner",
            json_name: "owner",
            number: 1i32,
            message_fields: None,
        };
        pub const PAGE_SIZE_FIELD: &'static MessageField = &MessageField {
            name: "page_size",
            json_name: "pageSize",
            number: 2i32,
            message_fields: None,
        };
        pub const PAGE_TOKEN_FIELD: &'static MessageField = &MessageField {
            name: "page_token",
            json_name: "pageToken",
            number: 3i32,
            message_fields: None,
        };
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 4i32,
            message_fields: None,
        };
        pub const OBJECT_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "object_type",
            json_name: "objectType",
            number: 5i32,
            message_fields: None,
        };
    }
    impl MessageFields for ListOwnedObjectsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OWNER_FIELD,
            Self::PAGE_SIZE_FIELD,
            Self::PAGE_TOKEN_FIELD,
            Self::READ_MASK_FIELD,
            Self::OBJECT_TYPE_FIELD,
        ];
    }
    impl ListOwnedObjectsRequest {
        pub fn path_builder() -> ListOwnedObjectsRequestFieldPathBuilder {
            ListOwnedObjectsRequestFieldPathBuilder::new()
        }
    }
    pub struct ListOwnedObjectsRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListOwnedObjectsRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn owner(mut self) -> String {
            self.path.push(ListOwnedObjectsRequest::OWNER_FIELD.name);
            self.finish()
        }
        pub fn page_size(mut self) -> String {
            self.path.push(ListOwnedObjectsRequest::PAGE_SIZE_FIELD.name);
            self.finish()
        }
        pub fn page_token(mut self) -> String {
            self.path.push(ListOwnedObjectsRequest::PAGE_TOKEN_FIELD.name);
            self.finish()
        }
        pub fn read_mask(mut self) -> String {
            self.path.push(ListOwnedObjectsRequest::READ_MASK_FIELD.name);
            self.finish()
        }
        pub fn object_type(mut self) -> String {
            self.path.push(ListOwnedObjectsRequest::OBJECT_TYPE_FIELD.name);
            self.finish()
        }
    }
    impl ListOwnedObjectsResponse {
        pub const OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "objects",
            json_name: "objects",
            number: 1i32,
            message_fields: Some(Object::FIELDS),
        };
        pub const NEXT_PAGE_TOKEN_FIELD: &'static MessageField = &MessageField {
            name: "next_page_token",
            json_name: "nextPageToken",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for ListOwnedObjectsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECTS_FIELD,
            Self::NEXT_PAGE_TOKEN_FIELD,
        ];
    }
    impl ListOwnedObjectsResponse {
        pub fn path_builder() -> ListOwnedObjectsResponseFieldPathBuilder {
            ListOwnedObjectsResponseFieldPathBuilder::new()
        }
    }
    pub struct ListOwnedObjectsResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListOwnedObjectsResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn objects(mut self) -> ObjectFieldPathBuilder {
            self.path.push(ListOwnedObjectsResponse::OBJECTS_FIELD.name);
            ObjectFieldPathBuilder::new_with_base(self.path)
        }
        pub fn next_page_token(mut self) -> String {
            self.path.push(ListOwnedObjectsResponse::NEXT_PAGE_TOKEN_FIELD.name);
            self.finish()
        }
    }
    impl SubscribeCheckpointsRequest {
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for SubscribeCheckpointsRequest {
        const FIELDS: &'static [&'static MessageField] = &[Self::READ_MASK_FIELD];
    }
    impl SubscribeCheckpointsRequest {
        pub fn path_builder() -> SubscribeCheckpointsRequestFieldPathBuilder {
            SubscribeCheckpointsRequestFieldPathBuilder::new()
        }
    }
    pub struct SubscribeCheckpointsRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SubscribeCheckpointsRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn read_mask(mut self) -> String {
            self.path.push(SubscribeCheckpointsRequest::READ_MASK_FIELD.name);
            self.finish()
        }
    }
    impl SubscribeCheckpointsResponse {
        pub const CURSOR_FIELD: &'static MessageField = &MessageField {
            name: "cursor",
            json_name: "cursor",
            number: 1i32,
            message_fields: None,
        };
        pub const CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "checkpoint",
            json_name: "checkpoint",
            number: 2i32,
            message_fields: Some(Checkpoint::FIELDS),
        };
    }
    impl MessageFields for SubscribeCheckpointsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CURSOR_FIELD,
            Self::CHECKPOINT_FIELD,
        ];
    }
    impl SubscribeCheckpointsResponse {
        pub fn path_builder() -> SubscribeCheckpointsResponseFieldPathBuilder {
            SubscribeCheckpointsResponseFieldPathBuilder::new()
        }
    }
    pub struct SubscribeCheckpointsResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SubscribeCheckpointsResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn cursor(mut self) -> String {
            self.path.push(SubscribeCheckpointsResponse::CURSOR_FIELD.name);
            self.finish()
        }
        pub fn checkpoint(mut self) -> CheckpointFieldPathBuilder {
            self.path.push(SubscribeCheckpointsResponse::CHECKPOINT_FIELD.name);
            CheckpointFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl SystemState {
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 1i32,
            message_fields: None,
        };
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 2i32,
            message_fields: None,
        };
        pub const PROTOCOL_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "protocol_version",
            json_name: "protocolVersion",
            number: 3i32,
            message_fields: None,
        };
        pub const VALIDATORS_FIELD: &'static MessageField = &MessageField {
            name: "validators",
            json_name: "validators",
            number: 4i32,
            message_fields: Some(ValidatorSet::FIELDS),
        };
        pub const STORAGE_FUND_FIELD: &'static MessageField = &MessageField {
            name: "storage_fund",
            json_name: "storageFund",
            number: 5i32,
            message_fields: Some(StorageFund::FIELDS),
        };
        pub const PARAMETERS_FIELD: &'static MessageField = &MessageField {
            name: "parameters",
            json_name: "parameters",
            number: 6i32,
            message_fields: Some(SystemParameters::FIELDS),
        };
        pub const REFERENCE_GAS_PRICE_FIELD: &'static MessageField = &MessageField {
            name: "reference_gas_price",
            json_name: "referenceGasPrice",
            number: 7i32,
            message_fields: None,
        };
        pub const VALIDATOR_REPORT_RECORDS_FIELD: &'static MessageField = &MessageField {
            name: "validator_report_records",
            json_name: "validatorReportRecords",
            number: 8i32,
            message_fields: Some(ValidatorReportRecord::FIELDS),
        };
        pub const STAKE_SUBSIDY_FIELD: &'static MessageField = &MessageField {
            name: "stake_subsidy",
            json_name: "stakeSubsidy",
            number: 9i32,
            message_fields: Some(StakeSubsidy::FIELDS),
        };
        pub const SAFE_MODE_FIELD: &'static MessageField = &MessageField {
            name: "safe_mode",
            json_name: "safeMode",
            number: 10i32,
            message_fields: None,
        };
        pub const SAFE_MODE_STORAGE_REWARDS_FIELD: &'static MessageField = &MessageField {
            name: "safe_mode_storage_rewards",
            json_name: "safeModeStorageRewards",
            number: 11i32,
            message_fields: None,
        };
        pub const SAFE_MODE_COMPUTATION_REWARDS_FIELD: &'static MessageField = &MessageField {
            name: "safe_mode_computation_rewards",
            json_name: "safeModeComputationRewards",
            number: 12i32,
            message_fields: None,
        };
        pub const SAFE_MODE_STORAGE_REBATES_FIELD: &'static MessageField = &MessageField {
            name: "safe_mode_storage_rebates",
            json_name: "safeModeStorageRebates",
            number: 13i32,
            message_fields: None,
        };
        pub const SAFE_MODE_NON_REFUNDABLE_STORAGE_FEE_FIELD: &'static MessageField = &MessageField {
            name: "safe_mode_non_refundable_storage_fee",
            json_name: "safeModeNonRefundableStorageFee",
            number: 14i32,
            message_fields: None,
        };
        pub const EPOCH_START_TIMESTAMP_MS_FIELD: &'static MessageField = &MessageField {
            name: "epoch_start_timestamp_ms",
            json_name: "epochStartTimestampMs",
            number: 15i32,
            message_fields: None,
        };
        pub const EXTRA_FIELDS_FIELD: &'static MessageField = &MessageField {
            name: "extra_fields",
            json_name: "extraFields",
            number: 16i32,
            message_fields: Some(MoveTable::FIELDS),
        };
    }
    impl MessageFields for SystemState {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::VERSION_FIELD,
            Self::EPOCH_FIELD,
            Self::PROTOCOL_VERSION_FIELD,
            Self::VALIDATORS_FIELD,
            Self::STORAGE_FUND_FIELD,
            Self::PARAMETERS_FIELD,
            Self::REFERENCE_GAS_PRICE_FIELD,
            Self::VALIDATOR_REPORT_RECORDS_FIELD,
            Self::STAKE_SUBSIDY_FIELD,
            Self::SAFE_MODE_FIELD,
            Self::SAFE_MODE_STORAGE_REWARDS_FIELD,
            Self::SAFE_MODE_COMPUTATION_REWARDS_FIELD,
            Self::SAFE_MODE_STORAGE_REBATES_FIELD,
            Self::SAFE_MODE_NON_REFUNDABLE_STORAGE_FEE_FIELD,
            Self::EPOCH_START_TIMESTAMP_MS_FIELD,
            Self::EXTRA_FIELDS_FIELD,
        ];
    }
    impl SystemState {
        pub fn path_builder() -> SystemStateFieldPathBuilder {
            SystemStateFieldPathBuilder::new()
        }
    }
    pub struct SystemStateFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SystemStateFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn version(mut self) -> String {
            self.path.push(SystemState::VERSION_FIELD.name);
            self.finish()
        }
        pub fn epoch(mut self) -> String {
            self.path.push(SystemState::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn protocol_version(mut self) -> String {
            self.path.push(SystemState::PROTOCOL_VERSION_FIELD.name);
            self.finish()
        }
        pub fn validators(mut self) -> ValidatorSetFieldPathBuilder {
            self.path.push(SystemState::VALIDATORS_FIELD.name);
            ValidatorSetFieldPathBuilder::new_with_base(self.path)
        }
        pub fn storage_fund(mut self) -> StorageFundFieldPathBuilder {
            self.path.push(SystemState::STORAGE_FUND_FIELD.name);
            StorageFundFieldPathBuilder::new_with_base(self.path)
        }
        pub fn parameters(mut self) -> SystemParametersFieldPathBuilder {
            self.path.push(SystemState::PARAMETERS_FIELD.name);
            SystemParametersFieldPathBuilder::new_with_base(self.path)
        }
        pub fn reference_gas_price(mut self) -> String {
            self.path.push(SystemState::REFERENCE_GAS_PRICE_FIELD.name);
            self.finish()
        }
        pub fn validator_report_records(
            mut self,
        ) -> ValidatorReportRecordFieldPathBuilder {
            self.path.push(SystemState::VALIDATOR_REPORT_RECORDS_FIELD.name);
            ValidatorReportRecordFieldPathBuilder::new_with_base(self.path)
        }
        pub fn stake_subsidy(mut self) -> StakeSubsidyFieldPathBuilder {
            self.path.push(SystemState::STAKE_SUBSIDY_FIELD.name);
            StakeSubsidyFieldPathBuilder::new_with_base(self.path)
        }
        pub fn safe_mode(mut self) -> String {
            self.path.push(SystemState::SAFE_MODE_FIELD.name);
            self.finish()
        }
        pub fn safe_mode_storage_rewards(mut self) -> String {
            self.path.push(SystemState::SAFE_MODE_STORAGE_REWARDS_FIELD.name);
            self.finish()
        }
        pub fn safe_mode_computation_rewards(mut self) -> String {
            self.path.push(SystemState::SAFE_MODE_COMPUTATION_REWARDS_FIELD.name);
            self.finish()
        }
        pub fn safe_mode_storage_rebates(mut self) -> String {
            self.path.push(SystemState::SAFE_MODE_STORAGE_REBATES_FIELD.name);
            self.finish()
        }
        pub fn safe_mode_non_refundable_storage_fee(mut self) -> String {
            self.path.push(SystemState::SAFE_MODE_NON_REFUNDABLE_STORAGE_FEE_FIELD.name);
            self.finish()
        }
        pub fn epoch_start_timestamp_ms(mut self) -> String {
            self.path.push(SystemState::EPOCH_START_TIMESTAMP_MS_FIELD.name);
            self.finish()
        }
        pub fn extra_fields(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(SystemState::EXTRA_FIELDS_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ValidatorReportRecord {
        pub const REPORTED_FIELD: &'static MessageField = &MessageField {
            name: "reported",
            json_name: "reported",
            number: 1i32,
            message_fields: None,
        };
        pub const REPORTERS_FIELD: &'static MessageField = &MessageField {
            name: "reporters",
            json_name: "reporters",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for ValidatorReportRecord {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::REPORTED_FIELD,
            Self::REPORTERS_FIELD,
        ];
    }
    impl ValidatorReportRecord {
        pub fn path_builder() -> ValidatorReportRecordFieldPathBuilder {
            ValidatorReportRecordFieldPathBuilder::new()
        }
    }
    pub struct ValidatorReportRecordFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ValidatorReportRecordFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn reported(mut self) -> String {
            self.path.push(ValidatorReportRecord::REPORTED_FIELD.name);
            self.finish()
        }
        pub fn reporters(mut self) -> String {
            self.path.push(ValidatorReportRecord::REPORTERS_FIELD.name);
            self.finish()
        }
    }
    impl SystemParameters {
        pub const EPOCH_DURATION_MS_FIELD: &'static MessageField = &MessageField {
            name: "epoch_duration_ms",
            json_name: "epochDurationMs",
            number: 1i32,
            message_fields: None,
        };
        pub const STAKE_SUBSIDY_START_EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "stake_subsidy_start_epoch",
            json_name: "stakeSubsidyStartEpoch",
            number: 2i32,
            message_fields: None,
        };
        pub const MIN_VALIDATOR_COUNT_FIELD: &'static MessageField = &MessageField {
            name: "min_validator_count",
            json_name: "minValidatorCount",
            number: 3i32,
            message_fields: None,
        };
        pub const MAX_VALIDATOR_COUNT_FIELD: &'static MessageField = &MessageField {
            name: "max_validator_count",
            json_name: "maxValidatorCount",
            number: 4i32,
            message_fields: None,
        };
        pub const MIN_VALIDATOR_JOINING_STAKE_FIELD: &'static MessageField = &MessageField {
            name: "min_validator_joining_stake",
            json_name: "minValidatorJoiningStake",
            number: 5i32,
            message_fields: None,
        };
        pub const VALIDATOR_LOW_STAKE_THRESHOLD_FIELD: &'static MessageField = &MessageField {
            name: "validator_low_stake_threshold",
            json_name: "validatorLowStakeThreshold",
            number: 6i32,
            message_fields: None,
        };
        pub const VALIDATOR_VERY_LOW_STAKE_THRESHOLD_FIELD: &'static MessageField = &MessageField {
            name: "validator_very_low_stake_threshold",
            json_name: "validatorVeryLowStakeThreshold",
            number: 7i32,
            message_fields: None,
        };
        pub const VALIDATOR_LOW_STAKE_GRACE_PERIOD_FIELD: &'static MessageField = &MessageField {
            name: "validator_low_stake_grace_period",
            json_name: "validatorLowStakeGracePeriod",
            number: 8i32,
            message_fields: None,
        };
        pub const EXTRA_FIELDS_FIELD: &'static MessageField = &MessageField {
            name: "extra_fields",
            json_name: "extraFields",
            number: 9i32,
            message_fields: Some(MoveTable::FIELDS),
        };
    }
    impl MessageFields for SystemParameters {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EPOCH_DURATION_MS_FIELD,
            Self::STAKE_SUBSIDY_START_EPOCH_FIELD,
            Self::MIN_VALIDATOR_COUNT_FIELD,
            Self::MAX_VALIDATOR_COUNT_FIELD,
            Self::MIN_VALIDATOR_JOINING_STAKE_FIELD,
            Self::VALIDATOR_LOW_STAKE_THRESHOLD_FIELD,
            Self::VALIDATOR_VERY_LOW_STAKE_THRESHOLD_FIELD,
            Self::VALIDATOR_LOW_STAKE_GRACE_PERIOD_FIELD,
            Self::EXTRA_FIELDS_FIELD,
        ];
    }
    impl SystemParameters {
        pub fn path_builder() -> SystemParametersFieldPathBuilder {
            SystemParametersFieldPathBuilder::new()
        }
    }
    pub struct SystemParametersFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SystemParametersFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn epoch_duration_ms(mut self) -> String {
            self.path.push(SystemParameters::EPOCH_DURATION_MS_FIELD.name);
            self.finish()
        }
        pub fn stake_subsidy_start_epoch(mut self) -> String {
            self.path.push(SystemParameters::STAKE_SUBSIDY_START_EPOCH_FIELD.name);
            self.finish()
        }
        pub fn min_validator_count(mut self) -> String {
            self.path.push(SystemParameters::MIN_VALIDATOR_COUNT_FIELD.name);
            self.finish()
        }
        pub fn max_validator_count(mut self) -> String {
            self.path.push(SystemParameters::MAX_VALIDATOR_COUNT_FIELD.name);
            self.finish()
        }
        pub fn min_validator_joining_stake(mut self) -> String {
            self.path.push(SystemParameters::MIN_VALIDATOR_JOINING_STAKE_FIELD.name);
            self.finish()
        }
        pub fn validator_low_stake_threshold(mut self) -> String {
            self.path.push(SystemParameters::VALIDATOR_LOW_STAKE_THRESHOLD_FIELD.name);
            self.finish()
        }
        pub fn validator_very_low_stake_threshold(mut self) -> String {
            self.path
                .push(SystemParameters::VALIDATOR_VERY_LOW_STAKE_THRESHOLD_FIELD.name);
            self.finish()
        }
        pub fn validator_low_stake_grace_period(mut self) -> String {
            self.path
                .push(SystemParameters::VALIDATOR_LOW_STAKE_GRACE_PERIOD_FIELD.name);
            self.finish()
        }
        pub fn extra_fields(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(SystemParameters::EXTRA_FIELDS_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MoveTable {
        pub const ID_FIELD: &'static MessageField = &MessageField {
            name: "id",
            json_name: "id",
            number: 1i32,
            message_fields: None,
        };
        pub const SIZE_FIELD: &'static MessageField = &MessageField {
            name: "size",
            json_name: "size",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for MoveTable {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ID_FIELD,
            Self::SIZE_FIELD,
        ];
    }
    impl MoveTable {
        pub fn path_builder() -> MoveTableFieldPathBuilder {
            MoveTableFieldPathBuilder::new()
        }
    }
    pub struct MoveTableFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MoveTableFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn id(mut self) -> String {
            self.path.push(MoveTable::ID_FIELD.name);
            self.finish()
        }
        pub fn size(mut self) -> String {
            self.path.push(MoveTable::SIZE_FIELD.name);
            self.finish()
        }
    }
    impl StakeSubsidy {
        pub const BALANCE_FIELD: &'static MessageField = &MessageField {
            name: "balance",
            json_name: "balance",
            number: 1i32,
            message_fields: None,
        };
        pub const DISTRIBUTION_COUNTER_FIELD: &'static MessageField = &MessageField {
            name: "distribution_counter",
            json_name: "distributionCounter",
            number: 2i32,
            message_fields: None,
        };
        pub const CURRENT_DISTRIBUTION_AMOUNT_FIELD: &'static MessageField = &MessageField {
            name: "current_distribution_amount",
            json_name: "currentDistributionAmount",
            number: 3i32,
            message_fields: None,
        };
        pub const STAKE_SUBSIDY_PERIOD_LENGTH_FIELD: &'static MessageField = &MessageField {
            name: "stake_subsidy_period_length",
            json_name: "stakeSubsidyPeriodLength",
            number: 4i32,
            message_fields: None,
        };
        pub const STAKE_SUBSIDY_DECREASE_RATE_FIELD: &'static MessageField = &MessageField {
            name: "stake_subsidy_decrease_rate",
            json_name: "stakeSubsidyDecreaseRate",
            number: 5i32,
            message_fields: None,
        };
        pub const EXTRA_FIELDS_FIELD: &'static MessageField = &MessageField {
            name: "extra_fields",
            json_name: "extraFields",
            number: 6i32,
            message_fields: Some(MoveTable::FIELDS),
        };
    }
    impl MessageFields for StakeSubsidy {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::BALANCE_FIELD,
            Self::DISTRIBUTION_COUNTER_FIELD,
            Self::CURRENT_DISTRIBUTION_AMOUNT_FIELD,
            Self::STAKE_SUBSIDY_PERIOD_LENGTH_FIELD,
            Self::STAKE_SUBSIDY_DECREASE_RATE_FIELD,
            Self::EXTRA_FIELDS_FIELD,
        ];
    }
    impl StakeSubsidy {
        pub fn path_builder() -> StakeSubsidyFieldPathBuilder {
            StakeSubsidyFieldPathBuilder::new()
        }
    }
    pub struct StakeSubsidyFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl StakeSubsidyFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn balance(mut self) -> String {
            self.path.push(StakeSubsidy::BALANCE_FIELD.name);
            self.finish()
        }
        pub fn distribution_counter(mut self) -> String {
            self.path.push(StakeSubsidy::DISTRIBUTION_COUNTER_FIELD.name);
            self.finish()
        }
        pub fn current_distribution_amount(mut self) -> String {
            self.path.push(StakeSubsidy::CURRENT_DISTRIBUTION_AMOUNT_FIELD.name);
            self.finish()
        }
        pub fn stake_subsidy_period_length(mut self) -> String {
            self.path.push(StakeSubsidy::STAKE_SUBSIDY_PERIOD_LENGTH_FIELD.name);
            self.finish()
        }
        pub fn stake_subsidy_decrease_rate(mut self) -> String {
            self.path.push(StakeSubsidy::STAKE_SUBSIDY_DECREASE_RATE_FIELD.name);
            self.finish()
        }
        pub fn extra_fields(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(StakeSubsidy::EXTRA_FIELDS_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl StorageFund {
        pub const TOTAL_OBJECT_STORAGE_REBATES_FIELD: &'static MessageField = &MessageField {
            name: "total_object_storage_rebates",
            json_name: "totalObjectStorageRebates",
            number: 1i32,
            message_fields: None,
        };
        pub const NON_REFUNDABLE_BALANCE_FIELD: &'static MessageField = &MessageField {
            name: "non_refundable_balance",
            json_name: "nonRefundableBalance",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for StorageFund {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TOTAL_OBJECT_STORAGE_REBATES_FIELD,
            Self::NON_REFUNDABLE_BALANCE_FIELD,
        ];
    }
    impl StorageFund {
        pub fn path_builder() -> StorageFundFieldPathBuilder {
            StorageFundFieldPathBuilder::new()
        }
    }
    pub struct StorageFundFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl StorageFundFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn total_object_storage_rebates(mut self) -> String {
            self.path.push(StorageFund::TOTAL_OBJECT_STORAGE_REBATES_FIELD.name);
            self.finish()
        }
        pub fn non_refundable_balance(mut self) -> String {
            self.path.push(StorageFund::NON_REFUNDABLE_BALANCE_FIELD.name);
            self.finish()
        }
    }
    impl ValidatorSet {
        pub const TOTAL_STAKE_FIELD: &'static MessageField = &MessageField {
            name: "total_stake",
            json_name: "totalStake",
            number: 1i32,
            message_fields: None,
        };
        pub const ACTIVE_VALIDATORS_FIELD: &'static MessageField = &MessageField {
            name: "active_validators",
            json_name: "activeValidators",
            number: 2i32,
            message_fields: Some(Validator::FIELDS),
        };
        pub const PENDING_ACTIVE_VALIDATORS_FIELD: &'static MessageField = &MessageField {
            name: "pending_active_validators",
            json_name: "pendingActiveValidators",
            number: 3i32,
            message_fields: Some(MoveTable::FIELDS),
        };
        pub const PENDING_REMOVALS_FIELD: &'static MessageField = &MessageField {
            name: "pending_removals",
            json_name: "pendingRemovals",
            number: 4i32,
            message_fields: None,
        };
        pub const STAKING_POOL_MAPPINGS_FIELD: &'static MessageField = &MessageField {
            name: "staking_pool_mappings",
            json_name: "stakingPoolMappings",
            number: 5i32,
            message_fields: Some(MoveTable::FIELDS),
        };
        pub const INACTIVE_VALIDATORS_FIELD: &'static MessageField = &MessageField {
            name: "inactive_validators",
            json_name: "inactiveValidators",
            number: 6i32,
            message_fields: Some(MoveTable::FIELDS),
        };
        pub const VALIDATOR_CANDIDATES_FIELD: &'static MessageField = &MessageField {
            name: "validator_candidates",
            json_name: "validatorCandidates",
            number: 7i32,
            message_fields: Some(MoveTable::FIELDS),
        };
        pub const AT_RISK_VALIDATORS_FIELD: &'static MessageField = &MessageField {
            name: "at_risk_validators",
            json_name: "atRiskValidators",
            number: 8i32,
            message_fields: None,
        };
        pub const EXTRA_FIELDS_FIELD: &'static MessageField = &MessageField {
            name: "extra_fields",
            json_name: "extraFields",
            number: 9i32,
            message_fields: Some(MoveTable::FIELDS),
        };
    }
    impl MessageFields for ValidatorSet {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TOTAL_STAKE_FIELD,
            Self::ACTIVE_VALIDATORS_FIELD,
            Self::PENDING_ACTIVE_VALIDATORS_FIELD,
            Self::PENDING_REMOVALS_FIELD,
            Self::STAKING_POOL_MAPPINGS_FIELD,
            Self::INACTIVE_VALIDATORS_FIELD,
            Self::VALIDATOR_CANDIDATES_FIELD,
            Self::AT_RISK_VALIDATORS_FIELD,
            Self::EXTRA_FIELDS_FIELD,
        ];
    }
    impl ValidatorSet {
        pub fn path_builder() -> ValidatorSetFieldPathBuilder {
            ValidatorSetFieldPathBuilder::new()
        }
    }
    pub struct ValidatorSetFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ValidatorSetFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn total_stake(mut self) -> String {
            self.path.push(ValidatorSet::TOTAL_STAKE_FIELD.name);
            self.finish()
        }
        pub fn active_validators(mut self) -> ValidatorFieldPathBuilder {
            self.path.push(ValidatorSet::ACTIVE_VALIDATORS_FIELD.name);
            ValidatorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn pending_active_validators(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(ValidatorSet::PENDING_ACTIVE_VALIDATORS_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
        pub fn pending_removals(mut self) -> String {
            self.path.push(ValidatorSet::PENDING_REMOVALS_FIELD.name);
            self.finish()
        }
        pub fn staking_pool_mappings(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(ValidatorSet::STAKING_POOL_MAPPINGS_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
        pub fn inactive_validators(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(ValidatorSet::INACTIVE_VALIDATORS_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
        pub fn validator_candidates(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(ValidatorSet::VALIDATOR_CANDIDATES_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
        pub fn at_risk_validators(mut self) -> String {
            self.path.push(ValidatorSet::AT_RISK_VALIDATORS_FIELD.name);
            self.finish()
        }
        pub fn extra_fields(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(ValidatorSet::EXTRA_FIELDS_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl Validator {
        pub const NAME_FIELD: &'static MessageField = &MessageField {
            name: "name",
            json_name: "name",
            number: 1i32,
            message_fields: None,
        };
        pub const ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "address",
            json_name: "address",
            number: 2i32,
            message_fields: None,
        };
        pub const DESCRIPTION_FIELD: &'static MessageField = &MessageField {
            name: "description",
            json_name: "description",
            number: 3i32,
            message_fields: None,
        };
        pub const IMAGE_URL_FIELD: &'static MessageField = &MessageField {
            name: "image_url",
            json_name: "imageUrl",
            number: 4i32,
            message_fields: None,
        };
        pub const PROJECT_URL_FIELD: &'static MessageField = &MessageField {
            name: "project_url",
            json_name: "projectUrl",
            number: 5i32,
            message_fields: None,
        };
        pub const PROTOCOL_PUBLIC_KEY_FIELD: &'static MessageField = &MessageField {
            name: "protocol_public_key",
            json_name: "protocolPublicKey",
            number: 7i32,
            message_fields: None,
        };
        pub const PROOF_OF_POSSESSION_FIELD: &'static MessageField = &MessageField {
            name: "proof_of_possession",
            json_name: "proofOfPossession",
            number: 8i32,
            message_fields: None,
        };
        pub const NETWORK_PUBLIC_KEY_FIELD: &'static MessageField = &MessageField {
            name: "network_public_key",
            json_name: "networkPublicKey",
            number: 10i32,
            message_fields: None,
        };
        pub const WORKER_PUBLIC_KEY_FIELD: &'static MessageField = &MessageField {
            name: "worker_public_key",
            json_name: "workerPublicKey",
            number: 12i32,
            message_fields: None,
        };
        pub const NETWORK_ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "network_address",
            json_name: "networkAddress",
            number: 13i32,
            message_fields: None,
        };
        pub const P2P_ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "p2p_address",
            json_name: "p2pAddress",
            number: 14i32,
            message_fields: None,
        };
        pub const PRIMARY_ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "primary_address",
            json_name: "primaryAddress",
            number: 15i32,
            message_fields: None,
        };
        pub const WORKER_ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "worker_address",
            json_name: "workerAddress",
            number: 16i32,
            message_fields: None,
        };
        pub const NEXT_EPOCH_PROTOCOL_PUBLIC_KEY_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_protocol_public_key",
            json_name: "nextEpochProtocolPublicKey",
            number: 18i32,
            message_fields: None,
        };
        pub const NEXT_EPOCH_PROOF_OF_POSSESSION_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_proof_of_possession",
            json_name: "nextEpochProofOfPossession",
            number: 19i32,
            message_fields: None,
        };
        pub const NEXT_EPOCH_NETWORK_PUBLIC_KEY_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_network_public_key",
            json_name: "nextEpochNetworkPublicKey",
            number: 21i32,
            message_fields: None,
        };
        pub const NEXT_EPOCH_WORKER_PUBLIC_KEY_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_worker_public_key",
            json_name: "nextEpochWorkerPublicKey",
            number: 23i32,
            message_fields: None,
        };
        pub const NEXT_EPOCH_NETWORK_ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_network_address",
            json_name: "nextEpochNetworkAddress",
            number: 24i32,
            message_fields: None,
        };
        pub const NEXT_EPOCH_P2P_ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_p2p_address",
            json_name: "nextEpochP2pAddress",
            number: 25i32,
            message_fields: None,
        };
        pub const NEXT_EPOCH_PRIMARY_ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_primary_address",
            json_name: "nextEpochPrimaryAddress",
            number: 26i32,
            message_fields: None,
        };
        pub const NEXT_EPOCH_WORKER_ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_worker_address",
            json_name: "nextEpochWorkerAddress",
            number: 27i32,
            message_fields: None,
        };
        pub const METADATA_EXTRA_FIELDS_FIELD: &'static MessageField = &MessageField {
            name: "metadata_extra_fields",
            json_name: "metadataExtraFields",
            number: 28i32,
            message_fields: Some(MoveTable::FIELDS),
        };
        pub const VOTING_POWER_FIELD: &'static MessageField = &MessageField {
            name: "voting_power",
            json_name: "votingPower",
            number: 29i32,
            message_fields: None,
        };
        pub const OPERATION_CAP_ID_FIELD: &'static MessageField = &MessageField {
            name: "operation_cap_id",
            json_name: "operationCapId",
            number: 30i32,
            message_fields: None,
        };
        pub const GAS_PRICE_FIELD: &'static MessageField = &MessageField {
            name: "gas_price",
            json_name: "gasPrice",
            number: 31i32,
            message_fields: None,
        };
        pub const STAKING_POOL_FIELD: &'static MessageField = &MessageField {
            name: "staking_pool",
            json_name: "stakingPool",
            number: 32i32,
            message_fields: Some(StakingPool::FIELDS),
        };
        pub const COMMISSION_RATE_FIELD: &'static MessageField = &MessageField {
            name: "commission_rate",
            json_name: "commissionRate",
            number: 33i32,
            message_fields: None,
        };
        pub const NEXT_EPOCH_STAKE_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_stake",
            json_name: "nextEpochStake",
            number: 34i32,
            message_fields: None,
        };
        pub const NEXT_EPOCH_GAS_PRICE_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_gas_price",
            json_name: "nextEpochGasPrice",
            number: 35i32,
            message_fields: None,
        };
        pub const NEXT_EPOCH_COMMISSION_RATE_FIELD: &'static MessageField = &MessageField {
            name: "next_epoch_commission_rate",
            json_name: "nextEpochCommissionRate",
            number: 36i32,
            message_fields: None,
        };
        pub const EXTRA_FIELDS_FIELD: &'static MessageField = &MessageField {
            name: "extra_fields",
            json_name: "extraFields",
            number: 37i32,
            message_fields: Some(MoveTable::FIELDS),
        };
    }
    impl MessageFields for Validator {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::NAME_FIELD,
            Self::ADDRESS_FIELD,
            Self::DESCRIPTION_FIELD,
            Self::IMAGE_URL_FIELD,
            Self::PROJECT_URL_FIELD,
            Self::PROTOCOL_PUBLIC_KEY_FIELD,
            Self::PROOF_OF_POSSESSION_FIELD,
            Self::NETWORK_PUBLIC_KEY_FIELD,
            Self::WORKER_PUBLIC_KEY_FIELD,
            Self::NETWORK_ADDRESS_FIELD,
            Self::P2P_ADDRESS_FIELD,
            Self::PRIMARY_ADDRESS_FIELD,
            Self::WORKER_ADDRESS_FIELD,
            Self::NEXT_EPOCH_PROTOCOL_PUBLIC_KEY_FIELD,
            Self::NEXT_EPOCH_PROOF_OF_POSSESSION_FIELD,
            Self::NEXT_EPOCH_NETWORK_PUBLIC_KEY_FIELD,
            Self::NEXT_EPOCH_WORKER_PUBLIC_KEY_FIELD,
            Self::NEXT_EPOCH_NETWORK_ADDRESS_FIELD,
            Self::NEXT_EPOCH_P2P_ADDRESS_FIELD,
            Self::NEXT_EPOCH_PRIMARY_ADDRESS_FIELD,
            Self::NEXT_EPOCH_WORKER_ADDRESS_FIELD,
            Self::METADATA_EXTRA_FIELDS_FIELD,
            Self::VOTING_POWER_FIELD,
            Self::OPERATION_CAP_ID_FIELD,
            Self::GAS_PRICE_FIELD,
            Self::STAKING_POOL_FIELD,
            Self::COMMISSION_RATE_FIELD,
            Self::NEXT_EPOCH_STAKE_FIELD,
            Self::NEXT_EPOCH_GAS_PRICE_FIELD,
            Self::NEXT_EPOCH_COMMISSION_RATE_FIELD,
            Self::EXTRA_FIELDS_FIELD,
        ];
    }
    impl Validator {
        pub fn path_builder() -> ValidatorFieldPathBuilder {
            ValidatorFieldPathBuilder::new()
        }
    }
    pub struct ValidatorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ValidatorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn name(mut self) -> String {
            self.path.push(Validator::NAME_FIELD.name);
            self.finish()
        }
        pub fn address(mut self) -> String {
            self.path.push(Validator::ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn description(mut self) -> String {
            self.path.push(Validator::DESCRIPTION_FIELD.name);
            self.finish()
        }
        pub fn image_url(mut self) -> String {
            self.path.push(Validator::IMAGE_URL_FIELD.name);
            self.finish()
        }
        pub fn project_url(mut self) -> String {
            self.path.push(Validator::PROJECT_URL_FIELD.name);
            self.finish()
        }
        pub fn protocol_public_key(mut self) -> String {
            self.path.push(Validator::PROTOCOL_PUBLIC_KEY_FIELD.name);
            self.finish()
        }
        pub fn proof_of_possession(mut self) -> String {
            self.path.push(Validator::PROOF_OF_POSSESSION_FIELD.name);
            self.finish()
        }
        pub fn network_public_key(mut self) -> String {
            self.path.push(Validator::NETWORK_PUBLIC_KEY_FIELD.name);
            self.finish()
        }
        pub fn worker_public_key(mut self) -> String {
            self.path.push(Validator::WORKER_PUBLIC_KEY_FIELD.name);
            self.finish()
        }
        pub fn network_address(mut self) -> String {
            self.path.push(Validator::NETWORK_ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn p2p_address(mut self) -> String {
            self.path.push(Validator::P2P_ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn primary_address(mut self) -> String {
            self.path.push(Validator::PRIMARY_ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn worker_address(mut self) -> String {
            self.path.push(Validator::WORKER_ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn next_epoch_protocol_public_key(mut self) -> String {
            self.path.push(Validator::NEXT_EPOCH_PROTOCOL_PUBLIC_KEY_FIELD.name);
            self.finish()
        }
        pub fn next_epoch_proof_of_possession(mut self) -> String {
            self.path.push(Validator::NEXT_EPOCH_PROOF_OF_POSSESSION_FIELD.name);
            self.finish()
        }
        pub fn next_epoch_network_public_key(mut self) -> String {
            self.path.push(Validator::NEXT_EPOCH_NETWORK_PUBLIC_KEY_FIELD.name);
            self.finish()
        }
        pub fn next_epoch_worker_public_key(mut self) -> String {
            self.path.push(Validator::NEXT_EPOCH_WORKER_PUBLIC_KEY_FIELD.name);
            self.finish()
        }
        pub fn next_epoch_network_address(mut self) -> String {
            self.path.push(Validator::NEXT_EPOCH_NETWORK_ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn next_epoch_p2p_address(mut self) -> String {
            self.path.push(Validator::NEXT_EPOCH_P2P_ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn next_epoch_primary_address(mut self) -> String {
            self.path.push(Validator::NEXT_EPOCH_PRIMARY_ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn next_epoch_worker_address(mut self) -> String {
            self.path.push(Validator::NEXT_EPOCH_WORKER_ADDRESS_FIELD.name);
            self.finish()
        }
        pub fn metadata_extra_fields(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(Validator::METADATA_EXTRA_FIELDS_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
        pub fn voting_power(mut self) -> String {
            self.path.push(Validator::VOTING_POWER_FIELD.name);
            self.finish()
        }
        pub fn operation_cap_id(mut self) -> String {
            self.path.push(Validator::OPERATION_CAP_ID_FIELD.name);
            self.finish()
        }
        pub fn gas_price(mut self) -> String {
            self.path.push(Validator::GAS_PRICE_FIELD.name);
            self.finish()
        }
        pub fn staking_pool(mut self) -> StakingPoolFieldPathBuilder {
            self.path.push(Validator::STAKING_POOL_FIELD.name);
            StakingPoolFieldPathBuilder::new_with_base(self.path)
        }
        pub fn commission_rate(mut self) -> String {
            self.path.push(Validator::COMMISSION_RATE_FIELD.name);
            self.finish()
        }
        pub fn next_epoch_stake(mut self) -> String {
            self.path.push(Validator::NEXT_EPOCH_STAKE_FIELD.name);
            self.finish()
        }
        pub fn next_epoch_gas_price(mut self) -> String {
            self.path.push(Validator::NEXT_EPOCH_GAS_PRICE_FIELD.name);
            self.finish()
        }
        pub fn next_epoch_commission_rate(mut self) -> String {
            self.path.push(Validator::NEXT_EPOCH_COMMISSION_RATE_FIELD.name);
            self.finish()
        }
        pub fn extra_fields(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(Validator::EXTRA_FIELDS_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl StakingPool {
        pub const ID_FIELD: &'static MessageField = &MessageField {
            name: "id",
            json_name: "id",
            number: 1i32,
            message_fields: None,
        };
        pub const ACTIVATION_EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "activation_epoch",
            json_name: "activationEpoch",
            number: 2i32,
            message_fields: None,
        };
        pub const DEACTIVATION_EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "deactivation_epoch",
            json_name: "deactivationEpoch",
            number: 3i32,
            message_fields: None,
        };
        pub const SUI_BALANCE_FIELD: &'static MessageField = &MessageField {
            name: "sui_balance",
            json_name: "suiBalance",
            number: 4i32,
            message_fields: None,
        };
        pub const REWARDS_POOL_FIELD: &'static MessageField = &MessageField {
            name: "rewards_pool",
            json_name: "rewardsPool",
            number: 5i32,
            message_fields: None,
        };
        pub const POOL_TOKEN_BALANCE_FIELD: &'static MessageField = &MessageField {
            name: "pool_token_balance",
            json_name: "poolTokenBalance",
            number: 6i32,
            message_fields: None,
        };
        pub const EXCHANGE_RATES_FIELD: &'static MessageField = &MessageField {
            name: "exchange_rates",
            json_name: "exchangeRates",
            number: 7i32,
            message_fields: Some(MoveTable::FIELDS),
        };
        pub const PENDING_STAKE_FIELD: &'static MessageField = &MessageField {
            name: "pending_stake",
            json_name: "pendingStake",
            number: 8i32,
            message_fields: None,
        };
        pub const PENDING_TOTAL_SUI_WITHDRAW_FIELD: &'static MessageField = &MessageField {
            name: "pending_total_sui_withdraw",
            json_name: "pendingTotalSuiWithdraw",
            number: 9i32,
            message_fields: None,
        };
        pub const PENDING_POOL_TOKEN_WITHDRAW_FIELD: &'static MessageField = &MessageField {
            name: "pending_pool_token_withdraw",
            json_name: "pendingPoolTokenWithdraw",
            number: 10i32,
            message_fields: None,
        };
        pub const EXTRA_FIELDS_FIELD: &'static MessageField = &MessageField {
            name: "extra_fields",
            json_name: "extraFields",
            number: 11i32,
            message_fields: Some(MoveTable::FIELDS),
        };
    }
    impl MessageFields for StakingPool {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ID_FIELD,
            Self::ACTIVATION_EPOCH_FIELD,
            Self::DEACTIVATION_EPOCH_FIELD,
            Self::SUI_BALANCE_FIELD,
            Self::REWARDS_POOL_FIELD,
            Self::POOL_TOKEN_BALANCE_FIELD,
            Self::EXCHANGE_RATES_FIELD,
            Self::PENDING_STAKE_FIELD,
            Self::PENDING_TOTAL_SUI_WITHDRAW_FIELD,
            Self::PENDING_POOL_TOKEN_WITHDRAW_FIELD,
            Self::EXTRA_FIELDS_FIELD,
        ];
    }
    impl StakingPool {
        pub fn path_builder() -> StakingPoolFieldPathBuilder {
            StakingPoolFieldPathBuilder::new()
        }
    }
    pub struct StakingPoolFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl StakingPoolFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn id(mut self) -> String {
            self.path.push(StakingPool::ID_FIELD.name);
            self.finish()
        }
        pub fn activation_epoch(mut self) -> String {
            self.path.push(StakingPool::ACTIVATION_EPOCH_FIELD.name);
            self.finish()
        }
        pub fn deactivation_epoch(mut self) -> String {
            self.path.push(StakingPool::DEACTIVATION_EPOCH_FIELD.name);
            self.finish()
        }
        pub fn sui_balance(mut self) -> String {
            self.path.push(StakingPool::SUI_BALANCE_FIELD.name);
            self.finish()
        }
        pub fn rewards_pool(mut self) -> String {
            self.path.push(StakingPool::REWARDS_POOL_FIELD.name);
            self.finish()
        }
        pub fn pool_token_balance(mut self) -> String {
            self.path.push(StakingPool::POOL_TOKEN_BALANCE_FIELD.name);
            self.finish()
        }
        pub fn exchange_rates(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(StakingPool::EXCHANGE_RATES_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
        pub fn pending_stake(mut self) -> String {
            self.path.push(StakingPool::PENDING_STAKE_FIELD.name);
            self.finish()
        }
        pub fn pending_total_sui_withdraw(mut self) -> String {
            self.path.push(StakingPool::PENDING_TOTAL_SUI_WITHDRAW_FIELD.name);
            self.finish()
        }
        pub fn pending_pool_token_withdraw(mut self) -> String {
            self.path.push(StakingPool::PENDING_POOL_TOKEN_WITHDRAW_FIELD.name);
            self.finish()
        }
        pub fn extra_fields(mut self) -> MoveTableFieldPathBuilder {
            self.path.push(StakingPool::EXTRA_FIELDS_FIELD.name);
            MoveTableFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl Transaction {
        pub const BCS_FIELD: &'static MessageField = &MessageField {
            name: "bcs",
            json_name: "bcs",
            number: 1i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 2i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 3i32,
            message_fields: None,
        };
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 4i32,
            message_fields: Some(TransactionKind::FIELDS),
        };
        pub const SENDER_FIELD: &'static MessageField = &MessageField {
            name: "sender",
            json_name: "sender",
            number: 5i32,
            message_fields: None,
        };
        pub const GAS_PAYMENT_FIELD: &'static MessageField = &MessageField {
            name: "gas_payment",
            json_name: "gasPayment",
            number: 6i32,
            message_fields: Some(GasPayment::FIELDS),
        };
        pub const EXPIRATION_FIELD: &'static MessageField = &MessageField {
            name: "expiration",
            json_name: "expiration",
            number: 7i32,
            message_fields: Some(TransactionExpiration::FIELDS),
        };
    }
    impl MessageFields for Transaction {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::BCS_FIELD,
            Self::DIGEST_FIELD,
            Self::VERSION_FIELD,
            Self::KIND_FIELD,
            Self::SENDER_FIELD,
            Self::GAS_PAYMENT_FIELD,
            Self::EXPIRATION_FIELD,
        ];
    }
    impl Transaction {
        pub fn path_builder() -> TransactionFieldPathBuilder {
            TransactionFieldPathBuilder::new()
        }
    }
    pub struct TransactionFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TransactionFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn bcs(mut self) -> BcsFieldPathBuilder {
            self.path.push(Transaction::BCS_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn digest(mut self) -> String {
            self.path.push(Transaction::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(Transaction::VERSION_FIELD.name);
            self.finish()
        }
        pub fn kind(mut self) -> TransactionKindFieldPathBuilder {
            self.path.push(Transaction::KIND_FIELD.name);
            TransactionKindFieldPathBuilder::new_with_base(self.path)
        }
        pub fn sender(mut self) -> String {
            self.path.push(Transaction::SENDER_FIELD.name);
            self.finish()
        }
        pub fn gas_payment(mut self) -> GasPaymentFieldPathBuilder {
            self.path.push(Transaction::GAS_PAYMENT_FIELD.name);
            GasPaymentFieldPathBuilder::new_with_base(self.path)
        }
        pub fn expiration(mut self) -> TransactionExpirationFieldPathBuilder {
            self.path.push(Transaction::EXPIRATION_FIELD.name);
            TransactionExpirationFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl GasPayment {
        pub const OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "objects",
            json_name: "objects",
            number: 1i32,
            message_fields: Some(ObjectReference::FIELDS),
        };
        pub const OWNER_FIELD: &'static MessageField = &MessageField {
            name: "owner",
            json_name: "owner",
            number: 2i32,
            message_fields: None,
        };
        pub const PRICE_FIELD: &'static MessageField = &MessageField {
            name: "price",
            json_name: "price",
            number: 3i32,
            message_fields: None,
        };
        pub const BUDGET_FIELD: &'static MessageField = &MessageField {
            name: "budget",
            json_name: "budget",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for GasPayment {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECTS_FIELD,
            Self::OWNER_FIELD,
            Self::PRICE_FIELD,
            Self::BUDGET_FIELD,
        ];
    }
    impl GasPayment {
        pub fn path_builder() -> GasPaymentFieldPathBuilder {
            GasPaymentFieldPathBuilder::new()
        }
    }
    pub struct GasPaymentFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GasPaymentFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn objects(mut self) -> ObjectReferenceFieldPathBuilder {
            self.path.push(GasPayment::OBJECTS_FIELD.name);
            ObjectReferenceFieldPathBuilder::new_with_base(self.path)
        }
        pub fn owner(mut self) -> String {
            self.path.push(GasPayment::OWNER_FIELD.name);
            self.finish()
        }
        pub fn price(mut self) -> String {
            self.path.push(GasPayment::PRICE_FIELD.name);
            self.finish()
        }
        pub fn budget(mut self) -> String {
            self.path.push(GasPayment::BUDGET_FIELD.name);
            self.finish()
        }
    }
    impl TransactionExpiration {
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 1i32,
            message_fields: None,
        };
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 2i32,
            message_fields: None,
        };
        pub const MIN_EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "min_epoch",
            json_name: "minEpoch",
            number: 3i32,
            message_fields: None,
        };
        pub const MIN_TIMESTAMP_FIELD: &'static MessageField = &MessageField {
            name: "min_timestamp",
            json_name: "minTimestamp",
            number: 4i32,
            message_fields: None,
        };
        pub const MAX_TIMESTAMP_FIELD: &'static MessageField = &MessageField {
            name: "max_timestamp",
            json_name: "maxTimestamp",
            number: 5i32,
            message_fields: None,
        };
        pub const CHAIN_FIELD: &'static MessageField = &MessageField {
            name: "chain",
            json_name: "chain",
            number: 6i32,
            message_fields: None,
        };
        pub const NONCE_FIELD: &'static MessageField = &MessageField {
            name: "nonce",
            json_name: "nonce",
            number: 7i32,
            message_fields: None,
        };
    }
    impl MessageFields for TransactionExpiration {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::EPOCH_FIELD,
            Self::MIN_EPOCH_FIELD,
            Self::MIN_TIMESTAMP_FIELD,
            Self::MAX_TIMESTAMP_FIELD,
            Self::CHAIN_FIELD,
            Self::NONCE_FIELD,
        ];
    }
    impl TransactionExpiration {
        pub fn path_builder() -> TransactionExpirationFieldPathBuilder {
            TransactionExpirationFieldPathBuilder::new()
        }
    }
    pub struct TransactionExpirationFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TransactionExpirationFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kind(mut self) -> String {
            self.path.push(TransactionExpiration::KIND_FIELD.name);
            self.finish()
        }
        pub fn epoch(mut self) -> String {
            self.path.push(TransactionExpiration::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn min_epoch(mut self) -> String {
            self.path.push(TransactionExpiration::MIN_EPOCH_FIELD.name);
            self.finish()
        }
        pub fn min_timestamp(mut self) -> String {
            self.path.push(TransactionExpiration::MIN_TIMESTAMP_FIELD.name);
            self.finish()
        }
        pub fn max_timestamp(mut self) -> String {
            self.path.push(TransactionExpiration::MAX_TIMESTAMP_FIELD.name);
            self.finish()
        }
        pub fn chain(mut self) -> String {
            self.path.push(TransactionExpiration::CHAIN_FIELD.name);
            self.finish()
        }
        pub fn nonce(mut self) -> String {
            self.path.push(TransactionExpiration::NONCE_FIELD.name);
            self.finish()
        }
    }
    impl TransactionKind {
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 1i32,
            message_fields: None,
        };
        pub const PROGRAMMABLE_TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "programmable_transaction",
            json_name: "programmableTransaction",
            number: 2i32,
            message_fields: Some(ProgrammableTransaction::FIELDS),
        };
        pub const CHANGE_EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "change_epoch",
            json_name: "changeEpoch",
            number: 3i32,
            message_fields: Some(ChangeEpoch::FIELDS),
        };
        pub const GENESIS_FIELD: &'static MessageField = &MessageField {
            name: "genesis",
            json_name: "genesis",
            number: 4i32,
            message_fields: Some(GenesisTransaction::FIELDS),
        };
        pub const CONSENSUS_COMMIT_PROLOGUE_FIELD: &'static MessageField = &MessageField {
            name: "consensus_commit_prologue",
            json_name: "consensusCommitPrologue",
            number: 5i32,
            message_fields: Some(ConsensusCommitPrologue::FIELDS),
        };
        pub const AUTHENTICATOR_STATE_UPDATE_FIELD: &'static MessageField = &MessageField {
            name: "authenticator_state_update",
            json_name: "authenticatorStateUpdate",
            number: 6i32,
            message_fields: Some(AuthenticatorStateUpdate::FIELDS),
        };
        pub const END_OF_EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "end_of_epoch",
            json_name: "endOfEpoch",
            number: 7i32,
            message_fields: Some(EndOfEpochTransaction::FIELDS),
        };
        pub const RANDOMNESS_STATE_UPDATE_FIELD: &'static MessageField = &MessageField {
            name: "randomness_state_update",
            json_name: "randomnessStateUpdate",
            number: 8i32,
            message_fields: Some(RandomnessStateUpdate::FIELDS),
        };
    }
    impl MessageFields for TransactionKind {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::PROGRAMMABLE_TRANSACTION_FIELD,
            Self::CHANGE_EPOCH_FIELD,
            Self::GENESIS_FIELD,
            Self::CONSENSUS_COMMIT_PROLOGUE_FIELD,
            Self::AUTHENTICATOR_STATE_UPDATE_FIELD,
            Self::END_OF_EPOCH_FIELD,
            Self::RANDOMNESS_STATE_UPDATE_FIELD,
        ];
    }
    impl TransactionKind {
        pub fn path_builder() -> TransactionKindFieldPathBuilder {
            TransactionKindFieldPathBuilder::new()
        }
    }
    pub struct TransactionKindFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TransactionKindFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kind(mut self) -> String {
            self.path.push(TransactionKind::KIND_FIELD.name);
            self.finish()
        }
        pub fn programmable_transaction(
            mut self,
        ) -> ProgrammableTransactionFieldPathBuilder {
            self.path.push(TransactionKind::PROGRAMMABLE_TRANSACTION_FIELD.name);
            ProgrammableTransactionFieldPathBuilder::new_with_base(self.path)
        }
        pub fn change_epoch(mut self) -> ChangeEpochFieldPathBuilder {
            self.path.push(TransactionKind::CHANGE_EPOCH_FIELD.name);
            ChangeEpochFieldPathBuilder::new_with_base(self.path)
        }
        pub fn genesis(mut self) -> GenesisTransactionFieldPathBuilder {
            self.path.push(TransactionKind::GENESIS_FIELD.name);
            GenesisTransactionFieldPathBuilder::new_with_base(self.path)
        }
        pub fn consensus_commit_prologue(
            mut self,
        ) -> ConsensusCommitPrologueFieldPathBuilder {
            self.path.push(TransactionKind::CONSENSUS_COMMIT_PROLOGUE_FIELD.name);
            ConsensusCommitPrologueFieldPathBuilder::new_with_base(self.path)
        }
        pub fn authenticator_state_update(
            mut self,
        ) -> AuthenticatorStateUpdateFieldPathBuilder {
            self.path.push(TransactionKind::AUTHENTICATOR_STATE_UPDATE_FIELD.name);
            AuthenticatorStateUpdateFieldPathBuilder::new_with_base(self.path)
        }
        pub fn end_of_epoch(mut self) -> EndOfEpochTransactionFieldPathBuilder {
            self.path.push(TransactionKind::END_OF_EPOCH_FIELD.name);
            EndOfEpochTransactionFieldPathBuilder::new_with_base(self.path)
        }
        pub fn randomness_state_update(
            mut self,
        ) -> RandomnessStateUpdateFieldPathBuilder {
            self.path.push(TransactionKind::RANDOMNESS_STATE_UPDATE_FIELD.name);
            RandomnessStateUpdateFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ProgrammableTransaction {
        pub const INPUTS_FIELD: &'static MessageField = &MessageField {
            name: "inputs",
            json_name: "inputs",
            number: 1i32,
            message_fields: Some(Input::FIELDS),
        };
        pub const COMMANDS_FIELD: &'static MessageField = &MessageField {
            name: "commands",
            json_name: "commands",
            number: 2i32,
            message_fields: Some(Command::FIELDS),
        };
    }
    impl MessageFields for ProgrammableTransaction {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::INPUTS_FIELD,
            Self::COMMANDS_FIELD,
        ];
    }
    impl ProgrammableTransaction {
        pub fn path_builder() -> ProgrammableTransactionFieldPathBuilder {
            ProgrammableTransactionFieldPathBuilder::new()
        }
    }
    pub struct ProgrammableTransactionFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ProgrammableTransactionFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn inputs(mut self) -> InputFieldPathBuilder {
            self.path.push(ProgrammableTransaction::INPUTS_FIELD.name);
            InputFieldPathBuilder::new_with_base(self.path)
        }
        pub fn commands(mut self) -> CommandFieldPathBuilder {
            self.path.push(ProgrammableTransaction::COMMANDS_FIELD.name);
            CommandFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl Command {
        pub const MOVE_CALL_FIELD: &'static MessageField = &MessageField {
            name: "move_call",
            json_name: "moveCall",
            number: 1i32,
            message_fields: Some(MoveCall::FIELDS),
        };
        pub const TRANSFER_OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "transfer_objects",
            json_name: "transferObjects",
            number: 2i32,
            message_fields: Some(TransferObjects::FIELDS),
        };
        pub const SPLIT_COINS_FIELD: &'static MessageField = &MessageField {
            name: "split_coins",
            json_name: "splitCoins",
            number: 3i32,
            message_fields: Some(SplitCoins::FIELDS),
        };
        pub const MERGE_COINS_FIELD: &'static MessageField = &MessageField {
            name: "merge_coins",
            json_name: "mergeCoins",
            number: 4i32,
            message_fields: Some(MergeCoins::FIELDS),
        };
        pub const PUBLISH_FIELD: &'static MessageField = &MessageField {
            name: "publish",
            json_name: "publish",
            number: 5i32,
            message_fields: Some(Publish::FIELDS),
        };
        pub const MAKE_MOVE_VECTOR_FIELD: &'static MessageField = &MessageField {
            name: "make_move_vector",
            json_name: "makeMoveVector",
            number: 6i32,
            message_fields: Some(MakeMoveVector::FIELDS),
        };
        pub const UPGRADE_FIELD: &'static MessageField = &MessageField {
            name: "upgrade",
            json_name: "upgrade",
            number: 7i32,
            message_fields: Some(Upgrade::FIELDS),
        };
    }
    impl MessageFields for Command {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::MOVE_CALL_FIELD,
            Self::TRANSFER_OBJECTS_FIELD,
            Self::SPLIT_COINS_FIELD,
            Self::MERGE_COINS_FIELD,
            Self::PUBLISH_FIELD,
            Self::MAKE_MOVE_VECTOR_FIELD,
            Self::UPGRADE_FIELD,
        ];
    }
    impl Command {
        pub fn path_builder() -> CommandFieldPathBuilder {
            CommandFieldPathBuilder::new()
        }
    }
    pub struct CommandFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CommandFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn move_call(mut self) -> MoveCallFieldPathBuilder {
            self.path.push(Command::MOVE_CALL_FIELD.name);
            MoveCallFieldPathBuilder::new_with_base(self.path)
        }
        pub fn transfer_objects(mut self) -> TransferObjectsFieldPathBuilder {
            self.path.push(Command::TRANSFER_OBJECTS_FIELD.name);
            TransferObjectsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn split_coins(mut self) -> SplitCoinsFieldPathBuilder {
            self.path.push(Command::SPLIT_COINS_FIELD.name);
            SplitCoinsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn merge_coins(mut self) -> MergeCoinsFieldPathBuilder {
            self.path.push(Command::MERGE_COINS_FIELD.name);
            MergeCoinsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn publish(mut self) -> PublishFieldPathBuilder {
            self.path.push(Command::PUBLISH_FIELD.name);
            PublishFieldPathBuilder::new_with_base(self.path)
        }
        pub fn make_move_vector(mut self) -> MakeMoveVectorFieldPathBuilder {
            self.path.push(Command::MAKE_MOVE_VECTOR_FIELD.name);
            MakeMoveVectorFieldPathBuilder::new_with_base(self.path)
        }
        pub fn upgrade(mut self) -> UpgradeFieldPathBuilder {
            self.path.push(Command::UPGRADE_FIELD.name);
            UpgradeFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MoveCall {
        pub const PACKAGE_FIELD: &'static MessageField = &MessageField {
            name: "package",
            json_name: "package",
            number: 1i32,
            message_fields: None,
        };
        pub const MODULE_FIELD: &'static MessageField = &MessageField {
            name: "module",
            json_name: "module",
            number: 2i32,
            message_fields: None,
        };
        pub const FUNCTION_FIELD: &'static MessageField = &MessageField {
            name: "function",
            json_name: "function",
            number: 3i32,
            message_fields: None,
        };
        pub const TYPE_ARGUMENTS_FIELD: &'static MessageField = &MessageField {
            name: "type_arguments",
            json_name: "typeArguments",
            number: 4i32,
            message_fields: None,
        };
        pub const ARGUMENTS_FIELD: &'static MessageField = &MessageField {
            name: "arguments",
            json_name: "arguments",
            number: 5i32,
            message_fields: Some(Argument::FIELDS),
        };
    }
    impl MessageFields for MoveCall {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PACKAGE_FIELD,
            Self::MODULE_FIELD,
            Self::FUNCTION_FIELD,
            Self::TYPE_ARGUMENTS_FIELD,
            Self::ARGUMENTS_FIELD,
        ];
    }
    impl MoveCall {
        pub fn path_builder() -> MoveCallFieldPathBuilder {
            MoveCallFieldPathBuilder::new()
        }
    }
    pub struct MoveCallFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MoveCallFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn package(mut self) -> String {
            self.path.push(MoveCall::PACKAGE_FIELD.name);
            self.finish()
        }
        pub fn module(mut self) -> String {
            self.path.push(MoveCall::MODULE_FIELD.name);
            self.finish()
        }
        pub fn function(mut self) -> String {
            self.path.push(MoveCall::FUNCTION_FIELD.name);
            self.finish()
        }
        pub fn type_arguments(mut self) -> String {
            self.path.push(MoveCall::TYPE_ARGUMENTS_FIELD.name);
            self.finish()
        }
        pub fn arguments(mut self) -> ArgumentFieldPathBuilder {
            self.path.push(MoveCall::ARGUMENTS_FIELD.name);
            ArgumentFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl TransferObjects {
        pub const OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "objects",
            json_name: "objects",
            number: 1i32,
            message_fields: Some(Argument::FIELDS),
        };
        pub const ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "address",
            json_name: "address",
            number: 2i32,
            message_fields: Some(Argument::FIELDS),
        };
    }
    impl MessageFields for TransferObjects {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECTS_FIELD,
            Self::ADDRESS_FIELD,
        ];
    }
    impl TransferObjects {
        pub fn path_builder() -> TransferObjectsFieldPathBuilder {
            TransferObjectsFieldPathBuilder::new()
        }
    }
    pub struct TransferObjectsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TransferObjectsFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn objects(mut self) -> ArgumentFieldPathBuilder {
            self.path.push(TransferObjects::OBJECTS_FIELD.name);
            ArgumentFieldPathBuilder::new_with_base(self.path)
        }
        pub fn address(mut self) -> ArgumentFieldPathBuilder {
            self.path.push(TransferObjects::ADDRESS_FIELD.name);
            ArgumentFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl SplitCoins {
        pub const COIN_FIELD: &'static MessageField = &MessageField {
            name: "coin",
            json_name: "coin",
            number: 1i32,
            message_fields: Some(Argument::FIELDS),
        };
        pub const AMOUNTS_FIELD: &'static MessageField = &MessageField {
            name: "amounts",
            json_name: "amounts",
            number: 2i32,
            message_fields: Some(Argument::FIELDS),
        };
    }
    impl MessageFields for SplitCoins {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::COIN_FIELD,
            Self::AMOUNTS_FIELD,
        ];
    }
    impl SplitCoins {
        pub fn path_builder() -> SplitCoinsFieldPathBuilder {
            SplitCoinsFieldPathBuilder::new()
        }
    }
    pub struct SplitCoinsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SplitCoinsFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn coin(mut self) -> ArgumentFieldPathBuilder {
            self.path.push(SplitCoins::COIN_FIELD.name);
            ArgumentFieldPathBuilder::new_with_base(self.path)
        }
        pub fn amounts(mut self) -> ArgumentFieldPathBuilder {
            self.path.push(SplitCoins::AMOUNTS_FIELD.name);
            ArgumentFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MergeCoins {
        pub const COIN_FIELD: &'static MessageField = &MessageField {
            name: "coin",
            json_name: "coin",
            number: 1i32,
            message_fields: Some(Argument::FIELDS),
        };
        pub const COINS_TO_MERGE_FIELD: &'static MessageField = &MessageField {
            name: "coins_to_merge",
            json_name: "coinsToMerge",
            number: 2i32,
            message_fields: Some(Argument::FIELDS),
        };
    }
    impl MessageFields for MergeCoins {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::COIN_FIELD,
            Self::COINS_TO_MERGE_FIELD,
        ];
    }
    impl MergeCoins {
        pub fn path_builder() -> MergeCoinsFieldPathBuilder {
            MergeCoinsFieldPathBuilder::new()
        }
    }
    pub struct MergeCoinsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MergeCoinsFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn coin(mut self) -> ArgumentFieldPathBuilder {
            self.path.push(MergeCoins::COIN_FIELD.name);
            ArgumentFieldPathBuilder::new_with_base(self.path)
        }
        pub fn coins_to_merge(mut self) -> ArgumentFieldPathBuilder {
            self.path.push(MergeCoins::COINS_TO_MERGE_FIELD.name);
            ArgumentFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl Publish {
        pub const MODULES_FIELD: &'static MessageField = &MessageField {
            name: "modules",
            json_name: "modules",
            number: 1i32,
            message_fields: None,
        };
        pub const DEPENDENCIES_FIELD: &'static MessageField = &MessageField {
            name: "dependencies",
            json_name: "dependencies",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for Publish {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::MODULES_FIELD,
            Self::DEPENDENCIES_FIELD,
        ];
    }
    impl Publish {
        pub fn path_builder() -> PublishFieldPathBuilder {
            PublishFieldPathBuilder::new()
        }
    }
    pub struct PublishFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl PublishFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn modules(mut self) -> String {
            self.path.push(Publish::MODULES_FIELD.name);
            self.finish()
        }
        pub fn dependencies(mut self) -> String {
            self.path.push(Publish::DEPENDENCIES_FIELD.name);
            self.finish()
        }
    }
    impl MakeMoveVector {
        pub const ELEMENT_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "element_type",
            json_name: "elementType",
            number: 1i32,
            message_fields: None,
        };
        pub const ELEMENTS_FIELD: &'static MessageField = &MessageField {
            name: "elements",
            json_name: "elements",
            number: 2i32,
            message_fields: Some(Argument::FIELDS),
        };
    }
    impl MessageFields for MakeMoveVector {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ELEMENT_TYPE_FIELD,
            Self::ELEMENTS_FIELD,
        ];
    }
    impl MakeMoveVector {
        pub fn path_builder() -> MakeMoveVectorFieldPathBuilder {
            MakeMoveVectorFieldPathBuilder::new()
        }
    }
    pub struct MakeMoveVectorFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MakeMoveVectorFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn element_type(mut self) -> String {
            self.path.push(MakeMoveVector::ELEMENT_TYPE_FIELD.name);
            self.finish()
        }
        pub fn elements(mut self) -> ArgumentFieldPathBuilder {
            self.path.push(MakeMoveVector::ELEMENTS_FIELD.name);
            ArgumentFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl Upgrade {
        pub const MODULES_FIELD: &'static MessageField = &MessageField {
            name: "modules",
            json_name: "modules",
            number: 1i32,
            message_fields: None,
        };
        pub const DEPENDENCIES_FIELD: &'static MessageField = &MessageField {
            name: "dependencies",
            json_name: "dependencies",
            number: 2i32,
            message_fields: None,
        };
        pub const PACKAGE_FIELD: &'static MessageField = &MessageField {
            name: "package",
            json_name: "package",
            number: 3i32,
            message_fields: None,
        };
        pub const TICKET_FIELD: &'static MessageField = &MessageField {
            name: "ticket",
            json_name: "ticket",
            number: 4i32,
            message_fields: Some(Argument::FIELDS),
        };
    }
    impl MessageFields for Upgrade {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::MODULES_FIELD,
            Self::DEPENDENCIES_FIELD,
            Self::PACKAGE_FIELD,
            Self::TICKET_FIELD,
        ];
    }
    impl Upgrade {
        pub fn path_builder() -> UpgradeFieldPathBuilder {
            UpgradeFieldPathBuilder::new()
        }
    }
    pub struct UpgradeFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl UpgradeFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn modules(mut self) -> String {
            self.path.push(Upgrade::MODULES_FIELD.name);
            self.finish()
        }
        pub fn dependencies(mut self) -> String {
            self.path.push(Upgrade::DEPENDENCIES_FIELD.name);
            self.finish()
        }
        pub fn package(mut self) -> String {
            self.path.push(Upgrade::PACKAGE_FIELD.name);
            self.finish()
        }
        pub fn ticket(mut self) -> ArgumentFieldPathBuilder {
            self.path.push(Upgrade::TICKET_FIELD.name);
            ArgumentFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl RandomnessStateUpdate {
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 1i32,
            message_fields: None,
        };
        pub const RANDOMNESS_ROUND_FIELD: &'static MessageField = &MessageField {
            name: "randomness_round",
            json_name: "randomnessRound",
            number: 2i32,
            message_fields: None,
        };
        pub const RANDOM_BYTES_FIELD: &'static MessageField = &MessageField {
            name: "random_bytes",
            json_name: "randomBytes",
            number: 3i32,
            message_fields: None,
        };
        pub const RANDOMNESS_OBJECT_INITIAL_SHARED_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "randomness_object_initial_shared_version",
            json_name: "randomnessObjectInitialSharedVersion",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for RandomnessStateUpdate {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EPOCH_FIELD,
            Self::RANDOMNESS_ROUND_FIELD,
            Self::RANDOM_BYTES_FIELD,
            Self::RANDOMNESS_OBJECT_INITIAL_SHARED_VERSION_FIELD,
        ];
    }
    impl RandomnessStateUpdate {
        pub fn path_builder() -> RandomnessStateUpdateFieldPathBuilder {
            RandomnessStateUpdateFieldPathBuilder::new()
        }
    }
    pub struct RandomnessStateUpdateFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl RandomnessStateUpdateFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn epoch(mut self) -> String {
            self.path.push(RandomnessStateUpdate::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn randomness_round(mut self) -> String {
            self.path.push(RandomnessStateUpdate::RANDOMNESS_ROUND_FIELD.name);
            self.finish()
        }
        pub fn random_bytes(mut self) -> String {
            self.path.push(RandomnessStateUpdate::RANDOM_BYTES_FIELD.name);
            self.finish()
        }
        pub fn randomness_object_initial_shared_version(mut self) -> String {
            self.path
                .push(
                    RandomnessStateUpdate::RANDOMNESS_OBJECT_INITIAL_SHARED_VERSION_FIELD
                        .name,
                );
            self.finish()
        }
    }
    impl ChangeEpoch {
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 1i32,
            message_fields: None,
        };
        pub const PROTOCOL_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "protocol_version",
            json_name: "protocolVersion",
            number: 2i32,
            message_fields: None,
        };
        pub const STORAGE_CHARGE_FIELD: &'static MessageField = &MessageField {
            name: "storage_charge",
            json_name: "storageCharge",
            number: 3i32,
            message_fields: None,
        };
        pub const COMPUTATION_CHARGE_FIELD: &'static MessageField = &MessageField {
            name: "computation_charge",
            json_name: "computationCharge",
            number: 4i32,
            message_fields: None,
        };
        pub const STORAGE_REBATE_FIELD: &'static MessageField = &MessageField {
            name: "storage_rebate",
            json_name: "storageRebate",
            number: 5i32,
            message_fields: None,
        };
        pub const NON_REFUNDABLE_STORAGE_FEE_FIELD: &'static MessageField = &MessageField {
            name: "non_refundable_storage_fee",
            json_name: "nonRefundableStorageFee",
            number: 6i32,
            message_fields: None,
        };
        pub const EPOCH_START_TIMESTAMP_FIELD: &'static MessageField = &MessageField {
            name: "epoch_start_timestamp",
            json_name: "epochStartTimestamp",
            number: 7i32,
            message_fields: None,
        };
        pub const SYSTEM_PACKAGES_FIELD: &'static MessageField = &MessageField {
            name: "system_packages",
            json_name: "systemPackages",
            number: 8i32,
            message_fields: Some(SystemPackage::FIELDS),
        };
    }
    impl MessageFields for ChangeEpoch {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EPOCH_FIELD,
            Self::PROTOCOL_VERSION_FIELD,
            Self::STORAGE_CHARGE_FIELD,
            Self::COMPUTATION_CHARGE_FIELD,
            Self::STORAGE_REBATE_FIELD,
            Self::NON_REFUNDABLE_STORAGE_FEE_FIELD,
            Self::EPOCH_START_TIMESTAMP_FIELD,
            Self::SYSTEM_PACKAGES_FIELD,
        ];
    }
    impl ChangeEpoch {
        pub fn path_builder() -> ChangeEpochFieldPathBuilder {
            ChangeEpochFieldPathBuilder::new()
        }
    }
    pub struct ChangeEpochFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ChangeEpochFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn epoch(mut self) -> String {
            self.path.push(ChangeEpoch::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn protocol_version(mut self) -> String {
            self.path.push(ChangeEpoch::PROTOCOL_VERSION_FIELD.name);
            self.finish()
        }
        pub fn storage_charge(mut self) -> String {
            self.path.push(ChangeEpoch::STORAGE_CHARGE_FIELD.name);
            self.finish()
        }
        pub fn computation_charge(mut self) -> String {
            self.path.push(ChangeEpoch::COMPUTATION_CHARGE_FIELD.name);
            self.finish()
        }
        pub fn storage_rebate(mut self) -> String {
            self.path.push(ChangeEpoch::STORAGE_REBATE_FIELD.name);
            self.finish()
        }
        pub fn non_refundable_storage_fee(mut self) -> String {
            self.path.push(ChangeEpoch::NON_REFUNDABLE_STORAGE_FEE_FIELD.name);
            self.finish()
        }
        pub fn epoch_start_timestamp(mut self) -> String {
            self.path.push(ChangeEpoch::EPOCH_START_TIMESTAMP_FIELD.name);
            self.finish()
        }
        pub fn system_packages(mut self) -> SystemPackageFieldPathBuilder {
            self.path.push(ChangeEpoch::SYSTEM_PACKAGES_FIELD.name);
            SystemPackageFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl SystemPackage {
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 1i32,
            message_fields: None,
        };
        pub const MODULES_FIELD: &'static MessageField = &MessageField {
            name: "modules",
            json_name: "modules",
            number: 2i32,
            message_fields: None,
        };
        pub const DEPENDENCIES_FIELD: &'static MessageField = &MessageField {
            name: "dependencies",
            json_name: "dependencies",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for SystemPackage {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::VERSION_FIELD,
            Self::MODULES_FIELD,
            Self::DEPENDENCIES_FIELD,
        ];
    }
    impl SystemPackage {
        pub fn path_builder() -> SystemPackageFieldPathBuilder {
            SystemPackageFieldPathBuilder::new()
        }
    }
    pub struct SystemPackageFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SystemPackageFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn version(mut self) -> String {
            self.path.push(SystemPackage::VERSION_FIELD.name);
            self.finish()
        }
        pub fn modules(mut self) -> String {
            self.path.push(SystemPackage::MODULES_FIELD.name);
            self.finish()
        }
        pub fn dependencies(mut self) -> String {
            self.path.push(SystemPackage::DEPENDENCIES_FIELD.name);
            self.finish()
        }
    }
    impl GenesisTransaction {
        pub const OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "objects",
            json_name: "objects",
            number: 1i32,
            message_fields: Some(Object::FIELDS),
        };
    }
    impl MessageFields for GenesisTransaction {
        const FIELDS: &'static [&'static MessageField] = &[Self::OBJECTS_FIELD];
    }
    impl GenesisTransaction {
        pub fn path_builder() -> GenesisTransactionFieldPathBuilder {
            GenesisTransactionFieldPathBuilder::new()
        }
    }
    pub struct GenesisTransactionFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GenesisTransactionFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn objects(mut self) -> ObjectFieldPathBuilder {
            self.path.push(GenesisTransaction::OBJECTS_FIELD.name);
            ObjectFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ConsensusCommitPrologue {
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 1i32,
            message_fields: None,
        };
        pub const ROUND_FIELD: &'static MessageField = &MessageField {
            name: "round",
            json_name: "round",
            number: 2i32,
            message_fields: None,
        };
        pub const COMMIT_TIMESTAMP_FIELD: &'static MessageField = &MessageField {
            name: "commit_timestamp",
            json_name: "commitTimestamp",
            number: 3i32,
            message_fields: None,
        };
        pub const CONSENSUS_COMMIT_DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "consensus_commit_digest",
            json_name: "consensusCommitDigest",
            number: 4i32,
            message_fields: None,
        };
        pub const SUB_DAG_INDEX_FIELD: &'static MessageField = &MessageField {
            name: "sub_dag_index",
            json_name: "subDagIndex",
            number: 5i32,
            message_fields: None,
        };
        pub const CONSENSUS_DETERMINED_VERSION_ASSIGNMENTS_FIELD: &'static MessageField = &MessageField {
            name: "consensus_determined_version_assignments",
            json_name: "consensusDeterminedVersionAssignments",
            number: 6i32,
            message_fields: Some(ConsensusDeterminedVersionAssignments::FIELDS),
        };
        pub const ADDITIONAL_STATE_DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "additional_state_digest",
            json_name: "additionalStateDigest",
            number: 7i32,
            message_fields: None,
        };
    }
    impl MessageFields for ConsensusCommitPrologue {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EPOCH_FIELD,
            Self::ROUND_FIELD,
            Self::COMMIT_TIMESTAMP_FIELD,
            Self::CONSENSUS_COMMIT_DIGEST_FIELD,
            Self::SUB_DAG_INDEX_FIELD,
            Self::CONSENSUS_DETERMINED_VERSION_ASSIGNMENTS_FIELD,
            Self::ADDITIONAL_STATE_DIGEST_FIELD,
        ];
    }
    impl ConsensusCommitPrologue {
        pub fn path_builder() -> ConsensusCommitPrologueFieldPathBuilder {
            ConsensusCommitPrologueFieldPathBuilder::new()
        }
    }
    pub struct ConsensusCommitPrologueFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ConsensusCommitPrologueFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn epoch(mut self) -> String {
            self.path.push(ConsensusCommitPrologue::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn round(mut self) -> String {
            self.path.push(ConsensusCommitPrologue::ROUND_FIELD.name);
            self.finish()
        }
        pub fn commit_timestamp(mut self) -> String {
            self.path.push(ConsensusCommitPrologue::COMMIT_TIMESTAMP_FIELD.name);
            self.finish()
        }
        pub fn consensus_commit_digest(mut self) -> String {
            self.path.push(ConsensusCommitPrologue::CONSENSUS_COMMIT_DIGEST_FIELD.name);
            self.finish()
        }
        pub fn sub_dag_index(mut self) -> String {
            self.path.push(ConsensusCommitPrologue::SUB_DAG_INDEX_FIELD.name);
            self.finish()
        }
        pub fn consensus_determined_version_assignments(
            mut self,
        ) -> ConsensusDeterminedVersionAssignmentsFieldPathBuilder {
            self.path
                .push(
                    ConsensusCommitPrologue::CONSENSUS_DETERMINED_VERSION_ASSIGNMENTS_FIELD
                        .name,
                );
            ConsensusDeterminedVersionAssignmentsFieldPathBuilder::new_with_base(
                self.path,
            )
        }
        pub fn additional_state_digest(mut self) -> String {
            self.path.push(ConsensusCommitPrologue::ADDITIONAL_STATE_DIGEST_FIELD.name);
            self.finish()
        }
    }
    impl VersionAssignment {
        pub const OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "object_id",
            json_name: "objectId",
            number: 1i32,
            message_fields: None,
        };
        pub const START_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "start_version",
            json_name: "startVersion",
            number: 2i32,
            message_fields: None,
        };
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for VersionAssignment {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECT_ID_FIELD,
            Self::START_VERSION_FIELD,
            Self::VERSION_FIELD,
        ];
    }
    impl VersionAssignment {
        pub fn path_builder() -> VersionAssignmentFieldPathBuilder {
            VersionAssignmentFieldPathBuilder::new()
        }
    }
    pub struct VersionAssignmentFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl VersionAssignmentFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn object_id(mut self) -> String {
            self.path.push(VersionAssignment::OBJECT_ID_FIELD.name);
            self.finish()
        }
        pub fn start_version(mut self) -> String {
            self.path.push(VersionAssignment::START_VERSION_FIELD.name);
            self.finish()
        }
        pub fn version(mut self) -> String {
            self.path.push(VersionAssignment::VERSION_FIELD.name);
            self.finish()
        }
    }
    impl CanceledTransaction {
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 1i32,
            message_fields: None,
        };
        pub const VERSION_ASSIGNMENTS_FIELD: &'static MessageField = &MessageField {
            name: "version_assignments",
            json_name: "versionAssignments",
            number: 2i32,
            message_fields: Some(VersionAssignment::FIELDS),
        };
    }
    impl MessageFields for CanceledTransaction {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::DIGEST_FIELD,
            Self::VERSION_ASSIGNMENTS_FIELD,
        ];
    }
    impl CanceledTransaction {
        pub fn path_builder() -> CanceledTransactionFieldPathBuilder {
            CanceledTransactionFieldPathBuilder::new()
        }
    }
    pub struct CanceledTransactionFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CanceledTransactionFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn digest(mut self) -> String {
            self.path.push(CanceledTransaction::DIGEST_FIELD.name);
            self.finish()
        }
        pub fn version_assignments(mut self) -> VersionAssignmentFieldPathBuilder {
            self.path.push(CanceledTransaction::VERSION_ASSIGNMENTS_FIELD.name);
            VersionAssignmentFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ConsensusDeterminedVersionAssignments {
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 1i32,
            message_fields: None,
        };
        pub const CANCELED_TRANSACTIONS_FIELD: &'static MessageField = &MessageField {
            name: "canceled_transactions",
            json_name: "canceledTransactions",
            number: 3i32,
            message_fields: Some(CanceledTransaction::FIELDS),
        };
    }
    impl MessageFields for ConsensusDeterminedVersionAssignments {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::VERSION_FIELD,
            Self::CANCELED_TRANSACTIONS_FIELD,
        ];
    }
    impl ConsensusDeterminedVersionAssignments {
        pub fn path_builder() -> ConsensusDeterminedVersionAssignmentsFieldPathBuilder {
            ConsensusDeterminedVersionAssignmentsFieldPathBuilder::new()
        }
    }
    pub struct ConsensusDeterminedVersionAssignmentsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ConsensusDeterminedVersionAssignmentsFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn version(mut self) -> String {
            self.path.push(ConsensusDeterminedVersionAssignments::VERSION_FIELD.name);
            self.finish()
        }
        pub fn canceled_transactions(mut self) -> CanceledTransactionFieldPathBuilder {
            self.path
                .push(
                    ConsensusDeterminedVersionAssignments::CANCELED_TRANSACTIONS_FIELD
                        .name,
                );
            CanceledTransactionFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl AuthenticatorStateUpdate {
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 1i32,
            message_fields: None,
        };
        pub const ROUND_FIELD: &'static MessageField = &MessageField {
            name: "round",
            json_name: "round",
            number: 2i32,
            message_fields: None,
        };
        pub const NEW_ACTIVE_JWKS_FIELD: &'static MessageField = &MessageField {
            name: "new_active_jwks",
            json_name: "newActiveJwks",
            number: 3i32,
            message_fields: Some(ActiveJwk::FIELDS),
        };
        pub const AUTHENTICATOR_OBJECT_INITIAL_SHARED_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "authenticator_object_initial_shared_version",
            json_name: "authenticatorObjectInitialSharedVersion",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for AuthenticatorStateUpdate {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EPOCH_FIELD,
            Self::ROUND_FIELD,
            Self::NEW_ACTIVE_JWKS_FIELD,
            Self::AUTHENTICATOR_OBJECT_INITIAL_SHARED_VERSION_FIELD,
        ];
    }
    impl AuthenticatorStateUpdate {
        pub fn path_builder() -> AuthenticatorStateUpdateFieldPathBuilder {
            AuthenticatorStateUpdateFieldPathBuilder::new()
        }
    }
    pub struct AuthenticatorStateUpdateFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl AuthenticatorStateUpdateFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn epoch(mut self) -> String {
            self.path.push(AuthenticatorStateUpdate::EPOCH_FIELD.name);
            self.finish()
        }
        pub fn round(mut self) -> String {
            self.path.push(AuthenticatorStateUpdate::ROUND_FIELD.name);
            self.finish()
        }
        pub fn new_active_jwks(mut self) -> ActiveJwkFieldPathBuilder {
            self.path.push(AuthenticatorStateUpdate::NEW_ACTIVE_JWKS_FIELD.name);
            ActiveJwkFieldPathBuilder::new_with_base(self.path)
        }
        pub fn authenticator_object_initial_shared_version(mut self) -> String {
            self.path
                .push(
                    AuthenticatorStateUpdate::AUTHENTICATOR_OBJECT_INITIAL_SHARED_VERSION_FIELD
                        .name,
                );
            self.finish()
        }
    }
    impl ActiveJwk {
        pub const ID_FIELD: &'static MessageField = &MessageField {
            name: "id",
            json_name: "id",
            number: 1i32,
            message_fields: Some(JwkId::FIELDS),
        };
        pub const JWK_FIELD: &'static MessageField = &MessageField {
            name: "jwk",
            json_name: "jwk",
            number: 2i32,
            message_fields: Some(Jwk::FIELDS),
        };
        pub const EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "epoch",
            json_name: "epoch",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for ActiveJwk {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ID_FIELD,
            Self::JWK_FIELD,
            Self::EPOCH_FIELD,
        ];
    }
    impl ActiveJwk {
        pub fn path_builder() -> ActiveJwkFieldPathBuilder {
            ActiveJwkFieldPathBuilder::new()
        }
    }
    pub struct ActiveJwkFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ActiveJwkFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn id(mut self) -> JwkIdFieldPathBuilder {
            self.path.push(ActiveJwk::ID_FIELD.name);
            JwkIdFieldPathBuilder::new_with_base(self.path)
        }
        pub fn jwk(mut self) -> JwkFieldPathBuilder {
            self.path.push(ActiveJwk::JWK_FIELD.name);
            JwkFieldPathBuilder::new_with_base(self.path)
        }
        pub fn epoch(mut self) -> String {
            self.path.push(ActiveJwk::EPOCH_FIELD.name);
            self.finish()
        }
    }
    impl EndOfEpochTransaction {
        pub const TRANSACTIONS_FIELD: &'static MessageField = &MessageField {
            name: "transactions",
            json_name: "transactions",
            number: 1i32,
            message_fields: Some(EndOfEpochTransactionKind::FIELDS),
        };
    }
    impl MessageFields for EndOfEpochTransaction {
        const FIELDS: &'static [&'static MessageField] = &[Self::TRANSACTIONS_FIELD];
    }
    impl EndOfEpochTransaction {
        pub fn path_builder() -> EndOfEpochTransactionFieldPathBuilder {
            EndOfEpochTransactionFieldPathBuilder::new()
        }
    }
    pub struct EndOfEpochTransactionFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EndOfEpochTransactionFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn transactions(mut self) -> EndOfEpochTransactionKindFieldPathBuilder {
            self.path.push(EndOfEpochTransaction::TRANSACTIONS_FIELD.name);
            EndOfEpochTransactionKindFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl EndOfEpochTransactionKind {
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 1i32,
            message_fields: None,
        };
        pub const CHANGE_EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "change_epoch",
            json_name: "changeEpoch",
            number: 2i32,
            message_fields: Some(ChangeEpoch::FIELDS),
        };
        pub const AUTHENTICATOR_STATE_EXPIRE_FIELD: &'static MessageField = &MessageField {
            name: "authenticator_state_expire",
            json_name: "authenticatorStateExpire",
            number: 3i32,
            message_fields: Some(AuthenticatorStateExpire::FIELDS),
        };
        pub const EXECUTION_TIME_OBSERVATIONS_FIELD: &'static MessageField = &MessageField {
            name: "execution_time_observations",
            json_name: "executionTimeObservations",
            number: 4i32,
            message_fields: Some(ExecutionTimeObservations::FIELDS),
        };
        pub const BRIDGE_CHAIN_ID_FIELD: &'static MessageField = &MessageField {
            name: "bridge_chain_id",
            json_name: "bridgeChainId",
            number: 5i32,
            message_fields: None,
        };
        pub const BRIDGE_OBJECT_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "bridge_object_version",
            json_name: "bridgeObjectVersion",
            number: 6i32,
            message_fields: None,
        };
    }
    impl MessageFields for EndOfEpochTransactionKind {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::CHANGE_EPOCH_FIELD,
            Self::AUTHENTICATOR_STATE_EXPIRE_FIELD,
            Self::EXECUTION_TIME_OBSERVATIONS_FIELD,
            Self::BRIDGE_CHAIN_ID_FIELD,
            Self::BRIDGE_OBJECT_VERSION_FIELD,
        ];
    }
    impl EndOfEpochTransactionKind {
        pub fn path_builder() -> EndOfEpochTransactionKindFieldPathBuilder {
            EndOfEpochTransactionKindFieldPathBuilder::new()
        }
    }
    pub struct EndOfEpochTransactionKindFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EndOfEpochTransactionKindFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kind(mut self) -> String {
            self.path.push(EndOfEpochTransactionKind::KIND_FIELD.name);
            self.finish()
        }
        pub fn change_epoch(mut self) -> ChangeEpochFieldPathBuilder {
            self.path.push(EndOfEpochTransactionKind::CHANGE_EPOCH_FIELD.name);
            ChangeEpochFieldPathBuilder::new_with_base(self.path)
        }
        pub fn authenticator_state_expire(
            mut self,
        ) -> AuthenticatorStateExpireFieldPathBuilder {
            self.path
                .push(EndOfEpochTransactionKind::AUTHENTICATOR_STATE_EXPIRE_FIELD.name);
            AuthenticatorStateExpireFieldPathBuilder::new_with_base(self.path)
        }
        pub fn execution_time_observations(
            mut self,
        ) -> ExecutionTimeObservationsFieldPathBuilder {
            self.path
                .push(EndOfEpochTransactionKind::EXECUTION_TIME_OBSERVATIONS_FIELD.name);
            ExecutionTimeObservationsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn bridge_chain_id(mut self) -> String {
            self.path.push(EndOfEpochTransactionKind::BRIDGE_CHAIN_ID_FIELD.name);
            self.finish()
        }
        pub fn bridge_object_version(mut self) -> String {
            self.path.push(EndOfEpochTransactionKind::BRIDGE_OBJECT_VERSION_FIELD.name);
            self.finish()
        }
    }
    impl AuthenticatorStateExpire {
        pub const MIN_EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "min_epoch",
            json_name: "minEpoch",
            number: 1i32,
            message_fields: None,
        };
        pub const AUTHENTICATOR_OBJECT_INITIAL_SHARED_VERSION_FIELD: &'static MessageField = &MessageField {
            name: "authenticator_object_initial_shared_version",
            json_name: "authenticatorObjectInitialSharedVersion",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for AuthenticatorStateExpire {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::MIN_EPOCH_FIELD,
            Self::AUTHENTICATOR_OBJECT_INITIAL_SHARED_VERSION_FIELD,
        ];
    }
    impl AuthenticatorStateExpire {
        pub fn path_builder() -> AuthenticatorStateExpireFieldPathBuilder {
            AuthenticatorStateExpireFieldPathBuilder::new()
        }
    }
    pub struct AuthenticatorStateExpireFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl AuthenticatorStateExpireFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn min_epoch(mut self) -> String {
            self.path.push(AuthenticatorStateExpire::MIN_EPOCH_FIELD.name);
            self.finish()
        }
        pub fn authenticator_object_initial_shared_version(mut self) -> String {
            self.path
                .push(
                    AuthenticatorStateExpire::AUTHENTICATOR_OBJECT_INITIAL_SHARED_VERSION_FIELD
                        .name,
                );
            self.finish()
        }
    }
    impl ExecutionTimeObservations {
        pub const VERSION_FIELD: &'static MessageField = &MessageField {
            name: "version",
            json_name: "version",
            number: 1i32,
            message_fields: None,
        };
        pub const OBSERVATIONS_FIELD: &'static MessageField = &MessageField {
            name: "observations",
            json_name: "observations",
            number: 2i32,
            message_fields: Some(ExecutionTimeObservation::FIELDS),
        };
    }
    impl MessageFields for ExecutionTimeObservations {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::VERSION_FIELD,
            Self::OBSERVATIONS_FIELD,
        ];
    }
    impl ExecutionTimeObservations {
        pub fn path_builder() -> ExecutionTimeObservationsFieldPathBuilder {
            ExecutionTimeObservationsFieldPathBuilder::new()
        }
    }
    pub struct ExecutionTimeObservationsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ExecutionTimeObservationsFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn version(mut self) -> String {
            self.path.push(ExecutionTimeObservations::VERSION_FIELD.name);
            self.finish()
        }
        pub fn observations(mut self) -> ExecutionTimeObservationFieldPathBuilder {
            self.path.push(ExecutionTimeObservations::OBSERVATIONS_FIELD.name);
            ExecutionTimeObservationFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ExecutionTimeObservation {
        pub const KIND_FIELD: &'static MessageField = &MessageField {
            name: "kind",
            json_name: "kind",
            number: 1i32,
            message_fields: None,
        };
        pub const MOVE_ENTRY_POINT_FIELD: &'static MessageField = &MessageField {
            name: "move_entry_point",
            json_name: "moveEntryPoint",
            number: 2i32,
            message_fields: Some(MoveCall::FIELDS),
        };
        pub const VALIDATOR_OBSERVATIONS_FIELD: &'static MessageField = &MessageField {
            name: "validator_observations",
            json_name: "validatorObservations",
            number: 3i32,
            message_fields: Some(ValidatorExecutionTimeObservation::FIELDS),
        };
    }
    impl MessageFields for ExecutionTimeObservation {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::MOVE_ENTRY_POINT_FIELD,
            Self::VALIDATOR_OBSERVATIONS_FIELD,
        ];
    }
    impl ExecutionTimeObservation {
        pub fn path_builder() -> ExecutionTimeObservationFieldPathBuilder {
            ExecutionTimeObservationFieldPathBuilder::new()
        }
    }
    pub struct ExecutionTimeObservationFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ExecutionTimeObservationFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn kind(mut self) -> String {
            self.path.push(ExecutionTimeObservation::KIND_FIELD.name);
            self.finish()
        }
        pub fn move_entry_point(mut self) -> MoveCallFieldPathBuilder {
            self.path.push(ExecutionTimeObservation::MOVE_ENTRY_POINT_FIELD.name);
            MoveCallFieldPathBuilder::new_with_base(self.path)
        }
        pub fn validator_observations(
            mut self,
        ) -> ValidatorExecutionTimeObservationFieldPathBuilder {
            self.path.push(ExecutionTimeObservation::VALIDATOR_OBSERVATIONS_FIELD.name);
            ValidatorExecutionTimeObservationFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ValidatorExecutionTimeObservation {
        pub const VALIDATOR_FIELD: &'static MessageField = &MessageField {
            name: "validator",
            json_name: "validator",
            number: 1i32,
            message_fields: None,
        };
        pub const DURATION_FIELD: &'static MessageField = &MessageField {
            name: "duration",
            json_name: "duration",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for ValidatorExecutionTimeObservation {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::VALIDATOR_FIELD,
            Self::DURATION_FIELD,
        ];
    }
    impl ValidatorExecutionTimeObservation {
        pub fn path_builder() -> ValidatorExecutionTimeObservationFieldPathBuilder {
            ValidatorExecutionTimeObservationFieldPathBuilder::new()
        }
    }
    pub struct ValidatorExecutionTimeObservationFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ValidatorExecutionTimeObservationFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn validator(mut self) -> String {
            self.path.push(ValidatorExecutionTimeObservation::VALIDATOR_FIELD.name);
            self.finish()
        }
        pub fn duration(mut self) -> String {
            self.path.push(ValidatorExecutionTimeObservation::DURATION_FIELD.name);
            self.finish()
        }
    }
    impl ExecuteTransactionRequest {
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 1i32,
            message_fields: Some(Transaction::FIELDS),
        };
        pub const SIGNATURES_FIELD: &'static MessageField = &MessageField {
            name: "signatures",
            json_name: "signatures",
            number: 2i32,
            message_fields: Some(UserSignature::FIELDS),
        };
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for ExecuteTransactionRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TRANSACTION_FIELD,
            Self::SIGNATURES_FIELD,
            Self::READ_MASK_FIELD,
        ];
    }
    impl ExecuteTransactionRequest {
        pub fn path_builder() -> ExecuteTransactionRequestFieldPathBuilder {
            ExecuteTransactionRequestFieldPathBuilder::new()
        }
    }
    pub struct ExecuteTransactionRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ExecuteTransactionRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn transaction(mut self) -> TransactionFieldPathBuilder {
            self.path.push(ExecuteTransactionRequest::TRANSACTION_FIELD.name);
            TransactionFieldPathBuilder::new_with_base(self.path)
        }
        pub fn signatures(mut self) -> UserSignatureFieldPathBuilder {
            self.path.push(ExecuteTransactionRequest::SIGNATURES_FIELD.name);
            UserSignatureFieldPathBuilder::new_with_base(self.path)
        }
        pub fn read_mask(mut self) -> String {
            self.path.push(ExecuteTransactionRequest::READ_MASK_FIELD.name);
            self.finish()
        }
    }
    impl ExecuteTransactionResponse {
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 1i32,
            message_fields: Some(ExecutedTransaction::FIELDS),
        };
    }
    impl MessageFields for ExecuteTransactionResponse {
        const FIELDS: &'static [&'static MessageField] = &[Self::TRANSACTION_FIELD];
    }
    impl ExecuteTransactionResponse {
        pub fn path_builder() -> ExecuteTransactionResponseFieldPathBuilder {
            ExecuteTransactionResponseFieldPathBuilder::new()
        }
    }
    pub struct ExecuteTransactionResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ExecuteTransactionResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn transaction(mut self) -> ExecutedTransactionFieldPathBuilder {
            self.path.push(ExecuteTransactionResponse::TRANSACTION_FIELD.name);
            ExecutedTransactionFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl SimulateTransactionRequest {
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 1i32,
            message_fields: Some(Transaction::FIELDS),
        };
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 2i32,
            message_fields: None,
        };
        pub const CHECKS_FIELD: &'static MessageField = &MessageField {
            name: "checks",
            json_name: "checks",
            number: 3i32,
            message_fields: None,
        };
        pub const DO_GAS_SELECTION_FIELD: &'static MessageField = &MessageField {
            name: "do_gas_selection",
            json_name: "doGasSelection",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for SimulateTransactionRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TRANSACTION_FIELD,
            Self::READ_MASK_FIELD,
            Self::CHECKS_FIELD,
            Self::DO_GAS_SELECTION_FIELD,
        ];
    }
    impl SimulateTransactionRequest {
        pub fn path_builder() -> SimulateTransactionRequestFieldPathBuilder {
            SimulateTransactionRequestFieldPathBuilder::new()
        }
    }
    pub struct SimulateTransactionRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SimulateTransactionRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn transaction(mut self) -> TransactionFieldPathBuilder {
            self.path.push(SimulateTransactionRequest::TRANSACTION_FIELD.name);
            TransactionFieldPathBuilder::new_with_base(self.path)
        }
        pub fn read_mask(mut self) -> String {
            self.path.push(SimulateTransactionRequest::READ_MASK_FIELD.name);
            self.finish()
        }
        pub fn checks(mut self) -> String {
            self.path.push(SimulateTransactionRequest::CHECKS_FIELD.name);
            self.finish()
        }
        pub fn do_gas_selection(mut self) -> String {
            self.path.push(SimulateTransactionRequest::DO_GAS_SELECTION_FIELD.name);
            self.finish()
        }
    }
    impl SimulateTransactionResponse {
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 1i32,
            message_fields: Some(ExecutedTransaction::FIELDS),
        };
        pub const COMMAND_OUTPUTS_FIELD: &'static MessageField = &MessageField {
            name: "command_outputs",
            json_name: "commandOutputs",
            number: 2i32,
            message_fields: Some(CommandResult::FIELDS),
        };
    }
    impl MessageFields for SimulateTransactionResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TRANSACTION_FIELD,
            Self::COMMAND_OUTPUTS_FIELD,
        ];
    }
    impl SimulateTransactionResponse {
        pub fn path_builder() -> SimulateTransactionResponseFieldPathBuilder {
            SimulateTransactionResponseFieldPathBuilder::new()
        }
    }
    pub struct SimulateTransactionResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SimulateTransactionResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn transaction(mut self) -> ExecutedTransactionFieldPathBuilder {
            self.path.push(SimulateTransactionResponse::TRANSACTION_FIELD.name);
            ExecutedTransactionFieldPathBuilder::new_with_base(self.path)
        }
        pub fn command_outputs(mut self) -> CommandResultFieldPathBuilder {
            self.path.push(SimulateTransactionResponse::COMMAND_OUTPUTS_FIELD.name);
            CommandResultFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl CommandResult {
        pub const RETURN_VALUES_FIELD: &'static MessageField = &MessageField {
            name: "return_values",
            json_name: "returnValues",
            number: 1i32,
            message_fields: Some(CommandOutput::FIELDS),
        };
        pub const MUTATED_BY_REF_FIELD: &'static MessageField = &MessageField {
            name: "mutated_by_ref",
            json_name: "mutatedByRef",
            number: 2i32,
            message_fields: Some(CommandOutput::FIELDS),
        };
    }
    impl MessageFields for CommandResult {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::RETURN_VALUES_FIELD,
            Self::MUTATED_BY_REF_FIELD,
        ];
    }
    impl CommandResult {
        pub fn path_builder() -> CommandResultFieldPathBuilder {
            CommandResultFieldPathBuilder::new()
        }
    }
    pub struct CommandResultFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CommandResultFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn return_values(mut self) -> CommandOutputFieldPathBuilder {
            self.path.push(CommandResult::RETURN_VALUES_FIELD.name);
            CommandOutputFieldPathBuilder::new_with_base(self.path)
        }
        pub fn mutated_by_ref(mut self) -> CommandOutputFieldPathBuilder {
            self.path.push(CommandResult::MUTATED_BY_REF_FIELD.name);
            CommandOutputFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl CommandOutput {
        pub const ARGUMENT_FIELD: &'static MessageField = &MessageField {
            name: "argument",
            json_name: "argument",
            number: 1i32,
            message_fields: Some(Argument::FIELDS),
        };
        pub const VALUE_FIELD: &'static MessageField = &MessageField {
            name: "value",
            json_name: "value",
            number: 2i32,
            message_fields: Some(Bcs::FIELDS),
        };
        pub const JSON_FIELD: &'static MessageField = &MessageField {
            name: "json",
            json_name: "json",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for CommandOutput {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ARGUMENT_FIELD,
            Self::VALUE_FIELD,
            Self::JSON_FIELD,
        ];
    }
    impl CommandOutput {
        pub fn path_builder() -> CommandOutputFieldPathBuilder {
            CommandOutputFieldPathBuilder::new()
        }
    }
    pub struct CommandOutputFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CommandOutputFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn argument(mut self) -> ArgumentFieldPathBuilder {
            self.path.push(CommandOutput::ARGUMENT_FIELD.name);
            ArgumentFieldPathBuilder::new_with_base(self.path)
        }
        pub fn value(mut self) -> BcsFieldPathBuilder {
            self.path.push(CommandOutput::VALUE_FIELD.name);
            BcsFieldPathBuilder::new_with_base(self.path)
        }
        pub fn json(mut self) -> String {
            self.path.push(CommandOutput::JSON_FIELD.name);
            self.finish()
        }
    }
}
