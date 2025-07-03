mod _field_impls {
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
    }
    impl MessageFields for Checkpoint {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::SEQUENCE_NUMBER_FIELD,
            Self::DIGEST_FIELD,
            Self::SUMMARY_FIELD,
            Self::SIGNATURE_FIELD,
            Self::CONTENTS_FIELD,
            Self::TRANSACTIONS_FIELD,
        ];
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
    }
    impl MessageFields for CheckpointedTransactionInfo {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TRANSACTION_FIELD,
            Self::EFFECTS_FIELD,
            Self::SIGNATURES_FIELD,
        ];
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
        pub const UNCHANGED_SHARED_OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "unchanged_shared_objects",
            json_name: "unchangedSharedObjects",
            number: 13i32,
            message_fields: Some(UnchangedSharedObject::FIELDS),
        };
        pub const AUXILIARY_DATA_DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "auxiliary_data_digest",
            json_name: "auxiliaryDataDigest",
            number: 14i32,
            message_fields: None,
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
            Self::UNCHANGED_SHARED_OBJECTS_FIELD,
            Self::AUXILIARY_DATA_DIGEST_FIELD,
        ];
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
            Self::ID_OPERATION_FIELD,
            Self::OBJECT_TYPE_FIELD,
        ];
    }
    impl UnchangedSharedObject {
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
    impl MessageFields for UnchangedSharedObject {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::OBJECT_ID_FIELD,
            Self::VERSION_FIELD,
            Self::DIGEST_FIELD,
            Self::OBJECT_TYPE_FIELD,
        ];
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
        pub const INPUT_OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "input_objects",
            json_name: "inputObjects",
            number: 10i32,
            message_fields: Some(Object::FIELDS),
        };
        pub const OUTPUT_OBJECTS_FIELD: &'static MessageField = &MessageField {
            name: "output_objects",
            json_name: "outputObjects",
            number: 11i32,
            message_fields: Some(Object::FIELDS),
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
            Self::INPUT_OBJECTS_FIELD,
            Self::OUTPUT_OBJECTS_FIELD,
        ];
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
            Self::LITERAL_FIELD,
        ];
    }
    impl GetServiceInfoRequest {}
    impl MessageFields for GetServiceInfoRequest {
        const FIELDS: &'static [&'static MessageField] = &[];
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
    }
    impl MessageFields for CoinMetadata {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ID_FIELD,
            Self::DECIMALS_FIELD,
            Self::NAME_FIELD,
            Self::SYMBOL_FIELD,
            Self::DESCRIPTION_FIELD,
            Self::ICON_URL_FIELD,
        ];
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
    }
    impl MessageFields for CoinTreasury {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ID_FIELD,
            Self::TOTAL_SUPPLY_FIELD,
        ];
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
    }
    impl MessageFields for RegulatedCoinMetadata {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ID_FIELD,
            Self::COIN_METADATA_OBJECT_FIELD,
            Self::DENY_CAP_OBJECT_FIELD,
        ];
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
        pub const NAME_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "name_type",
            json_name: "nameType",
            number: 4i32,
            message_fields: None,
        };
        pub const NAME_VALUE_FIELD: &'static MessageField = &MessageField {
            name: "name_value",
            json_name: "nameValue",
            number: 5i32,
            message_fields: None,
        };
        pub const VALUE_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "value_type",
            json_name: "valueType",
            number: 6i32,
            message_fields: None,
        };
        pub const DYNAMIC_OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "dynamic_object_id",
            json_name: "dynamicObjectId",
            number: 7i32,
            message_fields: None,
        };
        pub const OBJECT_FIELD: &'static MessageField = &MessageField {
            name: "object",
            json_name: "object",
            number: 8i32,
            message_fields: Some(Object::FIELDS),
        };
    }
    impl MessageFields for DynamicField {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::PARENT_FIELD,
            Self::FIELD_ID_FIELD,
            Self::NAME_TYPE_FIELD,
            Self::NAME_VALUE_FIELD,
            Self::VALUE_TYPE_FIELD,
            Self::DYNAMIC_OBJECT_ID_FIELD,
            Self::OBJECT_FIELD,
        ];
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
    impl SimulateTransactionResponse {
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 1i32,
            message_fields: Some(ExecutedTransaction::FIELDS),
        };
        pub const OUTPUTS_FIELD: &'static MessageField = &MessageField {
            name: "outputs",
            json_name: "outputs",
            number: 2i32,
            message_fields: Some(CommandResult::FIELDS),
        };
    }
    impl MessageFields for SimulateTransactionResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TRANSACTION_FIELD,
            Self::OUTPUTS_FIELD,
        ];
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
    }
    impl MessageFields for ZkLoginAuthenticator {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::INPUTS_FIELD,
            Self::MAX_EPOCH_FIELD,
            Self::SIGNATURE_FIELD,
        ];
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
    }
    impl MessageFields for TransactionExpiration {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::KIND_FIELD,
            Self::EPOCH_FIELD,
        ];
    }
    impl TransactionKind {
        pub const PROGRAMMABLE_TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "programmable_transaction",
            json_name: "programmableTransaction",
            number: 2i32,
            message_fields: Some(ProgrammableTransaction::FIELDS),
        };
        pub const PROGRAMMABLE_SYSTEM_TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "programmable_system_transaction",
            json_name: "programmableSystemTransaction",
            number: 3i32,
            message_fields: Some(ProgrammableTransaction::FIELDS),
        };
        pub const CHANGE_EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "change_epoch",
            json_name: "changeEpoch",
            number: 100i32,
            message_fields: Some(ChangeEpoch::FIELDS),
        };
        pub const GENESIS_FIELD: &'static MessageField = &MessageField {
            name: "genesis",
            json_name: "genesis",
            number: 101i32,
            message_fields: Some(GenesisTransaction::FIELDS),
        };
        pub const CONSENSUS_COMMIT_PROLOGUE_V1_FIELD: &'static MessageField = &MessageField {
            name: "consensus_commit_prologue_v1",
            json_name: "consensusCommitPrologueV1",
            number: 102i32,
            message_fields: Some(ConsensusCommitPrologue::FIELDS),
        };
        pub const AUTHENTICATOR_STATE_UPDATE_FIELD: &'static MessageField = &MessageField {
            name: "authenticator_state_update",
            json_name: "authenticatorStateUpdate",
            number: 103i32,
            message_fields: Some(AuthenticatorStateUpdate::FIELDS),
        };
        pub const END_OF_EPOCH_FIELD: &'static MessageField = &MessageField {
            name: "end_of_epoch",
            json_name: "endOfEpoch",
            number: 104i32,
            message_fields: Some(EndOfEpochTransaction::FIELDS),
        };
        pub const RANDOMNESS_STATE_UPDATE_FIELD: &'static MessageField = &MessageField {
            name: "randomness_state_update",
            json_name: "randomnessStateUpdate",
            number: 105i32,
            message_fields: Some(RandomnessStateUpdate::FIELDS),
        };
        pub const CONSENSUS_COMMIT_PROLOGUE_V2_FIELD: &'static MessageField = &MessageField {
            name: "consensus_commit_prologue_v2",
            json_name: "consensusCommitPrologueV2",
            number: 106i32,
            message_fields: Some(ConsensusCommitPrologue::FIELDS),
        };
        pub const CONSENSUS_COMMIT_PROLOGUE_V3_FIELD: &'static MessageField = &MessageField {
            name: "consensus_commit_prologue_v3",
            json_name: "consensusCommitPrologueV3",
            number: 107i32,
            message_fields: Some(ConsensusCommitPrologue::FIELDS),
        };
        pub const CONSENSUS_COMMIT_PROLOGUE_V4_FIELD: &'static MessageField = &MessageField {
            name: "consensus_commit_prologue_v4",
            json_name: "consensusCommitPrologueV4",
            number: 108i32,
            message_fields: Some(ConsensusCommitPrologue::FIELDS),
        };
    }
    impl MessageFields for TransactionKind {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PROGRAMMABLE_TRANSACTION_FIELD,
            Self::PROGRAMMABLE_SYSTEM_TRANSACTION_FIELD,
            Self::CHANGE_EPOCH_FIELD,
            Self::GENESIS_FIELD,
            Self::CONSENSUS_COMMIT_PROLOGUE_V1_FIELD,
            Self::AUTHENTICATOR_STATE_UPDATE_FIELD,
            Self::END_OF_EPOCH_FIELD,
            Self::RANDOMNESS_STATE_UPDATE_FIELD,
            Self::CONSENSUS_COMMIT_PROLOGUE_V2_FIELD,
            Self::CONSENSUS_COMMIT_PROLOGUE_V3_FIELD,
            Self::CONSENSUS_COMMIT_PROLOGUE_V4_FIELD,
        ];
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
    impl EndOfEpochTransactionKind {
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
        pub const AUTHENTICATOR_STATE_CREATE_FIELD: &'static MessageField = &MessageField {
            name: "authenticator_state_create",
            json_name: "authenticatorStateCreate",
            number: 200i32,
            message_fields: None,
        };
        pub const RANDOMNESS_STATE_CREATE_FIELD: &'static MessageField = &MessageField {
            name: "randomness_state_create",
            json_name: "randomnessStateCreate",
            number: 201i32,
            message_fields: None,
        };
        pub const DENY_LIST_STATE_CREATE_FIELD: &'static MessageField = &MessageField {
            name: "deny_list_state_create",
            json_name: "denyListStateCreate",
            number: 202i32,
            message_fields: None,
        };
        pub const BRIDGE_STATE_CREATE_FIELD: &'static MessageField = &MessageField {
            name: "bridge_state_create",
            json_name: "bridgeStateCreate",
            number: 203i32,
            message_fields: None,
        };
        pub const BRIDGE_COMMITTEE_INIT_FIELD: &'static MessageField = &MessageField {
            name: "bridge_committee_init",
            json_name: "bridgeCommitteeInit",
            number: 204i32,
            message_fields: None,
        };
        pub const ACCUMULATOR_ROOT_CREATE_FIELD: &'static MessageField = &MessageField {
            name: "accumulator_root_create",
            json_name: "accumulatorRootCreate",
            number: 205i32,
            message_fields: None,
        };
        pub const COIN_REGISTRY_CREATE_FIELD: &'static MessageField = &MessageField {
            name: "coin_registry_create",
            json_name: "coinRegistryCreate",
            number: 206i32,
            message_fields: None,
        };
    }
    impl MessageFields for EndOfEpochTransactionKind {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CHANGE_EPOCH_FIELD,
            Self::AUTHENTICATOR_STATE_EXPIRE_FIELD,
            Self::EXECUTION_TIME_OBSERVATIONS_FIELD,
            Self::AUTHENTICATOR_STATE_CREATE_FIELD,
            Self::RANDOMNESS_STATE_CREATE_FIELD,
            Self::DENY_LIST_STATE_CREATE_FIELD,
            Self::BRIDGE_STATE_CREATE_FIELD,
            Self::BRIDGE_COMMITTEE_INIT_FIELD,
            Self::ACCUMULATOR_ROOT_CREATE_FIELD,
            Self::COIN_REGISTRY_CREATE_FIELD,
        ];
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
    impl ExecuteTransactionResponse {
        pub const FINALITY_FIELD: &'static MessageField = &MessageField {
            name: "finality",
            json_name: "finality",
            number: 1i32,
            message_fields: Some(TransactionFinality::FIELDS),
        };
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 2i32,
            message_fields: Some(ExecutedTransaction::FIELDS),
        };
    }
    impl MessageFields for ExecuteTransactionResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::FINALITY_FIELD,
            Self::TRANSACTION_FIELD,
        ];
    }
    impl TransactionFinality {
        pub const CERTIFIED_FIELD: &'static MessageField = &MessageField {
            name: "certified",
            json_name: "certified",
            number: 1i32,
            message_fields: Some(ValidatorAggregatedSignature::FIELDS),
        };
        pub const CHECKPOINTED_FIELD: &'static MessageField = &MessageField {
            name: "checkpointed",
            json_name: "checkpointed",
            number: 2i32,
            message_fields: None,
        };
        pub const QUORUM_EXECUTED_FIELD: &'static MessageField = &MessageField {
            name: "quorum_executed",
            json_name: "quorumExecuted",
            number: 3i32,
            message_fields: None,
        };
    }
    impl MessageFields for TransactionFinality {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CERTIFIED_FIELD,
            Self::CHECKPOINTED_FIELD,
            Self::QUORUM_EXECUTED_FIELD,
        ];
    }
}
