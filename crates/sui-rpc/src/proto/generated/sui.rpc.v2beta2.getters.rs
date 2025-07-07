mod _getter_impls {
    use super::*;
    impl Argument {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Argument> = std::sync::LazyLock::new(
                Argument::default,
            );
            &DEFAULT
        }
    }
    impl BalanceChange {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<BalanceChange> = std::sync::LazyLock::new(
                BalanceChange::default,
            );
            &DEFAULT
        }
    }
    impl Bcs {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Bcs> = std::sync::LazyLock::new(
                Bcs::default,
            );
            &DEFAULT
        }
    }
    impl Checkpoint {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Checkpoint> = std::sync::LazyLock::new(
                Checkpoint::default,
            );
            &DEFAULT
        }
        pub fn summary(&self) -> &CheckpointSummary {
            if let Some(r) = &self.summary {
                r as _
            } else {
                CheckpointSummary::default_ref()
            }
        }
        pub fn signature(&self) -> &ValidatorAggregatedSignature {
            if let Some(r) = &self.signature {
                r as _
            } else {
                ValidatorAggregatedSignature::default_ref()
            }
        }
        pub fn contents(&self) -> &CheckpointContents {
            if let Some(r) = &self.contents {
                r as _
            } else {
                CheckpointContents::default_ref()
            }
        }
        pub fn transactions(&self) -> &[ExecutedTransaction] {
            &self.transactions
        }
    }
    impl CheckpointContents {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CheckpointContents> = std::sync::LazyLock::new(
                CheckpointContents::default,
            );
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            if let Some(r) = &self.bcs { r as _ } else { Bcs::default_ref() }
        }
        pub fn transactions(&self) -> &[CheckpointedTransactionInfo] {
            &self.transactions
        }
    }
    impl CheckpointedTransactionInfo {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CheckpointedTransactionInfo> = std::sync::LazyLock::new(
                CheckpointedTransactionInfo::default,
            );
            &DEFAULT
        }
        pub fn signatures(&self) -> &[UserSignature] {
            &self.signatures
        }
    }
    impl CheckpointSummary {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CheckpointSummary> = std::sync::LazyLock::new(
                CheckpointSummary::default,
            );
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            if let Some(r) = &self.bcs { r as _ } else { Bcs::default_ref() }
        }
        pub fn epoch_rolling_gas_cost_summary(&self) -> &GasCostSummary {
            if let Some(r) = &self.epoch_rolling_gas_cost_summary {
                r as _
            } else {
                GasCostSummary::default_ref()
            }
        }
        pub fn commitments(&self) -> &[CheckpointCommitment] {
            &self.commitments
        }
        pub fn end_of_epoch_data(&self) -> &EndOfEpochData {
            if let Some(r) = &self.end_of_epoch_data {
                r as _
            } else {
                EndOfEpochData::default_ref()
            }
        }
    }
    impl EndOfEpochData {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<EndOfEpochData> = std::sync::LazyLock::new(
                EndOfEpochData::default,
            );
            &DEFAULT
        }
        pub fn next_epoch_committee(&self) -> &[ValidatorCommitteeMember] {
            &self.next_epoch_committee
        }
        pub fn epoch_commitments(&self) -> &[CheckpointCommitment] {
            &self.epoch_commitments
        }
    }
    impl CheckpointCommitment {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CheckpointCommitment> = std::sync::LazyLock::new(
                CheckpointCommitment::default,
            );
            &DEFAULT
        }
    }
    impl TransactionEffects {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<TransactionEffects> = std::sync::LazyLock::new(
                TransactionEffects::default,
            );
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            if let Some(r) = &self.bcs { r as _ } else { Bcs::default_ref() }
        }
        pub fn status(&self) -> &ExecutionStatus {
            if let Some(r) = &self.status {
                r as _
            } else {
                ExecutionStatus::default_ref()
            }
        }
        pub fn gas_used(&self) -> &GasCostSummary {
            if let Some(r) = &self.gas_used {
                r as _
            } else {
                GasCostSummary::default_ref()
            }
        }
        pub fn gas_object(&self) -> &ChangedObject {
            if let Some(r) = &self.gas_object {
                r as _
            } else {
                ChangedObject::default_ref()
            }
        }
        pub fn changed_objects(&self) -> &[ChangedObject] {
            &self.changed_objects
        }
        pub fn unchanged_shared_objects(&self) -> &[UnchangedSharedObject] {
            &self.unchanged_shared_objects
        }
    }
    impl ChangedObject {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ChangedObject> = std::sync::LazyLock::new(
                ChangedObject::default,
            );
            &DEFAULT
        }
        pub fn input_owner(&self) -> &Owner {
            if let Some(r) = &self.input_owner { r as _ } else { Owner::default_ref() }
        }
        pub fn output_owner(&self) -> &Owner {
            if let Some(r) = &self.output_owner { r as _ } else { Owner::default_ref() }
        }
    }
    impl UnchangedSharedObject {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<UnchangedSharedObject> = std::sync::LazyLock::new(
                UnchangedSharedObject::default,
            );
            &DEFAULT
        }
    }
    impl Epoch {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Epoch> = std::sync::LazyLock::new(
                Epoch::default,
            );
            &DEFAULT
        }
        pub fn committee(&self) -> &ValidatorCommittee {
            if let Some(r) = &self.committee {
                r as _
            } else {
                ValidatorCommittee::default_ref()
            }
        }
        pub fn system_state(&self) -> &SystemState {
            if let Some(r) = &self.system_state {
                r as _
            } else {
                SystemState::default_ref()
            }
        }
        pub fn protocol_config(&self) -> &ProtocolConfig {
            if let Some(r) = &self.protocol_config {
                r as _
            } else {
                ProtocolConfig::default_ref()
            }
        }
    }
    impl TransactionEvents {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<TransactionEvents> = std::sync::LazyLock::new(
                TransactionEvents::default,
            );
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            if let Some(r) = &self.bcs { r as _ } else { Bcs::default_ref() }
        }
        pub fn events(&self) -> &[Event] {
            &self.events
        }
    }
    impl Event {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Event> = std::sync::LazyLock::new(
                Event::default,
            );
            &DEFAULT
        }
        pub fn contents(&self) -> &Bcs {
            if let Some(r) = &self.contents { r as _ } else { Bcs::default_ref() }
        }
    }
    impl ExecutedTransaction {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ExecutedTransaction> = std::sync::LazyLock::new(
                ExecutedTransaction::default,
            );
            &DEFAULT
        }
        pub fn transaction(&self) -> &Transaction {
            if let Some(r) = &self.transaction {
                r as _
            } else {
                Transaction::default_ref()
            }
        }
        pub fn signatures(&self) -> &[UserSignature] {
            &self.signatures
        }
        pub fn effects(&self) -> &TransactionEffects {
            if let Some(r) = &self.effects {
                r as _
            } else {
                TransactionEffects::default_ref()
            }
        }
        pub fn events(&self) -> &TransactionEvents {
            if let Some(r) = &self.events {
                r as _
            } else {
                TransactionEvents::default_ref()
            }
        }
        pub fn balance_changes(&self) -> &[BalanceChange] {
            &self.balance_changes
        }
        pub fn input_objects(&self) -> &[Object] {
            &self.input_objects
        }
        pub fn output_objects(&self) -> &[Object] {
            &self.output_objects
        }
    }
    impl ExecutionStatus {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ExecutionStatus> = std::sync::LazyLock::new(
                ExecutionStatus::default,
            );
            &DEFAULT
        }
        pub fn error(&self) -> &ExecutionError {
            if let Some(r) = &self.error {
                r as _
            } else {
                ExecutionError::default_ref()
            }
        }
    }
    impl ExecutionError {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ExecutionError> = std::sync::LazyLock::new(
                ExecutionError::default,
            );
            &DEFAULT
        }
    }
    impl MoveAbort {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<MoveAbort> = std::sync::LazyLock::new(
                MoveAbort::default,
            );
            &DEFAULT
        }
        pub fn location(&self) -> &MoveLocation {
            if let Some(r) = &self.location {
                r as _
            } else {
                MoveLocation::default_ref()
            }
        }
        pub fn clever_error(&self) -> &CleverError {
            if let Some(r) = &self.clever_error {
                r as _
            } else {
                CleverError::default_ref()
            }
        }
    }
    impl MoveLocation {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<MoveLocation> = std::sync::LazyLock::new(
                MoveLocation::default,
            );
            &DEFAULT
        }
    }
    impl CleverError {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CleverError> = std::sync::LazyLock::new(
                CleverError::default,
            );
            &DEFAULT
        }
    }
    impl SizeError {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<SizeError> = std::sync::LazyLock::new(
                SizeError::default,
            );
            &DEFAULT
        }
    }
    impl IndexError {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<IndexError> = std::sync::LazyLock::new(
                IndexError::default,
            );
            &DEFAULT
        }
    }
    impl CoinDenyListError {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CoinDenyListError> = std::sync::LazyLock::new(
                CoinDenyListError::default,
            );
            &DEFAULT
        }
    }
    impl CongestedObjects {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CongestedObjects> = std::sync::LazyLock::new(
                CongestedObjects::default,
            );
            &DEFAULT
        }
    }
    impl CommandArgumentError {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CommandArgumentError> = std::sync::LazyLock::new(
                CommandArgumentError::default,
            );
            &DEFAULT
        }
        pub fn index_error(&self) -> &IndexError {
            if let Some(r) = &self.index_error {
                r as _
            } else {
                IndexError::default_ref()
            }
        }
    }
    impl PackageUpgradeError {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<PackageUpgradeError> = std::sync::LazyLock::new(
                PackageUpgradeError::default,
            );
            &DEFAULT
        }
    }
    impl TypeArgumentError {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<TypeArgumentError> = std::sync::LazyLock::new(
                TypeArgumentError::default,
            );
            &DEFAULT
        }
    }
    impl GasCostSummary {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GasCostSummary> = std::sync::LazyLock::new(
                GasCostSummary::default,
            );
            &DEFAULT
        }
    }
    impl Input {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Input> = std::sync::LazyLock::new(
                Input::default,
            );
            &DEFAULT
        }
    }
    impl GetServiceInfoRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetServiceInfoRequest> = std::sync::LazyLock::new(
                GetServiceInfoRequest::default,
            );
            &DEFAULT
        }
    }
    impl GetServiceInfoResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetServiceInfoResponse> = std::sync::LazyLock::new(
                GetServiceInfoResponse::default,
            );
            &DEFAULT
        }
    }
    impl GetObjectRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetObjectRequest> = std::sync::LazyLock::new(
                GetObjectRequest::default,
            );
            &DEFAULT
        }
    }
    impl GetObjectResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetObjectResponse> = std::sync::LazyLock::new(
                GetObjectResponse::default,
            );
            &DEFAULT
        }
        pub fn object(&self) -> &Object {
            if let Some(r) = &self.object { r as _ } else { Object::default_ref() }
        }
    }
    impl BatchGetObjectsRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<BatchGetObjectsRequest> = std::sync::LazyLock::new(
                BatchGetObjectsRequest::default,
            );
            &DEFAULT
        }
        pub fn requests(&self) -> &[GetObjectRequest] {
            &self.requests
        }
    }
    impl BatchGetObjectsResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<BatchGetObjectsResponse> = std::sync::LazyLock::new(
                BatchGetObjectsResponse::default,
            );
            &DEFAULT
        }
        pub fn objects(&self) -> &[GetObjectResult] {
            &self.objects
        }
    }
    impl GetObjectResult {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetObjectResult> = std::sync::LazyLock::new(
                GetObjectResult::default,
            );
            &DEFAULT
        }
    }
    impl GetTransactionRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetTransactionRequest> = std::sync::LazyLock::new(
                GetTransactionRequest::default,
            );
            &DEFAULT
        }
    }
    impl GetTransactionResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetTransactionResponse> = std::sync::LazyLock::new(
                GetTransactionResponse::default,
            );
            &DEFAULT
        }
        pub fn transaction(&self) -> &ExecutedTransaction {
            if let Some(r) = &self.transaction {
                r as _
            } else {
                ExecutedTransaction::default_ref()
            }
        }
    }
    impl BatchGetTransactionsRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<BatchGetTransactionsRequest> = std::sync::LazyLock::new(
                BatchGetTransactionsRequest::default,
            );
            &DEFAULT
        }
    }
    impl BatchGetTransactionsResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<BatchGetTransactionsResponse> = std::sync::LazyLock::new(
                BatchGetTransactionsResponse::default,
            );
            &DEFAULT
        }
        pub fn transactions(&self) -> &[GetTransactionResult] {
            &self.transactions
        }
    }
    impl GetTransactionResult {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetTransactionResult> = std::sync::LazyLock::new(
                GetTransactionResult::default,
            );
            &DEFAULT
        }
    }
    impl GetCheckpointRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetCheckpointRequest> = std::sync::LazyLock::new(
                GetCheckpointRequest::default,
            );
            &DEFAULT
        }
    }
    impl GetCheckpointResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetCheckpointResponse> = std::sync::LazyLock::new(
                GetCheckpointResponse::default,
            );
            &DEFAULT
        }
        pub fn checkpoint(&self) -> &Checkpoint {
            if let Some(r) = &self.checkpoint {
                r as _
            } else {
                Checkpoint::default_ref()
            }
        }
    }
    impl GetEpochRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetEpochRequest> = std::sync::LazyLock::new(
                GetEpochRequest::default,
            );
            &DEFAULT
        }
    }
    impl GetEpochResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetEpochResponse> = std::sync::LazyLock::new(
                GetEpochResponse::default,
            );
            &DEFAULT
        }
        pub fn epoch(&self) -> &Epoch {
            if let Some(r) = &self.epoch { r as _ } else { Epoch::default_ref() }
        }
    }
    impl GetCoinInfoRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetCoinInfoRequest> = std::sync::LazyLock::new(
                GetCoinInfoRequest::default,
            );
            &DEFAULT
        }
    }
    impl GetCoinInfoResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetCoinInfoResponse> = std::sync::LazyLock::new(
                GetCoinInfoResponse::default,
            );
            &DEFAULT
        }
        pub fn metadata(&self) -> &CoinMetadata {
            if let Some(r) = &self.metadata {
                r as _
            } else {
                CoinMetadata::default_ref()
            }
        }
        pub fn treasury(&self) -> &CoinTreasury {
            if let Some(r) = &self.treasury {
                r as _
            } else {
                CoinTreasury::default_ref()
            }
        }
        pub fn regulated_metadata(&self) -> &RegulatedCoinMetadata {
            if let Some(r) = &self.regulated_metadata {
                r as _
            } else {
                RegulatedCoinMetadata::default_ref()
            }
        }
    }
    impl CoinMetadata {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CoinMetadata> = std::sync::LazyLock::new(
                CoinMetadata::default,
            );
            &DEFAULT
        }
    }
    impl CoinTreasury {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CoinTreasury> = std::sync::LazyLock::new(
                CoinTreasury::default,
            );
            &DEFAULT
        }
    }
    impl RegulatedCoinMetadata {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<RegulatedCoinMetadata> = std::sync::LazyLock::new(
                RegulatedCoinMetadata::default,
            );
            &DEFAULT
        }
    }
    impl GetBalanceRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetBalanceRequest> = std::sync::LazyLock::new(
                GetBalanceRequest::default,
            );
            &DEFAULT
        }
    }
    impl GetBalanceResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetBalanceResponse> = std::sync::LazyLock::new(
                GetBalanceResponse::default,
            );
            &DEFAULT
        }
        pub fn balance(&self) -> &Balance {
            if let Some(r) = &self.balance { r as _ } else { Balance::default_ref() }
        }
    }
    impl ListBalancesRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ListBalancesRequest> = std::sync::LazyLock::new(
                ListBalancesRequest::default,
            );
            &DEFAULT
        }
    }
    impl ListBalancesResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ListBalancesResponse> = std::sync::LazyLock::new(
                ListBalancesResponse::default,
            );
            &DEFAULT
        }
        pub fn balances(&self) -> &[Balance] {
            &self.balances
        }
    }
    impl Balance {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Balance> = std::sync::LazyLock::new(
                Balance::default,
            );
            &DEFAULT
        }
    }
    impl ListDynamicFieldsRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ListDynamicFieldsRequest> = std::sync::LazyLock::new(
                ListDynamicFieldsRequest::default,
            );
            &DEFAULT
        }
    }
    impl ListDynamicFieldsResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ListDynamicFieldsResponse> = std::sync::LazyLock::new(
                ListDynamicFieldsResponse::default,
            );
            &DEFAULT
        }
        pub fn dynamic_fields(&self) -> &[DynamicField] {
            &self.dynamic_fields
        }
    }
    impl DynamicField {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<DynamicField> = std::sync::LazyLock::new(
                DynamicField::default,
            );
            &DEFAULT
        }
        pub fn object(&self) -> &Object {
            if let Some(r) = &self.object { r as _ } else { Object::default_ref() }
        }
    }
    impl SimulateTransactionRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<SimulateTransactionRequest> = std::sync::LazyLock::new(
                SimulateTransactionRequest::default,
            );
            &DEFAULT
        }
        pub fn transaction(&self) -> &Transaction {
            if let Some(r) = &self.transaction {
                r as _
            } else {
                Transaction::default_ref()
            }
        }
    }
    impl SimulateTransactionResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<SimulateTransactionResponse> = std::sync::LazyLock::new(
                SimulateTransactionResponse::default,
            );
            &DEFAULT
        }
        pub fn transaction(&self) -> &ExecutedTransaction {
            if let Some(r) = &self.transaction {
                r as _
            } else {
                ExecutedTransaction::default_ref()
            }
        }
        pub fn outputs(&self) -> &[CommandResult] {
            &self.outputs
        }
    }
    impl CommandResult {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CommandResult> = std::sync::LazyLock::new(
                CommandResult::default,
            );
            &DEFAULT
        }
        pub fn return_values(&self) -> &[CommandOutput] {
            &self.return_values
        }
        pub fn mutated_by_ref(&self) -> &[CommandOutput] {
            &self.mutated_by_ref
        }
    }
    impl CommandOutput {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CommandOutput> = std::sync::LazyLock::new(
                CommandOutput::default,
            );
            &DEFAULT
        }
        pub fn argument(&self) -> &Argument {
            if let Some(r) = &self.argument { r as _ } else { Argument::default_ref() }
        }
        pub fn value(&self) -> &Bcs {
            if let Some(r) = &self.value { r as _ } else { Bcs::default_ref() }
        }
    }
    impl ListOwnedObjectsRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ListOwnedObjectsRequest> = std::sync::LazyLock::new(
                ListOwnedObjectsRequest::default,
            );
            &DEFAULT
        }
    }
    impl ListOwnedObjectsResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ListOwnedObjectsResponse> = std::sync::LazyLock::new(
                ListOwnedObjectsResponse::default,
            );
            &DEFAULT
        }
        pub fn objects(&self) -> &[Object] {
            &self.objects
        }
    }
    impl Package {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Package> = std::sync::LazyLock::new(
                Package::default,
            );
            &DEFAULT
        }
        pub fn modules(&self) -> &[Module] {
            &self.modules
        }
        pub fn type_origins(&self) -> &[TypeOrigin] {
            &self.type_origins
        }
        pub fn linkage(&self) -> &[Linkage] {
            &self.linkage
        }
    }
    impl Module {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Module> = std::sync::LazyLock::new(
                Module::default,
            );
            &DEFAULT
        }
        pub fn datatypes(&self) -> &[DatatypeDescriptor] {
            &self.datatypes
        }
        pub fn functions(&self) -> &[FunctionDescriptor] {
            &self.functions
        }
    }
    impl DatatypeDescriptor {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<DatatypeDescriptor> = std::sync::LazyLock::new(
                DatatypeDescriptor::default,
            );
            &DEFAULT
        }
        pub fn type_parameters(&self) -> &[TypeParameter] {
            &self.type_parameters
        }
        pub fn fields(&self) -> &[FieldDescriptor] {
            &self.fields
        }
        pub fn variants(&self) -> &[VariantDescriptor] {
            &self.variants
        }
    }
    impl TypeParameter {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<TypeParameter> = std::sync::LazyLock::new(
                TypeParameter::default,
            );
            &DEFAULT
        }
    }
    impl FieldDescriptor {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<FieldDescriptor> = std::sync::LazyLock::new(
                FieldDescriptor::default,
            );
            &DEFAULT
        }
        pub fn r#type(&self) -> &OpenSignatureBody {
            if let Some(r) = &self.r#type {
                r as _
            } else {
                OpenSignatureBody::default_ref()
            }
        }
    }
    impl VariantDescriptor {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<VariantDescriptor> = std::sync::LazyLock::new(
                VariantDescriptor::default,
            );
            &DEFAULT
        }
        pub fn fields(&self) -> &[FieldDescriptor] {
            &self.fields
        }
    }
    impl OpenSignatureBody {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<OpenSignatureBody> = std::sync::LazyLock::new(
                OpenSignatureBody::default,
            );
            &DEFAULT
        }
        pub fn type_parameter_instantiation(&self) -> &[OpenSignatureBody] {
            &self.type_parameter_instantiation
        }
    }
    impl FunctionDescriptor {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<FunctionDescriptor> = std::sync::LazyLock::new(
                FunctionDescriptor::default,
            );
            &DEFAULT
        }
        pub fn type_parameters(&self) -> &[TypeParameter] {
            &self.type_parameters
        }
        pub fn parameters(&self) -> &[OpenSignature] {
            &self.parameters
        }
        pub fn returns(&self) -> &[OpenSignature] {
            &self.returns
        }
    }
    impl OpenSignature {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<OpenSignature> = std::sync::LazyLock::new(
                OpenSignature::default,
            );
            &DEFAULT
        }
        pub fn body(&self) -> &OpenSignatureBody {
            if let Some(r) = &self.body {
                r as _
            } else {
                OpenSignatureBody::default_ref()
            }
        }
    }
    impl TypeOrigin {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<TypeOrigin> = std::sync::LazyLock::new(
                TypeOrigin::default,
            );
            &DEFAULT
        }
    }
    impl Linkage {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Linkage> = std::sync::LazyLock::new(
                Linkage::default,
            );
            &DEFAULT
        }
    }
    impl GetPackageRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetPackageRequest> = std::sync::LazyLock::new(
                GetPackageRequest::default,
            );
            &DEFAULT
        }
    }
    impl GetPackageResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetPackageResponse> = std::sync::LazyLock::new(
                GetPackageResponse::default,
            );
            &DEFAULT
        }
        pub fn package(&self) -> &Package {
            if let Some(r) = &self.package { r as _ } else { Package::default_ref() }
        }
    }
    impl GetDatatypeRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetDatatypeRequest> = std::sync::LazyLock::new(
                GetDatatypeRequest::default,
            );
            &DEFAULT
        }
    }
    impl GetDatatypeResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetDatatypeResponse> = std::sync::LazyLock::new(
                GetDatatypeResponse::default,
            );
            &DEFAULT
        }
        pub fn datatype(&self) -> &DatatypeDescriptor {
            if let Some(r) = &self.datatype {
                r as _
            } else {
                DatatypeDescriptor::default_ref()
            }
        }
    }
    impl GetFunctionRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetFunctionRequest> = std::sync::LazyLock::new(
                GetFunctionRequest::default,
            );
            &DEFAULT
        }
    }
    impl GetFunctionResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GetFunctionResponse> = std::sync::LazyLock::new(
                GetFunctionResponse::default,
            );
            &DEFAULT
        }
        pub fn function(&self) -> &FunctionDescriptor {
            if let Some(r) = &self.function {
                r as _
            } else {
                FunctionDescriptor::default_ref()
            }
        }
    }
    impl ListPackageVersionsRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ListPackageVersionsRequest> = std::sync::LazyLock::new(
                ListPackageVersionsRequest::default,
            );
            &DEFAULT
        }
    }
    impl ListPackageVersionsResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ListPackageVersionsResponse> = std::sync::LazyLock::new(
                ListPackageVersionsResponse::default,
            );
            &DEFAULT
        }
        pub fn versions(&self) -> &[PackageVersion] {
            &self.versions
        }
    }
    impl PackageVersion {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<PackageVersion> = std::sync::LazyLock::new(
                PackageVersion::default,
            );
            &DEFAULT
        }
    }
    impl Object {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Object> = std::sync::LazyLock::new(
                Object::default,
            );
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            if let Some(r) = &self.bcs { r as _ } else { Bcs::default_ref() }
        }
        pub fn owner(&self) -> &Owner {
            if let Some(r) = &self.owner { r as _ } else { Owner::default_ref() }
        }
        pub fn contents(&self) -> &Bcs {
            if let Some(r) = &self.contents { r as _ } else { Bcs::default_ref() }
        }
        pub fn package(&self) -> &Package {
            if let Some(r) = &self.package { r as _ } else { Package::default_ref() }
        }
    }
    impl ObjectReference {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ObjectReference> = std::sync::LazyLock::new(
                ObjectReference::default,
            );
            &DEFAULT
        }
    }
    impl Owner {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Owner> = std::sync::LazyLock::new(
                Owner::default,
            );
            &DEFAULT
        }
    }
    impl ProtocolConfig {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ProtocolConfig> = std::sync::LazyLock::new(
                ProtocolConfig::default,
            );
            &DEFAULT
        }
    }
    impl UserSignature {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<UserSignature> = std::sync::LazyLock::new(
                UserSignature::default,
            );
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            if let Some(r) = &self.bcs { r as _ } else { Bcs::default_ref() }
        }
    }
    impl SimpleSignature {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<SimpleSignature> = std::sync::LazyLock::new(
                SimpleSignature::default,
            );
            &DEFAULT
        }
    }
    impl ZkLoginPublicIdentifier {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ZkLoginPublicIdentifier> = std::sync::LazyLock::new(
                ZkLoginPublicIdentifier::default,
            );
            &DEFAULT
        }
    }
    impl MultisigMemberPublicKey {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<MultisigMemberPublicKey> = std::sync::LazyLock::new(
                MultisigMemberPublicKey::default,
            );
            &DEFAULT
        }
        pub fn zklogin(&self) -> &ZkLoginPublicIdentifier {
            if let Some(r) = &self.zklogin {
                r as _
            } else {
                ZkLoginPublicIdentifier::default_ref()
            }
        }
    }
    impl MultisigMember {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<MultisigMember> = std::sync::LazyLock::new(
                MultisigMember::default,
            );
            &DEFAULT
        }
        pub fn public_key(&self) -> &MultisigMemberPublicKey {
            if let Some(r) = &self.public_key {
                r as _
            } else {
                MultisigMemberPublicKey::default_ref()
            }
        }
    }
    impl MultisigCommittee {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<MultisigCommittee> = std::sync::LazyLock::new(
                MultisigCommittee::default,
            );
            &DEFAULT
        }
        pub fn members(&self) -> &[MultisigMember] {
            &self.members
        }
    }
    impl MultisigAggregatedSignature {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<MultisigAggregatedSignature> = std::sync::LazyLock::new(
                MultisigAggregatedSignature::default,
            );
            &DEFAULT
        }
        pub fn signatures(&self) -> &[MultisigMemberSignature] {
            &self.signatures
        }
        pub fn committee(&self) -> &MultisigCommittee {
            if let Some(r) = &self.committee {
                r as _
            } else {
                MultisigCommittee::default_ref()
            }
        }
    }
    impl MultisigMemberSignature {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<MultisigMemberSignature> = std::sync::LazyLock::new(
                MultisigMemberSignature::default,
            );
            &DEFAULT
        }
        pub fn zklogin(&self) -> &ZkLoginAuthenticator {
            if let Some(r) = &self.zklogin {
                r as _
            } else {
                ZkLoginAuthenticator::default_ref()
            }
        }
        pub fn passkey(&self) -> &PasskeyAuthenticator {
            if let Some(r) = &self.passkey {
                r as _
            } else {
                PasskeyAuthenticator::default_ref()
            }
        }
    }
    impl ZkLoginAuthenticator {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ZkLoginAuthenticator> = std::sync::LazyLock::new(
                ZkLoginAuthenticator::default,
            );
            &DEFAULT
        }
        pub fn inputs(&self) -> &ZkLoginInputs {
            if let Some(r) = &self.inputs {
                r as _
            } else {
                ZkLoginInputs::default_ref()
            }
        }
        pub fn signature(&self) -> &SimpleSignature {
            if let Some(r) = &self.signature {
                r as _
            } else {
                SimpleSignature::default_ref()
            }
        }
    }
    impl ZkLoginInputs {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ZkLoginInputs> = std::sync::LazyLock::new(
                ZkLoginInputs::default,
            );
            &DEFAULT
        }
        pub fn proof_points(&self) -> &ZkLoginProof {
            if let Some(r) = &self.proof_points {
                r as _
            } else {
                ZkLoginProof::default_ref()
            }
        }
        pub fn iss_base64_details(&self) -> &ZkLoginClaim {
            if let Some(r) = &self.iss_base64_details {
                r as _
            } else {
                ZkLoginClaim::default_ref()
            }
        }
    }
    impl ZkLoginProof {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ZkLoginProof> = std::sync::LazyLock::new(
                ZkLoginProof::default,
            );
            &DEFAULT
        }
        pub fn a(&self) -> &CircomG1 {
            if let Some(r) = &self.a { r as _ } else { CircomG1::default_ref() }
        }
        pub fn b(&self) -> &CircomG2 {
            if let Some(r) = &self.b { r as _ } else { CircomG2::default_ref() }
        }
        pub fn c(&self) -> &CircomG1 {
            if let Some(r) = &self.c { r as _ } else { CircomG1::default_ref() }
        }
    }
    impl ZkLoginClaim {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ZkLoginClaim> = std::sync::LazyLock::new(
                ZkLoginClaim::default,
            );
            &DEFAULT
        }
    }
    impl CircomG1 {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CircomG1> = std::sync::LazyLock::new(
                CircomG1::default,
            );
            &DEFAULT
        }
    }
    impl CircomG2 {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CircomG2> = std::sync::LazyLock::new(
                CircomG2::default,
            );
            &DEFAULT
        }
    }
    impl PasskeyAuthenticator {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<PasskeyAuthenticator> = std::sync::LazyLock::new(
                PasskeyAuthenticator::default,
            );
            &DEFAULT
        }
        pub fn signature(&self) -> &SimpleSignature {
            if let Some(r) = &self.signature {
                r as _
            } else {
                SimpleSignature::default_ref()
            }
        }
    }
    impl ValidatorCommittee {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ValidatorCommittee> = std::sync::LazyLock::new(
                ValidatorCommittee::default,
            );
            &DEFAULT
        }
        pub fn members(&self) -> &[ValidatorCommitteeMember] {
            &self.members
        }
    }
    impl ValidatorCommitteeMember {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ValidatorCommitteeMember> = std::sync::LazyLock::new(
                ValidatorCommitteeMember::default,
            );
            &DEFAULT
        }
    }
    impl ValidatorAggregatedSignature {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ValidatorAggregatedSignature> = std::sync::LazyLock::new(
                ValidatorAggregatedSignature::default,
            );
            &DEFAULT
        }
    }
    impl VerifySignatureRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<VerifySignatureRequest> = std::sync::LazyLock::new(
                VerifySignatureRequest::default,
            );
            &DEFAULT
        }
        pub fn message(&self) -> &Bcs {
            if let Some(r) = &self.message { r as _ } else { Bcs::default_ref() }
        }
        pub fn signature(&self) -> &UserSignature {
            if let Some(r) = &self.signature {
                r as _
            } else {
                UserSignature::default_ref()
            }
        }
        pub fn jwks(&self) -> &[ActiveJwk] {
            &self.jwks
        }
    }
    impl VerifySignatureResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<VerifySignatureResponse> = std::sync::LazyLock::new(
                VerifySignatureResponse::default,
            );
            &DEFAULT
        }
    }
    impl SubscribeCheckpointsRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<SubscribeCheckpointsRequest> = std::sync::LazyLock::new(
                SubscribeCheckpointsRequest::default,
            );
            &DEFAULT
        }
    }
    impl SubscribeCheckpointsResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<SubscribeCheckpointsResponse> = std::sync::LazyLock::new(
                SubscribeCheckpointsResponse::default,
            );
            &DEFAULT
        }
        pub fn checkpoint(&self) -> &Checkpoint {
            if let Some(r) = &self.checkpoint {
                r as _
            } else {
                Checkpoint::default_ref()
            }
        }
    }
    impl SystemState {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<SystemState> = std::sync::LazyLock::new(
                SystemState::default,
            );
            &DEFAULT
        }
        pub fn validators(&self) -> &ValidatorSet {
            if let Some(r) = &self.validators {
                r as _
            } else {
                ValidatorSet::default_ref()
            }
        }
        pub fn storage_fund(&self) -> &StorageFund {
            if let Some(r) = &self.storage_fund {
                r as _
            } else {
                StorageFund::default_ref()
            }
        }
        pub fn parameters(&self) -> &SystemParameters {
            if let Some(r) = &self.parameters {
                r as _
            } else {
                SystemParameters::default_ref()
            }
        }
        pub fn validator_report_records(&self) -> &[ValidatorReportRecord] {
            &self.validator_report_records
        }
        pub fn stake_subsidy(&self) -> &StakeSubsidy {
            if let Some(r) = &self.stake_subsidy {
                r as _
            } else {
                StakeSubsidy::default_ref()
            }
        }
        pub fn extra_fields(&self) -> &MoveTable {
            if let Some(r) = &self.extra_fields {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
    }
    impl ValidatorReportRecord {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ValidatorReportRecord> = std::sync::LazyLock::new(
                ValidatorReportRecord::default,
            );
            &DEFAULT
        }
    }
    impl SystemParameters {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<SystemParameters> = std::sync::LazyLock::new(
                SystemParameters::default,
            );
            &DEFAULT
        }
        pub fn extra_fields(&self) -> &MoveTable {
            if let Some(r) = &self.extra_fields {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
    }
    impl MoveTable {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<MoveTable> = std::sync::LazyLock::new(
                MoveTable::default,
            );
            &DEFAULT
        }
    }
    impl StakeSubsidy {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<StakeSubsidy> = std::sync::LazyLock::new(
                StakeSubsidy::default,
            );
            &DEFAULT
        }
        pub fn extra_fields(&self) -> &MoveTable {
            if let Some(r) = &self.extra_fields {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
    }
    impl StorageFund {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<StorageFund> = std::sync::LazyLock::new(
                StorageFund::default,
            );
            &DEFAULT
        }
    }
    impl ValidatorSet {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ValidatorSet> = std::sync::LazyLock::new(
                ValidatorSet::default,
            );
            &DEFAULT
        }
        pub fn active_validators(&self) -> &[Validator] {
            &self.active_validators
        }
        pub fn pending_active_validators(&self) -> &MoveTable {
            if let Some(r) = &self.pending_active_validators {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
        pub fn staking_pool_mappings(&self) -> &MoveTable {
            if let Some(r) = &self.staking_pool_mappings {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
        pub fn inactive_validators(&self) -> &MoveTable {
            if let Some(r) = &self.inactive_validators {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
        pub fn validator_candidates(&self) -> &MoveTable {
            if let Some(r) = &self.validator_candidates {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
        pub fn extra_fields(&self) -> &MoveTable {
            if let Some(r) = &self.extra_fields {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
    }
    impl Validator {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Validator> = std::sync::LazyLock::new(
                Validator::default,
            );
            &DEFAULT
        }
        pub fn metadata_extra_fields(&self) -> &MoveTable {
            if let Some(r) = &self.metadata_extra_fields {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
        pub fn staking_pool(&self) -> &StakingPool {
            if let Some(r) = &self.staking_pool {
                r as _
            } else {
                StakingPool::default_ref()
            }
        }
        pub fn extra_fields(&self) -> &MoveTable {
            if let Some(r) = &self.extra_fields {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
    }
    impl StakingPool {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<StakingPool> = std::sync::LazyLock::new(
                StakingPool::default,
            );
            &DEFAULT
        }
        pub fn exchange_rates(&self) -> &MoveTable {
            if let Some(r) = &self.exchange_rates {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
        pub fn extra_fields(&self) -> &MoveTable {
            if let Some(r) = &self.extra_fields {
                r as _
            } else {
                MoveTable::default_ref()
            }
        }
    }
    impl Transaction {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Transaction> = std::sync::LazyLock::new(
                Transaction::default,
            );
            &DEFAULT
        }
        pub fn bcs(&self) -> &Bcs {
            if let Some(r) = &self.bcs { r as _ } else { Bcs::default_ref() }
        }
        pub fn kind(&self) -> &TransactionKind {
            if let Some(r) = &self.kind {
                r as _
            } else {
                TransactionKind::default_ref()
            }
        }
        pub fn gas_payment(&self) -> &GasPayment {
            if let Some(r) = &self.gas_payment {
                r as _
            } else {
                GasPayment::default_ref()
            }
        }
        pub fn expiration(&self) -> &TransactionExpiration {
            if let Some(r) = &self.expiration {
                r as _
            } else {
                TransactionExpiration::default_ref()
            }
        }
    }
    impl GasPayment {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GasPayment> = std::sync::LazyLock::new(
                GasPayment::default,
            );
            &DEFAULT
        }
        pub fn objects(&self) -> &[ObjectReference] {
            &self.objects
        }
    }
    impl TransactionExpiration {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<TransactionExpiration> = std::sync::LazyLock::new(
                TransactionExpiration::default,
            );
            &DEFAULT
        }
    }
    impl TransactionKind {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<TransactionKind> = std::sync::LazyLock::new(
                TransactionKind::default,
            );
            &DEFAULT
        }
    }
    impl ProgrammableTransaction {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ProgrammableTransaction> = std::sync::LazyLock::new(
                ProgrammableTransaction::default,
            );
            &DEFAULT
        }
        pub fn inputs(&self) -> &[Input] {
            &self.inputs
        }
        pub fn commands(&self) -> &[Command] {
            &self.commands
        }
    }
    impl Command {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Command> = std::sync::LazyLock::new(
                Command::default,
            );
            &DEFAULT
        }
    }
    impl MoveCall {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<MoveCall> = std::sync::LazyLock::new(
                MoveCall::default,
            );
            &DEFAULT
        }
        pub fn arguments(&self) -> &[Argument] {
            &self.arguments
        }
    }
    impl TransferObjects {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<TransferObjects> = std::sync::LazyLock::new(
                TransferObjects::default,
            );
            &DEFAULT
        }
        pub fn objects(&self) -> &[Argument] {
            &self.objects
        }
        pub fn address(&self) -> &Argument {
            if let Some(r) = &self.address { r as _ } else { Argument::default_ref() }
        }
    }
    impl SplitCoins {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<SplitCoins> = std::sync::LazyLock::new(
                SplitCoins::default,
            );
            &DEFAULT
        }
        pub fn coin(&self) -> &Argument {
            if let Some(r) = &self.coin { r as _ } else { Argument::default_ref() }
        }
        pub fn amounts(&self) -> &[Argument] {
            &self.amounts
        }
    }
    impl MergeCoins {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<MergeCoins> = std::sync::LazyLock::new(
                MergeCoins::default,
            );
            &DEFAULT
        }
        pub fn coin(&self) -> &Argument {
            if let Some(r) = &self.coin { r as _ } else { Argument::default_ref() }
        }
        pub fn coins_to_merge(&self) -> &[Argument] {
            &self.coins_to_merge
        }
    }
    impl Publish {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Publish> = std::sync::LazyLock::new(
                Publish::default,
            );
            &DEFAULT
        }
    }
    impl MakeMoveVector {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<MakeMoveVector> = std::sync::LazyLock::new(
                MakeMoveVector::default,
            );
            &DEFAULT
        }
        pub fn elements(&self) -> &[Argument] {
            &self.elements
        }
    }
    impl Upgrade {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Upgrade> = std::sync::LazyLock::new(
                Upgrade::default,
            );
            &DEFAULT
        }
        pub fn ticket(&self) -> &Argument {
            if let Some(r) = &self.ticket { r as _ } else { Argument::default_ref() }
        }
    }
    impl RandomnessStateUpdate {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<RandomnessStateUpdate> = std::sync::LazyLock::new(
                RandomnessStateUpdate::default,
            );
            &DEFAULT
        }
    }
    impl ChangeEpoch {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ChangeEpoch> = std::sync::LazyLock::new(
                ChangeEpoch::default,
            );
            &DEFAULT
        }
        pub fn system_packages(&self) -> &[SystemPackage] {
            &self.system_packages
        }
    }
    impl SystemPackage {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<SystemPackage> = std::sync::LazyLock::new(
                SystemPackage::default,
            );
            &DEFAULT
        }
    }
    impl GenesisTransaction {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<GenesisTransaction> = std::sync::LazyLock::new(
                GenesisTransaction::default,
            );
            &DEFAULT
        }
        pub fn objects(&self) -> &[Object] {
            &self.objects
        }
    }
    impl ConsensusCommitPrologue {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ConsensusCommitPrologue> = std::sync::LazyLock::new(
                ConsensusCommitPrologue::default,
            );
            &DEFAULT
        }
        pub fn consensus_determined_version_assignments(
            &self,
        ) -> &ConsensusDeterminedVersionAssignments {
            if let Some(r) = &self.consensus_determined_version_assignments {
                r as _
            } else {
                ConsensusDeterminedVersionAssignments::default_ref()
            }
        }
    }
    impl VersionAssignment {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<VersionAssignment> = std::sync::LazyLock::new(
                VersionAssignment::default,
            );
            &DEFAULT
        }
    }
    impl CanceledTransaction {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<CanceledTransaction> = std::sync::LazyLock::new(
                CanceledTransaction::default,
            );
            &DEFAULT
        }
        pub fn version_assignments(&self) -> &[VersionAssignment] {
            &self.version_assignments
        }
    }
    impl ConsensusDeterminedVersionAssignments {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ConsensusDeterminedVersionAssignments> = std::sync::LazyLock::new(
                ConsensusDeterminedVersionAssignments::default,
            );
            &DEFAULT
        }
        pub fn canceled_transactions(&self) -> &[CanceledTransaction] {
            &self.canceled_transactions
        }
    }
    impl AuthenticatorStateUpdate {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<AuthenticatorStateUpdate> = std::sync::LazyLock::new(
                AuthenticatorStateUpdate::default,
            );
            &DEFAULT
        }
        pub fn new_active_jwks(&self) -> &[ActiveJwk] {
            &self.new_active_jwks
        }
    }
    impl ActiveJwk {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ActiveJwk> = std::sync::LazyLock::new(
                ActiveJwk::default,
            );
            &DEFAULT
        }
        pub fn id(&self) -> &JwkId {
            if let Some(r) = &self.id { r as _ } else { JwkId::default_ref() }
        }
        pub fn jwk(&self) -> &Jwk {
            if let Some(r) = &self.jwk { r as _ } else { Jwk::default_ref() }
        }
    }
    impl JwkId {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<JwkId> = std::sync::LazyLock::new(
                JwkId::default,
            );
            &DEFAULT
        }
    }
    impl Jwk {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<Jwk> = std::sync::LazyLock::new(
                Jwk::default,
            );
            &DEFAULT
        }
    }
    impl EndOfEpochTransaction {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<EndOfEpochTransaction> = std::sync::LazyLock::new(
                EndOfEpochTransaction::default,
            );
            &DEFAULT
        }
        pub fn transactions(&self) -> &[EndOfEpochTransactionKind] {
            &self.transactions
        }
    }
    impl EndOfEpochTransactionKind {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<EndOfEpochTransactionKind> = std::sync::LazyLock::new(
                EndOfEpochTransactionKind::default,
            );
            &DEFAULT
        }
    }
    impl AuthenticatorStateExpire {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<AuthenticatorStateExpire> = std::sync::LazyLock::new(
                AuthenticatorStateExpire::default,
            );
            &DEFAULT
        }
    }
    impl ExecutionTimeObservations {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ExecutionTimeObservations> = std::sync::LazyLock::new(
                ExecutionTimeObservations::default,
            );
            &DEFAULT
        }
        pub fn observations(&self) -> &[ExecutionTimeObservation] {
            &self.observations
        }
    }
    impl ExecutionTimeObservation {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ExecutionTimeObservation> = std::sync::LazyLock::new(
                ExecutionTimeObservation::default,
            );
            &DEFAULT
        }
        pub fn move_entry_point(&self) -> &MoveCall {
            if let Some(r) = &self.move_entry_point {
                r as _
            } else {
                MoveCall::default_ref()
            }
        }
        pub fn validator_observations(&self) -> &[ValidatorExecutionTimeObservation] {
            &self.validator_observations
        }
    }
    impl ValidatorExecutionTimeObservation {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ValidatorExecutionTimeObservation> = std::sync::LazyLock::new(
                ValidatorExecutionTimeObservation::default,
            );
            &DEFAULT
        }
    }
    impl ExecuteTransactionRequest {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ExecuteTransactionRequest> = std::sync::LazyLock::new(
                ExecuteTransactionRequest::default,
            );
            &DEFAULT
        }
        pub fn transaction(&self) -> &Transaction {
            if let Some(r) = &self.transaction {
                r as _
            } else {
                Transaction::default_ref()
            }
        }
        pub fn signatures(&self) -> &[UserSignature] {
            &self.signatures
        }
    }
    impl ExecuteTransactionResponse {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<ExecuteTransactionResponse> = std::sync::LazyLock::new(
                ExecuteTransactionResponse::default,
            );
            &DEFAULT
        }
        pub fn finality(&self) -> &TransactionFinality {
            if let Some(r) = &self.finality {
                r as _
            } else {
                TransactionFinality::default_ref()
            }
        }
        pub fn transaction(&self) -> &ExecutedTransaction {
            if let Some(r) = &self.transaction {
                r as _
            } else {
                ExecutedTransaction::default_ref()
            }
        }
    }
    impl TransactionFinality {
        #[doc(hidden)]
        pub fn default_ref() -> &'static Self {
            static DEFAULT: std::sync::LazyLock<TransactionFinality> = std::sync::LazyLock::new(
                TransactionFinality::default,
            );
            &DEFAULT
        }
    }
}
