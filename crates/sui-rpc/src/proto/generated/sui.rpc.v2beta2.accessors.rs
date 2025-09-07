mod _accessor_impls {
    #![allow(clippy::useless_conversion)]
    impl super::ActiveJwk {
        pub const fn const_default() -> Self {
            Self {
                id: None,
                jwk: None,
                epoch: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ActiveJwk = super::ActiveJwk::const_default();
            &DEFAULT
        }
        pub fn id(&self) -> &super::JwkId {
            self.id
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::JwkId::default_instance() as _)
        }
        pub fn id_opt_mut(&mut self) -> Option<&mut super::JwkId> {
            self.id.as_mut().map(|field| field as _)
        }
        pub fn id_mut(&mut self) -> &mut super::JwkId {
            self.id.get_or_insert_default()
        }
        pub fn id_opt(&self) -> Option<&super::JwkId> {
            self.id.as_ref().map(|field| field as _)
        }
        pub fn set_id<T: Into<super::JwkId>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        pub fn with_id<T: Into<super::JwkId>>(mut self, field: T) -> Self {
            self.set_id(field);
            self
        }
        pub fn jwk(&self) -> &super::Jwk {
            self.jwk
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Jwk::default_instance() as _)
        }
        pub fn jwk_opt_mut(&mut self) -> Option<&mut super::Jwk> {
            self.jwk.as_mut().map(|field| field as _)
        }
        pub fn jwk_mut(&mut self) -> &mut super::Jwk {
            self.jwk.get_or_insert_default()
        }
        pub fn jwk_opt(&self) -> Option<&super::Jwk> {
            self.jwk.as_ref().map(|field| field as _)
        }
        pub fn set_jwk<T: Into<super::Jwk>>(&mut self, field: T) {
            self.jwk = Some(field.into().into());
        }
        pub fn with_jwk<T: Into<super::Jwk>>(mut self, field: T) -> Self {
            self.set_jwk(field);
            self
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
    }
    impl super::Argument {
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
            static DEFAULT: super::Argument = super::Argument::const_default();
            &DEFAULT
        }
        pub fn input_opt_mut(&mut self) -> Option<&mut u32> {
            self.input.as_mut().map(|field| field as _)
        }
        pub fn input_mut(&mut self) -> &mut u32 {
            self.input.get_or_insert_default()
        }
        pub fn input_opt(&self) -> Option<u32> {
            self.input.as_ref().map(|field| *field)
        }
        pub fn set_input<T: Into<u32>>(&mut self, field: T) {
            self.input = Some(field.into().into());
        }
        pub fn with_input<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_input(field);
            self
        }
        pub fn result_opt_mut(&mut self) -> Option<&mut u32> {
            self.result.as_mut().map(|field| field as _)
        }
        pub fn result_mut(&mut self) -> &mut u32 {
            self.result.get_or_insert_default()
        }
        pub fn result_opt(&self) -> Option<u32> {
            self.result.as_ref().map(|field| *field)
        }
        pub fn set_result<T: Into<u32>>(&mut self, field: T) {
            self.result = Some(field.into().into());
        }
        pub fn with_result<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_result(field);
            self
        }
        pub fn subresult_opt_mut(&mut self) -> Option<&mut u32> {
            self.subresult.as_mut().map(|field| field as _)
        }
        pub fn subresult_mut(&mut self) -> &mut u32 {
            self.subresult.get_or_insert_default()
        }
        pub fn subresult_opt(&self) -> Option<u32> {
            self.subresult.as_ref().map(|field| *field)
        }
        pub fn set_subresult<T: Into<u32>>(&mut self, field: T) {
            self.subresult = Some(field.into().into());
        }
        pub fn with_subresult<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_subresult(field);
            self
        }
    }
    impl super::AuthenticatorStateExpire {
        pub const fn const_default() -> Self {
            Self {
                min_epoch: None,
                authenticator_object_initial_shared_version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::AuthenticatorStateExpire = super::AuthenticatorStateExpire::const_default();
            &DEFAULT
        }
        pub fn min_epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.min_epoch.as_mut().map(|field| field as _)
        }
        pub fn min_epoch_mut(&mut self) -> &mut u64 {
            self.min_epoch.get_or_insert_default()
        }
        pub fn min_epoch_opt(&self) -> Option<u64> {
            self.min_epoch.as_ref().map(|field| *field)
        }
        pub fn set_min_epoch<T: Into<u64>>(&mut self, field: T) {
            self.min_epoch = Some(field.into().into());
        }
        pub fn with_min_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_min_epoch(field);
            self
        }
        pub fn authenticator_object_initial_shared_version_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.authenticator_object_initial_shared_version
                .as_mut()
                .map(|field| field as _)
        }
        pub fn authenticator_object_initial_shared_version_mut(&mut self) -> &mut u64 {
            self.authenticator_object_initial_shared_version.get_or_insert_default()
        }
        pub fn authenticator_object_initial_shared_version_opt(&self) -> Option<u64> {
            self.authenticator_object_initial_shared_version.as_ref().map(|field| *field)
        }
        pub fn set_authenticator_object_initial_shared_version<T: Into<u64>>(
            &mut self,
            field: T,
        ) {
            self.authenticator_object_initial_shared_version = Some(field.into().into());
        }
        pub fn with_authenticator_object_initial_shared_version<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_authenticator_object_initial_shared_version(field);
            self
        }
    }
    impl super::AuthenticatorStateUpdate {
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
            static DEFAULT: super::AuthenticatorStateUpdate = super::AuthenticatorStateUpdate::const_default();
            &DEFAULT
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn round_opt_mut(&mut self) -> Option<&mut u64> {
            self.round.as_mut().map(|field| field as _)
        }
        pub fn round_mut(&mut self) -> &mut u64 {
            self.round.get_or_insert_default()
        }
        pub fn round_opt(&self) -> Option<u64> {
            self.round.as_ref().map(|field| *field)
        }
        pub fn set_round<T: Into<u64>>(&mut self, field: T) {
            self.round = Some(field.into().into());
        }
        pub fn with_round<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_round(field);
            self
        }
        pub fn new_active_jwks(&self) -> &[super::ActiveJwk] {
            &self.new_active_jwks
        }
        pub fn set_new_active_jwks(&mut self, field: Vec<super::ActiveJwk>) {
            self.new_active_jwks = field;
        }
        pub fn with_new_active_jwks(mut self, field: Vec<super::ActiveJwk>) -> Self {
            self.set_new_active_jwks(field);
            self
        }
        pub fn authenticator_object_initial_shared_version_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.authenticator_object_initial_shared_version
                .as_mut()
                .map(|field| field as _)
        }
        pub fn authenticator_object_initial_shared_version_mut(&mut self) -> &mut u64 {
            self.authenticator_object_initial_shared_version.get_or_insert_default()
        }
        pub fn authenticator_object_initial_shared_version_opt(&self) -> Option<u64> {
            self.authenticator_object_initial_shared_version.as_ref().map(|field| *field)
        }
        pub fn set_authenticator_object_initial_shared_version<T: Into<u64>>(
            &mut self,
            field: T,
        ) {
            self.authenticator_object_initial_shared_version = Some(field.into().into());
        }
        pub fn with_authenticator_object_initial_shared_version<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_authenticator_object_initial_shared_version(field);
            self
        }
    }
    impl super::Balance {
        pub const fn const_default() -> Self {
            Self {
                coin_type: None,
                balance: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Balance = super::Balance::const_default();
            &DEFAULT
        }
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field);
            self
        }
        pub fn balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.balance.as_mut().map(|field| field as _)
        }
        pub fn balance_mut(&mut self) -> &mut u64 {
            self.balance.get_or_insert_default()
        }
        pub fn balance_opt(&self) -> Option<u64> {
            self.balance.as_ref().map(|field| *field)
        }
        pub fn set_balance<T: Into<u64>>(&mut self, field: T) {
            self.balance = Some(field.into().into());
        }
        pub fn with_balance<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_balance(field);
            self
        }
    }
    impl super::BalanceChange {
        pub const fn const_default() -> Self {
            Self {
                address: None,
                coin_type: None,
                amount: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::BalanceChange = super::BalanceChange::const_default();
            &DEFAULT
        }
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field);
            self
        }
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field);
            self
        }
        pub fn amount_opt_mut(&mut self) -> Option<&mut String> {
            self.amount.as_mut().map(|field| field as _)
        }
        pub fn amount_mut(&mut self) -> &mut String {
            self.amount.get_or_insert_default()
        }
        pub fn amount_opt(&self) -> Option<&str> {
            self.amount.as_ref().map(|field| field as _)
        }
        pub fn set_amount<T: Into<String>>(&mut self, field: T) {
            self.amount = Some(field.into().into());
        }
        pub fn with_amount<T: Into<String>>(mut self, field: T) -> Self {
            self.set_amount(field);
            self
        }
    }
    impl super::BatchGetObjectsRequest {
        pub const fn const_default() -> Self {
            Self {
                requests: Vec::new(),
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::BatchGetObjectsRequest = super::BatchGetObjectsRequest::const_default();
            &DEFAULT
        }
        pub fn requests(&self) -> &[super::GetObjectRequest] {
            &self.requests
        }
        pub fn set_requests(&mut self, field: Vec<super::GetObjectRequest>) {
            self.requests = field;
        }
        pub fn with_requests(mut self, field: Vec<super::GetObjectRequest>) -> Self {
            self.set_requests(field);
            self
        }
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field);
            self
        }
    }
    impl super::BatchGetObjectsResponse {
        pub const fn const_default() -> Self {
            Self { objects: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::BatchGetObjectsResponse = super::BatchGetObjectsResponse::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[super::GetObjectResult] {
            &self.objects
        }
        pub fn set_objects(&mut self, field: Vec<super::GetObjectResult>) {
            self.objects = field;
        }
        pub fn with_objects(mut self, field: Vec<super::GetObjectResult>) -> Self {
            self.set_objects(field);
            self
        }
    }
    impl super::BatchGetTransactionsRequest {
        pub const fn const_default() -> Self {
            Self {
                digests: Vec::new(),
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::BatchGetTransactionsRequest = super::BatchGetTransactionsRequest::const_default();
            &DEFAULT
        }
        pub fn digests(&self) -> &[String] {
            &self.digests
        }
        pub fn set_digests(&mut self, field: Vec<String>) {
            self.digests = field;
        }
        pub fn with_digests(mut self, field: Vec<String>) -> Self {
            self.set_digests(field);
            self
        }
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field);
            self
        }
    }
    impl super::BatchGetTransactionsResponse {
        pub const fn const_default() -> Self {
            Self { transactions: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::BatchGetTransactionsResponse = super::BatchGetTransactionsResponse::const_default();
            &DEFAULT
        }
        pub fn transactions(&self) -> &[super::GetTransactionResult] {
            &self.transactions
        }
        pub fn set_transactions(&mut self, field: Vec<super::GetTransactionResult>) {
            self.transactions = field;
        }
        pub fn with_transactions(
            mut self,
            field: Vec<super::GetTransactionResult>,
        ) -> Self {
            self.set_transactions(field);
            self
        }
    }
    impl super::Bcs {
        pub const fn const_default() -> Self {
            Self { name: None, value: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Bcs = super::Bcs::const_default();
            &DEFAULT
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
        pub fn value_opt(&self) -> Option<&[u8]> {
            self.value.as_ref().map(|field| field as _)
        }
        pub fn set_value<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.value = Some(field.into().into());
        }
        pub fn with_value<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_value(field);
            self
        }
    }
    impl super::CanceledTransaction {
        pub const fn const_default() -> Self {
            Self {
                digest: None,
                version_assignments: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CanceledTransaction = super::CanceledTransaction::const_default();
            &DEFAULT
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn version_assignments(&self) -> &[super::VersionAssignment] {
            &self.version_assignments
        }
        pub fn set_version_assignments(&mut self, field: Vec<super::VersionAssignment>) {
            self.version_assignments = field;
        }
        pub fn with_version_assignments(
            mut self,
            field: Vec<super::VersionAssignment>,
        ) -> Self {
            self.set_version_assignments(field);
            self
        }
    }
    impl super::ChangeEpoch {
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
            static DEFAULT: super::ChangeEpoch = super::ChangeEpoch::const_default();
            &DEFAULT
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn protocol_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.protocol_version.as_mut().map(|field| field as _)
        }
        pub fn protocol_version_mut(&mut self) -> &mut u64 {
            self.protocol_version.get_or_insert_default()
        }
        pub fn protocol_version_opt(&self) -> Option<u64> {
            self.protocol_version.as_ref().map(|field| *field)
        }
        pub fn set_protocol_version<T: Into<u64>>(&mut self, field: T) {
            self.protocol_version = Some(field.into().into());
        }
        pub fn with_protocol_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_protocol_version(field);
            self
        }
        pub fn storage_charge_opt_mut(&mut self) -> Option<&mut u64> {
            self.storage_charge.as_mut().map(|field| field as _)
        }
        pub fn storage_charge_mut(&mut self) -> &mut u64 {
            self.storage_charge.get_or_insert_default()
        }
        pub fn storage_charge_opt(&self) -> Option<u64> {
            self.storage_charge.as_ref().map(|field| *field)
        }
        pub fn set_storage_charge<T: Into<u64>>(&mut self, field: T) {
            self.storage_charge = Some(field.into().into());
        }
        pub fn with_storage_charge<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_storage_charge(field);
            self
        }
        pub fn computation_charge_opt_mut(&mut self) -> Option<&mut u64> {
            self.computation_charge.as_mut().map(|field| field as _)
        }
        pub fn computation_charge_mut(&mut self) -> &mut u64 {
            self.computation_charge.get_or_insert_default()
        }
        pub fn computation_charge_opt(&self) -> Option<u64> {
            self.computation_charge.as_ref().map(|field| *field)
        }
        pub fn set_computation_charge<T: Into<u64>>(&mut self, field: T) {
            self.computation_charge = Some(field.into().into());
        }
        pub fn with_computation_charge<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_computation_charge(field);
            self
        }
        pub fn storage_rebate_opt_mut(&mut self) -> Option<&mut u64> {
            self.storage_rebate.as_mut().map(|field| field as _)
        }
        pub fn storage_rebate_mut(&mut self) -> &mut u64 {
            self.storage_rebate.get_or_insert_default()
        }
        pub fn storage_rebate_opt(&self) -> Option<u64> {
            self.storage_rebate.as_ref().map(|field| *field)
        }
        pub fn set_storage_rebate<T: Into<u64>>(&mut self, field: T) {
            self.storage_rebate = Some(field.into().into());
        }
        pub fn with_storage_rebate<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_storage_rebate(field);
            self
        }
        pub fn non_refundable_storage_fee_opt_mut(&mut self) -> Option<&mut u64> {
            self.non_refundable_storage_fee.as_mut().map(|field| field as _)
        }
        pub fn non_refundable_storage_fee_mut(&mut self) -> &mut u64 {
            self.non_refundable_storage_fee.get_or_insert_default()
        }
        pub fn non_refundable_storage_fee_opt(&self) -> Option<u64> {
            self.non_refundable_storage_fee.as_ref().map(|field| *field)
        }
        pub fn set_non_refundable_storage_fee<T: Into<u64>>(&mut self, field: T) {
            self.non_refundable_storage_fee = Some(field.into().into());
        }
        pub fn with_non_refundable_storage_fee<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_non_refundable_storage_fee(field);
            self
        }
        pub fn epoch_start_timestamp_opt_mut(
            &mut self,
        ) -> Option<&mut ::prost_types::Timestamp> {
            self.epoch_start_timestamp.as_mut().map(|field| field as _)
        }
        pub fn epoch_start_timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.epoch_start_timestamp.get_or_insert_default()
        }
        pub fn epoch_start_timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.epoch_start_timestamp.as_ref().map(|field| field as _)
        }
        pub fn set_epoch_start_timestamp<T: Into<::prost_types::Timestamp>>(
            &mut self,
            field: T,
        ) {
            self.epoch_start_timestamp = Some(field.into().into());
        }
        pub fn with_epoch_start_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_epoch_start_timestamp(field);
            self
        }
        pub fn system_packages(&self) -> &[super::SystemPackage] {
            &self.system_packages
        }
        pub fn set_system_packages(&mut self, field: Vec<super::SystemPackage>) {
            self.system_packages = field;
        }
        pub fn with_system_packages(mut self, field: Vec<super::SystemPackage>) -> Self {
            self.set_system_packages(field);
            self
        }
    }
    impl super::ChangedObject {
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
            static DEFAULT: super::ChangedObject = super::ChangedObject::const_default();
            &DEFAULT
        }
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field);
            self
        }
        pub fn input_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.input_version.as_mut().map(|field| field as _)
        }
        pub fn input_version_mut(&mut self) -> &mut u64 {
            self.input_version.get_or_insert_default()
        }
        pub fn input_version_opt(&self) -> Option<u64> {
            self.input_version.as_ref().map(|field| *field)
        }
        pub fn set_input_version<T: Into<u64>>(&mut self, field: T) {
            self.input_version = Some(field.into().into());
        }
        pub fn with_input_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_input_version(field);
            self
        }
        pub fn input_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.input_digest.as_mut().map(|field| field as _)
        }
        pub fn input_digest_mut(&mut self) -> &mut String {
            self.input_digest.get_or_insert_default()
        }
        pub fn input_digest_opt(&self) -> Option<&str> {
            self.input_digest.as_ref().map(|field| field as _)
        }
        pub fn set_input_digest<T: Into<String>>(&mut self, field: T) {
            self.input_digest = Some(field.into().into());
        }
        pub fn with_input_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_input_digest(field);
            self
        }
        pub fn input_owner(&self) -> &super::Owner {
            self.input_owner
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Owner::default_instance() as _)
        }
        pub fn input_owner_opt_mut(&mut self) -> Option<&mut super::Owner> {
            self.input_owner.as_mut().map(|field| field as _)
        }
        pub fn input_owner_mut(&mut self) -> &mut super::Owner {
            self.input_owner.get_or_insert_default()
        }
        pub fn input_owner_opt(&self) -> Option<&super::Owner> {
            self.input_owner.as_ref().map(|field| field as _)
        }
        pub fn set_input_owner<T: Into<super::Owner>>(&mut self, field: T) {
            self.input_owner = Some(field.into().into());
        }
        pub fn with_input_owner<T: Into<super::Owner>>(mut self, field: T) -> Self {
            self.set_input_owner(field);
            self
        }
        pub fn output_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.output_version.as_mut().map(|field| field as _)
        }
        pub fn output_version_mut(&mut self) -> &mut u64 {
            self.output_version.get_or_insert_default()
        }
        pub fn output_version_opt(&self) -> Option<u64> {
            self.output_version.as_ref().map(|field| *field)
        }
        pub fn set_output_version<T: Into<u64>>(&mut self, field: T) {
            self.output_version = Some(field.into().into());
        }
        pub fn with_output_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_output_version(field);
            self
        }
        pub fn output_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.output_digest.as_mut().map(|field| field as _)
        }
        pub fn output_digest_mut(&mut self) -> &mut String {
            self.output_digest.get_or_insert_default()
        }
        pub fn output_digest_opt(&self) -> Option<&str> {
            self.output_digest.as_ref().map(|field| field as _)
        }
        pub fn set_output_digest<T: Into<String>>(&mut self, field: T) {
            self.output_digest = Some(field.into().into());
        }
        pub fn with_output_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_output_digest(field);
            self
        }
        pub fn output_owner(&self) -> &super::Owner {
            self.output_owner
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Owner::default_instance() as _)
        }
        pub fn output_owner_opt_mut(&mut self) -> Option<&mut super::Owner> {
            self.output_owner.as_mut().map(|field| field as _)
        }
        pub fn output_owner_mut(&mut self) -> &mut super::Owner {
            self.output_owner.get_or_insert_default()
        }
        pub fn output_owner_opt(&self) -> Option<&super::Owner> {
            self.output_owner.as_ref().map(|field| field as _)
        }
        pub fn set_output_owner<T: Into<super::Owner>>(&mut self, field: T) {
            self.output_owner = Some(field.into().into());
        }
        pub fn with_output_owner<T: Into<super::Owner>>(mut self, field: T) -> Self {
            self.set_output_owner(field);
            self
        }
        pub fn object_type_opt_mut(&mut self) -> Option<&mut String> {
            self.object_type.as_mut().map(|field| field as _)
        }
        pub fn object_type_mut(&mut self) -> &mut String {
            self.object_type.get_or_insert_default()
        }
        pub fn object_type_opt(&self) -> Option<&str> {
            self.object_type.as_ref().map(|field| field as _)
        }
        pub fn set_object_type<T: Into<String>>(&mut self, field: T) {
            self.object_type = Some(field.into().into());
        }
        pub fn with_object_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_type(field);
            self
        }
    }
    impl super::Checkpoint {
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
            static DEFAULT: super::Checkpoint = super::Checkpoint::const_default();
            &DEFAULT
        }
        pub fn sequence_number_opt_mut(&mut self) -> Option<&mut u64> {
            self.sequence_number.as_mut().map(|field| field as _)
        }
        pub fn sequence_number_mut(&mut self) -> &mut u64 {
            self.sequence_number.get_or_insert_default()
        }
        pub fn sequence_number_opt(&self) -> Option<u64> {
            self.sequence_number.as_ref().map(|field| *field)
        }
        pub fn set_sequence_number<T: Into<u64>>(&mut self, field: T) {
            self.sequence_number = Some(field.into().into());
        }
        pub fn with_sequence_number<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_sequence_number(field);
            self
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn summary(&self) -> &super::CheckpointSummary {
            self.summary
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CheckpointSummary::default_instance() as _)
        }
        pub fn summary_opt_mut(&mut self) -> Option<&mut super::CheckpointSummary> {
            self.summary.as_mut().map(|field| field as _)
        }
        pub fn summary_mut(&mut self) -> &mut super::CheckpointSummary {
            self.summary.get_or_insert_default()
        }
        pub fn summary_opt(&self) -> Option<&super::CheckpointSummary> {
            self.summary.as_ref().map(|field| field as _)
        }
        pub fn set_summary<T: Into<super::CheckpointSummary>>(&mut self, field: T) {
            self.summary = Some(field.into().into());
        }
        pub fn with_summary<T: Into<super::CheckpointSummary>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_summary(field);
            self
        }
        pub fn signature(&self) -> &super::ValidatorAggregatedSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::ValidatorAggregatedSignature::default_instance() as _
                })
        }
        pub fn signature_opt_mut(
            &mut self,
        ) -> Option<&mut super::ValidatorAggregatedSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        pub fn signature_mut(&mut self) -> &mut super::ValidatorAggregatedSignature {
            self.signature.get_or_insert_default()
        }
        pub fn signature_opt(&self) -> Option<&super::ValidatorAggregatedSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        pub fn set_signature<T: Into<super::ValidatorAggregatedSignature>>(
            &mut self,
            field: T,
        ) {
            self.signature = Some(field.into().into());
        }
        pub fn with_signature<T: Into<super::ValidatorAggregatedSignature>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field);
            self
        }
        pub fn contents(&self) -> &super::CheckpointContents {
            self.contents
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CheckpointContents::default_instance() as _)
        }
        pub fn contents_opt_mut(&mut self) -> Option<&mut super::CheckpointContents> {
            self.contents.as_mut().map(|field| field as _)
        }
        pub fn contents_mut(&mut self) -> &mut super::CheckpointContents {
            self.contents.get_or_insert_default()
        }
        pub fn contents_opt(&self) -> Option<&super::CheckpointContents> {
            self.contents.as_ref().map(|field| field as _)
        }
        pub fn set_contents<T: Into<super::CheckpointContents>>(&mut self, field: T) {
            self.contents = Some(field.into().into());
        }
        pub fn with_contents<T: Into<super::CheckpointContents>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_contents(field);
            self
        }
        pub fn transactions(&self) -> &[super::ExecutedTransaction] {
            &self.transactions
        }
        pub fn set_transactions(&mut self, field: Vec<super::ExecutedTransaction>) {
            self.transactions = field;
        }
        pub fn with_transactions(
            mut self,
            field: Vec<super::ExecutedTransaction>,
        ) -> Self {
            self.set_transactions(field);
            self
        }
    }
    impl super::CheckpointCommitment {
        pub const fn const_default() -> Self {
            Self { kind: None, digest: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CheckpointCommitment = super::CheckpointCommitment::const_default();
            &DEFAULT
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
    }
    impl super::CheckpointContents {
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
            static DEFAULT: super::CheckpointContents = super::CheckpointContents::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field);
            self
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut i32> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut i32 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<i32> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<i32>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<i32>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn transactions(&self) -> &[super::CheckpointedTransactionInfo] {
            &self.transactions
        }
        pub fn set_transactions(
            &mut self,
            field: Vec<super::CheckpointedTransactionInfo>,
        ) {
            self.transactions = field;
        }
        pub fn with_transactions(
            mut self,
            field: Vec<super::CheckpointedTransactionInfo>,
        ) -> Self {
            self.set_transactions(field);
            self
        }
    }
    impl super::CheckpointSummary {
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
            static DEFAULT: super::CheckpointSummary = super::CheckpointSummary::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field);
            self
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn sequence_number_opt_mut(&mut self) -> Option<&mut u64> {
            self.sequence_number.as_mut().map(|field| field as _)
        }
        pub fn sequence_number_mut(&mut self) -> &mut u64 {
            self.sequence_number.get_or_insert_default()
        }
        pub fn sequence_number_opt(&self) -> Option<u64> {
            self.sequence_number.as_ref().map(|field| *field)
        }
        pub fn set_sequence_number<T: Into<u64>>(&mut self, field: T) {
            self.sequence_number = Some(field.into().into());
        }
        pub fn with_sequence_number<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_sequence_number(field);
            self
        }
        pub fn total_network_transactions_opt_mut(&mut self) -> Option<&mut u64> {
            self.total_network_transactions.as_mut().map(|field| field as _)
        }
        pub fn total_network_transactions_mut(&mut self) -> &mut u64 {
            self.total_network_transactions.get_or_insert_default()
        }
        pub fn total_network_transactions_opt(&self) -> Option<u64> {
            self.total_network_transactions.as_ref().map(|field| *field)
        }
        pub fn set_total_network_transactions<T: Into<u64>>(&mut self, field: T) {
            self.total_network_transactions = Some(field.into().into());
        }
        pub fn with_total_network_transactions<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_total_network_transactions(field);
            self
        }
        pub fn content_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.content_digest.as_mut().map(|field| field as _)
        }
        pub fn content_digest_mut(&mut self) -> &mut String {
            self.content_digest.get_or_insert_default()
        }
        pub fn content_digest_opt(&self) -> Option<&str> {
            self.content_digest.as_ref().map(|field| field as _)
        }
        pub fn set_content_digest<T: Into<String>>(&mut self, field: T) {
            self.content_digest = Some(field.into().into());
        }
        pub fn with_content_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_content_digest(field);
            self
        }
        pub fn previous_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.previous_digest.as_mut().map(|field| field as _)
        }
        pub fn previous_digest_mut(&mut self) -> &mut String {
            self.previous_digest.get_or_insert_default()
        }
        pub fn previous_digest_opt(&self) -> Option<&str> {
            self.previous_digest.as_ref().map(|field| field as _)
        }
        pub fn set_previous_digest<T: Into<String>>(&mut self, field: T) {
            self.previous_digest = Some(field.into().into());
        }
        pub fn with_previous_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_previous_digest(field);
            self
        }
        pub fn epoch_rolling_gas_cost_summary(&self) -> &super::GasCostSummary {
            self.epoch_rolling_gas_cost_summary
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::GasCostSummary::default_instance() as _)
        }
        pub fn epoch_rolling_gas_cost_summary_opt_mut(
            &mut self,
        ) -> Option<&mut super::GasCostSummary> {
            self.epoch_rolling_gas_cost_summary.as_mut().map(|field| field as _)
        }
        pub fn epoch_rolling_gas_cost_summary_mut(
            &mut self,
        ) -> &mut super::GasCostSummary {
            self.epoch_rolling_gas_cost_summary.get_or_insert_default()
        }
        pub fn epoch_rolling_gas_cost_summary_opt(
            &self,
        ) -> Option<&super::GasCostSummary> {
            self.epoch_rolling_gas_cost_summary.as_ref().map(|field| field as _)
        }
        pub fn set_epoch_rolling_gas_cost_summary<T: Into<super::GasCostSummary>>(
            &mut self,
            field: T,
        ) {
            self.epoch_rolling_gas_cost_summary = Some(field.into().into());
        }
        pub fn with_epoch_rolling_gas_cost_summary<T: Into<super::GasCostSummary>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_epoch_rolling_gas_cost_summary(field);
            self
        }
        pub fn timestamp_opt_mut(&mut self) -> Option<&mut ::prost_types::Timestamp> {
            self.timestamp.as_mut().map(|field| field as _)
        }
        pub fn timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.timestamp.get_or_insert_default()
        }
        pub fn timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.timestamp.as_ref().map(|field| field as _)
        }
        pub fn set_timestamp<T: Into<::prost_types::Timestamp>>(&mut self, field: T) {
            self.timestamp = Some(field.into().into());
        }
        pub fn with_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_timestamp(field);
            self
        }
        pub fn commitments(&self) -> &[super::CheckpointCommitment] {
            &self.commitments
        }
        pub fn set_commitments(&mut self, field: Vec<super::CheckpointCommitment>) {
            self.commitments = field;
        }
        pub fn with_commitments(
            mut self,
            field: Vec<super::CheckpointCommitment>,
        ) -> Self {
            self.set_commitments(field);
            self
        }
        pub fn end_of_epoch_data(&self) -> &super::EndOfEpochData {
            self.end_of_epoch_data
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::EndOfEpochData::default_instance() as _)
        }
        pub fn end_of_epoch_data_opt_mut(
            &mut self,
        ) -> Option<&mut super::EndOfEpochData> {
            self.end_of_epoch_data.as_mut().map(|field| field as _)
        }
        pub fn end_of_epoch_data_mut(&mut self) -> &mut super::EndOfEpochData {
            self.end_of_epoch_data.get_or_insert_default()
        }
        pub fn end_of_epoch_data_opt(&self) -> Option<&super::EndOfEpochData> {
            self.end_of_epoch_data.as_ref().map(|field| field as _)
        }
        pub fn set_end_of_epoch_data<T: Into<super::EndOfEpochData>>(
            &mut self,
            field: T,
        ) {
            self.end_of_epoch_data = Some(field.into().into());
        }
        pub fn with_end_of_epoch_data<T: Into<super::EndOfEpochData>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_end_of_epoch_data(field);
            self
        }
        pub fn version_specific_data_opt(&self) -> Option<&[u8]> {
            self.version_specific_data.as_ref().map(|field| field as _)
        }
        pub fn set_version_specific_data<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.version_specific_data = Some(field.into().into());
        }
        pub fn with_version_specific_data<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_version_specific_data(field);
            self
        }
    }
    impl super::CheckpointedTransactionInfo {
        pub const fn const_default() -> Self {
            Self {
                transaction: None,
                effects: None,
                signatures: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CheckpointedTransactionInfo = super::CheckpointedTransactionInfo::const_default();
            &DEFAULT
        }
        pub fn transaction_opt_mut(&mut self) -> Option<&mut String> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut String {
            self.transaction.get_or_insert_default()
        }
        pub fn transaction_opt(&self) -> Option<&str> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn set_transaction<T: Into<String>>(&mut self, field: T) {
            self.transaction = Some(field.into().into());
        }
        pub fn with_transaction<T: Into<String>>(mut self, field: T) -> Self {
            self.set_transaction(field);
            self
        }
        pub fn effects_opt_mut(&mut self) -> Option<&mut String> {
            self.effects.as_mut().map(|field| field as _)
        }
        pub fn effects_mut(&mut self) -> &mut String {
            self.effects.get_or_insert_default()
        }
        pub fn effects_opt(&self) -> Option<&str> {
            self.effects.as_ref().map(|field| field as _)
        }
        pub fn set_effects<T: Into<String>>(&mut self, field: T) {
            self.effects = Some(field.into().into());
        }
        pub fn with_effects<T: Into<String>>(mut self, field: T) -> Self {
            self.set_effects(field);
            self
        }
        pub fn signatures(&self) -> &[super::UserSignature] {
            &self.signatures
        }
        pub fn set_signatures(&mut self, field: Vec<super::UserSignature>) {
            self.signatures = field;
        }
        pub fn with_signatures(mut self, field: Vec<super::UserSignature>) -> Self {
            self.set_signatures(field);
            self
        }
    }
    impl super::CircomG1 {
        pub const fn const_default() -> Self {
            Self {
                e0: None,
                e1: None,
                e2: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CircomG1 = super::CircomG1::const_default();
            &DEFAULT
        }
        pub fn e0_opt_mut(&mut self) -> Option<&mut String> {
            self.e0.as_mut().map(|field| field as _)
        }
        pub fn e0_mut(&mut self) -> &mut String {
            self.e0.get_or_insert_default()
        }
        pub fn e0_opt(&self) -> Option<&str> {
            self.e0.as_ref().map(|field| field as _)
        }
        pub fn set_e0<T: Into<String>>(&mut self, field: T) {
            self.e0 = Some(field.into().into());
        }
        pub fn with_e0<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e0(field);
            self
        }
        pub fn e1_opt_mut(&mut self) -> Option<&mut String> {
            self.e1.as_mut().map(|field| field as _)
        }
        pub fn e1_mut(&mut self) -> &mut String {
            self.e1.get_or_insert_default()
        }
        pub fn e1_opt(&self) -> Option<&str> {
            self.e1.as_ref().map(|field| field as _)
        }
        pub fn set_e1<T: Into<String>>(&mut self, field: T) {
            self.e1 = Some(field.into().into());
        }
        pub fn with_e1<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e1(field);
            self
        }
        pub fn e2_opt_mut(&mut self) -> Option<&mut String> {
            self.e2.as_mut().map(|field| field as _)
        }
        pub fn e2_mut(&mut self) -> &mut String {
            self.e2.get_or_insert_default()
        }
        pub fn e2_opt(&self) -> Option<&str> {
            self.e2.as_ref().map(|field| field as _)
        }
        pub fn set_e2<T: Into<String>>(&mut self, field: T) {
            self.e2 = Some(field.into().into());
        }
        pub fn with_e2<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e2(field);
            self
        }
    }
    impl super::CircomG2 {
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
            static DEFAULT: super::CircomG2 = super::CircomG2::const_default();
            &DEFAULT
        }
        pub fn e00_opt_mut(&mut self) -> Option<&mut String> {
            self.e00.as_mut().map(|field| field as _)
        }
        pub fn e00_mut(&mut self) -> &mut String {
            self.e00.get_or_insert_default()
        }
        pub fn e00_opt(&self) -> Option<&str> {
            self.e00.as_ref().map(|field| field as _)
        }
        pub fn set_e00<T: Into<String>>(&mut self, field: T) {
            self.e00 = Some(field.into().into());
        }
        pub fn with_e00<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e00(field);
            self
        }
        pub fn e01_opt_mut(&mut self) -> Option<&mut String> {
            self.e01.as_mut().map(|field| field as _)
        }
        pub fn e01_mut(&mut self) -> &mut String {
            self.e01.get_or_insert_default()
        }
        pub fn e01_opt(&self) -> Option<&str> {
            self.e01.as_ref().map(|field| field as _)
        }
        pub fn set_e01<T: Into<String>>(&mut self, field: T) {
            self.e01 = Some(field.into().into());
        }
        pub fn with_e01<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e01(field);
            self
        }
        pub fn e10_opt_mut(&mut self) -> Option<&mut String> {
            self.e10.as_mut().map(|field| field as _)
        }
        pub fn e10_mut(&mut self) -> &mut String {
            self.e10.get_or_insert_default()
        }
        pub fn e10_opt(&self) -> Option<&str> {
            self.e10.as_ref().map(|field| field as _)
        }
        pub fn set_e10<T: Into<String>>(&mut self, field: T) {
            self.e10 = Some(field.into().into());
        }
        pub fn with_e10<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e10(field);
            self
        }
        pub fn e11_opt_mut(&mut self) -> Option<&mut String> {
            self.e11.as_mut().map(|field| field as _)
        }
        pub fn e11_mut(&mut self) -> &mut String {
            self.e11.get_or_insert_default()
        }
        pub fn e11_opt(&self) -> Option<&str> {
            self.e11.as_ref().map(|field| field as _)
        }
        pub fn set_e11<T: Into<String>>(&mut self, field: T) {
            self.e11 = Some(field.into().into());
        }
        pub fn with_e11<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e11(field);
            self
        }
        pub fn e20_opt_mut(&mut self) -> Option<&mut String> {
            self.e20.as_mut().map(|field| field as _)
        }
        pub fn e20_mut(&mut self) -> &mut String {
            self.e20.get_or_insert_default()
        }
        pub fn e20_opt(&self) -> Option<&str> {
            self.e20.as_ref().map(|field| field as _)
        }
        pub fn set_e20<T: Into<String>>(&mut self, field: T) {
            self.e20 = Some(field.into().into());
        }
        pub fn with_e20<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e20(field);
            self
        }
        pub fn e21_opt_mut(&mut self) -> Option<&mut String> {
            self.e21.as_mut().map(|field| field as _)
        }
        pub fn e21_mut(&mut self) -> &mut String {
            self.e21.get_or_insert_default()
        }
        pub fn e21_opt(&self) -> Option<&str> {
            self.e21.as_ref().map(|field| field as _)
        }
        pub fn set_e21<T: Into<String>>(&mut self, field: T) {
            self.e21 = Some(field.into().into());
        }
        pub fn with_e21<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e21(field);
            self
        }
    }
    impl super::CleverError {
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
            static DEFAULT: super::CleverError = super::CleverError::const_default();
            &DEFAULT
        }
        pub fn error_code_opt_mut(&mut self) -> Option<&mut u64> {
            self.error_code.as_mut().map(|field| field as _)
        }
        pub fn error_code_mut(&mut self) -> &mut u64 {
            self.error_code.get_or_insert_default()
        }
        pub fn error_code_opt(&self) -> Option<u64> {
            self.error_code.as_ref().map(|field| *field)
        }
        pub fn set_error_code<T: Into<u64>>(&mut self, field: T) {
            self.error_code = Some(field.into().into());
        }
        pub fn with_error_code<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_error_code(field);
            self
        }
        pub fn line_number_opt_mut(&mut self) -> Option<&mut u64> {
            self.line_number.as_mut().map(|field| field as _)
        }
        pub fn line_number_mut(&mut self) -> &mut u64 {
            self.line_number.get_or_insert_default()
        }
        pub fn line_number_opt(&self) -> Option<u64> {
            self.line_number.as_ref().map(|field| *field)
        }
        pub fn set_line_number<T: Into<u64>>(&mut self, field: T) {
            self.line_number = Some(field.into().into());
        }
        pub fn with_line_number<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_line_number(field);
            self
        }
        pub fn constant_name_opt_mut(&mut self) -> Option<&mut String> {
            self.constant_name.as_mut().map(|field| field as _)
        }
        pub fn constant_name_mut(&mut self) -> &mut String {
            self.constant_name.get_or_insert_default()
        }
        pub fn constant_name_opt(&self) -> Option<&str> {
            self.constant_name.as_ref().map(|field| field as _)
        }
        pub fn set_constant_name<T: Into<String>>(&mut self, field: T) {
            self.constant_name = Some(field.into().into());
        }
        pub fn with_constant_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_constant_name(field);
            self
        }
        pub fn constant_type_opt_mut(&mut self) -> Option<&mut String> {
            self.constant_type.as_mut().map(|field| field as _)
        }
        pub fn constant_type_mut(&mut self) -> &mut String {
            self.constant_type.get_or_insert_default()
        }
        pub fn constant_type_opt(&self) -> Option<&str> {
            self.constant_type.as_ref().map(|field| field as _)
        }
        pub fn set_constant_type<T: Into<String>>(&mut self, field: T) {
            self.constant_type = Some(field.into().into());
        }
        pub fn with_constant_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_constant_type(field);
            self
        }
        pub fn rendered(&self) -> &str {
            if let Some(super::clever_error::Value::Rendered(field)) = &self.value {
                field as _
            } else {
                ""
            }
        }
        pub fn rendered_opt(&self) -> Option<&str> {
            if let Some(super::clever_error::Value::Rendered(field)) = &self.value {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn rendered_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(super::clever_error::Value::Rendered(field)) = &mut self.value {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn rendered_mut(&mut self) -> &mut String {
            if self.rendered_opt_mut().is_none() {
                self.value = Some(
                    super::clever_error::Value::Rendered(String::default()),
                );
            }
            self.rendered_opt_mut().unwrap()
        }
        pub fn set_rendered<T: Into<String>>(&mut self, field: T) {
            self.value = Some(super::clever_error::Value::Rendered(field.into().into()));
        }
        pub fn with_rendered<T: Into<String>>(mut self, field: T) -> Self {
            self.set_rendered(field);
            self
        }
        pub fn raw(&self) -> &[u8] {
            if let Some(super::clever_error::Value::Raw(field)) = &self.value {
                field as _
            } else {
                &[]
            }
        }
        pub fn raw_opt(&self) -> Option<&[u8]> {
            if let Some(super::clever_error::Value::Raw(field)) = &self.value {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn raw_opt_mut(&mut self) -> Option<&mut ::prost::bytes::Bytes> {
            if let Some(super::clever_error::Value::Raw(field)) = &mut self.value {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn raw_mut(&mut self) -> &mut ::prost::bytes::Bytes {
            if self.raw_opt_mut().is_none() {
                self.value = Some(
                    super::clever_error::Value::Raw(::prost::bytes::Bytes::default()),
                );
            }
            self.raw_opt_mut().unwrap()
        }
        pub fn set_raw<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.value = Some(super::clever_error::Value::Raw(field.into().into()));
        }
        pub fn with_raw<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_raw(field);
            self
        }
    }
    impl super::CoinDenyListError {
        pub const fn const_default() -> Self {
            Self {
                address: None,
                coin_type: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CoinDenyListError = super::CoinDenyListError::const_default();
            &DEFAULT
        }
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field);
            self
        }
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field);
            self
        }
    }
    impl super::CoinMetadata {
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
            static DEFAULT: super::CoinMetadata = super::CoinMetadata::const_default();
            &DEFAULT
        }
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field);
            self
        }
        pub fn decimals_opt_mut(&mut self) -> Option<&mut u32> {
            self.decimals.as_mut().map(|field| field as _)
        }
        pub fn decimals_mut(&mut self) -> &mut u32 {
            self.decimals.get_or_insert_default()
        }
        pub fn decimals_opt(&self) -> Option<u32> {
            self.decimals.as_ref().map(|field| *field)
        }
        pub fn set_decimals<T: Into<u32>>(&mut self, field: T) {
            self.decimals = Some(field.into().into());
        }
        pub fn with_decimals<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_decimals(field);
            self
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
        pub fn symbol_opt_mut(&mut self) -> Option<&mut String> {
            self.symbol.as_mut().map(|field| field as _)
        }
        pub fn symbol_mut(&mut self) -> &mut String {
            self.symbol.get_or_insert_default()
        }
        pub fn symbol_opt(&self) -> Option<&str> {
            self.symbol.as_ref().map(|field| field as _)
        }
        pub fn set_symbol<T: Into<String>>(&mut self, field: T) {
            self.symbol = Some(field.into().into());
        }
        pub fn with_symbol<T: Into<String>>(mut self, field: T) -> Self {
            self.set_symbol(field);
            self
        }
        pub fn description_opt_mut(&mut self) -> Option<&mut String> {
            self.description.as_mut().map(|field| field as _)
        }
        pub fn description_mut(&mut self) -> &mut String {
            self.description.get_or_insert_default()
        }
        pub fn description_opt(&self) -> Option<&str> {
            self.description.as_ref().map(|field| field as _)
        }
        pub fn set_description<T: Into<String>>(&mut self, field: T) {
            self.description = Some(field.into().into());
        }
        pub fn with_description<T: Into<String>>(mut self, field: T) -> Self {
            self.set_description(field);
            self
        }
        pub fn icon_url_opt_mut(&mut self) -> Option<&mut String> {
            self.icon_url.as_mut().map(|field| field as _)
        }
        pub fn icon_url_mut(&mut self) -> &mut String {
            self.icon_url.get_or_insert_default()
        }
        pub fn icon_url_opt(&self) -> Option<&str> {
            self.icon_url.as_ref().map(|field| field as _)
        }
        pub fn set_icon_url<T: Into<String>>(&mut self, field: T) {
            self.icon_url = Some(field.into().into());
        }
        pub fn with_icon_url<T: Into<String>>(mut self, field: T) -> Self {
            self.set_icon_url(field);
            self
        }
        pub fn metadata_cap_id_opt_mut(&mut self) -> Option<&mut String> {
            self.metadata_cap_id.as_mut().map(|field| field as _)
        }
        pub fn metadata_cap_id_mut(&mut self) -> &mut String {
            self.metadata_cap_id.get_or_insert_default()
        }
        pub fn metadata_cap_id_opt(&self) -> Option<&str> {
            self.metadata_cap_id.as_ref().map(|field| field as _)
        }
        pub fn set_metadata_cap_id<T: Into<String>>(&mut self, field: T) {
            self.metadata_cap_id = Some(field.into().into());
        }
        pub fn with_metadata_cap_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_metadata_cap_id(field);
            self
        }
    }
    impl super::CoinTreasury {
        pub const fn const_default() -> Self {
            Self {
                id: None,
                total_supply: None,
                supply_state: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CoinTreasury = super::CoinTreasury::const_default();
            &DEFAULT
        }
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field);
            self
        }
        pub fn total_supply_opt_mut(&mut self) -> Option<&mut u64> {
            self.total_supply.as_mut().map(|field| field as _)
        }
        pub fn total_supply_mut(&mut self) -> &mut u64 {
            self.total_supply.get_or_insert_default()
        }
        pub fn total_supply_opt(&self) -> Option<u64> {
            self.total_supply.as_ref().map(|field| *field)
        }
        pub fn set_total_supply<T: Into<u64>>(&mut self, field: T) {
            self.total_supply = Some(field.into().into());
        }
        pub fn with_total_supply<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_total_supply(field);
            self
        }
    }
    impl super::Command {
        pub const fn const_default() -> Self {
            Self { command: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Command = super::Command::const_default();
            &DEFAULT
        }
        pub fn move_call(&self) -> &super::MoveCall {
            if let Some(super::command::Command::MoveCall(field)) = &self.command {
                field as _
            } else {
                super::MoveCall::default_instance() as _
            }
        }
        pub fn move_call_opt(&self) -> Option<&super::MoveCall> {
            if let Some(super::command::Command::MoveCall(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn move_call_opt_mut(&mut self) -> Option<&mut super::MoveCall> {
            if let Some(super::command::Command::MoveCall(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn move_call_mut(&mut self) -> &mut super::MoveCall {
            if self.move_call_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::MoveCall(super::MoveCall::default()),
                );
            }
            self.move_call_opt_mut().unwrap()
        }
        pub fn set_move_call<T: Into<super::MoveCall>>(&mut self, field: T) {
            self.command = Some(super::command::Command::MoveCall(field.into().into()));
        }
        pub fn with_move_call<T: Into<super::MoveCall>>(mut self, field: T) -> Self {
            self.set_move_call(field);
            self
        }
        pub fn transfer_objects(&self) -> &super::TransferObjects {
            if let Some(super::command::Command::TransferObjects(field)) = &self.command
            {
                field as _
            } else {
                super::TransferObjects::default_instance() as _
            }
        }
        pub fn transfer_objects_opt(&self) -> Option<&super::TransferObjects> {
            if let Some(super::command::Command::TransferObjects(field)) = &self.command
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn transfer_objects_opt_mut(
            &mut self,
        ) -> Option<&mut super::TransferObjects> {
            if let Some(super::command::Command::TransferObjects(field)) = &mut self
                .command
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn transfer_objects_mut(&mut self) -> &mut super::TransferObjects {
            if self.transfer_objects_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::TransferObjects(
                        super::TransferObjects::default(),
                    ),
                );
            }
            self.transfer_objects_opt_mut().unwrap()
        }
        pub fn set_transfer_objects<T: Into<super::TransferObjects>>(
            &mut self,
            field: T,
        ) {
            self.command = Some(
                super::command::Command::TransferObjects(field.into().into()),
            );
        }
        pub fn with_transfer_objects<T: Into<super::TransferObjects>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transfer_objects(field);
            self
        }
        pub fn split_coins(&self) -> &super::SplitCoins {
            if let Some(super::command::Command::SplitCoins(field)) = &self.command {
                field as _
            } else {
                super::SplitCoins::default_instance() as _
            }
        }
        pub fn split_coins_opt(&self) -> Option<&super::SplitCoins> {
            if let Some(super::command::Command::SplitCoins(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn split_coins_opt_mut(&mut self) -> Option<&mut super::SplitCoins> {
            if let Some(super::command::Command::SplitCoins(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn split_coins_mut(&mut self) -> &mut super::SplitCoins {
            if self.split_coins_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::SplitCoins(super::SplitCoins::default()),
                );
            }
            self.split_coins_opt_mut().unwrap()
        }
        pub fn set_split_coins<T: Into<super::SplitCoins>>(&mut self, field: T) {
            self.command = Some(
                super::command::Command::SplitCoins(field.into().into()),
            );
        }
        pub fn with_split_coins<T: Into<super::SplitCoins>>(mut self, field: T) -> Self {
            self.set_split_coins(field);
            self
        }
        pub fn merge_coins(&self) -> &super::MergeCoins {
            if let Some(super::command::Command::MergeCoins(field)) = &self.command {
                field as _
            } else {
                super::MergeCoins::default_instance() as _
            }
        }
        pub fn merge_coins_opt(&self) -> Option<&super::MergeCoins> {
            if let Some(super::command::Command::MergeCoins(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn merge_coins_opt_mut(&mut self) -> Option<&mut super::MergeCoins> {
            if let Some(super::command::Command::MergeCoins(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn merge_coins_mut(&mut self) -> &mut super::MergeCoins {
            if self.merge_coins_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::MergeCoins(super::MergeCoins::default()),
                );
            }
            self.merge_coins_opt_mut().unwrap()
        }
        pub fn set_merge_coins<T: Into<super::MergeCoins>>(&mut self, field: T) {
            self.command = Some(
                super::command::Command::MergeCoins(field.into().into()),
            );
        }
        pub fn with_merge_coins<T: Into<super::MergeCoins>>(mut self, field: T) -> Self {
            self.set_merge_coins(field);
            self
        }
        pub fn publish(&self) -> &super::Publish {
            if let Some(super::command::Command::Publish(field)) = &self.command {
                field as _
            } else {
                super::Publish::default_instance() as _
            }
        }
        pub fn publish_opt(&self) -> Option<&super::Publish> {
            if let Some(super::command::Command::Publish(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn publish_opt_mut(&mut self) -> Option<&mut super::Publish> {
            if let Some(super::command::Command::Publish(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn publish_mut(&mut self) -> &mut super::Publish {
            if self.publish_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::Publish(super::Publish::default()),
                );
            }
            self.publish_opt_mut().unwrap()
        }
        pub fn set_publish<T: Into<super::Publish>>(&mut self, field: T) {
            self.command = Some(super::command::Command::Publish(field.into().into()));
        }
        pub fn with_publish<T: Into<super::Publish>>(mut self, field: T) -> Self {
            self.set_publish(field);
            self
        }
        pub fn make_move_vector(&self) -> &super::MakeMoveVector {
            if let Some(super::command::Command::MakeMoveVector(field)) = &self.command {
                field as _
            } else {
                super::MakeMoveVector::default_instance() as _
            }
        }
        pub fn make_move_vector_opt(&self) -> Option<&super::MakeMoveVector> {
            if let Some(super::command::Command::MakeMoveVector(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn make_move_vector_opt_mut(
            &mut self,
        ) -> Option<&mut super::MakeMoveVector> {
            if let Some(super::command::Command::MakeMoveVector(field)) = &mut self
                .command
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn make_move_vector_mut(&mut self) -> &mut super::MakeMoveVector {
            if self.make_move_vector_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::MakeMoveVector(
                        super::MakeMoveVector::default(),
                    ),
                );
            }
            self.make_move_vector_opt_mut().unwrap()
        }
        pub fn set_make_move_vector<T: Into<super::MakeMoveVector>>(
            &mut self,
            field: T,
        ) {
            self.command = Some(
                super::command::Command::MakeMoveVector(field.into().into()),
            );
        }
        pub fn with_make_move_vector<T: Into<super::MakeMoveVector>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_make_move_vector(field);
            self
        }
        pub fn upgrade(&self) -> &super::Upgrade {
            if let Some(super::command::Command::Upgrade(field)) = &self.command {
                field as _
            } else {
                super::Upgrade::default_instance() as _
            }
        }
        pub fn upgrade_opt(&self) -> Option<&super::Upgrade> {
            if let Some(super::command::Command::Upgrade(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn upgrade_opt_mut(&mut self) -> Option<&mut super::Upgrade> {
            if let Some(super::command::Command::Upgrade(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn upgrade_mut(&mut self) -> &mut super::Upgrade {
            if self.upgrade_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::Upgrade(super::Upgrade::default()),
                );
            }
            self.upgrade_opt_mut().unwrap()
        }
        pub fn set_upgrade<T: Into<super::Upgrade>>(&mut self, field: T) {
            self.command = Some(super::command::Command::Upgrade(field.into().into()));
        }
        pub fn with_upgrade<T: Into<super::Upgrade>>(mut self, field: T) -> Self {
            self.set_upgrade(field);
            self
        }
    }
    impl super::CommandArgumentError {
        pub const fn const_default() -> Self {
            Self {
                argument: None,
                kind: None,
                index_error: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CommandArgumentError = super::CommandArgumentError::const_default();
            &DEFAULT
        }
        pub fn argument_opt_mut(&mut self) -> Option<&mut u32> {
            self.argument.as_mut().map(|field| field as _)
        }
        pub fn argument_mut(&mut self) -> &mut u32 {
            self.argument.get_or_insert_default()
        }
        pub fn argument_opt(&self) -> Option<u32> {
            self.argument.as_ref().map(|field| *field)
        }
        pub fn set_argument<T: Into<u32>>(&mut self, field: T) {
            self.argument = Some(field.into().into());
        }
        pub fn with_argument<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_argument(field);
            self
        }
        pub fn index_error(&self) -> &super::IndexError {
            self.index_error
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::IndexError::default_instance() as _)
        }
        pub fn index_error_opt_mut(&mut self) -> Option<&mut super::IndexError> {
            self.index_error.as_mut().map(|field| field as _)
        }
        pub fn index_error_mut(&mut self) -> &mut super::IndexError {
            self.index_error.get_or_insert_default()
        }
        pub fn index_error_opt(&self) -> Option<&super::IndexError> {
            self.index_error.as_ref().map(|field| field as _)
        }
        pub fn set_index_error<T: Into<super::IndexError>>(&mut self, field: T) {
            self.index_error = Some(field.into().into());
        }
        pub fn with_index_error<T: Into<super::IndexError>>(mut self, field: T) -> Self {
            self.set_index_error(field);
            self
        }
    }
    impl super::CommandOutput {
        pub const fn const_default() -> Self {
            Self {
                argument: None,
                value: None,
                json: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CommandOutput = super::CommandOutput::const_default();
            &DEFAULT
        }
        pub fn argument(&self) -> &super::Argument {
            self.argument
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Argument::default_instance() as _)
        }
        pub fn argument_opt_mut(&mut self) -> Option<&mut super::Argument> {
            self.argument.as_mut().map(|field| field as _)
        }
        pub fn argument_mut(&mut self) -> &mut super::Argument {
            self.argument.get_or_insert_default()
        }
        pub fn argument_opt(&self) -> Option<&super::Argument> {
            self.argument.as_ref().map(|field| field as _)
        }
        pub fn set_argument<T: Into<super::Argument>>(&mut self, field: T) {
            self.argument = Some(field.into().into());
        }
        pub fn with_argument<T: Into<super::Argument>>(mut self, field: T) -> Self {
            self.set_argument(field);
            self
        }
        pub fn value(&self) -> &super::Bcs {
            self.value
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        pub fn value_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.value.as_mut().map(|field| field as _)
        }
        pub fn value_mut(&mut self) -> &mut super::Bcs {
            self.value.get_or_insert_default()
        }
        pub fn value_opt(&self) -> Option<&super::Bcs> {
            self.value.as_ref().map(|field| field as _)
        }
        pub fn set_value<T: Into<super::Bcs>>(&mut self, field: T) {
            self.value = Some(field.into().into());
        }
        pub fn with_value<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_value(field);
            self
        }
        pub fn json_opt_mut(&mut self) -> Option<&mut ::prost_types::Value> {
            self.json.as_mut().map(|field| field as _)
        }
        pub fn json_mut(&mut self) -> &mut ::prost_types::Value {
            self.json.get_or_insert_default()
        }
        pub fn json_opt(&self) -> Option<&::prost_types::Value> {
            self.json.as_ref().map(|field| field as _)
        }
        pub fn set_json<T: Into<::prost_types::Value>>(&mut self, field: T) {
            self.json = Some(field.into().into());
        }
        pub fn with_json<T: Into<::prost_types::Value>>(mut self, field: T) -> Self {
            self.set_json(field);
            self
        }
    }
    impl super::CommandResult {
        pub const fn const_default() -> Self {
            Self {
                return_values: Vec::new(),
                mutated_by_ref: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CommandResult = super::CommandResult::const_default();
            &DEFAULT
        }
        pub fn return_values(&self) -> &[super::CommandOutput] {
            &self.return_values
        }
        pub fn set_return_values(&mut self, field: Vec<super::CommandOutput>) {
            self.return_values = field;
        }
        pub fn with_return_values(mut self, field: Vec<super::CommandOutput>) -> Self {
            self.set_return_values(field);
            self
        }
        pub fn mutated_by_ref(&self) -> &[super::CommandOutput] {
            &self.mutated_by_ref
        }
        pub fn set_mutated_by_ref(&mut self, field: Vec<super::CommandOutput>) {
            self.mutated_by_ref = field;
        }
        pub fn with_mutated_by_ref(mut self, field: Vec<super::CommandOutput>) -> Self {
            self.set_mutated_by_ref(field);
            self
        }
    }
    impl super::CongestedObjects {
        pub const fn const_default() -> Self {
            Self { objects: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CongestedObjects = super::CongestedObjects::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[String] {
            &self.objects
        }
        pub fn set_objects(&mut self, field: Vec<String>) {
            self.objects = field;
        }
        pub fn with_objects(mut self, field: Vec<String>) -> Self {
            self.set_objects(field);
            self
        }
    }
    impl super::ConsensusCommitPrologue {
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
            static DEFAULT: super::ConsensusCommitPrologue = super::ConsensusCommitPrologue::const_default();
            &DEFAULT
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn round_opt_mut(&mut self) -> Option<&mut u64> {
            self.round.as_mut().map(|field| field as _)
        }
        pub fn round_mut(&mut self) -> &mut u64 {
            self.round.get_or_insert_default()
        }
        pub fn round_opt(&self) -> Option<u64> {
            self.round.as_ref().map(|field| *field)
        }
        pub fn set_round<T: Into<u64>>(&mut self, field: T) {
            self.round = Some(field.into().into());
        }
        pub fn with_round<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_round(field);
            self
        }
        pub fn commit_timestamp_opt_mut(
            &mut self,
        ) -> Option<&mut ::prost_types::Timestamp> {
            self.commit_timestamp.as_mut().map(|field| field as _)
        }
        pub fn commit_timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.commit_timestamp.get_or_insert_default()
        }
        pub fn commit_timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.commit_timestamp.as_ref().map(|field| field as _)
        }
        pub fn set_commit_timestamp<T: Into<::prost_types::Timestamp>>(
            &mut self,
            field: T,
        ) {
            self.commit_timestamp = Some(field.into().into());
        }
        pub fn with_commit_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_commit_timestamp(field);
            self
        }
        pub fn consensus_commit_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.consensus_commit_digest.as_mut().map(|field| field as _)
        }
        pub fn consensus_commit_digest_mut(&mut self) -> &mut String {
            self.consensus_commit_digest.get_or_insert_default()
        }
        pub fn consensus_commit_digest_opt(&self) -> Option<&str> {
            self.consensus_commit_digest.as_ref().map(|field| field as _)
        }
        pub fn set_consensus_commit_digest<T: Into<String>>(&mut self, field: T) {
            self.consensus_commit_digest = Some(field.into().into());
        }
        pub fn with_consensus_commit_digest<T: Into<String>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_consensus_commit_digest(field);
            self
        }
        pub fn sub_dag_index_opt_mut(&mut self) -> Option<&mut u64> {
            self.sub_dag_index.as_mut().map(|field| field as _)
        }
        pub fn sub_dag_index_mut(&mut self) -> &mut u64 {
            self.sub_dag_index.get_or_insert_default()
        }
        pub fn sub_dag_index_opt(&self) -> Option<u64> {
            self.sub_dag_index.as_ref().map(|field| *field)
        }
        pub fn set_sub_dag_index<T: Into<u64>>(&mut self, field: T) {
            self.sub_dag_index = Some(field.into().into());
        }
        pub fn with_sub_dag_index<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_sub_dag_index(field);
            self
        }
        pub fn consensus_determined_version_assignments(
            &self,
        ) -> &super::ConsensusDeterminedVersionAssignments {
            self.consensus_determined_version_assignments
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::ConsensusDeterminedVersionAssignments::default_instance() as _
                })
        }
        pub fn consensus_determined_version_assignments_opt_mut(
            &mut self,
        ) -> Option<&mut super::ConsensusDeterminedVersionAssignments> {
            self.consensus_determined_version_assignments
                .as_mut()
                .map(|field| field as _)
        }
        pub fn consensus_determined_version_assignments_mut(
            &mut self,
        ) -> &mut super::ConsensusDeterminedVersionAssignments {
            self.consensus_determined_version_assignments.get_or_insert_default()
        }
        pub fn consensus_determined_version_assignments_opt(
            &self,
        ) -> Option<&super::ConsensusDeterminedVersionAssignments> {
            self.consensus_determined_version_assignments
                .as_ref()
                .map(|field| field as _)
        }
        pub fn set_consensus_determined_version_assignments<
            T: Into<super::ConsensusDeterminedVersionAssignments>,
        >(&mut self, field: T) {
            self.consensus_determined_version_assignments = Some(field.into().into());
        }
        pub fn with_consensus_determined_version_assignments<
            T: Into<super::ConsensusDeterminedVersionAssignments>,
        >(mut self, field: T) -> Self {
            self.set_consensus_determined_version_assignments(field);
            self
        }
        pub fn additional_state_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.additional_state_digest.as_mut().map(|field| field as _)
        }
        pub fn additional_state_digest_mut(&mut self) -> &mut String {
            self.additional_state_digest.get_or_insert_default()
        }
        pub fn additional_state_digest_opt(&self) -> Option<&str> {
            self.additional_state_digest.as_ref().map(|field| field as _)
        }
        pub fn set_additional_state_digest<T: Into<String>>(&mut self, field: T) {
            self.additional_state_digest = Some(field.into().into());
        }
        pub fn with_additional_state_digest<T: Into<String>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_additional_state_digest(field);
            self
        }
    }
    impl super::ConsensusDeterminedVersionAssignments {
        pub const fn const_default() -> Self {
            Self {
                version: None,
                canceled_transactions: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ConsensusDeterminedVersionAssignments = super::ConsensusDeterminedVersionAssignments::const_default();
            &DEFAULT
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut i32> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut i32 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<i32> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<i32>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<i32>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn canceled_transactions(&self) -> &[super::CanceledTransaction] {
            &self.canceled_transactions
        }
        pub fn set_canceled_transactions(
            &mut self,
            field: Vec<super::CanceledTransaction>,
        ) {
            self.canceled_transactions = field;
        }
        pub fn with_canceled_transactions(
            mut self,
            field: Vec<super::CanceledTransaction>,
        ) -> Self {
            self.set_canceled_transactions(field);
            self
        }
    }
    impl super::DatatypeDescriptor {
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
            static DEFAULT: super::DatatypeDescriptor = super::DatatypeDescriptor::const_default();
            &DEFAULT
        }
        pub fn type_name_opt_mut(&mut self) -> Option<&mut String> {
            self.type_name.as_mut().map(|field| field as _)
        }
        pub fn type_name_mut(&mut self) -> &mut String {
            self.type_name.get_or_insert_default()
        }
        pub fn type_name_opt(&self) -> Option<&str> {
            self.type_name.as_ref().map(|field| field as _)
        }
        pub fn set_type_name<T: Into<String>>(&mut self, field: T) {
            self.type_name = Some(field.into().into());
        }
        pub fn with_type_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_type_name(field);
            self
        }
        pub fn defining_id_opt_mut(&mut self) -> Option<&mut String> {
            self.defining_id.as_mut().map(|field| field as _)
        }
        pub fn defining_id_mut(&mut self) -> &mut String {
            self.defining_id.get_or_insert_default()
        }
        pub fn defining_id_opt(&self) -> Option<&str> {
            self.defining_id.as_ref().map(|field| field as _)
        }
        pub fn set_defining_id<T: Into<String>>(&mut self, field: T) {
            self.defining_id = Some(field.into().into());
        }
        pub fn with_defining_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_defining_id(field);
            self
        }
        pub fn module_opt_mut(&mut self) -> Option<&mut String> {
            self.module.as_mut().map(|field| field as _)
        }
        pub fn module_mut(&mut self) -> &mut String {
            self.module.get_or_insert_default()
        }
        pub fn module_opt(&self) -> Option<&str> {
            self.module.as_ref().map(|field| field as _)
        }
        pub fn set_module<T: Into<String>>(&mut self, field: T) {
            self.module = Some(field.into().into());
        }
        pub fn with_module<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module(field);
            self
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
        pub fn type_parameters(&self) -> &[super::TypeParameter] {
            &self.type_parameters
        }
        pub fn set_type_parameters(&mut self, field: Vec<super::TypeParameter>) {
            self.type_parameters = field;
        }
        pub fn with_type_parameters(mut self, field: Vec<super::TypeParameter>) -> Self {
            self.set_type_parameters(field);
            self
        }
        pub fn fields(&self) -> &[super::FieldDescriptor] {
            &self.fields
        }
        pub fn set_fields(&mut self, field: Vec<super::FieldDescriptor>) {
            self.fields = field;
        }
        pub fn with_fields(mut self, field: Vec<super::FieldDescriptor>) -> Self {
            self.set_fields(field);
            self
        }
        pub fn variants(&self) -> &[super::VariantDescriptor] {
            &self.variants
        }
        pub fn set_variants(&mut self, field: Vec<super::VariantDescriptor>) {
            self.variants = field;
        }
        pub fn with_variants(mut self, field: Vec<super::VariantDescriptor>) -> Self {
            self.set_variants(field);
            self
        }
    }
    impl super::DynamicField {
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
            static DEFAULT: super::DynamicField = super::DynamicField::const_default();
            &DEFAULT
        }
        pub fn parent_opt_mut(&mut self) -> Option<&mut String> {
            self.parent.as_mut().map(|field| field as _)
        }
        pub fn parent_mut(&mut self) -> &mut String {
            self.parent.get_or_insert_default()
        }
        pub fn parent_opt(&self) -> Option<&str> {
            self.parent.as_ref().map(|field| field as _)
        }
        pub fn set_parent<T: Into<String>>(&mut self, field: T) {
            self.parent = Some(field.into().into());
        }
        pub fn with_parent<T: Into<String>>(mut self, field: T) -> Self {
            self.set_parent(field);
            self
        }
        pub fn field_id_opt_mut(&mut self) -> Option<&mut String> {
            self.field_id.as_mut().map(|field| field as _)
        }
        pub fn field_id_mut(&mut self) -> &mut String {
            self.field_id.get_or_insert_default()
        }
        pub fn field_id_opt(&self) -> Option<&str> {
            self.field_id.as_ref().map(|field| field as _)
        }
        pub fn set_field_id<T: Into<String>>(&mut self, field: T) {
            self.field_id = Some(field.into().into());
        }
        pub fn with_field_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_field_id(field);
            self
        }
        pub fn name_type_opt_mut(&mut self) -> Option<&mut String> {
            self.name_type.as_mut().map(|field| field as _)
        }
        pub fn name_type_mut(&mut self) -> &mut String {
            self.name_type.get_or_insert_default()
        }
        pub fn name_type_opt(&self) -> Option<&str> {
            self.name_type.as_ref().map(|field| field as _)
        }
        pub fn set_name_type<T: Into<String>>(&mut self, field: T) {
            self.name_type = Some(field.into().into());
        }
        pub fn with_name_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name_type(field);
            self
        }
        pub fn name_value_opt(&self) -> Option<&[u8]> {
            self.name_value.as_ref().map(|field| field as _)
        }
        pub fn set_name_value<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.name_value = Some(field.into().into());
        }
        pub fn with_name_value<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_name_value(field);
            self
        }
        pub fn value_type_opt_mut(&mut self) -> Option<&mut String> {
            self.value_type.as_mut().map(|field| field as _)
        }
        pub fn value_type_mut(&mut self) -> &mut String {
            self.value_type.get_or_insert_default()
        }
        pub fn value_type_opt(&self) -> Option<&str> {
            self.value_type.as_ref().map(|field| field as _)
        }
        pub fn set_value_type<T: Into<String>>(&mut self, field: T) {
            self.value_type = Some(field.into().into());
        }
        pub fn with_value_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_value_type(field);
            self
        }
        pub fn dynamic_object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.dynamic_object_id.as_mut().map(|field| field as _)
        }
        pub fn dynamic_object_id_mut(&mut self) -> &mut String {
            self.dynamic_object_id.get_or_insert_default()
        }
        pub fn dynamic_object_id_opt(&self) -> Option<&str> {
            self.dynamic_object_id.as_ref().map(|field| field as _)
        }
        pub fn set_dynamic_object_id<T: Into<String>>(&mut self, field: T) {
            self.dynamic_object_id = Some(field.into().into());
        }
        pub fn with_dynamic_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_dynamic_object_id(field);
            self
        }
        pub fn object(&self) -> &super::Object {
            self.object
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Object::default_instance() as _)
        }
        pub fn object_opt_mut(&mut self) -> Option<&mut super::Object> {
            self.object.as_mut().map(|field| field as _)
        }
        pub fn object_mut(&mut self) -> &mut super::Object {
            self.object.get_or_insert_default()
        }
        pub fn object_opt(&self) -> Option<&super::Object> {
            self.object.as_ref().map(|field| field as _)
        }
        pub fn set_object<T: Into<super::Object>>(&mut self, field: T) {
            self.object = Some(field.into().into());
        }
        pub fn with_object<T: Into<super::Object>>(mut self, field: T) -> Self {
            self.set_object(field);
            self
        }
    }
    impl super::EndOfEpochData {
        pub const fn const_default() -> Self {
            Self {
                next_epoch_committee: Vec::new(),
                next_epoch_protocol_version: None,
                epoch_commitments: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EndOfEpochData = super::EndOfEpochData::const_default();
            &DEFAULT
        }
        pub fn next_epoch_committee(&self) -> &[super::ValidatorCommitteeMember] {
            &self.next_epoch_committee
        }
        pub fn set_next_epoch_committee(
            &mut self,
            field: Vec<super::ValidatorCommitteeMember>,
        ) {
            self.next_epoch_committee = field;
        }
        pub fn with_next_epoch_committee(
            mut self,
            field: Vec<super::ValidatorCommitteeMember>,
        ) -> Self {
            self.set_next_epoch_committee(field);
            self
        }
        pub fn next_epoch_protocol_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.next_epoch_protocol_version.as_mut().map(|field| field as _)
        }
        pub fn next_epoch_protocol_version_mut(&mut self) -> &mut u64 {
            self.next_epoch_protocol_version.get_or_insert_default()
        }
        pub fn next_epoch_protocol_version_opt(&self) -> Option<u64> {
            self.next_epoch_protocol_version.as_ref().map(|field| *field)
        }
        pub fn set_next_epoch_protocol_version<T: Into<u64>>(&mut self, field: T) {
            self.next_epoch_protocol_version = Some(field.into().into());
        }
        pub fn with_next_epoch_protocol_version<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_protocol_version(field);
            self
        }
        pub fn epoch_commitments(&self) -> &[super::CheckpointCommitment] {
            &self.epoch_commitments
        }
        pub fn set_epoch_commitments(
            &mut self,
            field: Vec<super::CheckpointCommitment>,
        ) {
            self.epoch_commitments = field;
        }
        pub fn with_epoch_commitments(
            mut self,
            field: Vec<super::CheckpointCommitment>,
        ) -> Self {
            self.set_epoch_commitments(field);
            self
        }
    }
    impl super::EndOfEpochTransaction {
        pub const fn const_default() -> Self {
            Self { transactions: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EndOfEpochTransaction = super::EndOfEpochTransaction::const_default();
            &DEFAULT
        }
        pub fn transactions(&self) -> &[super::EndOfEpochTransactionKind] {
            &self.transactions
        }
        pub fn set_transactions(
            &mut self,
            field: Vec<super::EndOfEpochTransactionKind>,
        ) {
            self.transactions = field;
        }
        pub fn with_transactions(
            mut self,
            field: Vec<super::EndOfEpochTransactionKind>,
        ) -> Self {
            self.set_transactions(field);
            self
        }
    }
    impl super::EndOfEpochTransactionKind {
        pub const fn const_default() -> Self {
            Self { kind: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EndOfEpochTransactionKind = super::EndOfEpochTransactionKind::const_default();
            &DEFAULT
        }
        pub fn change_epoch(&self) -> &super::ChangeEpoch {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::ChangeEpoch(field),
            ) = &self.kind
            {
                field as _
            } else {
                super::ChangeEpoch::default_instance() as _
            }
        }
        pub fn change_epoch_opt(&self) -> Option<&super::ChangeEpoch> {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::ChangeEpoch(field),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn change_epoch_opt_mut(&mut self) -> Option<&mut super::ChangeEpoch> {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::ChangeEpoch(field),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn change_epoch_mut(&mut self) -> &mut super::ChangeEpoch {
            if self.change_epoch_opt_mut().is_none() {
                self.kind = Some(
                    super::end_of_epoch_transaction_kind::Kind::ChangeEpoch(
                        super::ChangeEpoch::default(),
                    ),
                );
            }
            self.change_epoch_opt_mut().unwrap()
        }
        pub fn set_change_epoch<T: Into<super::ChangeEpoch>>(&mut self, field: T) {
            self.kind = Some(
                super::end_of_epoch_transaction_kind::Kind::ChangeEpoch(
                    field.into().into(),
                ),
            );
        }
        pub fn with_change_epoch<T: Into<super::ChangeEpoch>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_change_epoch(field);
            self
        }
        pub fn authenticator_state_expire(&self) -> &super::AuthenticatorStateExpire {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::AuthenticatorStateExpire(
                    field,
                ),
            ) = &self.kind
            {
                field as _
            } else {
                super::AuthenticatorStateExpire::default_instance() as _
            }
        }
        pub fn authenticator_state_expire_opt(
            &self,
        ) -> Option<&super::AuthenticatorStateExpire> {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::AuthenticatorStateExpire(
                    field,
                ),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn authenticator_state_expire_opt_mut(
            &mut self,
        ) -> Option<&mut super::AuthenticatorStateExpire> {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::AuthenticatorStateExpire(
                    field,
                ),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn authenticator_state_expire_mut(
            &mut self,
        ) -> &mut super::AuthenticatorStateExpire {
            if self.authenticator_state_expire_opt_mut().is_none() {
                self.kind = Some(
                    super::end_of_epoch_transaction_kind::Kind::AuthenticatorStateExpire(
                        super::AuthenticatorStateExpire::default(),
                    ),
                );
            }
            self.authenticator_state_expire_opt_mut().unwrap()
        }
        pub fn set_authenticator_state_expire<T: Into<super::AuthenticatorStateExpire>>(
            &mut self,
            field: T,
        ) {
            self.kind = Some(
                super::end_of_epoch_transaction_kind::Kind::AuthenticatorStateExpire(
                    field.into().into(),
                ),
            );
        }
        pub fn with_authenticator_state_expire<T: Into<super::AuthenticatorStateExpire>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_authenticator_state_expire(field);
            self
        }
        pub fn execution_time_observations(&self) -> &super::ExecutionTimeObservations {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::ExecutionTimeObservations(
                    field,
                ),
            ) = &self.kind
            {
                field as _
            } else {
                super::ExecutionTimeObservations::default_instance() as _
            }
        }
        pub fn execution_time_observations_opt(
            &self,
        ) -> Option<&super::ExecutionTimeObservations> {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::ExecutionTimeObservations(
                    field,
                ),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn execution_time_observations_opt_mut(
            &mut self,
        ) -> Option<&mut super::ExecutionTimeObservations> {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::ExecutionTimeObservations(
                    field,
                ),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn execution_time_observations_mut(
            &mut self,
        ) -> &mut super::ExecutionTimeObservations {
            if self.execution_time_observations_opt_mut().is_none() {
                self.kind = Some(
                    super::end_of_epoch_transaction_kind::Kind::ExecutionTimeObservations(
                        super::ExecutionTimeObservations::default(),
                    ),
                );
            }
            self.execution_time_observations_opt_mut().unwrap()
        }
        pub fn set_execution_time_observations<
            T: Into<super::ExecutionTimeObservations>,
        >(&mut self, field: T) {
            self.kind = Some(
                super::end_of_epoch_transaction_kind::Kind::ExecutionTimeObservations(
                    field.into().into(),
                ),
            );
        }
        pub fn with_execution_time_observations<
            T: Into<super::ExecutionTimeObservations>,
        >(mut self, field: T) -> Self {
            self.set_execution_time_observations(field);
            self
        }
        pub fn bridge_state_create(&self) -> &str {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::BridgeStateCreate(field),
            ) = &self.kind
            {
                field as _
            } else {
                ""
            }
        }
        pub fn bridge_state_create_opt(&self) -> Option<&str> {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::BridgeStateCreate(field),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn bridge_state_create_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::BridgeStateCreate(field),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn bridge_state_create_mut(&mut self) -> &mut String {
            if self.bridge_state_create_opt_mut().is_none() {
                self.kind = Some(
                    super::end_of_epoch_transaction_kind::Kind::BridgeStateCreate(
                        String::default(),
                    ),
                );
            }
            self.bridge_state_create_opt_mut().unwrap()
        }
        pub fn set_bridge_state_create<T: Into<String>>(&mut self, field: T) {
            self.kind = Some(
                super::end_of_epoch_transaction_kind::Kind::BridgeStateCreate(
                    field.into().into(),
                ),
            );
        }
        pub fn with_bridge_state_create<T: Into<String>>(mut self, field: T) -> Self {
            self.set_bridge_state_create(field);
            self
        }
        pub fn bridge_committee_init(&self) -> u64 {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::BridgeCommitteeInit(field),
            ) = &self.kind
            {
                *field
            } else {
                0u64
            }
        }
        pub fn bridge_committee_init_opt(&self) -> Option<u64> {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::BridgeCommitteeInit(field),
            ) = &self.kind
            {
                Some(*field)
            } else {
                None
            }
        }
        pub fn bridge_committee_init_opt_mut(&mut self) -> Option<&mut u64> {
            if let Some(
                super::end_of_epoch_transaction_kind::Kind::BridgeCommitteeInit(field),
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
                    super::end_of_epoch_transaction_kind::Kind::BridgeCommitteeInit(
                        u64::default(),
                    ),
                );
            }
            self.bridge_committee_init_opt_mut().unwrap()
        }
        pub fn set_bridge_committee_init<T: Into<u64>>(&mut self, field: T) {
            self.kind = Some(
                super::end_of_epoch_transaction_kind::Kind::BridgeCommitteeInit(
                    field.into().into(),
                ),
            );
        }
        pub fn with_bridge_committee_init<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_bridge_committee_init(field);
            self
        }
    }
    impl super::Epoch {
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
            static DEFAULT: super::Epoch = super::Epoch::const_default();
            &DEFAULT
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn committee(&self) -> &super::ValidatorCommittee {
            self.committee
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ValidatorCommittee::default_instance() as _)
        }
        pub fn committee_opt_mut(&mut self) -> Option<&mut super::ValidatorCommittee> {
            self.committee.as_mut().map(|field| field as _)
        }
        pub fn committee_mut(&mut self) -> &mut super::ValidatorCommittee {
            self.committee.get_or_insert_default()
        }
        pub fn committee_opt(&self) -> Option<&super::ValidatorCommittee> {
            self.committee.as_ref().map(|field| field as _)
        }
        pub fn set_committee<T: Into<super::ValidatorCommittee>>(&mut self, field: T) {
            self.committee = Some(field.into().into());
        }
        pub fn with_committee<T: Into<super::ValidatorCommittee>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_committee(field);
            self
        }
        pub fn system_state(&self) -> &super::SystemState {
            self.system_state
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::SystemState::default_instance() as _)
        }
        pub fn system_state_opt_mut(&mut self) -> Option<&mut super::SystemState> {
            self.system_state.as_mut().map(|field| field as _)
        }
        pub fn system_state_mut(&mut self) -> &mut super::SystemState {
            self.system_state.get_or_insert_default()
        }
        pub fn system_state_opt(&self) -> Option<&super::SystemState> {
            self.system_state.as_ref().map(|field| field as _)
        }
        pub fn set_system_state<T: Into<super::SystemState>>(&mut self, field: T) {
            self.system_state = Some(field.into().into());
        }
        pub fn with_system_state<T: Into<super::SystemState>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_system_state(field);
            self
        }
        pub fn first_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.first_checkpoint.as_mut().map(|field| field as _)
        }
        pub fn first_checkpoint_mut(&mut self) -> &mut u64 {
            self.first_checkpoint.get_or_insert_default()
        }
        pub fn first_checkpoint_opt(&self) -> Option<u64> {
            self.first_checkpoint.as_ref().map(|field| *field)
        }
        pub fn set_first_checkpoint<T: Into<u64>>(&mut self, field: T) {
            self.first_checkpoint = Some(field.into().into());
        }
        pub fn with_first_checkpoint<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_first_checkpoint(field);
            self
        }
        pub fn last_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.last_checkpoint.as_mut().map(|field| field as _)
        }
        pub fn last_checkpoint_mut(&mut self) -> &mut u64 {
            self.last_checkpoint.get_or_insert_default()
        }
        pub fn last_checkpoint_opt(&self) -> Option<u64> {
            self.last_checkpoint.as_ref().map(|field| *field)
        }
        pub fn set_last_checkpoint<T: Into<u64>>(&mut self, field: T) {
            self.last_checkpoint = Some(field.into().into());
        }
        pub fn with_last_checkpoint<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_last_checkpoint(field);
            self
        }
        pub fn start_opt_mut(&mut self) -> Option<&mut ::prost_types::Timestamp> {
            self.start.as_mut().map(|field| field as _)
        }
        pub fn start_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.start.get_or_insert_default()
        }
        pub fn start_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.start.as_ref().map(|field| field as _)
        }
        pub fn set_start<T: Into<::prost_types::Timestamp>>(&mut self, field: T) {
            self.start = Some(field.into().into());
        }
        pub fn with_start<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_start(field);
            self
        }
        pub fn end_opt_mut(&mut self) -> Option<&mut ::prost_types::Timestamp> {
            self.end.as_mut().map(|field| field as _)
        }
        pub fn end_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.end.get_or_insert_default()
        }
        pub fn end_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.end.as_ref().map(|field| field as _)
        }
        pub fn set_end<T: Into<::prost_types::Timestamp>>(&mut self, field: T) {
            self.end = Some(field.into().into());
        }
        pub fn with_end<T: Into<::prost_types::Timestamp>>(mut self, field: T) -> Self {
            self.set_end(field);
            self
        }
        pub fn reference_gas_price_opt_mut(&mut self) -> Option<&mut u64> {
            self.reference_gas_price.as_mut().map(|field| field as _)
        }
        pub fn reference_gas_price_mut(&mut self) -> &mut u64 {
            self.reference_gas_price.get_or_insert_default()
        }
        pub fn reference_gas_price_opt(&self) -> Option<u64> {
            self.reference_gas_price.as_ref().map(|field| *field)
        }
        pub fn set_reference_gas_price<T: Into<u64>>(&mut self, field: T) {
            self.reference_gas_price = Some(field.into().into());
        }
        pub fn with_reference_gas_price<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_reference_gas_price(field);
            self
        }
        pub fn protocol_config(&self) -> &super::ProtocolConfig {
            self.protocol_config
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ProtocolConfig::default_instance() as _)
        }
        pub fn protocol_config_opt_mut(&mut self) -> Option<&mut super::ProtocolConfig> {
            self.protocol_config.as_mut().map(|field| field as _)
        }
        pub fn protocol_config_mut(&mut self) -> &mut super::ProtocolConfig {
            self.protocol_config.get_or_insert_default()
        }
        pub fn protocol_config_opt(&self) -> Option<&super::ProtocolConfig> {
            self.protocol_config.as_ref().map(|field| field as _)
        }
        pub fn set_protocol_config<T: Into<super::ProtocolConfig>>(&mut self, field: T) {
            self.protocol_config = Some(field.into().into());
        }
        pub fn with_protocol_config<T: Into<super::ProtocolConfig>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_protocol_config(field);
            self
        }
    }
    impl super::Event {
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
            static DEFAULT: super::Event = super::Event::const_default();
            &DEFAULT
        }
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field);
            self
        }
        pub fn module_opt_mut(&mut self) -> Option<&mut String> {
            self.module.as_mut().map(|field| field as _)
        }
        pub fn module_mut(&mut self) -> &mut String {
            self.module.get_or_insert_default()
        }
        pub fn module_opt(&self) -> Option<&str> {
            self.module.as_ref().map(|field| field as _)
        }
        pub fn set_module<T: Into<String>>(&mut self, field: T) {
            self.module = Some(field.into().into());
        }
        pub fn with_module<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module(field);
            self
        }
        pub fn sender_opt_mut(&mut self) -> Option<&mut String> {
            self.sender.as_mut().map(|field| field as _)
        }
        pub fn sender_mut(&mut self) -> &mut String {
            self.sender.get_or_insert_default()
        }
        pub fn sender_opt(&self) -> Option<&str> {
            self.sender.as_ref().map(|field| field as _)
        }
        pub fn set_sender<T: Into<String>>(&mut self, field: T) {
            self.sender = Some(field.into().into());
        }
        pub fn with_sender<T: Into<String>>(mut self, field: T) -> Self {
            self.set_sender(field);
            self
        }
        pub fn event_type_opt_mut(&mut self) -> Option<&mut String> {
            self.event_type.as_mut().map(|field| field as _)
        }
        pub fn event_type_mut(&mut self) -> &mut String {
            self.event_type.get_or_insert_default()
        }
        pub fn event_type_opt(&self) -> Option<&str> {
            self.event_type.as_ref().map(|field| field as _)
        }
        pub fn set_event_type<T: Into<String>>(&mut self, field: T) {
            self.event_type = Some(field.into().into());
        }
        pub fn with_event_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_event_type(field);
            self
        }
        pub fn contents(&self) -> &super::Bcs {
            self.contents
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        pub fn contents_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.contents.as_mut().map(|field| field as _)
        }
        pub fn contents_mut(&mut self) -> &mut super::Bcs {
            self.contents.get_or_insert_default()
        }
        pub fn contents_opt(&self) -> Option<&super::Bcs> {
            self.contents.as_ref().map(|field| field as _)
        }
        pub fn set_contents<T: Into<super::Bcs>>(&mut self, field: T) {
            self.contents = Some(field.into().into());
        }
        pub fn with_contents<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_contents(field);
            self
        }
        pub fn json_opt_mut(&mut self) -> Option<&mut ::prost_types::Value> {
            self.json.as_mut().map(|field| field as _)
        }
        pub fn json_mut(&mut self) -> &mut ::prost_types::Value {
            self.json.get_or_insert_default()
        }
        pub fn json_opt(&self) -> Option<&::prost_types::Value> {
            self.json.as_ref().map(|field| field as _)
        }
        pub fn set_json<T: Into<::prost_types::Value>>(&mut self, field: T) {
            self.json = Some(field.into().into());
        }
        pub fn with_json<T: Into<::prost_types::Value>>(mut self, field: T) -> Self {
            self.set_json(field);
            self
        }
    }
    impl super::ExecuteTransactionRequest {
        pub const fn const_default() -> Self {
            Self {
                transaction: None,
                signatures: Vec::new(),
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ExecuteTransactionRequest = super::ExecuteTransactionRequest::const_default();
            &DEFAULT
        }
        pub fn transaction(&self) -> &super::Transaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Transaction::default_instance() as _)
        }
        pub fn transaction_opt_mut(&mut self) -> Option<&mut super::Transaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut super::Transaction {
            self.transaction.get_or_insert_default()
        }
        pub fn transaction_opt(&self) -> Option<&super::Transaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn set_transaction<T: Into<super::Transaction>>(&mut self, field: T) {
            self.transaction = Some(field.into().into());
        }
        pub fn with_transaction<T: Into<super::Transaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field);
            self
        }
        pub fn signatures(&self) -> &[super::UserSignature] {
            &self.signatures
        }
        pub fn set_signatures(&mut self, field: Vec<super::UserSignature>) {
            self.signatures = field;
        }
        pub fn with_signatures(mut self, field: Vec<super::UserSignature>) -> Self {
            self.set_signatures(field);
            self
        }
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field);
            self
        }
    }
    impl super::ExecuteTransactionResponse {
        pub const fn const_default() -> Self {
            Self {
                finality: None,
                transaction: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ExecuteTransactionResponse = super::ExecuteTransactionResponse::const_default();
            &DEFAULT
        }
        pub fn finality(&self) -> &super::TransactionFinality {
            self.finality
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::TransactionFinality::default_instance() as _)
        }
        pub fn finality_opt_mut(&mut self) -> Option<&mut super::TransactionFinality> {
            self.finality.as_mut().map(|field| field as _)
        }
        pub fn finality_mut(&mut self) -> &mut super::TransactionFinality {
            self.finality.get_or_insert_default()
        }
        pub fn finality_opt(&self) -> Option<&super::TransactionFinality> {
            self.finality.as_ref().map(|field| field as _)
        }
        pub fn set_finality<T: Into<super::TransactionFinality>>(&mut self, field: T) {
            self.finality = Some(field.into().into());
        }
        pub fn with_finality<T: Into<super::TransactionFinality>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_finality(field);
            self
        }
        pub fn transaction(&self) -> &super::ExecutedTransaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ExecutedTransaction::default_instance() as _)
        }
        pub fn transaction_opt_mut(
            &mut self,
        ) -> Option<&mut super::ExecutedTransaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut super::ExecutedTransaction {
            self.transaction.get_or_insert_default()
        }
        pub fn transaction_opt(&self) -> Option<&super::ExecutedTransaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn set_transaction<T: Into<super::ExecutedTransaction>>(
            &mut self,
            field: T,
        ) {
            self.transaction = Some(field.into().into());
        }
        pub fn with_transaction<T: Into<super::ExecutedTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field);
            self
        }
    }
    impl super::ExecutedTransaction {
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
            static DEFAULT: super::ExecutedTransaction = super::ExecutedTransaction::const_default();
            &DEFAULT
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn transaction(&self) -> &super::Transaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Transaction::default_instance() as _)
        }
        pub fn transaction_opt_mut(&mut self) -> Option<&mut super::Transaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut super::Transaction {
            self.transaction.get_or_insert_default()
        }
        pub fn transaction_opt(&self) -> Option<&super::Transaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn set_transaction<T: Into<super::Transaction>>(&mut self, field: T) {
            self.transaction = Some(field.into().into());
        }
        pub fn with_transaction<T: Into<super::Transaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field);
            self
        }
        pub fn signatures(&self) -> &[super::UserSignature] {
            &self.signatures
        }
        pub fn set_signatures(&mut self, field: Vec<super::UserSignature>) {
            self.signatures = field;
        }
        pub fn with_signatures(mut self, field: Vec<super::UserSignature>) -> Self {
            self.set_signatures(field);
            self
        }
        pub fn effects(&self) -> &super::TransactionEffects {
            self.effects
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::TransactionEffects::default_instance() as _)
        }
        pub fn effects_opt_mut(&mut self) -> Option<&mut super::TransactionEffects> {
            self.effects.as_mut().map(|field| field as _)
        }
        pub fn effects_mut(&mut self) -> &mut super::TransactionEffects {
            self.effects.get_or_insert_default()
        }
        pub fn effects_opt(&self) -> Option<&super::TransactionEffects> {
            self.effects.as_ref().map(|field| field as _)
        }
        pub fn set_effects<T: Into<super::TransactionEffects>>(&mut self, field: T) {
            self.effects = Some(field.into().into());
        }
        pub fn with_effects<T: Into<super::TransactionEffects>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_effects(field);
            self
        }
        pub fn events(&self) -> &super::TransactionEvents {
            self.events
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::TransactionEvents::default_instance() as _)
        }
        pub fn events_opt_mut(&mut self) -> Option<&mut super::TransactionEvents> {
            self.events.as_mut().map(|field| field as _)
        }
        pub fn events_mut(&mut self) -> &mut super::TransactionEvents {
            self.events.get_or_insert_default()
        }
        pub fn events_opt(&self) -> Option<&super::TransactionEvents> {
            self.events.as_ref().map(|field| field as _)
        }
        pub fn set_events<T: Into<super::TransactionEvents>>(&mut self, field: T) {
            self.events = Some(field.into().into());
        }
        pub fn with_events<T: Into<super::TransactionEvents>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_events(field);
            self
        }
        pub fn checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.checkpoint.as_mut().map(|field| field as _)
        }
        pub fn checkpoint_mut(&mut self) -> &mut u64 {
            self.checkpoint.get_or_insert_default()
        }
        pub fn checkpoint_opt(&self) -> Option<u64> {
            self.checkpoint.as_ref().map(|field| *field)
        }
        pub fn set_checkpoint<T: Into<u64>>(&mut self, field: T) {
            self.checkpoint = Some(field.into().into());
        }
        pub fn with_checkpoint<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_checkpoint(field);
            self
        }
        pub fn timestamp_opt_mut(&mut self) -> Option<&mut ::prost_types::Timestamp> {
            self.timestamp.as_mut().map(|field| field as _)
        }
        pub fn timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.timestamp.get_or_insert_default()
        }
        pub fn timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.timestamp.as_ref().map(|field| field as _)
        }
        pub fn set_timestamp<T: Into<::prost_types::Timestamp>>(&mut self, field: T) {
            self.timestamp = Some(field.into().into());
        }
        pub fn with_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_timestamp(field);
            self
        }
        pub fn balance_changes(&self) -> &[super::BalanceChange] {
            &self.balance_changes
        }
        pub fn set_balance_changes(&mut self, field: Vec<super::BalanceChange>) {
            self.balance_changes = field;
        }
        pub fn with_balance_changes(mut self, field: Vec<super::BalanceChange>) -> Self {
            self.set_balance_changes(field);
            self
        }
        pub fn input_objects(&self) -> &[super::Object] {
            &self.input_objects
        }
        pub fn set_input_objects(&mut self, field: Vec<super::Object>) {
            self.input_objects = field;
        }
        pub fn with_input_objects(mut self, field: Vec<super::Object>) -> Self {
            self.set_input_objects(field);
            self
        }
        pub fn output_objects(&self) -> &[super::Object] {
            &self.output_objects
        }
        pub fn set_output_objects(&mut self, field: Vec<super::Object>) {
            self.output_objects = field;
        }
        pub fn with_output_objects(mut self, field: Vec<super::Object>) -> Self {
            self.set_output_objects(field);
            self
        }
    }
    impl super::ExecutionError {
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
            static DEFAULT: super::ExecutionError = super::ExecutionError::const_default();
            &DEFAULT
        }
        pub fn description_opt_mut(&mut self) -> Option<&mut String> {
            self.description.as_mut().map(|field| field as _)
        }
        pub fn description_mut(&mut self) -> &mut String {
            self.description.get_or_insert_default()
        }
        pub fn description_opt(&self) -> Option<&str> {
            self.description.as_ref().map(|field| field as _)
        }
        pub fn set_description<T: Into<String>>(&mut self, field: T) {
            self.description = Some(field.into().into());
        }
        pub fn with_description<T: Into<String>>(mut self, field: T) -> Self {
            self.set_description(field);
            self
        }
        pub fn command_opt_mut(&mut self) -> Option<&mut u64> {
            self.command.as_mut().map(|field| field as _)
        }
        pub fn command_mut(&mut self) -> &mut u64 {
            self.command.get_or_insert_default()
        }
        pub fn command_opt(&self) -> Option<u64> {
            self.command.as_ref().map(|field| *field)
        }
        pub fn set_command<T: Into<u64>>(&mut self, field: T) {
            self.command = Some(field.into().into());
        }
        pub fn with_command<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_command(field);
            self
        }
        pub fn abort(&self) -> &super::MoveAbort {
            if let Some(super::execution_error::ErrorDetails::Abort(field)) = &self
                .error_details
            {
                field as _
            } else {
                super::MoveAbort::default_instance() as _
            }
        }
        pub fn abort_opt(&self) -> Option<&super::MoveAbort> {
            if let Some(super::execution_error::ErrorDetails::Abort(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn abort_opt_mut(&mut self) -> Option<&mut super::MoveAbort> {
            if let Some(super::execution_error::ErrorDetails::Abort(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn abort_mut(&mut self) -> &mut super::MoveAbort {
            if self.abort_opt_mut().is_none() {
                self.error_details = Some(
                    super::execution_error::ErrorDetails::Abort(
                        super::MoveAbort::default(),
                    ),
                );
            }
            self.abort_opt_mut().unwrap()
        }
        pub fn set_abort<T: Into<super::MoveAbort>>(&mut self, field: T) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::Abort(field.into().into()),
            );
        }
        pub fn with_abort<T: Into<super::MoveAbort>>(mut self, field: T) -> Self {
            self.set_abort(field);
            self
        }
        pub fn size_error(&self) -> &super::SizeError {
            if let Some(super::execution_error::ErrorDetails::SizeError(field)) = &self
                .error_details
            {
                field as _
            } else {
                super::SizeError::default_instance() as _
            }
        }
        pub fn size_error_opt(&self) -> Option<&super::SizeError> {
            if let Some(super::execution_error::ErrorDetails::SizeError(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn size_error_opt_mut(&mut self) -> Option<&mut super::SizeError> {
            if let Some(super::execution_error::ErrorDetails::SizeError(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn size_error_mut(&mut self) -> &mut super::SizeError {
            if self.size_error_opt_mut().is_none() {
                self.error_details = Some(
                    super::execution_error::ErrorDetails::SizeError(
                        super::SizeError::default(),
                    ),
                );
            }
            self.size_error_opt_mut().unwrap()
        }
        pub fn set_size_error<T: Into<super::SizeError>>(&mut self, field: T) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::SizeError(field.into().into()),
            );
        }
        pub fn with_size_error<T: Into<super::SizeError>>(mut self, field: T) -> Self {
            self.set_size_error(field);
            self
        }
        pub fn command_argument_error(&self) -> &super::CommandArgumentError {
            if let Some(
                super::execution_error::ErrorDetails::CommandArgumentError(field),
            ) = &self.error_details
            {
                field as _
            } else {
                super::CommandArgumentError::default_instance() as _
            }
        }
        pub fn command_argument_error_opt(
            &self,
        ) -> Option<&super::CommandArgumentError> {
            if let Some(
                super::execution_error::ErrorDetails::CommandArgumentError(field),
            ) = &self.error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn command_argument_error_opt_mut(
            &mut self,
        ) -> Option<&mut super::CommandArgumentError> {
            if let Some(
                super::execution_error::ErrorDetails::CommandArgumentError(field),
            ) = &mut self.error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn command_argument_error_mut(
            &mut self,
        ) -> &mut super::CommandArgumentError {
            if self.command_argument_error_opt_mut().is_none() {
                self.error_details = Some(
                    super::execution_error::ErrorDetails::CommandArgumentError(
                        super::CommandArgumentError::default(),
                    ),
                );
            }
            self.command_argument_error_opt_mut().unwrap()
        }
        pub fn set_command_argument_error<T: Into<super::CommandArgumentError>>(
            &mut self,
            field: T,
        ) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::CommandArgumentError(
                    field.into().into(),
                ),
            );
        }
        pub fn with_command_argument_error<T: Into<super::CommandArgumentError>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_command_argument_error(field);
            self
        }
        pub fn type_argument_error(&self) -> &super::TypeArgumentError {
            if let Some(
                super::execution_error::ErrorDetails::TypeArgumentError(field),
            ) = &self.error_details
            {
                field as _
            } else {
                super::TypeArgumentError::default_instance() as _
            }
        }
        pub fn type_argument_error_opt(&self) -> Option<&super::TypeArgumentError> {
            if let Some(
                super::execution_error::ErrorDetails::TypeArgumentError(field),
            ) = &self.error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn type_argument_error_opt_mut(
            &mut self,
        ) -> Option<&mut super::TypeArgumentError> {
            if let Some(
                super::execution_error::ErrorDetails::TypeArgumentError(field),
            ) = &mut self.error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn type_argument_error_mut(&mut self) -> &mut super::TypeArgumentError {
            if self.type_argument_error_opt_mut().is_none() {
                self.error_details = Some(
                    super::execution_error::ErrorDetails::TypeArgumentError(
                        super::TypeArgumentError::default(),
                    ),
                );
            }
            self.type_argument_error_opt_mut().unwrap()
        }
        pub fn set_type_argument_error<T: Into<super::TypeArgumentError>>(
            &mut self,
            field: T,
        ) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::TypeArgumentError(
                    field.into().into(),
                ),
            );
        }
        pub fn with_type_argument_error<T: Into<super::TypeArgumentError>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_type_argument_error(field);
            self
        }
        pub fn package_upgrade_error(&self) -> &super::PackageUpgradeError {
            if let Some(
                super::execution_error::ErrorDetails::PackageUpgradeError(field),
            ) = &self.error_details
            {
                field as _
            } else {
                super::PackageUpgradeError::default_instance() as _
            }
        }
        pub fn package_upgrade_error_opt(&self) -> Option<&super::PackageUpgradeError> {
            if let Some(
                super::execution_error::ErrorDetails::PackageUpgradeError(field),
            ) = &self.error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn package_upgrade_error_opt_mut(
            &mut self,
        ) -> Option<&mut super::PackageUpgradeError> {
            if let Some(
                super::execution_error::ErrorDetails::PackageUpgradeError(field),
            ) = &mut self.error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn package_upgrade_error_mut(&mut self) -> &mut super::PackageUpgradeError {
            if self.package_upgrade_error_opt_mut().is_none() {
                self.error_details = Some(
                    super::execution_error::ErrorDetails::PackageUpgradeError(
                        super::PackageUpgradeError::default(),
                    ),
                );
            }
            self.package_upgrade_error_opt_mut().unwrap()
        }
        pub fn set_package_upgrade_error<T: Into<super::PackageUpgradeError>>(
            &mut self,
            field: T,
        ) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::PackageUpgradeError(
                    field.into().into(),
                ),
            );
        }
        pub fn with_package_upgrade_error<T: Into<super::PackageUpgradeError>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_package_upgrade_error(field);
            self
        }
        pub fn index_error(&self) -> &super::IndexError {
            if let Some(super::execution_error::ErrorDetails::IndexError(field)) = &self
                .error_details
            {
                field as _
            } else {
                super::IndexError::default_instance() as _
            }
        }
        pub fn index_error_opt(&self) -> Option<&super::IndexError> {
            if let Some(super::execution_error::ErrorDetails::IndexError(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn index_error_opt_mut(&mut self) -> Option<&mut super::IndexError> {
            if let Some(super::execution_error::ErrorDetails::IndexError(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn index_error_mut(&mut self) -> &mut super::IndexError {
            if self.index_error_opt_mut().is_none() {
                self.error_details = Some(
                    super::execution_error::ErrorDetails::IndexError(
                        super::IndexError::default(),
                    ),
                );
            }
            self.index_error_opt_mut().unwrap()
        }
        pub fn set_index_error<T: Into<super::IndexError>>(&mut self, field: T) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::IndexError(field.into().into()),
            );
        }
        pub fn with_index_error<T: Into<super::IndexError>>(mut self, field: T) -> Self {
            self.set_index_error(field);
            self
        }
        pub fn object_id(&self) -> &str {
            if let Some(super::execution_error::ErrorDetails::ObjectId(field)) = &self
                .error_details
            {
                field as _
            } else {
                ""
            }
        }
        pub fn object_id_opt(&self) -> Option<&str> {
            if let Some(super::execution_error::ErrorDetails::ObjectId(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(super::execution_error::ErrorDetails::ObjectId(field)) = &mut self
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
                    super::execution_error::ErrorDetails::ObjectId(String::default()),
                );
            }
            self.object_id_opt_mut().unwrap()
        }
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::ObjectId(field.into().into()),
            );
        }
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field);
            self
        }
        pub fn coin_deny_list_error(&self) -> &super::CoinDenyListError {
            if let Some(
                super::execution_error::ErrorDetails::CoinDenyListError(field),
            ) = &self.error_details
            {
                field as _
            } else {
                super::CoinDenyListError::default_instance() as _
            }
        }
        pub fn coin_deny_list_error_opt(&self) -> Option<&super::CoinDenyListError> {
            if let Some(
                super::execution_error::ErrorDetails::CoinDenyListError(field),
            ) = &self.error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn coin_deny_list_error_opt_mut(
            &mut self,
        ) -> Option<&mut super::CoinDenyListError> {
            if let Some(
                super::execution_error::ErrorDetails::CoinDenyListError(field),
            ) = &mut self.error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn coin_deny_list_error_mut(&mut self) -> &mut super::CoinDenyListError {
            if self.coin_deny_list_error_opt_mut().is_none() {
                self.error_details = Some(
                    super::execution_error::ErrorDetails::CoinDenyListError(
                        super::CoinDenyListError::default(),
                    ),
                );
            }
            self.coin_deny_list_error_opt_mut().unwrap()
        }
        pub fn set_coin_deny_list_error<T: Into<super::CoinDenyListError>>(
            &mut self,
            field: T,
        ) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::CoinDenyListError(
                    field.into().into(),
                ),
            );
        }
        pub fn with_coin_deny_list_error<T: Into<super::CoinDenyListError>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_coin_deny_list_error(field);
            self
        }
        pub fn congested_objects(&self) -> &super::CongestedObjects {
            if let Some(super::execution_error::ErrorDetails::CongestedObjects(field)) = &self
                .error_details
            {
                field as _
            } else {
                super::CongestedObjects::default_instance() as _
            }
        }
        pub fn congested_objects_opt(&self) -> Option<&super::CongestedObjects> {
            if let Some(super::execution_error::ErrorDetails::CongestedObjects(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn congested_objects_opt_mut(
            &mut self,
        ) -> Option<&mut super::CongestedObjects> {
            if let Some(super::execution_error::ErrorDetails::CongestedObjects(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn congested_objects_mut(&mut self) -> &mut super::CongestedObjects {
            if self.congested_objects_opt_mut().is_none() {
                self.error_details = Some(
                    super::execution_error::ErrorDetails::CongestedObjects(
                        super::CongestedObjects::default(),
                    ),
                );
            }
            self.congested_objects_opt_mut().unwrap()
        }
        pub fn set_congested_objects<T: Into<super::CongestedObjects>>(
            &mut self,
            field: T,
        ) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::CongestedObjects(
                    field.into().into(),
                ),
            );
        }
        pub fn with_congested_objects<T: Into<super::CongestedObjects>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_congested_objects(field);
            self
        }
    }
    impl super::ExecutionStatus {
        pub const fn const_default() -> Self {
            Self { success: None, error: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ExecutionStatus = super::ExecutionStatus::const_default();
            &DEFAULT
        }
        pub fn success_opt_mut(&mut self) -> Option<&mut bool> {
            self.success.as_mut().map(|field| field as _)
        }
        pub fn success_mut(&mut self) -> &mut bool {
            self.success.get_or_insert_default()
        }
        pub fn success_opt(&self) -> Option<bool> {
            self.success.as_ref().map(|field| *field)
        }
        pub fn set_success<T: Into<bool>>(&mut self, field: T) {
            self.success = Some(field.into().into());
        }
        pub fn with_success<T: Into<bool>>(mut self, field: T) -> Self {
            self.set_success(field);
            self
        }
        pub fn error(&self) -> &super::ExecutionError {
            self.error
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ExecutionError::default_instance() as _)
        }
        pub fn error_opt_mut(&mut self) -> Option<&mut super::ExecutionError> {
            self.error.as_mut().map(|field| field as _)
        }
        pub fn error_mut(&mut self) -> &mut super::ExecutionError {
            self.error.get_or_insert_default()
        }
        pub fn error_opt(&self) -> Option<&super::ExecutionError> {
            self.error.as_ref().map(|field| field as _)
        }
        pub fn set_error<T: Into<super::ExecutionError>>(&mut self, field: T) {
            self.error = Some(field.into().into());
        }
        pub fn with_error<T: Into<super::ExecutionError>>(mut self, field: T) -> Self {
            self.set_error(field);
            self
        }
    }
    impl super::ExecutionTimeObservation {
        pub const fn const_default() -> Self {
            Self {
                kind: None,
                move_entry_point: None,
                validator_observations: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ExecutionTimeObservation = super::ExecutionTimeObservation::const_default();
            &DEFAULT
        }
        pub fn move_entry_point(&self) -> &super::MoveCall {
            self.move_entry_point
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveCall::default_instance() as _)
        }
        pub fn move_entry_point_opt_mut(&mut self) -> Option<&mut super::MoveCall> {
            self.move_entry_point.as_mut().map(|field| field as _)
        }
        pub fn move_entry_point_mut(&mut self) -> &mut super::MoveCall {
            self.move_entry_point.get_or_insert_default()
        }
        pub fn move_entry_point_opt(&self) -> Option<&super::MoveCall> {
            self.move_entry_point.as_ref().map(|field| field as _)
        }
        pub fn set_move_entry_point<T: Into<super::MoveCall>>(&mut self, field: T) {
            self.move_entry_point = Some(field.into().into());
        }
        pub fn with_move_entry_point<T: Into<super::MoveCall>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_move_entry_point(field);
            self
        }
        pub fn validator_observations(
            &self,
        ) -> &[super::ValidatorExecutionTimeObservation] {
            &self.validator_observations
        }
        pub fn set_validator_observations(
            &mut self,
            field: Vec<super::ValidatorExecutionTimeObservation>,
        ) {
            self.validator_observations = field;
        }
        pub fn with_validator_observations(
            mut self,
            field: Vec<super::ValidatorExecutionTimeObservation>,
        ) -> Self {
            self.set_validator_observations(field);
            self
        }
    }
    impl super::ExecutionTimeObservations {
        pub const fn const_default() -> Self {
            Self {
                version: None,
                observations: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ExecutionTimeObservations = super::ExecutionTimeObservations::const_default();
            &DEFAULT
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut i32> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut i32 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<i32> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<i32>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<i32>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn observations(&self) -> &[super::ExecutionTimeObservation] {
            &self.observations
        }
        pub fn set_observations(&mut self, field: Vec<super::ExecutionTimeObservation>) {
            self.observations = field;
        }
        pub fn with_observations(
            mut self,
            field: Vec<super::ExecutionTimeObservation>,
        ) -> Self {
            self.set_observations(field);
            self
        }
    }
    impl super::FieldDescriptor {
        pub const fn const_default() -> Self {
            Self {
                name: None,
                position: None,
                r#type: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::FieldDescriptor = super::FieldDescriptor::const_default();
            &DEFAULT
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
        pub fn position_opt_mut(&mut self) -> Option<&mut u32> {
            self.position.as_mut().map(|field| field as _)
        }
        pub fn position_mut(&mut self) -> &mut u32 {
            self.position.get_or_insert_default()
        }
        pub fn position_opt(&self) -> Option<u32> {
            self.position.as_ref().map(|field| *field)
        }
        pub fn set_position<T: Into<u32>>(&mut self, field: T) {
            self.position = Some(field.into().into());
        }
        pub fn with_position<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_position(field);
            self
        }
        pub fn r#type(&self) -> &super::OpenSignatureBody {
            self.r#type
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::OpenSignatureBody::default_instance() as _)
        }
        pub fn type_opt_mut(&mut self) -> Option<&mut super::OpenSignatureBody> {
            self.r#type.as_mut().map(|field| field as _)
        }
        pub fn type_mut(&mut self) -> &mut super::OpenSignatureBody {
            self.r#type.get_or_insert_default()
        }
        pub fn type_opt(&self) -> Option<&super::OpenSignatureBody> {
            self.r#type.as_ref().map(|field| field as _)
        }
        pub fn set_type<T: Into<super::OpenSignatureBody>>(&mut self, field: T) {
            self.r#type = Some(field.into().into());
        }
        pub fn with_type<T: Into<super::OpenSignatureBody>>(mut self, field: T) -> Self {
            self.set_type(field);
            self
        }
    }
    impl super::FunctionDescriptor {
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
            static DEFAULT: super::FunctionDescriptor = super::FunctionDescriptor::const_default();
            &DEFAULT
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
        pub fn is_entry_opt_mut(&mut self) -> Option<&mut bool> {
            self.is_entry.as_mut().map(|field| field as _)
        }
        pub fn is_entry_mut(&mut self) -> &mut bool {
            self.is_entry.get_or_insert_default()
        }
        pub fn is_entry_opt(&self) -> Option<bool> {
            self.is_entry.as_ref().map(|field| *field)
        }
        pub fn set_is_entry<T: Into<bool>>(&mut self, field: T) {
            self.is_entry = Some(field.into().into());
        }
        pub fn with_is_entry<T: Into<bool>>(mut self, field: T) -> Self {
            self.set_is_entry(field);
            self
        }
        pub fn type_parameters(&self) -> &[super::TypeParameter] {
            &self.type_parameters
        }
        pub fn set_type_parameters(&mut self, field: Vec<super::TypeParameter>) {
            self.type_parameters = field;
        }
        pub fn with_type_parameters(mut self, field: Vec<super::TypeParameter>) -> Self {
            self.set_type_parameters(field);
            self
        }
        pub fn parameters(&self) -> &[super::OpenSignature] {
            &self.parameters
        }
        pub fn set_parameters(&mut self, field: Vec<super::OpenSignature>) {
            self.parameters = field;
        }
        pub fn with_parameters(mut self, field: Vec<super::OpenSignature>) -> Self {
            self.set_parameters(field);
            self
        }
        pub fn returns(&self) -> &[super::OpenSignature] {
            &self.returns
        }
        pub fn set_returns(&mut self, field: Vec<super::OpenSignature>) {
            self.returns = field;
        }
        pub fn with_returns(mut self, field: Vec<super::OpenSignature>) -> Self {
            self.set_returns(field);
            self
        }
    }
    impl super::GasCostSummary {
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
            static DEFAULT: super::GasCostSummary = super::GasCostSummary::const_default();
            &DEFAULT
        }
        pub fn computation_cost_opt_mut(&mut self) -> Option<&mut u64> {
            self.computation_cost.as_mut().map(|field| field as _)
        }
        pub fn computation_cost_mut(&mut self) -> &mut u64 {
            self.computation_cost.get_or_insert_default()
        }
        pub fn computation_cost_opt(&self) -> Option<u64> {
            self.computation_cost.as_ref().map(|field| *field)
        }
        pub fn set_computation_cost<T: Into<u64>>(&mut self, field: T) {
            self.computation_cost = Some(field.into().into());
        }
        pub fn with_computation_cost<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_computation_cost(field);
            self
        }
        pub fn storage_cost_opt_mut(&mut self) -> Option<&mut u64> {
            self.storage_cost.as_mut().map(|field| field as _)
        }
        pub fn storage_cost_mut(&mut self) -> &mut u64 {
            self.storage_cost.get_or_insert_default()
        }
        pub fn storage_cost_opt(&self) -> Option<u64> {
            self.storage_cost.as_ref().map(|field| *field)
        }
        pub fn set_storage_cost<T: Into<u64>>(&mut self, field: T) {
            self.storage_cost = Some(field.into().into());
        }
        pub fn with_storage_cost<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_storage_cost(field);
            self
        }
        pub fn storage_rebate_opt_mut(&mut self) -> Option<&mut u64> {
            self.storage_rebate.as_mut().map(|field| field as _)
        }
        pub fn storage_rebate_mut(&mut self) -> &mut u64 {
            self.storage_rebate.get_or_insert_default()
        }
        pub fn storage_rebate_opt(&self) -> Option<u64> {
            self.storage_rebate.as_ref().map(|field| *field)
        }
        pub fn set_storage_rebate<T: Into<u64>>(&mut self, field: T) {
            self.storage_rebate = Some(field.into().into());
        }
        pub fn with_storage_rebate<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_storage_rebate(field);
            self
        }
        pub fn non_refundable_storage_fee_opt_mut(&mut self) -> Option<&mut u64> {
            self.non_refundable_storage_fee.as_mut().map(|field| field as _)
        }
        pub fn non_refundable_storage_fee_mut(&mut self) -> &mut u64 {
            self.non_refundable_storage_fee.get_or_insert_default()
        }
        pub fn non_refundable_storage_fee_opt(&self) -> Option<u64> {
            self.non_refundable_storage_fee.as_ref().map(|field| *field)
        }
        pub fn set_non_refundable_storage_fee<T: Into<u64>>(&mut self, field: T) {
            self.non_refundable_storage_fee = Some(field.into().into());
        }
        pub fn with_non_refundable_storage_fee<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_non_refundable_storage_fee(field);
            self
        }
    }
    impl super::GasPayment {
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
            static DEFAULT: super::GasPayment = super::GasPayment::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[super::ObjectReference] {
            &self.objects
        }
        pub fn set_objects(&mut self, field: Vec<super::ObjectReference>) {
            self.objects = field;
        }
        pub fn with_objects(mut self, field: Vec<super::ObjectReference>) -> Self {
            self.set_objects(field);
            self
        }
        pub fn owner_opt_mut(&mut self) -> Option<&mut String> {
            self.owner.as_mut().map(|field| field as _)
        }
        pub fn owner_mut(&mut self) -> &mut String {
            self.owner.get_or_insert_default()
        }
        pub fn owner_opt(&self) -> Option<&str> {
            self.owner.as_ref().map(|field| field as _)
        }
        pub fn set_owner<T: Into<String>>(&mut self, field: T) {
            self.owner = Some(field.into().into());
        }
        pub fn with_owner<T: Into<String>>(mut self, field: T) -> Self {
            self.set_owner(field);
            self
        }
        pub fn price_opt_mut(&mut self) -> Option<&mut u64> {
            self.price.as_mut().map(|field| field as _)
        }
        pub fn price_mut(&mut self) -> &mut u64 {
            self.price.get_or_insert_default()
        }
        pub fn price_opt(&self) -> Option<u64> {
            self.price.as_ref().map(|field| *field)
        }
        pub fn set_price<T: Into<u64>>(&mut self, field: T) {
            self.price = Some(field.into().into());
        }
        pub fn with_price<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_price(field);
            self
        }
        pub fn budget_opt_mut(&mut self) -> Option<&mut u64> {
            self.budget.as_mut().map(|field| field as _)
        }
        pub fn budget_mut(&mut self) -> &mut u64 {
            self.budget.get_or_insert_default()
        }
        pub fn budget_opt(&self) -> Option<u64> {
            self.budget.as_ref().map(|field| *field)
        }
        pub fn set_budget<T: Into<u64>>(&mut self, field: T) {
            self.budget = Some(field.into().into());
        }
        pub fn with_budget<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_budget(field);
            self
        }
    }
    impl super::GenesisTransaction {
        pub const fn const_default() -> Self {
            Self { objects: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GenesisTransaction = super::GenesisTransaction::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[super::Object] {
            &self.objects
        }
        pub fn set_objects(&mut self, field: Vec<super::Object>) {
            self.objects = field;
        }
        pub fn with_objects(mut self, field: Vec<super::Object>) -> Self {
            self.set_objects(field);
            self
        }
    }
    impl super::GetBalanceRequest {
        pub const fn const_default() -> Self {
            Self {
                owner: None,
                coin_type: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetBalanceRequest = super::GetBalanceRequest::const_default();
            &DEFAULT
        }
        pub fn owner_opt_mut(&mut self) -> Option<&mut String> {
            self.owner.as_mut().map(|field| field as _)
        }
        pub fn owner_mut(&mut self) -> &mut String {
            self.owner.get_or_insert_default()
        }
        pub fn owner_opt(&self) -> Option<&str> {
            self.owner.as_ref().map(|field| field as _)
        }
        pub fn set_owner<T: Into<String>>(&mut self, field: T) {
            self.owner = Some(field.into().into());
        }
        pub fn with_owner<T: Into<String>>(mut self, field: T) -> Self {
            self.set_owner(field);
            self
        }
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field);
            self
        }
    }
    impl super::GetBalanceResponse {
        pub const fn const_default() -> Self {
            Self { balance: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetBalanceResponse = super::GetBalanceResponse::const_default();
            &DEFAULT
        }
        pub fn balance(&self) -> &super::Balance {
            self.balance
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Balance::default_instance() as _)
        }
        pub fn balance_opt_mut(&mut self) -> Option<&mut super::Balance> {
            self.balance.as_mut().map(|field| field as _)
        }
        pub fn balance_mut(&mut self) -> &mut super::Balance {
            self.balance.get_or_insert_default()
        }
        pub fn balance_opt(&self) -> Option<&super::Balance> {
            self.balance.as_ref().map(|field| field as _)
        }
        pub fn set_balance<T: Into<super::Balance>>(&mut self, field: T) {
            self.balance = Some(field.into().into());
        }
        pub fn with_balance<T: Into<super::Balance>>(mut self, field: T) -> Self {
            self.set_balance(field);
            self
        }
    }
    impl super::GetCheckpointRequest {
        pub const fn const_default() -> Self {
            Self {
                read_mask: None,
                checkpoint_id: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetCheckpointRequest = super::GetCheckpointRequest::const_default();
            &DEFAULT
        }
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field);
            self
        }
        pub fn sequence_number(&self) -> u64 {
            if let Some(
                super::get_checkpoint_request::CheckpointId::SequenceNumber(field),
            ) = &self.checkpoint_id
            {
                *field
            } else {
                0u64
            }
        }
        pub fn sequence_number_opt(&self) -> Option<u64> {
            if let Some(
                super::get_checkpoint_request::CheckpointId::SequenceNumber(field),
            ) = &self.checkpoint_id
            {
                Some(*field)
            } else {
                None
            }
        }
        pub fn sequence_number_opt_mut(&mut self) -> Option<&mut u64> {
            if let Some(
                super::get_checkpoint_request::CheckpointId::SequenceNumber(field),
            ) = &mut self.checkpoint_id
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn sequence_number_mut(&mut self) -> &mut u64 {
            if self.sequence_number_opt_mut().is_none() {
                self.checkpoint_id = Some(
                    super::get_checkpoint_request::CheckpointId::SequenceNumber(
                        u64::default(),
                    ),
                );
            }
            self.sequence_number_opt_mut().unwrap()
        }
        pub fn set_sequence_number<T: Into<u64>>(&mut self, field: T) {
            self.checkpoint_id = Some(
                super::get_checkpoint_request::CheckpointId::SequenceNumber(
                    field.into().into(),
                ),
            );
        }
        pub fn with_sequence_number<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_sequence_number(field);
            self
        }
        pub fn digest(&self) -> &str {
            if let Some(super::get_checkpoint_request::CheckpointId::Digest(field)) = &self
                .checkpoint_id
            {
                field as _
            } else {
                ""
            }
        }
        pub fn digest_opt(&self) -> Option<&str> {
            if let Some(super::get_checkpoint_request::CheckpointId::Digest(field)) = &self
                .checkpoint_id
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(super::get_checkpoint_request::CheckpointId::Digest(field)) = &mut self
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
                    super::get_checkpoint_request::CheckpointId::Digest(
                        String::default(),
                    ),
                );
            }
            self.digest_opt_mut().unwrap()
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.checkpoint_id = Some(
                super::get_checkpoint_request::CheckpointId::Digest(field.into().into()),
            );
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
    }
    impl super::GetCheckpointResponse {
        pub const fn const_default() -> Self {
            Self { checkpoint: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetCheckpointResponse = super::GetCheckpointResponse::const_default();
            &DEFAULT
        }
        pub fn checkpoint(&self) -> &super::Checkpoint {
            self.checkpoint
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Checkpoint::default_instance() as _)
        }
        pub fn checkpoint_opt_mut(&mut self) -> Option<&mut super::Checkpoint> {
            self.checkpoint.as_mut().map(|field| field as _)
        }
        pub fn checkpoint_mut(&mut self) -> &mut super::Checkpoint {
            self.checkpoint.get_or_insert_default()
        }
        pub fn checkpoint_opt(&self) -> Option<&super::Checkpoint> {
            self.checkpoint.as_ref().map(|field| field as _)
        }
        pub fn set_checkpoint<T: Into<super::Checkpoint>>(&mut self, field: T) {
            self.checkpoint = Some(field.into().into());
        }
        pub fn with_checkpoint<T: Into<super::Checkpoint>>(mut self, field: T) -> Self {
            self.set_checkpoint(field);
            self
        }
    }
    impl super::GetCoinInfoRequest {
        pub const fn const_default() -> Self {
            Self { coin_type: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetCoinInfoRequest = super::GetCoinInfoRequest::const_default();
            &DEFAULT
        }
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field);
            self
        }
    }
    impl super::GetCoinInfoResponse {
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
            static DEFAULT: super::GetCoinInfoResponse = super::GetCoinInfoResponse::const_default();
            &DEFAULT
        }
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field);
            self
        }
        pub fn metadata(&self) -> &super::CoinMetadata {
            self.metadata
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CoinMetadata::default_instance() as _)
        }
        pub fn metadata_opt_mut(&mut self) -> Option<&mut super::CoinMetadata> {
            self.metadata.as_mut().map(|field| field as _)
        }
        pub fn metadata_mut(&mut self) -> &mut super::CoinMetadata {
            self.metadata.get_or_insert_default()
        }
        pub fn metadata_opt(&self) -> Option<&super::CoinMetadata> {
            self.metadata.as_ref().map(|field| field as _)
        }
        pub fn set_metadata<T: Into<super::CoinMetadata>>(&mut self, field: T) {
            self.metadata = Some(field.into().into());
        }
        pub fn with_metadata<T: Into<super::CoinMetadata>>(mut self, field: T) -> Self {
            self.set_metadata(field);
            self
        }
        pub fn treasury(&self) -> &super::CoinTreasury {
            self.treasury
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CoinTreasury::default_instance() as _)
        }
        pub fn treasury_opt_mut(&mut self) -> Option<&mut super::CoinTreasury> {
            self.treasury.as_mut().map(|field| field as _)
        }
        pub fn treasury_mut(&mut self) -> &mut super::CoinTreasury {
            self.treasury.get_or_insert_default()
        }
        pub fn treasury_opt(&self) -> Option<&super::CoinTreasury> {
            self.treasury.as_ref().map(|field| field as _)
        }
        pub fn set_treasury<T: Into<super::CoinTreasury>>(&mut self, field: T) {
            self.treasury = Some(field.into().into());
        }
        pub fn with_treasury<T: Into<super::CoinTreasury>>(mut self, field: T) -> Self {
            self.set_treasury(field);
            self
        }
        pub fn regulated_metadata(&self) -> &super::RegulatedCoinMetadata {
            self.regulated_metadata
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::RegulatedCoinMetadata::default_instance() as _)
        }
        pub fn regulated_metadata_opt_mut(
            &mut self,
        ) -> Option<&mut super::RegulatedCoinMetadata> {
            self.regulated_metadata.as_mut().map(|field| field as _)
        }
        pub fn regulated_metadata_mut(&mut self) -> &mut super::RegulatedCoinMetadata {
            self.regulated_metadata.get_or_insert_default()
        }
        pub fn regulated_metadata_opt(&self) -> Option<&super::RegulatedCoinMetadata> {
            self.regulated_metadata.as_ref().map(|field| field as _)
        }
        pub fn set_regulated_metadata<T: Into<super::RegulatedCoinMetadata>>(
            &mut self,
            field: T,
        ) {
            self.regulated_metadata = Some(field.into().into());
        }
        pub fn with_regulated_metadata<T: Into<super::RegulatedCoinMetadata>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_regulated_metadata(field);
            self
        }
    }
    impl super::GetDatatypeRequest {
        pub const fn const_default() -> Self {
            Self {
                package_id: None,
                module_name: None,
                name: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetDatatypeRequest = super::GetDatatypeRequest::const_default();
            &DEFAULT
        }
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field);
            self
        }
        pub fn module_name_opt_mut(&mut self) -> Option<&mut String> {
            self.module_name.as_mut().map(|field| field as _)
        }
        pub fn module_name_mut(&mut self) -> &mut String {
            self.module_name.get_or_insert_default()
        }
        pub fn module_name_opt(&self) -> Option<&str> {
            self.module_name.as_ref().map(|field| field as _)
        }
        pub fn set_module_name<T: Into<String>>(&mut self, field: T) {
            self.module_name = Some(field.into().into());
        }
        pub fn with_module_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module_name(field);
            self
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
    }
    impl super::GetDatatypeResponse {
        pub const fn const_default() -> Self {
            Self { datatype: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetDatatypeResponse = super::GetDatatypeResponse::const_default();
            &DEFAULT
        }
        pub fn datatype(&self) -> &super::DatatypeDescriptor {
            self.datatype
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::DatatypeDescriptor::default_instance() as _)
        }
        pub fn datatype_opt_mut(&mut self) -> Option<&mut super::DatatypeDescriptor> {
            self.datatype.as_mut().map(|field| field as _)
        }
        pub fn datatype_mut(&mut self) -> &mut super::DatatypeDescriptor {
            self.datatype.get_or_insert_default()
        }
        pub fn datatype_opt(&self) -> Option<&super::DatatypeDescriptor> {
            self.datatype.as_ref().map(|field| field as _)
        }
        pub fn set_datatype<T: Into<super::DatatypeDescriptor>>(&mut self, field: T) {
            self.datatype = Some(field.into().into());
        }
        pub fn with_datatype<T: Into<super::DatatypeDescriptor>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_datatype(field);
            self
        }
    }
    impl super::GetEpochRequest {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetEpochRequest = super::GetEpochRequest::const_default();
            &DEFAULT
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field);
            self
        }
    }
    impl super::GetEpochResponse {
        pub const fn const_default() -> Self {
            Self { epoch: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetEpochResponse = super::GetEpochResponse::const_default();
            &DEFAULT
        }
        pub fn epoch(&self) -> &super::Epoch {
            self.epoch
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Epoch::default_instance() as _)
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut super::Epoch> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut super::Epoch {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<&super::Epoch> {
            self.epoch.as_ref().map(|field| field as _)
        }
        pub fn set_epoch<T: Into<super::Epoch>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<super::Epoch>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
    }
    impl super::GetFunctionRequest {
        pub const fn const_default() -> Self {
            Self {
                package_id: None,
                module_name: None,
                name: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetFunctionRequest = super::GetFunctionRequest::const_default();
            &DEFAULT
        }
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field);
            self
        }
        pub fn module_name_opt_mut(&mut self) -> Option<&mut String> {
            self.module_name.as_mut().map(|field| field as _)
        }
        pub fn module_name_mut(&mut self) -> &mut String {
            self.module_name.get_or_insert_default()
        }
        pub fn module_name_opt(&self) -> Option<&str> {
            self.module_name.as_ref().map(|field| field as _)
        }
        pub fn set_module_name<T: Into<String>>(&mut self, field: T) {
            self.module_name = Some(field.into().into());
        }
        pub fn with_module_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module_name(field);
            self
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
    }
    impl super::GetFunctionResponse {
        pub const fn const_default() -> Self {
            Self { function: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetFunctionResponse = super::GetFunctionResponse::const_default();
            &DEFAULT
        }
        pub fn function(&self) -> &super::FunctionDescriptor {
            self.function
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::FunctionDescriptor::default_instance() as _)
        }
        pub fn function_opt_mut(&mut self) -> Option<&mut super::FunctionDescriptor> {
            self.function.as_mut().map(|field| field as _)
        }
        pub fn function_mut(&mut self) -> &mut super::FunctionDescriptor {
            self.function.get_or_insert_default()
        }
        pub fn function_opt(&self) -> Option<&super::FunctionDescriptor> {
            self.function.as_ref().map(|field| field as _)
        }
        pub fn set_function<T: Into<super::FunctionDescriptor>>(&mut self, field: T) {
            self.function = Some(field.into().into());
        }
        pub fn with_function<T: Into<super::FunctionDescriptor>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_function(field);
            self
        }
    }
    impl super::GetObjectRequest {
        pub const fn const_default() -> Self {
            Self {
                object_id: None,
                version: None,
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetObjectRequest = super::GetObjectRequest::const_default();
            &DEFAULT
        }
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<u64>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field);
            self
        }
    }
    impl super::GetObjectResponse {
        pub const fn const_default() -> Self {
            Self { object: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetObjectResponse = super::GetObjectResponse::const_default();
            &DEFAULT
        }
        pub fn object(&self) -> &super::Object {
            self.object
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Object::default_instance() as _)
        }
        pub fn object_opt_mut(&mut self) -> Option<&mut super::Object> {
            self.object.as_mut().map(|field| field as _)
        }
        pub fn object_mut(&mut self) -> &mut super::Object {
            self.object.get_or_insert_default()
        }
        pub fn object_opt(&self) -> Option<&super::Object> {
            self.object.as_ref().map(|field| field as _)
        }
        pub fn set_object<T: Into<super::Object>>(&mut self, field: T) {
            self.object = Some(field.into().into());
        }
        pub fn with_object<T: Into<super::Object>>(mut self, field: T) -> Self {
            self.set_object(field);
            self
        }
    }
    impl super::GetObjectResult {
        pub const fn const_default() -> Self {
            Self { result: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetObjectResult = super::GetObjectResult::const_default();
            &DEFAULT
        }
        pub fn object(&self) -> &super::Object {
            if let Some(super::get_object_result::Result::Object(field)) = &self.result {
                field as _
            } else {
                super::Object::default_instance() as _
            }
        }
        pub fn object_opt(&self) -> Option<&super::Object> {
            if let Some(super::get_object_result::Result::Object(field)) = &self.result {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn object_opt_mut(&mut self) -> Option<&mut super::Object> {
            if let Some(super::get_object_result::Result::Object(field)) = &mut self
                .result
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn object_mut(&mut self) -> &mut super::Object {
            if self.object_opt_mut().is_none() {
                self.result = Some(
                    super::get_object_result::Result::Object(super::Object::default()),
                );
            }
            self.object_opt_mut().unwrap()
        }
        pub fn set_object<T: Into<super::Object>>(&mut self, field: T) {
            self.result = Some(
                super::get_object_result::Result::Object(field.into().into()),
            );
        }
        pub fn with_object<T: Into<super::Object>>(mut self, field: T) -> Self {
            self.set_object(field);
            self
        }
        pub fn error(&self) -> &super::super::super::super::google::rpc::Status {
            if let Some(super::get_object_result::Result::Error(field)) = &self.result {
                field as _
            } else {
                super::super::super::super::google::rpc::Status::default_instance() as _
            }
        }
        pub fn error_opt(
            &self,
        ) -> Option<&super::super::super::super::google::rpc::Status> {
            if let Some(super::get_object_result::Result::Error(field)) = &self.result {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn error_opt_mut(
            &mut self,
        ) -> Option<&mut super::super::super::super::google::rpc::Status> {
            if let Some(super::get_object_result::Result::Error(field)) = &mut self
                .result
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn error_mut(
            &mut self,
        ) -> &mut super::super::super::super::google::rpc::Status {
            if self.error_opt_mut().is_none() {
                self.result = Some(
                    super::get_object_result::Result::Error(
                        super::super::super::super::google::rpc::Status::default(),
                    ),
                );
            }
            self.error_opt_mut().unwrap()
        }
        pub fn set_error<T: Into<super::super::super::super::google::rpc::Status>>(
            &mut self,
            field: T,
        ) {
            self.result = Some(
                super::get_object_result::Result::Error(field.into().into()),
            );
        }
        pub fn with_error<T: Into<super::super::super::super::google::rpc::Status>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_error(field);
            self
        }
    }
    impl super::GetPackageRequest {
        pub const fn const_default() -> Self {
            Self { package_id: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetPackageRequest = super::GetPackageRequest::const_default();
            &DEFAULT
        }
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field);
            self
        }
    }
    impl super::GetPackageResponse {
        pub const fn const_default() -> Self {
            Self { package: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetPackageResponse = super::GetPackageResponse::const_default();
            &DEFAULT
        }
        pub fn package(&self) -> &super::Package {
            self.package
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Package::default_instance() as _)
        }
        pub fn package_opt_mut(&mut self) -> Option<&mut super::Package> {
            self.package.as_mut().map(|field| field as _)
        }
        pub fn package_mut(&mut self) -> &mut super::Package {
            self.package.get_or_insert_default()
        }
        pub fn package_opt(&self) -> Option<&super::Package> {
            self.package.as_ref().map(|field| field as _)
        }
        pub fn set_package<T: Into<super::Package>>(&mut self, field: T) {
            self.package = Some(field.into().into());
        }
        pub fn with_package<T: Into<super::Package>>(mut self, field: T) -> Self {
            self.set_package(field);
            self
        }
    }
    impl super::GetServiceInfoRequest {
        pub const fn const_default() -> Self {
            Self {}
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetServiceInfoRequest = super::GetServiceInfoRequest::const_default();
            &DEFAULT
        }
    }
    impl super::GetServiceInfoResponse {
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
            static DEFAULT: super::GetServiceInfoResponse = super::GetServiceInfoResponse::const_default();
            &DEFAULT
        }
        pub fn chain_id_opt_mut(&mut self) -> Option<&mut String> {
            self.chain_id.as_mut().map(|field| field as _)
        }
        pub fn chain_id_mut(&mut self) -> &mut String {
            self.chain_id.get_or_insert_default()
        }
        pub fn chain_id_opt(&self) -> Option<&str> {
            self.chain_id.as_ref().map(|field| field as _)
        }
        pub fn set_chain_id<T: Into<String>>(&mut self, field: T) {
            self.chain_id = Some(field.into().into());
        }
        pub fn with_chain_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_chain_id(field);
            self
        }
        pub fn chain_opt_mut(&mut self) -> Option<&mut String> {
            self.chain.as_mut().map(|field| field as _)
        }
        pub fn chain_mut(&mut self) -> &mut String {
            self.chain.get_or_insert_default()
        }
        pub fn chain_opt(&self) -> Option<&str> {
            self.chain.as_ref().map(|field| field as _)
        }
        pub fn set_chain<T: Into<String>>(&mut self, field: T) {
            self.chain = Some(field.into().into());
        }
        pub fn with_chain<T: Into<String>>(mut self, field: T) -> Self {
            self.set_chain(field);
            self
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn checkpoint_height_opt_mut(&mut self) -> Option<&mut u64> {
            self.checkpoint_height.as_mut().map(|field| field as _)
        }
        pub fn checkpoint_height_mut(&mut self) -> &mut u64 {
            self.checkpoint_height.get_or_insert_default()
        }
        pub fn checkpoint_height_opt(&self) -> Option<u64> {
            self.checkpoint_height.as_ref().map(|field| *field)
        }
        pub fn set_checkpoint_height<T: Into<u64>>(&mut self, field: T) {
            self.checkpoint_height = Some(field.into().into());
        }
        pub fn with_checkpoint_height<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_checkpoint_height(field);
            self
        }
        pub fn timestamp_opt_mut(&mut self) -> Option<&mut ::prost_types::Timestamp> {
            self.timestamp.as_mut().map(|field| field as _)
        }
        pub fn timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.timestamp.get_or_insert_default()
        }
        pub fn timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.timestamp.as_ref().map(|field| field as _)
        }
        pub fn set_timestamp<T: Into<::prost_types::Timestamp>>(&mut self, field: T) {
            self.timestamp = Some(field.into().into());
        }
        pub fn with_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_timestamp(field);
            self
        }
        pub fn lowest_available_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.lowest_available_checkpoint.as_mut().map(|field| field as _)
        }
        pub fn lowest_available_checkpoint_mut(&mut self) -> &mut u64 {
            self.lowest_available_checkpoint.get_or_insert_default()
        }
        pub fn lowest_available_checkpoint_opt(&self) -> Option<u64> {
            self.lowest_available_checkpoint.as_ref().map(|field| *field)
        }
        pub fn set_lowest_available_checkpoint<T: Into<u64>>(&mut self, field: T) {
            self.lowest_available_checkpoint = Some(field.into().into());
        }
        pub fn with_lowest_available_checkpoint<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_lowest_available_checkpoint(field);
            self
        }
        pub fn lowest_available_checkpoint_objects_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.lowest_available_checkpoint_objects.as_mut().map(|field| field as _)
        }
        pub fn lowest_available_checkpoint_objects_mut(&mut self) -> &mut u64 {
            self.lowest_available_checkpoint_objects.get_or_insert_default()
        }
        pub fn lowest_available_checkpoint_objects_opt(&self) -> Option<u64> {
            self.lowest_available_checkpoint_objects.as_ref().map(|field| *field)
        }
        pub fn set_lowest_available_checkpoint_objects<T: Into<u64>>(
            &mut self,
            field: T,
        ) {
            self.lowest_available_checkpoint_objects = Some(field.into().into());
        }
        pub fn with_lowest_available_checkpoint_objects<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_lowest_available_checkpoint_objects(field);
            self
        }
        pub fn server_opt_mut(&mut self) -> Option<&mut String> {
            self.server.as_mut().map(|field| field as _)
        }
        pub fn server_mut(&mut self) -> &mut String {
            self.server.get_or_insert_default()
        }
        pub fn server_opt(&self) -> Option<&str> {
            self.server.as_ref().map(|field| field as _)
        }
        pub fn set_server<T: Into<String>>(&mut self, field: T) {
            self.server = Some(field.into().into());
        }
        pub fn with_server<T: Into<String>>(mut self, field: T) -> Self {
            self.set_server(field);
            self
        }
    }
    impl super::GetTransactionRequest {
        pub const fn const_default() -> Self {
            Self {
                digest: None,
                read_mask: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetTransactionRequest = super::GetTransactionRequest::const_default();
            &DEFAULT
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field);
            self
        }
    }
    impl super::GetTransactionResponse {
        pub const fn const_default() -> Self {
            Self { transaction: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetTransactionResponse = super::GetTransactionResponse::const_default();
            &DEFAULT
        }
        pub fn transaction(&self) -> &super::ExecutedTransaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ExecutedTransaction::default_instance() as _)
        }
        pub fn transaction_opt_mut(
            &mut self,
        ) -> Option<&mut super::ExecutedTransaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut super::ExecutedTransaction {
            self.transaction.get_or_insert_default()
        }
        pub fn transaction_opt(&self) -> Option<&super::ExecutedTransaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn set_transaction<T: Into<super::ExecutedTransaction>>(
            &mut self,
            field: T,
        ) {
            self.transaction = Some(field.into().into());
        }
        pub fn with_transaction<T: Into<super::ExecutedTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field);
            self
        }
    }
    impl super::GetTransactionResult {
        pub const fn const_default() -> Self {
            Self { result: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetTransactionResult = super::GetTransactionResult::const_default();
            &DEFAULT
        }
        pub fn transaction(&self) -> &super::ExecutedTransaction {
            if let Some(super::get_transaction_result::Result::Transaction(field)) = &self
                .result
            {
                field as _
            } else {
                super::ExecutedTransaction::default_instance() as _
            }
        }
        pub fn transaction_opt(&self) -> Option<&super::ExecutedTransaction> {
            if let Some(super::get_transaction_result::Result::Transaction(field)) = &self
                .result
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn transaction_opt_mut(
            &mut self,
        ) -> Option<&mut super::ExecutedTransaction> {
            if let Some(super::get_transaction_result::Result::Transaction(field)) = &mut self
                .result
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn transaction_mut(&mut self) -> &mut super::ExecutedTransaction {
            if self.transaction_opt_mut().is_none() {
                self.result = Some(
                    super::get_transaction_result::Result::Transaction(
                        super::ExecutedTransaction::default(),
                    ),
                );
            }
            self.transaction_opt_mut().unwrap()
        }
        pub fn set_transaction<T: Into<super::ExecutedTransaction>>(
            &mut self,
            field: T,
        ) {
            self.result = Some(
                super::get_transaction_result::Result::Transaction(field.into().into()),
            );
        }
        pub fn with_transaction<T: Into<super::ExecutedTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field);
            self
        }
        pub fn error(&self) -> &super::super::super::super::google::rpc::Status {
            if let Some(super::get_transaction_result::Result::Error(field)) = &self
                .result
            {
                field as _
            } else {
                super::super::super::super::google::rpc::Status::default_instance() as _
            }
        }
        pub fn error_opt(
            &self,
        ) -> Option<&super::super::super::super::google::rpc::Status> {
            if let Some(super::get_transaction_result::Result::Error(field)) = &self
                .result
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn error_opt_mut(
            &mut self,
        ) -> Option<&mut super::super::super::super::google::rpc::Status> {
            if let Some(super::get_transaction_result::Result::Error(field)) = &mut self
                .result
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn error_mut(
            &mut self,
        ) -> &mut super::super::super::super::google::rpc::Status {
            if self.error_opt_mut().is_none() {
                self.result = Some(
                    super::get_transaction_result::Result::Error(
                        super::super::super::super::google::rpc::Status::default(),
                    ),
                );
            }
            self.error_opt_mut().unwrap()
        }
        pub fn set_error<T: Into<super::super::super::super::google::rpc::Status>>(
            &mut self,
            field: T,
        ) {
            self.result = Some(
                super::get_transaction_result::Result::Error(field.into().into()),
            );
        }
        pub fn with_error<T: Into<super::super::super::super::google::rpc::Status>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_error(field);
            self
        }
    }
    impl super::IndexError {
        pub const fn const_default() -> Self {
            Self {
                index: None,
                subresult: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::IndexError = super::IndexError::const_default();
            &DEFAULT
        }
        pub fn index_opt_mut(&mut self) -> Option<&mut u32> {
            self.index.as_mut().map(|field| field as _)
        }
        pub fn index_mut(&mut self) -> &mut u32 {
            self.index.get_or_insert_default()
        }
        pub fn index_opt(&self) -> Option<u32> {
            self.index.as_ref().map(|field| *field)
        }
        pub fn set_index<T: Into<u32>>(&mut self, field: T) {
            self.index = Some(field.into().into());
        }
        pub fn with_index<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_index(field);
            self
        }
        pub fn subresult_opt_mut(&mut self) -> Option<&mut u32> {
            self.subresult.as_mut().map(|field| field as _)
        }
        pub fn subresult_mut(&mut self) -> &mut u32 {
            self.subresult.get_or_insert_default()
        }
        pub fn subresult_opt(&self) -> Option<u32> {
            self.subresult.as_ref().map(|field| *field)
        }
        pub fn set_subresult<T: Into<u32>>(&mut self, field: T) {
            self.subresult = Some(field.into().into());
        }
        pub fn with_subresult<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_subresult(field);
            self
        }
    }
    impl super::Input {
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
            static DEFAULT: super::Input = super::Input::const_default();
            &DEFAULT
        }
        pub fn pure_opt(&self) -> Option<&[u8]> {
            self.pure.as_ref().map(|field| field as _)
        }
        pub fn set_pure<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.pure = Some(field.into().into());
        }
        pub fn with_pure<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_pure(field);
            self
        }
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<u64>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn mutable_opt_mut(&mut self) -> Option<&mut bool> {
            self.mutable.as_mut().map(|field| field as _)
        }
        pub fn mutable_mut(&mut self) -> &mut bool {
            self.mutable.get_or_insert_default()
        }
        pub fn mutable_opt(&self) -> Option<bool> {
            self.mutable.as_ref().map(|field| *field)
        }
        pub fn set_mutable<T: Into<bool>>(&mut self, field: T) {
            self.mutable = Some(field.into().into());
        }
        pub fn with_mutable<T: Into<bool>>(mut self, field: T) -> Self {
            self.set_mutable(field);
            self
        }
        pub fn literal_opt_mut(&mut self) -> Option<&mut ::prost_types::Value> {
            self.literal.as_mut().map(|field| field as _)
        }
        pub fn literal_mut(&mut self) -> &mut ::prost_types::Value {
            self.literal.get_or_insert_default()
        }
        pub fn literal_opt(&self) -> Option<&::prost_types::Value> {
            self.literal.as_ref().map(|field| field as _)
        }
        pub fn set_literal<T: Into<::prost_types::Value>>(&mut self, field: T) {
            self.literal = Some(field.into().into());
        }
        pub fn with_literal<T: Into<::prost_types::Value>>(mut self, field: T) -> Self {
            self.set_literal(field);
            self
        }
    }
    impl super::Jwk {
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
            static DEFAULT: super::Jwk = super::Jwk::const_default();
            &DEFAULT
        }
        pub fn kty_opt_mut(&mut self) -> Option<&mut String> {
            self.kty.as_mut().map(|field| field as _)
        }
        pub fn kty_mut(&mut self) -> &mut String {
            self.kty.get_or_insert_default()
        }
        pub fn kty_opt(&self) -> Option<&str> {
            self.kty.as_ref().map(|field| field as _)
        }
        pub fn set_kty<T: Into<String>>(&mut self, field: T) {
            self.kty = Some(field.into().into());
        }
        pub fn with_kty<T: Into<String>>(mut self, field: T) -> Self {
            self.set_kty(field);
            self
        }
        pub fn e_opt_mut(&mut self) -> Option<&mut String> {
            self.e.as_mut().map(|field| field as _)
        }
        pub fn e_mut(&mut self) -> &mut String {
            self.e.get_or_insert_default()
        }
        pub fn e_opt(&self) -> Option<&str> {
            self.e.as_ref().map(|field| field as _)
        }
        pub fn set_e<T: Into<String>>(&mut self, field: T) {
            self.e = Some(field.into().into());
        }
        pub fn with_e<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e(field);
            self
        }
        pub fn n_opt_mut(&mut self) -> Option<&mut String> {
            self.n.as_mut().map(|field| field as _)
        }
        pub fn n_mut(&mut self) -> &mut String {
            self.n.get_or_insert_default()
        }
        pub fn n_opt(&self) -> Option<&str> {
            self.n.as_ref().map(|field| field as _)
        }
        pub fn set_n<T: Into<String>>(&mut self, field: T) {
            self.n = Some(field.into().into());
        }
        pub fn with_n<T: Into<String>>(mut self, field: T) -> Self {
            self.set_n(field);
            self
        }
        pub fn alg_opt_mut(&mut self) -> Option<&mut String> {
            self.alg.as_mut().map(|field| field as _)
        }
        pub fn alg_mut(&mut self) -> &mut String {
            self.alg.get_or_insert_default()
        }
        pub fn alg_opt(&self) -> Option<&str> {
            self.alg.as_ref().map(|field| field as _)
        }
        pub fn set_alg<T: Into<String>>(&mut self, field: T) {
            self.alg = Some(field.into().into());
        }
        pub fn with_alg<T: Into<String>>(mut self, field: T) -> Self {
            self.set_alg(field);
            self
        }
    }
    impl super::JwkId {
        pub const fn const_default() -> Self {
            Self { iss: None, kid: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::JwkId = super::JwkId::const_default();
            &DEFAULT
        }
        pub fn iss_opt_mut(&mut self) -> Option<&mut String> {
            self.iss.as_mut().map(|field| field as _)
        }
        pub fn iss_mut(&mut self) -> &mut String {
            self.iss.get_or_insert_default()
        }
        pub fn iss_opt(&self) -> Option<&str> {
            self.iss.as_ref().map(|field| field as _)
        }
        pub fn set_iss<T: Into<String>>(&mut self, field: T) {
            self.iss = Some(field.into().into());
        }
        pub fn with_iss<T: Into<String>>(mut self, field: T) -> Self {
            self.set_iss(field);
            self
        }
        pub fn kid_opt_mut(&mut self) -> Option<&mut String> {
            self.kid.as_mut().map(|field| field as _)
        }
        pub fn kid_mut(&mut self) -> &mut String {
            self.kid.get_or_insert_default()
        }
        pub fn kid_opt(&self) -> Option<&str> {
            self.kid.as_ref().map(|field| field as _)
        }
        pub fn set_kid<T: Into<String>>(&mut self, field: T) {
            self.kid = Some(field.into().into());
        }
        pub fn with_kid<T: Into<String>>(mut self, field: T) -> Self {
            self.set_kid(field);
            self
        }
    }
    impl super::Linkage {
        pub const fn const_default() -> Self {
            Self {
                original_id: None,
                upgraded_id: None,
                upgraded_version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Linkage = super::Linkage::const_default();
            &DEFAULT
        }
        pub fn original_id_opt_mut(&mut self) -> Option<&mut String> {
            self.original_id.as_mut().map(|field| field as _)
        }
        pub fn original_id_mut(&mut self) -> &mut String {
            self.original_id.get_or_insert_default()
        }
        pub fn original_id_opt(&self) -> Option<&str> {
            self.original_id.as_ref().map(|field| field as _)
        }
        pub fn set_original_id<T: Into<String>>(&mut self, field: T) {
            self.original_id = Some(field.into().into());
        }
        pub fn with_original_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_original_id(field);
            self
        }
        pub fn upgraded_id_opt_mut(&mut self) -> Option<&mut String> {
            self.upgraded_id.as_mut().map(|field| field as _)
        }
        pub fn upgraded_id_mut(&mut self) -> &mut String {
            self.upgraded_id.get_or_insert_default()
        }
        pub fn upgraded_id_opt(&self) -> Option<&str> {
            self.upgraded_id.as_ref().map(|field| field as _)
        }
        pub fn set_upgraded_id<T: Into<String>>(&mut self, field: T) {
            self.upgraded_id = Some(field.into().into());
        }
        pub fn with_upgraded_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_upgraded_id(field);
            self
        }
        pub fn upgraded_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.upgraded_version.as_mut().map(|field| field as _)
        }
        pub fn upgraded_version_mut(&mut self) -> &mut u64 {
            self.upgraded_version.get_or_insert_default()
        }
        pub fn upgraded_version_opt(&self) -> Option<u64> {
            self.upgraded_version.as_ref().map(|field| *field)
        }
        pub fn set_upgraded_version<T: Into<u64>>(&mut self, field: T) {
            self.upgraded_version = Some(field.into().into());
        }
        pub fn with_upgraded_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_upgraded_version(field);
            self
        }
    }
    impl super::ListBalancesRequest {
        pub const fn const_default() -> Self {
            Self {
                owner: None,
                page_size: None,
                page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListBalancesRequest = super::ListBalancesRequest::const_default();
            &DEFAULT
        }
        pub fn owner_opt_mut(&mut self) -> Option<&mut String> {
            self.owner.as_mut().map(|field| field as _)
        }
        pub fn owner_mut(&mut self) -> &mut String {
            self.owner.get_or_insert_default()
        }
        pub fn owner_opt(&self) -> Option<&str> {
            self.owner.as_ref().map(|field| field as _)
        }
        pub fn set_owner<T: Into<String>>(&mut self, field: T) {
            self.owner = Some(field.into().into());
        }
        pub fn with_owner<T: Into<String>>(mut self, field: T) -> Self {
            self.set_owner(field);
            self
        }
        pub fn page_size_opt_mut(&mut self) -> Option<&mut u32> {
            self.page_size.as_mut().map(|field| field as _)
        }
        pub fn page_size_mut(&mut self) -> &mut u32 {
            self.page_size.get_or_insert_default()
        }
        pub fn page_size_opt(&self) -> Option<u32> {
            self.page_size.as_ref().map(|field| *field)
        }
        pub fn set_page_size<T: Into<u32>>(&mut self, field: T) {
            self.page_size = Some(field.into().into());
        }
        pub fn with_page_size<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_page_size(field);
            self
        }
        pub fn page_token_opt(&self) -> Option<&[u8]> {
            self.page_token.as_ref().map(|field| field as _)
        }
        pub fn set_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.page_token = Some(field.into().into());
        }
        pub fn with_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_page_token(field);
            self
        }
    }
    impl super::ListBalancesResponse {
        pub const fn const_default() -> Self {
            Self {
                balances: Vec::new(),
                next_page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListBalancesResponse = super::ListBalancesResponse::const_default();
            &DEFAULT
        }
        pub fn balances(&self) -> &[super::Balance] {
            &self.balances
        }
        pub fn set_balances(&mut self, field: Vec<super::Balance>) {
            self.balances = field;
        }
        pub fn with_balances(mut self, field: Vec<super::Balance>) -> Self {
            self.set_balances(field);
            self
        }
        pub fn next_page_token_opt(&self) -> Option<&[u8]> {
            self.next_page_token.as_ref().map(|field| field as _)
        }
        pub fn set_next_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.next_page_token = Some(field.into().into());
        }
        pub fn with_next_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_page_token(field);
            self
        }
    }
    impl super::ListDynamicFieldsRequest {
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
            static DEFAULT: super::ListDynamicFieldsRequest = super::ListDynamicFieldsRequest::const_default();
            &DEFAULT
        }
        pub fn parent_opt_mut(&mut self) -> Option<&mut String> {
            self.parent.as_mut().map(|field| field as _)
        }
        pub fn parent_mut(&mut self) -> &mut String {
            self.parent.get_or_insert_default()
        }
        pub fn parent_opt(&self) -> Option<&str> {
            self.parent.as_ref().map(|field| field as _)
        }
        pub fn set_parent<T: Into<String>>(&mut self, field: T) {
            self.parent = Some(field.into().into());
        }
        pub fn with_parent<T: Into<String>>(mut self, field: T) -> Self {
            self.set_parent(field);
            self
        }
        pub fn page_size_opt_mut(&mut self) -> Option<&mut u32> {
            self.page_size.as_mut().map(|field| field as _)
        }
        pub fn page_size_mut(&mut self) -> &mut u32 {
            self.page_size.get_or_insert_default()
        }
        pub fn page_size_opt(&self) -> Option<u32> {
            self.page_size.as_ref().map(|field| *field)
        }
        pub fn set_page_size<T: Into<u32>>(&mut self, field: T) {
            self.page_size = Some(field.into().into());
        }
        pub fn with_page_size<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_page_size(field);
            self
        }
        pub fn page_token_opt(&self) -> Option<&[u8]> {
            self.page_token.as_ref().map(|field| field as _)
        }
        pub fn set_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.page_token = Some(field.into().into());
        }
        pub fn with_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_page_token(field);
            self
        }
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field);
            self
        }
    }
    impl super::ListDynamicFieldsResponse {
        pub const fn const_default() -> Self {
            Self {
                dynamic_fields: Vec::new(),
                next_page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListDynamicFieldsResponse = super::ListDynamicFieldsResponse::const_default();
            &DEFAULT
        }
        pub fn dynamic_fields(&self) -> &[super::DynamicField] {
            &self.dynamic_fields
        }
        pub fn set_dynamic_fields(&mut self, field: Vec<super::DynamicField>) {
            self.dynamic_fields = field;
        }
        pub fn with_dynamic_fields(mut self, field: Vec<super::DynamicField>) -> Self {
            self.set_dynamic_fields(field);
            self
        }
        pub fn next_page_token_opt(&self) -> Option<&[u8]> {
            self.next_page_token.as_ref().map(|field| field as _)
        }
        pub fn set_next_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.next_page_token = Some(field.into().into());
        }
        pub fn with_next_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_page_token(field);
            self
        }
    }
    impl super::ListOwnedObjectsRequest {
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
            static DEFAULT: super::ListOwnedObjectsRequest = super::ListOwnedObjectsRequest::const_default();
            &DEFAULT
        }
        pub fn owner_opt_mut(&mut self) -> Option<&mut String> {
            self.owner.as_mut().map(|field| field as _)
        }
        pub fn owner_mut(&mut self) -> &mut String {
            self.owner.get_or_insert_default()
        }
        pub fn owner_opt(&self) -> Option<&str> {
            self.owner.as_ref().map(|field| field as _)
        }
        pub fn set_owner<T: Into<String>>(&mut self, field: T) {
            self.owner = Some(field.into().into());
        }
        pub fn with_owner<T: Into<String>>(mut self, field: T) -> Self {
            self.set_owner(field);
            self
        }
        pub fn page_size_opt_mut(&mut self) -> Option<&mut u32> {
            self.page_size.as_mut().map(|field| field as _)
        }
        pub fn page_size_mut(&mut self) -> &mut u32 {
            self.page_size.get_or_insert_default()
        }
        pub fn page_size_opt(&self) -> Option<u32> {
            self.page_size.as_ref().map(|field| *field)
        }
        pub fn set_page_size<T: Into<u32>>(&mut self, field: T) {
            self.page_size = Some(field.into().into());
        }
        pub fn with_page_size<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_page_size(field);
            self
        }
        pub fn page_token_opt(&self) -> Option<&[u8]> {
            self.page_token.as_ref().map(|field| field as _)
        }
        pub fn set_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.page_token = Some(field.into().into());
        }
        pub fn with_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_page_token(field);
            self
        }
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field);
            self
        }
        pub fn object_type_opt_mut(&mut self) -> Option<&mut String> {
            self.object_type.as_mut().map(|field| field as _)
        }
        pub fn object_type_mut(&mut self) -> &mut String {
            self.object_type.get_or_insert_default()
        }
        pub fn object_type_opt(&self) -> Option<&str> {
            self.object_type.as_ref().map(|field| field as _)
        }
        pub fn set_object_type<T: Into<String>>(&mut self, field: T) {
            self.object_type = Some(field.into().into());
        }
        pub fn with_object_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_type(field);
            self
        }
    }
    impl super::ListOwnedObjectsResponse {
        pub const fn const_default() -> Self {
            Self {
                objects: Vec::new(),
                next_page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListOwnedObjectsResponse = super::ListOwnedObjectsResponse::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[super::Object] {
            &self.objects
        }
        pub fn set_objects(&mut self, field: Vec<super::Object>) {
            self.objects = field;
        }
        pub fn with_objects(mut self, field: Vec<super::Object>) -> Self {
            self.set_objects(field);
            self
        }
        pub fn next_page_token_opt(&self) -> Option<&[u8]> {
            self.next_page_token.as_ref().map(|field| field as _)
        }
        pub fn set_next_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.next_page_token = Some(field.into().into());
        }
        pub fn with_next_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_page_token(field);
            self
        }
    }
    impl super::ListPackageVersionsRequest {
        pub const fn const_default() -> Self {
            Self {
                package_id: None,
                page_size: None,
                page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListPackageVersionsRequest = super::ListPackageVersionsRequest::const_default();
            &DEFAULT
        }
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field);
            self
        }
        pub fn page_size_opt_mut(&mut self) -> Option<&mut u32> {
            self.page_size.as_mut().map(|field| field as _)
        }
        pub fn page_size_mut(&mut self) -> &mut u32 {
            self.page_size.get_or_insert_default()
        }
        pub fn page_size_opt(&self) -> Option<u32> {
            self.page_size.as_ref().map(|field| *field)
        }
        pub fn set_page_size<T: Into<u32>>(&mut self, field: T) {
            self.page_size = Some(field.into().into());
        }
        pub fn with_page_size<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_page_size(field);
            self
        }
        pub fn page_token_opt(&self) -> Option<&[u8]> {
            self.page_token.as_ref().map(|field| field as _)
        }
        pub fn set_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.page_token = Some(field.into().into());
        }
        pub fn with_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_page_token(field);
            self
        }
    }
    impl super::ListPackageVersionsResponse {
        pub const fn const_default() -> Self {
            Self {
                versions: Vec::new(),
                next_page_token: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListPackageVersionsResponse = super::ListPackageVersionsResponse::const_default();
            &DEFAULT
        }
        pub fn versions(&self) -> &[super::PackageVersion] {
            &self.versions
        }
        pub fn set_versions(&mut self, field: Vec<super::PackageVersion>) {
            self.versions = field;
        }
        pub fn with_versions(mut self, field: Vec<super::PackageVersion>) -> Self {
            self.set_versions(field);
            self
        }
        pub fn next_page_token_opt(&self) -> Option<&[u8]> {
            self.next_page_token.as_ref().map(|field| field as _)
        }
        pub fn set_next_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.next_page_token = Some(field.into().into());
        }
        pub fn with_next_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_page_token(field);
            self
        }
    }
    impl super::LookupNameRequest {
        pub const fn const_default() -> Self {
            Self { name: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::LookupNameRequest = super::LookupNameRequest::const_default();
            &DEFAULT
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
    }
    impl super::LookupNameResponse {
        pub const fn const_default() -> Self {
            Self { record: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::LookupNameResponse = super::LookupNameResponse::const_default();
            &DEFAULT
        }
        pub fn record(&self) -> &super::NameRecord {
            self.record
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::NameRecord::default_instance() as _)
        }
        pub fn record_opt_mut(&mut self) -> Option<&mut super::NameRecord> {
            self.record.as_mut().map(|field| field as _)
        }
        pub fn record_mut(&mut self) -> &mut super::NameRecord {
            self.record.get_or_insert_default()
        }
        pub fn record_opt(&self) -> Option<&super::NameRecord> {
            self.record.as_ref().map(|field| field as _)
        }
        pub fn set_record<T: Into<super::NameRecord>>(&mut self, field: T) {
            self.record = Some(field.into().into());
        }
        pub fn with_record<T: Into<super::NameRecord>>(mut self, field: T) -> Self {
            self.set_record(field);
            self
        }
    }
    impl super::MakeMoveVector {
        pub const fn const_default() -> Self {
            Self {
                element_type: None,
                elements: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MakeMoveVector = super::MakeMoveVector::const_default();
            &DEFAULT
        }
        pub fn element_type_opt_mut(&mut self) -> Option<&mut String> {
            self.element_type.as_mut().map(|field| field as _)
        }
        pub fn element_type_mut(&mut self) -> &mut String {
            self.element_type.get_or_insert_default()
        }
        pub fn element_type_opt(&self) -> Option<&str> {
            self.element_type.as_ref().map(|field| field as _)
        }
        pub fn set_element_type<T: Into<String>>(&mut self, field: T) {
            self.element_type = Some(field.into().into());
        }
        pub fn with_element_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_element_type(field);
            self
        }
        pub fn elements(&self) -> &[super::Argument] {
            &self.elements
        }
        pub fn set_elements(&mut self, field: Vec<super::Argument>) {
            self.elements = field;
        }
        pub fn with_elements(mut self, field: Vec<super::Argument>) -> Self {
            self.set_elements(field);
            self
        }
    }
    impl super::MergeCoins {
        pub const fn const_default() -> Self {
            Self {
                coin: None,
                coins_to_merge: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MergeCoins = super::MergeCoins::const_default();
            &DEFAULT
        }
        pub fn coin(&self) -> &super::Argument {
            self.coin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Argument::default_instance() as _)
        }
        pub fn coin_opt_mut(&mut self) -> Option<&mut super::Argument> {
            self.coin.as_mut().map(|field| field as _)
        }
        pub fn coin_mut(&mut self) -> &mut super::Argument {
            self.coin.get_or_insert_default()
        }
        pub fn coin_opt(&self) -> Option<&super::Argument> {
            self.coin.as_ref().map(|field| field as _)
        }
        pub fn set_coin<T: Into<super::Argument>>(&mut self, field: T) {
            self.coin = Some(field.into().into());
        }
        pub fn with_coin<T: Into<super::Argument>>(mut self, field: T) -> Self {
            self.set_coin(field);
            self
        }
        pub fn coins_to_merge(&self) -> &[super::Argument] {
            &self.coins_to_merge
        }
        pub fn set_coins_to_merge(&mut self, field: Vec<super::Argument>) {
            self.coins_to_merge = field;
        }
        pub fn with_coins_to_merge(mut self, field: Vec<super::Argument>) -> Self {
            self.set_coins_to_merge(field);
            self
        }
    }
    impl super::Module {
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
            static DEFAULT: super::Module = super::Module::const_default();
            &DEFAULT
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
        pub fn contents_opt(&self) -> Option<&[u8]> {
            self.contents.as_ref().map(|field| field as _)
        }
        pub fn set_contents<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.contents = Some(field.into().into());
        }
        pub fn with_contents<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_contents(field);
            self
        }
        pub fn datatypes(&self) -> &[super::DatatypeDescriptor] {
            &self.datatypes
        }
        pub fn set_datatypes(&mut self, field: Vec<super::DatatypeDescriptor>) {
            self.datatypes = field;
        }
        pub fn with_datatypes(mut self, field: Vec<super::DatatypeDescriptor>) -> Self {
            self.set_datatypes(field);
            self
        }
        pub fn functions(&self) -> &[super::FunctionDescriptor] {
            &self.functions
        }
        pub fn set_functions(&mut self, field: Vec<super::FunctionDescriptor>) {
            self.functions = field;
        }
        pub fn with_functions(mut self, field: Vec<super::FunctionDescriptor>) -> Self {
            self.set_functions(field);
            self
        }
    }
    impl super::MoveAbort {
        pub const fn const_default() -> Self {
            Self {
                abort_code: None,
                location: None,
                clever_error: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MoveAbort = super::MoveAbort::const_default();
            &DEFAULT
        }
        pub fn abort_code_opt_mut(&mut self) -> Option<&mut u64> {
            self.abort_code.as_mut().map(|field| field as _)
        }
        pub fn abort_code_mut(&mut self) -> &mut u64 {
            self.abort_code.get_or_insert_default()
        }
        pub fn abort_code_opt(&self) -> Option<u64> {
            self.abort_code.as_ref().map(|field| *field)
        }
        pub fn set_abort_code<T: Into<u64>>(&mut self, field: T) {
            self.abort_code = Some(field.into().into());
        }
        pub fn with_abort_code<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_abort_code(field);
            self
        }
        pub fn location(&self) -> &super::MoveLocation {
            self.location
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveLocation::default_instance() as _)
        }
        pub fn location_opt_mut(&mut self) -> Option<&mut super::MoveLocation> {
            self.location.as_mut().map(|field| field as _)
        }
        pub fn location_mut(&mut self) -> &mut super::MoveLocation {
            self.location.get_or_insert_default()
        }
        pub fn location_opt(&self) -> Option<&super::MoveLocation> {
            self.location.as_ref().map(|field| field as _)
        }
        pub fn set_location<T: Into<super::MoveLocation>>(&mut self, field: T) {
            self.location = Some(field.into().into());
        }
        pub fn with_location<T: Into<super::MoveLocation>>(mut self, field: T) -> Self {
            self.set_location(field);
            self
        }
        pub fn clever_error(&self) -> &super::CleverError {
            self.clever_error
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CleverError::default_instance() as _)
        }
        pub fn clever_error_opt_mut(&mut self) -> Option<&mut super::CleverError> {
            self.clever_error.as_mut().map(|field| field as _)
        }
        pub fn clever_error_mut(&mut self) -> &mut super::CleverError {
            self.clever_error.get_or_insert_default()
        }
        pub fn clever_error_opt(&self) -> Option<&super::CleverError> {
            self.clever_error.as_ref().map(|field| field as _)
        }
        pub fn set_clever_error<T: Into<super::CleverError>>(&mut self, field: T) {
            self.clever_error = Some(field.into().into());
        }
        pub fn with_clever_error<T: Into<super::CleverError>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_clever_error(field);
            self
        }
    }
    impl super::MoveCall {
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
            static DEFAULT: super::MoveCall = super::MoveCall::const_default();
            &DEFAULT
        }
        pub fn package_opt_mut(&mut self) -> Option<&mut String> {
            self.package.as_mut().map(|field| field as _)
        }
        pub fn package_mut(&mut self) -> &mut String {
            self.package.get_or_insert_default()
        }
        pub fn package_opt(&self) -> Option<&str> {
            self.package.as_ref().map(|field| field as _)
        }
        pub fn set_package<T: Into<String>>(&mut self, field: T) {
            self.package = Some(field.into().into());
        }
        pub fn with_package<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package(field);
            self
        }
        pub fn module_opt_mut(&mut self) -> Option<&mut String> {
            self.module.as_mut().map(|field| field as _)
        }
        pub fn module_mut(&mut self) -> &mut String {
            self.module.get_or_insert_default()
        }
        pub fn module_opt(&self) -> Option<&str> {
            self.module.as_ref().map(|field| field as _)
        }
        pub fn set_module<T: Into<String>>(&mut self, field: T) {
            self.module = Some(field.into().into());
        }
        pub fn with_module<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module(field);
            self
        }
        pub fn function_opt_mut(&mut self) -> Option<&mut String> {
            self.function.as_mut().map(|field| field as _)
        }
        pub fn function_mut(&mut self) -> &mut String {
            self.function.get_or_insert_default()
        }
        pub fn function_opt(&self) -> Option<&str> {
            self.function.as_ref().map(|field| field as _)
        }
        pub fn set_function<T: Into<String>>(&mut self, field: T) {
            self.function = Some(field.into().into());
        }
        pub fn with_function<T: Into<String>>(mut self, field: T) -> Self {
            self.set_function(field);
            self
        }
        pub fn type_arguments(&self) -> &[String] {
            &self.type_arguments
        }
        pub fn set_type_arguments(&mut self, field: Vec<String>) {
            self.type_arguments = field;
        }
        pub fn with_type_arguments(mut self, field: Vec<String>) -> Self {
            self.set_type_arguments(field);
            self
        }
        pub fn arguments(&self) -> &[super::Argument] {
            &self.arguments
        }
        pub fn set_arguments(&mut self, field: Vec<super::Argument>) {
            self.arguments = field;
        }
        pub fn with_arguments(mut self, field: Vec<super::Argument>) -> Self {
            self.set_arguments(field);
            self
        }
    }
    impl super::MoveLocation {
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
            static DEFAULT: super::MoveLocation = super::MoveLocation::const_default();
            &DEFAULT
        }
        pub fn package_opt_mut(&mut self) -> Option<&mut String> {
            self.package.as_mut().map(|field| field as _)
        }
        pub fn package_mut(&mut self) -> &mut String {
            self.package.get_or_insert_default()
        }
        pub fn package_opt(&self) -> Option<&str> {
            self.package.as_ref().map(|field| field as _)
        }
        pub fn set_package<T: Into<String>>(&mut self, field: T) {
            self.package = Some(field.into().into());
        }
        pub fn with_package<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package(field);
            self
        }
        pub fn module_opt_mut(&mut self) -> Option<&mut String> {
            self.module.as_mut().map(|field| field as _)
        }
        pub fn module_mut(&mut self) -> &mut String {
            self.module.get_or_insert_default()
        }
        pub fn module_opt(&self) -> Option<&str> {
            self.module.as_ref().map(|field| field as _)
        }
        pub fn set_module<T: Into<String>>(&mut self, field: T) {
            self.module = Some(field.into().into());
        }
        pub fn with_module<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module(field);
            self
        }
        pub fn function_opt_mut(&mut self) -> Option<&mut u32> {
            self.function.as_mut().map(|field| field as _)
        }
        pub fn function_mut(&mut self) -> &mut u32 {
            self.function.get_or_insert_default()
        }
        pub fn function_opt(&self) -> Option<u32> {
            self.function.as_ref().map(|field| *field)
        }
        pub fn set_function<T: Into<u32>>(&mut self, field: T) {
            self.function = Some(field.into().into());
        }
        pub fn with_function<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_function(field);
            self
        }
        pub fn instruction_opt_mut(&mut self) -> Option<&mut u32> {
            self.instruction.as_mut().map(|field| field as _)
        }
        pub fn instruction_mut(&mut self) -> &mut u32 {
            self.instruction.get_or_insert_default()
        }
        pub fn instruction_opt(&self) -> Option<u32> {
            self.instruction.as_ref().map(|field| *field)
        }
        pub fn set_instruction<T: Into<u32>>(&mut self, field: T) {
            self.instruction = Some(field.into().into());
        }
        pub fn with_instruction<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_instruction(field);
            self
        }
        pub fn function_name_opt_mut(&mut self) -> Option<&mut String> {
            self.function_name.as_mut().map(|field| field as _)
        }
        pub fn function_name_mut(&mut self) -> &mut String {
            self.function_name.get_or_insert_default()
        }
        pub fn function_name_opt(&self) -> Option<&str> {
            self.function_name.as_ref().map(|field| field as _)
        }
        pub fn set_function_name<T: Into<String>>(&mut self, field: T) {
            self.function_name = Some(field.into().into());
        }
        pub fn with_function_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_function_name(field);
            self
        }
    }
    impl super::MoveTable {
        pub const fn const_default() -> Self {
            Self { id: None, size: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MoveTable = super::MoveTable::const_default();
            &DEFAULT
        }
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field);
            self
        }
        pub fn size_opt_mut(&mut self) -> Option<&mut u64> {
            self.size.as_mut().map(|field| field as _)
        }
        pub fn size_mut(&mut self) -> &mut u64 {
            self.size.get_or_insert_default()
        }
        pub fn size_opt(&self) -> Option<u64> {
            self.size.as_ref().map(|field| *field)
        }
        pub fn set_size<T: Into<u64>>(&mut self, field: T) {
            self.size = Some(field.into().into());
        }
        pub fn with_size<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_size(field);
            self
        }
    }
    impl super::MultisigAggregatedSignature {
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
            static DEFAULT: super::MultisigAggregatedSignature = super::MultisigAggregatedSignature::const_default();
            &DEFAULT
        }
        pub fn signatures(&self) -> &[super::MultisigMemberSignature] {
            &self.signatures
        }
        pub fn set_signatures(&mut self, field: Vec<super::MultisigMemberSignature>) {
            self.signatures = field;
        }
        pub fn with_signatures(
            mut self,
            field: Vec<super::MultisigMemberSignature>,
        ) -> Self {
            self.set_signatures(field);
            self
        }
        pub fn bitmap_opt_mut(&mut self) -> Option<&mut u32> {
            self.bitmap.as_mut().map(|field| field as _)
        }
        pub fn bitmap_mut(&mut self) -> &mut u32 {
            self.bitmap.get_or_insert_default()
        }
        pub fn bitmap_opt(&self) -> Option<u32> {
            self.bitmap.as_ref().map(|field| *field)
        }
        pub fn set_bitmap<T: Into<u32>>(&mut self, field: T) {
            self.bitmap = Some(field.into().into());
        }
        pub fn with_bitmap<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_bitmap(field);
            self
        }
        pub fn legacy_bitmap(&self) -> &[u32] {
            &self.legacy_bitmap
        }
        pub fn set_legacy_bitmap(&mut self, field: Vec<u32>) {
            self.legacy_bitmap = field;
        }
        pub fn with_legacy_bitmap(mut self, field: Vec<u32>) -> Self {
            self.set_legacy_bitmap(field);
            self
        }
        pub fn committee(&self) -> &super::MultisigCommittee {
            self.committee
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MultisigCommittee::default_instance() as _)
        }
        pub fn committee_opt_mut(&mut self) -> Option<&mut super::MultisigCommittee> {
            self.committee.as_mut().map(|field| field as _)
        }
        pub fn committee_mut(&mut self) -> &mut super::MultisigCommittee {
            self.committee.get_or_insert_default()
        }
        pub fn committee_opt(&self) -> Option<&super::MultisigCommittee> {
            self.committee.as_ref().map(|field| field as _)
        }
        pub fn set_committee<T: Into<super::MultisigCommittee>>(&mut self, field: T) {
            self.committee = Some(field.into().into());
        }
        pub fn with_committee<T: Into<super::MultisigCommittee>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_committee(field);
            self
        }
    }
    impl super::MultisigCommittee {
        pub const fn const_default() -> Self {
            Self {
                members: Vec::new(),
                threshold: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MultisigCommittee = super::MultisigCommittee::const_default();
            &DEFAULT
        }
        pub fn members(&self) -> &[super::MultisigMember] {
            &self.members
        }
        pub fn set_members(&mut self, field: Vec<super::MultisigMember>) {
            self.members = field;
        }
        pub fn with_members(mut self, field: Vec<super::MultisigMember>) -> Self {
            self.set_members(field);
            self
        }
        pub fn threshold_opt_mut(&mut self) -> Option<&mut u32> {
            self.threshold.as_mut().map(|field| field as _)
        }
        pub fn threshold_mut(&mut self) -> &mut u32 {
            self.threshold.get_or_insert_default()
        }
        pub fn threshold_opt(&self) -> Option<u32> {
            self.threshold.as_ref().map(|field| *field)
        }
        pub fn set_threshold<T: Into<u32>>(&mut self, field: T) {
            self.threshold = Some(field.into().into());
        }
        pub fn with_threshold<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_threshold(field);
            self
        }
    }
    impl super::MultisigMember {
        pub const fn const_default() -> Self {
            Self {
                public_key: None,
                weight: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MultisigMember = super::MultisigMember::const_default();
            &DEFAULT
        }
        pub fn public_key(&self) -> &super::MultisigMemberPublicKey {
            self.public_key
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::MultisigMemberPublicKey::default_instance() as _
                })
        }
        pub fn public_key_opt_mut(
            &mut self,
        ) -> Option<&mut super::MultisigMemberPublicKey> {
            self.public_key.as_mut().map(|field| field as _)
        }
        pub fn public_key_mut(&mut self) -> &mut super::MultisigMemberPublicKey {
            self.public_key.get_or_insert_default()
        }
        pub fn public_key_opt(&self) -> Option<&super::MultisigMemberPublicKey> {
            self.public_key.as_ref().map(|field| field as _)
        }
        pub fn set_public_key<T: Into<super::MultisigMemberPublicKey>>(
            &mut self,
            field: T,
        ) {
            self.public_key = Some(field.into().into());
        }
        pub fn with_public_key<T: Into<super::MultisigMemberPublicKey>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_public_key(field);
            self
        }
        pub fn weight_opt_mut(&mut self) -> Option<&mut u32> {
            self.weight.as_mut().map(|field| field as _)
        }
        pub fn weight_mut(&mut self) -> &mut u32 {
            self.weight.get_or_insert_default()
        }
        pub fn weight_opt(&self) -> Option<u32> {
            self.weight.as_ref().map(|field| *field)
        }
        pub fn set_weight<T: Into<u32>>(&mut self, field: T) {
            self.weight = Some(field.into().into());
        }
        pub fn with_weight<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_weight(field);
            self
        }
    }
    impl super::MultisigMemberPublicKey {
        pub const fn const_default() -> Self {
            Self {
                scheme: None,
                public_key: None,
                zklogin: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MultisigMemberPublicKey = super::MultisigMemberPublicKey::const_default();
            &DEFAULT
        }
        pub fn public_key_opt(&self) -> Option<&[u8]> {
            self.public_key.as_ref().map(|field| field as _)
        }
        pub fn set_public_key<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.public_key = Some(field.into().into());
        }
        pub fn with_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_public_key(field);
            self
        }
        pub fn zklogin(&self) -> &super::ZkLoginPublicIdentifier {
            self.zklogin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::ZkLoginPublicIdentifier::default_instance() as _
                })
        }
        pub fn zklogin_opt_mut(
            &mut self,
        ) -> Option<&mut super::ZkLoginPublicIdentifier> {
            self.zklogin.as_mut().map(|field| field as _)
        }
        pub fn zklogin_mut(&mut self) -> &mut super::ZkLoginPublicIdentifier {
            self.zklogin.get_or_insert_default()
        }
        pub fn zklogin_opt(&self) -> Option<&super::ZkLoginPublicIdentifier> {
            self.zklogin.as_ref().map(|field| field as _)
        }
        pub fn set_zklogin<T: Into<super::ZkLoginPublicIdentifier>>(
            &mut self,
            field: T,
        ) {
            self.zklogin = Some(field.into().into());
        }
        pub fn with_zklogin<T: Into<super::ZkLoginPublicIdentifier>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_zklogin(field);
            self
        }
    }
    impl super::MultisigMemberSignature {
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
            static DEFAULT: super::MultisigMemberSignature = super::MultisigMemberSignature::const_default();
            &DEFAULT
        }
        pub fn signature_opt(&self) -> Option<&[u8]> {
            self.signature.as_ref().map(|field| field as _)
        }
        pub fn set_signature<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        pub fn with_signature<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field);
            self
        }
        pub fn zklogin(&self) -> &super::ZkLoginAuthenticator {
            self.zklogin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ZkLoginAuthenticator::default_instance() as _)
        }
        pub fn zklogin_opt_mut(&mut self) -> Option<&mut super::ZkLoginAuthenticator> {
            self.zklogin.as_mut().map(|field| field as _)
        }
        pub fn zklogin_mut(&mut self) -> &mut super::ZkLoginAuthenticator {
            self.zklogin.get_or_insert_default()
        }
        pub fn zklogin_opt(&self) -> Option<&super::ZkLoginAuthenticator> {
            self.zklogin.as_ref().map(|field| field as _)
        }
        pub fn set_zklogin<T: Into<super::ZkLoginAuthenticator>>(&mut self, field: T) {
            self.zklogin = Some(field.into().into());
        }
        pub fn with_zklogin<T: Into<super::ZkLoginAuthenticator>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_zklogin(field);
            self
        }
        pub fn passkey(&self) -> &super::PasskeyAuthenticator {
            self.passkey
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::PasskeyAuthenticator::default_instance() as _)
        }
        pub fn passkey_opt_mut(&mut self) -> Option<&mut super::PasskeyAuthenticator> {
            self.passkey.as_mut().map(|field| field as _)
        }
        pub fn passkey_mut(&mut self) -> &mut super::PasskeyAuthenticator {
            self.passkey.get_or_insert_default()
        }
        pub fn passkey_opt(&self) -> Option<&super::PasskeyAuthenticator> {
            self.passkey.as_ref().map(|field| field as _)
        }
        pub fn set_passkey<T: Into<super::PasskeyAuthenticator>>(&mut self, field: T) {
            self.passkey = Some(field.into().into());
        }
        pub fn with_passkey<T: Into<super::PasskeyAuthenticator>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_passkey(field);
            self
        }
    }
    impl super::NameRecord {
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
            static DEFAULT: super::NameRecord = super::NameRecord::const_default();
            &DEFAULT
        }
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field);
            self
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
        pub fn registration_nft_id_opt_mut(&mut self) -> Option<&mut String> {
            self.registration_nft_id.as_mut().map(|field| field as _)
        }
        pub fn registration_nft_id_mut(&mut self) -> &mut String {
            self.registration_nft_id.get_or_insert_default()
        }
        pub fn registration_nft_id_opt(&self) -> Option<&str> {
            self.registration_nft_id.as_ref().map(|field| field as _)
        }
        pub fn set_registration_nft_id<T: Into<String>>(&mut self, field: T) {
            self.registration_nft_id = Some(field.into().into());
        }
        pub fn with_registration_nft_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_registration_nft_id(field);
            self
        }
        pub fn expiration_timestamp_opt_mut(
            &mut self,
        ) -> Option<&mut ::prost_types::Timestamp> {
            self.expiration_timestamp.as_mut().map(|field| field as _)
        }
        pub fn expiration_timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.expiration_timestamp.get_or_insert_default()
        }
        pub fn expiration_timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.expiration_timestamp.as_ref().map(|field| field as _)
        }
        pub fn set_expiration_timestamp<T: Into<::prost_types::Timestamp>>(
            &mut self,
            field: T,
        ) {
            self.expiration_timestamp = Some(field.into().into());
        }
        pub fn with_expiration_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_expiration_timestamp(field);
            self
        }
        pub fn target_address_opt_mut(&mut self) -> Option<&mut String> {
            self.target_address.as_mut().map(|field| field as _)
        }
        pub fn target_address_mut(&mut self) -> &mut String {
            self.target_address.get_or_insert_default()
        }
        pub fn target_address_opt(&self) -> Option<&str> {
            self.target_address.as_ref().map(|field| field as _)
        }
        pub fn set_target_address<T: Into<String>>(&mut self, field: T) {
            self.target_address = Some(field.into().into());
        }
        pub fn with_target_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_target_address(field);
            self
        }
    }
    impl super::Object {
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
            static DEFAULT: super::Object = super::Object::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field);
            self
        }
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<u64>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn owner(&self) -> &super::Owner {
            self.owner
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Owner::default_instance() as _)
        }
        pub fn owner_opt_mut(&mut self) -> Option<&mut super::Owner> {
            self.owner.as_mut().map(|field| field as _)
        }
        pub fn owner_mut(&mut self) -> &mut super::Owner {
            self.owner.get_or_insert_default()
        }
        pub fn owner_opt(&self) -> Option<&super::Owner> {
            self.owner.as_ref().map(|field| field as _)
        }
        pub fn set_owner<T: Into<super::Owner>>(&mut self, field: T) {
            self.owner = Some(field.into().into());
        }
        pub fn with_owner<T: Into<super::Owner>>(mut self, field: T) -> Self {
            self.set_owner(field);
            self
        }
        pub fn object_type_opt_mut(&mut self) -> Option<&mut String> {
            self.object_type.as_mut().map(|field| field as _)
        }
        pub fn object_type_mut(&mut self) -> &mut String {
            self.object_type.get_or_insert_default()
        }
        pub fn object_type_opt(&self) -> Option<&str> {
            self.object_type.as_ref().map(|field| field as _)
        }
        pub fn set_object_type<T: Into<String>>(&mut self, field: T) {
            self.object_type = Some(field.into().into());
        }
        pub fn with_object_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_type(field);
            self
        }
        pub fn has_public_transfer_opt_mut(&mut self) -> Option<&mut bool> {
            self.has_public_transfer.as_mut().map(|field| field as _)
        }
        pub fn has_public_transfer_mut(&mut self) -> &mut bool {
            self.has_public_transfer.get_or_insert_default()
        }
        pub fn has_public_transfer_opt(&self) -> Option<bool> {
            self.has_public_transfer.as_ref().map(|field| *field)
        }
        pub fn set_has_public_transfer<T: Into<bool>>(&mut self, field: T) {
            self.has_public_transfer = Some(field.into().into());
        }
        pub fn with_has_public_transfer<T: Into<bool>>(mut self, field: T) -> Self {
            self.set_has_public_transfer(field);
            self
        }
        pub fn contents(&self) -> &super::Bcs {
            self.contents
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        pub fn contents_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.contents.as_mut().map(|field| field as _)
        }
        pub fn contents_mut(&mut self) -> &mut super::Bcs {
            self.contents.get_or_insert_default()
        }
        pub fn contents_opt(&self) -> Option<&super::Bcs> {
            self.contents.as_ref().map(|field| field as _)
        }
        pub fn set_contents<T: Into<super::Bcs>>(&mut self, field: T) {
            self.contents = Some(field.into().into());
        }
        pub fn with_contents<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_contents(field);
            self
        }
        pub fn package(&self) -> &super::Package {
            self.package
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Package::default_instance() as _)
        }
        pub fn package_opt_mut(&mut self) -> Option<&mut super::Package> {
            self.package.as_mut().map(|field| field as _)
        }
        pub fn package_mut(&mut self) -> &mut super::Package {
            self.package.get_or_insert_default()
        }
        pub fn package_opt(&self) -> Option<&super::Package> {
            self.package.as_ref().map(|field| field as _)
        }
        pub fn set_package<T: Into<super::Package>>(&mut self, field: T) {
            self.package = Some(field.into().into());
        }
        pub fn with_package<T: Into<super::Package>>(mut self, field: T) -> Self {
            self.set_package(field);
            self
        }
        pub fn previous_transaction_opt_mut(&mut self) -> Option<&mut String> {
            self.previous_transaction.as_mut().map(|field| field as _)
        }
        pub fn previous_transaction_mut(&mut self) -> &mut String {
            self.previous_transaction.get_or_insert_default()
        }
        pub fn previous_transaction_opt(&self) -> Option<&str> {
            self.previous_transaction.as_ref().map(|field| field as _)
        }
        pub fn set_previous_transaction<T: Into<String>>(&mut self, field: T) {
            self.previous_transaction = Some(field.into().into());
        }
        pub fn with_previous_transaction<T: Into<String>>(mut self, field: T) -> Self {
            self.set_previous_transaction(field);
            self
        }
        pub fn storage_rebate_opt_mut(&mut self) -> Option<&mut u64> {
            self.storage_rebate.as_mut().map(|field| field as _)
        }
        pub fn storage_rebate_mut(&mut self) -> &mut u64 {
            self.storage_rebate.get_or_insert_default()
        }
        pub fn storage_rebate_opt(&self) -> Option<u64> {
            self.storage_rebate.as_ref().map(|field| *field)
        }
        pub fn set_storage_rebate<T: Into<u64>>(&mut self, field: T) {
            self.storage_rebate = Some(field.into().into());
        }
        pub fn with_storage_rebate<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_storage_rebate(field);
            self
        }
        pub fn json_opt_mut(&mut self) -> Option<&mut ::prost_types::Value> {
            self.json.as_mut().map(|field| field as _)
        }
        pub fn json_mut(&mut self) -> &mut ::prost_types::Value {
            self.json.get_or_insert_default()
        }
        pub fn json_opt(&self) -> Option<&::prost_types::Value> {
            self.json.as_ref().map(|field| field as _)
        }
        pub fn set_json<T: Into<::prost_types::Value>>(&mut self, field: T) {
            self.json = Some(field.into().into());
        }
        pub fn with_json<T: Into<::prost_types::Value>>(mut self, field: T) -> Self {
            self.set_json(field);
            self
        }
        pub fn balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.balance.as_mut().map(|field| field as _)
        }
        pub fn balance_mut(&mut self) -> &mut u64 {
            self.balance.get_or_insert_default()
        }
        pub fn balance_opt(&self) -> Option<u64> {
            self.balance.as_ref().map(|field| *field)
        }
        pub fn set_balance<T: Into<u64>>(&mut self, field: T) {
            self.balance = Some(field.into().into());
        }
        pub fn with_balance<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_balance(field);
            self
        }
    }
    impl super::ObjectReference {
        pub const fn const_default() -> Self {
            Self {
                object_id: None,
                version: None,
                digest: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ObjectReference = super::ObjectReference::const_default();
            &DEFAULT
        }
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<u64>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
    }
    impl super::OpenSignature {
        pub const fn const_default() -> Self {
            Self {
                reference: None,
                body: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::OpenSignature = super::OpenSignature::const_default();
            &DEFAULT
        }
        pub fn body(&self) -> &super::OpenSignatureBody {
            self.body
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::OpenSignatureBody::default_instance() as _)
        }
        pub fn body_opt_mut(&mut self) -> Option<&mut super::OpenSignatureBody> {
            self.body.as_mut().map(|field| field as _)
        }
        pub fn body_mut(&mut self) -> &mut super::OpenSignatureBody {
            self.body.get_or_insert_default()
        }
        pub fn body_opt(&self) -> Option<&super::OpenSignatureBody> {
            self.body.as_ref().map(|field| field as _)
        }
        pub fn set_body<T: Into<super::OpenSignatureBody>>(&mut self, field: T) {
            self.body = Some(field.into().into());
        }
        pub fn with_body<T: Into<super::OpenSignatureBody>>(mut self, field: T) -> Self {
            self.set_body(field);
            self
        }
    }
    impl super::OpenSignatureBody {
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
            static DEFAULT: super::OpenSignatureBody = super::OpenSignatureBody::const_default();
            &DEFAULT
        }
        pub fn type_name_opt_mut(&mut self) -> Option<&mut String> {
            self.type_name.as_mut().map(|field| field as _)
        }
        pub fn type_name_mut(&mut self) -> &mut String {
            self.type_name.get_or_insert_default()
        }
        pub fn type_name_opt(&self) -> Option<&str> {
            self.type_name.as_ref().map(|field| field as _)
        }
        pub fn set_type_name<T: Into<String>>(&mut self, field: T) {
            self.type_name = Some(field.into().into());
        }
        pub fn with_type_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_type_name(field);
            self
        }
        pub fn type_parameter_instantiation(&self) -> &[super::OpenSignatureBody] {
            &self.type_parameter_instantiation
        }
        pub fn set_type_parameter_instantiation(
            &mut self,
            field: Vec<super::OpenSignatureBody>,
        ) {
            self.type_parameter_instantiation = field;
        }
        pub fn with_type_parameter_instantiation(
            mut self,
            field: Vec<super::OpenSignatureBody>,
        ) -> Self {
            self.set_type_parameter_instantiation(field);
            self
        }
        pub fn type_parameter_opt_mut(&mut self) -> Option<&mut u32> {
            self.type_parameter.as_mut().map(|field| field as _)
        }
        pub fn type_parameter_mut(&mut self) -> &mut u32 {
            self.type_parameter.get_or_insert_default()
        }
        pub fn type_parameter_opt(&self) -> Option<u32> {
            self.type_parameter.as_ref().map(|field| *field)
        }
        pub fn set_type_parameter<T: Into<u32>>(&mut self, field: T) {
            self.type_parameter = Some(field.into().into());
        }
        pub fn with_type_parameter<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_type_parameter(field);
            self
        }
    }
    impl super::Owner {
        pub const fn const_default() -> Self {
            Self {
                kind: None,
                address: None,
                version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Owner = super::Owner::const_default();
            &DEFAULT
        }
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<u64>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
    }
    impl super::Package {
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
            static DEFAULT: super::Package = super::Package::const_default();
            &DEFAULT
        }
        pub fn storage_id_opt_mut(&mut self) -> Option<&mut String> {
            self.storage_id.as_mut().map(|field| field as _)
        }
        pub fn storage_id_mut(&mut self) -> &mut String {
            self.storage_id.get_or_insert_default()
        }
        pub fn storage_id_opt(&self) -> Option<&str> {
            self.storage_id.as_ref().map(|field| field as _)
        }
        pub fn set_storage_id<T: Into<String>>(&mut self, field: T) {
            self.storage_id = Some(field.into().into());
        }
        pub fn with_storage_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_storage_id(field);
            self
        }
        pub fn original_id_opt_mut(&mut self) -> Option<&mut String> {
            self.original_id.as_mut().map(|field| field as _)
        }
        pub fn original_id_mut(&mut self) -> &mut String {
            self.original_id.get_or_insert_default()
        }
        pub fn original_id_opt(&self) -> Option<&str> {
            self.original_id.as_ref().map(|field| field as _)
        }
        pub fn set_original_id<T: Into<String>>(&mut self, field: T) {
            self.original_id = Some(field.into().into());
        }
        pub fn with_original_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_original_id(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<u64>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn modules(&self) -> &[super::Module] {
            &self.modules
        }
        pub fn set_modules(&mut self, field: Vec<super::Module>) {
            self.modules = field;
        }
        pub fn with_modules(mut self, field: Vec<super::Module>) -> Self {
            self.set_modules(field);
            self
        }
        pub fn type_origins(&self) -> &[super::TypeOrigin] {
            &self.type_origins
        }
        pub fn set_type_origins(&mut self, field: Vec<super::TypeOrigin>) {
            self.type_origins = field;
        }
        pub fn with_type_origins(mut self, field: Vec<super::TypeOrigin>) -> Self {
            self.set_type_origins(field);
            self
        }
        pub fn linkage(&self) -> &[super::Linkage] {
            &self.linkage
        }
        pub fn set_linkage(&mut self, field: Vec<super::Linkage>) {
            self.linkage = field;
        }
        pub fn with_linkage(mut self, field: Vec<super::Linkage>) -> Self {
            self.set_linkage(field);
            self
        }
    }
    impl super::PackageUpgradeError {
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
            static DEFAULT: super::PackageUpgradeError = super::PackageUpgradeError::const_default();
            &DEFAULT
        }
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field);
            self
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn policy_opt_mut(&mut self) -> Option<&mut u32> {
            self.policy.as_mut().map(|field| field as _)
        }
        pub fn policy_mut(&mut self) -> &mut u32 {
            self.policy.get_or_insert_default()
        }
        pub fn policy_opt(&self) -> Option<u32> {
            self.policy.as_ref().map(|field| *field)
        }
        pub fn set_policy<T: Into<u32>>(&mut self, field: T) {
            self.policy = Some(field.into().into());
        }
        pub fn with_policy<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_policy(field);
            self
        }
        pub fn ticket_id_opt_mut(&mut self) -> Option<&mut String> {
            self.ticket_id.as_mut().map(|field| field as _)
        }
        pub fn ticket_id_mut(&mut self) -> &mut String {
            self.ticket_id.get_or_insert_default()
        }
        pub fn ticket_id_opt(&self) -> Option<&str> {
            self.ticket_id.as_ref().map(|field| field as _)
        }
        pub fn set_ticket_id<T: Into<String>>(&mut self, field: T) {
            self.ticket_id = Some(field.into().into());
        }
        pub fn with_ticket_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_ticket_id(field);
            self
        }
    }
    impl super::PackageVersion {
        pub const fn const_default() -> Self {
            Self {
                package_id: None,
                version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::PackageVersion = super::PackageVersion::const_default();
            &DEFAULT
        }
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<u64>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
    }
    impl super::PasskeyAuthenticator {
        pub const fn const_default() -> Self {
            Self {
                authenticator_data: None,
                client_data_json: None,
                signature: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::PasskeyAuthenticator = super::PasskeyAuthenticator::const_default();
            &DEFAULT
        }
        pub fn authenticator_data_opt(&self) -> Option<&[u8]> {
            self.authenticator_data.as_ref().map(|field| field as _)
        }
        pub fn set_authenticator_data<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.authenticator_data = Some(field.into().into());
        }
        pub fn with_authenticator_data<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_authenticator_data(field);
            self
        }
        pub fn client_data_json_opt_mut(&mut self) -> Option<&mut String> {
            self.client_data_json.as_mut().map(|field| field as _)
        }
        pub fn client_data_json_mut(&mut self) -> &mut String {
            self.client_data_json.get_or_insert_default()
        }
        pub fn client_data_json_opt(&self) -> Option<&str> {
            self.client_data_json.as_ref().map(|field| field as _)
        }
        pub fn set_client_data_json<T: Into<String>>(&mut self, field: T) {
            self.client_data_json = Some(field.into().into());
        }
        pub fn with_client_data_json<T: Into<String>>(mut self, field: T) -> Self {
            self.set_client_data_json(field);
            self
        }
        pub fn signature(&self) -> &super::SimpleSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::SimpleSignature::default_instance() as _)
        }
        pub fn signature_opt_mut(&mut self) -> Option<&mut super::SimpleSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        pub fn signature_mut(&mut self) -> &mut super::SimpleSignature {
            self.signature.get_or_insert_default()
        }
        pub fn signature_opt(&self) -> Option<&super::SimpleSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        pub fn set_signature<T: Into<super::SimpleSignature>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        pub fn with_signature<T: Into<super::SimpleSignature>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field);
            self
        }
    }
    impl super::ProgrammableTransaction {
        pub const fn const_default() -> Self {
            Self {
                inputs: Vec::new(),
                commands: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ProgrammableTransaction = super::ProgrammableTransaction::const_default();
            &DEFAULT
        }
        pub fn inputs(&self) -> &[super::Input] {
            &self.inputs
        }
        pub fn set_inputs(&mut self, field: Vec<super::Input>) {
            self.inputs = field;
        }
        pub fn with_inputs(mut self, field: Vec<super::Input>) -> Self {
            self.set_inputs(field);
            self
        }
        pub fn commands(&self) -> &[super::Command] {
            &self.commands
        }
        pub fn set_commands(&mut self, field: Vec<super::Command>) {
            self.commands = field;
        }
        pub fn with_commands(mut self, field: Vec<super::Command>) -> Self {
            self.set_commands(field);
            self
        }
    }
    impl super::ProtocolConfig {
        pub const fn const_default() -> Self {
            Self {
                protocol_version: None,
                feature_flags: std::collections::BTreeMap::new(),
                attributes: std::collections::BTreeMap::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ProtocolConfig = super::ProtocolConfig::const_default();
            &DEFAULT
        }
        pub fn protocol_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.protocol_version.as_mut().map(|field| field as _)
        }
        pub fn protocol_version_mut(&mut self) -> &mut u64 {
            self.protocol_version.get_or_insert_default()
        }
        pub fn protocol_version_opt(&self) -> Option<u64> {
            self.protocol_version.as_ref().map(|field| *field)
        }
        pub fn set_protocol_version<T: Into<u64>>(&mut self, field: T) {
            self.protocol_version = Some(field.into().into());
        }
        pub fn with_protocol_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_protocol_version(field);
            self
        }
    }
    impl super::Publish {
        pub const fn const_default() -> Self {
            Self {
                modules: Vec::new(),
                dependencies: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Publish = super::Publish::const_default();
            &DEFAULT
        }
        pub fn modules(&self) -> &[::prost::bytes::Bytes] {
            &self.modules
        }
        pub fn set_modules(&mut self, field: Vec<::prost::bytes::Bytes>) {
            self.modules = field;
        }
        pub fn with_modules(mut self, field: Vec<::prost::bytes::Bytes>) -> Self {
            self.set_modules(field);
            self
        }
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        pub fn set_dependencies(&mut self, field: Vec<String>) {
            self.dependencies = field;
        }
        pub fn with_dependencies(mut self, field: Vec<String>) -> Self {
            self.set_dependencies(field);
            self
        }
    }
    impl super::RandomnessStateUpdate {
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
            static DEFAULT: super::RandomnessStateUpdate = super::RandomnessStateUpdate::const_default();
            &DEFAULT
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn randomness_round_opt_mut(&mut self) -> Option<&mut u64> {
            self.randomness_round.as_mut().map(|field| field as _)
        }
        pub fn randomness_round_mut(&mut self) -> &mut u64 {
            self.randomness_round.get_or_insert_default()
        }
        pub fn randomness_round_opt(&self) -> Option<u64> {
            self.randomness_round.as_ref().map(|field| *field)
        }
        pub fn set_randomness_round<T: Into<u64>>(&mut self, field: T) {
            self.randomness_round = Some(field.into().into());
        }
        pub fn with_randomness_round<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_randomness_round(field);
            self
        }
        pub fn random_bytes_opt(&self) -> Option<&[u8]> {
            self.random_bytes.as_ref().map(|field| field as _)
        }
        pub fn set_random_bytes<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.random_bytes = Some(field.into().into());
        }
        pub fn with_random_bytes<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_random_bytes(field);
            self
        }
        pub fn randomness_object_initial_shared_version_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.randomness_object_initial_shared_version
                .as_mut()
                .map(|field| field as _)
        }
        pub fn randomness_object_initial_shared_version_mut(&mut self) -> &mut u64 {
            self.randomness_object_initial_shared_version.get_or_insert_default()
        }
        pub fn randomness_object_initial_shared_version_opt(&self) -> Option<u64> {
            self.randomness_object_initial_shared_version.as_ref().map(|field| *field)
        }
        pub fn set_randomness_object_initial_shared_version<T: Into<u64>>(
            &mut self,
            field: T,
        ) {
            self.randomness_object_initial_shared_version = Some(field.into().into());
        }
        pub fn with_randomness_object_initial_shared_version<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_randomness_object_initial_shared_version(field);
            self
        }
    }
    impl super::RegulatedCoinMetadata {
        pub const fn const_default() -> Self {
            Self {
                id: None,
                coin_metadata_object: None,
                deny_cap_object: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::RegulatedCoinMetadata = super::RegulatedCoinMetadata::const_default();
            &DEFAULT
        }
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field);
            self
        }
        pub fn coin_metadata_object_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_metadata_object.as_mut().map(|field| field as _)
        }
        pub fn coin_metadata_object_mut(&mut self) -> &mut String {
            self.coin_metadata_object.get_or_insert_default()
        }
        pub fn coin_metadata_object_opt(&self) -> Option<&str> {
            self.coin_metadata_object.as_ref().map(|field| field as _)
        }
        pub fn set_coin_metadata_object<T: Into<String>>(&mut self, field: T) {
            self.coin_metadata_object = Some(field.into().into());
        }
        pub fn with_coin_metadata_object<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_metadata_object(field);
            self
        }
        pub fn deny_cap_object_opt_mut(&mut self) -> Option<&mut String> {
            self.deny_cap_object.as_mut().map(|field| field as _)
        }
        pub fn deny_cap_object_mut(&mut self) -> &mut String {
            self.deny_cap_object.get_or_insert_default()
        }
        pub fn deny_cap_object_opt(&self) -> Option<&str> {
            self.deny_cap_object.as_ref().map(|field| field as _)
        }
        pub fn set_deny_cap_object<T: Into<String>>(&mut self, field: T) {
            self.deny_cap_object = Some(field.into().into());
        }
        pub fn with_deny_cap_object<T: Into<String>>(mut self, field: T) -> Self {
            self.set_deny_cap_object(field);
            self
        }
    }
    impl super::ReverseLookupNameRequest {
        pub const fn const_default() -> Self {
            Self { address: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ReverseLookupNameRequest = super::ReverseLookupNameRequest::const_default();
            &DEFAULT
        }
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field);
            self
        }
    }
    impl super::ReverseLookupNameResponse {
        pub const fn const_default() -> Self {
            Self { record: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ReverseLookupNameResponse = super::ReverseLookupNameResponse::const_default();
            &DEFAULT
        }
        pub fn record(&self) -> &super::NameRecord {
            self.record
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::NameRecord::default_instance() as _)
        }
        pub fn record_opt_mut(&mut self) -> Option<&mut super::NameRecord> {
            self.record.as_mut().map(|field| field as _)
        }
        pub fn record_mut(&mut self) -> &mut super::NameRecord {
            self.record.get_or_insert_default()
        }
        pub fn record_opt(&self) -> Option<&super::NameRecord> {
            self.record.as_ref().map(|field| field as _)
        }
        pub fn set_record<T: Into<super::NameRecord>>(&mut self, field: T) {
            self.record = Some(field.into().into());
        }
        pub fn with_record<T: Into<super::NameRecord>>(mut self, field: T) -> Self {
            self.set_record(field);
            self
        }
    }
    impl super::SimpleSignature {
        pub const fn const_default() -> Self {
            Self {
                scheme: None,
                signature: None,
                public_key: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SimpleSignature = super::SimpleSignature::const_default();
            &DEFAULT
        }
        pub fn signature_opt(&self) -> Option<&[u8]> {
            self.signature.as_ref().map(|field| field as _)
        }
        pub fn set_signature<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        pub fn with_signature<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field);
            self
        }
        pub fn public_key_opt(&self) -> Option<&[u8]> {
            self.public_key.as_ref().map(|field| field as _)
        }
        pub fn set_public_key<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.public_key = Some(field.into().into());
        }
        pub fn with_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_public_key(field);
            self
        }
    }
    impl super::SimulateTransactionRequest {
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
            static DEFAULT: super::SimulateTransactionRequest = super::SimulateTransactionRequest::const_default();
            &DEFAULT
        }
        pub fn transaction(&self) -> &super::Transaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Transaction::default_instance() as _)
        }
        pub fn transaction_opt_mut(&mut self) -> Option<&mut super::Transaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut super::Transaction {
            self.transaction.get_or_insert_default()
        }
        pub fn transaction_opt(&self) -> Option<&super::Transaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn set_transaction<T: Into<super::Transaction>>(&mut self, field: T) {
            self.transaction = Some(field.into().into());
        }
        pub fn with_transaction<T: Into<super::Transaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field);
            self
        }
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field);
            self
        }
        pub fn do_gas_selection_opt_mut(&mut self) -> Option<&mut bool> {
            self.do_gas_selection.as_mut().map(|field| field as _)
        }
        pub fn do_gas_selection_mut(&mut self) -> &mut bool {
            self.do_gas_selection.get_or_insert_default()
        }
        pub fn do_gas_selection_opt(&self) -> Option<bool> {
            self.do_gas_selection.as_ref().map(|field| *field)
        }
        pub fn set_do_gas_selection<T: Into<bool>>(&mut self, field: T) {
            self.do_gas_selection = Some(field.into().into());
        }
        pub fn with_do_gas_selection<T: Into<bool>>(mut self, field: T) -> Self {
            self.set_do_gas_selection(field);
            self
        }
    }
    impl super::SimulateTransactionResponse {
        pub const fn const_default() -> Self {
            Self {
                transaction: None,
                outputs: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SimulateTransactionResponse = super::SimulateTransactionResponse::const_default();
            &DEFAULT
        }
        pub fn transaction(&self) -> &super::ExecutedTransaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ExecutedTransaction::default_instance() as _)
        }
        pub fn transaction_opt_mut(
            &mut self,
        ) -> Option<&mut super::ExecutedTransaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        pub fn transaction_mut(&mut self) -> &mut super::ExecutedTransaction {
            self.transaction.get_or_insert_default()
        }
        pub fn transaction_opt(&self) -> Option<&super::ExecutedTransaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        pub fn set_transaction<T: Into<super::ExecutedTransaction>>(
            &mut self,
            field: T,
        ) {
            self.transaction = Some(field.into().into());
        }
        pub fn with_transaction<T: Into<super::ExecutedTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field);
            self
        }
        pub fn outputs(&self) -> &[super::CommandResult] {
            &self.outputs
        }
        pub fn set_outputs(&mut self, field: Vec<super::CommandResult>) {
            self.outputs = field;
        }
        pub fn with_outputs(mut self, field: Vec<super::CommandResult>) -> Self {
            self.set_outputs(field);
            self
        }
    }
    impl super::SizeError {
        pub const fn const_default() -> Self {
            Self { size: None, max_size: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SizeError = super::SizeError::const_default();
            &DEFAULT
        }
        pub fn size_opt_mut(&mut self) -> Option<&mut u64> {
            self.size.as_mut().map(|field| field as _)
        }
        pub fn size_mut(&mut self) -> &mut u64 {
            self.size.get_or_insert_default()
        }
        pub fn size_opt(&self) -> Option<u64> {
            self.size.as_ref().map(|field| *field)
        }
        pub fn set_size<T: Into<u64>>(&mut self, field: T) {
            self.size = Some(field.into().into());
        }
        pub fn with_size<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_size(field);
            self
        }
        pub fn max_size_opt_mut(&mut self) -> Option<&mut u64> {
            self.max_size.as_mut().map(|field| field as _)
        }
        pub fn max_size_mut(&mut self) -> &mut u64 {
            self.max_size.get_or_insert_default()
        }
        pub fn max_size_opt(&self) -> Option<u64> {
            self.max_size.as_ref().map(|field| *field)
        }
        pub fn set_max_size<T: Into<u64>>(&mut self, field: T) {
            self.max_size = Some(field.into().into());
        }
        pub fn with_max_size<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_max_size(field);
            self
        }
    }
    impl super::SplitCoins {
        pub const fn const_default() -> Self {
            Self {
                coin: None,
                amounts: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SplitCoins = super::SplitCoins::const_default();
            &DEFAULT
        }
        pub fn coin(&self) -> &super::Argument {
            self.coin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Argument::default_instance() as _)
        }
        pub fn coin_opt_mut(&mut self) -> Option<&mut super::Argument> {
            self.coin.as_mut().map(|field| field as _)
        }
        pub fn coin_mut(&mut self) -> &mut super::Argument {
            self.coin.get_or_insert_default()
        }
        pub fn coin_opt(&self) -> Option<&super::Argument> {
            self.coin.as_ref().map(|field| field as _)
        }
        pub fn set_coin<T: Into<super::Argument>>(&mut self, field: T) {
            self.coin = Some(field.into().into());
        }
        pub fn with_coin<T: Into<super::Argument>>(mut self, field: T) -> Self {
            self.set_coin(field);
            self
        }
        pub fn amounts(&self) -> &[super::Argument] {
            &self.amounts
        }
        pub fn set_amounts(&mut self, field: Vec<super::Argument>) {
            self.amounts = field;
        }
        pub fn with_amounts(mut self, field: Vec<super::Argument>) -> Self {
            self.set_amounts(field);
            self
        }
    }
    impl super::StakeSubsidy {
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
            static DEFAULT: super::StakeSubsidy = super::StakeSubsidy::const_default();
            &DEFAULT
        }
        pub fn balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.balance.as_mut().map(|field| field as _)
        }
        pub fn balance_mut(&mut self) -> &mut u64 {
            self.balance.get_or_insert_default()
        }
        pub fn balance_opt(&self) -> Option<u64> {
            self.balance.as_ref().map(|field| *field)
        }
        pub fn set_balance<T: Into<u64>>(&mut self, field: T) {
            self.balance = Some(field.into().into());
        }
        pub fn with_balance<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_balance(field);
            self
        }
        pub fn distribution_counter_opt_mut(&mut self) -> Option<&mut u64> {
            self.distribution_counter.as_mut().map(|field| field as _)
        }
        pub fn distribution_counter_mut(&mut self) -> &mut u64 {
            self.distribution_counter.get_or_insert_default()
        }
        pub fn distribution_counter_opt(&self) -> Option<u64> {
            self.distribution_counter.as_ref().map(|field| *field)
        }
        pub fn set_distribution_counter<T: Into<u64>>(&mut self, field: T) {
            self.distribution_counter = Some(field.into().into());
        }
        pub fn with_distribution_counter<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_distribution_counter(field);
            self
        }
        pub fn current_distribution_amount_opt_mut(&mut self) -> Option<&mut u64> {
            self.current_distribution_amount.as_mut().map(|field| field as _)
        }
        pub fn current_distribution_amount_mut(&mut self) -> &mut u64 {
            self.current_distribution_amount.get_or_insert_default()
        }
        pub fn current_distribution_amount_opt(&self) -> Option<u64> {
            self.current_distribution_amount.as_ref().map(|field| *field)
        }
        pub fn set_current_distribution_amount<T: Into<u64>>(&mut self, field: T) {
            self.current_distribution_amount = Some(field.into().into());
        }
        pub fn with_current_distribution_amount<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_current_distribution_amount(field);
            self
        }
        pub fn stake_subsidy_period_length_opt_mut(&mut self) -> Option<&mut u64> {
            self.stake_subsidy_period_length.as_mut().map(|field| field as _)
        }
        pub fn stake_subsidy_period_length_mut(&mut self) -> &mut u64 {
            self.stake_subsidy_period_length.get_or_insert_default()
        }
        pub fn stake_subsidy_period_length_opt(&self) -> Option<u64> {
            self.stake_subsidy_period_length.as_ref().map(|field| *field)
        }
        pub fn set_stake_subsidy_period_length<T: Into<u64>>(&mut self, field: T) {
            self.stake_subsidy_period_length = Some(field.into().into());
        }
        pub fn with_stake_subsidy_period_length<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_stake_subsidy_period_length(field);
            self
        }
        pub fn stake_subsidy_decrease_rate_opt_mut(&mut self) -> Option<&mut u32> {
            self.stake_subsidy_decrease_rate.as_mut().map(|field| field as _)
        }
        pub fn stake_subsidy_decrease_rate_mut(&mut self) -> &mut u32 {
            self.stake_subsidy_decrease_rate.get_or_insert_default()
        }
        pub fn stake_subsidy_decrease_rate_opt(&self) -> Option<u32> {
            self.stake_subsidy_decrease_rate.as_ref().map(|field| *field)
        }
        pub fn set_stake_subsidy_decrease_rate<T: Into<u32>>(&mut self, field: T) {
            self.stake_subsidy_decrease_rate = Some(field.into().into());
        }
        pub fn with_stake_subsidy_decrease_rate<T: Into<u32>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_stake_subsidy_decrease_rate(field);
            self
        }
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field);
            self
        }
    }
    impl super::StakingPool {
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
            static DEFAULT: super::StakingPool = super::StakingPool::const_default();
            &DEFAULT
        }
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field);
            self
        }
        pub fn activation_epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.activation_epoch.as_mut().map(|field| field as _)
        }
        pub fn activation_epoch_mut(&mut self) -> &mut u64 {
            self.activation_epoch.get_or_insert_default()
        }
        pub fn activation_epoch_opt(&self) -> Option<u64> {
            self.activation_epoch.as_ref().map(|field| *field)
        }
        pub fn set_activation_epoch<T: Into<u64>>(&mut self, field: T) {
            self.activation_epoch = Some(field.into().into());
        }
        pub fn with_activation_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_activation_epoch(field);
            self
        }
        pub fn deactivation_epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.deactivation_epoch.as_mut().map(|field| field as _)
        }
        pub fn deactivation_epoch_mut(&mut self) -> &mut u64 {
            self.deactivation_epoch.get_or_insert_default()
        }
        pub fn deactivation_epoch_opt(&self) -> Option<u64> {
            self.deactivation_epoch.as_ref().map(|field| *field)
        }
        pub fn set_deactivation_epoch<T: Into<u64>>(&mut self, field: T) {
            self.deactivation_epoch = Some(field.into().into());
        }
        pub fn with_deactivation_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_deactivation_epoch(field);
            self
        }
        pub fn sui_balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.sui_balance.as_mut().map(|field| field as _)
        }
        pub fn sui_balance_mut(&mut self) -> &mut u64 {
            self.sui_balance.get_or_insert_default()
        }
        pub fn sui_balance_opt(&self) -> Option<u64> {
            self.sui_balance.as_ref().map(|field| *field)
        }
        pub fn set_sui_balance<T: Into<u64>>(&mut self, field: T) {
            self.sui_balance = Some(field.into().into());
        }
        pub fn with_sui_balance<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_sui_balance(field);
            self
        }
        pub fn rewards_pool_opt_mut(&mut self) -> Option<&mut u64> {
            self.rewards_pool.as_mut().map(|field| field as _)
        }
        pub fn rewards_pool_mut(&mut self) -> &mut u64 {
            self.rewards_pool.get_or_insert_default()
        }
        pub fn rewards_pool_opt(&self) -> Option<u64> {
            self.rewards_pool.as_ref().map(|field| *field)
        }
        pub fn set_rewards_pool<T: Into<u64>>(&mut self, field: T) {
            self.rewards_pool = Some(field.into().into());
        }
        pub fn with_rewards_pool<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_rewards_pool(field);
            self
        }
        pub fn pool_token_balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.pool_token_balance.as_mut().map(|field| field as _)
        }
        pub fn pool_token_balance_mut(&mut self) -> &mut u64 {
            self.pool_token_balance.get_or_insert_default()
        }
        pub fn pool_token_balance_opt(&self) -> Option<u64> {
            self.pool_token_balance.as_ref().map(|field| *field)
        }
        pub fn set_pool_token_balance<T: Into<u64>>(&mut self, field: T) {
            self.pool_token_balance = Some(field.into().into());
        }
        pub fn with_pool_token_balance<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_pool_token_balance(field);
            self
        }
        pub fn exchange_rates(&self) -> &super::MoveTable {
            self.exchange_rates
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn exchange_rates_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.exchange_rates.as_mut().map(|field| field as _)
        }
        pub fn exchange_rates_mut(&mut self) -> &mut super::MoveTable {
            self.exchange_rates.get_or_insert_default()
        }
        pub fn exchange_rates_opt(&self) -> Option<&super::MoveTable> {
            self.exchange_rates.as_ref().map(|field| field as _)
        }
        pub fn set_exchange_rates<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.exchange_rates = Some(field.into().into());
        }
        pub fn with_exchange_rates<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_exchange_rates(field);
            self
        }
        pub fn pending_stake_opt_mut(&mut self) -> Option<&mut u64> {
            self.pending_stake.as_mut().map(|field| field as _)
        }
        pub fn pending_stake_mut(&mut self) -> &mut u64 {
            self.pending_stake.get_or_insert_default()
        }
        pub fn pending_stake_opt(&self) -> Option<u64> {
            self.pending_stake.as_ref().map(|field| *field)
        }
        pub fn set_pending_stake<T: Into<u64>>(&mut self, field: T) {
            self.pending_stake = Some(field.into().into());
        }
        pub fn with_pending_stake<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_pending_stake(field);
            self
        }
        pub fn pending_total_sui_withdraw_opt_mut(&mut self) -> Option<&mut u64> {
            self.pending_total_sui_withdraw.as_mut().map(|field| field as _)
        }
        pub fn pending_total_sui_withdraw_mut(&mut self) -> &mut u64 {
            self.pending_total_sui_withdraw.get_or_insert_default()
        }
        pub fn pending_total_sui_withdraw_opt(&self) -> Option<u64> {
            self.pending_total_sui_withdraw.as_ref().map(|field| *field)
        }
        pub fn set_pending_total_sui_withdraw<T: Into<u64>>(&mut self, field: T) {
            self.pending_total_sui_withdraw = Some(field.into().into());
        }
        pub fn with_pending_total_sui_withdraw<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_pending_total_sui_withdraw(field);
            self
        }
        pub fn pending_pool_token_withdraw_opt_mut(&mut self) -> Option<&mut u64> {
            self.pending_pool_token_withdraw.as_mut().map(|field| field as _)
        }
        pub fn pending_pool_token_withdraw_mut(&mut self) -> &mut u64 {
            self.pending_pool_token_withdraw.get_or_insert_default()
        }
        pub fn pending_pool_token_withdraw_opt(&self) -> Option<u64> {
            self.pending_pool_token_withdraw.as_ref().map(|field| *field)
        }
        pub fn set_pending_pool_token_withdraw<T: Into<u64>>(&mut self, field: T) {
            self.pending_pool_token_withdraw = Some(field.into().into());
        }
        pub fn with_pending_pool_token_withdraw<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_pending_pool_token_withdraw(field);
            self
        }
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field);
            self
        }
    }
    impl super::StorageFund {
        pub const fn const_default() -> Self {
            Self {
                total_object_storage_rebates: None,
                non_refundable_balance: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::StorageFund = super::StorageFund::const_default();
            &DEFAULT
        }
        pub fn total_object_storage_rebates_opt_mut(&mut self) -> Option<&mut u64> {
            self.total_object_storage_rebates.as_mut().map(|field| field as _)
        }
        pub fn total_object_storage_rebates_mut(&mut self) -> &mut u64 {
            self.total_object_storage_rebates.get_or_insert_default()
        }
        pub fn total_object_storage_rebates_opt(&self) -> Option<u64> {
            self.total_object_storage_rebates.as_ref().map(|field| *field)
        }
        pub fn set_total_object_storage_rebates<T: Into<u64>>(&mut self, field: T) {
            self.total_object_storage_rebates = Some(field.into().into());
        }
        pub fn with_total_object_storage_rebates<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_total_object_storage_rebates(field);
            self
        }
        pub fn non_refundable_balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.non_refundable_balance.as_mut().map(|field| field as _)
        }
        pub fn non_refundable_balance_mut(&mut self) -> &mut u64 {
            self.non_refundable_balance.get_or_insert_default()
        }
        pub fn non_refundable_balance_opt(&self) -> Option<u64> {
            self.non_refundable_balance.as_ref().map(|field| *field)
        }
        pub fn set_non_refundable_balance<T: Into<u64>>(&mut self, field: T) {
            self.non_refundable_balance = Some(field.into().into());
        }
        pub fn with_non_refundable_balance<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_non_refundable_balance(field);
            self
        }
    }
    impl super::SubscribeCheckpointsRequest {
        pub const fn const_default() -> Self {
            Self { read_mask: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SubscribeCheckpointsRequest = super::SubscribeCheckpointsRequest::const_default();
            &DEFAULT
        }
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field);
            self
        }
    }
    impl super::SubscribeCheckpointsResponse {
        pub const fn const_default() -> Self {
            Self {
                cursor: None,
                checkpoint: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SubscribeCheckpointsResponse = super::SubscribeCheckpointsResponse::const_default();
            &DEFAULT
        }
        pub fn cursor_opt_mut(&mut self) -> Option<&mut u64> {
            self.cursor.as_mut().map(|field| field as _)
        }
        pub fn cursor_mut(&mut self) -> &mut u64 {
            self.cursor.get_or_insert_default()
        }
        pub fn cursor_opt(&self) -> Option<u64> {
            self.cursor.as_ref().map(|field| *field)
        }
        pub fn set_cursor<T: Into<u64>>(&mut self, field: T) {
            self.cursor = Some(field.into().into());
        }
        pub fn with_cursor<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_cursor(field);
            self
        }
        pub fn checkpoint(&self) -> &super::Checkpoint {
            self.checkpoint
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Checkpoint::default_instance() as _)
        }
        pub fn checkpoint_opt_mut(&mut self) -> Option<&mut super::Checkpoint> {
            self.checkpoint.as_mut().map(|field| field as _)
        }
        pub fn checkpoint_mut(&mut self) -> &mut super::Checkpoint {
            self.checkpoint.get_or_insert_default()
        }
        pub fn checkpoint_opt(&self) -> Option<&super::Checkpoint> {
            self.checkpoint.as_ref().map(|field| field as _)
        }
        pub fn set_checkpoint<T: Into<super::Checkpoint>>(&mut self, field: T) {
            self.checkpoint = Some(field.into().into());
        }
        pub fn with_checkpoint<T: Into<super::Checkpoint>>(mut self, field: T) -> Self {
            self.set_checkpoint(field);
            self
        }
    }
    impl super::SystemPackage {
        pub const fn const_default() -> Self {
            Self {
                version: None,
                modules: Vec::new(),
                dependencies: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SystemPackage = super::SystemPackage::const_default();
            &DEFAULT
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<u64>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn modules(&self) -> &[::prost::bytes::Bytes] {
            &self.modules
        }
        pub fn set_modules(&mut self, field: Vec<::prost::bytes::Bytes>) {
            self.modules = field;
        }
        pub fn with_modules(mut self, field: Vec<::prost::bytes::Bytes>) -> Self {
            self.set_modules(field);
            self
        }
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        pub fn set_dependencies(&mut self, field: Vec<String>) {
            self.dependencies = field;
        }
        pub fn with_dependencies(mut self, field: Vec<String>) -> Self {
            self.set_dependencies(field);
            self
        }
    }
    impl super::SystemParameters {
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
            static DEFAULT: super::SystemParameters = super::SystemParameters::const_default();
            &DEFAULT
        }
        pub fn epoch_duration_ms_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch_duration_ms.as_mut().map(|field| field as _)
        }
        pub fn epoch_duration_ms_mut(&mut self) -> &mut u64 {
            self.epoch_duration_ms.get_or_insert_default()
        }
        pub fn epoch_duration_ms_opt(&self) -> Option<u64> {
            self.epoch_duration_ms.as_ref().map(|field| *field)
        }
        pub fn set_epoch_duration_ms<T: Into<u64>>(&mut self, field: T) {
            self.epoch_duration_ms = Some(field.into().into());
        }
        pub fn with_epoch_duration_ms<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch_duration_ms(field);
            self
        }
        pub fn stake_subsidy_start_epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.stake_subsidy_start_epoch.as_mut().map(|field| field as _)
        }
        pub fn stake_subsidy_start_epoch_mut(&mut self) -> &mut u64 {
            self.stake_subsidy_start_epoch.get_or_insert_default()
        }
        pub fn stake_subsidy_start_epoch_opt(&self) -> Option<u64> {
            self.stake_subsidy_start_epoch.as_ref().map(|field| *field)
        }
        pub fn set_stake_subsidy_start_epoch<T: Into<u64>>(&mut self, field: T) {
            self.stake_subsidy_start_epoch = Some(field.into().into());
        }
        pub fn with_stake_subsidy_start_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_stake_subsidy_start_epoch(field);
            self
        }
        pub fn min_validator_count_opt_mut(&mut self) -> Option<&mut u64> {
            self.min_validator_count.as_mut().map(|field| field as _)
        }
        pub fn min_validator_count_mut(&mut self) -> &mut u64 {
            self.min_validator_count.get_or_insert_default()
        }
        pub fn min_validator_count_opt(&self) -> Option<u64> {
            self.min_validator_count.as_ref().map(|field| *field)
        }
        pub fn set_min_validator_count<T: Into<u64>>(&mut self, field: T) {
            self.min_validator_count = Some(field.into().into());
        }
        pub fn with_min_validator_count<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_min_validator_count(field);
            self
        }
        pub fn max_validator_count_opt_mut(&mut self) -> Option<&mut u64> {
            self.max_validator_count.as_mut().map(|field| field as _)
        }
        pub fn max_validator_count_mut(&mut self) -> &mut u64 {
            self.max_validator_count.get_or_insert_default()
        }
        pub fn max_validator_count_opt(&self) -> Option<u64> {
            self.max_validator_count.as_ref().map(|field| *field)
        }
        pub fn set_max_validator_count<T: Into<u64>>(&mut self, field: T) {
            self.max_validator_count = Some(field.into().into());
        }
        pub fn with_max_validator_count<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_max_validator_count(field);
            self
        }
        pub fn min_validator_joining_stake_opt_mut(&mut self) -> Option<&mut u64> {
            self.min_validator_joining_stake.as_mut().map(|field| field as _)
        }
        pub fn min_validator_joining_stake_mut(&mut self) -> &mut u64 {
            self.min_validator_joining_stake.get_or_insert_default()
        }
        pub fn min_validator_joining_stake_opt(&self) -> Option<u64> {
            self.min_validator_joining_stake.as_ref().map(|field| *field)
        }
        pub fn set_min_validator_joining_stake<T: Into<u64>>(&mut self, field: T) {
            self.min_validator_joining_stake = Some(field.into().into());
        }
        pub fn with_min_validator_joining_stake<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_min_validator_joining_stake(field);
            self
        }
        pub fn validator_low_stake_threshold_opt_mut(&mut self) -> Option<&mut u64> {
            self.validator_low_stake_threshold.as_mut().map(|field| field as _)
        }
        pub fn validator_low_stake_threshold_mut(&mut self) -> &mut u64 {
            self.validator_low_stake_threshold.get_or_insert_default()
        }
        pub fn validator_low_stake_threshold_opt(&self) -> Option<u64> {
            self.validator_low_stake_threshold.as_ref().map(|field| *field)
        }
        pub fn set_validator_low_stake_threshold<T: Into<u64>>(&mut self, field: T) {
            self.validator_low_stake_threshold = Some(field.into().into());
        }
        pub fn with_validator_low_stake_threshold<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_validator_low_stake_threshold(field);
            self
        }
        pub fn validator_very_low_stake_threshold_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.validator_very_low_stake_threshold.as_mut().map(|field| field as _)
        }
        pub fn validator_very_low_stake_threshold_mut(&mut self) -> &mut u64 {
            self.validator_very_low_stake_threshold.get_or_insert_default()
        }
        pub fn validator_very_low_stake_threshold_opt(&self) -> Option<u64> {
            self.validator_very_low_stake_threshold.as_ref().map(|field| *field)
        }
        pub fn set_validator_very_low_stake_threshold<T: Into<u64>>(
            &mut self,
            field: T,
        ) {
            self.validator_very_low_stake_threshold = Some(field.into().into());
        }
        pub fn with_validator_very_low_stake_threshold<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_validator_very_low_stake_threshold(field);
            self
        }
        pub fn validator_low_stake_grace_period_opt_mut(&mut self) -> Option<&mut u64> {
            self.validator_low_stake_grace_period.as_mut().map(|field| field as _)
        }
        pub fn validator_low_stake_grace_period_mut(&mut self) -> &mut u64 {
            self.validator_low_stake_grace_period.get_or_insert_default()
        }
        pub fn validator_low_stake_grace_period_opt(&self) -> Option<u64> {
            self.validator_low_stake_grace_period.as_ref().map(|field| *field)
        }
        pub fn set_validator_low_stake_grace_period<T: Into<u64>>(&mut self, field: T) {
            self.validator_low_stake_grace_period = Some(field.into().into());
        }
        pub fn with_validator_low_stake_grace_period<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_validator_low_stake_grace_period(field);
            self
        }
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field);
            self
        }
    }
    impl super::SystemState {
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
            static DEFAULT: super::SystemState = super::SystemState::const_default();
            &DEFAULT
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<u64>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn protocol_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.protocol_version.as_mut().map(|field| field as _)
        }
        pub fn protocol_version_mut(&mut self) -> &mut u64 {
            self.protocol_version.get_or_insert_default()
        }
        pub fn protocol_version_opt(&self) -> Option<u64> {
            self.protocol_version.as_ref().map(|field| *field)
        }
        pub fn set_protocol_version<T: Into<u64>>(&mut self, field: T) {
            self.protocol_version = Some(field.into().into());
        }
        pub fn with_protocol_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_protocol_version(field);
            self
        }
        pub fn validators(&self) -> &super::ValidatorSet {
            self.validators
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ValidatorSet::default_instance() as _)
        }
        pub fn validators_opt_mut(&mut self) -> Option<&mut super::ValidatorSet> {
            self.validators.as_mut().map(|field| field as _)
        }
        pub fn validators_mut(&mut self) -> &mut super::ValidatorSet {
            self.validators.get_or_insert_default()
        }
        pub fn validators_opt(&self) -> Option<&super::ValidatorSet> {
            self.validators.as_ref().map(|field| field as _)
        }
        pub fn set_validators<T: Into<super::ValidatorSet>>(&mut self, field: T) {
            self.validators = Some(field.into().into());
        }
        pub fn with_validators<T: Into<super::ValidatorSet>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_validators(field);
            self
        }
        pub fn storage_fund(&self) -> &super::StorageFund {
            self.storage_fund
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::StorageFund::default_instance() as _)
        }
        pub fn storage_fund_opt_mut(&mut self) -> Option<&mut super::StorageFund> {
            self.storage_fund.as_mut().map(|field| field as _)
        }
        pub fn storage_fund_mut(&mut self) -> &mut super::StorageFund {
            self.storage_fund.get_or_insert_default()
        }
        pub fn storage_fund_opt(&self) -> Option<&super::StorageFund> {
            self.storage_fund.as_ref().map(|field| field as _)
        }
        pub fn set_storage_fund<T: Into<super::StorageFund>>(&mut self, field: T) {
            self.storage_fund = Some(field.into().into());
        }
        pub fn with_storage_fund<T: Into<super::StorageFund>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_storage_fund(field);
            self
        }
        pub fn parameters(&self) -> &super::SystemParameters {
            self.parameters
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::SystemParameters::default_instance() as _)
        }
        pub fn parameters_opt_mut(&mut self) -> Option<&mut super::SystemParameters> {
            self.parameters.as_mut().map(|field| field as _)
        }
        pub fn parameters_mut(&mut self) -> &mut super::SystemParameters {
            self.parameters.get_or_insert_default()
        }
        pub fn parameters_opt(&self) -> Option<&super::SystemParameters> {
            self.parameters.as_ref().map(|field| field as _)
        }
        pub fn set_parameters<T: Into<super::SystemParameters>>(&mut self, field: T) {
            self.parameters = Some(field.into().into());
        }
        pub fn with_parameters<T: Into<super::SystemParameters>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_parameters(field);
            self
        }
        pub fn reference_gas_price_opt_mut(&mut self) -> Option<&mut u64> {
            self.reference_gas_price.as_mut().map(|field| field as _)
        }
        pub fn reference_gas_price_mut(&mut self) -> &mut u64 {
            self.reference_gas_price.get_or_insert_default()
        }
        pub fn reference_gas_price_opt(&self) -> Option<u64> {
            self.reference_gas_price.as_ref().map(|field| *field)
        }
        pub fn set_reference_gas_price<T: Into<u64>>(&mut self, field: T) {
            self.reference_gas_price = Some(field.into().into());
        }
        pub fn with_reference_gas_price<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_reference_gas_price(field);
            self
        }
        pub fn validator_report_records(&self) -> &[super::ValidatorReportRecord] {
            &self.validator_report_records
        }
        pub fn set_validator_report_records(
            &mut self,
            field: Vec<super::ValidatorReportRecord>,
        ) {
            self.validator_report_records = field;
        }
        pub fn with_validator_report_records(
            mut self,
            field: Vec<super::ValidatorReportRecord>,
        ) -> Self {
            self.set_validator_report_records(field);
            self
        }
        pub fn stake_subsidy(&self) -> &super::StakeSubsidy {
            self.stake_subsidy
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::StakeSubsidy::default_instance() as _)
        }
        pub fn stake_subsidy_opt_mut(&mut self) -> Option<&mut super::StakeSubsidy> {
            self.stake_subsidy.as_mut().map(|field| field as _)
        }
        pub fn stake_subsidy_mut(&mut self) -> &mut super::StakeSubsidy {
            self.stake_subsidy.get_or_insert_default()
        }
        pub fn stake_subsidy_opt(&self) -> Option<&super::StakeSubsidy> {
            self.stake_subsidy.as_ref().map(|field| field as _)
        }
        pub fn set_stake_subsidy<T: Into<super::StakeSubsidy>>(&mut self, field: T) {
            self.stake_subsidy = Some(field.into().into());
        }
        pub fn with_stake_subsidy<T: Into<super::StakeSubsidy>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_stake_subsidy(field);
            self
        }
        pub fn safe_mode_opt_mut(&mut self) -> Option<&mut bool> {
            self.safe_mode.as_mut().map(|field| field as _)
        }
        pub fn safe_mode_mut(&mut self) -> &mut bool {
            self.safe_mode.get_or_insert_default()
        }
        pub fn safe_mode_opt(&self) -> Option<bool> {
            self.safe_mode.as_ref().map(|field| *field)
        }
        pub fn set_safe_mode<T: Into<bool>>(&mut self, field: T) {
            self.safe_mode = Some(field.into().into());
        }
        pub fn with_safe_mode<T: Into<bool>>(mut self, field: T) -> Self {
            self.set_safe_mode(field);
            self
        }
        pub fn safe_mode_storage_rewards_opt_mut(&mut self) -> Option<&mut u64> {
            self.safe_mode_storage_rewards.as_mut().map(|field| field as _)
        }
        pub fn safe_mode_storage_rewards_mut(&mut self) -> &mut u64 {
            self.safe_mode_storage_rewards.get_or_insert_default()
        }
        pub fn safe_mode_storage_rewards_opt(&self) -> Option<u64> {
            self.safe_mode_storage_rewards.as_ref().map(|field| *field)
        }
        pub fn set_safe_mode_storage_rewards<T: Into<u64>>(&mut self, field: T) {
            self.safe_mode_storage_rewards = Some(field.into().into());
        }
        pub fn with_safe_mode_storage_rewards<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_safe_mode_storage_rewards(field);
            self
        }
        pub fn safe_mode_computation_rewards_opt_mut(&mut self) -> Option<&mut u64> {
            self.safe_mode_computation_rewards.as_mut().map(|field| field as _)
        }
        pub fn safe_mode_computation_rewards_mut(&mut self) -> &mut u64 {
            self.safe_mode_computation_rewards.get_or_insert_default()
        }
        pub fn safe_mode_computation_rewards_opt(&self) -> Option<u64> {
            self.safe_mode_computation_rewards.as_ref().map(|field| *field)
        }
        pub fn set_safe_mode_computation_rewards<T: Into<u64>>(&mut self, field: T) {
            self.safe_mode_computation_rewards = Some(field.into().into());
        }
        pub fn with_safe_mode_computation_rewards<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_safe_mode_computation_rewards(field);
            self
        }
        pub fn safe_mode_storage_rebates_opt_mut(&mut self) -> Option<&mut u64> {
            self.safe_mode_storage_rebates.as_mut().map(|field| field as _)
        }
        pub fn safe_mode_storage_rebates_mut(&mut self) -> &mut u64 {
            self.safe_mode_storage_rebates.get_or_insert_default()
        }
        pub fn safe_mode_storage_rebates_opt(&self) -> Option<u64> {
            self.safe_mode_storage_rebates.as_ref().map(|field| *field)
        }
        pub fn set_safe_mode_storage_rebates<T: Into<u64>>(&mut self, field: T) {
            self.safe_mode_storage_rebates = Some(field.into().into());
        }
        pub fn with_safe_mode_storage_rebates<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_safe_mode_storage_rebates(field);
            self
        }
        pub fn safe_mode_non_refundable_storage_fee_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.safe_mode_non_refundable_storage_fee.as_mut().map(|field| field as _)
        }
        pub fn safe_mode_non_refundable_storage_fee_mut(&mut self) -> &mut u64 {
            self.safe_mode_non_refundable_storage_fee.get_or_insert_default()
        }
        pub fn safe_mode_non_refundable_storage_fee_opt(&self) -> Option<u64> {
            self.safe_mode_non_refundable_storage_fee.as_ref().map(|field| *field)
        }
        pub fn set_safe_mode_non_refundable_storage_fee<T: Into<u64>>(
            &mut self,
            field: T,
        ) {
            self.safe_mode_non_refundable_storage_fee = Some(field.into().into());
        }
        pub fn with_safe_mode_non_refundable_storage_fee<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_safe_mode_non_refundable_storage_fee(field);
            self
        }
        pub fn epoch_start_timestamp_ms_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch_start_timestamp_ms.as_mut().map(|field| field as _)
        }
        pub fn epoch_start_timestamp_ms_mut(&mut self) -> &mut u64 {
            self.epoch_start_timestamp_ms.get_or_insert_default()
        }
        pub fn epoch_start_timestamp_ms_opt(&self) -> Option<u64> {
            self.epoch_start_timestamp_ms.as_ref().map(|field| *field)
        }
        pub fn set_epoch_start_timestamp_ms<T: Into<u64>>(&mut self, field: T) {
            self.epoch_start_timestamp_ms = Some(field.into().into());
        }
        pub fn with_epoch_start_timestamp_ms<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch_start_timestamp_ms(field);
            self
        }
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field);
            self
        }
    }
    impl super::Transaction {
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
            static DEFAULT: super::Transaction = super::Transaction::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field);
            self
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut i32> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut i32 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<i32> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<i32>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<i32>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn kind(&self) -> &super::TransactionKind {
            self.kind
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::TransactionKind::default_instance() as _)
        }
        pub fn kind_opt_mut(&mut self) -> Option<&mut super::TransactionKind> {
            self.kind.as_mut().map(|field| field as _)
        }
        pub fn kind_mut(&mut self) -> &mut super::TransactionKind {
            self.kind.get_or_insert_default()
        }
        pub fn kind_opt(&self) -> Option<&super::TransactionKind> {
            self.kind.as_ref().map(|field| field as _)
        }
        pub fn set_kind<T: Into<super::TransactionKind>>(&mut self, field: T) {
            self.kind = Some(field.into().into());
        }
        pub fn with_kind<T: Into<super::TransactionKind>>(mut self, field: T) -> Self {
            self.set_kind(field);
            self
        }
        pub fn sender_opt_mut(&mut self) -> Option<&mut String> {
            self.sender.as_mut().map(|field| field as _)
        }
        pub fn sender_mut(&mut self) -> &mut String {
            self.sender.get_or_insert_default()
        }
        pub fn sender_opt(&self) -> Option<&str> {
            self.sender.as_ref().map(|field| field as _)
        }
        pub fn set_sender<T: Into<String>>(&mut self, field: T) {
            self.sender = Some(field.into().into());
        }
        pub fn with_sender<T: Into<String>>(mut self, field: T) -> Self {
            self.set_sender(field);
            self
        }
        pub fn gas_payment(&self) -> &super::GasPayment {
            self.gas_payment
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::GasPayment::default_instance() as _)
        }
        pub fn gas_payment_opt_mut(&mut self) -> Option<&mut super::GasPayment> {
            self.gas_payment.as_mut().map(|field| field as _)
        }
        pub fn gas_payment_mut(&mut self) -> &mut super::GasPayment {
            self.gas_payment.get_or_insert_default()
        }
        pub fn gas_payment_opt(&self) -> Option<&super::GasPayment> {
            self.gas_payment.as_ref().map(|field| field as _)
        }
        pub fn set_gas_payment<T: Into<super::GasPayment>>(&mut self, field: T) {
            self.gas_payment = Some(field.into().into());
        }
        pub fn with_gas_payment<T: Into<super::GasPayment>>(mut self, field: T) -> Self {
            self.set_gas_payment(field);
            self
        }
        pub fn expiration(&self) -> &super::TransactionExpiration {
            self.expiration
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::TransactionExpiration::default_instance() as _)
        }
        pub fn expiration_opt_mut(
            &mut self,
        ) -> Option<&mut super::TransactionExpiration> {
            self.expiration.as_mut().map(|field| field as _)
        }
        pub fn expiration_mut(&mut self) -> &mut super::TransactionExpiration {
            self.expiration.get_or_insert_default()
        }
        pub fn expiration_opt(&self) -> Option<&super::TransactionExpiration> {
            self.expiration.as_ref().map(|field| field as _)
        }
        pub fn set_expiration<T: Into<super::TransactionExpiration>>(
            &mut self,
            field: T,
        ) {
            self.expiration = Some(field.into().into());
        }
        pub fn with_expiration<T: Into<super::TransactionExpiration>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_expiration(field);
            self
        }
    }
    impl super::TransactionEffects {
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
            static DEFAULT: super::TransactionEffects = super::TransactionEffects::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field);
            self
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut i32> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut i32 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<i32> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<i32>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<i32>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn status(&self) -> &super::ExecutionStatus {
            self.status
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ExecutionStatus::default_instance() as _)
        }
        pub fn status_opt_mut(&mut self) -> Option<&mut super::ExecutionStatus> {
            self.status.as_mut().map(|field| field as _)
        }
        pub fn status_mut(&mut self) -> &mut super::ExecutionStatus {
            self.status.get_or_insert_default()
        }
        pub fn status_opt(&self) -> Option<&super::ExecutionStatus> {
            self.status.as_ref().map(|field| field as _)
        }
        pub fn set_status<T: Into<super::ExecutionStatus>>(&mut self, field: T) {
            self.status = Some(field.into().into());
        }
        pub fn with_status<T: Into<super::ExecutionStatus>>(mut self, field: T) -> Self {
            self.set_status(field);
            self
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn gas_used(&self) -> &super::GasCostSummary {
            self.gas_used
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::GasCostSummary::default_instance() as _)
        }
        pub fn gas_used_opt_mut(&mut self) -> Option<&mut super::GasCostSummary> {
            self.gas_used.as_mut().map(|field| field as _)
        }
        pub fn gas_used_mut(&mut self) -> &mut super::GasCostSummary {
            self.gas_used.get_or_insert_default()
        }
        pub fn gas_used_opt(&self) -> Option<&super::GasCostSummary> {
            self.gas_used.as_ref().map(|field| field as _)
        }
        pub fn set_gas_used<T: Into<super::GasCostSummary>>(&mut self, field: T) {
            self.gas_used = Some(field.into().into());
        }
        pub fn with_gas_used<T: Into<super::GasCostSummary>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_gas_used(field);
            self
        }
        pub fn transaction_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.transaction_digest.as_mut().map(|field| field as _)
        }
        pub fn transaction_digest_mut(&mut self) -> &mut String {
            self.transaction_digest.get_or_insert_default()
        }
        pub fn transaction_digest_opt(&self) -> Option<&str> {
            self.transaction_digest.as_ref().map(|field| field as _)
        }
        pub fn set_transaction_digest<T: Into<String>>(&mut self, field: T) {
            self.transaction_digest = Some(field.into().into());
        }
        pub fn with_transaction_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_transaction_digest(field);
            self
        }
        pub fn gas_object(&self) -> &super::ChangedObject {
            self.gas_object
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ChangedObject::default_instance() as _)
        }
        pub fn gas_object_opt_mut(&mut self) -> Option<&mut super::ChangedObject> {
            self.gas_object.as_mut().map(|field| field as _)
        }
        pub fn gas_object_mut(&mut self) -> &mut super::ChangedObject {
            self.gas_object.get_or_insert_default()
        }
        pub fn gas_object_opt(&self) -> Option<&super::ChangedObject> {
            self.gas_object.as_ref().map(|field| field as _)
        }
        pub fn set_gas_object<T: Into<super::ChangedObject>>(&mut self, field: T) {
            self.gas_object = Some(field.into().into());
        }
        pub fn with_gas_object<T: Into<super::ChangedObject>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_gas_object(field);
            self
        }
        pub fn events_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.events_digest.as_mut().map(|field| field as _)
        }
        pub fn events_digest_mut(&mut self) -> &mut String {
            self.events_digest.get_or_insert_default()
        }
        pub fn events_digest_opt(&self) -> Option<&str> {
            self.events_digest.as_ref().map(|field| field as _)
        }
        pub fn set_events_digest<T: Into<String>>(&mut self, field: T) {
            self.events_digest = Some(field.into().into());
        }
        pub fn with_events_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_events_digest(field);
            self
        }
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        pub fn set_dependencies(&mut self, field: Vec<String>) {
            self.dependencies = field;
        }
        pub fn with_dependencies(mut self, field: Vec<String>) -> Self {
            self.set_dependencies(field);
            self
        }
        pub fn lamport_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.lamport_version.as_mut().map(|field| field as _)
        }
        pub fn lamport_version_mut(&mut self) -> &mut u64 {
            self.lamport_version.get_or_insert_default()
        }
        pub fn lamport_version_opt(&self) -> Option<u64> {
            self.lamport_version.as_ref().map(|field| *field)
        }
        pub fn set_lamport_version<T: Into<u64>>(&mut self, field: T) {
            self.lamport_version = Some(field.into().into());
        }
        pub fn with_lamport_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_lamport_version(field);
            self
        }
        pub fn changed_objects(&self) -> &[super::ChangedObject] {
            &self.changed_objects
        }
        pub fn set_changed_objects(&mut self, field: Vec<super::ChangedObject>) {
            self.changed_objects = field;
        }
        pub fn with_changed_objects(mut self, field: Vec<super::ChangedObject>) -> Self {
            self.set_changed_objects(field);
            self
        }
        pub fn unchanged_consensus_objects(&self) -> &[super::UnchangedConsensusObject] {
            &self.unchanged_consensus_objects
        }
        pub fn set_unchanged_consensus_objects(
            &mut self,
            field: Vec<super::UnchangedConsensusObject>,
        ) {
            self.unchanged_consensus_objects = field;
        }
        pub fn with_unchanged_consensus_objects(
            mut self,
            field: Vec<super::UnchangedConsensusObject>,
        ) -> Self {
            self.set_unchanged_consensus_objects(field);
            self
        }
        pub fn auxiliary_data_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.auxiliary_data_digest.as_mut().map(|field| field as _)
        }
        pub fn auxiliary_data_digest_mut(&mut self) -> &mut String {
            self.auxiliary_data_digest.get_or_insert_default()
        }
        pub fn auxiliary_data_digest_opt(&self) -> Option<&str> {
            self.auxiliary_data_digest.as_ref().map(|field| field as _)
        }
        pub fn set_auxiliary_data_digest<T: Into<String>>(&mut self, field: T) {
            self.auxiliary_data_digest = Some(field.into().into());
        }
        pub fn with_auxiliary_data_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_auxiliary_data_digest(field);
            self
        }
    }
    impl super::TransactionEvents {
        pub const fn const_default() -> Self {
            Self {
                bcs: None,
                digest: None,
                events: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionEvents = super::TransactionEvents::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field);
            self
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn events(&self) -> &[super::Event] {
            &self.events
        }
        pub fn set_events(&mut self, field: Vec<super::Event>) {
            self.events = field;
        }
        pub fn with_events(mut self, field: Vec<super::Event>) -> Self {
            self.set_events(field);
            self
        }
    }
    impl super::TransactionExpiration {
        pub const fn const_default() -> Self {
            Self { kind: None, epoch: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionExpiration = super::TransactionExpiration::const_default();
            &DEFAULT
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
    }
    impl super::TransactionFinality {
        pub const fn const_default() -> Self {
            Self { finality: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionFinality = super::TransactionFinality::const_default();
            &DEFAULT
        }
        pub fn certified(&self) -> &super::ValidatorAggregatedSignature {
            if let Some(super::transaction_finality::Finality::Certified(field)) = &self
                .finality
            {
                field as _
            } else {
                super::ValidatorAggregatedSignature::default_instance() as _
            }
        }
        pub fn certified_opt(&self) -> Option<&super::ValidatorAggregatedSignature> {
            if let Some(super::transaction_finality::Finality::Certified(field)) = &self
                .finality
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn certified_opt_mut(
            &mut self,
        ) -> Option<&mut super::ValidatorAggregatedSignature> {
            if let Some(super::transaction_finality::Finality::Certified(field)) = &mut self
                .finality
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn certified_mut(&mut self) -> &mut super::ValidatorAggregatedSignature {
            if self.certified_opt_mut().is_none() {
                self.finality = Some(
                    super::transaction_finality::Finality::Certified(
                        super::ValidatorAggregatedSignature::default(),
                    ),
                );
            }
            self.certified_opt_mut().unwrap()
        }
        pub fn set_certified<T: Into<super::ValidatorAggregatedSignature>>(
            &mut self,
            field: T,
        ) {
            self.finality = Some(
                super::transaction_finality::Finality::Certified(field.into().into()),
            );
        }
        pub fn with_certified<T: Into<super::ValidatorAggregatedSignature>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_certified(field);
            self
        }
        pub fn checkpointed(&self) -> u64 {
            if let Some(super::transaction_finality::Finality::Checkpointed(field)) = &self
                .finality
            {
                *field
            } else {
                0u64
            }
        }
        pub fn checkpointed_opt(&self) -> Option<u64> {
            if let Some(super::transaction_finality::Finality::Checkpointed(field)) = &self
                .finality
            {
                Some(*field)
            } else {
                None
            }
        }
        pub fn checkpointed_opt_mut(&mut self) -> Option<&mut u64> {
            if let Some(super::transaction_finality::Finality::Checkpointed(field)) = &mut self
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
                    super::transaction_finality::Finality::Checkpointed(u64::default()),
                );
            }
            self.checkpointed_opt_mut().unwrap()
        }
        pub fn set_checkpointed<T: Into<u64>>(&mut self, field: T) {
            self.finality = Some(
                super::transaction_finality::Finality::Checkpointed(field.into().into()),
            );
        }
        pub fn with_checkpointed<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_checkpointed(field);
            self
        }
    }
    impl super::TransactionKind {
        pub const fn const_default() -> Self {
            Self { kind: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionKind = super::TransactionKind::const_default();
            &DEFAULT
        }
        pub fn programmable_transaction(&self) -> &super::ProgrammableTransaction {
            if let Some(super::transaction_kind::Kind::ProgrammableTransaction(field)) = &self
                .kind
            {
                field as _
            } else {
                super::ProgrammableTransaction::default_instance() as _
            }
        }
        pub fn programmable_transaction_opt(
            &self,
        ) -> Option<&super::ProgrammableTransaction> {
            if let Some(super::transaction_kind::Kind::ProgrammableTransaction(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn programmable_transaction_opt_mut(
            &mut self,
        ) -> Option<&mut super::ProgrammableTransaction> {
            if let Some(super::transaction_kind::Kind::ProgrammableTransaction(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn programmable_transaction_mut(
            &mut self,
        ) -> &mut super::ProgrammableTransaction {
            if self.programmable_transaction_opt_mut().is_none() {
                self.kind = Some(
                    super::transaction_kind::Kind::ProgrammableTransaction(
                        super::ProgrammableTransaction::default(),
                    ),
                );
            }
            self.programmable_transaction_opt_mut().unwrap()
        }
        pub fn set_programmable_transaction<T: Into<super::ProgrammableTransaction>>(
            &mut self,
            field: T,
        ) {
            self.kind = Some(
                super::transaction_kind::Kind::ProgrammableTransaction(
                    field.into().into(),
                ),
            );
        }
        pub fn with_programmable_transaction<T: Into<super::ProgrammableTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_programmable_transaction(field);
            self
        }
        pub fn programmable_system_transaction(
            &self,
        ) -> &super::ProgrammableTransaction {
            if let Some(
                super::transaction_kind::Kind::ProgrammableSystemTransaction(field),
            ) = &self.kind
            {
                field as _
            } else {
                super::ProgrammableTransaction::default_instance() as _
            }
        }
        pub fn programmable_system_transaction_opt(
            &self,
        ) -> Option<&super::ProgrammableTransaction> {
            if let Some(
                super::transaction_kind::Kind::ProgrammableSystemTransaction(field),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn programmable_system_transaction_opt_mut(
            &mut self,
        ) -> Option<&mut super::ProgrammableTransaction> {
            if let Some(
                super::transaction_kind::Kind::ProgrammableSystemTransaction(field),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn programmable_system_transaction_mut(
            &mut self,
        ) -> &mut super::ProgrammableTransaction {
            if self.programmable_system_transaction_opt_mut().is_none() {
                self.kind = Some(
                    super::transaction_kind::Kind::ProgrammableSystemTransaction(
                        super::ProgrammableTransaction::default(),
                    ),
                );
            }
            self.programmable_system_transaction_opt_mut().unwrap()
        }
        pub fn set_programmable_system_transaction<
            T: Into<super::ProgrammableTransaction>,
        >(&mut self, field: T) {
            self.kind = Some(
                super::transaction_kind::Kind::ProgrammableSystemTransaction(
                    field.into().into(),
                ),
            );
        }
        pub fn with_programmable_system_transaction<
            T: Into<super::ProgrammableTransaction>,
        >(mut self, field: T) -> Self {
            self.set_programmable_system_transaction(field);
            self
        }
        pub fn change_epoch(&self) -> &super::ChangeEpoch {
            if let Some(super::transaction_kind::Kind::ChangeEpoch(field)) = &self.kind {
                field as _
            } else {
                super::ChangeEpoch::default_instance() as _
            }
        }
        pub fn change_epoch_opt(&self) -> Option<&super::ChangeEpoch> {
            if let Some(super::transaction_kind::Kind::ChangeEpoch(field)) = &self.kind {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn change_epoch_opt_mut(&mut self) -> Option<&mut super::ChangeEpoch> {
            if let Some(super::transaction_kind::Kind::ChangeEpoch(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn change_epoch_mut(&mut self) -> &mut super::ChangeEpoch {
            if self.change_epoch_opt_mut().is_none() {
                self.kind = Some(
                    super::transaction_kind::Kind::ChangeEpoch(
                        super::ChangeEpoch::default(),
                    ),
                );
            }
            self.change_epoch_opt_mut().unwrap()
        }
        pub fn set_change_epoch<T: Into<super::ChangeEpoch>>(&mut self, field: T) {
            self.kind = Some(
                super::transaction_kind::Kind::ChangeEpoch(field.into().into()),
            );
        }
        pub fn with_change_epoch<T: Into<super::ChangeEpoch>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_change_epoch(field);
            self
        }
        pub fn genesis(&self) -> &super::GenesisTransaction {
            if let Some(super::transaction_kind::Kind::Genesis(field)) = &self.kind {
                field as _
            } else {
                super::GenesisTransaction::default_instance() as _
            }
        }
        pub fn genesis_opt(&self) -> Option<&super::GenesisTransaction> {
            if let Some(super::transaction_kind::Kind::Genesis(field)) = &self.kind {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn genesis_opt_mut(&mut self) -> Option<&mut super::GenesisTransaction> {
            if let Some(super::transaction_kind::Kind::Genesis(field)) = &mut self.kind {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn genesis_mut(&mut self) -> &mut super::GenesisTransaction {
            if self.genesis_opt_mut().is_none() {
                self.kind = Some(
                    super::transaction_kind::Kind::Genesis(
                        super::GenesisTransaction::default(),
                    ),
                );
            }
            self.genesis_opt_mut().unwrap()
        }
        pub fn set_genesis<T: Into<super::GenesisTransaction>>(&mut self, field: T) {
            self.kind = Some(
                super::transaction_kind::Kind::Genesis(field.into().into()),
            );
        }
        pub fn with_genesis<T: Into<super::GenesisTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_genesis(field);
            self
        }
        pub fn consensus_commit_prologue_v1(&self) -> &super::ConsensusCommitPrologue {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV1(field),
            ) = &self.kind
            {
                field as _
            } else {
                super::ConsensusCommitPrologue::default_instance() as _
            }
        }
        pub fn consensus_commit_prologue_v1_opt(
            &self,
        ) -> Option<&super::ConsensusCommitPrologue> {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV1(field),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v1_opt_mut(
            &mut self,
        ) -> Option<&mut super::ConsensusCommitPrologue> {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV1(field),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v1_mut(
            &mut self,
        ) -> &mut super::ConsensusCommitPrologue {
            if self.consensus_commit_prologue_v1_opt_mut().is_none() {
                self.kind = Some(
                    super::transaction_kind::Kind::ConsensusCommitPrologueV1(
                        super::ConsensusCommitPrologue::default(),
                    ),
                );
            }
            self.consensus_commit_prologue_v1_opt_mut().unwrap()
        }
        pub fn set_consensus_commit_prologue_v1<T: Into<super::ConsensusCommitPrologue>>(
            &mut self,
            field: T,
        ) {
            self.kind = Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV1(
                    field.into().into(),
                ),
            );
        }
        pub fn with_consensus_commit_prologue_v1<
            T: Into<super::ConsensusCommitPrologue>,
        >(mut self, field: T) -> Self {
            self.set_consensus_commit_prologue_v1(field);
            self
        }
        pub fn authenticator_state_update(&self) -> &super::AuthenticatorStateUpdate {
            if let Some(
                super::transaction_kind::Kind::AuthenticatorStateUpdate(field),
            ) = &self.kind
            {
                field as _
            } else {
                super::AuthenticatorStateUpdate::default_instance() as _
            }
        }
        pub fn authenticator_state_update_opt(
            &self,
        ) -> Option<&super::AuthenticatorStateUpdate> {
            if let Some(
                super::transaction_kind::Kind::AuthenticatorStateUpdate(field),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn authenticator_state_update_opt_mut(
            &mut self,
        ) -> Option<&mut super::AuthenticatorStateUpdate> {
            if let Some(
                super::transaction_kind::Kind::AuthenticatorStateUpdate(field),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn authenticator_state_update_mut(
            &mut self,
        ) -> &mut super::AuthenticatorStateUpdate {
            if self.authenticator_state_update_opt_mut().is_none() {
                self.kind = Some(
                    super::transaction_kind::Kind::AuthenticatorStateUpdate(
                        super::AuthenticatorStateUpdate::default(),
                    ),
                );
            }
            self.authenticator_state_update_opt_mut().unwrap()
        }
        pub fn set_authenticator_state_update<T: Into<super::AuthenticatorStateUpdate>>(
            &mut self,
            field: T,
        ) {
            self.kind = Some(
                super::transaction_kind::Kind::AuthenticatorStateUpdate(
                    field.into().into(),
                ),
            );
        }
        pub fn with_authenticator_state_update<T: Into<super::AuthenticatorStateUpdate>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_authenticator_state_update(field);
            self
        }
        pub fn end_of_epoch(&self) -> &super::EndOfEpochTransaction {
            if let Some(super::transaction_kind::Kind::EndOfEpoch(field)) = &self.kind {
                field as _
            } else {
                super::EndOfEpochTransaction::default_instance() as _
            }
        }
        pub fn end_of_epoch_opt(&self) -> Option<&super::EndOfEpochTransaction> {
            if let Some(super::transaction_kind::Kind::EndOfEpoch(field)) = &self.kind {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn end_of_epoch_opt_mut(
            &mut self,
        ) -> Option<&mut super::EndOfEpochTransaction> {
            if let Some(super::transaction_kind::Kind::EndOfEpoch(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn end_of_epoch_mut(&mut self) -> &mut super::EndOfEpochTransaction {
            if self.end_of_epoch_opt_mut().is_none() {
                self.kind = Some(
                    super::transaction_kind::Kind::EndOfEpoch(
                        super::EndOfEpochTransaction::default(),
                    ),
                );
            }
            self.end_of_epoch_opt_mut().unwrap()
        }
        pub fn set_end_of_epoch<T: Into<super::EndOfEpochTransaction>>(
            &mut self,
            field: T,
        ) {
            self.kind = Some(
                super::transaction_kind::Kind::EndOfEpoch(field.into().into()),
            );
        }
        pub fn with_end_of_epoch<T: Into<super::EndOfEpochTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_end_of_epoch(field);
            self
        }
        pub fn randomness_state_update(&self) -> &super::RandomnessStateUpdate {
            if let Some(super::transaction_kind::Kind::RandomnessStateUpdate(field)) = &self
                .kind
            {
                field as _
            } else {
                super::RandomnessStateUpdate::default_instance() as _
            }
        }
        pub fn randomness_state_update_opt(
            &self,
        ) -> Option<&super::RandomnessStateUpdate> {
            if let Some(super::transaction_kind::Kind::RandomnessStateUpdate(field)) = &self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn randomness_state_update_opt_mut(
            &mut self,
        ) -> Option<&mut super::RandomnessStateUpdate> {
            if let Some(super::transaction_kind::Kind::RandomnessStateUpdate(field)) = &mut self
                .kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn randomness_state_update_mut(
            &mut self,
        ) -> &mut super::RandomnessStateUpdate {
            if self.randomness_state_update_opt_mut().is_none() {
                self.kind = Some(
                    super::transaction_kind::Kind::RandomnessStateUpdate(
                        super::RandomnessStateUpdate::default(),
                    ),
                );
            }
            self.randomness_state_update_opt_mut().unwrap()
        }
        pub fn set_randomness_state_update<T: Into<super::RandomnessStateUpdate>>(
            &mut self,
            field: T,
        ) {
            self.kind = Some(
                super::transaction_kind::Kind::RandomnessStateUpdate(field.into().into()),
            );
        }
        pub fn with_randomness_state_update<T: Into<super::RandomnessStateUpdate>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_randomness_state_update(field);
            self
        }
        pub fn consensus_commit_prologue_v2(&self) -> &super::ConsensusCommitPrologue {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV2(field),
            ) = &self.kind
            {
                field as _
            } else {
                super::ConsensusCommitPrologue::default_instance() as _
            }
        }
        pub fn consensus_commit_prologue_v2_opt(
            &self,
        ) -> Option<&super::ConsensusCommitPrologue> {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV2(field),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v2_opt_mut(
            &mut self,
        ) -> Option<&mut super::ConsensusCommitPrologue> {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV2(field),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v2_mut(
            &mut self,
        ) -> &mut super::ConsensusCommitPrologue {
            if self.consensus_commit_prologue_v2_opt_mut().is_none() {
                self.kind = Some(
                    super::transaction_kind::Kind::ConsensusCommitPrologueV2(
                        super::ConsensusCommitPrologue::default(),
                    ),
                );
            }
            self.consensus_commit_prologue_v2_opt_mut().unwrap()
        }
        pub fn set_consensus_commit_prologue_v2<T: Into<super::ConsensusCommitPrologue>>(
            &mut self,
            field: T,
        ) {
            self.kind = Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV2(
                    field.into().into(),
                ),
            );
        }
        pub fn with_consensus_commit_prologue_v2<
            T: Into<super::ConsensusCommitPrologue>,
        >(mut self, field: T) -> Self {
            self.set_consensus_commit_prologue_v2(field);
            self
        }
        pub fn consensus_commit_prologue_v3(&self) -> &super::ConsensusCommitPrologue {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV3(field),
            ) = &self.kind
            {
                field as _
            } else {
                super::ConsensusCommitPrologue::default_instance() as _
            }
        }
        pub fn consensus_commit_prologue_v3_opt(
            &self,
        ) -> Option<&super::ConsensusCommitPrologue> {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV3(field),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v3_opt_mut(
            &mut self,
        ) -> Option<&mut super::ConsensusCommitPrologue> {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV3(field),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v3_mut(
            &mut self,
        ) -> &mut super::ConsensusCommitPrologue {
            if self.consensus_commit_prologue_v3_opt_mut().is_none() {
                self.kind = Some(
                    super::transaction_kind::Kind::ConsensusCommitPrologueV3(
                        super::ConsensusCommitPrologue::default(),
                    ),
                );
            }
            self.consensus_commit_prologue_v3_opt_mut().unwrap()
        }
        pub fn set_consensus_commit_prologue_v3<T: Into<super::ConsensusCommitPrologue>>(
            &mut self,
            field: T,
        ) {
            self.kind = Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV3(
                    field.into().into(),
                ),
            );
        }
        pub fn with_consensus_commit_prologue_v3<
            T: Into<super::ConsensusCommitPrologue>,
        >(mut self, field: T) -> Self {
            self.set_consensus_commit_prologue_v3(field);
            self
        }
        pub fn consensus_commit_prologue_v4(&self) -> &super::ConsensusCommitPrologue {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV4(field),
            ) = &self.kind
            {
                field as _
            } else {
                super::ConsensusCommitPrologue::default_instance() as _
            }
        }
        pub fn consensus_commit_prologue_v4_opt(
            &self,
        ) -> Option<&super::ConsensusCommitPrologue> {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV4(field),
            ) = &self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v4_opt_mut(
            &mut self,
        ) -> Option<&mut super::ConsensusCommitPrologue> {
            if let Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV4(field),
            ) = &mut self.kind
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn consensus_commit_prologue_v4_mut(
            &mut self,
        ) -> &mut super::ConsensusCommitPrologue {
            if self.consensus_commit_prologue_v4_opt_mut().is_none() {
                self.kind = Some(
                    super::transaction_kind::Kind::ConsensusCommitPrologueV4(
                        super::ConsensusCommitPrologue::default(),
                    ),
                );
            }
            self.consensus_commit_prologue_v4_opt_mut().unwrap()
        }
        pub fn set_consensus_commit_prologue_v4<T: Into<super::ConsensusCommitPrologue>>(
            &mut self,
            field: T,
        ) {
            self.kind = Some(
                super::transaction_kind::Kind::ConsensusCommitPrologueV4(
                    field.into().into(),
                ),
            );
        }
        pub fn with_consensus_commit_prologue_v4<
            T: Into<super::ConsensusCommitPrologue>,
        >(mut self, field: T) -> Self {
            self.set_consensus_commit_prologue_v4(field);
            self
        }
    }
    impl super::TransferObjects {
        pub const fn const_default() -> Self {
            Self {
                objects: Vec::new(),
                address: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransferObjects = super::TransferObjects::const_default();
            &DEFAULT
        }
        pub fn objects(&self) -> &[super::Argument] {
            &self.objects
        }
        pub fn set_objects(&mut self, field: Vec<super::Argument>) {
            self.objects = field;
        }
        pub fn with_objects(mut self, field: Vec<super::Argument>) -> Self {
            self.set_objects(field);
            self
        }
        pub fn address(&self) -> &super::Argument {
            self.address
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Argument::default_instance() as _)
        }
        pub fn address_opt_mut(&mut self) -> Option<&mut super::Argument> {
            self.address.as_mut().map(|field| field as _)
        }
        pub fn address_mut(&mut self) -> &mut super::Argument {
            self.address.get_or_insert_default()
        }
        pub fn address_opt(&self) -> Option<&super::Argument> {
            self.address.as_ref().map(|field| field as _)
        }
        pub fn set_address<T: Into<super::Argument>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        pub fn with_address<T: Into<super::Argument>>(mut self, field: T) -> Self {
            self.set_address(field);
            self
        }
    }
    impl super::TypeArgumentError {
        pub const fn const_default() -> Self {
            Self {
                type_argument: None,
                kind: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TypeArgumentError = super::TypeArgumentError::const_default();
            &DEFAULT
        }
        pub fn type_argument_opt_mut(&mut self) -> Option<&mut u32> {
            self.type_argument.as_mut().map(|field| field as _)
        }
        pub fn type_argument_mut(&mut self) -> &mut u32 {
            self.type_argument.get_or_insert_default()
        }
        pub fn type_argument_opt(&self) -> Option<u32> {
            self.type_argument.as_ref().map(|field| *field)
        }
        pub fn set_type_argument<T: Into<u32>>(&mut self, field: T) {
            self.type_argument = Some(field.into().into());
        }
        pub fn with_type_argument<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_type_argument(field);
            self
        }
    }
    impl super::TypeOrigin {
        pub const fn const_default() -> Self {
            Self {
                module_name: None,
                datatype_name: None,
                package_id: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TypeOrigin = super::TypeOrigin::const_default();
            &DEFAULT
        }
        pub fn module_name_opt_mut(&mut self) -> Option<&mut String> {
            self.module_name.as_mut().map(|field| field as _)
        }
        pub fn module_name_mut(&mut self) -> &mut String {
            self.module_name.get_or_insert_default()
        }
        pub fn module_name_opt(&self) -> Option<&str> {
            self.module_name.as_ref().map(|field| field as _)
        }
        pub fn set_module_name<T: Into<String>>(&mut self, field: T) {
            self.module_name = Some(field.into().into());
        }
        pub fn with_module_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module_name(field);
            self
        }
        pub fn datatype_name_opt_mut(&mut self) -> Option<&mut String> {
            self.datatype_name.as_mut().map(|field| field as _)
        }
        pub fn datatype_name_mut(&mut self) -> &mut String {
            self.datatype_name.get_or_insert_default()
        }
        pub fn datatype_name_opt(&self) -> Option<&str> {
            self.datatype_name.as_ref().map(|field| field as _)
        }
        pub fn set_datatype_name<T: Into<String>>(&mut self, field: T) {
            self.datatype_name = Some(field.into().into());
        }
        pub fn with_datatype_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_datatype_name(field);
            self
        }
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field);
            self
        }
    }
    impl super::TypeParameter {
        pub const fn const_default() -> Self {
            Self {
                constraints: Vec::new(),
                is_phantom: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TypeParameter = super::TypeParameter::const_default();
            &DEFAULT
        }
        pub fn is_phantom_opt_mut(&mut self) -> Option<&mut bool> {
            self.is_phantom.as_mut().map(|field| field as _)
        }
        pub fn is_phantom_mut(&mut self) -> &mut bool {
            self.is_phantom.get_or_insert_default()
        }
        pub fn is_phantom_opt(&self) -> Option<bool> {
            self.is_phantom.as_ref().map(|field| *field)
        }
        pub fn set_is_phantom<T: Into<bool>>(&mut self, field: T) {
            self.is_phantom = Some(field.into().into());
        }
        pub fn with_is_phantom<T: Into<bool>>(mut self, field: T) -> Self {
            self.set_is_phantom(field);
            self
        }
    }
    impl super::UnchangedConsensusObject {
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
            static DEFAULT: super::UnchangedConsensusObject = super::UnchangedConsensusObject::const_default();
            &DEFAULT
        }
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<u64>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field);
            self
        }
        pub fn object_type_opt_mut(&mut self) -> Option<&mut String> {
            self.object_type.as_mut().map(|field| field as _)
        }
        pub fn object_type_mut(&mut self) -> &mut String {
            self.object_type.get_or_insert_default()
        }
        pub fn object_type_opt(&self) -> Option<&str> {
            self.object_type.as_ref().map(|field| field as _)
        }
        pub fn set_object_type<T: Into<String>>(&mut self, field: T) {
            self.object_type = Some(field.into().into());
        }
        pub fn with_object_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_type(field);
            self
        }
    }
    impl super::Upgrade {
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
            static DEFAULT: super::Upgrade = super::Upgrade::const_default();
            &DEFAULT
        }
        pub fn modules(&self) -> &[::prost::bytes::Bytes] {
            &self.modules
        }
        pub fn set_modules(&mut self, field: Vec<::prost::bytes::Bytes>) {
            self.modules = field;
        }
        pub fn with_modules(mut self, field: Vec<::prost::bytes::Bytes>) -> Self {
            self.set_modules(field);
            self
        }
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        pub fn set_dependencies(&mut self, field: Vec<String>) {
            self.dependencies = field;
        }
        pub fn with_dependencies(mut self, field: Vec<String>) -> Self {
            self.set_dependencies(field);
            self
        }
        pub fn package_opt_mut(&mut self) -> Option<&mut String> {
            self.package.as_mut().map(|field| field as _)
        }
        pub fn package_mut(&mut self) -> &mut String {
            self.package.get_or_insert_default()
        }
        pub fn package_opt(&self) -> Option<&str> {
            self.package.as_ref().map(|field| field as _)
        }
        pub fn set_package<T: Into<String>>(&mut self, field: T) {
            self.package = Some(field.into().into());
        }
        pub fn with_package<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package(field);
            self
        }
        pub fn ticket(&self) -> &super::Argument {
            self.ticket
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Argument::default_instance() as _)
        }
        pub fn ticket_opt_mut(&mut self) -> Option<&mut super::Argument> {
            self.ticket.as_mut().map(|field| field as _)
        }
        pub fn ticket_mut(&mut self) -> &mut super::Argument {
            self.ticket.get_or_insert_default()
        }
        pub fn ticket_opt(&self) -> Option<&super::Argument> {
            self.ticket.as_ref().map(|field| field as _)
        }
        pub fn set_ticket<T: Into<super::Argument>>(&mut self, field: T) {
            self.ticket = Some(field.into().into());
        }
        pub fn with_ticket<T: Into<super::Argument>>(mut self, field: T) -> Self {
            self.set_ticket(field);
            self
        }
    }
    impl super::UserSignature {
        pub const fn const_default() -> Self {
            Self {
                bcs: None,
                scheme: None,
                signature: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::UserSignature = super::UserSignature::const_default();
            &DEFAULT
        }
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field);
            self
        }
        pub fn simple(&self) -> &super::SimpleSignature {
            if let Some(super::user_signature::Signature::Simple(field)) = &self
                .signature
            {
                field as _
            } else {
                super::SimpleSignature::default_instance() as _
            }
        }
        pub fn simple_opt(&self) -> Option<&super::SimpleSignature> {
            if let Some(super::user_signature::Signature::Simple(field)) = &self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn simple_opt_mut(&mut self) -> Option<&mut super::SimpleSignature> {
            if let Some(super::user_signature::Signature::Simple(field)) = &mut self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn simple_mut(&mut self) -> &mut super::SimpleSignature {
            if self.simple_opt_mut().is_none() {
                self.signature = Some(
                    super::user_signature::Signature::Simple(
                        super::SimpleSignature::default(),
                    ),
                );
            }
            self.simple_opt_mut().unwrap()
        }
        pub fn set_simple<T: Into<super::SimpleSignature>>(&mut self, field: T) {
            self.signature = Some(
                super::user_signature::Signature::Simple(field.into().into()),
            );
        }
        pub fn with_simple<T: Into<super::SimpleSignature>>(mut self, field: T) -> Self {
            self.set_simple(field);
            self
        }
        pub fn multisig(&self) -> &super::MultisigAggregatedSignature {
            if let Some(super::user_signature::Signature::Multisig(field)) = &self
                .signature
            {
                field as _
            } else {
                super::MultisigAggregatedSignature::default_instance() as _
            }
        }
        pub fn multisig_opt(&self) -> Option<&super::MultisigAggregatedSignature> {
            if let Some(super::user_signature::Signature::Multisig(field)) = &self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn multisig_opt_mut(
            &mut self,
        ) -> Option<&mut super::MultisigAggregatedSignature> {
            if let Some(super::user_signature::Signature::Multisig(field)) = &mut self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn multisig_mut(&mut self) -> &mut super::MultisigAggregatedSignature {
            if self.multisig_opt_mut().is_none() {
                self.signature = Some(
                    super::user_signature::Signature::Multisig(
                        super::MultisigAggregatedSignature::default(),
                    ),
                );
            }
            self.multisig_opt_mut().unwrap()
        }
        pub fn set_multisig<T: Into<super::MultisigAggregatedSignature>>(
            &mut self,
            field: T,
        ) {
            self.signature = Some(
                super::user_signature::Signature::Multisig(field.into().into()),
            );
        }
        pub fn with_multisig<T: Into<super::MultisigAggregatedSignature>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_multisig(field);
            self
        }
        pub fn zklogin(&self) -> &super::ZkLoginAuthenticator {
            if let Some(super::user_signature::Signature::Zklogin(field)) = &self
                .signature
            {
                field as _
            } else {
                super::ZkLoginAuthenticator::default_instance() as _
            }
        }
        pub fn zklogin_opt(&self) -> Option<&super::ZkLoginAuthenticator> {
            if let Some(super::user_signature::Signature::Zklogin(field)) = &self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn zklogin_opt_mut(&mut self) -> Option<&mut super::ZkLoginAuthenticator> {
            if let Some(super::user_signature::Signature::Zklogin(field)) = &mut self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn zklogin_mut(&mut self) -> &mut super::ZkLoginAuthenticator {
            if self.zklogin_opt_mut().is_none() {
                self.signature = Some(
                    super::user_signature::Signature::Zklogin(
                        super::ZkLoginAuthenticator::default(),
                    ),
                );
            }
            self.zklogin_opt_mut().unwrap()
        }
        pub fn set_zklogin<T: Into<super::ZkLoginAuthenticator>>(&mut self, field: T) {
            self.signature = Some(
                super::user_signature::Signature::Zklogin(field.into().into()),
            );
        }
        pub fn with_zklogin<T: Into<super::ZkLoginAuthenticator>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_zklogin(field);
            self
        }
        pub fn passkey(&self) -> &super::PasskeyAuthenticator {
            if let Some(super::user_signature::Signature::Passkey(field)) = &self
                .signature
            {
                field as _
            } else {
                super::PasskeyAuthenticator::default_instance() as _
            }
        }
        pub fn passkey_opt(&self) -> Option<&super::PasskeyAuthenticator> {
            if let Some(super::user_signature::Signature::Passkey(field)) = &self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn passkey_opt_mut(&mut self) -> Option<&mut super::PasskeyAuthenticator> {
            if let Some(super::user_signature::Signature::Passkey(field)) = &mut self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        pub fn passkey_mut(&mut self) -> &mut super::PasskeyAuthenticator {
            if self.passkey_opt_mut().is_none() {
                self.signature = Some(
                    super::user_signature::Signature::Passkey(
                        super::PasskeyAuthenticator::default(),
                    ),
                );
            }
            self.passkey_opt_mut().unwrap()
        }
        pub fn set_passkey<T: Into<super::PasskeyAuthenticator>>(&mut self, field: T) {
            self.signature = Some(
                super::user_signature::Signature::Passkey(field.into().into()),
            );
        }
        pub fn with_passkey<T: Into<super::PasskeyAuthenticator>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_passkey(field);
            self
        }
    }
    impl super::Validator {
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
            static DEFAULT: super::Validator = super::Validator::const_default();
            &DEFAULT
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field);
            self
        }
        pub fn description_opt_mut(&mut self) -> Option<&mut String> {
            self.description.as_mut().map(|field| field as _)
        }
        pub fn description_mut(&mut self) -> &mut String {
            self.description.get_or_insert_default()
        }
        pub fn description_opt(&self) -> Option<&str> {
            self.description.as_ref().map(|field| field as _)
        }
        pub fn set_description<T: Into<String>>(&mut self, field: T) {
            self.description = Some(field.into().into());
        }
        pub fn with_description<T: Into<String>>(mut self, field: T) -> Self {
            self.set_description(field);
            self
        }
        pub fn image_url_opt_mut(&mut self) -> Option<&mut String> {
            self.image_url.as_mut().map(|field| field as _)
        }
        pub fn image_url_mut(&mut self) -> &mut String {
            self.image_url.get_or_insert_default()
        }
        pub fn image_url_opt(&self) -> Option<&str> {
            self.image_url.as_ref().map(|field| field as _)
        }
        pub fn set_image_url<T: Into<String>>(&mut self, field: T) {
            self.image_url = Some(field.into().into());
        }
        pub fn with_image_url<T: Into<String>>(mut self, field: T) -> Self {
            self.set_image_url(field);
            self
        }
        pub fn project_url_opt_mut(&mut self) -> Option<&mut String> {
            self.project_url.as_mut().map(|field| field as _)
        }
        pub fn project_url_mut(&mut self) -> &mut String {
            self.project_url.get_or_insert_default()
        }
        pub fn project_url_opt(&self) -> Option<&str> {
            self.project_url.as_ref().map(|field| field as _)
        }
        pub fn set_project_url<T: Into<String>>(&mut self, field: T) {
            self.project_url = Some(field.into().into());
        }
        pub fn with_project_url<T: Into<String>>(mut self, field: T) -> Self {
            self.set_project_url(field);
            self
        }
        pub fn protocol_public_key_opt(&self) -> Option<&[u8]> {
            self.protocol_public_key.as_ref().map(|field| field as _)
        }
        pub fn set_protocol_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.protocol_public_key = Some(field.into().into());
        }
        pub fn with_protocol_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_protocol_public_key(field);
            self
        }
        pub fn proof_of_possession_opt(&self) -> Option<&[u8]> {
            self.proof_of_possession.as_ref().map(|field| field as _)
        }
        pub fn set_proof_of_possession<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.proof_of_possession = Some(field.into().into());
        }
        pub fn with_proof_of_possession<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_proof_of_possession(field);
            self
        }
        pub fn network_public_key_opt(&self) -> Option<&[u8]> {
            self.network_public_key.as_ref().map(|field| field as _)
        }
        pub fn set_network_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.network_public_key = Some(field.into().into());
        }
        pub fn with_network_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_network_public_key(field);
            self
        }
        pub fn worker_public_key_opt(&self) -> Option<&[u8]> {
            self.worker_public_key.as_ref().map(|field| field as _)
        }
        pub fn set_worker_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.worker_public_key = Some(field.into().into());
        }
        pub fn with_worker_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_worker_public_key(field);
            self
        }
        pub fn network_address_opt_mut(&mut self) -> Option<&mut String> {
            self.network_address.as_mut().map(|field| field as _)
        }
        pub fn network_address_mut(&mut self) -> &mut String {
            self.network_address.get_or_insert_default()
        }
        pub fn network_address_opt(&self) -> Option<&str> {
            self.network_address.as_ref().map(|field| field as _)
        }
        pub fn set_network_address<T: Into<String>>(&mut self, field: T) {
            self.network_address = Some(field.into().into());
        }
        pub fn with_network_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_network_address(field);
            self
        }
        pub fn p2p_address_opt_mut(&mut self) -> Option<&mut String> {
            self.p2p_address.as_mut().map(|field| field as _)
        }
        pub fn p2p_address_mut(&mut self) -> &mut String {
            self.p2p_address.get_or_insert_default()
        }
        pub fn p2p_address_opt(&self) -> Option<&str> {
            self.p2p_address.as_ref().map(|field| field as _)
        }
        pub fn set_p2p_address<T: Into<String>>(&mut self, field: T) {
            self.p2p_address = Some(field.into().into());
        }
        pub fn with_p2p_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_p2p_address(field);
            self
        }
        pub fn primary_address_opt_mut(&mut self) -> Option<&mut String> {
            self.primary_address.as_mut().map(|field| field as _)
        }
        pub fn primary_address_mut(&mut self) -> &mut String {
            self.primary_address.get_or_insert_default()
        }
        pub fn primary_address_opt(&self) -> Option<&str> {
            self.primary_address.as_ref().map(|field| field as _)
        }
        pub fn set_primary_address<T: Into<String>>(&mut self, field: T) {
            self.primary_address = Some(field.into().into());
        }
        pub fn with_primary_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_primary_address(field);
            self
        }
        pub fn worker_address_opt_mut(&mut self) -> Option<&mut String> {
            self.worker_address.as_mut().map(|field| field as _)
        }
        pub fn worker_address_mut(&mut self) -> &mut String {
            self.worker_address.get_or_insert_default()
        }
        pub fn worker_address_opt(&self) -> Option<&str> {
            self.worker_address.as_ref().map(|field| field as _)
        }
        pub fn set_worker_address<T: Into<String>>(&mut self, field: T) {
            self.worker_address = Some(field.into().into());
        }
        pub fn with_worker_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_worker_address(field);
            self
        }
        pub fn next_epoch_protocol_public_key_opt(&self) -> Option<&[u8]> {
            self.next_epoch_protocol_public_key.as_ref().map(|field| field as _)
        }
        pub fn set_next_epoch_protocol_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.next_epoch_protocol_public_key = Some(field.into().into());
        }
        pub fn with_next_epoch_protocol_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_protocol_public_key(field);
            self
        }
        pub fn next_epoch_proof_of_possession_opt(&self) -> Option<&[u8]> {
            self.next_epoch_proof_of_possession.as_ref().map(|field| field as _)
        }
        pub fn set_next_epoch_proof_of_possession<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.next_epoch_proof_of_possession = Some(field.into().into());
        }
        pub fn with_next_epoch_proof_of_possession<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_proof_of_possession(field);
            self
        }
        pub fn next_epoch_network_public_key_opt(&self) -> Option<&[u8]> {
            self.next_epoch_network_public_key.as_ref().map(|field| field as _)
        }
        pub fn set_next_epoch_network_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.next_epoch_network_public_key = Some(field.into().into());
        }
        pub fn with_next_epoch_network_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_network_public_key(field);
            self
        }
        pub fn next_epoch_worker_public_key_opt(&self) -> Option<&[u8]> {
            self.next_epoch_worker_public_key.as_ref().map(|field| field as _)
        }
        pub fn set_next_epoch_worker_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.next_epoch_worker_public_key = Some(field.into().into());
        }
        pub fn with_next_epoch_worker_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_worker_public_key(field);
            self
        }
        pub fn next_epoch_network_address_opt_mut(&mut self) -> Option<&mut String> {
            self.next_epoch_network_address.as_mut().map(|field| field as _)
        }
        pub fn next_epoch_network_address_mut(&mut self) -> &mut String {
            self.next_epoch_network_address.get_or_insert_default()
        }
        pub fn next_epoch_network_address_opt(&self) -> Option<&str> {
            self.next_epoch_network_address.as_ref().map(|field| field as _)
        }
        pub fn set_next_epoch_network_address<T: Into<String>>(&mut self, field: T) {
            self.next_epoch_network_address = Some(field.into().into());
        }
        pub fn with_next_epoch_network_address<T: Into<String>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_network_address(field);
            self
        }
        pub fn next_epoch_p2p_address_opt_mut(&mut self) -> Option<&mut String> {
            self.next_epoch_p2p_address.as_mut().map(|field| field as _)
        }
        pub fn next_epoch_p2p_address_mut(&mut self) -> &mut String {
            self.next_epoch_p2p_address.get_or_insert_default()
        }
        pub fn next_epoch_p2p_address_opt(&self) -> Option<&str> {
            self.next_epoch_p2p_address.as_ref().map(|field| field as _)
        }
        pub fn set_next_epoch_p2p_address<T: Into<String>>(&mut self, field: T) {
            self.next_epoch_p2p_address = Some(field.into().into());
        }
        pub fn with_next_epoch_p2p_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_next_epoch_p2p_address(field);
            self
        }
        pub fn next_epoch_primary_address_opt_mut(&mut self) -> Option<&mut String> {
            self.next_epoch_primary_address.as_mut().map(|field| field as _)
        }
        pub fn next_epoch_primary_address_mut(&mut self) -> &mut String {
            self.next_epoch_primary_address.get_or_insert_default()
        }
        pub fn next_epoch_primary_address_opt(&self) -> Option<&str> {
            self.next_epoch_primary_address.as_ref().map(|field| field as _)
        }
        pub fn set_next_epoch_primary_address<T: Into<String>>(&mut self, field: T) {
            self.next_epoch_primary_address = Some(field.into().into());
        }
        pub fn with_next_epoch_primary_address<T: Into<String>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_primary_address(field);
            self
        }
        pub fn next_epoch_worker_address_opt_mut(&mut self) -> Option<&mut String> {
            self.next_epoch_worker_address.as_mut().map(|field| field as _)
        }
        pub fn next_epoch_worker_address_mut(&mut self) -> &mut String {
            self.next_epoch_worker_address.get_or_insert_default()
        }
        pub fn next_epoch_worker_address_opt(&self) -> Option<&str> {
            self.next_epoch_worker_address.as_ref().map(|field| field as _)
        }
        pub fn set_next_epoch_worker_address<T: Into<String>>(&mut self, field: T) {
            self.next_epoch_worker_address = Some(field.into().into());
        }
        pub fn with_next_epoch_worker_address<T: Into<String>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_worker_address(field);
            self
        }
        pub fn metadata_extra_fields(&self) -> &super::MoveTable {
            self.metadata_extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn metadata_extra_fields_opt_mut(
            &mut self,
        ) -> Option<&mut super::MoveTable> {
            self.metadata_extra_fields.as_mut().map(|field| field as _)
        }
        pub fn metadata_extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.metadata_extra_fields.get_or_insert_default()
        }
        pub fn metadata_extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.metadata_extra_fields.as_ref().map(|field| field as _)
        }
        pub fn set_metadata_extra_fields<T: Into<super::MoveTable>>(
            &mut self,
            field: T,
        ) {
            self.metadata_extra_fields = Some(field.into().into());
        }
        pub fn with_metadata_extra_fields<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_metadata_extra_fields(field);
            self
        }
        pub fn voting_power_opt_mut(&mut self) -> Option<&mut u64> {
            self.voting_power.as_mut().map(|field| field as _)
        }
        pub fn voting_power_mut(&mut self) -> &mut u64 {
            self.voting_power.get_or_insert_default()
        }
        pub fn voting_power_opt(&self) -> Option<u64> {
            self.voting_power.as_ref().map(|field| *field)
        }
        pub fn set_voting_power<T: Into<u64>>(&mut self, field: T) {
            self.voting_power = Some(field.into().into());
        }
        pub fn with_voting_power<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_voting_power(field);
            self
        }
        pub fn operation_cap_id_opt_mut(&mut self) -> Option<&mut String> {
            self.operation_cap_id.as_mut().map(|field| field as _)
        }
        pub fn operation_cap_id_mut(&mut self) -> &mut String {
            self.operation_cap_id.get_or_insert_default()
        }
        pub fn operation_cap_id_opt(&self) -> Option<&str> {
            self.operation_cap_id.as_ref().map(|field| field as _)
        }
        pub fn set_operation_cap_id<T: Into<String>>(&mut self, field: T) {
            self.operation_cap_id = Some(field.into().into());
        }
        pub fn with_operation_cap_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_operation_cap_id(field);
            self
        }
        pub fn gas_price_opt_mut(&mut self) -> Option<&mut u64> {
            self.gas_price.as_mut().map(|field| field as _)
        }
        pub fn gas_price_mut(&mut self) -> &mut u64 {
            self.gas_price.get_or_insert_default()
        }
        pub fn gas_price_opt(&self) -> Option<u64> {
            self.gas_price.as_ref().map(|field| *field)
        }
        pub fn set_gas_price<T: Into<u64>>(&mut self, field: T) {
            self.gas_price = Some(field.into().into());
        }
        pub fn with_gas_price<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_gas_price(field);
            self
        }
        pub fn staking_pool(&self) -> &super::StakingPool {
            self.staking_pool
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::StakingPool::default_instance() as _)
        }
        pub fn staking_pool_opt_mut(&mut self) -> Option<&mut super::StakingPool> {
            self.staking_pool.as_mut().map(|field| field as _)
        }
        pub fn staking_pool_mut(&mut self) -> &mut super::StakingPool {
            self.staking_pool.get_or_insert_default()
        }
        pub fn staking_pool_opt(&self) -> Option<&super::StakingPool> {
            self.staking_pool.as_ref().map(|field| field as _)
        }
        pub fn set_staking_pool<T: Into<super::StakingPool>>(&mut self, field: T) {
            self.staking_pool = Some(field.into().into());
        }
        pub fn with_staking_pool<T: Into<super::StakingPool>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_staking_pool(field);
            self
        }
        pub fn commission_rate_opt_mut(&mut self) -> Option<&mut u64> {
            self.commission_rate.as_mut().map(|field| field as _)
        }
        pub fn commission_rate_mut(&mut self) -> &mut u64 {
            self.commission_rate.get_or_insert_default()
        }
        pub fn commission_rate_opt(&self) -> Option<u64> {
            self.commission_rate.as_ref().map(|field| *field)
        }
        pub fn set_commission_rate<T: Into<u64>>(&mut self, field: T) {
            self.commission_rate = Some(field.into().into());
        }
        pub fn with_commission_rate<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_commission_rate(field);
            self
        }
        pub fn next_epoch_stake_opt_mut(&mut self) -> Option<&mut u64> {
            self.next_epoch_stake.as_mut().map(|field| field as _)
        }
        pub fn next_epoch_stake_mut(&mut self) -> &mut u64 {
            self.next_epoch_stake.get_or_insert_default()
        }
        pub fn next_epoch_stake_opt(&self) -> Option<u64> {
            self.next_epoch_stake.as_ref().map(|field| *field)
        }
        pub fn set_next_epoch_stake<T: Into<u64>>(&mut self, field: T) {
            self.next_epoch_stake = Some(field.into().into());
        }
        pub fn with_next_epoch_stake<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_next_epoch_stake(field);
            self
        }
        pub fn next_epoch_gas_price_opt_mut(&mut self) -> Option<&mut u64> {
            self.next_epoch_gas_price.as_mut().map(|field| field as _)
        }
        pub fn next_epoch_gas_price_mut(&mut self) -> &mut u64 {
            self.next_epoch_gas_price.get_or_insert_default()
        }
        pub fn next_epoch_gas_price_opt(&self) -> Option<u64> {
            self.next_epoch_gas_price.as_ref().map(|field| *field)
        }
        pub fn set_next_epoch_gas_price<T: Into<u64>>(&mut self, field: T) {
            self.next_epoch_gas_price = Some(field.into().into());
        }
        pub fn with_next_epoch_gas_price<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_next_epoch_gas_price(field);
            self
        }
        pub fn next_epoch_commission_rate_opt_mut(&mut self) -> Option<&mut u64> {
            self.next_epoch_commission_rate.as_mut().map(|field| field as _)
        }
        pub fn next_epoch_commission_rate_mut(&mut self) -> &mut u64 {
            self.next_epoch_commission_rate.get_or_insert_default()
        }
        pub fn next_epoch_commission_rate_opt(&self) -> Option<u64> {
            self.next_epoch_commission_rate.as_ref().map(|field| *field)
        }
        pub fn set_next_epoch_commission_rate<T: Into<u64>>(&mut self, field: T) {
            self.next_epoch_commission_rate = Some(field.into().into());
        }
        pub fn with_next_epoch_commission_rate<T: Into<u64>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_commission_rate(field);
            self
        }
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field);
            self
        }
    }
    impl super::ValidatorAggregatedSignature {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                signature: None,
                bitmap: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ValidatorAggregatedSignature = super::ValidatorAggregatedSignature::const_default();
            &DEFAULT
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn signature_opt(&self) -> Option<&[u8]> {
            self.signature.as_ref().map(|field| field as _)
        }
        pub fn set_signature<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        pub fn with_signature<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field);
            self
        }
        pub fn bitmap(&self) -> &[u32] {
            &self.bitmap
        }
        pub fn set_bitmap(&mut self, field: Vec<u32>) {
            self.bitmap = field;
        }
        pub fn with_bitmap(mut self, field: Vec<u32>) -> Self {
            self.set_bitmap(field);
            self
        }
    }
    impl super::ValidatorCommittee {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                members: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ValidatorCommittee = super::ValidatorCommittee::const_default();
            &DEFAULT
        }
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        pub fn set_epoch<T: Into<u64>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        pub fn with_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_epoch(field);
            self
        }
        pub fn members(&self) -> &[super::ValidatorCommitteeMember] {
            &self.members
        }
        pub fn set_members(&mut self, field: Vec<super::ValidatorCommitteeMember>) {
            self.members = field;
        }
        pub fn with_members(
            mut self,
            field: Vec<super::ValidatorCommitteeMember>,
        ) -> Self {
            self.set_members(field);
            self
        }
    }
    impl super::ValidatorCommitteeMember {
        pub const fn const_default() -> Self {
            Self {
                public_key: None,
                weight: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ValidatorCommitteeMember = super::ValidatorCommitteeMember::const_default();
            &DEFAULT
        }
        pub fn public_key_opt(&self) -> Option<&[u8]> {
            self.public_key.as_ref().map(|field| field as _)
        }
        pub fn set_public_key<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.public_key = Some(field.into().into());
        }
        pub fn with_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_public_key(field);
            self
        }
        pub fn weight_opt_mut(&mut self) -> Option<&mut u64> {
            self.weight.as_mut().map(|field| field as _)
        }
        pub fn weight_mut(&mut self) -> &mut u64 {
            self.weight.get_or_insert_default()
        }
        pub fn weight_opt(&self) -> Option<u64> {
            self.weight.as_ref().map(|field| *field)
        }
        pub fn set_weight<T: Into<u64>>(&mut self, field: T) {
            self.weight = Some(field.into().into());
        }
        pub fn with_weight<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_weight(field);
            self
        }
    }
    impl super::ValidatorExecutionTimeObservation {
        pub const fn const_default() -> Self {
            Self {
                validator: None,
                duration: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ValidatorExecutionTimeObservation = super::ValidatorExecutionTimeObservation::const_default();
            &DEFAULT
        }
        pub fn validator_opt(&self) -> Option<&[u8]> {
            self.validator.as_ref().map(|field| field as _)
        }
        pub fn set_validator<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.validator = Some(field.into().into());
        }
        pub fn with_validator<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_validator(field);
            self
        }
        pub fn duration_opt_mut(&mut self) -> Option<&mut ::prost_types::Duration> {
            self.duration.as_mut().map(|field| field as _)
        }
        pub fn duration_mut(&mut self) -> &mut ::prost_types::Duration {
            self.duration.get_or_insert_default()
        }
        pub fn duration_opt(&self) -> Option<&::prost_types::Duration> {
            self.duration.as_ref().map(|field| field as _)
        }
        pub fn set_duration<T: Into<::prost_types::Duration>>(&mut self, field: T) {
            self.duration = Some(field.into().into());
        }
        pub fn with_duration<T: Into<::prost_types::Duration>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_duration(field);
            self
        }
    }
    impl super::ValidatorReportRecord {
        pub const fn const_default() -> Self {
            Self {
                reported: None,
                reporters: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ValidatorReportRecord = super::ValidatorReportRecord::const_default();
            &DEFAULT
        }
        pub fn reported_opt_mut(&mut self) -> Option<&mut String> {
            self.reported.as_mut().map(|field| field as _)
        }
        pub fn reported_mut(&mut self) -> &mut String {
            self.reported.get_or_insert_default()
        }
        pub fn reported_opt(&self) -> Option<&str> {
            self.reported.as_ref().map(|field| field as _)
        }
        pub fn set_reported<T: Into<String>>(&mut self, field: T) {
            self.reported = Some(field.into().into());
        }
        pub fn with_reported<T: Into<String>>(mut self, field: T) -> Self {
            self.set_reported(field);
            self
        }
        pub fn reporters(&self) -> &[String] {
            &self.reporters
        }
        pub fn set_reporters(&mut self, field: Vec<String>) {
            self.reporters = field;
        }
        pub fn with_reporters(mut self, field: Vec<String>) -> Self {
            self.set_reporters(field);
            self
        }
    }
    impl super::ValidatorSet {
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
            static DEFAULT: super::ValidatorSet = super::ValidatorSet::const_default();
            &DEFAULT
        }
        pub fn total_stake_opt_mut(&mut self) -> Option<&mut u64> {
            self.total_stake.as_mut().map(|field| field as _)
        }
        pub fn total_stake_mut(&mut self) -> &mut u64 {
            self.total_stake.get_or_insert_default()
        }
        pub fn total_stake_opt(&self) -> Option<u64> {
            self.total_stake.as_ref().map(|field| *field)
        }
        pub fn set_total_stake<T: Into<u64>>(&mut self, field: T) {
            self.total_stake = Some(field.into().into());
        }
        pub fn with_total_stake<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_total_stake(field);
            self
        }
        pub fn active_validators(&self) -> &[super::Validator] {
            &self.active_validators
        }
        pub fn set_active_validators(&mut self, field: Vec<super::Validator>) {
            self.active_validators = field;
        }
        pub fn with_active_validators(mut self, field: Vec<super::Validator>) -> Self {
            self.set_active_validators(field);
            self
        }
        pub fn pending_active_validators(&self) -> &super::MoveTable {
            self.pending_active_validators
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn pending_active_validators_opt_mut(
            &mut self,
        ) -> Option<&mut super::MoveTable> {
            self.pending_active_validators.as_mut().map(|field| field as _)
        }
        pub fn pending_active_validators_mut(&mut self) -> &mut super::MoveTable {
            self.pending_active_validators.get_or_insert_default()
        }
        pub fn pending_active_validators_opt(&self) -> Option<&super::MoveTable> {
            self.pending_active_validators.as_ref().map(|field| field as _)
        }
        pub fn set_pending_active_validators<T: Into<super::MoveTable>>(
            &mut self,
            field: T,
        ) {
            self.pending_active_validators = Some(field.into().into());
        }
        pub fn with_pending_active_validators<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_pending_active_validators(field);
            self
        }
        pub fn pending_removals(&self) -> &[u64] {
            &self.pending_removals
        }
        pub fn set_pending_removals(&mut self, field: Vec<u64>) {
            self.pending_removals = field;
        }
        pub fn with_pending_removals(mut self, field: Vec<u64>) -> Self {
            self.set_pending_removals(field);
            self
        }
        pub fn staking_pool_mappings(&self) -> &super::MoveTable {
            self.staking_pool_mappings
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn staking_pool_mappings_opt_mut(
            &mut self,
        ) -> Option<&mut super::MoveTable> {
            self.staking_pool_mappings.as_mut().map(|field| field as _)
        }
        pub fn staking_pool_mappings_mut(&mut self) -> &mut super::MoveTable {
            self.staking_pool_mappings.get_or_insert_default()
        }
        pub fn staking_pool_mappings_opt(&self) -> Option<&super::MoveTable> {
            self.staking_pool_mappings.as_ref().map(|field| field as _)
        }
        pub fn set_staking_pool_mappings<T: Into<super::MoveTable>>(
            &mut self,
            field: T,
        ) {
            self.staking_pool_mappings = Some(field.into().into());
        }
        pub fn with_staking_pool_mappings<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_staking_pool_mappings(field);
            self
        }
        pub fn inactive_validators(&self) -> &super::MoveTable {
            self.inactive_validators
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn inactive_validators_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.inactive_validators.as_mut().map(|field| field as _)
        }
        pub fn inactive_validators_mut(&mut self) -> &mut super::MoveTable {
            self.inactive_validators.get_or_insert_default()
        }
        pub fn inactive_validators_opt(&self) -> Option<&super::MoveTable> {
            self.inactive_validators.as_ref().map(|field| field as _)
        }
        pub fn set_inactive_validators<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.inactive_validators = Some(field.into().into());
        }
        pub fn with_inactive_validators<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_inactive_validators(field);
            self
        }
        pub fn validator_candidates(&self) -> &super::MoveTable {
            self.validator_candidates
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn validator_candidates_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.validator_candidates.as_mut().map(|field| field as _)
        }
        pub fn validator_candidates_mut(&mut self) -> &mut super::MoveTable {
            self.validator_candidates.get_or_insert_default()
        }
        pub fn validator_candidates_opt(&self) -> Option<&super::MoveTable> {
            self.validator_candidates.as_ref().map(|field| field as _)
        }
        pub fn set_validator_candidates<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.validator_candidates = Some(field.into().into());
        }
        pub fn with_validator_candidates<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_validator_candidates(field);
            self
        }
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field);
            self
        }
    }
    impl super::VariantDescriptor {
        pub const fn const_default() -> Self {
            Self {
                name: None,
                position: None,
                fields: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::VariantDescriptor = super::VariantDescriptor::const_default();
            &DEFAULT
        }
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field);
            self
        }
        pub fn position_opt_mut(&mut self) -> Option<&mut u32> {
            self.position.as_mut().map(|field| field as _)
        }
        pub fn position_mut(&mut self) -> &mut u32 {
            self.position.get_or_insert_default()
        }
        pub fn position_opt(&self) -> Option<u32> {
            self.position.as_ref().map(|field| *field)
        }
        pub fn set_position<T: Into<u32>>(&mut self, field: T) {
            self.position = Some(field.into().into());
        }
        pub fn with_position<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_position(field);
            self
        }
        pub fn fields(&self) -> &[super::FieldDescriptor] {
            &self.fields
        }
        pub fn set_fields(&mut self, field: Vec<super::FieldDescriptor>) {
            self.fields = field;
        }
        pub fn with_fields(mut self, field: Vec<super::FieldDescriptor>) -> Self {
            self.set_fields(field);
            self
        }
    }
    impl super::VerifySignatureRequest {
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
            static DEFAULT: super::VerifySignatureRequest = super::VerifySignatureRequest::const_default();
            &DEFAULT
        }
        pub fn message(&self) -> &super::Bcs {
            self.message
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        pub fn message_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.message.as_mut().map(|field| field as _)
        }
        pub fn message_mut(&mut self) -> &mut super::Bcs {
            self.message.get_or_insert_default()
        }
        pub fn message_opt(&self) -> Option<&super::Bcs> {
            self.message.as_ref().map(|field| field as _)
        }
        pub fn set_message<T: Into<super::Bcs>>(&mut self, field: T) {
            self.message = Some(field.into().into());
        }
        pub fn with_message<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_message(field);
            self
        }
        pub fn signature(&self) -> &super::UserSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::UserSignature::default_instance() as _)
        }
        pub fn signature_opt_mut(&mut self) -> Option<&mut super::UserSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        pub fn signature_mut(&mut self) -> &mut super::UserSignature {
            self.signature.get_or_insert_default()
        }
        pub fn signature_opt(&self) -> Option<&super::UserSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        pub fn set_signature<T: Into<super::UserSignature>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        pub fn with_signature<T: Into<super::UserSignature>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field);
            self
        }
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field);
            self
        }
        pub fn jwks(&self) -> &[super::ActiveJwk] {
            &self.jwks
        }
        pub fn set_jwks(&mut self, field: Vec<super::ActiveJwk>) {
            self.jwks = field;
        }
        pub fn with_jwks(mut self, field: Vec<super::ActiveJwk>) -> Self {
            self.set_jwks(field);
            self
        }
    }
    impl super::VerifySignatureResponse {
        pub const fn const_default() -> Self {
            Self {
                is_valid: None,
                reason: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::VerifySignatureResponse = super::VerifySignatureResponse::const_default();
            &DEFAULT
        }
        pub fn is_valid_opt_mut(&mut self) -> Option<&mut bool> {
            self.is_valid.as_mut().map(|field| field as _)
        }
        pub fn is_valid_mut(&mut self) -> &mut bool {
            self.is_valid.get_or_insert_default()
        }
        pub fn is_valid_opt(&self) -> Option<bool> {
            self.is_valid.as_ref().map(|field| *field)
        }
        pub fn set_is_valid<T: Into<bool>>(&mut self, field: T) {
            self.is_valid = Some(field.into().into());
        }
        pub fn with_is_valid<T: Into<bool>>(mut self, field: T) -> Self {
            self.set_is_valid(field);
            self
        }
        pub fn reason_opt_mut(&mut self) -> Option<&mut String> {
            self.reason.as_mut().map(|field| field as _)
        }
        pub fn reason_mut(&mut self) -> &mut String {
            self.reason.get_or_insert_default()
        }
        pub fn reason_opt(&self) -> Option<&str> {
            self.reason.as_ref().map(|field| field as _)
        }
        pub fn set_reason<T: Into<String>>(&mut self, field: T) {
            self.reason = Some(field.into().into());
        }
        pub fn with_reason<T: Into<String>>(mut self, field: T) -> Self {
            self.set_reason(field);
            self
        }
    }
    impl super::VersionAssignment {
        pub const fn const_default() -> Self {
            Self {
                object_id: None,
                start_version: None,
                version: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::VersionAssignment = super::VersionAssignment::const_default();
            &DEFAULT
        }
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field);
            self
        }
        pub fn start_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.start_version.as_mut().map(|field| field as _)
        }
        pub fn start_version_mut(&mut self) -> &mut u64 {
            self.start_version.get_or_insert_default()
        }
        pub fn start_version_opt(&self) -> Option<u64> {
            self.start_version.as_ref().map(|field| *field)
        }
        pub fn set_start_version<T: Into<u64>>(&mut self, field: T) {
            self.start_version = Some(field.into().into());
        }
        pub fn with_start_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_start_version(field);
            self
        }
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        pub fn set_version<T: Into<u64>>(&mut self, field: T) {
            self.version = Some(field.into().into());
        }
        pub fn with_version<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_version(field);
            self
        }
    }
    impl super::ZkLoginAuthenticator {
        pub const fn const_default() -> Self {
            Self {
                inputs: None,
                max_epoch: None,
                signature: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ZkLoginAuthenticator = super::ZkLoginAuthenticator::const_default();
            &DEFAULT
        }
        pub fn inputs(&self) -> &super::ZkLoginInputs {
            self.inputs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ZkLoginInputs::default_instance() as _)
        }
        pub fn inputs_opt_mut(&mut self) -> Option<&mut super::ZkLoginInputs> {
            self.inputs.as_mut().map(|field| field as _)
        }
        pub fn inputs_mut(&mut self) -> &mut super::ZkLoginInputs {
            self.inputs.get_or_insert_default()
        }
        pub fn inputs_opt(&self) -> Option<&super::ZkLoginInputs> {
            self.inputs.as_ref().map(|field| field as _)
        }
        pub fn set_inputs<T: Into<super::ZkLoginInputs>>(&mut self, field: T) {
            self.inputs = Some(field.into().into());
        }
        pub fn with_inputs<T: Into<super::ZkLoginInputs>>(mut self, field: T) -> Self {
            self.set_inputs(field);
            self
        }
        pub fn max_epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.max_epoch.as_mut().map(|field| field as _)
        }
        pub fn max_epoch_mut(&mut self) -> &mut u64 {
            self.max_epoch.get_or_insert_default()
        }
        pub fn max_epoch_opt(&self) -> Option<u64> {
            self.max_epoch.as_ref().map(|field| *field)
        }
        pub fn set_max_epoch<T: Into<u64>>(&mut self, field: T) {
            self.max_epoch = Some(field.into().into());
        }
        pub fn with_max_epoch<T: Into<u64>>(mut self, field: T) -> Self {
            self.set_max_epoch(field);
            self
        }
        pub fn signature(&self) -> &super::SimpleSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::SimpleSignature::default_instance() as _)
        }
        pub fn signature_opt_mut(&mut self) -> Option<&mut super::SimpleSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        pub fn signature_mut(&mut self) -> &mut super::SimpleSignature {
            self.signature.get_or_insert_default()
        }
        pub fn signature_opt(&self) -> Option<&super::SimpleSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        pub fn set_signature<T: Into<super::SimpleSignature>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        pub fn with_signature<T: Into<super::SimpleSignature>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field);
            self
        }
    }
    impl super::ZkLoginClaim {
        pub const fn const_default() -> Self {
            Self {
                value: None,
                index_mod_4: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ZkLoginClaim = super::ZkLoginClaim::const_default();
            &DEFAULT
        }
        pub fn value_opt_mut(&mut self) -> Option<&mut String> {
            self.value.as_mut().map(|field| field as _)
        }
        pub fn value_mut(&mut self) -> &mut String {
            self.value.get_or_insert_default()
        }
        pub fn value_opt(&self) -> Option<&str> {
            self.value.as_ref().map(|field| field as _)
        }
        pub fn set_value<T: Into<String>>(&mut self, field: T) {
            self.value = Some(field.into().into());
        }
        pub fn with_value<T: Into<String>>(mut self, field: T) -> Self {
            self.set_value(field);
            self
        }
        pub fn index_mod_4_opt_mut(&mut self) -> Option<&mut u32> {
            self.index_mod_4.as_mut().map(|field| field as _)
        }
        pub fn index_mod_4_mut(&mut self) -> &mut u32 {
            self.index_mod_4.get_or_insert_default()
        }
        pub fn index_mod_4_opt(&self) -> Option<u32> {
            self.index_mod_4.as_ref().map(|field| *field)
        }
        pub fn set_index_mod_4<T: Into<u32>>(&mut self, field: T) {
            self.index_mod_4 = Some(field.into().into());
        }
        pub fn with_index_mod_4<T: Into<u32>>(mut self, field: T) -> Self {
            self.set_index_mod_4(field);
            self
        }
    }
    impl super::ZkLoginInputs {
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
            static DEFAULT: super::ZkLoginInputs = super::ZkLoginInputs::const_default();
            &DEFAULT
        }
        pub fn proof_points(&self) -> &super::ZkLoginProof {
            self.proof_points
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ZkLoginProof::default_instance() as _)
        }
        pub fn proof_points_opt_mut(&mut self) -> Option<&mut super::ZkLoginProof> {
            self.proof_points.as_mut().map(|field| field as _)
        }
        pub fn proof_points_mut(&mut self) -> &mut super::ZkLoginProof {
            self.proof_points.get_or_insert_default()
        }
        pub fn proof_points_opt(&self) -> Option<&super::ZkLoginProof> {
            self.proof_points.as_ref().map(|field| field as _)
        }
        pub fn set_proof_points<T: Into<super::ZkLoginProof>>(&mut self, field: T) {
            self.proof_points = Some(field.into().into());
        }
        pub fn with_proof_points<T: Into<super::ZkLoginProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_proof_points(field);
            self
        }
        pub fn iss_base64_details(&self) -> &super::ZkLoginClaim {
            self.iss_base64_details
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ZkLoginClaim::default_instance() as _)
        }
        pub fn iss_base64_details_opt_mut(
            &mut self,
        ) -> Option<&mut super::ZkLoginClaim> {
            self.iss_base64_details.as_mut().map(|field| field as _)
        }
        pub fn iss_base64_details_mut(&mut self) -> &mut super::ZkLoginClaim {
            self.iss_base64_details.get_or_insert_default()
        }
        pub fn iss_base64_details_opt(&self) -> Option<&super::ZkLoginClaim> {
            self.iss_base64_details.as_ref().map(|field| field as _)
        }
        pub fn set_iss_base64_details<T: Into<super::ZkLoginClaim>>(
            &mut self,
            field: T,
        ) {
            self.iss_base64_details = Some(field.into().into());
        }
        pub fn with_iss_base64_details<T: Into<super::ZkLoginClaim>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_iss_base64_details(field);
            self
        }
        pub fn header_base64_opt_mut(&mut self) -> Option<&mut String> {
            self.header_base64.as_mut().map(|field| field as _)
        }
        pub fn header_base64_mut(&mut self) -> &mut String {
            self.header_base64.get_or_insert_default()
        }
        pub fn header_base64_opt(&self) -> Option<&str> {
            self.header_base64.as_ref().map(|field| field as _)
        }
        pub fn set_header_base64<T: Into<String>>(&mut self, field: T) {
            self.header_base64 = Some(field.into().into());
        }
        pub fn with_header_base64<T: Into<String>>(mut self, field: T) -> Self {
            self.set_header_base64(field);
            self
        }
        pub fn address_seed_opt_mut(&mut self) -> Option<&mut String> {
            self.address_seed.as_mut().map(|field| field as _)
        }
        pub fn address_seed_mut(&mut self) -> &mut String {
            self.address_seed.get_or_insert_default()
        }
        pub fn address_seed_opt(&self) -> Option<&str> {
            self.address_seed.as_ref().map(|field| field as _)
        }
        pub fn set_address_seed<T: Into<String>>(&mut self, field: T) {
            self.address_seed = Some(field.into().into());
        }
        pub fn with_address_seed<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address_seed(field);
            self
        }
    }
    impl super::ZkLoginProof {
        pub const fn const_default() -> Self {
            Self { a: None, b: None, c: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ZkLoginProof = super::ZkLoginProof::const_default();
            &DEFAULT
        }
        pub fn a(&self) -> &super::CircomG1 {
            self.a
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CircomG1::default_instance() as _)
        }
        pub fn a_opt_mut(&mut self) -> Option<&mut super::CircomG1> {
            self.a.as_mut().map(|field| field as _)
        }
        pub fn a_mut(&mut self) -> &mut super::CircomG1 {
            self.a.get_or_insert_default()
        }
        pub fn a_opt(&self) -> Option<&super::CircomG1> {
            self.a.as_ref().map(|field| field as _)
        }
        pub fn set_a<T: Into<super::CircomG1>>(&mut self, field: T) {
            self.a = Some(field.into().into());
        }
        pub fn with_a<T: Into<super::CircomG1>>(mut self, field: T) -> Self {
            self.set_a(field);
            self
        }
        pub fn b(&self) -> &super::CircomG2 {
            self.b
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CircomG2::default_instance() as _)
        }
        pub fn b_opt_mut(&mut self) -> Option<&mut super::CircomG2> {
            self.b.as_mut().map(|field| field as _)
        }
        pub fn b_mut(&mut self) -> &mut super::CircomG2 {
            self.b.get_or_insert_default()
        }
        pub fn b_opt(&self) -> Option<&super::CircomG2> {
            self.b.as_ref().map(|field| field as _)
        }
        pub fn set_b<T: Into<super::CircomG2>>(&mut self, field: T) {
            self.b = Some(field.into().into());
        }
        pub fn with_b<T: Into<super::CircomG2>>(mut self, field: T) -> Self {
            self.set_b(field);
            self
        }
        pub fn c(&self) -> &super::CircomG1 {
            self.c
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CircomG1::default_instance() as _)
        }
        pub fn c_opt_mut(&mut self) -> Option<&mut super::CircomG1> {
            self.c.as_mut().map(|field| field as _)
        }
        pub fn c_mut(&mut self) -> &mut super::CircomG1 {
            self.c.get_or_insert_default()
        }
        pub fn c_opt(&self) -> Option<&super::CircomG1> {
            self.c.as_ref().map(|field| field as _)
        }
        pub fn set_c<T: Into<super::CircomG1>>(&mut self, field: T) {
            self.c = Some(field.into().into());
        }
        pub fn with_c<T: Into<super::CircomG1>>(mut self, field: T) -> Self {
            self.set_c(field);
            self
        }
    }
    impl super::ZkLoginPublicIdentifier {
        pub const fn const_default() -> Self {
            Self {
                iss: None,
                address_seed: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ZkLoginPublicIdentifier = super::ZkLoginPublicIdentifier::const_default();
            &DEFAULT
        }
        pub fn iss_opt_mut(&mut self) -> Option<&mut String> {
            self.iss.as_mut().map(|field| field as _)
        }
        pub fn iss_mut(&mut self) -> &mut String {
            self.iss.get_or_insert_default()
        }
        pub fn iss_opt(&self) -> Option<&str> {
            self.iss.as_ref().map(|field| field as _)
        }
        pub fn set_iss<T: Into<String>>(&mut self, field: T) {
            self.iss = Some(field.into().into());
        }
        pub fn with_iss<T: Into<String>>(mut self, field: T) -> Self {
            self.set_iss(field);
            self
        }
        pub fn address_seed_opt_mut(&mut self) -> Option<&mut String> {
            self.address_seed.as_mut().map(|field| field as _)
        }
        pub fn address_seed_mut(&mut self) -> &mut String {
            self.address_seed.get_or_insert_default()
        }
        pub fn address_seed_opt(&self) -> Option<&str> {
            self.address_seed.as_ref().map(|field| field as _)
        }
        pub fn set_address_seed<T: Into<String>>(&mut self, field: T) {
            self.address_seed = Some(field.into().into());
        }
        pub fn with_address_seed<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address_seed(field);
            self
        }
    }
}
