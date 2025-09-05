mod _getter_impls {
    #![allow(clippy::useless_conversion)]
    use super::*;
    impl Argument {
        pub const fn const_default() -> Self {
            Self {
                kind: None,
                input: None,
                result: None,
                subresult: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Argument = Argument::const_default();
            &DEFAULT
        }
        pub fn with_input(mut self, field: u32) -> Self {
            self.input = Some(field.into());
            self
        }
        pub fn with_result(mut self, field: u32) -> Self {
            self.result = Some(field.into());
            self
        }
        pub fn with_subresult(mut self, field: u32) -> Self {
            self.subresult = Some(field.into());
            self
        }
    }
    impl BalanceChange {
        pub const fn const_default() -> Self {
            Self {
                address: None,
                coin_type: None,
                amount: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: BalanceChange = BalanceChange::const_default();
            &DEFAULT
        }
        pub fn with_address(mut self, field: String) -> Self {
            self.address = Some(field.into());
            self
        }
        pub fn with_coin_type(mut self, field: String) -> Self {
            self.coin_type = Some(field.into());
            self
        }
        pub fn with_amount(mut self, field: String) -> Self {
            self.amount = Some(field.into());
            self
        }
    }
    impl Bcs {
        pub const fn const_default() -> Self {
            Self { name: None, value: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Bcs = Bcs::const_default();
            &DEFAULT
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
        pub fn with_value(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.value = Some(field.into());
            self
        }
    }
    impl Checkpoint {
        pub const fn const_default() -> Self {
            Self {
                sequence_number: None,
                digest: None,
                summary: None,
                signature: None,
                contents: None,
                transactions: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Checkpoint = Checkpoint::const_default();
            &DEFAULT
        }
        pub fn with_sequence_number(mut self, field: u64) -> Self {
            self.sequence_number = Some(field.into());
            self
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn summary(&self) -> &CheckpointSummary {
            self.summary
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| CheckpointSummary::default_instance() as _)
        }
        pub fn summary_opt(&self) -> Option<&CheckpointSummary> {
            self.summary.as_ref().map(|field| field as _)
        }
        pub fn summary_opt_mut(&mut self) -> Option<&mut CheckpointSummary> {
            self.summary.as_mut().map(|field| field as _)
        }
        pub fn summary_mut(&mut self) -> &mut CheckpointSummary {
            self.summary.get_or_insert_default()
        }
        pub fn with_summary(mut self, field: CheckpointSummary) -> Self {
            self.summary = Some(field.into());
            self
        }
        pub fn signature(&self) -> &ValidatorAggregatedSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ValidatorAggregatedSignature::default_instance() as _)
        }
        pub fn signature_opt(&self) -> Option<&ValidatorAggregatedSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        pub fn signature_opt_mut(
            &mut self,
        ) -> Option<&mut ValidatorAggregatedSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        pub fn signature_mut(&mut self) -> &mut ValidatorAggregatedSignature {
            self.signature.get_or_insert_default()
        }
        pub fn with_signature(mut self, field: ValidatorAggregatedSignature) -> Self {
            self.signature = Some(field.into());
            self
        }
        pub fn contents(&self) -> &CheckpointContents {
            self.contents
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| CheckpointContents::default_instance() as _)
        }
        pub fn contents_opt(&self) -> Option<&CheckpointContents> {
            self.contents.as_ref().map(|field| field as _)
        }
        pub fn contents_opt_mut(&mut self) -> Option<&mut CheckpointContents> {
            self.contents.as_mut().map(|field| field as _)
        }
        pub fn contents_mut(&mut self) -> &mut CheckpointContents {
            self.contents.get_or_insert_default()
        }
        pub fn with_contents(mut self, field: CheckpointContents) -> Self {
            self.contents = Some(field.into());
            self
        }
        pub fn transactions(&self) -> &[ExecutedTransaction] {
            &self.transactions
        }
        pub fn with_transactions(mut self, field: Vec<ExecutedTransaction>) -> Self {
            self.transactions = field;
            self
        }
    }
    impl CheckpointContents {
        pub const fn const_default() -> Self {
            Self {
                bcs: None,
                digest: None,
                version: None,
                transactions: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CheckpointContents = CheckpointContents::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Bcs::default_instance() as _)
        }
        pub fn bcs_opt(&self) -> Option<&Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn with_bcs(mut self, field: Bcs) -> Self {
            self.bcs = Some(field.into());
            self
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: i32) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn transactions(&self) -> &[CheckpointedTransactionInfo] {
            &self.transactions
        }
        pub fn with_transactions(
            mut self,
            field: Vec<CheckpointedTransactionInfo>,
        ) -> Self {
            self.transactions = field;
            self
        }
    }
    impl CheckpointedTransactionInfo {
        pub const fn const_default() -> Self {
            Self {
                transaction: None,
                effects: None,
                signatures: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CheckpointedTransactionInfo = CheckpointedTransactionInfo::const_default();
            &DEFAULT
        }
        pub fn with_transaction(mut self, field: String) -> Self {
            self.transaction = Some(field.into());
            self
        }
        pub fn with_effects(mut self, field: String) -> Self {
            self.effects = Some(field.into());
            self
        }
        pub fn signatures(&self) -> &[UserSignature] {
            &self.signatures
        }
        pub fn with_signatures(mut self, field: Vec<UserSignature>) -> Self {
            self.signatures = field;
            self
        }
    }
    impl CheckpointSummary {
        pub const fn const_default() -> Self {
            Self {
                bcs: None,
                digest: None,
                epoch: None,
                sequence_number: None,
                total_network_transactions: None,
                content_digest: None,
                previous_digest: None,
                epoch_rolling_gas_cost_summary: None,
                timestamp: None,
                commitments: Vec::new(),
                end_of_epoch_data: None,
                version_specific_data: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CheckpointSummary = CheckpointSummary::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Bcs::default_instance() as _)
        }
        pub fn bcs_opt(&self) -> Option<&Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn with_bcs(mut self, field: Bcs) -> Self {
            self.bcs = Some(field.into());
            self
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
        pub fn with_sequence_number(mut self, field: u64) -> Self {
            self.sequence_number = Some(field.into());
            self
        }
        pub fn with_total_network_transactions(mut self, field: u64) -> Self {
            self.total_network_transactions = Some(field.into());
            self
        }
        pub fn with_content_digest(mut self, field: String) -> Self {
            self.content_digest = Some(field.into());
            self
        }
        pub fn with_previous_digest(mut self, field: String) -> Self {
            self.previous_digest = Some(field.into());
            self
        }
        pub fn epoch_rolling_gas_cost_summary(&self) -> &GasCostSummary {
            self.epoch_rolling_gas_cost_summary
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| GasCostSummary::default_instance() as _)
        }
        pub fn epoch_rolling_gas_cost_summary_opt(&self) -> Option<&GasCostSummary> {
            self.epoch_rolling_gas_cost_summary.as_ref().map(|field| field as _)
        }
        pub fn epoch_rolling_gas_cost_summary_opt_mut(
            &mut self,
        ) -> Option<&mut GasCostSummary> {
            self.epoch_rolling_gas_cost_summary.as_mut().map(|field| field as _)
        }
        pub fn epoch_rolling_gas_cost_summary_mut(&mut self) -> &mut GasCostSummary {
            self.epoch_rolling_gas_cost_summary.get_or_insert_default()
        }
        pub fn with_epoch_rolling_gas_cost_summary(
            mut self,
            field: GasCostSummary,
        ) -> Self {
            self.epoch_rolling_gas_cost_summary = Some(field.into());
            self
        }
        pub fn commitments(&self) -> &[CheckpointCommitment] {
            &self.commitments
        }
        pub fn with_commitments(mut self, field: Vec<CheckpointCommitment>) -> Self {
            self.commitments = field;
            self
        }
        pub fn end_of_epoch_data(&self) -> &EndOfEpochData {
            self.end_of_epoch_data
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| EndOfEpochData::default_instance() as _)
        }
        pub fn end_of_epoch_data_opt(&self) -> Option<&EndOfEpochData> {
            self.end_of_epoch_data.as_ref().map(|field| field as _)
        }
        pub fn end_of_epoch_data_opt_mut(&mut self) -> Option<&mut EndOfEpochData> {
            self.end_of_epoch_data.as_mut().map(|field| field as _)
        }
        pub fn end_of_epoch_data_mut(&mut self) -> &mut EndOfEpochData {
            self.end_of_epoch_data.get_or_insert_default()
        }
        pub fn with_end_of_epoch_data(mut self, field: EndOfEpochData) -> Self {
            self.end_of_epoch_data = Some(field.into());
            self
        }
        pub fn with_version_specific_data(
            mut self,
            field: ::prost::bytes::Bytes,
        ) -> Self {
            self.version_specific_data = Some(field.into());
            self
        }
    }
    impl EndOfEpochData {
        pub const fn const_default() -> Self {
            Self {
                next_epoch_committee: Vec::new(),
                next_epoch_protocol_version: None,
                epoch_commitments: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: EndOfEpochData = EndOfEpochData::const_default();
            &DEFAULT
        }
        pub fn next_epoch_committee(&self) -> &[ValidatorCommitteeMember] {
            &self.next_epoch_committee
        }
        pub fn with_next_epoch_committee(
            mut self,
            field: Vec<ValidatorCommitteeMember>,
        ) -> Self {
            self.next_epoch_committee = field;
            self
        }
        pub fn with_next_epoch_protocol_version(mut self, field: u64) -> Self {
            self.next_epoch_protocol_version = Some(field.into());
            self
        }
        pub fn epoch_commitments(&self) -> &[CheckpointCommitment] {
            &self.epoch_commitments
        }
        pub fn with_epoch_commitments(
            mut self,
            field: Vec<CheckpointCommitment>,
        ) -> Self {
            self.epoch_commitments = field;
            self
        }
    }
    impl CheckpointCommitment {
        pub const fn const_default() -> Self {
            Self { kind: None, digest: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CheckpointCommitment = CheckpointCommitment::const_default();
            &DEFAULT
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
    }
    impl TransactionEffects {
        pub const fn const_default() -> Self {
            Self {
                bcs: None,
                digest: None,
                version: None,
                status: None,
                epoch: None,
                gas_used: None,
                transaction_digest: None,
                gas_object: None,
                events_digest: None,
                dependencies: Vec::new(),
                lamport_version: None,
                changed_objects: Vec::new(),
                unchanged_consensus_objects: Vec::new(),
                auxiliary_data_digest: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: TransactionEffects = TransactionEffects::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Bcs::default_instance() as _)
        }
        pub fn bcs_opt(&self) -> Option<&Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn with_bcs(mut self, field: Bcs) -> Self {
            self.bcs = Some(field.into());
            self
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: i32) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn status(&self) -> &ExecutionStatus {
            self.status
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ExecutionStatus::default_instance() as _)
        }
        pub fn status_opt(&self) -> Option<&ExecutionStatus> {
            self.status.as_ref().map(|field| field as _)
        }
        pub fn status_opt_mut(&mut self) -> Option<&mut ExecutionStatus> {
            self.status.as_mut().map(|field| field as _)
        }
        pub fn status_mut(&mut self) -> &mut ExecutionStatus {
            self.status.get_or_insert_default()
        }
        pub fn with_status(mut self, field: ExecutionStatus) -> Self {
            self.status = Some(field.into());
            self
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
        pub fn gas_used(&self) -> &GasCostSummary {
            self.gas_used
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| GasCostSummary::default_instance() as _)
        }
        pub fn gas_used_opt(&self) -> Option<&GasCostSummary> {
            self.gas_used.as_ref().map(|field| field as _)
        }
        pub fn gas_used_opt_mut(&mut self) -> Option<&mut GasCostSummary> {
            self.gas_used.as_mut().map(|field| field as _)
        }
        pub fn gas_used_mut(&mut self) -> &mut GasCostSummary {
            self.gas_used.get_or_insert_default()
        }
        pub fn with_gas_used(mut self, field: GasCostSummary) -> Self {
            self.gas_used = Some(field.into());
            self
        }
        pub fn with_transaction_digest(mut self, field: String) -> Self {
            self.transaction_digest = Some(field.into());
            self
        }
        pub fn gas_object(&self) -> &ChangedObject {
            self.gas_object
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ChangedObject::default_instance() as _)
        }
        pub fn gas_object_opt(&self) -> Option<&ChangedObject> {
            self.gas_object.as_ref().map(|field| field as _)
        }
        pub fn gas_object_opt_mut(&mut self) -> Option<&mut ChangedObject> {
            self.gas_object.as_mut().map(|field| field as _)
        }
        pub fn gas_object_mut(&mut self) -> &mut ChangedObject {
            self.gas_object.get_or_insert_default()
        }
        pub fn with_gas_object(mut self, field: ChangedObject) -> Self {
            self.gas_object = Some(field.into());
            self
        }
        pub fn with_events_digest(mut self, field: String) -> Self {
            self.events_digest = Some(field.into());
            self
        }
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        pub fn with_dependencies(mut self, field: Vec<String>) -> Self {
            self.dependencies = field;
            self
        }
        pub fn with_lamport_version(mut self, field: u64) -> Self {
            self.lamport_version = Some(field.into());
            self
        }
        pub fn changed_objects(&self) -> &[ChangedObject] {
            &self.changed_objects
        }
        pub fn with_changed_objects(mut self, field: Vec<ChangedObject>) -> Self {
            self.changed_objects = field;
            self
        }
        pub fn unchanged_consensus_objects(&self) -> &[UnchangedConsensusObject] {
            &self.unchanged_consensus_objects
        }
        pub fn with_unchanged_consensus_objects(
            mut self,
            field: Vec<UnchangedConsensusObject>,
        ) -> Self {
            self.unchanged_consensus_objects = field;
            self
        }
        pub fn with_auxiliary_data_digest(mut self, field: String) -> Self {
            self.auxiliary_data_digest = Some(field.into());
            self
        }
    }
    impl ChangedObject {
        pub const fn const_default() -> Self {
            Self {
                object_id: None,
                input_state: None,
                input_version: None,
                input_digest: None,
                input_owner: None,
                output_state: None,
                output_version: None,
                output_digest: None,
                output_owner: None,
                id_operation: None,
                object_type: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ChangedObject = ChangedObject::const_default();
            &DEFAULT
        }
        pub fn with_object_id(mut self, field: String) -> Self {
            self.object_id = Some(field.into());
            self
        }
        pub fn with_input_version(mut self, field: u64) -> Self {
            self.input_version = Some(field.into());
            self
        }
        pub fn with_input_digest(mut self, field: String) -> Self {
            self.input_digest = Some(field.into());
            self
        }
        pub fn input_owner(&self) -> &Owner {
            self.input_owner
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Owner::default_instance() as _)
        }
        pub fn input_owner_opt(&self) -> Option<&Owner> {
            self.input_owner.as_ref().map(|field| field as _)
        }
        pub fn input_owner_opt_mut(&mut self) -> Option<&mut Owner> {
            self.input_owner.as_mut().map(|field| field as _)
        }
        pub fn input_owner_mut(&mut self) -> &mut Owner {
            self.input_owner.get_or_insert_default()
        }
        pub fn with_input_owner(mut self, field: Owner) -> Self {
            self.input_owner = Some(field.into());
            self
        }
        pub fn with_output_version(mut self, field: u64) -> Self {
            self.output_version = Some(field.into());
            self
        }
        pub fn with_output_digest(mut self, field: String) -> Self {
            self.output_digest = Some(field.into());
            self
        }
        pub fn output_owner(&self) -> &Owner {
            self.output_owner
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Owner::default_instance() as _)
        }
        pub fn output_owner_opt(&self) -> Option<&Owner> {
            self.output_owner.as_ref().map(|field| field as _)
        }
        pub fn output_owner_opt_mut(&mut self) -> Option<&mut Owner> {
            self.output_owner.as_mut().map(|field| field as _)
        }
        pub fn output_owner_mut(&mut self) -> &mut Owner {
            self.output_owner.get_or_insert_default()
        }
        pub fn with_output_owner(mut self, field: Owner) -> Self {
            self.output_owner = Some(field.into());
            self
        }
        pub fn with_object_type(mut self, field: String) -> Self {
            self.object_type = Some(field.into());
            self
        }
    }
    impl UnchangedConsensusObject {
        pub const fn const_default() -> Self {
            Self {
                kind: None,
                object_id: None,
                version: None,
                digest: None,
                object_type: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: UnchangedConsensusObject = UnchangedConsensusObject::const_default();
            &DEFAULT
        }
        pub fn with_object_id(mut self, field: String) -> Self {
            self.object_id = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: u64) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn with_object_type(mut self, field: String) -> Self {
            self.object_type = Some(field.into());
            self
        }
    }
    impl Epoch {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                committee: None,
                system_state: None,
                first_checkpoint: None,
                last_checkpoint: None,
                start: None,
                end: None,
                reference_gas_price: None,
                protocol_config: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Epoch = Epoch::const_default();
            &DEFAULT
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
        pub fn committee(&self) -> &ValidatorCommittee {
            self.committee
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ValidatorCommittee::default_instance() as _)
        }
        pub fn committee_opt(&self) -> Option<&ValidatorCommittee> {
            self.committee.as_ref().map(|field| field as _)
        }
        pub fn committee_opt_mut(&mut self) -> Option<&mut ValidatorCommittee> {
            self.committee.as_mut().map(|field| field as _)
        }
        pub fn committee_mut(&mut self) -> &mut ValidatorCommittee {
            self.committee.get_or_insert_default()
        }
        pub fn with_committee(mut self, field: ValidatorCommittee) -> Self {
            self.committee = Some(field.into());
            self
        }
        pub fn system_state(&self) -> &SystemState {
            self.system_state
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| SystemState::default_instance() as _)
        }
        pub fn system_state_opt(&self) -> Option<&SystemState> {
            self.system_state.as_ref().map(|field| field as _)
        }
        pub fn system_state_opt_mut(&mut self) -> Option<&mut SystemState> {
            self.system_state.as_mut().map(|field| field as _)
        }
        pub fn system_state_mut(&mut self) -> &mut SystemState {
            self.system_state.get_or_insert_default()
        }
        pub fn with_system_state(mut self, field: SystemState) -> Self {
            self.system_state = Some(field.into());
            self
        }
        pub fn with_first_checkpoint(mut self, field: u64) -> Self {
            self.first_checkpoint = Some(field.into());
            self
        }
        pub fn with_last_checkpoint(mut self, field: u64) -> Self {
            self.last_checkpoint = Some(field.into());
            self
        }
        pub fn with_reference_gas_price(mut self, field: u64) -> Self {
            self.reference_gas_price = Some(field.into());
            self
        }
        pub fn protocol_config(&self) -> &ProtocolConfig {
            self.protocol_config
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ProtocolConfig::default_instance() as _)
        }
        pub fn protocol_config_opt(&self) -> Option<&ProtocolConfig> {
            self.protocol_config.as_ref().map(|field| field as _)
        }
        pub fn protocol_config_opt_mut(&mut self) -> Option<&mut ProtocolConfig> {
            self.protocol_config.as_mut().map(|field| field as _)
        }
        pub fn protocol_config_mut(&mut self) -> &mut ProtocolConfig {
            self.protocol_config.get_or_insert_default()
        }
        pub fn with_protocol_config(mut self, field: ProtocolConfig) -> Self {
            self.protocol_config = Some(field.into());
            self
        }
    }
    impl TransactionEvents {
        pub const fn const_default() -> Self {
            Self {
                bcs: None,
                digest: None,
                events: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: TransactionEvents = TransactionEvents::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Bcs::default_instance() as _)
        }
        pub fn bcs_opt(&self) -> Option<&Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn with_bcs(mut self, field: Bcs) -> Self {
            self.bcs = Some(field.into());
            self
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn events(&self) -> &[Event] {
            &self.events
        }
        pub fn with_events(mut self, field: Vec<Event>) -> Self {
            self.events = field;
            self
        }
    }
    impl Event {
        pub const fn const_default() -> Self {
            Self {
                package_id: None,
                module: None,
                sender: None,
                event_type: None,
                contents: None,
                json: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Event = Event::const_default();
            &DEFAULT
        }
        pub fn with_package_id(mut self, field: String) -> Self {
            self.package_id = Some(field.into());
            self
        }
        pub fn with_module(mut self, field: String) -> Self {
            self.module = Some(field.into());
            self
        }
        pub fn with_sender(mut self, field: String) -> Self {
            self.sender = Some(field.into());
            self
        }
        pub fn with_event_type(mut self, field: String) -> Self {
            self.event_type = Some(field.into());
            self
        }
        pub fn contents(&self) -> &Bcs {
            self.contents
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Bcs::default_instance() as _)
        }
        pub fn contents_opt(&self) -> Option<&Bcs> {
            self.contents.as_ref().map(|field| field as _)
        }
        pub fn contents_opt_mut(&mut self) -> Option<&mut Bcs> {
            self.contents.as_mut().map(|field| field as _)
        }
        pub fn contents_mut(&mut self) -> &mut Bcs {
            self.contents.get_or_insert_default()
        }
        pub fn with_contents(mut self, field: Bcs) -> Self {
            self.contents = Some(field.into());
            self
        }
    }
    impl ExecutedTransaction {
        pub const fn const_default() -> Self {
            Self {
                digest: None,
                transaction: None,
                signatures: Vec::new(),
                effects: None,
                events: None,
                checkpoint: None,
                timestamp: None,
                balance_changes: Vec::new(),
                input_objects: Vec::new(),
                output_objects: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ExecutedTransaction = ExecutedTransaction::const_default();
            &DEFAULT
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn transaction(&self) -> &Transaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Transaction::default_instance() as _)
        }
        pub fn transaction_opt(&self) -> Option<&Transaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn transaction_opt_mut(&mut self) -> Option<&mut Transaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut Transaction {
            self.transaction.get_or_insert_default()
        }
        pub fn with_transaction(mut self, field: Transaction) -> Self {
            self.transaction = Some(field.into());
            self
        }
        pub fn signatures(&self) -> &[UserSignature] {
            &self.signatures
        }
        pub fn with_signatures(mut self, field: Vec<UserSignature>) -> Self {
            self.signatures = field;
            self
        }
        pub fn effects(&self) -> &TransactionEffects {
            self.effects
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| TransactionEffects::default_instance() as _)
        }
        pub fn effects_opt(&self) -> Option<&TransactionEffects> {
            self.effects.as_ref().map(|field| field as _)
        }
        pub fn effects_opt_mut(&mut self) -> Option<&mut TransactionEffects> {
            self.effects.as_mut().map(|field| field as _)
        }
        pub fn effects_mut(&mut self) -> &mut TransactionEffects {
            self.effects.get_or_insert_default()
        }
        pub fn with_effects(mut self, field: TransactionEffects) -> Self {
            self.effects = Some(field.into());
            self
        }
        pub fn events(&self) -> &TransactionEvents {
            self.events
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| TransactionEvents::default_instance() as _)
        }
        pub fn events_opt(&self) -> Option<&TransactionEvents> {
            self.events.as_ref().map(|field| field as _)
        }
        pub fn events_opt_mut(&mut self) -> Option<&mut TransactionEvents> {
            self.events.as_mut().map(|field| field as _)
        }
        pub fn events_mut(&mut self) -> &mut TransactionEvents {
            self.events.get_or_insert_default()
        }
        pub fn with_events(mut self, field: TransactionEvents) -> Self {
            self.events = Some(field.into());
            self
        }
        pub fn with_checkpoint(mut self, field: u64) -> Self {
            self.checkpoint = Some(field.into());
            self
        }
        pub fn balance_changes(&self) -> &[BalanceChange] {
            &self.balance_changes
        }
        pub fn with_balance_changes(mut self, field: Vec<BalanceChange>) -> Self {
            self.balance_changes = field;
            self
        }
        pub fn input_objects(&self) -> &[Object] {
            &self.input_objects
        }
        pub fn with_input_objects(mut self, field: Vec<Object>) -> Self {
            self.input_objects = field;
            self
        }
        pub fn output_objects(&self) -> &[Object] {
            &self.output_objects
        }
        pub fn with_output_objects(mut self, field: Vec<Object>) -> Self {
            self.output_objects = field;
            self
        }
    }
    impl ExecutionStatus {
        pub const fn const_default() -> Self {
            Self { success: None, error: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ExecutionStatus = ExecutionStatus::const_default();
            &DEFAULT
        }
        pub fn with_success(mut self, field: bool) -> Self {
            self.success = Some(field.into());
            self
        }
        pub fn error(&self) -> &ExecutionError {
            self.error
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ExecutionError::default_instance() as _)
        }
        pub fn error_opt(&self) -> Option<&ExecutionError> {
            self.error.as_ref().map(|field| field as _)
        }
        pub fn error_opt_mut(&mut self) -> Option<&mut ExecutionError> {
            self.error.as_mut().map(|field| field as _)
        }
        pub fn error_mut(&mut self) -> &mut ExecutionError {
            self.error.get_or_insert_default()
        }
        pub fn with_error(mut self, field: ExecutionError) -> Self {
            self.error = Some(field.into());
            self
        }
    }
    impl ExecutionError {
        pub const fn const_default() -> Self {
            Self {
                description: None,
                command: None,
                kind: None,
                error_details: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ExecutionError = ExecutionError::const_default();
            &DEFAULT
        }
        pub fn with_description(mut self, field: String) -> Self {
            self.description = Some(field.into());
            self
        }
        pub fn with_command(mut self, field: u64) -> Self {
            self.command = Some(field.into());
            self
        }
        pub fn abort(&self) -> &MoveAbort {
            if let Some(execution_error::ErrorDetails::Abort(field)) = &self
                .error_details
            {
                field as _
            } else {
                MoveAbort::default_instance() as _
            }
        }
        pub fn abort_opt(&self) -> Option<&MoveAbort> {
            if let Some(execution_error::ErrorDetails::Abort(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn abort_opt_mut(&mut self) -> Option<&mut MoveAbort> {
            if let Some(execution_error::ErrorDetails::Abort(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn abort_mut(&mut self) -> &mut MoveAbort {
            if self.abort_opt_mut().is_none() {
                self.error_details = Some(
                    execution_error::ErrorDetails::Abort(MoveAbort::default()),
                );
            }
            self.abort_opt_mut().unwrap()
        }
        pub fn with_abort(mut self, field: MoveAbort) -> Self {
            self.error_details = Some(
                execution_error::ErrorDetails::Abort(field.into()),
            );
            self
        }
        pub fn size_error(&self) -> &SizeError {
            if let Some(execution_error::ErrorDetails::SizeError(field)) = &self
                .error_details
            {
                field as _
            } else {
                SizeError::default_instance() as _
            }
        }
        pub fn size_error_opt(&self) -> Option<&SizeError> {
            if let Some(execution_error::ErrorDetails::SizeError(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn size_error_opt_mut(&mut self) -> Option<&mut SizeError> {
            if let Some(execution_error::ErrorDetails::SizeError(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn size_error_mut(&mut self) -> &mut SizeError {
            if self.size_error_opt_mut().is_none() {
                self.error_details = Some(
                    execution_error::ErrorDetails::SizeError(SizeError::default()),
                );
            }
            self.size_error_opt_mut().unwrap()
        }
        pub fn with_size_error(mut self, field: SizeError) -> Self {
            self.error_details = Some(
                execution_error::ErrorDetails::SizeError(field.into()),
            );
            self
        }
        pub fn command_argument_error(&self) -> &CommandArgumentError {
            if let Some(execution_error::ErrorDetails::CommandArgumentError(field)) = &self
                .error_details
            {
                field as _
            } else {
                CommandArgumentError::default_instance() as _
            }
        }
        pub fn command_argument_error_opt(&self) -> Option<&CommandArgumentError> {
            if let Some(execution_error::ErrorDetails::CommandArgumentError(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn command_argument_error_opt_mut(
            &mut self,
        ) -> Option<&mut CommandArgumentError> {
            if let Some(execution_error::ErrorDetails::CommandArgumentError(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn command_argument_error_mut(&mut self) -> &mut CommandArgumentError {
            if self.command_argument_error_opt_mut().is_none() {
                self.error_details = Some(
                    execution_error::ErrorDetails::CommandArgumentError(
                        CommandArgumentError::default(),
                    ),
                );
            }
            self.command_argument_error_opt_mut().unwrap()
        }
        pub fn with_command_argument_error(
            mut self,
            field: CommandArgumentError,
        ) -> Self {
            self.error_details = Some(
                execution_error::ErrorDetails::CommandArgumentError(field.into()),
            );
            self
        }
        pub fn type_argument_error(&self) -> &TypeArgumentError {
            if let Some(execution_error::ErrorDetails::TypeArgumentError(field)) = &self
                .error_details
            {
                field as _
            } else {
                TypeArgumentError::default_instance() as _
            }
        }
        pub fn type_argument_error_opt(&self) -> Option<&TypeArgumentError> {
            if let Some(execution_error::ErrorDetails::TypeArgumentError(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn type_argument_error_opt_mut(&mut self) -> Option<&mut TypeArgumentError> {
            if let Some(execution_error::ErrorDetails::TypeArgumentError(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn type_argument_error_mut(&mut self) -> &mut TypeArgumentError {
            if self.type_argument_error_opt_mut().is_none() {
                self.error_details = Some(
                    execution_error::ErrorDetails::TypeArgumentError(
                        TypeArgumentError::default(),
                    ),
                );
            }
            self.type_argument_error_opt_mut().unwrap()
        }
        pub fn with_type_argument_error(mut self, field: TypeArgumentError) -> Self {
            self.error_details = Some(
                execution_error::ErrorDetails::TypeArgumentError(field.into()),
            );
            self
        }
        pub fn package_upgrade_error(&self) -> &PackageUpgradeError {
            if let Some(execution_error::ErrorDetails::PackageUpgradeError(field)) = &self
                .error_details
            {
                field as _
            } else {
                PackageUpgradeError::default_instance() as _
            }
        }
        pub fn package_upgrade_error_opt(&self) -> Option<&PackageUpgradeError> {
            if let Some(execution_error::ErrorDetails::PackageUpgradeError(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn package_upgrade_error_opt_mut(
            &mut self,
        ) -> Option<&mut PackageUpgradeError> {
            if let Some(execution_error::ErrorDetails::PackageUpgradeError(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn package_upgrade_error_mut(&mut self) -> &mut PackageUpgradeError {
            if self.package_upgrade_error_opt_mut().is_none() {
                self.error_details = Some(
                    execution_error::ErrorDetails::PackageUpgradeError(
                        PackageUpgradeError::default(),
                    ),
                );
            }
            self.package_upgrade_error_opt_mut().unwrap()
        }
        pub fn with_package_upgrade_error(mut self, field: PackageUpgradeError) -> Self {
            self.error_details = Some(
                execution_error::ErrorDetails::PackageUpgradeError(field.into()),
            );
            self
        }
        pub fn index_error(&self) -> &IndexError {
            if let Some(execution_error::ErrorDetails::IndexError(field)) = &self
                .error_details
            {
                field as _
            } else {
                IndexError::default_instance() as _
            }
        }
        pub fn index_error_opt(&self) -> Option<&IndexError> {
            if let Some(execution_error::ErrorDetails::IndexError(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn index_error_opt_mut(&mut self) -> Option<&mut IndexError> {
            if let Some(execution_error::ErrorDetails::IndexError(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn index_error_mut(&mut self) -> &mut IndexError {
            if self.index_error_opt_mut().is_none() {
                self.error_details = Some(
                    execution_error::ErrorDetails::IndexError(IndexError::default()),
                );
            }
            self.index_error_opt_mut().unwrap()
        }
        pub fn with_index_error(mut self, field: IndexError) -> Self {
            self.error_details = Some(
                execution_error::ErrorDetails::IndexError(field.into()),
            );
            self
        }
        pub fn object_id(&self) -> &str {
            if let Some(execution_error::ErrorDetails::ObjectId(field)) = &self
                .error_details
            {
                field as _
            } else {
                ""
            }
        }
        pub fn object_id_opt(&self) -> Option<&str> {
            if let Some(execution_error::ErrorDetails::ObjectId(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(execution_error::ErrorDetails::ObjectId(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn object_id_mut(&mut self) -> &mut String {
            if self.object_id_opt_mut().is_none() {
                self.error_details = Some(
                    execution_error::ErrorDetails::ObjectId(String::default()),
                );
            }
            self.object_id_opt_mut().unwrap()
        }
        pub fn with_object_id(mut self, field: String) -> Self {
            self.error_details = Some(
                execution_error::ErrorDetails::ObjectId(field.into()),
            );
            self
        }
        pub fn coin_deny_list_error(&self) -> &CoinDenyListError {
            if let Some(execution_error::ErrorDetails::CoinDenyListError(field)) = &self
                .error_details
            {
                field as _
            } else {
                CoinDenyListError::default_instance() as _
            }
        }
        pub fn coin_deny_list_error_opt(&self) -> Option<&CoinDenyListError> {
            if let Some(execution_error::ErrorDetails::CoinDenyListError(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn coin_deny_list_error_opt_mut(
            &mut self,
        ) -> Option<&mut CoinDenyListError> {
            if let Some(execution_error::ErrorDetails::CoinDenyListError(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn coin_deny_list_error_mut(&mut self) -> &mut CoinDenyListError {
            if self.coin_deny_list_error_opt_mut().is_none() {
                self.error_details = Some(
                    execution_error::ErrorDetails::CoinDenyListError(
                        CoinDenyListError::default(),
                    ),
                );
            }
            self.coin_deny_list_error_opt_mut().unwrap()
        }
        pub fn with_coin_deny_list_error(mut self, field: CoinDenyListError) -> Self {
            self.error_details = Some(
                execution_error::ErrorDetails::CoinDenyListError(field.into()),
            );
            self
        }
        pub fn congested_objects(&self) -> &CongestedObjects {
            if let Some(execution_error::ErrorDetails::CongestedObjects(field)) = &self
                .error_details
            {
                field as _
            } else {
                CongestedObjects::default_instance() as _
            }
        }
        pub fn congested_objects_opt(&self) -> Option<&CongestedObjects> {
            if let Some(execution_error::ErrorDetails::CongestedObjects(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn congested_objects_opt_mut(&mut self) -> Option<&mut CongestedObjects> {
            if let Some(execution_error::ErrorDetails::CongestedObjects(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn congested_objects_mut(&mut self) -> &mut CongestedObjects {
            if self.congested_objects_opt_mut().is_none() {
                self.error_details = Some(
                    execution_error::ErrorDetails::CongestedObjects(
                        CongestedObjects::default(),
                    ),
                );
            }
            self.congested_objects_opt_mut().unwrap()
        }
        pub fn with_congested_objects(mut self, field: CongestedObjects) -> Self {
            self.error_details = Some(
                execution_error::ErrorDetails::CongestedObjects(field.into()),
            );
            self
        }
    }
    impl MoveAbort {
        pub const fn const_default() -> Self {
            Self {
                abort_code: None,
                location: None,
                clever_error: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: MoveAbort = MoveAbort::const_default();
            &DEFAULT
        }
        pub fn with_abort_code(mut self, field: u64) -> Self {
            self.abort_code = Some(field.into());
            self
        }
        pub fn location(&self) -> &MoveLocation {
            self.location
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveLocation::default_instance() as _)
        }
        pub fn location_opt(&self) -> Option<&MoveLocation> {
            self.location.as_ref().map(|field| field as _)
        }
        pub fn location_opt_mut(&mut self) -> Option<&mut MoveLocation> {
            self.location.as_mut().map(|field| field as _)
        }
        pub fn location_mut(&mut self) -> &mut MoveLocation {
            self.location.get_or_insert_default()
        }
        pub fn with_location(mut self, field: MoveLocation) -> Self {
            self.location = Some(field.into());
            self
        }
        pub fn clever_error(&self) -> &CleverError {
            self.clever_error
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| CleverError::default_instance() as _)
        }
        pub fn clever_error_opt(&self) -> Option<&CleverError> {
            self.clever_error.as_ref().map(|field| field as _)
        }
        pub fn clever_error_opt_mut(&mut self) -> Option<&mut CleverError> {
            self.clever_error.as_mut().map(|field| field as _)
        }
        pub fn clever_error_mut(&mut self) -> &mut CleverError {
            self.clever_error.get_or_insert_default()
        }
        pub fn with_clever_error(mut self, field: CleverError) -> Self {
            self.clever_error = Some(field.into());
            self
        }
    }
    impl MoveLocation {
        pub const fn const_default() -> Self {
            Self {
                package: None,
                module: None,
                function: None,
                instruction: None,
                function_name: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: MoveLocation = MoveLocation::const_default();
            &DEFAULT
        }
        pub fn with_package(mut self, field: String) -> Self {
            self.package = Some(field.into());
            self
        }
        pub fn with_module(mut self, field: String) -> Self {
            self.module = Some(field.into());
            self
        }
        pub fn with_function(mut self, field: u32) -> Self {
            self.function = Some(field.into());
            self
        }
        pub fn with_instruction(mut self, field: u32) -> Self {
            self.instruction = Some(field.into());
            self
        }
        pub fn with_function_name(mut self, field: String) -> Self {
            self.function_name = Some(field.into());
            self
        }
    }
    impl CleverError {
        pub const fn const_default() -> Self {
            Self {
                error_code: None,
                line_number: None,
                constant_name: None,
                constant_type: None,
                value: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CleverError = CleverError::const_default();
            &DEFAULT
        }
        pub fn with_error_code(mut self, field: u64) -> Self {
            self.error_code = Some(field.into());
            self
        }
        pub fn with_line_number(mut self, field: u64) -> Self {
            self.line_number = Some(field.into());
            self
        }
        pub fn with_constant_name(mut self, field: String) -> Self {
            self.constant_name = Some(field.into());
            self
        }
        pub fn with_constant_type(mut self, field: String) -> Self {
            self.constant_type = Some(field.into());
            self
        }
        pub fn rendered(&self) -> &str {
            if let Some(clever_error::Value::Rendered(field)) = &self.value {
                field as _
            } else {
                ""
            }
        }
        pub fn rendered_opt(&self) -> Option<&str> {
            if let Some(clever_error::Value::Rendered(field)) = &self.value {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn rendered_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(clever_error::Value::Rendered(field)) = &mut self.value {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn rendered_mut(&mut self) -> &mut String {
            if self.rendered_opt_mut().is_none() {
                self.value = Some(clever_error::Value::Rendered(String::default()));
            }
            self.rendered_opt_mut().unwrap()
        }
        pub fn with_rendered(mut self, field: String) -> Self {
            self.value = Some(clever_error::Value::Rendered(field.into()));
            self
        }
        pub fn raw(&self) -> &[u8] {
            if let Some(clever_error::Value::Raw(field)) = &self.value {
                field as _
            } else {
                &[]
            }
        }
        pub fn raw_opt(&self) -> Option<&[u8]> {
            if let Some(clever_error::Value::Raw(field)) = &self.value {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn raw_opt_mut(&mut self) -> Option<&mut ::prost::bytes::Bytes> {
            if let Some(clever_error::Value::Raw(field)) = &mut self.value {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn raw_mut(&mut self) -> &mut ::prost::bytes::Bytes {
            if self.raw_opt_mut().is_none() {
                self.value = Some(
                    clever_error::Value::Raw(::prost::bytes::Bytes::default()),
                );
            }
            self.raw_opt_mut().unwrap()
        }
        pub fn with_raw(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.value = Some(clever_error::Value::Raw(field.into()));
            self
        }
    }
    impl SizeError {
        pub const fn const_default() -> Self {
            Self { size: None, max_size: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: SizeError = SizeError::const_default();
            &DEFAULT
        }
        pub fn with_size(mut self, field: u64) -> Self {
            self.size = Some(field.into());
            self
        }
        pub fn with_max_size(mut self, field: u64) -> Self {
            self.max_size = Some(field.into());
            self
        }
    }
    impl IndexError {
        pub const fn const_default() -> Self {
            Self {
                index: None,
                subresult: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: IndexError = IndexError::const_default();
            &DEFAULT
        }
        pub fn with_index(mut self, field: u32) -> Self {
            self.index = Some(field.into());
            self
        }
        pub fn with_subresult(mut self, field: u32) -> Self {
            self.subresult = Some(field.into());
            self
        }
    }
    impl CoinDenyListError {
        pub const fn const_default() -> Self {
            Self {
                address: None,
                coin_type: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CoinDenyListError = CoinDenyListError::const_default();
            &DEFAULT
        }
        pub fn with_address(mut self, field: String) -> Self {
            self.address = Some(field.into());
            self
        }
        pub fn with_coin_type(mut self, field: String) -> Self {
            self.coin_type = Some(field.into());
            self
        }
    }
    impl CongestedObjects {
        pub const fn const_default() -> Self {
            Self { objects: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CongestedObjects = CongestedObjects::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[String] {
            &self.objects
        }
        pub fn with_objects(mut self, field: Vec<String>) -> Self {
            self.objects = field;
            self
        }
    }
    impl CommandArgumentError {
        pub const fn const_default() -> Self {
            Self {
                argument: None,
                kind: None,
                index_error: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CommandArgumentError = CommandArgumentError::const_default();
            &DEFAULT
        }
        pub fn with_argument(mut self, field: u32) -> Self {
            self.argument = Some(field.into());
            self
        }
        pub fn index_error(&self) -> &IndexError {
            self.index_error
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| IndexError::default_instance() as _)
        }
        pub fn index_error_opt(&self) -> Option<&IndexError> {
            self.index_error.as_ref().map(|field| field as _)
        }
        pub fn index_error_opt_mut(&mut self) -> Option<&mut IndexError> {
            self.index_error.as_mut().map(|field| field as _)
        }
        pub fn index_error_mut(&mut self) -> &mut IndexError {
            self.index_error.get_or_insert_default()
        }
        pub fn with_index_error(mut self, field: IndexError) -> Self {
            self.index_error = Some(field.into());
            self
        }
    }
    impl PackageUpgradeError {
        pub const fn const_default() -> Self {
            Self {
                kind: None,
                package_id: None,
                digest: None,
                policy: None,
                ticket_id: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: PackageUpgradeError = PackageUpgradeError::const_default();
            &DEFAULT
        }
        pub fn with_package_id(mut self, field: String) -> Self {
            self.package_id = Some(field.into());
            self
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn with_policy(mut self, field: u32) -> Self {
            self.policy = Some(field.into());
            self
        }
        pub fn with_ticket_id(mut self, field: String) -> Self {
            self.ticket_id = Some(field.into());
            self
        }
    }
    impl TypeArgumentError {
        pub const fn const_default() -> Self {
            Self {
                type_argument: None,
                kind: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: TypeArgumentError = TypeArgumentError::const_default();
            &DEFAULT
        }
        pub fn with_type_argument(mut self, field: u32) -> Self {
            self.type_argument = Some(field.into());
            self
        }
    }
    impl GasCostSummary {
        pub const fn const_default() -> Self {
            Self {
                computation_cost: None,
                storage_cost: None,
                storage_rebate: None,
                non_refundable_storage_fee: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GasCostSummary = GasCostSummary::const_default();
            &DEFAULT
        }
        pub fn with_computation_cost(mut self, field: u64) -> Self {
            self.computation_cost = Some(field.into());
            self
        }
        pub fn with_storage_cost(mut self, field: u64) -> Self {
            self.storage_cost = Some(field.into());
            self
        }
        pub fn with_storage_rebate(mut self, field: u64) -> Self {
            self.storage_rebate = Some(field.into());
            self
        }
        pub fn with_non_refundable_storage_fee(mut self, field: u64) -> Self {
            self.non_refundable_storage_fee = Some(field.into());
            self
        }
    }
    impl Input {
        pub const fn const_default() -> Self {
            Self {
                kind: None,
                pure: None,
                object_id: None,
                version: None,
                digest: None,
                mutable: None,
                literal: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Input = Input::const_default();
            &DEFAULT
        }
        pub fn with_pure(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.pure = Some(field.into());
            self
        }
        pub fn with_object_id(mut self, field: String) -> Self {
            self.object_id = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: u64) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn with_mutable(mut self, field: bool) -> Self {
            self.mutable = Some(field.into());
            self
        }
    }
    impl GetServiceInfoRequest {
        pub const fn const_default() -> Self {
            Self {}
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetServiceInfoRequest = GetServiceInfoRequest::const_default();
            &DEFAULT
        }
    }
    impl GetServiceInfoResponse {
        pub const fn const_default() -> Self {
            Self {
                chain_id: None,
                chain: None,
                epoch: None,
                checkpoint_height: None,
                timestamp: None,
                lowest_available_checkpoint: None,
                lowest_available_checkpoint_objects: None,
                server: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetServiceInfoResponse = GetServiceInfoResponse::const_default();
            &DEFAULT
        }
        pub fn with_chain_id(mut self, field: String) -> Self {
            self.chain_id = Some(field.into());
            self
        }
        pub fn with_chain(mut self, field: String) -> Self {
            self.chain = Some(field.into());
            self
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
        pub fn with_checkpoint_height(mut self, field: u64) -> Self {
            self.checkpoint_height = Some(field.into());
            self
        }
        pub fn with_lowest_available_checkpoint(mut self, field: u64) -> Self {
            self.lowest_available_checkpoint = Some(field.into());
            self
        }
        pub fn with_lowest_available_checkpoint_objects(mut self, field: u64) -> Self {
            self.lowest_available_checkpoint_objects = Some(field.into());
            self
        }
        pub fn with_server(mut self, field: String) -> Self {
            self.server = Some(field.into());
            self
        }
    }
    impl GetObjectRequest {
        pub const fn const_default() -> Self {
            Self {
                object_id: None,
                version: None,
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetObjectRequest = GetObjectRequest::const_default();
            &DEFAULT
        }
        pub fn with_object_id(mut self, field: String) -> Self {
            self.object_id = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: u64) -> Self {
            self.version = Some(field.into());
            self
        }
    }
    impl GetObjectResponse {
        pub const fn const_default() -> Self {
            Self { object: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetObjectResponse = GetObjectResponse::const_default();
            &DEFAULT
        }
        pub fn object(&self) -> &Object {
            self.object
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Object::default_instance() as _)
        }
        pub fn object_opt(&self) -> Option<&Object> {
            self.object.as_ref().map(|field| field as _)
        }
        pub fn object_opt_mut(&mut self) -> Option<&mut Object> {
            self.object.as_mut().map(|field| field as _)
        }
        pub fn object_mut(&mut self) -> &mut Object {
            self.object.get_or_insert_default()
        }
        pub fn with_object(mut self, field: Object) -> Self {
            self.object = Some(field.into());
            self
        }
    }
    impl BatchGetObjectsRequest {
        pub const fn const_default() -> Self {
            Self {
                requests: Vec::new(),
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: BatchGetObjectsRequest = BatchGetObjectsRequest::const_default();
            &DEFAULT
        }
        pub fn requests(&self) -> &[GetObjectRequest] {
            &self.requests
        }
        pub fn with_requests(mut self, field: Vec<GetObjectRequest>) -> Self {
            self.requests = field;
            self
        }
    }
    impl BatchGetObjectsResponse {
        pub const fn const_default() -> Self {
            Self { objects: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: BatchGetObjectsResponse = BatchGetObjectsResponse::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[GetObjectResult] {
            &self.objects
        }
        pub fn with_objects(mut self, field: Vec<GetObjectResult>) -> Self {
            self.objects = field;
            self
        }
    }
    impl GetObjectResult {
        pub const fn const_default() -> Self {
            Self { result: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetObjectResult = GetObjectResult::const_default();
            &DEFAULT
        }
        pub fn object(&self) -> &Object {
            if let Some(get_object_result::Result::Object(field)) = &self.result {
                field as _
            } else {
                Object::default_instance() as _
            }
        }
        pub fn object_opt(&self) -> Option<&Object> {
            if let Some(get_object_result::Result::Object(field)) = &self.result {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn object_opt_mut(&mut self) -> Option<&mut Object> {
            if let Some(get_object_result::Result::Object(field)) = &mut self.result {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn object_mut(&mut self) -> &mut Object {
            if self.object_opt_mut().is_none() {
                self.result = Some(get_object_result::Result::Object(Object::default()));
            }
            self.object_opt_mut().unwrap()
        }
        pub fn with_object(mut self, field: Object) -> Self {
            self.result = Some(get_object_result::Result::Object(field.into()));
            self
        }
    }
    impl GetTransactionRequest {
        pub const fn const_default() -> Self {
            Self {
                digest: None,
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetTransactionRequest = GetTransactionRequest::const_default();
            &DEFAULT
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
    }
    impl GetTransactionResponse {
        pub const fn const_default() -> Self {
            Self { transaction: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetTransactionResponse = GetTransactionResponse::const_default();
            &DEFAULT
        }
        pub fn transaction(&self) -> &ExecutedTransaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ExecutedTransaction::default_instance() as _)
        }
        pub fn transaction_opt(&self) -> Option<&ExecutedTransaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn transaction_opt_mut(&mut self) -> Option<&mut ExecutedTransaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut ExecutedTransaction {
            self.transaction.get_or_insert_default()
        }
        pub fn with_transaction(mut self, field: ExecutedTransaction) -> Self {
            self.transaction = Some(field.into());
            self
        }
    }
    impl BatchGetTransactionsRequest {
        pub const fn const_default() -> Self {
            Self {
                digests: Vec::new(),
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: BatchGetTransactionsRequest = BatchGetTransactionsRequest::const_default();
            &DEFAULT
        }
        pub fn digests(&self) -> &[String] {
            &self.digests
        }
        pub fn with_digests(mut self, field: Vec<String>) -> Self {
            self.digests = field;
            self
        }
    }
    impl BatchGetTransactionsResponse {
        pub const fn const_default() -> Self {
            Self { transactions: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: BatchGetTransactionsResponse = BatchGetTransactionsResponse::const_default();
            &DEFAULT
        }
        pub fn transactions(&self) -> &[GetTransactionResult] {
            &self.transactions
        }
        pub fn with_transactions(mut self, field: Vec<GetTransactionResult>) -> Self {
            self.transactions = field;
            self
        }
    }
    impl GetTransactionResult {
        pub const fn const_default() -> Self {
            Self { result: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetTransactionResult = GetTransactionResult::const_default();
            &DEFAULT
        }
        pub fn transaction(&self) -> &ExecutedTransaction {
            if let Some(get_transaction_result::Result::Transaction(field)) = &self
                .result
            {
                field as _
            } else {
                ExecutedTransaction::default_instance() as _
            }
        }
        pub fn transaction_opt(&self) -> Option<&ExecutedTransaction> {
            if let Some(get_transaction_result::Result::Transaction(field)) = &self
                .result
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn transaction_opt_mut(&mut self) -> Option<&mut ExecutedTransaction> {
            if let Some(get_transaction_result::Result::Transaction(field)) = &mut self
                .result
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn transaction_mut(&mut self) -> &mut ExecutedTransaction {
            if self.transaction_opt_mut().is_none() {
                self.result = Some(
                    get_transaction_result::Result::Transaction(
                        ExecutedTransaction::default(),
                    ),
                );
            }
            self.transaction_opt_mut().unwrap()
        }
        pub fn with_transaction(mut self, field: ExecutedTransaction) -> Self {
            self.result = Some(
                get_transaction_result::Result::Transaction(field.into()),
            );
            self
        }
    }
    impl GetCheckpointRequest {
        pub const fn const_default() -> Self {
            Self {
                read_mask: None,
                checkpoint_id: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetCheckpointRequest = GetCheckpointRequest::const_default();
            &DEFAULT
        }
        pub fn sequence_number(&self) -> u64 {
            if let Some(get_checkpoint_request::CheckpointId::SequenceNumber(field)) = &self
                .checkpoint_id
            {
                *field
            } else {
                0u64
            }
        }
        pub fn sequence_number_opt(&self) -> Option<u64> {
            if let Some(get_checkpoint_request::CheckpointId::SequenceNumber(field)) = &self
                .checkpoint_id
            {
                Some(*field)
            } else {
                None
            }
        }
        pub fn sequence_number_opt_mut(&mut self) -> Option<&mut u64> {
            if let Some(get_checkpoint_request::CheckpointId::SequenceNumber(field)) = &mut self
                .checkpoint_id
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn sequence_number_mut(&mut self) -> &mut u64 {
            if self.sequence_number_opt_mut().is_none() {
                self.checkpoint_id = Some(
                    get_checkpoint_request::CheckpointId::SequenceNumber(u64::default()),
                );
            }
            self.sequence_number_opt_mut().unwrap()
        }
        pub fn with_sequence_number(mut self, field: u64) -> Self {
            self.checkpoint_id = Some(
                get_checkpoint_request::CheckpointId::SequenceNumber(field.into()),
            );
            self
        }
        pub fn digest(&self) -> &str {
            if let Some(get_checkpoint_request::CheckpointId::Digest(field)) = &self
                .checkpoint_id
            {
                field as _
            } else {
                ""
            }
        }
        pub fn digest_opt(&self) -> Option<&str> {
            if let Some(get_checkpoint_request::CheckpointId::Digest(field)) = &self
                .checkpoint_id
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(get_checkpoint_request::CheckpointId::Digest(field)) = &mut self
                .checkpoint_id
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn digest_mut(&mut self) -> &mut String {
            if self.digest_opt_mut().is_none() {
                self.checkpoint_id = Some(
                    get_checkpoint_request::CheckpointId::Digest(String::default()),
                );
            }
            self.digest_opt_mut().unwrap()
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.checkpoint_id = Some(
                get_checkpoint_request::CheckpointId::Digest(field.into()),
            );
            self
        }
    }
    impl GetCheckpointResponse {
        pub const fn const_default() -> Self {
            Self { checkpoint: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetCheckpointResponse = GetCheckpointResponse::const_default();
            &DEFAULT
        }
        pub fn checkpoint(&self) -> &Checkpoint {
            self.checkpoint
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Checkpoint::default_instance() as _)
        }
        pub fn checkpoint_opt(&self) -> Option<&Checkpoint> {
            self.checkpoint.as_ref().map(|field| field as _)
        }
        pub fn checkpoint_opt_mut(&mut self) -> Option<&mut Checkpoint> {
            self.checkpoint.as_mut().map(|field| field as _)
        }
        pub fn checkpoint_mut(&mut self) -> &mut Checkpoint {
            self.checkpoint.get_or_insert_default()
        }
        pub fn with_checkpoint(mut self, field: Checkpoint) -> Self {
            self.checkpoint = Some(field.into());
            self
        }
    }
    impl GetEpochRequest {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetEpochRequest = GetEpochRequest::const_default();
            &DEFAULT
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
    }
    impl GetEpochResponse {
        pub const fn const_default() -> Self {
            Self { epoch: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetEpochResponse = GetEpochResponse::const_default();
            &DEFAULT
        }
        pub fn epoch(&self) -> &Epoch {
            self.epoch
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Epoch::default_instance() as _)
        }
        pub fn epoch_opt(&self) -> Option<&Epoch> {
            self.epoch.as_ref().map(|field| field as _)
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut Epoch> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut Epoch {
            self.epoch.get_or_insert_default()
        }
        pub fn with_epoch(mut self, field: Epoch) -> Self {
            self.epoch = Some(field.into());
            self
        }
    }
    impl GetCoinInfoRequest {
        pub const fn const_default() -> Self {
            Self { coin_type: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetCoinInfoRequest = GetCoinInfoRequest::const_default();
            &DEFAULT
        }
        pub fn with_coin_type(mut self, field: String) -> Self {
            self.coin_type = Some(field.into());
            self
        }
    }
    impl GetCoinInfoResponse {
        pub const fn const_default() -> Self {
            Self {
                coin_type: None,
                metadata: None,
                treasury: None,
                regulated_metadata: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetCoinInfoResponse = GetCoinInfoResponse::const_default();
            &DEFAULT
        }
        pub fn with_coin_type(mut self, field: String) -> Self {
            self.coin_type = Some(field.into());
            self
        }
        pub fn metadata(&self) -> &CoinMetadata {
            self.metadata
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| CoinMetadata::default_instance() as _)
        }
        pub fn metadata_opt(&self) -> Option<&CoinMetadata> {
            self.metadata.as_ref().map(|field| field as _)
        }
        pub fn metadata_opt_mut(&mut self) -> Option<&mut CoinMetadata> {
            self.metadata.as_mut().map(|field| field as _)
        }
        pub fn metadata_mut(&mut self) -> &mut CoinMetadata {
            self.metadata.get_or_insert_default()
        }
        pub fn with_metadata(mut self, field: CoinMetadata) -> Self {
            self.metadata = Some(field.into());
            self
        }
        pub fn treasury(&self) -> &CoinTreasury {
            self.treasury
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| CoinTreasury::default_instance() as _)
        }
        pub fn treasury_opt(&self) -> Option<&CoinTreasury> {
            self.treasury.as_ref().map(|field| field as _)
        }
        pub fn treasury_opt_mut(&mut self) -> Option<&mut CoinTreasury> {
            self.treasury.as_mut().map(|field| field as _)
        }
        pub fn treasury_mut(&mut self) -> &mut CoinTreasury {
            self.treasury.get_or_insert_default()
        }
        pub fn with_treasury(mut self, field: CoinTreasury) -> Self {
            self.treasury = Some(field.into());
            self
        }
        pub fn regulated_metadata(&self) -> &RegulatedCoinMetadata {
            self.regulated_metadata
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| RegulatedCoinMetadata::default_instance() as _)
        }
        pub fn regulated_metadata_opt(&self) -> Option<&RegulatedCoinMetadata> {
            self.regulated_metadata.as_ref().map(|field| field as _)
        }
        pub fn regulated_metadata_opt_mut(
            &mut self,
        ) -> Option<&mut RegulatedCoinMetadata> {
            self.regulated_metadata.as_mut().map(|field| field as _)
        }
        pub fn regulated_metadata_mut(&mut self) -> &mut RegulatedCoinMetadata {
            self.regulated_metadata.get_or_insert_default()
        }
        pub fn with_regulated_metadata(mut self, field: RegulatedCoinMetadata) -> Self {
            self.regulated_metadata = Some(field.into());
            self
        }
    }
    impl CoinMetadata {
        pub const fn const_default() -> Self {
            Self {
                id: None,
                decimals: None,
                name: None,
                symbol: None,
                description: None,
                icon_url: None,
                metadata_cap_id: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CoinMetadata = CoinMetadata::const_default();
            &DEFAULT
        }
        pub fn with_id(mut self, field: String) -> Self {
            self.id = Some(field.into());
            self
        }
        pub fn with_decimals(mut self, field: u32) -> Self {
            self.decimals = Some(field.into());
            self
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
        pub fn with_symbol(mut self, field: String) -> Self {
            self.symbol = Some(field.into());
            self
        }
        pub fn with_description(mut self, field: String) -> Self {
            self.description = Some(field.into());
            self
        }
        pub fn with_icon_url(mut self, field: String) -> Self {
            self.icon_url = Some(field.into());
            self
        }
        pub fn with_metadata_cap_id(mut self, field: String) -> Self {
            self.metadata_cap_id = Some(field.into());
            self
        }
    }
    impl CoinTreasury {
        pub const fn const_default() -> Self {
            Self {
                id: None,
                total_supply: None,
                supply_state: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CoinTreasury = CoinTreasury::const_default();
            &DEFAULT
        }
        pub fn with_id(mut self, field: String) -> Self {
            self.id = Some(field.into());
            self
        }
        pub fn with_total_supply(mut self, field: u64) -> Self {
            self.total_supply = Some(field.into());
            self
        }
    }
    impl RegulatedCoinMetadata {
        pub const fn const_default() -> Self {
            Self {
                id: None,
                coin_metadata_object: None,
                deny_cap_object: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: RegulatedCoinMetadata = RegulatedCoinMetadata::const_default();
            &DEFAULT
        }
        pub fn with_id(mut self, field: String) -> Self {
            self.id = Some(field.into());
            self
        }
        pub fn with_coin_metadata_object(mut self, field: String) -> Self {
            self.coin_metadata_object = Some(field.into());
            self
        }
        pub fn with_deny_cap_object(mut self, field: String) -> Self {
            self.deny_cap_object = Some(field.into());
            self
        }
    }
    impl GetBalanceRequest {
        pub const fn const_default() -> Self {
            Self {
                owner: None,
                coin_type: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetBalanceRequest = GetBalanceRequest::const_default();
            &DEFAULT
        }
        pub fn with_owner(mut self, field: String) -> Self {
            self.owner = Some(field.into());
            self
        }
        pub fn with_coin_type(mut self, field: String) -> Self {
            self.coin_type = Some(field.into());
            self
        }
    }
    impl GetBalanceResponse {
        pub const fn const_default() -> Self {
            Self { balance: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetBalanceResponse = GetBalanceResponse::const_default();
            &DEFAULT
        }
        pub fn balance(&self) -> &Balance {
            self.balance
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Balance::default_instance() as _)
        }
        pub fn balance_opt(&self) -> Option<&Balance> {
            self.balance.as_ref().map(|field| field as _)
        }
        pub fn balance_opt_mut(&mut self) -> Option<&mut Balance> {
            self.balance.as_mut().map(|field| field as _)
        }
        pub fn balance_mut(&mut self) -> &mut Balance {
            self.balance.get_or_insert_default()
        }
        pub fn with_balance(mut self, field: Balance) -> Self {
            self.balance = Some(field.into());
            self
        }
    }
    impl ListBalancesRequest {
        pub const fn const_default() -> Self {
            Self {
                owner: None,
                page_size: None,
                page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ListBalancesRequest = ListBalancesRequest::const_default();
            &DEFAULT
        }
        pub fn with_owner(mut self, field: String) -> Self {
            self.owner = Some(field.into());
            self
        }
        pub fn with_page_size(mut self, field: u32) -> Self {
            self.page_size = Some(field.into());
            self
        }
        pub fn with_page_token(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.page_token = Some(field.into());
            self
        }
    }
    impl ListBalancesResponse {
        pub const fn const_default() -> Self {
            Self {
                balances: Vec::new(),
                next_page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ListBalancesResponse = ListBalancesResponse::const_default();
            &DEFAULT
        }
        pub fn balances(&self) -> &[Balance] {
            &self.balances
        }
        pub fn with_balances(mut self, field: Vec<Balance>) -> Self {
            self.balances = field;
            self
        }
        pub fn with_next_page_token(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.next_page_token = Some(field.into());
            self
        }
    }
    impl Balance {
        pub const fn const_default() -> Self {
            Self {
                coin_type: None,
                balance: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Balance = Balance::const_default();
            &DEFAULT
        }
        pub fn with_coin_type(mut self, field: String) -> Self {
            self.coin_type = Some(field.into());
            self
        }
        pub fn with_balance(mut self, field: u64) -> Self {
            self.balance = Some(field.into());
            self
        }
    }
    impl ListDynamicFieldsRequest {
        pub const fn const_default() -> Self {
            Self {
                parent: None,
                page_size: None,
                page_token: None,
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ListDynamicFieldsRequest = ListDynamicFieldsRequest::const_default();
            &DEFAULT
        }
        pub fn with_parent(mut self, field: String) -> Self {
            self.parent = Some(field.into());
            self
        }
        pub fn with_page_size(mut self, field: u32) -> Self {
            self.page_size = Some(field.into());
            self
        }
        pub fn with_page_token(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.page_token = Some(field.into());
            self
        }
    }
    impl ListDynamicFieldsResponse {
        pub const fn const_default() -> Self {
            Self {
                dynamic_fields: Vec::new(),
                next_page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ListDynamicFieldsResponse = ListDynamicFieldsResponse::const_default();
            &DEFAULT
        }
        pub fn dynamic_fields(&self) -> &[DynamicField] {
            &self.dynamic_fields
        }
        pub fn with_dynamic_fields(mut self, field: Vec<DynamicField>) -> Self {
            self.dynamic_fields = field;
            self
        }
        pub fn with_next_page_token(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.next_page_token = Some(field.into());
            self
        }
    }
    impl DynamicField {
        pub const fn const_default() -> Self {
            Self {
                kind: None,
                parent: None,
                field_id: None,
                name_type: None,
                name_value: None,
                value_type: None,
                dynamic_object_id: None,
                object: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: DynamicField = DynamicField::const_default();
            &DEFAULT
        }
        pub fn with_parent(mut self, field: String) -> Self {
            self.parent = Some(field.into());
            self
        }
        pub fn with_field_id(mut self, field: String) -> Self {
            self.field_id = Some(field.into());
            self
        }
        pub fn with_name_type(mut self, field: String) -> Self {
            self.name_type = Some(field.into());
            self
        }
        pub fn with_name_value(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.name_value = Some(field.into());
            self
        }
        pub fn with_value_type(mut self, field: String) -> Self {
            self.value_type = Some(field.into());
            self
        }
        pub fn with_dynamic_object_id(mut self, field: String) -> Self {
            self.dynamic_object_id = Some(field.into());
            self
        }
        pub fn object(&self) -> &Object {
            self.object
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Object::default_instance() as _)
        }
        pub fn object_opt(&self) -> Option<&Object> {
            self.object.as_ref().map(|field| field as _)
        }
        pub fn object_opt_mut(&mut self) -> Option<&mut Object> {
            self.object.as_mut().map(|field| field as _)
        }
        pub fn object_mut(&mut self) -> &mut Object {
            self.object.get_or_insert_default()
        }
        pub fn with_object(mut self, field: Object) -> Self {
            self.object = Some(field.into());
            self
        }
    }
    impl SimulateTransactionRequest {
        pub const fn const_default() -> Self {
            Self {
                transaction: None,
                read_mask: None,
                checks: None,
                do_gas_selection: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: SimulateTransactionRequest = SimulateTransactionRequest::const_default();
            &DEFAULT
        }
        pub fn transaction(&self) -> &Transaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Transaction::default_instance() as _)
        }
        pub fn transaction_opt(&self) -> Option<&Transaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn transaction_opt_mut(&mut self) -> Option<&mut Transaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut Transaction {
            self.transaction.get_or_insert_default()
        }
        pub fn with_transaction(mut self, field: Transaction) -> Self {
            self.transaction = Some(field.into());
            self
        }
        pub fn with_do_gas_selection(mut self, field: bool) -> Self {
            self.do_gas_selection = Some(field.into());
            self
        }
    }
    impl SimulateTransactionResponse {
        pub const fn const_default() -> Self {
            Self {
                transaction: None,
                outputs: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: SimulateTransactionResponse = SimulateTransactionResponse::const_default();
            &DEFAULT
        }
        pub fn transaction(&self) -> &ExecutedTransaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ExecutedTransaction::default_instance() as _)
        }
        pub fn transaction_opt(&self) -> Option<&ExecutedTransaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn transaction_opt_mut(&mut self) -> Option<&mut ExecutedTransaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut ExecutedTransaction {
            self.transaction.get_or_insert_default()
        }
        pub fn with_transaction(mut self, field: ExecutedTransaction) -> Self {
            self.transaction = Some(field.into());
            self
        }
        pub fn outputs(&self) -> &[CommandResult] {
            &self.outputs
        }
        pub fn with_outputs(mut self, field: Vec<CommandResult>) -> Self {
            self.outputs = field;
            self
        }
    }
    impl CommandResult {
        pub const fn const_default() -> Self {
            Self {
                return_values: Vec::new(),
                mutated_by_ref: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CommandResult = CommandResult::const_default();
            &DEFAULT
        }
        pub fn return_values(&self) -> &[CommandOutput] {
            &self.return_values
        }
        pub fn with_return_values(mut self, field: Vec<CommandOutput>) -> Self {
            self.return_values = field;
            self
        }
        pub fn mutated_by_ref(&self) -> &[CommandOutput] {
            &self.mutated_by_ref
        }
        pub fn with_mutated_by_ref(mut self, field: Vec<CommandOutput>) -> Self {
            self.mutated_by_ref = field;
            self
        }
    }
    impl CommandOutput {
        pub const fn const_default() -> Self {
            Self {
                argument: None,
                value: None,
                json: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CommandOutput = CommandOutput::const_default();
            &DEFAULT
        }
        pub fn argument(&self) -> &Argument {
            self.argument
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Argument::default_instance() as _)
        }
        pub fn argument_opt(&self) -> Option<&Argument> {
            self.argument.as_ref().map(|field| field as _)
        }
        pub fn argument_opt_mut(&mut self) -> Option<&mut Argument> {
            self.argument.as_mut().map(|field| field as _)
        }
        pub fn argument_mut(&mut self) -> &mut Argument {
            self.argument.get_or_insert_default()
        }
        pub fn with_argument(mut self, field: Argument) -> Self {
            self.argument = Some(field.into());
            self
        }
        pub fn value(&self) -> &Bcs {
            self.value
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Bcs::default_instance() as _)
        }
        pub fn value_opt(&self) -> Option<&Bcs> {
            self.value.as_ref().map(|field| field as _)
        }
        pub fn value_opt_mut(&mut self) -> Option<&mut Bcs> {
            self.value.as_mut().map(|field| field as _)
        }
        pub fn value_mut(&mut self) -> &mut Bcs {
            self.value.get_or_insert_default()
        }
        pub fn with_value(mut self, field: Bcs) -> Self {
            self.value = Some(field.into());
            self
        }
    }
    impl ListOwnedObjectsRequest {
        pub const fn const_default() -> Self {
            Self {
                owner: None,
                page_size: None,
                page_token: None,
                read_mask: None,
                object_type: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ListOwnedObjectsRequest = ListOwnedObjectsRequest::const_default();
            &DEFAULT
        }
        pub fn with_owner(mut self, field: String) -> Self {
            self.owner = Some(field.into());
            self
        }
        pub fn with_page_size(mut self, field: u32) -> Self {
            self.page_size = Some(field.into());
            self
        }
        pub fn with_page_token(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.page_token = Some(field.into());
            self
        }
        pub fn with_object_type(mut self, field: String) -> Self {
            self.object_type = Some(field.into());
            self
        }
    }
    impl ListOwnedObjectsResponse {
        pub const fn const_default() -> Self {
            Self {
                objects: Vec::new(),
                next_page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ListOwnedObjectsResponse = ListOwnedObjectsResponse::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[Object] {
            &self.objects
        }
        pub fn with_objects(mut self, field: Vec<Object>) -> Self {
            self.objects = field;
            self
        }
        pub fn with_next_page_token(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.next_page_token = Some(field.into());
            self
        }
    }
    impl Package {
        pub const fn const_default() -> Self {
            Self {
                storage_id: None,
                original_id: None,
                version: None,
                modules: Vec::new(),
                type_origins: Vec::new(),
                linkage: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Package = Package::const_default();
            &DEFAULT
        }
        pub fn with_storage_id(mut self, field: String) -> Self {
            self.storage_id = Some(field.into());
            self
        }
        pub fn with_original_id(mut self, field: String) -> Self {
            self.original_id = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: u64) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn modules(&self) -> &[Module] {
            &self.modules
        }
        pub fn with_modules(mut self, field: Vec<Module>) -> Self {
            self.modules = field;
            self
        }
        pub fn type_origins(&self) -> &[TypeOrigin] {
            &self.type_origins
        }
        pub fn with_type_origins(mut self, field: Vec<TypeOrigin>) -> Self {
            self.type_origins = field;
            self
        }
        pub fn linkage(&self) -> &[Linkage] {
            &self.linkage
        }
        pub fn with_linkage(mut self, field: Vec<Linkage>) -> Self {
            self.linkage = field;
            self
        }
    }
    impl Module {
        pub const fn const_default() -> Self {
            Self {
                name: None,
                contents: None,
                datatypes: Vec::new(),
                functions: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Module = Module::const_default();
            &DEFAULT
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
        pub fn with_contents(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.contents = Some(field.into());
            self
        }
        pub fn datatypes(&self) -> &[DatatypeDescriptor] {
            &self.datatypes
        }
        pub fn with_datatypes(mut self, field: Vec<DatatypeDescriptor>) -> Self {
            self.datatypes = field;
            self
        }
        pub fn functions(&self) -> &[FunctionDescriptor] {
            &self.functions
        }
        pub fn with_functions(mut self, field: Vec<FunctionDescriptor>) -> Self {
            self.functions = field;
            self
        }
    }
    impl DatatypeDescriptor {
        pub const fn const_default() -> Self {
            Self {
                type_name: None,
                defining_id: None,
                module: None,
                name: None,
                abilities: Vec::new(),
                type_parameters: Vec::new(),
                kind: None,
                fields: Vec::new(),
                variants: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: DatatypeDescriptor = DatatypeDescriptor::const_default();
            &DEFAULT
        }
        pub fn with_type_name(mut self, field: String) -> Self {
            self.type_name = Some(field.into());
            self
        }
        pub fn with_defining_id(mut self, field: String) -> Self {
            self.defining_id = Some(field.into());
            self
        }
        pub fn with_module(mut self, field: String) -> Self {
            self.module = Some(field.into());
            self
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
        pub fn type_parameters(&self) -> &[TypeParameter] {
            &self.type_parameters
        }
        pub fn with_type_parameters(mut self, field: Vec<TypeParameter>) -> Self {
            self.type_parameters = field;
            self
        }
        pub fn fields(&self) -> &[FieldDescriptor] {
            &self.fields
        }
        pub fn with_fields(mut self, field: Vec<FieldDescriptor>) -> Self {
            self.fields = field;
            self
        }
        pub fn variants(&self) -> &[VariantDescriptor] {
            &self.variants
        }
        pub fn with_variants(mut self, field: Vec<VariantDescriptor>) -> Self {
            self.variants = field;
            self
        }
    }
    impl TypeParameter {
        pub const fn const_default() -> Self {
            Self {
                constraints: Vec::new(),
                is_phantom: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: TypeParameter = TypeParameter::const_default();
            &DEFAULT
        }
        pub fn with_is_phantom(mut self, field: bool) -> Self {
            self.is_phantom = Some(field.into());
            self
        }
    }
    impl FieldDescriptor {
        pub const fn const_default() -> Self {
            Self {
                name: None,
                position: None,
                r#type: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: FieldDescriptor = FieldDescriptor::const_default();
            &DEFAULT
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
        pub fn with_position(mut self, field: u32) -> Self {
            self.position = Some(field.into());
            self
        }
        pub fn r#type(&self) -> &OpenSignatureBody {
            self.r#type
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| OpenSignatureBody::default_instance() as _)
        }
        pub fn type_opt(&self) -> Option<&OpenSignatureBody> {
            self.r#type.as_ref().map(|field| field as _)
        }
        pub fn type_opt_mut(&mut self) -> Option<&mut OpenSignatureBody> {
            self.r#type.as_mut().map(|field| field as _)
        }
        pub fn type_mut(&mut self) -> &mut OpenSignatureBody {
            self.r#type.get_or_insert_default()
        }
        pub fn with_type(mut self, field: OpenSignatureBody) -> Self {
            self.r#type = Some(field.into());
            self
        }
    }
    impl VariantDescriptor {
        pub const fn const_default() -> Self {
            Self {
                name: None,
                position: None,
                fields: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: VariantDescriptor = VariantDescriptor::const_default();
            &DEFAULT
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
        pub fn with_position(mut self, field: u32) -> Self {
            self.position = Some(field.into());
            self
        }
        pub fn fields(&self) -> &[FieldDescriptor] {
            &self.fields
        }
        pub fn with_fields(mut self, field: Vec<FieldDescriptor>) -> Self {
            self.fields = field;
            self
        }
    }
    impl OpenSignatureBody {
        pub const fn const_default() -> Self {
            Self {
                r#type: None,
                type_name: None,
                type_parameter_instantiation: Vec::new(),
                type_parameter: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: OpenSignatureBody = OpenSignatureBody::const_default();
            &DEFAULT
        }
        pub fn with_type_name(mut self, field: String) -> Self {
            self.type_name = Some(field.into());
            self
        }
        pub fn type_parameter_instantiation(&self) -> &[OpenSignatureBody] {
            &self.type_parameter_instantiation
        }
        pub fn with_type_parameter_instantiation(
            mut self,
            field: Vec<OpenSignatureBody>,
        ) -> Self {
            self.type_parameter_instantiation = field;
            self
        }
        pub fn with_type_parameter(mut self, field: u32) -> Self {
            self.type_parameter = Some(field.into());
            self
        }
    }
    impl FunctionDescriptor {
        pub const fn const_default() -> Self {
            Self {
                name: None,
                visibility: None,
                is_entry: None,
                type_parameters: Vec::new(),
                parameters: Vec::new(),
                returns: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: FunctionDescriptor = FunctionDescriptor::const_default();
            &DEFAULT
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
        pub fn with_is_entry(mut self, field: bool) -> Self {
            self.is_entry = Some(field.into());
            self
        }
        pub fn type_parameters(&self) -> &[TypeParameter] {
            &self.type_parameters
        }
        pub fn with_type_parameters(mut self, field: Vec<TypeParameter>) -> Self {
            self.type_parameters = field;
            self
        }
        pub fn parameters(&self) -> &[OpenSignature] {
            &self.parameters
        }
        pub fn with_parameters(mut self, field: Vec<OpenSignature>) -> Self {
            self.parameters = field;
            self
        }
        pub fn returns(&self) -> &[OpenSignature] {
            &self.returns
        }
        pub fn with_returns(mut self, field: Vec<OpenSignature>) -> Self {
            self.returns = field;
            self
        }
    }
    impl OpenSignature {
        pub const fn const_default() -> Self {
            Self {
                reference: None,
                body: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: OpenSignature = OpenSignature::const_default();
            &DEFAULT
        }
        pub fn body(&self) -> &OpenSignatureBody {
            self.body
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| OpenSignatureBody::default_instance() as _)
        }
        pub fn body_opt(&self) -> Option<&OpenSignatureBody> {
            self.body.as_ref().map(|field| field as _)
        }
        pub fn body_opt_mut(&mut self) -> Option<&mut OpenSignatureBody> {
            self.body.as_mut().map(|field| field as _)
        }
        pub fn body_mut(&mut self) -> &mut OpenSignatureBody {
            self.body.get_or_insert_default()
        }
        pub fn with_body(mut self, field: OpenSignatureBody) -> Self {
            self.body = Some(field.into());
            self
        }
    }
    impl TypeOrigin {
        pub const fn const_default() -> Self {
            Self {
                module_name: None,
                datatype_name: None,
                package_id: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: TypeOrigin = TypeOrigin::const_default();
            &DEFAULT
        }
        pub fn with_module_name(mut self, field: String) -> Self {
            self.module_name = Some(field.into());
            self
        }
        pub fn with_datatype_name(mut self, field: String) -> Self {
            self.datatype_name = Some(field.into());
            self
        }
        pub fn with_package_id(mut self, field: String) -> Self {
            self.package_id = Some(field.into());
            self
        }
    }
    impl Linkage {
        pub const fn const_default() -> Self {
            Self {
                original_id: None,
                upgraded_id: None,
                upgraded_version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Linkage = Linkage::const_default();
            &DEFAULT
        }
        pub fn with_original_id(mut self, field: String) -> Self {
            self.original_id = Some(field.into());
            self
        }
        pub fn with_upgraded_id(mut self, field: String) -> Self {
            self.upgraded_id = Some(field.into());
            self
        }
        pub fn with_upgraded_version(mut self, field: u64) -> Self {
            self.upgraded_version = Some(field.into());
            self
        }
    }
    impl GetPackageRequest {
        pub const fn const_default() -> Self {
            Self { package_id: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetPackageRequest = GetPackageRequest::const_default();
            &DEFAULT
        }
        pub fn with_package_id(mut self, field: String) -> Self {
            self.package_id = Some(field.into());
            self
        }
    }
    impl GetPackageResponse {
        pub const fn const_default() -> Self {
            Self { package: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetPackageResponse = GetPackageResponse::const_default();
            &DEFAULT
        }
        pub fn package(&self) -> &Package {
            self.package
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Package::default_instance() as _)
        }
        pub fn package_opt(&self) -> Option<&Package> {
            self.package.as_ref().map(|field| field as _)
        }
        pub fn package_opt_mut(&mut self) -> Option<&mut Package> {
            self.package.as_mut().map(|field| field as _)
        }
        pub fn package_mut(&mut self) -> &mut Package {
            self.package.get_or_insert_default()
        }
        pub fn with_package(mut self, field: Package) -> Self {
            self.package = Some(field.into());
            self
        }
    }
    impl GetDatatypeRequest {
        pub const fn const_default() -> Self {
            Self {
                package_id: None,
                module_name: None,
                name: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetDatatypeRequest = GetDatatypeRequest::const_default();
            &DEFAULT
        }
        pub fn with_package_id(mut self, field: String) -> Self {
            self.package_id = Some(field.into());
            self
        }
        pub fn with_module_name(mut self, field: String) -> Self {
            self.module_name = Some(field.into());
            self
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
    }
    impl GetDatatypeResponse {
        pub const fn const_default() -> Self {
            Self { datatype: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetDatatypeResponse = GetDatatypeResponse::const_default();
            &DEFAULT
        }
        pub fn datatype(&self) -> &DatatypeDescriptor {
            self.datatype
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| DatatypeDescriptor::default_instance() as _)
        }
        pub fn datatype_opt(&self) -> Option<&DatatypeDescriptor> {
            self.datatype.as_ref().map(|field| field as _)
        }
        pub fn datatype_opt_mut(&mut self) -> Option<&mut DatatypeDescriptor> {
            self.datatype.as_mut().map(|field| field as _)
        }
        pub fn datatype_mut(&mut self) -> &mut DatatypeDescriptor {
            self.datatype.get_or_insert_default()
        }
        pub fn with_datatype(mut self, field: DatatypeDescriptor) -> Self {
            self.datatype = Some(field.into());
            self
        }
    }
    impl GetFunctionRequest {
        pub const fn const_default() -> Self {
            Self {
                package_id: None,
                module_name: None,
                name: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetFunctionRequest = GetFunctionRequest::const_default();
            &DEFAULT
        }
        pub fn with_package_id(mut self, field: String) -> Self {
            self.package_id = Some(field.into());
            self
        }
        pub fn with_module_name(mut self, field: String) -> Self {
            self.module_name = Some(field.into());
            self
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
    }
    impl GetFunctionResponse {
        pub const fn const_default() -> Self {
            Self { function: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GetFunctionResponse = GetFunctionResponse::const_default();
            &DEFAULT
        }
        pub fn function(&self) -> &FunctionDescriptor {
            self.function
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| FunctionDescriptor::default_instance() as _)
        }
        pub fn function_opt(&self) -> Option<&FunctionDescriptor> {
            self.function.as_ref().map(|field| field as _)
        }
        pub fn function_opt_mut(&mut self) -> Option<&mut FunctionDescriptor> {
            self.function.as_mut().map(|field| field as _)
        }
        pub fn function_mut(&mut self) -> &mut FunctionDescriptor {
            self.function.get_or_insert_default()
        }
        pub fn with_function(mut self, field: FunctionDescriptor) -> Self {
            self.function = Some(field.into());
            self
        }
    }
    impl ListPackageVersionsRequest {
        pub const fn const_default() -> Self {
            Self {
                package_id: None,
                page_size: None,
                page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ListPackageVersionsRequest = ListPackageVersionsRequest::const_default();
            &DEFAULT
        }
        pub fn with_package_id(mut self, field: String) -> Self {
            self.package_id = Some(field.into());
            self
        }
        pub fn with_page_size(mut self, field: u32) -> Self {
            self.page_size = Some(field.into());
            self
        }
        pub fn with_page_token(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.page_token = Some(field.into());
            self
        }
    }
    impl ListPackageVersionsResponse {
        pub const fn const_default() -> Self {
            Self {
                versions: Vec::new(),
                next_page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ListPackageVersionsResponse = ListPackageVersionsResponse::const_default();
            &DEFAULT
        }
        pub fn versions(&self) -> &[PackageVersion] {
            &self.versions
        }
        pub fn with_versions(mut self, field: Vec<PackageVersion>) -> Self {
            self.versions = field;
            self
        }
        pub fn with_next_page_token(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.next_page_token = Some(field.into());
            self
        }
    }
    impl PackageVersion {
        pub const fn const_default() -> Self {
            Self {
                package_id: None,
                version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: PackageVersion = PackageVersion::const_default();
            &DEFAULT
        }
        pub fn with_package_id(mut self, field: String) -> Self {
            self.package_id = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: u64) -> Self {
            self.version = Some(field.into());
            self
        }
    }
    impl LookupNameRequest {
        pub const fn const_default() -> Self {
            Self { name: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: LookupNameRequest = LookupNameRequest::const_default();
            &DEFAULT
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
    }
    impl LookupNameResponse {
        pub const fn const_default() -> Self {
            Self { record: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: LookupNameResponse = LookupNameResponse::const_default();
            &DEFAULT
        }
        pub fn record(&self) -> &NameRecord {
            self.record
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| NameRecord::default_instance() as _)
        }
        pub fn record_opt(&self) -> Option<&NameRecord> {
            self.record.as_ref().map(|field| field as _)
        }
        pub fn record_opt_mut(&mut self) -> Option<&mut NameRecord> {
            self.record.as_mut().map(|field| field as _)
        }
        pub fn record_mut(&mut self) -> &mut NameRecord {
            self.record.get_or_insert_default()
        }
        pub fn with_record(mut self, field: NameRecord) -> Self {
            self.record = Some(field.into());
            self
        }
    }
    impl ReverseLookupNameRequest {
        pub const fn const_default() -> Self {
            Self { address: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ReverseLookupNameRequest = ReverseLookupNameRequest::const_default();
            &DEFAULT
        }
        pub fn with_address(mut self, field: String) -> Self {
            self.address = Some(field.into());
            self
        }
    }
    impl ReverseLookupNameResponse {
        pub const fn const_default() -> Self {
            Self { record: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ReverseLookupNameResponse = ReverseLookupNameResponse::const_default();
            &DEFAULT
        }
        pub fn record(&self) -> &NameRecord {
            self.record
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| NameRecord::default_instance() as _)
        }
        pub fn record_opt(&self) -> Option<&NameRecord> {
            self.record.as_ref().map(|field| field as _)
        }
        pub fn record_opt_mut(&mut self) -> Option<&mut NameRecord> {
            self.record.as_mut().map(|field| field as _)
        }
        pub fn record_mut(&mut self) -> &mut NameRecord {
            self.record.get_or_insert_default()
        }
        pub fn with_record(mut self, field: NameRecord) -> Self {
            self.record = Some(field.into());
            self
        }
    }
    impl NameRecord {
        pub const fn const_default() -> Self {
            Self {
                id: None,
                name: None,
                registration_nft_id: None,
                expiration_timestamp: None,
                target_address: None,
                data: std::collections::BTreeMap::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: NameRecord = NameRecord::const_default();
            &DEFAULT
        }
        pub fn with_id(mut self, field: String) -> Self {
            self.id = Some(field.into());
            self
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
        pub fn with_registration_nft_id(mut self, field: String) -> Self {
            self.registration_nft_id = Some(field.into());
            self
        }
        pub fn with_target_address(mut self, field: String) -> Self {
            self.target_address = Some(field.into());
            self
        }
    }
    impl Object {
        pub const fn const_default() -> Self {
            Self {
                bcs: None,
                object_id: None,
                version: None,
                digest: None,
                owner: None,
                object_type: None,
                has_public_transfer: None,
                contents: None,
                package: None,
                previous_transaction: None,
                storage_rebate: None,
                json: None,
                balance: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Object = Object::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Bcs::default_instance() as _)
        }
        pub fn bcs_opt(&self) -> Option<&Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn with_bcs(mut self, field: Bcs) -> Self {
            self.bcs = Some(field.into());
            self
        }
        pub fn with_object_id(mut self, field: String) -> Self {
            self.object_id = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: u64) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn owner(&self) -> &Owner {
            self.owner
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Owner::default_instance() as _)
        }
        pub fn owner_opt(&self) -> Option<&Owner> {
            self.owner.as_ref().map(|field| field as _)
        }
        pub fn owner_opt_mut(&mut self) -> Option<&mut Owner> {
            self.owner.as_mut().map(|field| field as _)
        }
        pub fn owner_mut(&mut self) -> &mut Owner {
            self.owner.get_or_insert_default()
        }
        pub fn with_owner(mut self, field: Owner) -> Self {
            self.owner = Some(field.into());
            self
        }
        pub fn with_object_type(mut self, field: String) -> Self {
            self.object_type = Some(field.into());
            self
        }
        pub fn with_has_public_transfer(mut self, field: bool) -> Self {
            self.has_public_transfer = Some(field.into());
            self
        }
        pub fn contents(&self) -> &Bcs {
            self.contents
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Bcs::default_instance() as _)
        }
        pub fn contents_opt(&self) -> Option<&Bcs> {
            self.contents.as_ref().map(|field| field as _)
        }
        pub fn contents_opt_mut(&mut self) -> Option<&mut Bcs> {
            self.contents.as_mut().map(|field| field as _)
        }
        pub fn contents_mut(&mut self) -> &mut Bcs {
            self.contents.get_or_insert_default()
        }
        pub fn with_contents(mut self, field: Bcs) -> Self {
            self.contents = Some(field.into());
            self
        }
        pub fn package(&self) -> &Package {
            self.package
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Package::default_instance() as _)
        }
        pub fn package_opt(&self) -> Option<&Package> {
            self.package.as_ref().map(|field| field as _)
        }
        pub fn package_opt_mut(&mut self) -> Option<&mut Package> {
            self.package.as_mut().map(|field| field as _)
        }
        pub fn package_mut(&mut self) -> &mut Package {
            self.package.get_or_insert_default()
        }
        pub fn with_package(mut self, field: Package) -> Self {
            self.package = Some(field.into());
            self
        }
        pub fn with_previous_transaction(mut self, field: String) -> Self {
            self.previous_transaction = Some(field.into());
            self
        }
        pub fn with_storage_rebate(mut self, field: u64) -> Self {
            self.storage_rebate = Some(field.into());
            self
        }
        pub fn with_balance(mut self, field: u64) -> Self {
            self.balance = Some(field.into());
            self
        }
    }
    impl ObjectReference {
        pub const fn const_default() -> Self {
            Self {
                object_id: None,
                version: None,
                digest: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ObjectReference = ObjectReference::const_default();
            &DEFAULT
        }
        pub fn with_object_id(mut self, field: String) -> Self {
            self.object_id = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: u64) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
    }
    impl Owner {
        pub const fn const_default() -> Self {
            Self {
                kind: None,
                address: None,
                version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Owner = Owner::const_default();
            &DEFAULT
        }
        pub fn with_address(mut self, field: String) -> Self {
            self.address = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: u64) -> Self {
            self.version = Some(field.into());
            self
        }
    }
    impl ProtocolConfig {
        pub const fn const_default() -> Self {
            Self {
                protocol_version: None,
                feature_flags: std::collections::BTreeMap::new(),
                attributes: std::collections::BTreeMap::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ProtocolConfig = ProtocolConfig::const_default();
            &DEFAULT
        }
        pub fn with_protocol_version(mut self, field: u64) -> Self {
            self.protocol_version = Some(field.into());
            self
        }
    }
    impl UserSignature {
        pub const fn const_default() -> Self {
            Self {
                bcs: None,
                scheme: None,
                signature: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: UserSignature = UserSignature::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Bcs::default_instance() as _)
        }
        pub fn bcs_opt(&self) -> Option<&Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn with_bcs(mut self, field: Bcs) -> Self {
            self.bcs = Some(field.into());
            self
        }
        pub fn simple(&self) -> &SimpleSignature {
            if let Some(user_signature::Signature::Simple(field)) = &self.signature {
                field as _
            } else {
                SimpleSignature::default_instance() as _
            }
        }
        pub fn simple_opt(&self) -> Option<&SimpleSignature> {
            if let Some(user_signature::Signature::Simple(field)) = &self.signature {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn simple_opt_mut(&mut self) -> Option<&mut SimpleSignature> {
            if let Some(user_signature::Signature::Simple(field)) = &mut self.signature {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn simple_mut(&mut self) -> &mut SimpleSignature {
            if self.simple_opt_mut().is_none() {
                self.signature = Some(
                    user_signature::Signature::Simple(SimpleSignature::default()),
                );
            }
            self.simple_opt_mut().unwrap()
        }
        pub fn with_simple(mut self, field: SimpleSignature) -> Self {
            self.signature = Some(user_signature::Signature::Simple(field.into()));
            self
        }
        pub fn multisig(&self) -> &MultisigAggregatedSignature {
            if let Some(user_signature::Signature::Multisig(field)) = &self.signature {
                field as _
            } else {
                MultisigAggregatedSignature::default_instance() as _
            }
        }
        pub fn multisig_opt(&self) -> Option<&MultisigAggregatedSignature> {
            if let Some(user_signature::Signature::Multisig(field)) = &self.signature {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn multisig_opt_mut(&mut self) -> Option<&mut MultisigAggregatedSignature> {
            if let Some(user_signature::Signature::Multisig(field)) = &mut self.signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn multisig_mut(&mut self) -> &mut MultisigAggregatedSignature {
            if self.multisig_opt_mut().is_none() {
                self.signature = Some(
                    user_signature::Signature::Multisig(
                        MultisigAggregatedSignature::default(),
                    ),
                );
            }
            self.multisig_opt_mut().unwrap()
        }
        pub fn with_multisig(mut self, field: MultisigAggregatedSignature) -> Self {
            self.signature = Some(user_signature::Signature::Multisig(field.into()));
            self
        }
        pub fn zklogin(&self) -> &ZkLoginAuthenticator {
            if let Some(user_signature::Signature::Zklogin(field)) = &self.signature {
                field as _
            } else {
                ZkLoginAuthenticator::default_instance() as _
            }
        }
        pub fn zklogin_opt(&self) -> Option<&ZkLoginAuthenticator> {
            if let Some(user_signature::Signature::Zklogin(field)) = &self.signature {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn zklogin_opt_mut(&mut self) -> Option<&mut ZkLoginAuthenticator> {
            if let Some(user_signature::Signature::Zklogin(field)) = &mut self.signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn zklogin_mut(&mut self) -> &mut ZkLoginAuthenticator {
            if self.zklogin_opt_mut().is_none() {
                self.signature = Some(
                    user_signature::Signature::Zklogin(ZkLoginAuthenticator::default()),
                );
            }
            self.zklogin_opt_mut().unwrap()
        }
        pub fn with_zklogin(mut self, field: ZkLoginAuthenticator) -> Self {
            self.signature = Some(user_signature::Signature::Zklogin(field.into()));
            self
        }
        pub fn passkey(&self) -> &PasskeyAuthenticator {
            if let Some(user_signature::Signature::Passkey(field)) = &self.signature {
                field as _
            } else {
                PasskeyAuthenticator::default_instance() as _
            }
        }
        pub fn passkey_opt(&self) -> Option<&PasskeyAuthenticator> {
            if let Some(user_signature::Signature::Passkey(field)) = &self.signature {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn passkey_opt_mut(&mut self) -> Option<&mut PasskeyAuthenticator> {
            if let Some(user_signature::Signature::Passkey(field)) = &mut self.signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn passkey_mut(&mut self) -> &mut PasskeyAuthenticator {
            if self.passkey_opt_mut().is_none() {
                self.signature = Some(
                    user_signature::Signature::Passkey(PasskeyAuthenticator::default()),
                );
            }
            self.passkey_opt_mut().unwrap()
        }
        pub fn with_passkey(mut self, field: PasskeyAuthenticator) -> Self {
            self.signature = Some(user_signature::Signature::Passkey(field.into()));
            self
        }
    }
    impl SimpleSignature {
        pub const fn const_default() -> Self {
            Self {
                scheme: None,
                signature: None,
                public_key: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: SimpleSignature = SimpleSignature::const_default();
            &DEFAULT
        }
        pub fn with_signature(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.signature = Some(field.into());
            self
        }
        pub fn with_public_key(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.public_key = Some(field.into());
            self
        }
    }
    impl ZkLoginPublicIdentifier {
        pub const fn const_default() -> Self {
            Self {
                iss: None,
                address_seed: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ZkLoginPublicIdentifier = ZkLoginPublicIdentifier::const_default();
            &DEFAULT
        }
        pub fn with_iss(mut self, field: String) -> Self {
            self.iss = Some(field.into());
            self
        }
        pub fn with_address_seed(mut self, field: String) -> Self {
            self.address_seed = Some(field.into());
            self
        }
    }
    impl MultisigMemberPublicKey {
        pub const fn const_default() -> Self {
            Self {
                scheme: None,
                public_key: None,
                zklogin: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: MultisigMemberPublicKey = MultisigMemberPublicKey::const_default();
            &DEFAULT
        }
        pub fn with_public_key(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.public_key = Some(field.into());
            self
        }
        pub fn zklogin(&self) -> &ZkLoginPublicIdentifier {
            self.zklogin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ZkLoginPublicIdentifier::default_instance() as _)
        }
        pub fn zklogin_opt(&self) -> Option<&ZkLoginPublicIdentifier> {
            self.zklogin.as_ref().map(|field| field as _)
        }
        pub fn zklogin_opt_mut(&mut self) -> Option<&mut ZkLoginPublicIdentifier> {
            self.zklogin.as_mut().map(|field| field as _)
        }
        pub fn zklogin_mut(&mut self) -> &mut ZkLoginPublicIdentifier {
            self.zklogin.get_or_insert_default()
        }
        pub fn with_zklogin(mut self, field: ZkLoginPublicIdentifier) -> Self {
            self.zklogin = Some(field.into());
            self
        }
    }
    impl MultisigMember {
        pub const fn const_default() -> Self {
            Self {
                public_key: None,
                weight: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: MultisigMember = MultisigMember::const_default();
            &DEFAULT
        }
        pub fn public_key(&self) -> &MultisigMemberPublicKey {
            self.public_key
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MultisigMemberPublicKey::default_instance() as _)
        }
        pub fn public_key_opt(&self) -> Option<&MultisigMemberPublicKey> {
            self.public_key.as_ref().map(|field| field as _)
        }
        pub fn public_key_opt_mut(&mut self) -> Option<&mut MultisigMemberPublicKey> {
            self.public_key.as_mut().map(|field| field as _)
        }
        pub fn public_key_mut(&mut self) -> &mut MultisigMemberPublicKey {
            self.public_key.get_or_insert_default()
        }
        pub fn with_public_key(mut self, field: MultisigMemberPublicKey) -> Self {
            self.public_key = Some(field.into());
            self
        }
        pub fn with_weight(mut self, field: u32) -> Self {
            self.weight = Some(field.into());
            self
        }
    }
    impl MultisigCommittee {
        pub const fn const_default() -> Self {
            Self {
                members: Vec::new(),
                threshold: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: MultisigCommittee = MultisigCommittee::const_default();
            &DEFAULT
        }
        pub fn members(&self) -> &[MultisigMember] {
            &self.members
        }
        pub fn with_members(mut self, field: Vec<MultisigMember>) -> Self {
            self.members = field;
            self
        }
        pub fn with_threshold(mut self, field: u32) -> Self {
            self.threshold = Some(field.into());
            self
        }
    }
    impl MultisigAggregatedSignature {
        pub const fn const_default() -> Self {
            Self {
                signatures: Vec::new(),
                bitmap: None,
                legacy_bitmap: Vec::new(),
                committee: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: MultisigAggregatedSignature = MultisigAggregatedSignature::const_default();
            &DEFAULT
        }
        pub fn signatures(&self) -> &[MultisigMemberSignature] {
            &self.signatures
        }
        pub fn with_signatures(mut self, field: Vec<MultisigMemberSignature>) -> Self {
            self.signatures = field;
            self
        }
        pub fn with_bitmap(mut self, field: u32) -> Self {
            self.bitmap = Some(field.into());
            self
        }
        pub fn legacy_bitmap(&self) -> &[u32] {
            &self.legacy_bitmap
        }
        pub fn with_legacy_bitmap(mut self, field: Vec<u32>) -> Self {
            self.legacy_bitmap = field;
            self
        }
        pub fn committee(&self) -> &MultisigCommittee {
            self.committee
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MultisigCommittee::default_instance() as _)
        }
        pub fn committee_opt(&self) -> Option<&MultisigCommittee> {
            self.committee.as_ref().map(|field| field as _)
        }
        pub fn committee_opt_mut(&mut self) -> Option<&mut MultisigCommittee> {
            self.committee.as_mut().map(|field| field as _)
        }
        pub fn committee_mut(&mut self) -> &mut MultisigCommittee {
            self.committee.get_or_insert_default()
        }
        pub fn with_committee(mut self, field: MultisigCommittee) -> Self {
            self.committee = Some(field.into());
            self
        }
    }
    impl MultisigMemberSignature {
        pub const fn const_default() -> Self {
            Self {
                scheme: None,
                signature: None,
                zklogin: None,
                passkey: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: MultisigMemberSignature = MultisigMemberSignature::const_default();
            &DEFAULT
        }
        pub fn with_signature(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.signature = Some(field.into());
            self
        }
        pub fn zklogin(&self) -> &ZkLoginAuthenticator {
            self.zklogin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ZkLoginAuthenticator::default_instance() as _)
        }
        pub fn zklogin_opt(&self) -> Option<&ZkLoginAuthenticator> {
            self.zklogin.as_ref().map(|field| field as _)
        }
        pub fn zklogin_opt_mut(&mut self) -> Option<&mut ZkLoginAuthenticator> {
            self.zklogin.as_mut().map(|field| field as _)
        }
        pub fn zklogin_mut(&mut self) -> &mut ZkLoginAuthenticator {
            self.zklogin.get_or_insert_default()
        }
        pub fn with_zklogin(mut self, field: ZkLoginAuthenticator) -> Self {
            self.zklogin = Some(field.into());
            self
        }
        pub fn passkey(&self) -> &PasskeyAuthenticator {
            self.passkey
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| PasskeyAuthenticator::default_instance() as _)
        }
        pub fn passkey_opt(&self) -> Option<&PasskeyAuthenticator> {
            self.passkey.as_ref().map(|field| field as _)
        }
        pub fn passkey_opt_mut(&mut self) -> Option<&mut PasskeyAuthenticator> {
            self.passkey.as_mut().map(|field| field as _)
        }
        pub fn passkey_mut(&mut self) -> &mut PasskeyAuthenticator {
            self.passkey.get_or_insert_default()
        }
        pub fn with_passkey(mut self, field: PasskeyAuthenticator) -> Self {
            self.passkey = Some(field.into());
            self
        }
    }
    impl ZkLoginAuthenticator {
        pub const fn const_default() -> Self {
            Self {
                inputs: None,
                max_epoch: None,
                signature: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ZkLoginAuthenticator = ZkLoginAuthenticator::const_default();
            &DEFAULT
        }
        pub fn inputs(&self) -> &ZkLoginInputs {
            self.inputs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ZkLoginInputs::default_instance() as _)
        }
        pub fn inputs_opt(&self) -> Option<&ZkLoginInputs> {
            self.inputs.as_ref().map(|field| field as _)
        }
        pub fn inputs_opt_mut(&mut self) -> Option<&mut ZkLoginInputs> {
            self.inputs.as_mut().map(|field| field as _)
        }
        pub fn inputs_mut(&mut self) -> &mut ZkLoginInputs {
            self.inputs.get_or_insert_default()
        }
        pub fn with_inputs(mut self, field: ZkLoginInputs) -> Self {
            self.inputs = Some(field.into());
            self
        }
        pub fn with_max_epoch(mut self, field: u64) -> Self {
            self.max_epoch = Some(field.into());
            self
        }
        pub fn signature(&self) -> &SimpleSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| SimpleSignature::default_instance() as _)
        }
        pub fn signature_opt(&self) -> Option<&SimpleSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        pub fn signature_opt_mut(&mut self) -> Option<&mut SimpleSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        pub fn signature_mut(&mut self) -> &mut SimpleSignature {
            self.signature.get_or_insert_default()
        }
        pub fn with_signature(mut self, field: SimpleSignature) -> Self {
            self.signature = Some(field.into());
            self
        }
    }
    impl ZkLoginInputs {
        pub const fn const_default() -> Self {
            Self {
                proof_points: None,
                iss_base64_details: None,
                header_base64: None,
                address_seed: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ZkLoginInputs = ZkLoginInputs::const_default();
            &DEFAULT
        }
        pub fn proof_points(&self) -> &ZkLoginProof {
            self.proof_points
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ZkLoginProof::default_instance() as _)
        }
        pub fn proof_points_opt(&self) -> Option<&ZkLoginProof> {
            self.proof_points.as_ref().map(|field| field as _)
        }
        pub fn proof_points_opt_mut(&mut self) -> Option<&mut ZkLoginProof> {
            self.proof_points.as_mut().map(|field| field as _)
        }
        pub fn proof_points_mut(&mut self) -> &mut ZkLoginProof {
            self.proof_points.get_or_insert_default()
        }
        pub fn with_proof_points(mut self, field: ZkLoginProof) -> Self {
            self.proof_points = Some(field.into());
            self
        }
        pub fn iss_base64_details(&self) -> &ZkLoginClaim {
            self.iss_base64_details
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ZkLoginClaim::default_instance() as _)
        }
        pub fn iss_base64_details_opt(&self) -> Option<&ZkLoginClaim> {
            self.iss_base64_details.as_ref().map(|field| field as _)
        }
        pub fn iss_base64_details_opt_mut(&mut self) -> Option<&mut ZkLoginClaim> {
            self.iss_base64_details.as_mut().map(|field| field as _)
        }
        pub fn iss_base64_details_mut(&mut self) -> &mut ZkLoginClaim {
            self.iss_base64_details.get_or_insert_default()
        }
        pub fn with_iss_base64_details(mut self, field: ZkLoginClaim) -> Self {
            self.iss_base64_details = Some(field.into());
            self
        }
        pub fn with_header_base64(mut self, field: String) -> Self {
            self.header_base64 = Some(field.into());
            self
        }
        pub fn with_address_seed(mut self, field: String) -> Self {
            self.address_seed = Some(field.into());
            self
        }
    }
    impl ZkLoginProof {
        pub const fn const_default() -> Self {
            Self { a: None, b: None, c: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ZkLoginProof = ZkLoginProof::const_default();
            &DEFAULT
        }
        pub fn a(&self) -> &CircomG1 {
            self.a
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| CircomG1::default_instance() as _)
        }
        pub fn a_opt(&self) -> Option<&CircomG1> {
            self.a.as_ref().map(|field| field as _)
        }
        pub fn a_opt_mut(&mut self) -> Option<&mut CircomG1> {
            self.a.as_mut().map(|field| field as _)
        }
        pub fn a_mut(&mut self) -> &mut CircomG1 {
            self.a.get_or_insert_default()
        }
        pub fn with_a(mut self, field: CircomG1) -> Self {
            self.a = Some(field.into());
            self
        }
        pub fn b(&self) -> &CircomG2 {
            self.b
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| CircomG2::default_instance() as _)
        }
        pub fn b_opt(&self) -> Option<&CircomG2> {
            self.b.as_ref().map(|field| field as _)
        }
        pub fn b_opt_mut(&mut self) -> Option<&mut CircomG2> {
            self.b.as_mut().map(|field| field as _)
        }
        pub fn b_mut(&mut self) -> &mut CircomG2 {
            self.b.get_or_insert_default()
        }
        pub fn with_b(mut self, field: CircomG2) -> Self {
            self.b = Some(field.into());
            self
        }
        pub fn c(&self) -> &CircomG1 {
            self.c
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| CircomG1::default_instance() as _)
        }
        pub fn c_opt(&self) -> Option<&CircomG1> {
            self.c.as_ref().map(|field| field as _)
        }
        pub fn c_opt_mut(&mut self) -> Option<&mut CircomG1> {
            self.c.as_mut().map(|field| field as _)
        }
        pub fn c_mut(&mut self) -> &mut CircomG1 {
            self.c.get_or_insert_default()
        }
        pub fn with_c(mut self, field: CircomG1) -> Self {
            self.c = Some(field.into());
            self
        }
    }
    impl ZkLoginClaim {
        pub const fn const_default() -> Self {
            Self {
                value: None,
                index_mod_4: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ZkLoginClaim = ZkLoginClaim::const_default();
            &DEFAULT
        }
        pub fn with_value(mut self, field: String) -> Self {
            self.value = Some(field.into());
            self
        }
        pub fn with_index_mod_4(mut self, field: u32) -> Self {
            self.index_mod_4 = Some(field.into());
            self
        }
    }
    impl CircomG1 {
        pub const fn const_default() -> Self {
            Self {
                e0: None,
                e1: None,
                e2: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CircomG1 = CircomG1::const_default();
            &DEFAULT
        }
        pub fn with_e0(mut self, field: String) -> Self {
            self.e0 = Some(field.into());
            self
        }
        pub fn with_e1(mut self, field: String) -> Self {
            self.e1 = Some(field.into());
            self
        }
        pub fn with_e2(mut self, field: String) -> Self {
            self.e2 = Some(field.into());
            self
        }
    }
    impl CircomG2 {
        pub const fn const_default() -> Self {
            Self {
                e00: None,
                e01: None,
                e10: None,
                e11: None,
                e20: None,
                e21: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CircomG2 = CircomG2::const_default();
            &DEFAULT
        }
        pub fn with_e00(mut self, field: String) -> Self {
            self.e00 = Some(field.into());
            self
        }
        pub fn with_e01(mut self, field: String) -> Self {
            self.e01 = Some(field.into());
            self
        }
        pub fn with_e10(mut self, field: String) -> Self {
            self.e10 = Some(field.into());
            self
        }
        pub fn with_e11(mut self, field: String) -> Self {
            self.e11 = Some(field.into());
            self
        }
        pub fn with_e20(mut self, field: String) -> Self {
            self.e20 = Some(field.into());
            self
        }
        pub fn with_e21(mut self, field: String) -> Self {
            self.e21 = Some(field.into());
            self
        }
    }
    impl PasskeyAuthenticator {
        pub const fn const_default() -> Self {
            Self {
                authenticator_data: None,
                client_data_json: None,
                signature: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: PasskeyAuthenticator = PasskeyAuthenticator::const_default();
            &DEFAULT
        }
        pub fn with_authenticator_data(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.authenticator_data = Some(field.into());
            self
        }
        pub fn with_client_data_json(mut self, field: String) -> Self {
            self.client_data_json = Some(field.into());
            self
        }
        pub fn signature(&self) -> &SimpleSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| SimpleSignature::default_instance() as _)
        }
        pub fn signature_opt(&self) -> Option<&SimpleSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        pub fn signature_opt_mut(&mut self) -> Option<&mut SimpleSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        pub fn signature_mut(&mut self) -> &mut SimpleSignature {
            self.signature.get_or_insert_default()
        }
        pub fn with_signature(mut self, field: SimpleSignature) -> Self {
            self.signature = Some(field.into());
            self
        }
    }
    impl ValidatorCommittee {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                members: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ValidatorCommittee = ValidatorCommittee::const_default();
            &DEFAULT
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
        pub fn members(&self) -> &[ValidatorCommitteeMember] {
            &self.members
        }
        pub fn with_members(mut self, field: Vec<ValidatorCommitteeMember>) -> Self {
            self.members = field;
            self
        }
    }
    impl ValidatorCommitteeMember {
        pub const fn const_default() -> Self {
            Self {
                public_key: None,
                weight: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ValidatorCommitteeMember = ValidatorCommitteeMember::const_default();
            &DEFAULT
        }
        pub fn with_public_key(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.public_key = Some(field.into());
            self
        }
        pub fn with_weight(mut self, field: u64) -> Self {
            self.weight = Some(field.into());
            self
        }
    }
    impl ValidatorAggregatedSignature {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                signature: None,
                bitmap: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ValidatorAggregatedSignature = ValidatorAggregatedSignature::const_default();
            &DEFAULT
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
        pub fn with_signature(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.signature = Some(field.into());
            self
        }
        pub fn bitmap(&self) -> &[u32] {
            &self.bitmap
        }
        pub fn with_bitmap(mut self, field: Vec<u32>) -> Self {
            self.bitmap = field;
            self
        }
    }
    impl VerifySignatureRequest {
        pub const fn const_default() -> Self {
            Self {
                message: None,
                signature: None,
                address: None,
                jwks: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: VerifySignatureRequest = VerifySignatureRequest::const_default();
            &DEFAULT
        }
        pub fn message(&self) -> &Bcs {
            self.message
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Bcs::default_instance() as _)
        }
        pub fn message_opt(&self) -> Option<&Bcs> {
            self.message.as_ref().map(|field| field as _)
        }
        pub fn message_opt_mut(&mut self) -> Option<&mut Bcs> {
            self.message.as_mut().map(|field| field as _)
        }
        pub fn message_mut(&mut self) -> &mut Bcs {
            self.message.get_or_insert_default()
        }
        pub fn with_message(mut self, field: Bcs) -> Self {
            self.message = Some(field.into());
            self
        }
        pub fn signature(&self) -> &UserSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| UserSignature::default_instance() as _)
        }
        pub fn signature_opt(&self) -> Option<&UserSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        pub fn signature_opt_mut(&mut self) -> Option<&mut UserSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        pub fn signature_mut(&mut self) -> &mut UserSignature {
            self.signature.get_or_insert_default()
        }
        pub fn with_signature(mut self, field: UserSignature) -> Self {
            self.signature = Some(field.into());
            self
        }
        pub fn with_address(mut self, field: String) -> Self {
            self.address = Some(field.into());
            self
        }
        pub fn jwks(&self) -> &[ActiveJwk] {
            &self.jwks
        }
        pub fn with_jwks(mut self, field: Vec<ActiveJwk>) -> Self {
            self.jwks = field;
            self
        }
    }
    impl VerifySignatureResponse {
        pub const fn const_default() -> Self {
            Self {
                is_valid: None,
                reason: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: VerifySignatureResponse = VerifySignatureResponse::const_default();
            &DEFAULT
        }
        pub fn with_is_valid(mut self, field: bool) -> Self {
            self.is_valid = Some(field.into());
            self
        }
        pub fn with_reason(mut self, field: String) -> Self {
            self.reason = Some(field.into());
            self
        }
    }
    impl SubscribeCheckpointsRequest {
        pub const fn const_default() -> Self {
            Self { read_mask: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: SubscribeCheckpointsRequest = SubscribeCheckpointsRequest::const_default();
            &DEFAULT
        }
    }
    impl SubscribeCheckpointsResponse {
        pub const fn const_default() -> Self {
            Self {
                cursor: None,
                checkpoint: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: SubscribeCheckpointsResponse = SubscribeCheckpointsResponse::const_default();
            &DEFAULT
        }
        pub fn with_cursor(mut self, field: u64) -> Self {
            self.cursor = Some(field.into());
            self
        }
        pub fn checkpoint(&self) -> &Checkpoint {
            self.checkpoint
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Checkpoint::default_instance() as _)
        }
        pub fn checkpoint_opt(&self) -> Option<&Checkpoint> {
            self.checkpoint.as_ref().map(|field| field as _)
        }
        pub fn checkpoint_opt_mut(&mut self) -> Option<&mut Checkpoint> {
            self.checkpoint.as_mut().map(|field| field as _)
        }
        pub fn checkpoint_mut(&mut self) -> &mut Checkpoint {
            self.checkpoint.get_or_insert_default()
        }
        pub fn with_checkpoint(mut self, field: Checkpoint) -> Self {
            self.checkpoint = Some(field.into());
            self
        }
    }
    impl SystemState {
        pub const fn const_default() -> Self {
            Self {
                version: None,
                epoch: None,
                protocol_version: None,
                validators: None,
                storage_fund: None,
                parameters: None,
                reference_gas_price: None,
                validator_report_records: Vec::new(),
                stake_subsidy: None,
                safe_mode: None,
                safe_mode_storage_rewards: None,
                safe_mode_computation_rewards: None,
                safe_mode_storage_rebates: None,
                safe_mode_non_refundable_storage_fee: None,
                epoch_start_timestamp_ms: None,
                extra_fields: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: SystemState = SystemState::const_default();
            &DEFAULT
        }
        pub fn with_version(mut self, field: u64) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
        pub fn with_protocol_version(mut self, field: u64) -> Self {
            self.protocol_version = Some(field.into());
            self
        }
        pub fn validators(&self) -> &ValidatorSet {
            self.validators
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ValidatorSet::default_instance() as _)
        }
        pub fn validators_opt(&self) -> Option<&ValidatorSet> {
            self.validators.as_ref().map(|field| field as _)
        }
        pub fn validators_opt_mut(&mut self) -> Option<&mut ValidatorSet> {
            self.validators.as_mut().map(|field| field as _)
        }
        pub fn validators_mut(&mut self) -> &mut ValidatorSet {
            self.validators.get_or_insert_default()
        }
        pub fn with_validators(mut self, field: ValidatorSet) -> Self {
            self.validators = Some(field.into());
            self
        }
        pub fn storage_fund(&self) -> &StorageFund {
            self.storage_fund
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| StorageFund::default_instance() as _)
        }
        pub fn storage_fund_opt(&self) -> Option<&StorageFund> {
            self.storage_fund.as_ref().map(|field| field as _)
        }
        pub fn storage_fund_opt_mut(&mut self) -> Option<&mut StorageFund> {
            self.storage_fund.as_mut().map(|field| field as _)
        }
        pub fn storage_fund_mut(&mut self) -> &mut StorageFund {
            self.storage_fund.get_or_insert_default()
        }
        pub fn with_storage_fund(mut self, field: StorageFund) -> Self {
            self.storage_fund = Some(field.into());
            self
        }
        pub fn parameters(&self) -> &SystemParameters {
            self.parameters
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| SystemParameters::default_instance() as _)
        }
        pub fn parameters_opt(&self) -> Option<&SystemParameters> {
            self.parameters.as_ref().map(|field| field as _)
        }
        pub fn parameters_opt_mut(&mut self) -> Option<&mut SystemParameters> {
            self.parameters.as_mut().map(|field| field as _)
        }
        pub fn parameters_mut(&mut self) -> &mut SystemParameters {
            self.parameters.get_or_insert_default()
        }
        pub fn with_parameters(mut self, field: SystemParameters) -> Self {
            self.parameters = Some(field.into());
            self
        }
        pub fn with_reference_gas_price(mut self, field: u64) -> Self {
            self.reference_gas_price = Some(field.into());
            self
        }
        pub fn validator_report_records(&self) -> &[ValidatorReportRecord] {
            &self.validator_report_records
        }
        pub fn with_validator_report_records(
            mut self,
            field: Vec<ValidatorReportRecord>,
        ) -> Self {
            self.validator_report_records = field;
            self
        }
        pub fn stake_subsidy(&self) -> &StakeSubsidy {
            self.stake_subsidy
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| StakeSubsidy::default_instance() as _)
        }
        pub fn stake_subsidy_opt(&self) -> Option<&StakeSubsidy> {
            self.stake_subsidy.as_ref().map(|field| field as _)
        }
        pub fn stake_subsidy_opt_mut(&mut self) -> Option<&mut StakeSubsidy> {
            self.stake_subsidy.as_mut().map(|field| field as _)
        }
        pub fn stake_subsidy_mut(&mut self) -> &mut StakeSubsidy {
            self.stake_subsidy.get_or_insert_default()
        }
        pub fn with_stake_subsidy(mut self, field: StakeSubsidy) -> Self {
            self.stake_subsidy = Some(field.into());
            self
        }
        pub fn with_safe_mode(mut self, field: bool) -> Self {
            self.safe_mode = Some(field.into());
            self
        }
        pub fn with_safe_mode_storage_rewards(mut self, field: u64) -> Self {
            self.safe_mode_storage_rewards = Some(field.into());
            self
        }
        pub fn with_safe_mode_computation_rewards(mut self, field: u64) -> Self {
            self.safe_mode_computation_rewards = Some(field.into());
            self
        }
        pub fn with_safe_mode_storage_rebates(mut self, field: u64) -> Self {
            self.safe_mode_storage_rebates = Some(field.into());
            self
        }
        pub fn with_safe_mode_non_refundable_storage_fee(mut self, field: u64) -> Self {
            self.safe_mode_non_refundable_storage_fee = Some(field.into());
            self
        }
        pub fn with_epoch_start_timestamp_ms(mut self, field: u64) -> Self {
            self.epoch_start_timestamp_ms = Some(field.into());
            self
        }
        pub fn extra_fields(&self) -> &MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt(&self) -> Option<&MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn with_extra_fields(mut self, field: MoveTable) -> Self {
            self.extra_fields = Some(field.into());
            self
        }
    }
    impl ValidatorReportRecord {
        pub const fn const_default() -> Self {
            Self {
                reported: None,
                reporters: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ValidatorReportRecord = ValidatorReportRecord::const_default();
            &DEFAULT
        }
        pub fn with_reported(mut self, field: String) -> Self {
            self.reported = Some(field.into());
            self
        }
        pub fn reporters(&self) -> &[String] {
            &self.reporters
        }
        pub fn with_reporters(mut self, field: Vec<String>) -> Self {
            self.reporters = field;
            self
        }
    }
    impl SystemParameters {
        pub const fn const_default() -> Self {
            Self {
                epoch_duration_ms: None,
                stake_subsidy_start_epoch: None,
                min_validator_count: None,
                max_validator_count: None,
                min_validator_joining_stake: None,
                validator_low_stake_threshold: None,
                validator_very_low_stake_threshold: None,
                validator_low_stake_grace_period: None,
                extra_fields: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: SystemParameters = SystemParameters::const_default();
            &DEFAULT
        }
        pub fn with_epoch_duration_ms(mut self, field: u64) -> Self {
            self.epoch_duration_ms = Some(field.into());
            self
        }
        pub fn with_stake_subsidy_start_epoch(mut self, field: u64) -> Self {
            self.stake_subsidy_start_epoch = Some(field.into());
            self
        }
        pub fn with_min_validator_count(mut self, field: u64) -> Self {
            self.min_validator_count = Some(field.into());
            self
        }
        pub fn with_max_validator_count(mut self, field: u64) -> Self {
            self.max_validator_count = Some(field.into());
            self
        }
        pub fn with_min_validator_joining_stake(mut self, field: u64) -> Self {
            self.min_validator_joining_stake = Some(field.into());
            self
        }
        pub fn with_validator_low_stake_threshold(mut self, field: u64) -> Self {
            self.validator_low_stake_threshold = Some(field.into());
            self
        }
        pub fn with_validator_very_low_stake_threshold(mut self, field: u64) -> Self {
            self.validator_very_low_stake_threshold = Some(field.into());
            self
        }
        pub fn with_validator_low_stake_grace_period(mut self, field: u64) -> Self {
            self.validator_low_stake_grace_period = Some(field.into());
            self
        }
        pub fn extra_fields(&self) -> &MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt(&self) -> Option<&MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn with_extra_fields(mut self, field: MoveTable) -> Self {
            self.extra_fields = Some(field.into());
            self
        }
    }
    impl MoveTable {
        pub const fn const_default() -> Self {
            Self { id: None, size: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: MoveTable = MoveTable::const_default();
            &DEFAULT
        }
        pub fn with_id(mut self, field: String) -> Self {
            self.id = Some(field.into());
            self
        }
        pub fn with_size(mut self, field: u64) -> Self {
            self.size = Some(field.into());
            self
        }
    }
    impl StakeSubsidy {
        pub const fn const_default() -> Self {
            Self {
                balance: None,
                distribution_counter: None,
                current_distribution_amount: None,
                stake_subsidy_period_length: None,
                stake_subsidy_decrease_rate: None,
                extra_fields: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: StakeSubsidy = StakeSubsidy::const_default();
            &DEFAULT
        }
        pub fn with_balance(mut self, field: u64) -> Self {
            self.balance = Some(field.into());
            self
        }
        pub fn with_distribution_counter(mut self, field: u64) -> Self {
            self.distribution_counter = Some(field.into());
            self
        }
        pub fn with_current_distribution_amount(mut self, field: u64) -> Self {
            self.current_distribution_amount = Some(field.into());
            self
        }
        pub fn with_stake_subsidy_period_length(mut self, field: u64) -> Self {
            self.stake_subsidy_period_length = Some(field.into());
            self
        }
        pub fn with_stake_subsidy_decrease_rate(mut self, field: u32) -> Self {
            self.stake_subsidy_decrease_rate = Some(field.into());
            self
        }
        pub fn extra_fields(&self) -> &MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt(&self) -> Option<&MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn with_extra_fields(mut self, field: MoveTable) -> Self {
            self.extra_fields = Some(field.into());
            self
        }
    }
    impl StorageFund {
        pub const fn const_default() -> Self {
            Self {
                total_object_storage_rebates: None,
                non_refundable_balance: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: StorageFund = StorageFund::const_default();
            &DEFAULT
        }
        pub fn with_total_object_storage_rebates(mut self, field: u64) -> Self {
            self.total_object_storage_rebates = Some(field.into());
            self
        }
        pub fn with_non_refundable_balance(mut self, field: u64) -> Self {
            self.non_refundable_balance = Some(field.into());
            self
        }
    }
    impl ValidatorSet {
        pub const fn const_default() -> Self {
            Self {
                total_stake: None,
                active_validators: Vec::new(),
                pending_active_validators: None,
                pending_removals: Vec::new(),
                staking_pool_mappings: None,
                inactive_validators: None,
                validator_candidates: None,
                at_risk_validators: std::collections::BTreeMap::new(),
                extra_fields: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ValidatorSet = ValidatorSet::const_default();
            &DEFAULT
        }
        pub fn with_total_stake(mut self, field: u64) -> Self {
            self.total_stake = Some(field.into());
            self
        }
        pub fn active_validators(&self) -> &[Validator] {
            &self.active_validators
        }
        pub fn with_active_validators(mut self, field: Vec<Validator>) -> Self {
            self.active_validators = field;
            self
        }
        pub fn pending_active_validators(&self) -> &MoveTable {
            self.pending_active_validators
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn pending_active_validators_opt(&self) -> Option<&MoveTable> {
            self.pending_active_validators.as_ref().map(|field| field as _)
        }
        pub fn pending_active_validators_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.pending_active_validators.as_mut().map(|field| field as _)
        }
        pub fn pending_active_validators_mut(&mut self) -> &mut MoveTable {
            self.pending_active_validators.get_or_insert_default()
        }
        pub fn with_pending_active_validators(mut self, field: MoveTable) -> Self {
            self.pending_active_validators = Some(field.into());
            self
        }
        pub fn pending_removals(&self) -> &[u64] {
            &self.pending_removals
        }
        pub fn with_pending_removals(mut self, field: Vec<u64>) -> Self {
            self.pending_removals = field;
            self
        }
        pub fn staking_pool_mappings(&self) -> &MoveTable {
            self.staking_pool_mappings
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn staking_pool_mappings_opt(&self) -> Option<&MoveTable> {
            self.staking_pool_mappings.as_ref().map(|field| field as _)
        }
        pub fn staking_pool_mappings_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.staking_pool_mappings.as_mut().map(|field| field as _)
        }
        pub fn staking_pool_mappings_mut(&mut self) -> &mut MoveTable {
            self.staking_pool_mappings.get_or_insert_default()
        }
        pub fn with_staking_pool_mappings(mut self, field: MoveTable) -> Self {
            self.staking_pool_mappings = Some(field.into());
            self
        }
        pub fn inactive_validators(&self) -> &MoveTable {
            self.inactive_validators
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn inactive_validators_opt(&self) -> Option<&MoveTable> {
            self.inactive_validators.as_ref().map(|field| field as _)
        }
        pub fn inactive_validators_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.inactive_validators.as_mut().map(|field| field as _)
        }
        pub fn inactive_validators_mut(&mut self) -> &mut MoveTable {
            self.inactive_validators.get_or_insert_default()
        }
        pub fn with_inactive_validators(mut self, field: MoveTable) -> Self {
            self.inactive_validators = Some(field.into());
            self
        }
        pub fn validator_candidates(&self) -> &MoveTable {
            self.validator_candidates
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn validator_candidates_opt(&self) -> Option<&MoveTable> {
            self.validator_candidates.as_ref().map(|field| field as _)
        }
        pub fn validator_candidates_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.validator_candidates.as_mut().map(|field| field as _)
        }
        pub fn validator_candidates_mut(&mut self) -> &mut MoveTable {
            self.validator_candidates.get_or_insert_default()
        }
        pub fn with_validator_candidates(mut self, field: MoveTable) -> Self {
            self.validator_candidates = Some(field.into());
            self
        }
        pub fn extra_fields(&self) -> &MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt(&self) -> Option<&MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn with_extra_fields(mut self, field: MoveTable) -> Self {
            self.extra_fields = Some(field.into());
            self
        }
    }
    impl Validator {
        pub const fn const_default() -> Self {
            Self {
                name: None,
                address: None,
                description: None,
                image_url: None,
                project_url: None,
                protocol_public_key: None,
                proof_of_possession: None,
                network_public_key: None,
                worker_public_key: None,
                network_address: None,
                p2p_address: None,
                primary_address: None,
                worker_address: None,
                next_epoch_protocol_public_key: None,
                next_epoch_proof_of_possession: None,
                next_epoch_network_public_key: None,
                next_epoch_worker_public_key: None,
                next_epoch_network_address: None,
                next_epoch_p2p_address: None,
                next_epoch_primary_address: None,
                next_epoch_worker_address: None,
                metadata_extra_fields: None,
                voting_power: None,
                operation_cap_id: None,
                gas_price: None,
                staking_pool: None,
                commission_rate: None,
                next_epoch_stake: None,
                next_epoch_gas_price: None,
                next_epoch_commission_rate: None,
                extra_fields: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Validator = Validator::const_default();
            &DEFAULT
        }
        pub fn with_name(mut self, field: String) -> Self {
            self.name = Some(field.into());
            self
        }
        pub fn with_address(mut self, field: String) -> Self {
            self.address = Some(field.into());
            self
        }
        pub fn with_description(mut self, field: String) -> Self {
            self.description = Some(field.into());
            self
        }
        pub fn with_image_url(mut self, field: String) -> Self {
            self.image_url = Some(field.into());
            self
        }
        pub fn with_project_url(mut self, field: String) -> Self {
            self.project_url = Some(field.into());
            self
        }
        pub fn with_protocol_public_key(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.protocol_public_key = Some(field.into());
            self
        }
        pub fn with_proof_of_possession(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.proof_of_possession = Some(field.into());
            self
        }
        pub fn with_network_public_key(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.network_public_key = Some(field.into());
            self
        }
        pub fn with_worker_public_key(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.worker_public_key = Some(field.into());
            self
        }
        pub fn with_network_address(mut self, field: String) -> Self {
            self.network_address = Some(field.into());
            self
        }
        pub fn with_p2p_address(mut self, field: String) -> Self {
            self.p2p_address = Some(field.into());
            self
        }
        pub fn with_primary_address(mut self, field: String) -> Self {
            self.primary_address = Some(field.into());
            self
        }
        pub fn with_worker_address(mut self, field: String) -> Self {
            self.worker_address = Some(field.into());
            self
        }
        pub fn with_next_epoch_protocol_public_key(
            mut self,
            field: ::prost::bytes::Bytes,
        ) -> Self {
            self.next_epoch_protocol_public_key = Some(field.into());
            self
        }
        pub fn with_next_epoch_proof_of_possession(
            mut self,
            field: ::prost::bytes::Bytes,
        ) -> Self {
            self.next_epoch_proof_of_possession = Some(field.into());
            self
        }
        pub fn with_next_epoch_network_public_key(
            mut self,
            field: ::prost::bytes::Bytes,
        ) -> Self {
            self.next_epoch_network_public_key = Some(field.into());
            self
        }
        pub fn with_next_epoch_worker_public_key(
            mut self,
            field: ::prost::bytes::Bytes,
        ) -> Self {
            self.next_epoch_worker_public_key = Some(field.into());
            self
        }
        pub fn with_next_epoch_network_address(mut self, field: String) -> Self {
            self.next_epoch_network_address = Some(field.into());
            self
        }
        pub fn with_next_epoch_p2p_address(mut self, field: String) -> Self {
            self.next_epoch_p2p_address = Some(field.into());
            self
        }
        pub fn with_next_epoch_primary_address(mut self, field: String) -> Self {
            self.next_epoch_primary_address = Some(field.into());
            self
        }
        pub fn with_next_epoch_worker_address(mut self, field: String) -> Self {
            self.next_epoch_worker_address = Some(field.into());
            self
        }
        pub fn metadata_extra_fields(&self) -> &MoveTable {
            self.metadata_extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn metadata_extra_fields_opt(&self) -> Option<&MoveTable> {
            self.metadata_extra_fields.as_ref().map(|field| field as _)
        }
        pub fn metadata_extra_fields_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.metadata_extra_fields.as_mut().map(|field| field as _)
        }
        pub fn metadata_extra_fields_mut(&mut self) -> &mut MoveTable {
            self.metadata_extra_fields.get_or_insert_default()
        }
        pub fn with_metadata_extra_fields(mut self, field: MoveTable) -> Self {
            self.metadata_extra_fields = Some(field.into());
            self
        }
        pub fn with_voting_power(mut self, field: u64) -> Self {
            self.voting_power = Some(field.into());
            self
        }
        pub fn with_operation_cap_id(mut self, field: String) -> Self {
            self.operation_cap_id = Some(field.into());
            self
        }
        pub fn with_gas_price(mut self, field: u64) -> Self {
            self.gas_price = Some(field.into());
            self
        }
        pub fn staking_pool(&self) -> &StakingPool {
            self.staking_pool
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| StakingPool::default_instance() as _)
        }
        pub fn staking_pool_opt(&self) -> Option<&StakingPool> {
            self.staking_pool.as_ref().map(|field| field as _)
        }
        pub fn staking_pool_opt_mut(&mut self) -> Option<&mut StakingPool> {
            self.staking_pool.as_mut().map(|field| field as _)
        }
        pub fn staking_pool_mut(&mut self) -> &mut StakingPool {
            self.staking_pool.get_or_insert_default()
        }
        pub fn with_staking_pool(mut self, field: StakingPool) -> Self {
            self.staking_pool = Some(field.into());
            self
        }
        pub fn with_commission_rate(mut self, field: u64) -> Self {
            self.commission_rate = Some(field.into());
            self
        }
        pub fn with_next_epoch_stake(mut self, field: u64) -> Self {
            self.next_epoch_stake = Some(field.into());
            self
        }
        pub fn with_next_epoch_gas_price(mut self, field: u64) -> Self {
            self.next_epoch_gas_price = Some(field.into());
            self
        }
        pub fn with_next_epoch_commission_rate(mut self, field: u64) -> Self {
            self.next_epoch_commission_rate = Some(field.into());
            self
        }
        pub fn extra_fields(&self) -> &MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt(&self) -> Option<&MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn with_extra_fields(mut self, field: MoveTable) -> Self {
            self.extra_fields = Some(field.into());
            self
        }
    }
    impl StakingPool {
        pub const fn const_default() -> Self {
            Self {
                id: None,
                activation_epoch: None,
                deactivation_epoch: None,
                sui_balance: None,
                rewards_pool: None,
                pool_token_balance: None,
                exchange_rates: None,
                pending_stake: None,
                pending_total_sui_withdraw: None,
                pending_pool_token_withdraw: None,
                extra_fields: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: StakingPool = StakingPool::const_default();
            &DEFAULT
        }
        pub fn with_id(mut self, field: String) -> Self {
            self.id = Some(field.into());
            self
        }
        pub fn with_activation_epoch(mut self, field: u64) -> Self {
            self.activation_epoch = Some(field.into());
            self
        }
        pub fn with_deactivation_epoch(mut self, field: u64) -> Self {
            self.deactivation_epoch = Some(field.into());
            self
        }
        pub fn with_sui_balance(mut self, field: u64) -> Self {
            self.sui_balance = Some(field.into());
            self
        }
        pub fn with_rewards_pool(mut self, field: u64) -> Self {
            self.rewards_pool = Some(field.into());
            self
        }
        pub fn with_pool_token_balance(mut self, field: u64) -> Self {
            self.pool_token_balance = Some(field.into());
            self
        }
        pub fn exchange_rates(&self) -> &MoveTable {
            self.exchange_rates
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn exchange_rates_opt(&self) -> Option<&MoveTable> {
            self.exchange_rates.as_ref().map(|field| field as _)
        }
        pub fn exchange_rates_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.exchange_rates.as_mut().map(|field| field as _)
        }
        pub fn exchange_rates_mut(&mut self) -> &mut MoveTable {
            self.exchange_rates.get_or_insert_default()
        }
        pub fn with_exchange_rates(mut self, field: MoveTable) -> Self {
            self.exchange_rates = Some(field.into());
            self
        }
        pub fn with_pending_stake(mut self, field: u64) -> Self {
            self.pending_stake = Some(field.into());
            self
        }
        pub fn with_pending_total_sui_withdraw(mut self, field: u64) -> Self {
            self.pending_total_sui_withdraw = Some(field.into());
            self
        }
        pub fn with_pending_pool_token_withdraw(mut self, field: u64) -> Self {
            self.pending_pool_token_withdraw = Some(field.into());
            self
        }
        pub fn extra_fields(&self) -> &MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt(&self) -> Option<&MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn with_extra_fields(mut self, field: MoveTable) -> Self {
            self.extra_fields = Some(field.into());
            self
        }
    }
    impl Transaction {
        pub const fn const_default() -> Self {
            Self {
                bcs: None,
                digest: None,
                version: None,
                kind: None,
                sender: None,
                gas_payment: None,
                expiration: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Transaction = Transaction::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Bcs::default_instance() as _)
        }
        pub fn bcs_opt(&self) -> Option<&Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn with_bcs(mut self, field: Bcs) -> Self {
            self.bcs = Some(field.into());
            self
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: i32) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn kind(&self) -> &TransactionKind {
            self.kind
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| TransactionKind::default_instance() as _)
        }
        pub fn kind_opt(&self) -> Option<&TransactionKind> {
            self.kind.as_ref().map(|field| field as _)
        }
        pub fn kind_opt_mut(&mut self) -> Option<&mut TransactionKind> {
            self.kind.as_mut().map(|field| field as _)
        }
        pub fn kind_mut(&mut self) -> &mut TransactionKind {
            self.kind.get_or_insert_default()
        }
        pub fn with_kind(mut self, field: TransactionKind) -> Self {
            self.kind = Some(field.into());
            self
        }
        pub fn with_sender(mut self, field: String) -> Self {
            self.sender = Some(field.into());
            self
        }
        pub fn gas_payment(&self) -> &GasPayment {
            self.gas_payment
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| GasPayment::default_instance() as _)
        }
        pub fn gas_payment_opt(&self) -> Option<&GasPayment> {
            self.gas_payment.as_ref().map(|field| field as _)
        }
        pub fn gas_payment_opt_mut(&mut self) -> Option<&mut GasPayment> {
            self.gas_payment.as_mut().map(|field| field as _)
        }
        pub fn gas_payment_mut(&mut self) -> &mut GasPayment {
            self.gas_payment.get_or_insert_default()
        }
        pub fn with_gas_payment(mut self, field: GasPayment) -> Self {
            self.gas_payment = Some(field.into());
            self
        }
        pub fn expiration(&self) -> &TransactionExpiration {
            self.expiration
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| TransactionExpiration::default_instance() as _)
        }
        pub fn expiration_opt(&self) -> Option<&TransactionExpiration> {
            self.expiration.as_ref().map(|field| field as _)
        }
        pub fn expiration_opt_mut(&mut self) -> Option<&mut TransactionExpiration> {
            self.expiration.as_mut().map(|field| field as _)
        }
        pub fn expiration_mut(&mut self) -> &mut TransactionExpiration {
            self.expiration.get_or_insert_default()
        }
        pub fn with_expiration(mut self, field: TransactionExpiration) -> Self {
            self.expiration = Some(field.into());
            self
        }
    }
    impl GasPayment {
        pub const fn const_default() -> Self {
            Self {
                objects: Vec::new(),
                owner: None,
                price: None,
                budget: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GasPayment = GasPayment::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[ObjectReference] {
            &self.objects
        }
        pub fn with_objects(mut self, field: Vec<ObjectReference>) -> Self {
            self.objects = field;
            self
        }
        pub fn with_owner(mut self, field: String) -> Self {
            self.owner = Some(field.into());
            self
        }
        pub fn with_price(mut self, field: u64) -> Self {
            self.price = Some(field.into());
            self
        }
        pub fn with_budget(mut self, field: u64) -> Self {
            self.budget = Some(field.into());
            self
        }
    }
    impl TransactionExpiration {
        pub const fn const_default() -> Self {
            Self { kind: None, epoch: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: TransactionExpiration = TransactionExpiration::const_default();
            &DEFAULT
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
    }
    impl TransactionKind {
        pub const fn const_default() -> Self {
            Self { kind: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: TransactionKind = TransactionKind::const_default();
            &DEFAULT
        }
        pub fn programmable_transaction(&self) -> &ProgrammableTransaction {
            if let Some(transaction_kind::Kind::ProgrammableTransaction(field)) = &self
                .kind
            {
                field as _
            } else {
                ProgrammableTransaction::default_instance() as _
            }
        }
        pub fn programmable_transaction_opt(&self) -> Option<&ProgrammableTransaction> {
            if let Some(transaction_kind::Kind::ProgrammableTransaction(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn programmable_transaction_opt_mut(
            &mut self,
        ) -> Option<&mut ProgrammableTransaction> {
            if let Some(transaction_kind::Kind::ProgrammableTransaction(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn programmable_transaction_mut(&mut self) -> &mut ProgrammableTransaction {
            if self.programmable_transaction_opt_mut().is_none() {
                self.kind = Some(
                    transaction_kind::Kind::ProgrammableTransaction(
                        ProgrammableTransaction::default(),
                    ),
                );
            }
            self.programmable_transaction_opt_mut().unwrap()
        }
        pub fn with_programmable_transaction(
            mut self,
            field: ProgrammableTransaction,
        ) -> Self {
            self.kind = Some(
                transaction_kind::Kind::ProgrammableTransaction(field.into()),
            );
            self
        }
        pub fn programmable_system_transaction(&self) -> &ProgrammableTransaction {
            if let Some(transaction_kind::Kind::ProgrammableSystemTransaction(field)) = &self
                .kind
            {
                field as _
            } else {
                ProgrammableTransaction::default_instance() as _
            }
        }
        pub fn programmable_system_transaction_opt(
            &self,
        ) -> Option<&ProgrammableTransaction> {
            if let Some(transaction_kind::Kind::ProgrammableSystemTransaction(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn programmable_system_transaction_opt_mut(
            &mut self,
        ) -> Option<&mut ProgrammableTransaction> {
            if let Some(transaction_kind::Kind::ProgrammableSystemTransaction(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn programmable_system_transaction_mut(
            &mut self,
        ) -> &mut ProgrammableTransaction {
            if self.programmable_system_transaction_opt_mut().is_none() {
                self.kind = Some(
                    transaction_kind::Kind::ProgrammableSystemTransaction(
                        ProgrammableTransaction::default(),
                    ),
                );
            }
            self.programmable_system_transaction_opt_mut().unwrap()
        }
        pub fn with_programmable_system_transaction(
            mut self,
            field: ProgrammableTransaction,
        ) -> Self {
            self.kind = Some(
                transaction_kind::Kind::ProgrammableSystemTransaction(field.into()),
            );
            self
        }
        pub fn change_epoch(&self) -> &ChangeEpoch {
            if let Some(transaction_kind::Kind::ChangeEpoch(field)) = &self.kind {
                field as _
            } else {
                ChangeEpoch::default_instance() as _
            }
        }
        pub fn change_epoch_opt(&self) -> Option<&ChangeEpoch> {
            if let Some(transaction_kind::Kind::ChangeEpoch(field)) = &self.kind {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn change_epoch_opt_mut(&mut self) -> Option<&mut ChangeEpoch> {
            if let Some(transaction_kind::Kind::ChangeEpoch(field)) = &mut self.kind {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn change_epoch_mut(&mut self) -> &mut ChangeEpoch {
            if self.change_epoch_opt_mut().is_none() {
                self.kind = Some(
                    transaction_kind::Kind::ChangeEpoch(ChangeEpoch::default()),
                );
            }
            self.change_epoch_opt_mut().unwrap()
        }
        pub fn with_change_epoch(mut self, field: ChangeEpoch) -> Self {
            self.kind = Some(transaction_kind::Kind::ChangeEpoch(field.into()));
            self
        }
        pub fn genesis(&self) -> &GenesisTransaction {
            if let Some(transaction_kind::Kind::Genesis(field)) = &self.kind {
                field as _
            } else {
                GenesisTransaction::default_instance() as _
            }
        }
        pub fn genesis_opt(&self) -> Option<&GenesisTransaction> {
            if let Some(transaction_kind::Kind::Genesis(field)) = &self.kind {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn genesis_opt_mut(&mut self) -> Option<&mut GenesisTransaction> {
            if let Some(transaction_kind::Kind::Genesis(field)) = &mut self.kind {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn genesis_mut(&mut self) -> &mut GenesisTransaction {
            if self.genesis_opt_mut().is_none() {
                self.kind = Some(
                    transaction_kind::Kind::Genesis(GenesisTransaction::default()),
                );
            }
            self.genesis_opt_mut().unwrap()
        }
        pub fn with_genesis(mut self, field: GenesisTransaction) -> Self {
            self.kind = Some(transaction_kind::Kind::Genesis(field.into()));
            self
        }
        pub fn consensus_commit_prologue_v1(&self) -> &ConsensusCommitPrologue {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV1(field)) = &self
                .kind
            {
                field as _
            } else {
                ConsensusCommitPrologue::default_instance() as _
            }
        }
        pub fn consensus_commit_prologue_v1_opt(
            &self,
        ) -> Option<&ConsensusCommitPrologue> {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV1(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v1_opt_mut(
            &mut self,
        ) -> Option<&mut ConsensusCommitPrologue> {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV1(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v1_mut(
            &mut self,
        ) -> &mut ConsensusCommitPrologue {
            if self.consensus_commit_prologue_v1_opt_mut().is_none() {
                self.kind = Some(
                    transaction_kind::Kind::ConsensusCommitPrologueV1(
                        ConsensusCommitPrologue::default(),
                    ),
                );
            }
            self.consensus_commit_prologue_v1_opt_mut().unwrap()
        }
        pub fn with_consensus_commit_prologue_v1(
            mut self,
            field: ConsensusCommitPrologue,
        ) -> Self {
            self.kind = Some(
                transaction_kind::Kind::ConsensusCommitPrologueV1(field.into()),
            );
            self
        }
        pub fn authenticator_state_update(&self) -> &AuthenticatorStateUpdate {
            if let Some(transaction_kind::Kind::AuthenticatorStateUpdate(field)) = &self
                .kind
            {
                field as _
            } else {
                AuthenticatorStateUpdate::default_instance() as _
            }
        }
        pub fn authenticator_state_update_opt(
            &self,
        ) -> Option<&AuthenticatorStateUpdate> {
            if let Some(transaction_kind::Kind::AuthenticatorStateUpdate(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn authenticator_state_update_opt_mut(
            &mut self,
        ) -> Option<&mut AuthenticatorStateUpdate> {
            if let Some(transaction_kind::Kind::AuthenticatorStateUpdate(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn authenticator_state_update_mut(
            &mut self,
        ) -> &mut AuthenticatorStateUpdate {
            if self.authenticator_state_update_opt_mut().is_none() {
                self.kind = Some(
                    transaction_kind::Kind::AuthenticatorStateUpdate(
                        AuthenticatorStateUpdate::default(),
                    ),
                );
            }
            self.authenticator_state_update_opt_mut().unwrap()
        }
        pub fn with_authenticator_state_update(
            mut self,
            field: AuthenticatorStateUpdate,
        ) -> Self {
            self.kind = Some(
                transaction_kind::Kind::AuthenticatorStateUpdate(field.into()),
            );
            self
        }
        pub fn end_of_epoch(&self) -> &EndOfEpochTransaction {
            if let Some(transaction_kind::Kind::EndOfEpoch(field)) = &self.kind {
                field as _
            } else {
                EndOfEpochTransaction::default_instance() as _
            }
        }
        pub fn end_of_epoch_opt(&self) -> Option<&EndOfEpochTransaction> {
            if let Some(transaction_kind::Kind::EndOfEpoch(field)) = &self.kind {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn end_of_epoch_opt_mut(&mut self) -> Option<&mut EndOfEpochTransaction> {
            if let Some(transaction_kind::Kind::EndOfEpoch(field)) = &mut self.kind {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn end_of_epoch_mut(&mut self) -> &mut EndOfEpochTransaction {
            if self.end_of_epoch_opt_mut().is_none() {
                self.kind = Some(
                    transaction_kind::Kind::EndOfEpoch(EndOfEpochTransaction::default()),
                );
            }
            self.end_of_epoch_opt_mut().unwrap()
        }
        pub fn with_end_of_epoch(mut self, field: EndOfEpochTransaction) -> Self {
            self.kind = Some(transaction_kind::Kind::EndOfEpoch(field.into()));
            self
        }
        pub fn randomness_state_update(&self) -> &RandomnessStateUpdate {
            if let Some(transaction_kind::Kind::RandomnessStateUpdate(field)) = &self
                .kind
            {
                field as _
            } else {
                RandomnessStateUpdate::default_instance() as _
            }
        }
        pub fn randomness_state_update_opt(&self) -> Option<&RandomnessStateUpdate> {
            if let Some(transaction_kind::Kind::RandomnessStateUpdate(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn randomness_state_update_opt_mut(
            &mut self,
        ) -> Option<&mut RandomnessStateUpdate> {
            if let Some(transaction_kind::Kind::RandomnessStateUpdate(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn randomness_state_update_mut(&mut self) -> &mut RandomnessStateUpdate {
            if self.randomness_state_update_opt_mut().is_none() {
                self.kind = Some(
                    transaction_kind::Kind::RandomnessStateUpdate(
                        RandomnessStateUpdate::default(),
                    ),
                );
            }
            self.randomness_state_update_opt_mut().unwrap()
        }
        pub fn with_randomness_state_update(
            mut self,
            field: RandomnessStateUpdate,
        ) -> Self {
            self.kind = Some(
                transaction_kind::Kind::RandomnessStateUpdate(field.into()),
            );
            self
        }
        pub fn consensus_commit_prologue_v2(&self) -> &ConsensusCommitPrologue {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV2(field)) = &self
                .kind
            {
                field as _
            } else {
                ConsensusCommitPrologue::default_instance() as _
            }
        }
        pub fn consensus_commit_prologue_v2_opt(
            &self,
        ) -> Option<&ConsensusCommitPrologue> {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV2(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v2_opt_mut(
            &mut self,
        ) -> Option<&mut ConsensusCommitPrologue> {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV2(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v2_mut(
            &mut self,
        ) -> &mut ConsensusCommitPrologue {
            if self.consensus_commit_prologue_v2_opt_mut().is_none() {
                self.kind = Some(
                    transaction_kind::Kind::ConsensusCommitPrologueV2(
                        ConsensusCommitPrologue::default(),
                    ),
                );
            }
            self.consensus_commit_prologue_v2_opt_mut().unwrap()
        }
        pub fn with_consensus_commit_prologue_v2(
            mut self,
            field: ConsensusCommitPrologue,
        ) -> Self {
            self.kind = Some(
                transaction_kind::Kind::ConsensusCommitPrologueV2(field.into()),
            );
            self
        }
        pub fn consensus_commit_prologue_v3(&self) -> &ConsensusCommitPrologue {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV3(field)) = &self
                .kind
            {
                field as _
            } else {
                ConsensusCommitPrologue::default_instance() as _
            }
        }
        pub fn consensus_commit_prologue_v3_opt(
            &self,
        ) -> Option<&ConsensusCommitPrologue> {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV3(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v3_opt_mut(
            &mut self,
        ) -> Option<&mut ConsensusCommitPrologue> {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV3(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v3_mut(
            &mut self,
        ) -> &mut ConsensusCommitPrologue {
            if self.consensus_commit_prologue_v3_opt_mut().is_none() {
                self.kind = Some(
                    transaction_kind::Kind::ConsensusCommitPrologueV3(
                        ConsensusCommitPrologue::default(),
                    ),
                );
            }
            self.consensus_commit_prologue_v3_opt_mut().unwrap()
        }
        pub fn with_consensus_commit_prologue_v3(
            mut self,
            field: ConsensusCommitPrologue,
        ) -> Self {
            self.kind = Some(
                transaction_kind::Kind::ConsensusCommitPrologueV3(field.into()),
            );
            self
        }
        pub fn consensus_commit_prologue_v4(&self) -> &ConsensusCommitPrologue {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV4(field)) = &self
                .kind
            {
                field as _
            } else {
                ConsensusCommitPrologue::default_instance() as _
            }
        }
        pub fn consensus_commit_prologue_v4_opt(
            &self,
        ) -> Option<&ConsensusCommitPrologue> {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV4(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v4_opt_mut(
            &mut self,
        ) -> Option<&mut ConsensusCommitPrologue> {
            if let Some(transaction_kind::Kind::ConsensusCommitPrologueV4(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v4_mut(
            &mut self,
        ) -> &mut ConsensusCommitPrologue {
            if self.consensus_commit_prologue_v4_opt_mut().is_none() {
                self.kind = Some(
                    transaction_kind::Kind::ConsensusCommitPrologueV4(
                        ConsensusCommitPrologue::default(),
                    ),
                );
            }
            self.consensus_commit_prologue_v4_opt_mut().unwrap()
        }
        pub fn with_consensus_commit_prologue_v4(
            mut self,
            field: ConsensusCommitPrologue,
        ) -> Self {
            self.kind = Some(
                transaction_kind::Kind::ConsensusCommitPrologueV4(field.into()),
            );
            self
        }
    }
    impl ProgrammableTransaction {
        pub const fn const_default() -> Self {
            Self {
                inputs: Vec::new(),
                commands: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ProgrammableTransaction = ProgrammableTransaction::const_default();
            &DEFAULT
        }
        pub fn inputs(&self) -> &[Input] {
            &self.inputs
        }
        pub fn with_inputs(mut self, field: Vec<Input>) -> Self {
            self.inputs = field;
            self
        }
        pub fn commands(&self) -> &[Command] {
            &self.commands
        }
        pub fn with_commands(mut self, field: Vec<Command>) -> Self {
            self.commands = field;
            self
        }
    }
    impl Command {
        pub const fn const_default() -> Self {
            Self { command: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Command = Command::const_default();
            &DEFAULT
        }
        pub fn move_call(&self) -> &MoveCall {
            if let Some(command::Command::MoveCall(field)) = &self.command {
                field as _
            } else {
                MoveCall::default_instance() as _
            }
        }
        pub fn move_call_opt(&self) -> Option<&MoveCall> {
            if let Some(command::Command::MoveCall(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn move_call_opt_mut(&mut self) -> Option<&mut MoveCall> {
            if let Some(command::Command::MoveCall(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn move_call_mut(&mut self) -> &mut MoveCall {
            if self.move_call_opt_mut().is_none() {
                self.command = Some(command::Command::MoveCall(MoveCall::default()));
            }
            self.move_call_opt_mut().unwrap()
        }
        pub fn with_move_call(mut self, field: MoveCall) -> Self {
            self.command = Some(command::Command::MoveCall(field.into()));
            self
        }
        pub fn transfer_objects(&self) -> &TransferObjects {
            if let Some(command::Command::TransferObjects(field)) = &self.command {
                field as _
            } else {
                TransferObjects::default_instance() as _
            }
        }
        pub fn transfer_objects_opt(&self) -> Option<&TransferObjects> {
            if let Some(command::Command::TransferObjects(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn transfer_objects_opt_mut(&mut self) -> Option<&mut TransferObjects> {
            if let Some(command::Command::TransferObjects(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn transfer_objects_mut(&mut self) -> &mut TransferObjects {
            if self.transfer_objects_opt_mut().is_none() {
                self.command = Some(
                    command::Command::TransferObjects(TransferObjects::default()),
                );
            }
            self.transfer_objects_opt_mut().unwrap()
        }
        pub fn with_transfer_objects(mut self, field: TransferObjects) -> Self {
            self.command = Some(command::Command::TransferObjects(field.into()));
            self
        }
        pub fn split_coins(&self) -> &SplitCoins {
            if let Some(command::Command::SplitCoins(field)) = &self.command {
                field as _
            } else {
                SplitCoins::default_instance() as _
            }
        }
        pub fn split_coins_opt(&self) -> Option<&SplitCoins> {
            if let Some(command::Command::SplitCoins(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn split_coins_opt_mut(&mut self) -> Option<&mut SplitCoins> {
            if let Some(command::Command::SplitCoins(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn split_coins_mut(&mut self) -> &mut SplitCoins {
            if self.split_coins_opt_mut().is_none() {
                self.command = Some(command::Command::SplitCoins(SplitCoins::default()));
            }
            self.split_coins_opt_mut().unwrap()
        }
        pub fn with_split_coins(mut self, field: SplitCoins) -> Self {
            self.command = Some(command::Command::SplitCoins(field.into()));
            self
        }
        pub fn merge_coins(&self) -> &MergeCoins {
            if let Some(command::Command::MergeCoins(field)) = &self.command {
                field as _
            } else {
                MergeCoins::default_instance() as _
            }
        }
        pub fn merge_coins_opt(&self) -> Option<&MergeCoins> {
            if let Some(command::Command::MergeCoins(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn merge_coins_opt_mut(&mut self) -> Option<&mut MergeCoins> {
            if let Some(command::Command::MergeCoins(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn merge_coins_mut(&mut self) -> &mut MergeCoins {
            if self.merge_coins_opt_mut().is_none() {
                self.command = Some(command::Command::MergeCoins(MergeCoins::default()));
            }
            self.merge_coins_opt_mut().unwrap()
        }
        pub fn with_merge_coins(mut self, field: MergeCoins) -> Self {
            self.command = Some(command::Command::MergeCoins(field.into()));
            self
        }
        pub fn publish(&self) -> &Publish {
            if let Some(command::Command::Publish(field)) = &self.command {
                field as _
            } else {
                Publish::default_instance() as _
            }
        }
        pub fn publish_opt(&self) -> Option<&Publish> {
            if let Some(command::Command::Publish(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn publish_opt_mut(&mut self) -> Option<&mut Publish> {
            if let Some(command::Command::Publish(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn publish_mut(&mut self) -> &mut Publish {
            if self.publish_opt_mut().is_none() {
                self.command = Some(command::Command::Publish(Publish::default()));
            }
            self.publish_opt_mut().unwrap()
        }
        pub fn with_publish(mut self, field: Publish) -> Self {
            self.command = Some(command::Command::Publish(field.into()));
            self
        }
        pub fn make_move_vector(&self) -> &MakeMoveVector {
            if let Some(command::Command::MakeMoveVector(field)) = &self.command {
                field as _
            } else {
                MakeMoveVector::default_instance() as _
            }
        }
        pub fn make_move_vector_opt(&self) -> Option<&MakeMoveVector> {
            if let Some(command::Command::MakeMoveVector(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn make_move_vector_opt_mut(&mut self) -> Option<&mut MakeMoveVector> {
            if let Some(command::Command::MakeMoveVector(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn make_move_vector_mut(&mut self) -> &mut MakeMoveVector {
            if self.make_move_vector_opt_mut().is_none() {
                self.command = Some(
                    command::Command::MakeMoveVector(MakeMoveVector::default()),
                );
            }
            self.make_move_vector_opt_mut().unwrap()
        }
        pub fn with_make_move_vector(mut self, field: MakeMoveVector) -> Self {
            self.command = Some(command::Command::MakeMoveVector(field.into()));
            self
        }
        pub fn upgrade(&self) -> &Upgrade {
            if let Some(command::Command::Upgrade(field)) = &self.command {
                field as _
            } else {
                Upgrade::default_instance() as _
            }
        }
        pub fn upgrade_opt(&self) -> Option<&Upgrade> {
            if let Some(command::Command::Upgrade(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn upgrade_opt_mut(&mut self) -> Option<&mut Upgrade> {
            if let Some(command::Command::Upgrade(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn upgrade_mut(&mut self) -> &mut Upgrade {
            if self.upgrade_opt_mut().is_none() {
                self.command = Some(command::Command::Upgrade(Upgrade::default()));
            }
            self.upgrade_opt_mut().unwrap()
        }
        pub fn with_upgrade(mut self, field: Upgrade) -> Self {
            self.command = Some(command::Command::Upgrade(field.into()));
            self
        }
    }
    impl MoveCall {
        pub const fn const_default() -> Self {
            Self {
                package: None,
                module: None,
                function: None,
                type_arguments: Vec::new(),
                arguments: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: MoveCall = MoveCall::const_default();
            &DEFAULT
        }
        pub fn with_package(mut self, field: String) -> Self {
            self.package = Some(field.into());
            self
        }
        pub fn with_module(mut self, field: String) -> Self {
            self.module = Some(field.into());
            self
        }
        pub fn with_function(mut self, field: String) -> Self {
            self.function = Some(field.into());
            self
        }
        pub fn type_arguments(&self) -> &[String] {
            &self.type_arguments
        }
        pub fn with_type_arguments(mut self, field: Vec<String>) -> Self {
            self.type_arguments = field;
            self
        }
        pub fn arguments(&self) -> &[Argument] {
            &self.arguments
        }
        pub fn with_arguments(mut self, field: Vec<Argument>) -> Self {
            self.arguments = field;
            self
        }
    }
    impl TransferObjects {
        pub const fn const_default() -> Self {
            Self {
                objects: Vec::new(),
                address: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: TransferObjects = TransferObjects::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[Argument] {
            &self.objects
        }
        pub fn with_objects(mut self, field: Vec<Argument>) -> Self {
            self.objects = field;
            self
        }
        pub fn address(&self) -> &Argument {
            self.address
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Argument::default_instance() as _)
        }
        pub fn address_opt(&self) -> Option<&Argument> {
            self.address.as_ref().map(|field| field as _)
        }
        pub fn address_opt_mut(&mut self) -> Option<&mut Argument> {
            self.address.as_mut().map(|field| field as _)
        }
        pub fn address_mut(&mut self) -> &mut Argument {
            self.address.get_or_insert_default()
        }
        pub fn with_address(mut self, field: Argument) -> Self {
            self.address = Some(field.into());
            self
        }
    }
    impl SplitCoins {
        pub const fn const_default() -> Self {
            Self {
                coin: None,
                amounts: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: SplitCoins = SplitCoins::const_default();
            &DEFAULT
        }
        pub fn coin(&self) -> &Argument {
            self.coin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Argument::default_instance() as _)
        }
        pub fn coin_opt(&self) -> Option<&Argument> {
            self.coin.as_ref().map(|field| field as _)
        }
        pub fn coin_opt_mut(&mut self) -> Option<&mut Argument> {
            self.coin.as_mut().map(|field| field as _)
        }
        pub fn coin_mut(&mut self) -> &mut Argument {
            self.coin.get_or_insert_default()
        }
        pub fn with_coin(mut self, field: Argument) -> Self {
            self.coin = Some(field.into());
            self
        }
        pub fn amounts(&self) -> &[Argument] {
            &self.amounts
        }
        pub fn with_amounts(mut self, field: Vec<Argument>) -> Self {
            self.amounts = field;
            self
        }
    }
    impl MergeCoins {
        pub const fn const_default() -> Self {
            Self {
                coin: None,
                coins_to_merge: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: MergeCoins = MergeCoins::const_default();
            &DEFAULT
        }
        pub fn coin(&self) -> &Argument {
            self.coin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Argument::default_instance() as _)
        }
        pub fn coin_opt(&self) -> Option<&Argument> {
            self.coin.as_ref().map(|field| field as _)
        }
        pub fn coin_opt_mut(&mut self) -> Option<&mut Argument> {
            self.coin.as_mut().map(|field| field as _)
        }
        pub fn coin_mut(&mut self) -> &mut Argument {
            self.coin.get_or_insert_default()
        }
        pub fn with_coin(mut self, field: Argument) -> Self {
            self.coin = Some(field.into());
            self
        }
        pub fn coins_to_merge(&self) -> &[Argument] {
            &self.coins_to_merge
        }
        pub fn with_coins_to_merge(mut self, field: Vec<Argument>) -> Self {
            self.coins_to_merge = field;
            self
        }
    }
    impl Publish {
        pub const fn const_default() -> Self {
            Self {
                modules: Vec::new(),
                dependencies: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Publish = Publish::const_default();
            &DEFAULT
        }
        pub fn modules(&self) -> &[::prost::bytes::Bytes] {
            &self.modules
        }
        pub fn with_modules(mut self, field: Vec<::prost::bytes::Bytes>) -> Self {
            self.modules = field;
            self
        }
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        pub fn with_dependencies(mut self, field: Vec<String>) -> Self {
            self.dependencies = field;
            self
        }
    }
    impl MakeMoveVector {
        pub const fn const_default() -> Self {
            Self {
                element_type: None,
                elements: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: MakeMoveVector = MakeMoveVector::const_default();
            &DEFAULT
        }
        pub fn with_element_type(mut self, field: String) -> Self {
            self.element_type = Some(field.into());
            self
        }
        pub fn elements(&self) -> &[Argument] {
            &self.elements
        }
        pub fn with_elements(mut self, field: Vec<Argument>) -> Self {
            self.elements = field;
            self
        }
    }
    impl Upgrade {
        pub const fn const_default() -> Self {
            Self {
                modules: Vec::new(),
                dependencies: Vec::new(),
                package: None,
                ticket: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Upgrade = Upgrade::const_default();
            &DEFAULT
        }
        pub fn modules(&self) -> &[::prost::bytes::Bytes] {
            &self.modules
        }
        pub fn with_modules(mut self, field: Vec<::prost::bytes::Bytes>) -> Self {
            self.modules = field;
            self
        }
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        pub fn with_dependencies(mut self, field: Vec<String>) -> Self {
            self.dependencies = field;
            self
        }
        pub fn with_package(mut self, field: String) -> Self {
            self.package = Some(field.into());
            self
        }
        pub fn ticket(&self) -> &Argument {
            self.ticket
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Argument::default_instance() as _)
        }
        pub fn ticket_opt(&self) -> Option<&Argument> {
            self.ticket.as_ref().map(|field| field as _)
        }
        pub fn ticket_opt_mut(&mut self) -> Option<&mut Argument> {
            self.ticket.as_mut().map(|field| field as _)
        }
        pub fn ticket_mut(&mut self) -> &mut Argument {
            self.ticket.get_or_insert_default()
        }
        pub fn with_ticket(mut self, field: Argument) -> Self {
            self.ticket = Some(field.into());
            self
        }
    }
    impl RandomnessStateUpdate {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                randomness_round: None,
                random_bytes: None,
                randomness_object_initial_shared_version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: RandomnessStateUpdate = RandomnessStateUpdate::const_default();
            &DEFAULT
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
        pub fn with_randomness_round(mut self, field: u64) -> Self {
            self.randomness_round = Some(field.into());
            self
        }
        pub fn with_random_bytes(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.random_bytes = Some(field.into());
            self
        }
        pub fn with_randomness_object_initial_shared_version(
            mut self,
            field: u64,
        ) -> Self {
            self.randomness_object_initial_shared_version = Some(field.into());
            self
        }
    }
    impl ChangeEpoch {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                protocol_version: None,
                storage_charge: None,
                computation_charge: None,
                storage_rebate: None,
                non_refundable_storage_fee: None,
                epoch_start_timestamp: None,
                system_packages: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ChangeEpoch = ChangeEpoch::const_default();
            &DEFAULT
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
        pub fn with_protocol_version(mut self, field: u64) -> Self {
            self.protocol_version = Some(field.into());
            self
        }
        pub fn with_storage_charge(mut self, field: u64) -> Self {
            self.storage_charge = Some(field.into());
            self
        }
        pub fn with_computation_charge(mut self, field: u64) -> Self {
            self.computation_charge = Some(field.into());
            self
        }
        pub fn with_storage_rebate(mut self, field: u64) -> Self {
            self.storage_rebate = Some(field.into());
            self
        }
        pub fn with_non_refundable_storage_fee(mut self, field: u64) -> Self {
            self.non_refundable_storage_fee = Some(field.into());
            self
        }
        pub fn system_packages(&self) -> &[SystemPackage] {
            &self.system_packages
        }
        pub fn with_system_packages(mut self, field: Vec<SystemPackage>) -> Self {
            self.system_packages = field;
            self
        }
    }
    impl SystemPackage {
        pub const fn const_default() -> Self {
            Self {
                version: None,
                modules: Vec::new(),
                dependencies: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: SystemPackage = SystemPackage::const_default();
            &DEFAULT
        }
        pub fn with_version(mut self, field: u64) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn modules(&self) -> &[::prost::bytes::Bytes] {
            &self.modules
        }
        pub fn with_modules(mut self, field: Vec<::prost::bytes::Bytes>) -> Self {
            self.modules = field;
            self
        }
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        pub fn with_dependencies(mut self, field: Vec<String>) -> Self {
            self.dependencies = field;
            self
        }
    }
    impl GenesisTransaction {
        pub const fn const_default() -> Self {
            Self { objects: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: GenesisTransaction = GenesisTransaction::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[Object] {
            &self.objects
        }
        pub fn with_objects(mut self, field: Vec<Object>) -> Self {
            self.objects = field;
            self
        }
    }
    impl ConsensusCommitPrologue {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                round: None,
                commit_timestamp: None,
                consensus_commit_digest: None,
                sub_dag_index: None,
                consensus_determined_version_assignments: None,
                additional_state_digest: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ConsensusCommitPrologue = ConsensusCommitPrologue::const_default();
            &DEFAULT
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
        pub fn with_round(mut self, field: u64) -> Self {
            self.round = Some(field.into());
            self
        }
        pub fn with_consensus_commit_digest(mut self, field: String) -> Self {
            self.consensus_commit_digest = Some(field.into());
            self
        }
        pub fn with_sub_dag_index(mut self, field: u64) -> Self {
            self.sub_dag_index = Some(field.into());
            self
        }
        pub fn consensus_determined_version_assignments(
            &self,
        ) -> &ConsensusDeterminedVersionAssignments {
            self.consensus_determined_version_assignments
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    ConsensusDeterminedVersionAssignments::default_instance() as _
                })
        }
        pub fn consensus_determined_version_assignments_opt(
            &self,
        ) -> Option<&ConsensusDeterminedVersionAssignments> {
            self.consensus_determined_version_assignments
                .as_ref()
                .map(|field| field as _)
        }
        pub fn consensus_determined_version_assignments_opt_mut(
            &mut self,
        ) -> Option<&mut ConsensusDeterminedVersionAssignments> {
            self.consensus_determined_version_assignments
                .as_mut()
                .map(|field| field as _)
        }
        pub fn consensus_determined_version_assignments_mut(
            &mut self,
        ) -> &mut ConsensusDeterminedVersionAssignments {
            self.consensus_determined_version_assignments.get_or_insert_default()
        }
        pub fn with_consensus_determined_version_assignments(
            mut self,
            field: ConsensusDeterminedVersionAssignments,
        ) -> Self {
            self.consensus_determined_version_assignments = Some(field.into());
            self
        }
        pub fn with_additional_state_digest(mut self, field: String) -> Self {
            self.additional_state_digest = Some(field.into());
            self
        }
    }
    impl VersionAssignment {
        pub const fn const_default() -> Self {
            Self {
                object_id: None,
                start_version: None,
                version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: VersionAssignment = VersionAssignment::const_default();
            &DEFAULT
        }
        pub fn with_object_id(mut self, field: String) -> Self {
            self.object_id = Some(field.into());
            self
        }
        pub fn with_start_version(mut self, field: u64) -> Self {
            self.start_version = Some(field.into());
            self
        }
        pub fn with_version(mut self, field: u64) -> Self {
            self.version = Some(field.into());
            self
        }
    }
    impl CanceledTransaction {
        pub const fn const_default() -> Self {
            Self {
                digest: None,
                version_assignments: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: CanceledTransaction = CanceledTransaction::const_default();
            &DEFAULT
        }
        pub fn with_digest(mut self, field: String) -> Self {
            self.digest = Some(field.into());
            self
        }
        pub fn version_assignments(&self) -> &[VersionAssignment] {
            &self.version_assignments
        }
        pub fn with_version_assignments(
            mut self,
            field: Vec<VersionAssignment>,
        ) -> Self {
            self.version_assignments = field;
            self
        }
    }
    impl ConsensusDeterminedVersionAssignments {
        pub const fn const_default() -> Self {
            Self {
                version: None,
                canceled_transactions: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ConsensusDeterminedVersionAssignments = ConsensusDeterminedVersionAssignments::const_default();
            &DEFAULT
        }
        pub fn with_version(mut self, field: i32) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn canceled_transactions(&self) -> &[CanceledTransaction] {
            &self.canceled_transactions
        }
        pub fn with_canceled_transactions(
            mut self,
            field: Vec<CanceledTransaction>,
        ) -> Self {
            self.canceled_transactions = field;
            self
        }
    }
    impl AuthenticatorStateUpdate {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                round: None,
                new_active_jwks: Vec::new(),
                authenticator_object_initial_shared_version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: AuthenticatorStateUpdate = AuthenticatorStateUpdate::const_default();
            &DEFAULT
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
        pub fn with_round(mut self, field: u64) -> Self {
            self.round = Some(field.into());
            self
        }
        pub fn new_active_jwks(&self) -> &[ActiveJwk] {
            &self.new_active_jwks
        }
        pub fn with_new_active_jwks(mut self, field: Vec<ActiveJwk>) -> Self {
            self.new_active_jwks = field;
            self
        }
        pub fn with_authenticator_object_initial_shared_version(
            mut self,
            field: u64,
        ) -> Self {
            self.authenticator_object_initial_shared_version = Some(field.into());
            self
        }
    }
    impl ActiveJwk {
        pub const fn const_default() -> Self {
            Self {
                id: None,
                jwk: None,
                epoch: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ActiveJwk = ActiveJwk::const_default();
            &DEFAULT
        }
        pub fn id(&self) -> &JwkId {
            self.id
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| JwkId::default_instance() as _)
        }
        pub fn id_opt(&self) -> Option<&JwkId> {
            self.id.as_ref().map(|field| field as _)
        }
        pub fn id_opt_mut(&mut self) -> Option<&mut JwkId> {
            self.id.as_mut().map(|field| field as _)
        }
        pub fn id_mut(&mut self) -> &mut JwkId {
            self.id.get_or_insert_default()
        }
        pub fn with_id(mut self, field: JwkId) -> Self {
            self.id = Some(field.into());
            self
        }
        pub fn jwk(&self) -> &Jwk {
            self.jwk
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Jwk::default_instance() as _)
        }
        pub fn jwk_opt(&self) -> Option<&Jwk> {
            self.jwk.as_ref().map(|field| field as _)
        }
        pub fn jwk_opt_mut(&mut self) -> Option<&mut Jwk> {
            self.jwk.as_mut().map(|field| field as _)
        }
        pub fn jwk_mut(&mut self) -> &mut Jwk {
            self.jwk.get_or_insert_default()
        }
        pub fn with_jwk(mut self, field: Jwk) -> Self {
            self.jwk = Some(field.into());
            self
        }
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.epoch = Some(field.into());
            self
        }
    }
    impl JwkId {
        pub const fn const_default() -> Self {
            Self { iss: None, kid: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: JwkId = JwkId::const_default();
            &DEFAULT
        }
        pub fn with_iss(mut self, field: String) -> Self {
            self.iss = Some(field.into());
            self
        }
        pub fn with_kid(mut self, field: String) -> Self {
            self.kid = Some(field.into());
            self
        }
    }
    impl Jwk {
        pub const fn const_default() -> Self {
            Self {
                kty: None,
                e: None,
                n: None,
                alg: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Jwk = Jwk::const_default();
            &DEFAULT
        }
        pub fn with_kty(mut self, field: String) -> Self {
            self.kty = Some(field.into());
            self
        }
        pub fn with_e(mut self, field: String) -> Self {
            self.e = Some(field.into());
            self
        }
        pub fn with_n(mut self, field: String) -> Self {
            self.n = Some(field.into());
            self
        }
        pub fn with_alg(mut self, field: String) -> Self {
            self.alg = Some(field.into());
            self
        }
    }
    impl EndOfEpochTransaction {
        pub const fn const_default() -> Self {
            Self { transactions: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: EndOfEpochTransaction = EndOfEpochTransaction::const_default();
            &DEFAULT
        }
        pub fn transactions(&self) -> &[EndOfEpochTransactionKind] {
            &self.transactions
        }
        pub fn with_transactions(
            mut self,
            field: Vec<EndOfEpochTransactionKind>,
        ) -> Self {
            self.transactions = field;
            self
        }
    }
    impl EndOfEpochTransactionKind {
        pub const fn const_default() -> Self {
            Self { kind: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: EndOfEpochTransactionKind = EndOfEpochTransactionKind::const_default();
            &DEFAULT
        }
        pub fn change_epoch(&self) -> &ChangeEpoch {
            if let Some(end_of_epoch_transaction_kind::Kind::ChangeEpoch(field)) = &self
                .kind
            {
                field as _
            } else {
                ChangeEpoch::default_instance() as _
            }
        }
        pub fn change_epoch_opt(&self) -> Option<&ChangeEpoch> {
            if let Some(end_of_epoch_transaction_kind::Kind::ChangeEpoch(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn change_epoch_opt_mut(&mut self) -> Option<&mut ChangeEpoch> {
            if let Some(end_of_epoch_transaction_kind::Kind::ChangeEpoch(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn change_epoch_mut(&mut self) -> &mut ChangeEpoch {
            if self.change_epoch_opt_mut().is_none() {
                self.kind = Some(
                    end_of_epoch_transaction_kind::Kind::ChangeEpoch(
                        ChangeEpoch::default(),
                    ),
                );
            }
            self.change_epoch_opt_mut().unwrap()
        }
        pub fn with_change_epoch(mut self, field: ChangeEpoch) -> Self {
            self.kind = Some(
                end_of_epoch_transaction_kind::Kind::ChangeEpoch(field.into()),
            );
            self
        }
        pub fn authenticator_state_expire(&self) -> &AuthenticatorStateExpire {
            if let Some(
                end_of_epoch_transaction_kind::Kind::AuthenticatorStateExpire(field),
            ) = &self.kind
            {
                field as _
            } else {
                AuthenticatorStateExpire::default_instance() as _
            }
        }
        pub fn authenticator_state_expire_opt(
            &self,
        ) -> Option<&AuthenticatorStateExpire> {
            if let Some(
                end_of_epoch_transaction_kind::Kind::AuthenticatorStateExpire(field),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn authenticator_state_expire_opt_mut(
            &mut self,
        ) -> Option<&mut AuthenticatorStateExpire> {
            if let Some(
                end_of_epoch_transaction_kind::Kind::AuthenticatorStateExpire(field),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn authenticator_state_expire_mut(
            &mut self,
        ) -> &mut AuthenticatorStateExpire {
            if self.authenticator_state_expire_opt_mut().is_none() {
                self.kind = Some(
                    end_of_epoch_transaction_kind::Kind::AuthenticatorStateExpire(
                        AuthenticatorStateExpire::default(),
                    ),
                );
            }
            self.authenticator_state_expire_opt_mut().unwrap()
        }
        pub fn with_authenticator_state_expire(
            mut self,
            field: AuthenticatorStateExpire,
        ) -> Self {
            self.kind = Some(
                end_of_epoch_transaction_kind::Kind::AuthenticatorStateExpire(
                    field.into(),
                ),
            );
            self
        }
        pub fn execution_time_observations(&self) -> &ExecutionTimeObservations {
            if let Some(
                end_of_epoch_transaction_kind::Kind::ExecutionTimeObservations(field),
            ) = &self.kind
            {
                field as _
            } else {
                ExecutionTimeObservations::default_instance() as _
            }
        }
        pub fn execution_time_observations_opt(
            &self,
        ) -> Option<&ExecutionTimeObservations> {
            if let Some(
                end_of_epoch_transaction_kind::Kind::ExecutionTimeObservations(field),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn execution_time_observations_opt_mut(
            &mut self,
        ) -> Option<&mut ExecutionTimeObservations> {
            if let Some(
                end_of_epoch_transaction_kind::Kind::ExecutionTimeObservations(field),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn execution_time_observations_mut(
            &mut self,
        ) -> &mut ExecutionTimeObservations {
            if self.execution_time_observations_opt_mut().is_none() {
                self.kind = Some(
                    end_of_epoch_transaction_kind::Kind::ExecutionTimeObservations(
                        ExecutionTimeObservations::default(),
                    ),
                );
            }
            self.execution_time_observations_opt_mut().unwrap()
        }
        pub fn with_execution_time_observations(
            mut self,
            field: ExecutionTimeObservations,
        ) -> Self {
            self.kind = Some(
                end_of_epoch_transaction_kind::Kind::ExecutionTimeObservations(
                    field.into(),
                ),
            );
            self
        }
        pub fn bridge_state_create(&self) -> &str {
            if let Some(end_of_epoch_transaction_kind::Kind::BridgeStateCreate(field)) = &self
                .kind
            {
                field as _
            } else {
                ""
            }
        }
        pub fn bridge_state_create_opt(&self) -> Option<&str> {
            if let Some(end_of_epoch_transaction_kind::Kind::BridgeStateCreate(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn bridge_state_create_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(end_of_epoch_transaction_kind::Kind::BridgeStateCreate(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn bridge_state_create_mut(&mut self) -> &mut String {
            if self.bridge_state_create_opt_mut().is_none() {
                self.kind = Some(
                    end_of_epoch_transaction_kind::Kind::BridgeStateCreate(
                        String::default(),
                    ),
                );
            }
            self.bridge_state_create_opt_mut().unwrap()
        }
        pub fn with_bridge_state_create(mut self, field: String) -> Self {
            self.kind = Some(
                end_of_epoch_transaction_kind::Kind::BridgeStateCreate(field.into()),
            );
            self
        }
        pub fn bridge_committee_init(&self) -> u64 {
            if let Some(
                end_of_epoch_transaction_kind::Kind::BridgeCommitteeInit(field),
            ) = &self.kind
            {
                *field
            } else {
                0u64
            }
        }
        pub fn bridge_committee_init_opt(&self) -> Option<u64> {
            if let Some(
                end_of_epoch_transaction_kind::Kind::BridgeCommitteeInit(field),
            ) = &self.kind
            {
                Some(*field)
            } else {
                None
            }
        }
        pub fn bridge_committee_init_opt_mut(&mut self) -> Option<&mut u64> {
            if let Some(
                end_of_epoch_transaction_kind::Kind::BridgeCommitteeInit(field),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn bridge_committee_init_mut(&mut self) -> &mut u64 {
            if self.bridge_committee_init_opt_mut().is_none() {
                self.kind = Some(
                    end_of_epoch_transaction_kind::Kind::BridgeCommitteeInit(
                        u64::default(),
                    ),
                );
            }
            self.bridge_committee_init_opt_mut().unwrap()
        }
        pub fn with_bridge_committee_init(mut self, field: u64) -> Self {
            self.kind = Some(
                end_of_epoch_transaction_kind::Kind::BridgeCommitteeInit(field.into()),
            );
            self
        }
    }
    impl AuthenticatorStateExpire {
        pub const fn const_default() -> Self {
            Self {
                min_epoch: None,
                authenticator_object_initial_shared_version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: AuthenticatorStateExpire = AuthenticatorStateExpire::const_default();
            &DEFAULT
        }
        pub fn with_min_epoch(mut self, field: u64) -> Self {
            self.min_epoch = Some(field.into());
            self
        }
        pub fn with_authenticator_object_initial_shared_version(
            mut self,
            field: u64,
        ) -> Self {
            self.authenticator_object_initial_shared_version = Some(field.into());
            self
        }
    }
    impl ExecutionTimeObservations {
        pub const fn const_default() -> Self {
            Self {
                version: None,
                observations: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ExecutionTimeObservations = ExecutionTimeObservations::const_default();
            &DEFAULT
        }
        pub fn with_version(mut self, field: i32) -> Self {
            self.version = Some(field.into());
            self
        }
        pub fn observations(&self) -> &[ExecutionTimeObservation] {
            &self.observations
        }
        pub fn with_observations(
            mut self,
            field: Vec<ExecutionTimeObservation>,
        ) -> Self {
            self.observations = field;
            self
        }
    }
    impl ExecutionTimeObservation {
        pub const fn const_default() -> Self {
            Self {
                kind: None,
                move_entry_point: None,
                validator_observations: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ExecutionTimeObservation = ExecutionTimeObservation::const_default();
            &DEFAULT
        }
        pub fn move_entry_point(&self) -> &MoveCall {
            self.move_entry_point
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| MoveCall::default_instance() as _)
        }
        pub fn move_entry_point_opt(&self) -> Option<&MoveCall> {
            self.move_entry_point.as_ref().map(|field| field as _)
        }
        pub fn move_entry_point_opt_mut(&mut self) -> Option<&mut MoveCall> {
            self.move_entry_point.as_mut().map(|field| field as _)
        }
        pub fn move_entry_point_mut(&mut self) -> &mut MoveCall {
            self.move_entry_point.get_or_insert_default()
        }
        pub fn with_move_entry_point(mut self, field: MoveCall) -> Self {
            self.move_entry_point = Some(field.into());
            self
        }
        pub fn validator_observations(&self) -> &[ValidatorExecutionTimeObservation] {
            &self.validator_observations
        }
        pub fn with_validator_observations(
            mut self,
            field: Vec<ValidatorExecutionTimeObservation>,
        ) -> Self {
            self.validator_observations = field;
            self
        }
    }
    impl ValidatorExecutionTimeObservation {
        pub const fn const_default() -> Self {
            Self {
                validator: None,
                duration: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ValidatorExecutionTimeObservation = ValidatorExecutionTimeObservation::const_default();
            &DEFAULT
        }
        pub fn with_validator(mut self, field: ::prost::bytes::Bytes) -> Self {
            self.validator = Some(field.into());
            self
        }
    }
    impl ExecuteTransactionRequest {
        pub const fn const_default() -> Self {
            Self {
                transaction: None,
                signatures: Vec::new(),
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ExecuteTransactionRequest = ExecuteTransactionRequest::const_default();
            &DEFAULT
        }
        pub fn transaction(&self) -> &Transaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| Transaction::default_instance() as _)
        }
        pub fn transaction_opt(&self) -> Option<&Transaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn transaction_opt_mut(&mut self) -> Option<&mut Transaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut Transaction {
            self.transaction.get_or_insert_default()
        }
        pub fn with_transaction(mut self, field: Transaction) -> Self {
            self.transaction = Some(field.into());
            self
        }
        pub fn signatures(&self) -> &[UserSignature] {
            &self.signatures
        }
        pub fn with_signatures(mut self, field: Vec<UserSignature>) -> Self {
            self.signatures = field;
            self
        }
    }
    impl ExecuteTransactionResponse {
        pub const fn const_default() -> Self {
            Self {
                finality: None,
                transaction: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: ExecuteTransactionResponse = ExecuteTransactionResponse::const_default();
            &DEFAULT
        }
        pub fn finality(&self) -> &TransactionFinality {
            self.finality
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| TransactionFinality::default_instance() as _)
        }
        pub fn finality_opt(&self) -> Option<&TransactionFinality> {
            self.finality.as_ref().map(|field| field as _)
        }
        pub fn finality_opt_mut(&mut self) -> Option<&mut TransactionFinality> {
            self.finality.as_mut().map(|field| field as _)
        }
        pub fn finality_mut(&mut self) -> &mut TransactionFinality {
            self.finality.get_or_insert_default()
        }
        pub fn with_finality(mut self, field: TransactionFinality) -> Self {
            self.finality = Some(field.into());
            self
        }
        pub fn transaction(&self) -> &ExecutedTransaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| ExecutedTransaction::default_instance() as _)
        }
        pub fn transaction_opt(&self) -> Option<&ExecutedTransaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn transaction_opt_mut(&mut self) -> Option<&mut ExecutedTransaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut ExecutedTransaction {
            self.transaction.get_or_insert_default()
        }
        pub fn with_transaction(mut self, field: ExecutedTransaction) -> Self {
            self.transaction = Some(field.into());
            self
        }
    }
    impl TransactionFinality {
        pub const fn const_default() -> Self {
            Self { finality: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: TransactionFinality = TransactionFinality::const_default();
            &DEFAULT
        }
        pub fn certified(&self) -> &ValidatorAggregatedSignature {
            if let Some(transaction_finality::Finality::Certified(field)) = &self
                .finality
            {
                field as _
            } else {
                ValidatorAggregatedSignature::default_instance() as _
            }
        }
        pub fn certified_opt(&self) -> Option<&ValidatorAggregatedSignature> {
            if let Some(transaction_finality::Finality::Certified(field)) = &self
                .finality
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn certified_opt_mut(
            &mut self,
        ) -> Option<&mut ValidatorAggregatedSignature> {
            if let Some(transaction_finality::Finality::Certified(field)) = &mut self
                .finality
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn certified_mut(&mut self) -> &mut ValidatorAggregatedSignature {
            if self.certified_opt_mut().is_none() {
                self.finality = Some(
                    transaction_finality::Finality::Certified(
                        ValidatorAggregatedSignature::default(),
                    ),
                );
            }
            self.certified_opt_mut().unwrap()
        }
        pub fn with_certified(mut self, field: ValidatorAggregatedSignature) -> Self {
            self.finality = Some(
                transaction_finality::Finality::Certified(field.into()),
            );
            self
        }
        pub fn checkpointed(&self) -> u64 {
            if let Some(transaction_finality::Finality::Checkpointed(field)) = &self
                .finality
            {
                *field
            } else {
                0u64
            }
        }
        pub fn checkpointed_opt(&self) -> Option<u64> {
            if let Some(transaction_finality::Finality::Checkpointed(field)) = &self
                .finality
            {
                Some(*field)
            } else {
                None
            }
        }
        pub fn checkpointed_opt_mut(&mut self) -> Option<&mut u64> {
            if let Some(transaction_finality::Finality::Checkpointed(field)) = &mut self
                .finality
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn checkpointed_mut(&mut self) -> &mut u64 {
            if self.checkpointed_opt_mut().is_none() {
                self.finality = Some(
                    transaction_finality::Finality::Checkpointed(u64::default()),
                );
            }
            self.checkpointed_opt_mut().unwrap()
        }
        pub fn with_checkpointed(mut self, field: u64) -> Self {
            self.finality = Some(
                transaction_finality::Finality::Checkpointed(field.into()),
            );
            self
        }
    }
}
