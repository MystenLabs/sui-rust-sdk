mod _getter_impls {
    #![allow(clippy::useless_conversion)]
    use super::*;
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl BatchGetObjectsResponse {
        pub const fn const_default() -> Self {
            Self { objects: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl BatchGetTransactionsResponse {
        pub const fn const_default() -> Self {
            Self { transactions: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl Bcs {
        pub const fn const_default() -> Self {
            Self { name: None, value: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl CheckpointCommitment {
        pub const fn const_default() -> Self {
            Self { kind: None, digest: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl Command {
        pub const fn const_default() -> Self {
            Self { command: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl CongestedObjects {
        pub const fn const_default() -> Self {
            Self { objects: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl EndOfEpochTransaction {
        pub const fn const_default() -> Self {
            Self { transactions: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl EndOfEpochTransactionKind {
        pub const fn const_default() -> Self {
            Self { kind: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl ExecutionStatus {
        pub const fn const_default() -> Self {
            Self { success: None, error: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GenesisTransaction {
        pub const fn const_default() -> Self {
            Self { objects: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetBalanceResponse {
        pub const fn const_default() -> Self {
            Self { balance: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetCheckpointResponse {
        pub const fn const_default() -> Self {
            Self { checkpoint: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetCoinInfoRequest {
        pub const fn const_default() -> Self {
            Self { coin_type: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetDatatypeResponse {
        pub const fn const_default() -> Self {
            Self { datatype: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetEpochResponse {
        pub const fn const_default() -> Self {
            Self { epoch: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetFunctionResponse {
        pub const fn const_default() -> Self {
            Self { function: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetObjectResponse {
        pub const fn const_default() -> Self {
            Self { object: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetObjectResult {
        pub const fn const_default() -> Self {
            Self { result: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetPackageRequest {
        pub const fn const_default() -> Self {
            Self { package_id: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetPackageResponse {
        pub const fn const_default() -> Self {
            Self { package: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetServiceInfoRequest {
        pub const fn const_default() -> Self {
            Self {}
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetTransactionResponse {
        pub const fn const_default() -> Self {
            Self { transaction: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl GetTransactionResult {
        pub const fn const_default() -> Self {
            Self { result: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl JwkId {
        pub const fn const_default() -> Self {
            Self { iss: None, kid: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl LookupNameRequest {
        pub const fn const_default() -> Self {
            Self { name: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl LookupNameResponse {
        pub const fn const_default() -> Self {
            Self { record: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl MoveTable {
        pub const fn const_default() -> Self {
            Self { id: None, size: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl ReverseLookupNameRequest {
        pub const fn const_default() -> Self {
            Self { address: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl ReverseLookupNameResponse {
        pub const fn const_default() -> Self {
            Self { record: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl SizeError {
        pub const fn const_default() -> Self {
            Self { size: None, max_size: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl SubscribeCheckpointsRequest {
        pub const fn const_default() -> Self {
            Self { read_mask: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl TransactionExpiration {
        pub const fn const_default() -> Self {
            Self { kind: None, epoch: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl TransactionFinality {
        pub const fn const_default() -> Self {
            Self { finality: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl TransactionKind {
        pub const fn const_default() -> Self {
            Self { kind: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl ZkLoginProof {
        pub const fn const_default() -> Self {
            Self { a: None, b: None, c: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
}
