mod _accessor_impls {
    #![allow(clippy::useless_conversion)]
    impl super::AccumulatorWrite {
        pub const fn const_default() -> Self {
            Self {
                address: None,
                accumulator_type: None,
                operation: None,
                value: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::AccumulatorWrite = super::AccumulatorWrite::const_default();
            &DEFAULT
        }
        ///If `address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        ///If `address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        ///Sets `address` with the provided value.
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        ///Sets `address` with the provided value.
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field.into());
            self
        }
        ///If `accumulator_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn accumulator_type_opt_mut(&mut self) -> Option<&mut String> {
            self.accumulator_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `accumulator_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn accumulator_type_mut(&mut self) -> &mut String {
            self.accumulator_type.get_or_insert_default()
        }
        ///If `accumulator_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn accumulator_type_opt(&self) -> Option<&str> {
            self.accumulator_type.as_ref().map(|field| field as _)
        }
        ///Sets `accumulator_type` with the provided value.
        pub fn set_accumulator_type<T: Into<String>>(&mut self, field: T) {
            self.accumulator_type = Some(field.into().into());
        }
        ///Sets `accumulator_type` with the provided value.
        pub fn with_accumulator_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_accumulator_type(field.into());
            self
        }
        ///Sets `operation` with the provided value.
        pub fn with_operation<T: Into<super::accumulator_write::AccumulatorOperation>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_operation(field.into());
            self
        }
        ///If `value` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn value_opt_mut(&mut self) -> Option<&mut u64> {
            self.value.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `value`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn value_mut(&mut self) -> &mut u64 {
            self.value.get_or_insert_default()
        }
        ///If `value` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn value_opt(&self) -> Option<u64> {
            self.value.as_ref().map(|field| *field)
        }
        ///Sets `value` with the provided value.
        pub fn set_value(&mut self, field: u64) {
            self.value = Some(field);
        }
        ///Sets `value` with the provided value.
        pub fn with_value(mut self, field: u64) -> Self {
            self.set_value(field);
            self
        }
    }
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
        ///Returns the value of `id`, or the default value if `id` is unset.
        pub fn id(&self) -> &super::JwkId {
            self.id
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::JwkId::default_instance() as _)
        }
        ///If `id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn id_opt_mut(&mut self) -> Option<&mut super::JwkId> {
            self.id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn id_mut(&mut self) -> &mut super::JwkId {
            self.id.get_or_insert_default()
        }
        ///If `id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn id_opt(&self) -> Option<&super::JwkId> {
            self.id.as_ref().map(|field| field as _)
        }
        ///Sets `id` with the provided value.
        pub fn set_id<T: Into<super::JwkId>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        ///Sets `id` with the provided value.
        pub fn with_id<T: Into<super::JwkId>>(mut self, field: T) -> Self {
            self.set_id(field.into());
            self
        }
        ///Returns the value of `jwk`, or the default value if `jwk` is unset.
        pub fn jwk(&self) -> &super::Jwk {
            self.jwk
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Jwk::default_instance() as _)
        }
        ///If `jwk` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn jwk_opt_mut(&mut self) -> Option<&mut super::Jwk> {
            self.jwk.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `jwk`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn jwk_mut(&mut self) -> &mut super::Jwk {
            self.jwk.get_or_insert_default()
        }
        ///If `jwk` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn jwk_opt(&self) -> Option<&super::Jwk> {
            self.jwk.as_ref().map(|field| field as _)
        }
        ///Sets `jwk` with the provided value.
        pub fn set_jwk<T: Into<super::Jwk>>(&mut self, field: T) {
            self.jwk = Some(field.into().into());
        }
        ///Sets `jwk` with the provided value.
        pub fn with_jwk<T: Into<super::Jwk>>(mut self, field: T) -> Self {
            self.set_jwk(field.into());
            self
        }
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
    }
    impl super::AddressAliasesVersion {
        pub const fn const_default() -> Self {
            Self { version: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::AddressAliasesVersion = super::AddressAliasesVersion::const_default();
            &DEFAULT
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
            self.set_version(field);
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
        ///Sets `kind` with the provided value.
        pub fn with_kind<T: Into<super::argument::ArgumentKind>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_kind(field.into());
            self
        }
        ///If `input` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn input_opt_mut(&mut self) -> Option<&mut u32> {
            self.input.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `input`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn input_mut(&mut self) -> &mut u32 {
            self.input.get_or_insert_default()
        }
        ///If `input` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn input_opt(&self) -> Option<u32> {
            self.input.as_ref().map(|field| *field)
        }
        ///Sets `input` with the provided value.
        pub fn set_input(&mut self, field: u32) {
            self.input = Some(field);
        }
        ///Sets `input` with the provided value.
        pub fn with_input(mut self, field: u32) -> Self {
            self.set_input(field);
            self
        }
        ///If `result` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn result_opt_mut(&mut self) -> Option<&mut u32> {
            self.result.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `result`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn result_mut(&mut self) -> &mut u32 {
            self.result.get_or_insert_default()
        }
        ///If `result` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn result_opt(&self) -> Option<u32> {
            self.result.as_ref().map(|field| *field)
        }
        ///Sets `result` with the provided value.
        pub fn set_result(&mut self, field: u32) {
            self.result = Some(field);
        }
        ///Sets `result` with the provided value.
        pub fn with_result(mut self, field: u32) -> Self {
            self.set_result(field);
            self
        }
        ///If `subresult` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn subresult_opt_mut(&mut self) -> Option<&mut u32> {
            self.subresult.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `subresult`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn subresult_mut(&mut self) -> &mut u32 {
            self.subresult.get_or_insert_default()
        }
        ///If `subresult` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn subresult_opt(&self) -> Option<u32> {
            self.subresult.as_ref().map(|field| *field)
        }
        ///Sets `subresult` with the provided value.
        pub fn set_subresult(&mut self, field: u32) {
            self.subresult = Some(field);
        }
        ///Sets `subresult` with the provided value.
        pub fn with_subresult(mut self, field: u32) -> Self {
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
        ///If `min_epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn min_epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.min_epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `min_epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn min_epoch_mut(&mut self) -> &mut u64 {
            self.min_epoch.get_or_insert_default()
        }
        ///If `min_epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn min_epoch_opt(&self) -> Option<u64> {
            self.min_epoch.as_ref().map(|field| *field)
        }
        ///Sets `min_epoch` with the provided value.
        pub fn set_min_epoch(&mut self, field: u64) {
            self.min_epoch = Some(field);
        }
        ///Sets `min_epoch` with the provided value.
        pub fn with_min_epoch(mut self, field: u64) -> Self {
            self.set_min_epoch(field);
            self
        }
        ///If `authenticator_object_initial_shared_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn authenticator_object_initial_shared_version_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.authenticator_object_initial_shared_version
                .as_mut()
                .map(|field| field as _)
        }
        ///Returns a mutable reference to `authenticator_object_initial_shared_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn authenticator_object_initial_shared_version_mut(&mut self) -> &mut u64 {
            self.authenticator_object_initial_shared_version.get_or_insert_default()
        }
        ///If `authenticator_object_initial_shared_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn authenticator_object_initial_shared_version_opt(&self) -> Option<u64> {
            self.authenticator_object_initial_shared_version.as_ref().map(|field| *field)
        }
        ///Sets `authenticator_object_initial_shared_version` with the provided value.
        pub fn set_authenticator_object_initial_shared_version(&mut self, field: u64) {
            self.authenticator_object_initial_shared_version = Some(field);
        }
        ///Sets `authenticator_object_initial_shared_version` with the provided value.
        pub fn with_authenticator_object_initial_shared_version(
            mut self,
            field: u64,
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
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///If `round` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn round_opt_mut(&mut self) -> Option<&mut u64> {
            self.round.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `round`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn round_mut(&mut self) -> &mut u64 {
            self.round.get_or_insert_default()
        }
        ///If `round` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn round_opt(&self) -> Option<u64> {
            self.round.as_ref().map(|field| *field)
        }
        ///Sets `round` with the provided value.
        pub fn set_round(&mut self, field: u64) {
            self.round = Some(field);
        }
        ///Sets `round` with the provided value.
        pub fn with_round(mut self, field: u64) -> Self {
            self.set_round(field);
            self
        }
        ///Returns the value of `new_active_jwks`, or the default value if `new_active_jwks` is unset.
        pub fn new_active_jwks(&self) -> &[super::ActiveJwk] {
            &self.new_active_jwks
        }
        ///Returns a mutable reference to `new_active_jwks`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn new_active_jwks_mut(&mut self) -> &mut Vec<super::ActiveJwk> {
            &mut self.new_active_jwks
        }
        ///Sets `new_active_jwks` with the provided value.
        pub fn set_new_active_jwks(&mut self, field: Vec<super::ActiveJwk>) {
            self.new_active_jwks = field;
        }
        ///Sets `new_active_jwks` with the provided value.
        pub fn with_new_active_jwks(mut self, field: Vec<super::ActiveJwk>) -> Self {
            self.set_new_active_jwks(field);
            self
        }
        ///If `authenticator_object_initial_shared_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn authenticator_object_initial_shared_version_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.authenticator_object_initial_shared_version
                .as_mut()
                .map(|field| field as _)
        }
        ///Returns a mutable reference to `authenticator_object_initial_shared_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn authenticator_object_initial_shared_version_mut(&mut self) -> &mut u64 {
            self.authenticator_object_initial_shared_version.get_or_insert_default()
        }
        ///If `authenticator_object_initial_shared_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn authenticator_object_initial_shared_version_opt(&self) -> Option<u64> {
            self.authenticator_object_initial_shared_version.as_ref().map(|field| *field)
        }
        ///Sets `authenticator_object_initial_shared_version` with the provided value.
        pub fn set_authenticator_object_initial_shared_version(&mut self, field: u64) {
            self.authenticator_object_initial_shared_version = Some(field);
        }
        ///Sets `authenticator_object_initial_shared_version` with the provided value.
        pub fn with_authenticator_object_initial_shared_version(
            mut self,
            field: u64,
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
                address_balance: None,
                coin_balance: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Balance = super::Balance::const_default();
            &DEFAULT
        }
        ///If `coin_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `coin_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        ///If `coin_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        ///Sets `coin_type` with the provided value.
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        ///Sets `coin_type` with the provided value.
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field.into());
            self
        }
        ///If `balance` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.balance.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `balance`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn balance_mut(&mut self) -> &mut u64 {
            self.balance.get_or_insert_default()
        }
        ///If `balance` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn balance_opt(&self) -> Option<u64> {
            self.balance.as_ref().map(|field| *field)
        }
        ///Sets `balance` with the provided value.
        pub fn set_balance(&mut self, field: u64) {
            self.balance = Some(field);
        }
        ///Sets `balance` with the provided value.
        pub fn with_balance(mut self, field: u64) -> Self {
            self.set_balance(field);
            self
        }
        ///If `address_balance` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn address_balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.address_balance.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `address_balance`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_balance_mut(&mut self) -> &mut u64 {
            self.address_balance.get_or_insert_default()
        }
        ///If `address_balance` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn address_balance_opt(&self) -> Option<u64> {
            self.address_balance.as_ref().map(|field| *field)
        }
        ///Sets `address_balance` with the provided value.
        pub fn set_address_balance(&mut self, field: u64) {
            self.address_balance = Some(field);
        }
        ///Sets `address_balance` with the provided value.
        pub fn with_address_balance(mut self, field: u64) -> Self {
            self.set_address_balance(field);
            self
        }
        ///If `coin_balance` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn coin_balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.coin_balance.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `coin_balance`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coin_balance_mut(&mut self) -> &mut u64 {
            self.coin_balance.get_or_insert_default()
        }
        ///If `coin_balance` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn coin_balance_opt(&self) -> Option<u64> {
            self.coin_balance.as_ref().map(|field| *field)
        }
        ///Sets `coin_balance` with the provided value.
        pub fn set_coin_balance(&mut self, field: u64) {
            self.coin_balance = Some(field);
        }
        ///Sets `coin_balance` with the provided value.
        pub fn with_coin_balance(mut self, field: u64) -> Self {
            self.set_coin_balance(field);
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
        ///If `address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        ///If `address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        ///Sets `address` with the provided value.
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        ///Sets `address` with the provided value.
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field.into());
            self
        }
        ///If `coin_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `coin_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        ///If `coin_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        ///Sets `coin_type` with the provided value.
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        ///Sets `coin_type` with the provided value.
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field.into());
            self
        }
        ///If `amount` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn amount_opt_mut(&mut self) -> Option<&mut String> {
            self.amount.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `amount`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn amount_mut(&mut self) -> &mut String {
            self.amount.get_or_insert_default()
        }
        ///If `amount` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn amount_opt(&self) -> Option<&str> {
            self.amount.as_ref().map(|field| field as _)
        }
        ///Sets `amount` with the provided value.
        pub fn set_amount<T: Into<String>>(&mut self, field: T) {
            self.amount = Some(field.into().into());
        }
        ///Sets `amount` with the provided value.
        pub fn with_amount<T: Into<String>>(mut self, field: T) -> Self {
            self.set_amount(field.into());
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
        ///Returns the value of `requests`, or the default value if `requests` is unset.
        pub fn requests(&self) -> &[super::GetObjectRequest] {
            &self.requests
        }
        ///Returns a mutable reference to `requests`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn requests_mut(&mut self) -> &mut Vec<super::GetObjectRequest> {
            &mut self.requests
        }
        ///Sets `requests` with the provided value.
        pub fn set_requests(&mut self, field: Vec<super::GetObjectRequest>) {
            self.requests = field;
        }
        ///Sets `requests` with the provided value.
        pub fn with_requests(mut self, field: Vec<super::GetObjectRequest>) -> Self {
            self.set_requests(field);
            self
        }
        ///If `read_mask` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `read_mask`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        ///If `read_mask` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        ///Sets `read_mask` with the provided value.
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        ///Sets `read_mask` with the provided value.
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field.into());
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
        ///Returns the value of `objects`, or the default value if `objects` is unset.
        pub fn objects(&self) -> &[super::GetObjectResult] {
            &self.objects
        }
        ///Returns a mutable reference to `objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn objects_mut(&mut self) -> &mut Vec<super::GetObjectResult> {
            &mut self.objects
        }
        ///Sets `objects` with the provided value.
        pub fn set_objects(&mut self, field: Vec<super::GetObjectResult>) {
            self.objects = field;
        }
        ///Sets `objects` with the provided value.
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
        ///Returns the value of `digests`, or the default value if `digests` is unset.
        pub fn digests(&self) -> &[String] {
            &self.digests
        }
        ///Returns a mutable reference to `digests`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digests_mut(&mut self) -> &mut Vec<String> {
            &mut self.digests
        }
        ///Sets `digests` with the provided value.
        pub fn set_digests(&mut self, field: Vec<String>) {
            self.digests = field;
        }
        ///Sets `digests` with the provided value.
        pub fn with_digests(mut self, field: Vec<String>) -> Self {
            self.set_digests(field);
            self
        }
        ///If `read_mask` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `read_mask`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        ///If `read_mask` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        ///Sets `read_mask` with the provided value.
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        ///Sets `read_mask` with the provided value.
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field.into());
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
        ///Returns the value of `transactions`, or the default value if `transactions` is unset.
        pub fn transactions(&self) -> &[super::GetTransactionResult] {
            &self.transactions
        }
        ///Returns a mutable reference to `transactions`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transactions_mut(&mut self) -> &mut Vec<super::GetTransactionResult> {
            &mut self.transactions
        }
        ///Sets `transactions` with the provided value.
        pub fn set_transactions(&mut self, field: Vec<super::GetTransactionResult>) {
            self.transactions = field;
        }
        ///Sets `transactions` with the provided value.
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
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
            self
        }
        ///If `value` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn value_opt(&self) -> Option<&[u8]> {
            self.value.as_ref().map(|field| field as _)
        }
        ///Sets `value` with the provided value.
        pub fn set_value<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.value = Some(field.into().into());
        }
        ///Sets `value` with the provided value.
        pub fn with_value<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_value(field.into());
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
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///Returns the value of `version_assignments`, or the default value if `version_assignments` is unset.
        pub fn version_assignments(&self) -> &[super::VersionAssignment] {
            &self.version_assignments
        }
        ///Returns a mutable reference to `version_assignments`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_assignments_mut(&mut self) -> &mut Vec<super::VersionAssignment> {
            &mut self.version_assignments
        }
        ///Sets `version_assignments` with the provided value.
        pub fn set_version_assignments(&mut self, field: Vec<super::VersionAssignment>) {
            self.version_assignments = field;
        }
        ///Sets `version_assignments` with the provided value.
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
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///If `protocol_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn protocol_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.protocol_version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `protocol_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn protocol_version_mut(&mut self) -> &mut u64 {
            self.protocol_version.get_or_insert_default()
        }
        ///If `protocol_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn protocol_version_opt(&self) -> Option<u64> {
            self.protocol_version.as_ref().map(|field| *field)
        }
        ///Sets `protocol_version` with the provided value.
        pub fn set_protocol_version(&mut self, field: u64) {
            self.protocol_version = Some(field);
        }
        ///Sets `protocol_version` with the provided value.
        pub fn with_protocol_version(mut self, field: u64) -> Self {
            self.set_protocol_version(field);
            self
        }
        ///If `storage_charge` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn storage_charge_opt_mut(&mut self) -> Option<&mut u64> {
            self.storage_charge.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `storage_charge`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn storage_charge_mut(&mut self) -> &mut u64 {
            self.storage_charge.get_or_insert_default()
        }
        ///If `storage_charge` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn storage_charge_opt(&self) -> Option<u64> {
            self.storage_charge.as_ref().map(|field| *field)
        }
        ///Sets `storage_charge` with the provided value.
        pub fn set_storage_charge(&mut self, field: u64) {
            self.storage_charge = Some(field);
        }
        ///Sets `storage_charge` with the provided value.
        pub fn with_storage_charge(mut self, field: u64) -> Self {
            self.set_storage_charge(field);
            self
        }
        ///If `computation_charge` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn computation_charge_opt_mut(&mut self) -> Option<&mut u64> {
            self.computation_charge.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `computation_charge`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn computation_charge_mut(&mut self) -> &mut u64 {
            self.computation_charge.get_or_insert_default()
        }
        ///If `computation_charge` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn computation_charge_opt(&self) -> Option<u64> {
            self.computation_charge.as_ref().map(|field| *field)
        }
        ///Sets `computation_charge` with the provided value.
        pub fn set_computation_charge(&mut self, field: u64) {
            self.computation_charge = Some(field);
        }
        ///Sets `computation_charge` with the provided value.
        pub fn with_computation_charge(mut self, field: u64) -> Self {
            self.set_computation_charge(field);
            self
        }
        ///If `storage_rebate` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn storage_rebate_opt_mut(&mut self) -> Option<&mut u64> {
            self.storage_rebate.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `storage_rebate`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn storage_rebate_mut(&mut self) -> &mut u64 {
            self.storage_rebate.get_or_insert_default()
        }
        ///If `storage_rebate` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn storage_rebate_opt(&self) -> Option<u64> {
            self.storage_rebate.as_ref().map(|field| *field)
        }
        ///Sets `storage_rebate` with the provided value.
        pub fn set_storage_rebate(&mut self, field: u64) {
            self.storage_rebate = Some(field);
        }
        ///Sets `storage_rebate` with the provided value.
        pub fn with_storage_rebate(mut self, field: u64) -> Self {
            self.set_storage_rebate(field);
            self
        }
        ///If `non_refundable_storage_fee` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn non_refundable_storage_fee_opt_mut(&mut self) -> Option<&mut u64> {
            self.non_refundable_storage_fee.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `non_refundable_storage_fee`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn non_refundable_storage_fee_mut(&mut self) -> &mut u64 {
            self.non_refundable_storage_fee.get_or_insert_default()
        }
        ///If `non_refundable_storage_fee` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn non_refundable_storage_fee_opt(&self) -> Option<u64> {
            self.non_refundable_storage_fee.as_ref().map(|field| *field)
        }
        ///Sets `non_refundable_storage_fee` with the provided value.
        pub fn set_non_refundable_storage_fee(&mut self, field: u64) {
            self.non_refundable_storage_fee = Some(field);
        }
        ///Sets `non_refundable_storage_fee` with the provided value.
        pub fn with_non_refundable_storage_fee(mut self, field: u64) -> Self {
            self.set_non_refundable_storage_fee(field);
            self
        }
        ///If `epoch_start_timestamp` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_start_timestamp_opt_mut(
            &mut self,
        ) -> Option<&mut ::prost_types::Timestamp> {
            self.epoch_start_timestamp.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch_start_timestamp`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_start_timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.epoch_start_timestamp.get_or_insert_default()
        }
        ///If `epoch_start_timestamp` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_start_timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.epoch_start_timestamp.as_ref().map(|field| field as _)
        }
        ///Sets `epoch_start_timestamp` with the provided value.
        pub fn set_epoch_start_timestamp<T: Into<::prost_types::Timestamp>>(
            &mut self,
            field: T,
        ) {
            self.epoch_start_timestamp = Some(field.into().into());
        }
        ///Sets `epoch_start_timestamp` with the provided value.
        pub fn with_epoch_start_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_epoch_start_timestamp(field.into());
            self
        }
        ///Returns the value of `system_packages`, or the default value if `system_packages` is unset.
        pub fn system_packages(&self) -> &[super::SystemPackage] {
            &self.system_packages
        }
        ///Returns a mutable reference to `system_packages`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn system_packages_mut(&mut self) -> &mut Vec<super::SystemPackage> {
            &mut self.system_packages
        }
        ///Sets `system_packages` with the provided value.
        pub fn set_system_packages(&mut self, field: Vec<super::SystemPackage>) {
            self.system_packages = field;
        }
        ///Sets `system_packages` with the provided value.
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
                accumulator_write: None,
                id_operation: None,
                object_type: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ChangedObject = super::ChangedObject::const_default();
            &DEFAULT
        }
        ///If `object_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        ///If `object_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        ///Sets `object_id` with the provided value.
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        ///Sets `object_id` with the provided value.
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field.into());
            self
        }
        ///Sets `input_state` with the provided value.
        pub fn with_input_state<T: Into<super::changed_object::InputObjectState>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_input_state(field.into());
            self
        }
        ///If `input_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn input_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.input_version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `input_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn input_version_mut(&mut self) -> &mut u64 {
            self.input_version.get_or_insert_default()
        }
        ///If `input_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn input_version_opt(&self) -> Option<u64> {
            self.input_version.as_ref().map(|field| *field)
        }
        ///Sets `input_version` with the provided value.
        pub fn set_input_version(&mut self, field: u64) {
            self.input_version = Some(field);
        }
        ///Sets `input_version` with the provided value.
        pub fn with_input_version(mut self, field: u64) -> Self {
            self.set_input_version(field);
            self
        }
        ///If `input_digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn input_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.input_digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `input_digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn input_digest_mut(&mut self) -> &mut String {
            self.input_digest.get_or_insert_default()
        }
        ///If `input_digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn input_digest_opt(&self) -> Option<&str> {
            self.input_digest.as_ref().map(|field| field as _)
        }
        ///Sets `input_digest` with the provided value.
        pub fn set_input_digest<T: Into<String>>(&mut self, field: T) {
            self.input_digest = Some(field.into().into());
        }
        ///Sets `input_digest` with the provided value.
        pub fn with_input_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_input_digest(field.into());
            self
        }
        ///Returns the value of `input_owner`, or the default value if `input_owner` is unset.
        pub fn input_owner(&self) -> &super::Owner {
            self.input_owner
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Owner::default_instance() as _)
        }
        ///If `input_owner` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn input_owner_opt_mut(&mut self) -> Option<&mut super::Owner> {
            self.input_owner.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `input_owner`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn input_owner_mut(&mut self) -> &mut super::Owner {
            self.input_owner.get_or_insert_default()
        }
        ///If `input_owner` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn input_owner_opt(&self) -> Option<&super::Owner> {
            self.input_owner.as_ref().map(|field| field as _)
        }
        ///Sets `input_owner` with the provided value.
        pub fn set_input_owner<T: Into<super::Owner>>(&mut self, field: T) {
            self.input_owner = Some(field.into().into());
        }
        ///Sets `input_owner` with the provided value.
        pub fn with_input_owner<T: Into<super::Owner>>(mut self, field: T) -> Self {
            self.set_input_owner(field.into());
            self
        }
        ///Sets `output_state` with the provided value.
        pub fn with_output_state<T: Into<super::changed_object::OutputObjectState>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_output_state(field.into());
            self
        }
        ///If `output_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn output_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.output_version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `output_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn output_version_mut(&mut self) -> &mut u64 {
            self.output_version.get_or_insert_default()
        }
        ///If `output_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn output_version_opt(&self) -> Option<u64> {
            self.output_version.as_ref().map(|field| *field)
        }
        ///Sets `output_version` with the provided value.
        pub fn set_output_version(&mut self, field: u64) {
            self.output_version = Some(field);
        }
        ///Sets `output_version` with the provided value.
        pub fn with_output_version(mut self, field: u64) -> Self {
            self.set_output_version(field);
            self
        }
        ///If `output_digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn output_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.output_digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `output_digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn output_digest_mut(&mut self) -> &mut String {
            self.output_digest.get_or_insert_default()
        }
        ///If `output_digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn output_digest_opt(&self) -> Option<&str> {
            self.output_digest.as_ref().map(|field| field as _)
        }
        ///Sets `output_digest` with the provided value.
        pub fn set_output_digest<T: Into<String>>(&mut self, field: T) {
            self.output_digest = Some(field.into().into());
        }
        ///Sets `output_digest` with the provided value.
        pub fn with_output_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_output_digest(field.into());
            self
        }
        ///Returns the value of `output_owner`, or the default value if `output_owner` is unset.
        pub fn output_owner(&self) -> &super::Owner {
            self.output_owner
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Owner::default_instance() as _)
        }
        ///If `output_owner` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn output_owner_opt_mut(&mut self) -> Option<&mut super::Owner> {
            self.output_owner.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `output_owner`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn output_owner_mut(&mut self) -> &mut super::Owner {
            self.output_owner.get_or_insert_default()
        }
        ///If `output_owner` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn output_owner_opt(&self) -> Option<&super::Owner> {
            self.output_owner.as_ref().map(|field| field as _)
        }
        ///Sets `output_owner` with the provided value.
        pub fn set_output_owner<T: Into<super::Owner>>(&mut self, field: T) {
            self.output_owner = Some(field.into().into());
        }
        ///Sets `output_owner` with the provided value.
        pub fn with_output_owner<T: Into<super::Owner>>(mut self, field: T) -> Self {
            self.set_output_owner(field.into());
            self
        }
        ///Returns the value of `accumulator_write`, or the default value if `accumulator_write` is unset.
        pub fn accumulator_write(&self) -> &super::AccumulatorWrite {
            self.accumulator_write
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::AccumulatorWrite::default_instance() as _)
        }
        ///If `accumulator_write` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn accumulator_write_opt_mut(
            &mut self,
        ) -> Option<&mut super::AccumulatorWrite> {
            self.accumulator_write.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `accumulator_write`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn accumulator_write_mut(&mut self) -> &mut super::AccumulatorWrite {
            self.accumulator_write.get_or_insert_default()
        }
        ///If `accumulator_write` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn accumulator_write_opt(&self) -> Option<&super::AccumulatorWrite> {
            self.accumulator_write.as_ref().map(|field| field as _)
        }
        ///Sets `accumulator_write` with the provided value.
        pub fn set_accumulator_write<T: Into<super::AccumulatorWrite>>(
            &mut self,
            field: T,
        ) {
            self.accumulator_write = Some(field.into().into());
        }
        ///Sets `accumulator_write` with the provided value.
        pub fn with_accumulator_write<T: Into<super::AccumulatorWrite>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_accumulator_write(field.into());
            self
        }
        ///Sets `id_operation` with the provided value.
        pub fn with_id_operation<T: Into<super::changed_object::IdOperation>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_id_operation(field.into());
            self
        }
        ///If `object_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_type_opt_mut(&mut self) -> Option<&mut String> {
            self.object_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_type_mut(&mut self) -> &mut String {
            self.object_type.get_or_insert_default()
        }
        ///If `object_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_type_opt(&self) -> Option<&str> {
            self.object_type.as_ref().map(|field| field as _)
        }
        ///Sets `object_type` with the provided value.
        pub fn set_object_type<T: Into<String>>(&mut self, field: T) {
            self.object_type = Some(field.into().into());
        }
        ///Sets `object_type` with the provided value.
        pub fn with_object_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_type(field.into());
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
                objects: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Checkpoint = super::Checkpoint::const_default();
            &DEFAULT
        }
        ///If `sequence_number` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn sequence_number_opt_mut(&mut self) -> Option<&mut u64> {
            self.sequence_number.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `sequence_number`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn sequence_number_mut(&mut self) -> &mut u64 {
            self.sequence_number.get_or_insert_default()
        }
        ///If `sequence_number` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn sequence_number_opt(&self) -> Option<u64> {
            self.sequence_number.as_ref().map(|field| *field)
        }
        ///Sets `sequence_number` with the provided value.
        pub fn set_sequence_number(&mut self, field: u64) {
            self.sequence_number = Some(field);
        }
        ///Sets `sequence_number` with the provided value.
        pub fn with_sequence_number(mut self, field: u64) -> Self {
            self.set_sequence_number(field);
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///Returns the value of `summary`, or the default value if `summary` is unset.
        pub fn summary(&self) -> &super::CheckpointSummary {
            self.summary
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CheckpointSummary::default_instance() as _)
        }
        ///If `summary` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn summary_opt_mut(&mut self) -> Option<&mut super::CheckpointSummary> {
            self.summary.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `summary`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn summary_mut(&mut self) -> &mut super::CheckpointSummary {
            self.summary.get_or_insert_default()
        }
        ///If `summary` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn summary_opt(&self) -> Option<&super::CheckpointSummary> {
            self.summary.as_ref().map(|field| field as _)
        }
        ///Sets `summary` with the provided value.
        pub fn set_summary<T: Into<super::CheckpointSummary>>(&mut self, field: T) {
            self.summary = Some(field.into().into());
        }
        ///Sets `summary` with the provided value.
        pub fn with_summary<T: Into<super::CheckpointSummary>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_summary(field.into());
            self
        }
        ///Returns the value of `signature`, or the default value if `signature` is unset.
        pub fn signature(&self) -> &super::ValidatorAggregatedSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::ValidatorAggregatedSignature::default_instance() as _
                })
        }
        ///If `signature` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn signature_opt_mut(
            &mut self,
        ) -> Option<&mut super::ValidatorAggregatedSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `signature`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn signature_mut(&mut self) -> &mut super::ValidatorAggregatedSignature {
            self.signature.get_or_insert_default()
        }
        ///If `signature` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn signature_opt(&self) -> Option<&super::ValidatorAggregatedSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        ///Sets `signature` with the provided value.
        pub fn set_signature<T: Into<super::ValidatorAggregatedSignature>>(
            &mut self,
            field: T,
        ) {
            self.signature = Some(field.into().into());
        }
        ///Sets `signature` with the provided value.
        pub fn with_signature<T: Into<super::ValidatorAggregatedSignature>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field.into());
            self
        }
        ///Returns the value of `contents`, or the default value if `contents` is unset.
        pub fn contents(&self) -> &super::CheckpointContents {
            self.contents
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CheckpointContents::default_instance() as _)
        }
        ///If `contents` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn contents_opt_mut(&mut self) -> Option<&mut super::CheckpointContents> {
            self.contents.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `contents`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn contents_mut(&mut self) -> &mut super::CheckpointContents {
            self.contents.get_or_insert_default()
        }
        ///If `contents` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn contents_opt(&self) -> Option<&super::CheckpointContents> {
            self.contents.as_ref().map(|field| field as _)
        }
        ///Sets `contents` with the provided value.
        pub fn set_contents<T: Into<super::CheckpointContents>>(&mut self, field: T) {
            self.contents = Some(field.into().into());
        }
        ///Sets `contents` with the provided value.
        pub fn with_contents<T: Into<super::CheckpointContents>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_contents(field.into());
            self
        }
        ///Returns the value of `transactions`, or the default value if `transactions` is unset.
        pub fn transactions(&self) -> &[super::ExecutedTransaction] {
            &self.transactions
        }
        ///Returns a mutable reference to `transactions`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transactions_mut(&mut self) -> &mut Vec<super::ExecutedTransaction> {
            &mut self.transactions
        }
        ///Sets `transactions` with the provided value.
        pub fn set_transactions(&mut self, field: Vec<super::ExecutedTransaction>) {
            self.transactions = field;
        }
        ///Sets `transactions` with the provided value.
        pub fn with_transactions(
            mut self,
            field: Vec<super::ExecutedTransaction>,
        ) -> Self {
            self.set_transactions(field);
            self
        }
        ///Returns the value of `objects`, or the default value if `objects` is unset.
        pub fn objects(&self) -> &super::ObjectSet {
            self.objects
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ObjectSet::default_instance() as _)
        }
        ///If `objects` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn objects_opt_mut(&mut self) -> Option<&mut super::ObjectSet> {
            self.objects.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn objects_mut(&mut self) -> &mut super::ObjectSet {
            self.objects.get_or_insert_default()
        }
        ///If `objects` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn objects_opt(&self) -> Option<&super::ObjectSet> {
            self.objects.as_ref().map(|field| field as _)
        }
        ///Sets `objects` with the provided value.
        pub fn set_objects<T: Into<super::ObjectSet>>(&mut self, field: T) {
            self.objects = Some(field.into().into());
        }
        ///Sets `objects` with the provided value.
        pub fn with_objects<T: Into<super::ObjectSet>>(mut self, field: T) -> Self {
            self.set_objects(field.into());
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
        ///Sets `kind` with the provided value.
        pub fn with_kind<
            T: Into<super::checkpoint_commitment::CheckpointCommitmentKind>,
        >(mut self, field: T) -> Self {
            self.set_kind(field.into());
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
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
        ///Returns the value of `bcs`, or the default value if `bcs` is unset.
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `bcs` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `bcs`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        ///If `bcs` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        ///Sets `bcs` with the provided value.
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        ///Sets `bcs` with the provided value.
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field.into());
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut i32> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut i32 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<i32> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: i32) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: i32) -> Self {
            self.set_version(field);
            self
        }
        ///Returns the value of `transactions`, or the default value if `transactions` is unset.
        pub fn transactions(&self) -> &[super::CheckpointedTransactionInfo] {
            &self.transactions
        }
        ///Returns a mutable reference to `transactions`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transactions_mut(
            &mut self,
        ) -> &mut Vec<super::CheckpointedTransactionInfo> {
            &mut self.transactions
        }
        ///Sets `transactions` with the provided value.
        pub fn set_transactions(
            &mut self,
            field: Vec<super::CheckpointedTransactionInfo>,
        ) {
            self.transactions = field;
        }
        ///Sets `transactions` with the provided value.
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
        ///Returns the value of `bcs`, or the default value if `bcs` is unset.
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `bcs` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `bcs`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        ///If `bcs` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        ///Sets `bcs` with the provided value.
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        ///Sets `bcs` with the provided value.
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field.into());
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///If `sequence_number` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn sequence_number_opt_mut(&mut self) -> Option<&mut u64> {
            self.sequence_number.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `sequence_number`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn sequence_number_mut(&mut self) -> &mut u64 {
            self.sequence_number.get_or_insert_default()
        }
        ///If `sequence_number` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn sequence_number_opt(&self) -> Option<u64> {
            self.sequence_number.as_ref().map(|field| *field)
        }
        ///Sets `sequence_number` with the provided value.
        pub fn set_sequence_number(&mut self, field: u64) {
            self.sequence_number = Some(field);
        }
        ///Sets `sequence_number` with the provided value.
        pub fn with_sequence_number(mut self, field: u64) -> Self {
            self.set_sequence_number(field);
            self
        }
        ///If `total_network_transactions` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn total_network_transactions_opt_mut(&mut self) -> Option<&mut u64> {
            self.total_network_transactions.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `total_network_transactions`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn total_network_transactions_mut(&mut self) -> &mut u64 {
            self.total_network_transactions.get_or_insert_default()
        }
        ///If `total_network_transactions` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn total_network_transactions_opt(&self) -> Option<u64> {
            self.total_network_transactions.as_ref().map(|field| *field)
        }
        ///Sets `total_network_transactions` with the provided value.
        pub fn set_total_network_transactions(&mut self, field: u64) {
            self.total_network_transactions = Some(field);
        }
        ///Sets `total_network_transactions` with the provided value.
        pub fn with_total_network_transactions(mut self, field: u64) -> Self {
            self.set_total_network_transactions(field);
            self
        }
        ///If `content_digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn content_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.content_digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `content_digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn content_digest_mut(&mut self) -> &mut String {
            self.content_digest.get_or_insert_default()
        }
        ///If `content_digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn content_digest_opt(&self) -> Option<&str> {
            self.content_digest.as_ref().map(|field| field as _)
        }
        ///Sets `content_digest` with the provided value.
        pub fn set_content_digest<T: Into<String>>(&mut self, field: T) {
            self.content_digest = Some(field.into().into());
        }
        ///Sets `content_digest` with the provided value.
        pub fn with_content_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_content_digest(field.into());
            self
        }
        ///If `previous_digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn previous_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.previous_digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `previous_digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn previous_digest_mut(&mut self) -> &mut String {
            self.previous_digest.get_or_insert_default()
        }
        ///If `previous_digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn previous_digest_opt(&self) -> Option<&str> {
            self.previous_digest.as_ref().map(|field| field as _)
        }
        ///Sets `previous_digest` with the provided value.
        pub fn set_previous_digest<T: Into<String>>(&mut self, field: T) {
            self.previous_digest = Some(field.into().into());
        }
        ///Sets `previous_digest` with the provided value.
        pub fn with_previous_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_previous_digest(field.into());
            self
        }
        ///Returns the value of `epoch_rolling_gas_cost_summary`, or the default value if `epoch_rolling_gas_cost_summary` is unset.
        pub fn epoch_rolling_gas_cost_summary(&self) -> &super::GasCostSummary {
            self.epoch_rolling_gas_cost_summary
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::GasCostSummary::default_instance() as _)
        }
        ///If `epoch_rolling_gas_cost_summary` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_rolling_gas_cost_summary_opt_mut(
            &mut self,
        ) -> Option<&mut super::GasCostSummary> {
            self.epoch_rolling_gas_cost_summary.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch_rolling_gas_cost_summary`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_rolling_gas_cost_summary_mut(
            &mut self,
        ) -> &mut super::GasCostSummary {
            self.epoch_rolling_gas_cost_summary.get_or_insert_default()
        }
        ///If `epoch_rolling_gas_cost_summary` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_rolling_gas_cost_summary_opt(
            &self,
        ) -> Option<&super::GasCostSummary> {
            self.epoch_rolling_gas_cost_summary.as_ref().map(|field| field as _)
        }
        ///Sets `epoch_rolling_gas_cost_summary` with the provided value.
        pub fn set_epoch_rolling_gas_cost_summary<T: Into<super::GasCostSummary>>(
            &mut self,
            field: T,
        ) {
            self.epoch_rolling_gas_cost_summary = Some(field.into().into());
        }
        ///Sets `epoch_rolling_gas_cost_summary` with the provided value.
        pub fn with_epoch_rolling_gas_cost_summary<T: Into<super::GasCostSummary>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_epoch_rolling_gas_cost_summary(field.into());
            self
        }
        ///If `timestamp` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn timestamp_opt_mut(&mut self) -> Option<&mut ::prost_types::Timestamp> {
            self.timestamp.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `timestamp`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.timestamp.get_or_insert_default()
        }
        ///If `timestamp` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.timestamp.as_ref().map(|field| field as _)
        }
        ///Sets `timestamp` with the provided value.
        pub fn set_timestamp<T: Into<::prost_types::Timestamp>>(&mut self, field: T) {
            self.timestamp = Some(field.into().into());
        }
        ///Sets `timestamp` with the provided value.
        pub fn with_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_timestamp(field.into());
            self
        }
        ///Returns the value of `commitments`, or the default value if `commitments` is unset.
        pub fn commitments(&self) -> &[super::CheckpointCommitment] {
            &self.commitments
        }
        ///Returns a mutable reference to `commitments`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn commitments_mut(&mut self) -> &mut Vec<super::CheckpointCommitment> {
            &mut self.commitments
        }
        ///Sets `commitments` with the provided value.
        pub fn set_commitments(&mut self, field: Vec<super::CheckpointCommitment>) {
            self.commitments = field;
        }
        ///Sets `commitments` with the provided value.
        pub fn with_commitments(
            mut self,
            field: Vec<super::CheckpointCommitment>,
        ) -> Self {
            self.set_commitments(field);
            self
        }
        ///Returns the value of `end_of_epoch_data`, or the default value if `end_of_epoch_data` is unset.
        pub fn end_of_epoch_data(&self) -> &super::EndOfEpochData {
            self.end_of_epoch_data
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::EndOfEpochData::default_instance() as _)
        }
        ///If `end_of_epoch_data` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn end_of_epoch_data_opt_mut(
            &mut self,
        ) -> Option<&mut super::EndOfEpochData> {
            self.end_of_epoch_data.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `end_of_epoch_data`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn end_of_epoch_data_mut(&mut self) -> &mut super::EndOfEpochData {
            self.end_of_epoch_data.get_or_insert_default()
        }
        ///If `end_of_epoch_data` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn end_of_epoch_data_opt(&self) -> Option<&super::EndOfEpochData> {
            self.end_of_epoch_data.as_ref().map(|field| field as _)
        }
        ///Sets `end_of_epoch_data` with the provided value.
        pub fn set_end_of_epoch_data<T: Into<super::EndOfEpochData>>(
            &mut self,
            field: T,
        ) {
            self.end_of_epoch_data = Some(field.into().into());
        }
        ///Sets `end_of_epoch_data` with the provided value.
        pub fn with_end_of_epoch_data<T: Into<super::EndOfEpochData>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_end_of_epoch_data(field.into());
            self
        }
        ///If `version_specific_data` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_specific_data_opt(&self) -> Option<&[u8]> {
            self.version_specific_data.as_ref().map(|field| field as _)
        }
        ///Sets `version_specific_data` with the provided value.
        pub fn set_version_specific_data<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.version_specific_data = Some(field.into().into());
        }
        ///Sets `version_specific_data` with the provided value.
        pub fn with_version_specific_data<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_version_specific_data(field.into());
            self
        }
    }
    impl super::CheckpointedTransactionInfo {
        pub const fn const_default() -> Self {
            Self {
                transaction: None,
                effects: None,
                signatures: Vec::new(),
                address_aliases_versions: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CheckpointedTransactionInfo = super::CheckpointedTransactionInfo::const_default();
            &DEFAULT
        }
        ///If `transaction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn transaction_opt_mut(&mut self) -> Option<&mut String> {
            self.transaction.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `transaction`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transaction_mut(&mut self) -> &mut String {
            self.transaction.get_or_insert_default()
        }
        ///If `transaction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn transaction_opt(&self) -> Option<&str> {
            self.transaction.as_ref().map(|field| field as _)
        }
        ///Sets `transaction` with the provided value.
        pub fn set_transaction<T: Into<String>>(&mut self, field: T) {
            self.transaction = Some(field.into().into());
        }
        ///Sets `transaction` with the provided value.
        pub fn with_transaction<T: Into<String>>(mut self, field: T) -> Self {
            self.set_transaction(field.into());
            self
        }
        ///If `effects` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn effects_opt_mut(&mut self) -> Option<&mut String> {
            self.effects.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `effects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn effects_mut(&mut self) -> &mut String {
            self.effects.get_or_insert_default()
        }
        ///If `effects` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn effects_opt(&self) -> Option<&str> {
            self.effects.as_ref().map(|field| field as _)
        }
        ///Sets `effects` with the provided value.
        pub fn set_effects<T: Into<String>>(&mut self, field: T) {
            self.effects = Some(field.into().into());
        }
        ///Sets `effects` with the provided value.
        pub fn with_effects<T: Into<String>>(mut self, field: T) -> Self {
            self.set_effects(field.into());
            self
        }
        ///Returns the value of `signatures`, or the default value if `signatures` is unset.
        pub fn signatures(&self) -> &[super::UserSignature] {
            &self.signatures
        }
        ///Returns a mutable reference to `signatures`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn signatures_mut(&mut self) -> &mut Vec<super::UserSignature> {
            &mut self.signatures
        }
        ///Sets `signatures` with the provided value.
        pub fn set_signatures(&mut self, field: Vec<super::UserSignature>) {
            self.signatures = field;
        }
        ///Sets `signatures` with the provided value.
        pub fn with_signatures(mut self, field: Vec<super::UserSignature>) -> Self {
            self.set_signatures(field);
            self
        }
        ///Returns the value of `address_aliases_versions`, or the default value if `address_aliases_versions` is unset.
        pub fn address_aliases_versions(&self) -> &[super::AddressAliasesVersion] {
            &self.address_aliases_versions
        }
        ///Returns a mutable reference to `address_aliases_versions`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_aliases_versions_mut(
            &mut self,
        ) -> &mut Vec<super::AddressAliasesVersion> {
            &mut self.address_aliases_versions
        }
        ///Sets `address_aliases_versions` with the provided value.
        pub fn set_address_aliases_versions(
            &mut self,
            field: Vec<super::AddressAliasesVersion>,
        ) {
            self.address_aliases_versions = field;
        }
        ///Sets `address_aliases_versions` with the provided value.
        pub fn with_address_aliases_versions(
            mut self,
            field: Vec<super::AddressAliasesVersion>,
        ) -> Self {
            self.set_address_aliases_versions(field);
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
        ///If `e0` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn e0_opt_mut(&mut self) -> Option<&mut String> {
            self.e0.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `e0`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn e0_mut(&mut self) -> &mut String {
            self.e0.get_or_insert_default()
        }
        ///If `e0` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn e0_opt(&self) -> Option<&str> {
            self.e0.as_ref().map(|field| field as _)
        }
        ///Sets `e0` with the provided value.
        pub fn set_e0<T: Into<String>>(&mut self, field: T) {
            self.e0 = Some(field.into().into());
        }
        ///Sets `e0` with the provided value.
        pub fn with_e0<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e0(field.into());
            self
        }
        ///If `e1` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn e1_opt_mut(&mut self) -> Option<&mut String> {
            self.e1.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `e1`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn e1_mut(&mut self) -> &mut String {
            self.e1.get_or_insert_default()
        }
        ///If `e1` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn e1_opt(&self) -> Option<&str> {
            self.e1.as_ref().map(|field| field as _)
        }
        ///Sets `e1` with the provided value.
        pub fn set_e1<T: Into<String>>(&mut self, field: T) {
            self.e1 = Some(field.into().into());
        }
        ///Sets `e1` with the provided value.
        pub fn with_e1<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e1(field.into());
            self
        }
        ///If `e2` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn e2_opt_mut(&mut self) -> Option<&mut String> {
            self.e2.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `e2`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn e2_mut(&mut self) -> &mut String {
            self.e2.get_or_insert_default()
        }
        ///If `e2` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn e2_opt(&self) -> Option<&str> {
            self.e2.as_ref().map(|field| field as _)
        }
        ///Sets `e2` with the provided value.
        pub fn set_e2<T: Into<String>>(&mut self, field: T) {
            self.e2 = Some(field.into().into());
        }
        ///Sets `e2` with the provided value.
        pub fn with_e2<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e2(field.into());
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
        ///If `e00` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn e00_opt_mut(&mut self) -> Option<&mut String> {
            self.e00.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `e00`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn e00_mut(&mut self) -> &mut String {
            self.e00.get_or_insert_default()
        }
        ///If `e00` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn e00_opt(&self) -> Option<&str> {
            self.e00.as_ref().map(|field| field as _)
        }
        ///Sets `e00` with the provided value.
        pub fn set_e00<T: Into<String>>(&mut self, field: T) {
            self.e00 = Some(field.into().into());
        }
        ///Sets `e00` with the provided value.
        pub fn with_e00<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e00(field.into());
            self
        }
        ///If `e01` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn e01_opt_mut(&mut self) -> Option<&mut String> {
            self.e01.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `e01`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn e01_mut(&mut self) -> &mut String {
            self.e01.get_or_insert_default()
        }
        ///If `e01` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn e01_opt(&self) -> Option<&str> {
            self.e01.as_ref().map(|field| field as _)
        }
        ///Sets `e01` with the provided value.
        pub fn set_e01<T: Into<String>>(&mut self, field: T) {
            self.e01 = Some(field.into().into());
        }
        ///Sets `e01` with the provided value.
        pub fn with_e01<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e01(field.into());
            self
        }
        ///If `e10` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn e10_opt_mut(&mut self) -> Option<&mut String> {
            self.e10.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `e10`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn e10_mut(&mut self) -> &mut String {
            self.e10.get_or_insert_default()
        }
        ///If `e10` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn e10_opt(&self) -> Option<&str> {
            self.e10.as_ref().map(|field| field as _)
        }
        ///Sets `e10` with the provided value.
        pub fn set_e10<T: Into<String>>(&mut self, field: T) {
            self.e10 = Some(field.into().into());
        }
        ///Sets `e10` with the provided value.
        pub fn with_e10<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e10(field.into());
            self
        }
        ///If `e11` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn e11_opt_mut(&mut self) -> Option<&mut String> {
            self.e11.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `e11`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn e11_mut(&mut self) -> &mut String {
            self.e11.get_or_insert_default()
        }
        ///If `e11` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn e11_opt(&self) -> Option<&str> {
            self.e11.as_ref().map(|field| field as _)
        }
        ///Sets `e11` with the provided value.
        pub fn set_e11<T: Into<String>>(&mut self, field: T) {
            self.e11 = Some(field.into().into());
        }
        ///Sets `e11` with the provided value.
        pub fn with_e11<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e11(field.into());
            self
        }
        ///If `e20` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn e20_opt_mut(&mut self) -> Option<&mut String> {
            self.e20.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `e20`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn e20_mut(&mut self) -> &mut String {
            self.e20.get_or_insert_default()
        }
        ///If `e20` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn e20_opt(&self) -> Option<&str> {
            self.e20.as_ref().map(|field| field as _)
        }
        ///Sets `e20` with the provided value.
        pub fn set_e20<T: Into<String>>(&mut self, field: T) {
            self.e20 = Some(field.into().into());
        }
        ///Sets `e20` with the provided value.
        pub fn with_e20<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e20(field.into());
            self
        }
        ///If `e21` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn e21_opt_mut(&mut self) -> Option<&mut String> {
            self.e21.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `e21`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn e21_mut(&mut self) -> &mut String {
            self.e21.get_or_insert_default()
        }
        ///If `e21` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn e21_opt(&self) -> Option<&str> {
            self.e21.as_ref().map(|field| field as _)
        }
        ///Sets `e21` with the provided value.
        pub fn set_e21<T: Into<String>>(&mut self, field: T) {
            self.e21 = Some(field.into().into());
        }
        ///Sets `e21` with the provided value.
        pub fn with_e21<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e21(field.into());
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
        ///If `error_code` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn error_code_opt_mut(&mut self) -> Option<&mut u64> {
            self.error_code.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `error_code`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn error_code_mut(&mut self) -> &mut u64 {
            self.error_code.get_or_insert_default()
        }
        ///If `error_code` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn error_code_opt(&self) -> Option<u64> {
            self.error_code.as_ref().map(|field| *field)
        }
        ///Sets `error_code` with the provided value.
        pub fn set_error_code(&mut self, field: u64) {
            self.error_code = Some(field);
        }
        ///Sets `error_code` with the provided value.
        pub fn with_error_code(mut self, field: u64) -> Self {
            self.set_error_code(field);
            self
        }
        ///If `line_number` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn line_number_opt_mut(&mut self) -> Option<&mut u64> {
            self.line_number.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `line_number`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn line_number_mut(&mut self) -> &mut u64 {
            self.line_number.get_or_insert_default()
        }
        ///If `line_number` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn line_number_opt(&self) -> Option<u64> {
            self.line_number.as_ref().map(|field| *field)
        }
        ///Sets `line_number` with the provided value.
        pub fn set_line_number(&mut self, field: u64) {
            self.line_number = Some(field);
        }
        ///Sets `line_number` with the provided value.
        pub fn with_line_number(mut self, field: u64) -> Self {
            self.set_line_number(field);
            self
        }
        ///If `constant_name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn constant_name_opt_mut(&mut self) -> Option<&mut String> {
            self.constant_name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `constant_name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn constant_name_mut(&mut self) -> &mut String {
            self.constant_name.get_or_insert_default()
        }
        ///If `constant_name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn constant_name_opt(&self) -> Option<&str> {
            self.constant_name.as_ref().map(|field| field as _)
        }
        ///Sets `constant_name` with the provided value.
        pub fn set_constant_name<T: Into<String>>(&mut self, field: T) {
            self.constant_name = Some(field.into().into());
        }
        ///Sets `constant_name` with the provided value.
        pub fn with_constant_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_constant_name(field.into());
            self
        }
        ///If `constant_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn constant_type_opt_mut(&mut self) -> Option<&mut String> {
            self.constant_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `constant_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn constant_type_mut(&mut self) -> &mut String {
            self.constant_type.get_or_insert_default()
        }
        ///If `constant_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn constant_type_opt(&self) -> Option<&str> {
            self.constant_type.as_ref().map(|field| field as _)
        }
        ///Sets `constant_type` with the provided value.
        pub fn set_constant_type<T: Into<String>>(&mut self, field: T) {
            self.constant_type = Some(field.into().into());
        }
        ///Sets `constant_type` with the provided value.
        pub fn with_constant_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_constant_type(field.into());
            self
        }
        ///Returns the value of `rendered`, or the default value if `rendered` is unset.
        pub fn rendered(&self) -> &str {
            if let Some(super::clever_error::Value::Rendered(field)) = &self.value {
                field as _
            } else {
                ""
            }
        }
        ///If `rendered` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn rendered_opt(&self) -> Option<&str> {
            if let Some(super::clever_error::Value::Rendered(field)) = &self.value {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `rendered` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn rendered_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(super::clever_error::Value::Rendered(field)) = &mut self.value {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `rendered`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn rendered_mut(&mut self) -> &mut String {
            if self.rendered_opt_mut().is_none() {
                self.value = Some(
                    super::clever_error::Value::Rendered(String::default()),
                );
            }
            self.rendered_opt_mut().unwrap()
        }
        ///Sets `rendered` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_rendered<T: Into<String>>(&mut self, field: T) {
            self.value = Some(super::clever_error::Value::Rendered(field.into().into()));
        }
        ///Sets `rendered` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_rendered<T: Into<String>>(mut self, field: T) -> Self {
            self.set_rendered(field.into());
            self
        }
        ///Returns the value of `raw`, or the default value if `raw` is unset.
        pub fn raw(&self) -> &[u8] {
            if let Some(super::clever_error::Value::Raw(field)) = &self.value {
                field as _
            } else {
                &[]
            }
        }
        ///If `raw` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn raw_opt(&self) -> Option<&[u8]> {
            if let Some(super::clever_error::Value::Raw(field)) = &self.value {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `raw` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn raw_opt_mut(&mut self) -> Option<&mut ::prost::bytes::Bytes> {
            if let Some(super::clever_error::Value::Raw(field)) = &mut self.value {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `raw`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn raw_mut(&mut self) -> &mut ::prost::bytes::Bytes {
            if self.raw_opt_mut().is_none() {
                self.value = Some(
                    super::clever_error::Value::Raw(::prost::bytes::Bytes::default()),
                );
            }
            self.raw_opt_mut().unwrap()
        }
        ///Sets `raw` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_raw<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.value = Some(super::clever_error::Value::Raw(field.into().into()));
        }
        ///Sets `raw` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_raw<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_raw(field.into());
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
        ///If `address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        ///If `address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        ///Sets `address` with the provided value.
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        ///Sets `address` with the provided value.
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field.into());
            self
        }
        ///If `coin_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `coin_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        ///If `coin_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        ///Sets `coin_type` with the provided value.
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        ///Sets `coin_type` with the provided value.
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field.into());
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
                metadata_cap_state: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CoinMetadata = super::CoinMetadata::const_default();
            &DEFAULT
        }
        ///If `id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        ///If `id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        ///Sets `id` with the provided value.
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        ///Sets `id` with the provided value.
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field.into());
            self
        }
        ///If `decimals` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn decimals_opt_mut(&mut self) -> Option<&mut u32> {
            self.decimals.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `decimals`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn decimals_mut(&mut self) -> &mut u32 {
            self.decimals.get_or_insert_default()
        }
        ///If `decimals` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn decimals_opt(&self) -> Option<u32> {
            self.decimals.as_ref().map(|field| *field)
        }
        ///Sets `decimals` with the provided value.
        pub fn set_decimals(&mut self, field: u32) {
            self.decimals = Some(field);
        }
        ///Sets `decimals` with the provided value.
        pub fn with_decimals(mut self, field: u32) -> Self {
            self.set_decimals(field);
            self
        }
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
            self
        }
        ///If `symbol` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn symbol_opt_mut(&mut self) -> Option<&mut String> {
            self.symbol.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `symbol`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn symbol_mut(&mut self) -> &mut String {
            self.symbol.get_or_insert_default()
        }
        ///If `symbol` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn symbol_opt(&self) -> Option<&str> {
            self.symbol.as_ref().map(|field| field as _)
        }
        ///Sets `symbol` with the provided value.
        pub fn set_symbol<T: Into<String>>(&mut self, field: T) {
            self.symbol = Some(field.into().into());
        }
        ///Sets `symbol` with the provided value.
        pub fn with_symbol<T: Into<String>>(mut self, field: T) -> Self {
            self.set_symbol(field.into());
            self
        }
        ///If `description` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn description_opt_mut(&mut self) -> Option<&mut String> {
            self.description.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `description`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn description_mut(&mut self) -> &mut String {
            self.description.get_or_insert_default()
        }
        ///If `description` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn description_opt(&self) -> Option<&str> {
            self.description.as_ref().map(|field| field as _)
        }
        ///Sets `description` with the provided value.
        pub fn set_description<T: Into<String>>(&mut self, field: T) {
            self.description = Some(field.into().into());
        }
        ///Sets `description` with the provided value.
        pub fn with_description<T: Into<String>>(mut self, field: T) -> Self {
            self.set_description(field.into());
            self
        }
        ///If `icon_url` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn icon_url_opt_mut(&mut self) -> Option<&mut String> {
            self.icon_url.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `icon_url`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn icon_url_mut(&mut self) -> &mut String {
            self.icon_url.get_or_insert_default()
        }
        ///If `icon_url` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn icon_url_opt(&self) -> Option<&str> {
            self.icon_url.as_ref().map(|field| field as _)
        }
        ///Sets `icon_url` with the provided value.
        pub fn set_icon_url<T: Into<String>>(&mut self, field: T) {
            self.icon_url = Some(field.into().into());
        }
        ///Sets `icon_url` with the provided value.
        pub fn with_icon_url<T: Into<String>>(mut self, field: T) -> Self {
            self.set_icon_url(field.into());
            self
        }
        ///If `metadata_cap_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn metadata_cap_id_opt_mut(&mut self) -> Option<&mut String> {
            self.metadata_cap_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `metadata_cap_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn metadata_cap_id_mut(&mut self) -> &mut String {
            self.metadata_cap_id.get_or_insert_default()
        }
        ///If `metadata_cap_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn metadata_cap_id_opt(&self) -> Option<&str> {
            self.metadata_cap_id.as_ref().map(|field| field as _)
        }
        ///Sets `metadata_cap_id` with the provided value.
        pub fn set_metadata_cap_id<T: Into<String>>(&mut self, field: T) {
            self.metadata_cap_id = Some(field.into().into());
        }
        ///Sets `metadata_cap_id` with the provided value.
        pub fn with_metadata_cap_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_metadata_cap_id(field.into());
            self
        }
        ///Sets `metadata_cap_state` with the provided value.
        pub fn with_metadata_cap_state<T: Into<super::coin_metadata::MetadataCapState>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_metadata_cap_state(field.into());
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
        ///If `id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        ///If `id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        ///Sets `id` with the provided value.
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        ///Sets `id` with the provided value.
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field.into());
            self
        }
        ///If `total_supply` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn total_supply_opt_mut(&mut self) -> Option<&mut u64> {
            self.total_supply.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `total_supply`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn total_supply_mut(&mut self) -> &mut u64 {
            self.total_supply.get_or_insert_default()
        }
        ///If `total_supply` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn total_supply_opt(&self) -> Option<u64> {
            self.total_supply.as_ref().map(|field| *field)
        }
        ///Sets `total_supply` with the provided value.
        pub fn set_total_supply(&mut self, field: u64) {
            self.total_supply = Some(field);
        }
        ///Sets `total_supply` with the provided value.
        pub fn with_total_supply(mut self, field: u64) -> Self {
            self.set_total_supply(field);
            self
        }
        ///Sets `supply_state` with the provided value.
        pub fn with_supply_state<T: Into<super::coin_treasury::SupplyState>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_supply_state(field.into());
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
        ///Returns the value of `move_call`, or the default value if `move_call` is unset.
        pub fn move_call(&self) -> &super::MoveCall {
            if let Some(super::command::Command::MoveCall(field)) = &self.command {
                field as _
            } else {
                super::MoveCall::default_instance() as _
            }
        }
        ///If `move_call` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn move_call_opt(&self) -> Option<&super::MoveCall> {
            if let Some(super::command::Command::MoveCall(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `move_call` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn move_call_opt_mut(&mut self) -> Option<&mut super::MoveCall> {
            if let Some(super::command::Command::MoveCall(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `move_call`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn move_call_mut(&mut self) -> &mut super::MoveCall {
            if self.move_call_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::MoveCall(super::MoveCall::default()),
                );
            }
            self.move_call_opt_mut().unwrap()
        }
        ///Sets `move_call` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_move_call<T: Into<super::MoveCall>>(&mut self, field: T) {
            self.command = Some(super::command::Command::MoveCall(field.into().into()));
        }
        ///Sets `move_call` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_move_call<T: Into<super::MoveCall>>(mut self, field: T) -> Self {
            self.set_move_call(field.into());
            self
        }
        ///Returns the value of `transfer_objects`, or the default value if `transfer_objects` is unset.
        pub fn transfer_objects(&self) -> &super::TransferObjects {
            if let Some(super::command::Command::TransferObjects(field)) = &self.command
            {
                field as _
            } else {
                super::TransferObjects::default_instance() as _
            }
        }
        ///If `transfer_objects` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn transfer_objects_opt(&self) -> Option<&super::TransferObjects> {
            if let Some(super::command::Command::TransferObjects(field)) = &self.command
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `transfer_objects` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `transfer_objects`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `transfer_objects` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_transfer_objects<T: Into<super::TransferObjects>>(
            &mut self,
            field: T,
        ) {
            self.command = Some(
                super::command::Command::TransferObjects(field.into().into()),
            );
        }
        ///Sets `transfer_objects` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_transfer_objects<T: Into<super::TransferObjects>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transfer_objects(field.into());
            self
        }
        ///Returns the value of `split_coins`, or the default value if `split_coins` is unset.
        pub fn split_coins(&self) -> &super::SplitCoins {
            if let Some(super::command::Command::SplitCoins(field)) = &self.command {
                field as _
            } else {
                super::SplitCoins::default_instance() as _
            }
        }
        ///If `split_coins` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn split_coins_opt(&self) -> Option<&super::SplitCoins> {
            if let Some(super::command::Command::SplitCoins(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `split_coins` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn split_coins_opt_mut(&mut self) -> Option<&mut super::SplitCoins> {
            if let Some(super::command::Command::SplitCoins(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `split_coins`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn split_coins_mut(&mut self) -> &mut super::SplitCoins {
            if self.split_coins_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::SplitCoins(super::SplitCoins::default()),
                );
            }
            self.split_coins_opt_mut().unwrap()
        }
        ///Sets `split_coins` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_split_coins<T: Into<super::SplitCoins>>(&mut self, field: T) {
            self.command = Some(
                super::command::Command::SplitCoins(field.into().into()),
            );
        }
        ///Sets `split_coins` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_split_coins<T: Into<super::SplitCoins>>(mut self, field: T) -> Self {
            self.set_split_coins(field.into());
            self
        }
        ///Returns the value of `merge_coins`, or the default value if `merge_coins` is unset.
        pub fn merge_coins(&self) -> &super::MergeCoins {
            if let Some(super::command::Command::MergeCoins(field)) = &self.command {
                field as _
            } else {
                super::MergeCoins::default_instance() as _
            }
        }
        ///If `merge_coins` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn merge_coins_opt(&self) -> Option<&super::MergeCoins> {
            if let Some(super::command::Command::MergeCoins(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `merge_coins` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn merge_coins_opt_mut(&mut self) -> Option<&mut super::MergeCoins> {
            if let Some(super::command::Command::MergeCoins(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `merge_coins`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn merge_coins_mut(&mut self) -> &mut super::MergeCoins {
            if self.merge_coins_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::MergeCoins(super::MergeCoins::default()),
                );
            }
            self.merge_coins_opt_mut().unwrap()
        }
        ///Sets `merge_coins` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_merge_coins<T: Into<super::MergeCoins>>(&mut self, field: T) {
            self.command = Some(
                super::command::Command::MergeCoins(field.into().into()),
            );
        }
        ///Sets `merge_coins` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_merge_coins<T: Into<super::MergeCoins>>(mut self, field: T) -> Self {
            self.set_merge_coins(field.into());
            self
        }
        ///Returns the value of `publish`, or the default value if `publish` is unset.
        pub fn publish(&self) -> &super::Publish {
            if let Some(super::command::Command::Publish(field)) = &self.command {
                field as _
            } else {
                super::Publish::default_instance() as _
            }
        }
        ///If `publish` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn publish_opt(&self) -> Option<&super::Publish> {
            if let Some(super::command::Command::Publish(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `publish` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn publish_opt_mut(&mut self) -> Option<&mut super::Publish> {
            if let Some(super::command::Command::Publish(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `publish`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn publish_mut(&mut self) -> &mut super::Publish {
            if self.publish_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::Publish(super::Publish::default()),
                );
            }
            self.publish_opt_mut().unwrap()
        }
        ///Sets `publish` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_publish<T: Into<super::Publish>>(&mut self, field: T) {
            self.command = Some(super::command::Command::Publish(field.into().into()));
        }
        ///Sets `publish` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_publish<T: Into<super::Publish>>(mut self, field: T) -> Self {
            self.set_publish(field.into());
            self
        }
        ///Returns the value of `make_move_vector`, or the default value if `make_move_vector` is unset.
        pub fn make_move_vector(&self) -> &super::MakeMoveVector {
            if let Some(super::command::Command::MakeMoveVector(field)) = &self.command {
                field as _
            } else {
                super::MakeMoveVector::default_instance() as _
            }
        }
        ///If `make_move_vector` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn make_move_vector_opt(&self) -> Option<&super::MakeMoveVector> {
            if let Some(super::command::Command::MakeMoveVector(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `make_move_vector` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `make_move_vector`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `make_move_vector` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_make_move_vector<T: Into<super::MakeMoveVector>>(
            &mut self,
            field: T,
        ) {
            self.command = Some(
                super::command::Command::MakeMoveVector(field.into().into()),
            );
        }
        ///Sets `make_move_vector` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_make_move_vector<T: Into<super::MakeMoveVector>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_make_move_vector(field.into());
            self
        }
        ///Returns the value of `upgrade`, or the default value if `upgrade` is unset.
        pub fn upgrade(&self) -> &super::Upgrade {
            if let Some(super::command::Command::Upgrade(field)) = &self.command {
                field as _
            } else {
                super::Upgrade::default_instance() as _
            }
        }
        ///If `upgrade` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn upgrade_opt(&self) -> Option<&super::Upgrade> {
            if let Some(super::command::Command::Upgrade(field)) = &self.command {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `upgrade` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn upgrade_opt_mut(&mut self) -> Option<&mut super::Upgrade> {
            if let Some(super::command::Command::Upgrade(field)) = &mut self.command {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `upgrade`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn upgrade_mut(&mut self) -> &mut super::Upgrade {
            if self.upgrade_opt_mut().is_none() {
                self.command = Some(
                    super::command::Command::Upgrade(super::Upgrade::default()),
                );
            }
            self.upgrade_opt_mut().unwrap()
        }
        ///Sets `upgrade` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_upgrade<T: Into<super::Upgrade>>(&mut self, field: T) {
            self.command = Some(super::command::Command::Upgrade(field.into().into()));
        }
        ///Sets `upgrade` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_upgrade<T: Into<super::Upgrade>>(mut self, field: T) -> Self {
            self.set_upgrade(field.into());
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
        ///If `argument` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn argument_opt_mut(&mut self) -> Option<&mut u32> {
            self.argument.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `argument`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn argument_mut(&mut self) -> &mut u32 {
            self.argument.get_or_insert_default()
        }
        ///If `argument` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn argument_opt(&self) -> Option<u32> {
            self.argument.as_ref().map(|field| *field)
        }
        ///Sets `argument` with the provided value.
        pub fn set_argument(&mut self, field: u32) {
            self.argument = Some(field);
        }
        ///Sets `argument` with the provided value.
        pub fn with_argument(mut self, field: u32) -> Self {
            self.set_argument(field);
            self
        }
        ///Sets `kind` with the provided value.
        pub fn with_kind<
            T: Into<super::command_argument_error::CommandArgumentErrorKind>,
        >(mut self, field: T) -> Self {
            self.set_kind(field.into());
            self
        }
        ///Returns the value of `index_error`, or the default value if `index_error` is unset.
        pub fn index_error(&self) -> &super::IndexError {
            self.index_error
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::IndexError::default_instance() as _)
        }
        ///If `index_error` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn index_error_opt_mut(&mut self) -> Option<&mut super::IndexError> {
            self.index_error.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `index_error`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn index_error_mut(&mut self) -> &mut super::IndexError {
            self.index_error.get_or_insert_default()
        }
        ///If `index_error` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn index_error_opt(&self) -> Option<&super::IndexError> {
            self.index_error.as_ref().map(|field| field as _)
        }
        ///Sets `index_error` with the provided value.
        pub fn set_index_error<T: Into<super::IndexError>>(&mut self, field: T) {
            self.index_error = Some(field.into().into());
        }
        ///Sets `index_error` with the provided value.
        pub fn with_index_error<T: Into<super::IndexError>>(mut self, field: T) -> Self {
            self.set_index_error(field.into());
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
        ///Returns the value of `argument`, or the default value if `argument` is unset.
        pub fn argument(&self) -> &super::Argument {
            self.argument
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Argument::default_instance() as _)
        }
        ///If `argument` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn argument_opt_mut(&mut self) -> Option<&mut super::Argument> {
            self.argument.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `argument`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn argument_mut(&mut self) -> &mut super::Argument {
            self.argument.get_or_insert_default()
        }
        ///If `argument` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn argument_opt(&self) -> Option<&super::Argument> {
            self.argument.as_ref().map(|field| field as _)
        }
        ///Sets `argument` with the provided value.
        pub fn set_argument<T: Into<super::Argument>>(&mut self, field: T) {
            self.argument = Some(field.into().into());
        }
        ///Sets `argument` with the provided value.
        pub fn with_argument<T: Into<super::Argument>>(mut self, field: T) -> Self {
            self.set_argument(field.into());
            self
        }
        ///Returns the value of `value`, or the default value if `value` is unset.
        pub fn value(&self) -> &super::Bcs {
            self.value
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `value` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn value_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.value.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `value`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn value_mut(&mut self) -> &mut super::Bcs {
            self.value.get_or_insert_default()
        }
        ///If `value` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn value_opt(&self) -> Option<&super::Bcs> {
            self.value.as_ref().map(|field| field as _)
        }
        ///Sets `value` with the provided value.
        pub fn set_value<T: Into<super::Bcs>>(&mut self, field: T) {
            self.value = Some(field.into().into());
        }
        ///Sets `value` with the provided value.
        pub fn with_value<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_value(field.into());
            self
        }
        ///If `json` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn json_opt_mut(&mut self) -> Option<&mut ::prost_types::Value> {
            self.json.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `json`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn json_mut(&mut self) -> &mut ::prost_types::Value {
            self.json.get_or_insert_default()
        }
        ///If `json` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn json_opt(&self) -> Option<&::prost_types::Value> {
            self.json.as_ref().map(|field| field as _)
        }
        ///Sets `json` with the provided value.
        pub fn set_json<T: Into<::prost_types::Value>>(&mut self, field: T) {
            self.json = Some(field.into().into());
        }
        ///Sets `json` with the provided value.
        pub fn with_json<T: Into<::prost_types::Value>>(mut self, field: T) -> Self {
            self.set_json(field.into());
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
        ///Returns the value of `return_values`, or the default value if `return_values` is unset.
        pub fn return_values(&self) -> &[super::CommandOutput] {
            &self.return_values
        }
        ///Returns a mutable reference to `return_values`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn return_values_mut(&mut self) -> &mut Vec<super::CommandOutput> {
            &mut self.return_values
        }
        ///Sets `return_values` with the provided value.
        pub fn set_return_values(&mut self, field: Vec<super::CommandOutput>) {
            self.return_values = field;
        }
        ///Sets `return_values` with the provided value.
        pub fn with_return_values(mut self, field: Vec<super::CommandOutput>) -> Self {
            self.set_return_values(field);
            self
        }
        ///Returns the value of `mutated_by_ref`, or the default value if `mutated_by_ref` is unset.
        pub fn mutated_by_ref(&self) -> &[super::CommandOutput] {
            &self.mutated_by_ref
        }
        ///Returns a mutable reference to `mutated_by_ref`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn mutated_by_ref_mut(&mut self) -> &mut Vec<super::CommandOutput> {
            &mut self.mutated_by_ref
        }
        ///Sets `mutated_by_ref` with the provided value.
        pub fn set_mutated_by_ref(&mut self, field: Vec<super::CommandOutput>) {
            self.mutated_by_ref = field;
        }
        ///Sets `mutated_by_ref` with the provided value.
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
        ///Returns the value of `objects`, or the default value if `objects` is unset.
        pub fn objects(&self) -> &[String] {
            &self.objects
        }
        ///Returns a mutable reference to `objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn objects_mut(&mut self) -> &mut Vec<String> {
            &mut self.objects
        }
        ///Sets `objects` with the provided value.
        pub fn set_objects(&mut self, field: Vec<String>) {
            self.objects = field;
        }
        ///Sets `objects` with the provided value.
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
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///If `round` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn round_opt_mut(&mut self) -> Option<&mut u64> {
            self.round.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `round`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn round_mut(&mut self) -> &mut u64 {
            self.round.get_or_insert_default()
        }
        ///If `round` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn round_opt(&self) -> Option<u64> {
            self.round.as_ref().map(|field| *field)
        }
        ///Sets `round` with the provided value.
        pub fn set_round(&mut self, field: u64) {
            self.round = Some(field);
        }
        ///Sets `round` with the provided value.
        pub fn with_round(mut self, field: u64) -> Self {
            self.set_round(field);
            self
        }
        ///If `commit_timestamp` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn commit_timestamp_opt_mut(
            &mut self,
        ) -> Option<&mut ::prost_types::Timestamp> {
            self.commit_timestamp.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `commit_timestamp`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn commit_timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.commit_timestamp.get_or_insert_default()
        }
        ///If `commit_timestamp` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn commit_timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.commit_timestamp.as_ref().map(|field| field as _)
        }
        ///Sets `commit_timestamp` with the provided value.
        pub fn set_commit_timestamp<T: Into<::prost_types::Timestamp>>(
            &mut self,
            field: T,
        ) {
            self.commit_timestamp = Some(field.into().into());
        }
        ///Sets `commit_timestamp` with the provided value.
        pub fn with_commit_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_commit_timestamp(field.into());
            self
        }
        ///If `consensus_commit_digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn consensus_commit_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.consensus_commit_digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `consensus_commit_digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn consensus_commit_digest_mut(&mut self) -> &mut String {
            self.consensus_commit_digest.get_or_insert_default()
        }
        ///If `consensus_commit_digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn consensus_commit_digest_opt(&self) -> Option<&str> {
            self.consensus_commit_digest.as_ref().map(|field| field as _)
        }
        ///Sets `consensus_commit_digest` with the provided value.
        pub fn set_consensus_commit_digest<T: Into<String>>(&mut self, field: T) {
            self.consensus_commit_digest = Some(field.into().into());
        }
        ///Sets `consensus_commit_digest` with the provided value.
        pub fn with_consensus_commit_digest<T: Into<String>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_consensus_commit_digest(field.into());
            self
        }
        ///If `sub_dag_index` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn sub_dag_index_opt_mut(&mut self) -> Option<&mut u64> {
            self.sub_dag_index.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `sub_dag_index`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn sub_dag_index_mut(&mut self) -> &mut u64 {
            self.sub_dag_index.get_or_insert_default()
        }
        ///If `sub_dag_index` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn sub_dag_index_opt(&self) -> Option<u64> {
            self.sub_dag_index.as_ref().map(|field| *field)
        }
        ///Sets `sub_dag_index` with the provided value.
        pub fn set_sub_dag_index(&mut self, field: u64) {
            self.sub_dag_index = Some(field);
        }
        ///Sets `sub_dag_index` with the provided value.
        pub fn with_sub_dag_index(mut self, field: u64) -> Self {
            self.set_sub_dag_index(field);
            self
        }
        ///Returns the value of `consensus_determined_version_assignments`, or the default value if `consensus_determined_version_assignments` is unset.
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
        ///If `consensus_determined_version_assignments` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn consensus_determined_version_assignments_opt_mut(
            &mut self,
        ) -> Option<&mut super::ConsensusDeterminedVersionAssignments> {
            self.consensus_determined_version_assignments
                .as_mut()
                .map(|field| field as _)
        }
        ///Returns a mutable reference to `consensus_determined_version_assignments`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn consensus_determined_version_assignments_mut(
            &mut self,
        ) -> &mut super::ConsensusDeterminedVersionAssignments {
            self.consensus_determined_version_assignments.get_or_insert_default()
        }
        ///If `consensus_determined_version_assignments` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn consensus_determined_version_assignments_opt(
            &self,
        ) -> Option<&super::ConsensusDeterminedVersionAssignments> {
            self.consensus_determined_version_assignments
                .as_ref()
                .map(|field| field as _)
        }
        ///Sets `consensus_determined_version_assignments` with the provided value.
        pub fn set_consensus_determined_version_assignments<
            T: Into<super::ConsensusDeterminedVersionAssignments>,
        >(&mut self, field: T) {
            self.consensus_determined_version_assignments = Some(field.into().into());
        }
        ///Sets `consensus_determined_version_assignments` with the provided value.
        pub fn with_consensus_determined_version_assignments<
            T: Into<super::ConsensusDeterminedVersionAssignments>,
        >(mut self, field: T) -> Self {
            self.set_consensus_determined_version_assignments(field.into());
            self
        }
        ///If `additional_state_digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn additional_state_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.additional_state_digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `additional_state_digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn additional_state_digest_mut(&mut self) -> &mut String {
            self.additional_state_digest.get_or_insert_default()
        }
        ///If `additional_state_digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn additional_state_digest_opt(&self) -> Option<&str> {
            self.additional_state_digest.as_ref().map(|field| field as _)
        }
        ///Sets `additional_state_digest` with the provided value.
        pub fn set_additional_state_digest<T: Into<String>>(&mut self, field: T) {
            self.additional_state_digest = Some(field.into().into());
        }
        ///Sets `additional_state_digest` with the provided value.
        pub fn with_additional_state_digest<T: Into<String>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_additional_state_digest(field.into());
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
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut i32> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut i32 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<i32> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: i32) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: i32) -> Self {
            self.set_version(field);
            self
        }
        ///Returns the value of `canceled_transactions`, or the default value if `canceled_transactions` is unset.
        pub fn canceled_transactions(&self) -> &[super::CanceledTransaction] {
            &self.canceled_transactions
        }
        ///Returns a mutable reference to `canceled_transactions`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn canceled_transactions_mut(
            &mut self,
        ) -> &mut Vec<super::CanceledTransaction> {
            &mut self.canceled_transactions
        }
        ///Sets `canceled_transactions` with the provided value.
        pub fn set_canceled_transactions(
            &mut self,
            field: Vec<super::CanceledTransaction>,
        ) {
            self.canceled_transactions = field;
        }
        ///Sets `canceled_transactions` with the provided value.
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
        ///If `type_name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn type_name_opt_mut(&mut self) -> Option<&mut String> {
            self.type_name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `type_name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn type_name_mut(&mut self) -> &mut String {
            self.type_name.get_or_insert_default()
        }
        ///If `type_name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn type_name_opt(&self) -> Option<&str> {
            self.type_name.as_ref().map(|field| field as _)
        }
        ///Sets `type_name` with the provided value.
        pub fn set_type_name<T: Into<String>>(&mut self, field: T) {
            self.type_name = Some(field.into().into());
        }
        ///Sets `type_name` with the provided value.
        pub fn with_type_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_type_name(field.into());
            self
        }
        ///If `defining_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn defining_id_opt_mut(&mut self) -> Option<&mut String> {
            self.defining_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `defining_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn defining_id_mut(&mut self) -> &mut String {
            self.defining_id.get_or_insert_default()
        }
        ///If `defining_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn defining_id_opt(&self) -> Option<&str> {
            self.defining_id.as_ref().map(|field| field as _)
        }
        ///Sets `defining_id` with the provided value.
        pub fn set_defining_id<T: Into<String>>(&mut self, field: T) {
            self.defining_id = Some(field.into().into());
        }
        ///Sets `defining_id` with the provided value.
        pub fn with_defining_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_defining_id(field.into());
            self
        }
        ///If `module` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn module_opt_mut(&mut self) -> Option<&mut String> {
            self.module.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `module`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn module_mut(&mut self) -> &mut String {
            self.module.get_or_insert_default()
        }
        ///If `module` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn module_opt(&self) -> Option<&str> {
            self.module.as_ref().map(|field| field as _)
        }
        ///Sets `module` with the provided value.
        pub fn set_module<T: Into<String>>(&mut self, field: T) {
            self.module = Some(field.into().into());
        }
        ///Sets `module` with the provided value.
        pub fn with_module<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module(field.into());
            self
        }
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
            self
        }
        ///Returns the value of `type_parameters`, or the default value if `type_parameters` is unset.
        pub fn type_parameters(&self) -> &[super::TypeParameter] {
            &self.type_parameters
        }
        ///Returns a mutable reference to `type_parameters`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn type_parameters_mut(&mut self) -> &mut Vec<super::TypeParameter> {
            &mut self.type_parameters
        }
        ///Sets `type_parameters` with the provided value.
        pub fn set_type_parameters(&mut self, field: Vec<super::TypeParameter>) {
            self.type_parameters = field;
        }
        ///Sets `type_parameters` with the provided value.
        pub fn with_type_parameters(mut self, field: Vec<super::TypeParameter>) -> Self {
            self.set_type_parameters(field);
            self
        }
        ///Sets `kind` with the provided value.
        pub fn with_kind<T: Into<super::datatype_descriptor::DatatypeKind>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_kind(field.into());
            self
        }
        ///Returns the value of `fields`, or the default value if `fields` is unset.
        pub fn fields(&self) -> &[super::FieldDescriptor] {
            &self.fields
        }
        ///Returns a mutable reference to `fields`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn fields_mut(&mut self) -> &mut Vec<super::FieldDescriptor> {
            &mut self.fields
        }
        ///Sets `fields` with the provided value.
        pub fn set_fields(&mut self, field: Vec<super::FieldDescriptor>) {
            self.fields = field;
        }
        ///Sets `fields` with the provided value.
        pub fn with_fields(mut self, field: Vec<super::FieldDescriptor>) -> Self {
            self.set_fields(field);
            self
        }
        ///Returns the value of `variants`, or the default value if `variants` is unset.
        pub fn variants(&self) -> &[super::VariantDescriptor] {
            &self.variants
        }
        ///Returns a mutable reference to `variants`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn variants_mut(&mut self) -> &mut Vec<super::VariantDescriptor> {
            &mut self.variants
        }
        ///Sets `variants` with the provided value.
        pub fn set_variants(&mut self, field: Vec<super::VariantDescriptor>) {
            self.variants = field;
        }
        ///Sets `variants` with the provided value.
        pub fn with_variants(mut self, field: Vec<super::VariantDescriptor>) -> Self {
            self.set_variants(field);
            self
        }
    }
    impl super::Display {
        pub const fn const_default() -> Self {
            Self { output: None, errors: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Display = super::Display::const_default();
            &DEFAULT
        }
        ///If `output` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn output_opt_mut(&mut self) -> Option<&mut ::prost_types::Value> {
            self.output.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `output`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn output_mut(&mut self) -> &mut ::prost_types::Value {
            self.output.get_or_insert_default()
        }
        ///If `output` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn output_opt(&self) -> Option<&::prost_types::Value> {
            self.output.as_ref().map(|field| field as _)
        }
        ///Sets `output` with the provided value.
        pub fn set_output<T: Into<::prost_types::Value>>(&mut self, field: T) {
            self.output = Some(field.into().into());
        }
        ///Sets `output` with the provided value.
        pub fn with_output<T: Into<::prost_types::Value>>(mut self, field: T) -> Self {
            self.set_output(field.into());
            self
        }
        ///If `errors` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn errors_opt_mut(&mut self) -> Option<&mut ::prost_types::Value> {
            self.errors.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `errors`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn errors_mut(&mut self) -> &mut ::prost_types::Value {
            self.errors.get_or_insert_default()
        }
        ///If `errors` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn errors_opt(&self) -> Option<&::prost_types::Value> {
            self.errors.as_ref().map(|field| field as _)
        }
        ///Sets `errors` with the provided value.
        pub fn set_errors<T: Into<::prost_types::Value>>(&mut self, field: T) {
            self.errors = Some(field.into().into());
        }
        ///Sets `errors` with the provided value.
        pub fn with_errors<T: Into<::prost_types::Value>>(mut self, field: T) -> Self {
            self.set_errors(field.into());
            self
        }
    }
    impl super::DynamicField {
        pub const fn const_default() -> Self {
            Self {
                kind: None,
                parent: None,
                field_id: None,
                field_object: None,
                name: None,
                value: None,
                value_type: None,
                child_id: None,
                child_object: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::DynamicField = super::DynamicField::const_default();
            &DEFAULT
        }
        ///Sets `kind` with the provided value.
        pub fn with_kind<T: Into<super::dynamic_field::DynamicFieldKind>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_kind(field.into());
            self
        }
        ///If `parent` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn parent_opt_mut(&mut self) -> Option<&mut String> {
            self.parent.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `parent`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn parent_mut(&mut self) -> &mut String {
            self.parent.get_or_insert_default()
        }
        ///If `parent` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn parent_opt(&self) -> Option<&str> {
            self.parent.as_ref().map(|field| field as _)
        }
        ///Sets `parent` with the provided value.
        pub fn set_parent<T: Into<String>>(&mut self, field: T) {
            self.parent = Some(field.into().into());
        }
        ///Sets `parent` with the provided value.
        pub fn with_parent<T: Into<String>>(mut self, field: T) -> Self {
            self.set_parent(field.into());
            self
        }
        ///If `field_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn field_id_opt_mut(&mut self) -> Option<&mut String> {
            self.field_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `field_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn field_id_mut(&mut self) -> &mut String {
            self.field_id.get_or_insert_default()
        }
        ///If `field_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn field_id_opt(&self) -> Option<&str> {
            self.field_id.as_ref().map(|field| field as _)
        }
        ///Sets `field_id` with the provided value.
        pub fn set_field_id<T: Into<String>>(&mut self, field: T) {
            self.field_id = Some(field.into().into());
        }
        ///Sets `field_id` with the provided value.
        pub fn with_field_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_field_id(field.into());
            self
        }
        ///Returns the value of `field_object`, or the default value if `field_object` is unset.
        pub fn field_object(&self) -> &super::Object {
            self.field_object
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Object::default_instance() as _)
        }
        ///If `field_object` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn field_object_opt_mut(&mut self) -> Option<&mut super::Object> {
            self.field_object.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `field_object`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn field_object_mut(&mut self) -> &mut super::Object {
            self.field_object.get_or_insert_default()
        }
        ///If `field_object` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn field_object_opt(&self) -> Option<&super::Object> {
            self.field_object.as_ref().map(|field| field as _)
        }
        ///Sets `field_object` with the provided value.
        pub fn set_field_object<T: Into<super::Object>>(&mut self, field: T) {
            self.field_object = Some(field.into().into());
        }
        ///Sets `field_object` with the provided value.
        pub fn with_field_object<T: Into<super::Object>>(mut self, field: T) -> Self {
            self.set_field_object(field.into());
            self
        }
        ///Returns the value of `name`, or the default value if `name` is unset.
        pub fn name(&self) -> &super::Bcs {
            self.name
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut super::Bcs {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&super::Bcs> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<super::Bcs>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_name(field.into());
            self
        }
        ///Returns the value of `value`, or the default value if `value` is unset.
        pub fn value(&self) -> &super::Bcs {
            self.value
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `value` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn value_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.value.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `value`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn value_mut(&mut self) -> &mut super::Bcs {
            self.value.get_or_insert_default()
        }
        ///If `value` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn value_opt(&self) -> Option<&super::Bcs> {
            self.value.as_ref().map(|field| field as _)
        }
        ///Sets `value` with the provided value.
        pub fn set_value<T: Into<super::Bcs>>(&mut self, field: T) {
            self.value = Some(field.into().into());
        }
        ///Sets `value` with the provided value.
        pub fn with_value<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_value(field.into());
            self
        }
        ///If `value_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn value_type_opt_mut(&mut self) -> Option<&mut String> {
            self.value_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `value_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn value_type_mut(&mut self) -> &mut String {
            self.value_type.get_or_insert_default()
        }
        ///If `value_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn value_type_opt(&self) -> Option<&str> {
            self.value_type.as_ref().map(|field| field as _)
        }
        ///Sets `value_type` with the provided value.
        pub fn set_value_type<T: Into<String>>(&mut self, field: T) {
            self.value_type = Some(field.into().into());
        }
        ///Sets `value_type` with the provided value.
        pub fn with_value_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_value_type(field.into());
            self
        }
        ///If `child_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn child_id_opt_mut(&mut self) -> Option<&mut String> {
            self.child_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `child_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn child_id_mut(&mut self) -> &mut String {
            self.child_id.get_or_insert_default()
        }
        ///If `child_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn child_id_opt(&self) -> Option<&str> {
            self.child_id.as_ref().map(|field| field as _)
        }
        ///Sets `child_id` with the provided value.
        pub fn set_child_id<T: Into<String>>(&mut self, field: T) {
            self.child_id = Some(field.into().into());
        }
        ///Sets `child_id` with the provided value.
        pub fn with_child_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_child_id(field.into());
            self
        }
        ///Returns the value of `child_object`, or the default value if `child_object` is unset.
        pub fn child_object(&self) -> &super::Object {
            self.child_object
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Object::default_instance() as _)
        }
        ///If `child_object` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn child_object_opt_mut(&mut self) -> Option<&mut super::Object> {
            self.child_object.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `child_object`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn child_object_mut(&mut self) -> &mut super::Object {
            self.child_object.get_or_insert_default()
        }
        ///If `child_object` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn child_object_opt(&self) -> Option<&super::Object> {
            self.child_object.as_ref().map(|field| field as _)
        }
        ///Sets `child_object` with the provided value.
        pub fn set_child_object<T: Into<super::Object>>(&mut self, field: T) {
            self.child_object = Some(field.into().into());
        }
        ///Sets `child_object` with the provided value.
        pub fn with_child_object<T: Into<super::Object>>(mut self, field: T) -> Self {
            self.set_child_object(field.into());
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
        ///Returns the value of `next_epoch_committee`, or the default value if `next_epoch_committee` is unset.
        pub fn next_epoch_committee(&self) -> &[super::ValidatorCommitteeMember] {
            &self.next_epoch_committee
        }
        ///Returns a mutable reference to `next_epoch_committee`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn next_epoch_committee_mut(
            &mut self,
        ) -> &mut Vec<super::ValidatorCommitteeMember> {
            &mut self.next_epoch_committee
        }
        ///Sets `next_epoch_committee` with the provided value.
        pub fn set_next_epoch_committee(
            &mut self,
            field: Vec<super::ValidatorCommitteeMember>,
        ) {
            self.next_epoch_committee = field;
        }
        ///Sets `next_epoch_committee` with the provided value.
        pub fn with_next_epoch_committee(
            mut self,
            field: Vec<super::ValidatorCommitteeMember>,
        ) -> Self {
            self.set_next_epoch_committee(field);
            self
        }
        ///If `next_epoch_protocol_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn next_epoch_protocol_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.next_epoch_protocol_version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `next_epoch_protocol_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn next_epoch_protocol_version_mut(&mut self) -> &mut u64 {
            self.next_epoch_protocol_version.get_or_insert_default()
        }
        ///If `next_epoch_protocol_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_protocol_version_opt(&self) -> Option<u64> {
            self.next_epoch_protocol_version.as_ref().map(|field| *field)
        }
        ///Sets `next_epoch_protocol_version` with the provided value.
        pub fn set_next_epoch_protocol_version(&mut self, field: u64) {
            self.next_epoch_protocol_version = Some(field);
        }
        ///Sets `next_epoch_protocol_version` with the provided value.
        pub fn with_next_epoch_protocol_version(mut self, field: u64) -> Self {
            self.set_next_epoch_protocol_version(field);
            self
        }
        ///Returns the value of `epoch_commitments`, or the default value if `epoch_commitments` is unset.
        pub fn epoch_commitments(&self) -> &[super::CheckpointCommitment] {
            &self.epoch_commitments
        }
        ///Returns a mutable reference to `epoch_commitments`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_commitments_mut(
            &mut self,
        ) -> &mut Vec<super::CheckpointCommitment> {
            &mut self.epoch_commitments
        }
        ///Sets `epoch_commitments` with the provided value.
        pub fn set_epoch_commitments(
            &mut self,
            field: Vec<super::CheckpointCommitment>,
        ) {
            self.epoch_commitments = field;
        }
        ///Sets `epoch_commitments` with the provided value.
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
        ///Returns the value of `transactions`, or the default value if `transactions` is unset.
        pub fn transactions(&self) -> &[super::EndOfEpochTransactionKind] {
            &self.transactions
        }
        ///Returns a mutable reference to `transactions`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transactions_mut(
            &mut self,
        ) -> &mut Vec<super::EndOfEpochTransactionKind> {
            &mut self.transactions
        }
        ///Sets `transactions` with the provided value.
        pub fn set_transactions(
            &mut self,
            field: Vec<super::EndOfEpochTransactionKind>,
        ) {
            self.transactions = field;
        }
        ///Sets `transactions` with the provided value.
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
            Self { kind: None, data: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EndOfEpochTransactionKind = super::EndOfEpochTransactionKind::const_default();
            &DEFAULT
        }
        ///Sets `kind` with the provided value.
        pub fn with_kind<T: Into<super::end_of_epoch_transaction_kind::Kind>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_kind(field.into());
            self
        }
        ///Returns the value of `change_epoch`, or the default value if `change_epoch` is unset.
        pub fn change_epoch(&self) -> &super::ChangeEpoch {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::ChangeEpoch(field),
            ) = &self.data
            {
                field as _
            } else {
                super::ChangeEpoch::default_instance() as _
            }
        }
        ///If `change_epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn change_epoch_opt(&self) -> Option<&super::ChangeEpoch> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::ChangeEpoch(field),
            ) = &self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `change_epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn change_epoch_opt_mut(&mut self) -> Option<&mut super::ChangeEpoch> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::ChangeEpoch(field),
            ) = &mut self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `change_epoch`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn change_epoch_mut(&mut self) -> &mut super::ChangeEpoch {
            if self.change_epoch_opt_mut().is_none() {
                self.data = Some(
                    super::end_of_epoch_transaction_kind::Data::ChangeEpoch(
                        super::ChangeEpoch::default(),
                    ),
                );
            }
            self.change_epoch_opt_mut().unwrap()
        }
        ///Sets `change_epoch` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_change_epoch<T: Into<super::ChangeEpoch>>(&mut self, field: T) {
            self.data = Some(
                super::end_of_epoch_transaction_kind::Data::ChangeEpoch(
                    field.into().into(),
                ),
            );
        }
        ///Sets `change_epoch` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_change_epoch<T: Into<super::ChangeEpoch>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_change_epoch(field.into());
            self
        }
        ///Returns the value of `authenticator_state_expire`, or the default value if `authenticator_state_expire` is unset.
        pub fn authenticator_state_expire(&self) -> &super::AuthenticatorStateExpire {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::AuthenticatorStateExpire(
                    field,
                ),
            ) = &self.data
            {
                field as _
            } else {
                super::AuthenticatorStateExpire::default_instance() as _
            }
        }
        ///If `authenticator_state_expire` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn authenticator_state_expire_opt(
            &self,
        ) -> Option<&super::AuthenticatorStateExpire> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::AuthenticatorStateExpire(
                    field,
                ),
            ) = &self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `authenticator_state_expire` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn authenticator_state_expire_opt_mut(
            &mut self,
        ) -> Option<&mut super::AuthenticatorStateExpire> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::AuthenticatorStateExpire(
                    field,
                ),
            ) = &mut self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `authenticator_state_expire`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn authenticator_state_expire_mut(
            &mut self,
        ) -> &mut super::AuthenticatorStateExpire {
            if self.authenticator_state_expire_opt_mut().is_none() {
                self.data = Some(
                    super::end_of_epoch_transaction_kind::Data::AuthenticatorStateExpire(
                        super::AuthenticatorStateExpire::default(),
                    ),
                );
            }
            self.authenticator_state_expire_opt_mut().unwrap()
        }
        ///Sets `authenticator_state_expire` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_authenticator_state_expire<T: Into<super::AuthenticatorStateExpire>>(
            &mut self,
            field: T,
        ) {
            self.data = Some(
                super::end_of_epoch_transaction_kind::Data::AuthenticatorStateExpire(
                    field.into().into(),
                ),
            );
        }
        ///Sets `authenticator_state_expire` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_authenticator_state_expire<T: Into<super::AuthenticatorStateExpire>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_authenticator_state_expire(field.into());
            self
        }
        ///Returns the value of `execution_time_observations`, or the default value if `execution_time_observations` is unset.
        pub fn execution_time_observations(&self) -> &super::ExecutionTimeObservations {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::ExecutionTimeObservations(
                    field,
                ),
            ) = &self.data
            {
                field as _
            } else {
                super::ExecutionTimeObservations::default_instance() as _
            }
        }
        ///If `execution_time_observations` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn execution_time_observations_opt(
            &self,
        ) -> Option<&super::ExecutionTimeObservations> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::ExecutionTimeObservations(
                    field,
                ),
            ) = &self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `execution_time_observations` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn execution_time_observations_opt_mut(
            &mut self,
        ) -> Option<&mut super::ExecutionTimeObservations> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::ExecutionTimeObservations(
                    field,
                ),
            ) = &mut self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `execution_time_observations`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn execution_time_observations_mut(
            &mut self,
        ) -> &mut super::ExecutionTimeObservations {
            if self.execution_time_observations_opt_mut().is_none() {
                self.data = Some(
                    super::end_of_epoch_transaction_kind::Data::ExecutionTimeObservations(
                        super::ExecutionTimeObservations::default(),
                    ),
                );
            }
            self.execution_time_observations_opt_mut().unwrap()
        }
        ///Sets `execution_time_observations` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_execution_time_observations<
            T: Into<super::ExecutionTimeObservations>,
        >(&mut self, field: T) {
            self.data = Some(
                super::end_of_epoch_transaction_kind::Data::ExecutionTimeObservations(
                    field.into().into(),
                ),
            );
        }
        ///Sets `execution_time_observations` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_execution_time_observations<
            T: Into<super::ExecutionTimeObservations>,
        >(mut self, field: T) -> Self {
            self.set_execution_time_observations(field.into());
            self
        }
        ///Returns the value of `bridge_chain_id`, or the default value if `bridge_chain_id` is unset.
        pub fn bridge_chain_id(&self) -> &str {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::BridgeChainId(field),
            ) = &self.data
            {
                field as _
            } else {
                ""
            }
        }
        ///If `bridge_chain_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn bridge_chain_id_opt(&self) -> Option<&str> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::BridgeChainId(field),
            ) = &self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `bridge_chain_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn bridge_chain_id_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::BridgeChainId(field),
            ) = &mut self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `bridge_chain_id`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn bridge_chain_id_mut(&mut self) -> &mut String {
            if self.bridge_chain_id_opt_mut().is_none() {
                self.data = Some(
                    super::end_of_epoch_transaction_kind::Data::BridgeChainId(
                        String::default(),
                    ),
                );
            }
            self.bridge_chain_id_opt_mut().unwrap()
        }
        ///Sets `bridge_chain_id` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_bridge_chain_id<T: Into<String>>(&mut self, field: T) {
            self.data = Some(
                super::end_of_epoch_transaction_kind::Data::BridgeChainId(
                    field.into().into(),
                ),
            );
        }
        ///Sets `bridge_chain_id` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_bridge_chain_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_bridge_chain_id(field.into());
            self
        }
        ///Returns the value of `bridge_object_version`, or the default value if `bridge_object_version` is unset.
        pub fn bridge_object_version(&self) -> u64 {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::BridgeObjectVersion(field),
            ) = &self.data
            {
                *field
            } else {
                0u64
            }
        }
        ///If `bridge_object_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn bridge_object_version_opt(&self) -> Option<u64> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::BridgeObjectVersion(field),
            ) = &self.data
            {
                Some(*field)
            } else {
                None
            }
        }
        ///If `bridge_object_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn bridge_object_version_opt_mut(&mut self) -> Option<&mut u64> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::BridgeObjectVersion(field),
            ) = &mut self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `bridge_object_version`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn bridge_object_version_mut(&mut self) -> &mut u64 {
            if self.bridge_object_version_opt_mut().is_none() {
                self.data = Some(
                    super::end_of_epoch_transaction_kind::Data::BridgeObjectVersion(
                        u64::default(),
                    ),
                );
            }
            self.bridge_object_version_opt_mut().unwrap()
        }
        ///Sets `bridge_object_version` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_bridge_object_version(&mut self, field: u64) {
            self.data = Some(
                super::end_of_epoch_transaction_kind::Data::BridgeObjectVersion(field),
            );
        }
        ///Sets `bridge_object_version` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_bridge_object_version(mut self, field: u64) -> Self {
            self.set_bridge_object_version(field);
            self
        }
        ///Returns the value of `storage_cost`, or the default value if `storage_cost` is unset.
        pub fn storage_cost(&self) -> u64 {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::StorageCost(field),
            ) = &self.data
            {
                *field
            } else {
                0u64
            }
        }
        ///If `storage_cost` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn storage_cost_opt(&self) -> Option<u64> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::StorageCost(field),
            ) = &self.data
            {
                Some(*field)
            } else {
                None
            }
        }
        ///If `storage_cost` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn storage_cost_opt_mut(&mut self) -> Option<&mut u64> {
            if let Some(
                super::end_of_epoch_transaction_kind::Data::StorageCost(field),
            ) = &mut self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `storage_cost`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn storage_cost_mut(&mut self) -> &mut u64 {
            if self.storage_cost_opt_mut().is_none() {
                self.data = Some(
                    super::end_of_epoch_transaction_kind::Data::StorageCost(
                        u64::default(),
                    ),
                );
            }
            self.storage_cost_opt_mut().unwrap()
        }
        ///Sets `storage_cost` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_storage_cost(&mut self, field: u64) {
            self.data = Some(
                super::end_of_epoch_transaction_kind::Data::StorageCost(field),
            );
        }
        ///Sets `storage_cost` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_storage_cost(mut self, field: u64) -> Self {
            self.set_storage_cost(field);
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
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///Returns the value of `committee`, or the default value if `committee` is unset.
        pub fn committee(&self) -> &super::ValidatorCommittee {
            self.committee
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ValidatorCommittee::default_instance() as _)
        }
        ///If `committee` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn committee_opt_mut(&mut self) -> Option<&mut super::ValidatorCommittee> {
            self.committee.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `committee`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn committee_mut(&mut self) -> &mut super::ValidatorCommittee {
            self.committee.get_or_insert_default()
        }
        ///If `committee` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn committee_opt(&self) -> Option<&super::ValidatorCommittee> {
            self.committee.as_ref().map(|field| field as _)
        }
        ///Sets `committee` with the provided value.
        pub fn set_committee<T: Into<super::ValidatorCommittee>>(&mut self, field: T) {
            self.committee = Some(field.into().into());
        }
        ///Sets `committee` with the provided value.
        pub fn with_committee<T: Into<super::ValidatorCommittee>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_committee(field.into());
            self
        }
        ///Returns the value of `system_state`, or the default value if `system_state` is unset.
        pub fn system_state(&self) -> &super::SystemState {
            self.system_state
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::SystemState::default_instance() as _)
        }
        ///If `system_state` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn system_state_opt_mut(&mut self) -> Option<&mut super::SystemState> {
            self.system_state.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `system_state`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn system_state_mut(&mut self) -> &mut super::SystemState {
            self.system_state.get_or_insert_default()
        }
        ///If `system_state` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn system_state_opt(&self) -> Option<&super::SystemState> {
            self.system_state.as_ref().map(|field| field as _)
        }
        ///Sets `system_state` with the provided value.
        pub fn set_system_state<T: Into<super::SystemState>>(&mut self, field: T) {
            self.system_state = Some(field.into().into());
        }
        ///Sets `system_state` with the provided value.
        pub fn with_system_state<T: Into<super::SystemState>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_system_state(field.into());
            self
        }
        ///If `first_checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn first_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.first_checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `first_checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn first_checkpoint_mut(&mut self) -> &mut u64 {
            self.first_checkpoint.get_or_insert_default()
        }
        ///If `first_checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn first_checkpoint_opt(&self) -> Option<u64> {
            self.first_checkpoint.as_ref().map(|field| *field)
        }
        ///Sets `first_checkpoint` with the provided value.
        pub fn set_first_checkpoint(&mut self, field: u64) {
            self.first_checkpoint = Some(field);
        }
        ///Sets `first_checkpoint` with the provided value.
        pub fn with_first_checkpoint(mut self, field: u64) -> Self {
            self.set_first_checkpoint(field);
            self
        }
        ///If `last_checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn last_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.last_checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `last_checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn last_checkpoint_mut(&mut self) -> &mut u64 {
            self.last_checkpoint.get_or_insert_default()
        }
        ///If `last_checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn last_checkpoint_opt(&self) -> Option<u64> {
            self.last_checkpoint.as_ref().map(|field| *field)
        }
        ///Sets `last_checkpoint` with the provided value.
        pub fn set_last_checkpoint(&mut self, field: u64) {
            self.last_checkpoint = Some(field);
        }
        ///Sets `last_checkpoint` with the provided value.
        pub fn with_last_checkpoint(mut self, field: u64) -> Self {
            self.set_last_checkpoint(field);
            self
        }
        ///If `start` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn start_opt_mut(&mut self) -> Option<&mut ::prost_types::Timestamp> {
            self.start.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `start`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn start_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.start.get_or_insert_default()
        }
        ///If `start` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn start_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.start.as_ref().map(|field| field as _)
        }
        ///Sets `start` with the provided value.
        pub fn set_start<T: Into<::prost_types::Timestamp>>(&mut self, field: T) {
            self.start = Some(field.into().into());
        }
        ///Sets `start` with the provided value.
        pub fn with_start<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_start(field.into());
            self
        }
        ///If `end` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn end_opt_mut(&mut self) -> Option<&mut ::prost_types::Timestamp> {
            self.end.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `end`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn end_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.end.get_or_insert_default()
        }
        ///If `end` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn end_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.end.as_ref().map(|field| field as _)
        }
        ///Sets `end` with the provided value.
        pub fn set_end<T: Into<::prost_types::Timestamp>>(&mut self, field: T) {
            self.end = Some(field.into().into());
        }
        ///Sets `end` with the provided value.
        pub fn with_end<T: Into<::prost_types::Timestamp>>(mut self, field: T) -> Self {
            self.set_end(field.into());
            self
        }
        ///If `reference_gas_price` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn reference_gas_price_opt_mut(&mut self) -> Option<&mut u64> {
            self.reference_gas_price.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `reference_gas_price`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn reference_gas_price_mut(&mut self) -> &mut u64 {
            self.reference_gas_price.get_or_insert_default()
        }
        ///If `reference_gas_price` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn reference_gas_price_opt(&self) -> Option<u64> {
            self.reference_gas_price.as_ref().map(|field| *field)
        }
        ///Sets `reference_gas_price` with the provided value.
        pub fn set_reference_gas_price(&mut self, field: u64) {
            self.reference_gas_price = Some(field);
        }
        ///Sets `reference_gas_price` with the provided value.
        pub fn with_reference_gas_price(mut self, field: u64) -> Self {
            self.set_reference_gas_price(field);
            self
        }
        ///Returns the value of `protocol_config`, or the default value if `protocol_config` is unset.
        pub fn protocol_config(&self) -> &super::ProtocolConfig {
            self.protocol_config
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ProtocolConfig::default_instance() as _)
        }
        ///If `protocol_config` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn protocol_config_opt_mut(&mut self) -> Option<&mut super::ProtocolConfig> {
            self.protocol_config.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `protocol_config`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn protocol_config_mut(&mut self) -> &mut super::ProtocolConfig {
            self.protocol_config.get_or_insert_default()
        }
        ///If `protocol_config` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn protocol_config_opt(&self) -> Option<&super::ProtocolConfig> {
            self.protocol_config.as_ref().map(|field| field as _)
        }
        ///Sets `protocol_config` with the provided value.
        pub fn set_protocol_config<T: Into<super::ProtocolConfig>>(&mut self, field: T) {
            self.protocol_config = Some(field.into().into());
        }
        ///Sets `protocol_config` with the provided value.
        pub fn with_protocol_config<T: Into<super::ProtocolConfig>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_protocol_config(field.into());
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
        ///If `package_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        ///If `package_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        ///Sets `package_id` with the provided value.
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        ///Sets `package_id` with the provided value.
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field.into());
            self
        }
        ///If `module` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn module_opt_mut(&mut self) -> Option<&mut String> {
            self.module.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `module`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn module_mut(&mut self) -> &mut String {
            self.module.get_or_insert_default()
        }
        ///If `module` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn module_opt(&self) -> Option<&str> {
            self.module.as_ref().map(|field| field as _)
        }
        ///Sets `module` with the provided value.
        pub fn set_module<T: Into<String>>(&mut self, field: T) {
            self.module = Some(field.into().into());
        }
        ///Sets `module` with the provided value.
        pub fn with_module<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module(field.into());
            self
        }
        ///If `sender` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn sender_opt_mut(&mut self) -> Option<&mut String> {
            self.sender.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `sender`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn sender_mut(&mut self) -> &mut String {
            self.sender.get_or_insert_default()
        }
        ///If `sender` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn sender_opt(&self) -> Option<&str> {
            self.sender.as_ref().map(|field| field as _)
        }
        ///Sets `sender` with the provided value.
        pub fn set_sender<T: Into<String>>(&mut self, field: T) {
            self.sender = Some(field.into().into());
        }
        ///Sets `sender` with the provided value.
        pub fn with_sender<T: Into<String>>(mut self, field: T) -> Self {
            self.set_sender(field.into());
            self
        }
        ///If `event_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn event_type_opt_mut(&mut self) -> Option<&mut String> {
            self.event_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `event_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn event_type_mut(&mut self) -> &mut String {
            self.event_type.get_or_insert_default()
        }
        ///If `event_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn event_type_opt(&self) -> Option<&str> {
            self.event_type.as_ref().map(|field| field as _)
        }
        ///Sets `event_type` with the provided value.
        pub fn set_event_type<T: Into<String>>(&mut self, field: T) {
            self.event_type = Some(field.into().into());
        }
        ///Sets `event_type` with the provided value.
        pub fn with_event_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_event_type(field.into());
            self
        }
        ///Returns the value of `contents`, or the default value if `contents` is unset.
        pub fn contents(&self) -> &super::Bcs {
            self.contents
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `contents` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn contents_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.contents.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `contents`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn contents_mut(&mut self) -> &mut super::Bcs {
            self.contents.get_or_insert_default()
        }
        ///If `contents` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn contents_opt(&self) -> Option<&super::Bcs> {
            self.contents.as_ref().map(|field| field as _)
        }
        ///Sets `contents` with the provided value.
        pub fn set_contents<T: Into<super::Bcs>>(&mut self, field: T) {
            self.contents = Some(field.into().into());
        }
        ///Sets `contents` with the provided value.
        pub fn with_contents<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_contents(field.into());
            self
        }
        ///If `json` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn json_opt_mut(&mut self) -> Option<&mut ::prost_types::Value> {
            self.json.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `json`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn json_mut(&mut self) -> &mut ::prost_types::Value {
            self.json.get_or_insert_default()
        }
        ///If `json` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn json_opt(&self) -> Option<&::prost_types::Value> {
            self.json.as_ref().map(|field| field as _)
        }
        ///Sets `json` with the provided value.
        pub fn set_json<T: Into<::prost_types::Value>>(&mut self, field: T) {
            self.json = Some(field.into().into());
        }
        ///Sets `json` with the provided value.
        pub fn with_json<T: Into<::prost_types::Value>>(mut self, field: T) -> Self {
            self.set_json(field.into());
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
        ///Returns the value of `transaction`, or the default value if `transaction` is unset.
        pub fn transaction(&self) -> &super::Transaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Transaction::default_instance() as _)
        }
        ///If `transaction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn transaction_opt_mut(&mut self) -> Option<&mut super::Transaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `transaction`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transaction_mut(&mut self) -> &mut super::Transaction {
            self.transaction.get_or_insert_default()
        }
        ///If `transaction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn transaction_opt(&self) -> Option<&super::Transaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        ///Sets `transaction` with the provided value.
        pub fn set_transaction<T: Into<super::Transaction>>(&mut self, field: T) {
            self.transaction = Some(field.into().into());
        }
        ///Sets `transaction` with the provided value.
        pub fn with_transaction<T: Into<super::Transaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field.into());
            self
        }
        ///Returns the value of `signatures`, or the default value if `signatures` is unset.
        pub fn signatures(&self) -> &[super::UserSignature] {
            &self.signatures
        }
        ///Returns a mutable reference to `signatures`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn signatures_mut(&mut self) -> &mut Vec<super::UserSignature> {
            &mut self.signatures
        }
        ///Sets `signatures` with the provided value.
        pub fn set_signatures(&mut self, field: Vec<super::UserSignature>) {
            self.signatures = field;
        }
        ///Sets `signatures` with the provided value.
        pub fn with_signatures(mut self, field: Vec<super::UserSignature>) -> Self {
            self.set_signatures(field);
            self
        }
        ///If `read_mask` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `read_mask`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        ///If `read_mask` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        ///Sets `read_mask` with the provided value.
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        ///Sets `read_mask` with the provided value.
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field.into());
            self
        }
    }
    impl super::ExecuteTransactionResponse {
        pub const fn const_default() -> Self {
            Self { transaction: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ExecuteTransactionResponse = super::ExecuteTransactionResponse::const_default();
            &DEFAULT
        }
        ///Returns the value of `transaction`, or the default value if `transaction` is unset.
        pub fn transaction(&self) -> &super::ExecutedTransaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ExecutedTransaction::default_instance() as _)
        }
        ///If `transaction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn transaction_opt_mut(
            &mut self,
        ) -> Option<&mut super::ExecutedTransaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `transaction`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transaction_mut(&mut self) -> &mut super::ExecutedTransaction {
            self.transaction.get_or_insert_default()
        }
        ///If `transaction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn transaction_opt(&self) -> Option<&super::ExecutedTransaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        ///Sets `transaction` with the provided value.
        pub fn set_transaction<T: Into<super::ExecutedTransaction>>(
            &mut self,
            field: T,
        ) {
            self.transaction = Some(field.into().into());
        }
        ///Sets `transaction` with the provided value.
        pub fn with_transaction<T: Into<super::ExecutedTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field.into());
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
                objects: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ExecutedTransaction = super::ExecutedTransaction::const_default();
            &DEFAULT
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///Returns the value of `transaction`, or the default value if `transaction` is unset.
        pub fn transaction(&self) -> &super::Transaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Transaction::default_instance() as _)
        }
        ///If `transaction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn transaction_opt_mut(&mut self) -> Option<&mut super::Transaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `transaction`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transaction_mut(&mut self) -> &mut super::Transaction {
            self.transaction.get_or_insert_default()
        }
        ///If `transaction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn transaction_opt(&self) -> Option<&super::Transaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        ///Sets `transaction` with the provided value.
        pub fn set_transaction<T: Into<super::Transaction>>(&mut self, field: T) {
            self.transaction = Some(field.into().into());
        }
        ///Sets `transaction` with the provided value.
        pub fn with_transaction<T: Into<super::Transaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field.into());
            self
        }
        ///Returns the value of `signatures`, or the default value if `signatures` is unset.
        pub fn signatures(&self) -> &[super::UserSignature] {
            &self.signatures
        }
        ///Returns a mutable reference to `signatures`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn signatures_mut(&mut self) -> &mut Vec<super::UserSignature> {
            &mut self.signatures
        }
        ///Sets `signatures` with the provided value.
        pub fn set_signatures(&mut self, field: Vec<super::UserSignature>) {
            self.signatures = field;
        }
        ///Sets `signatures` with the provided value.
        pub fn with_signatures(mut self, field: Vec<super::UserSignature>) -> Self {
            self.set_signatures(field);
            self
        }
        ///Returns the value of `effects`, or the default value if `effects` is unset.
        pub fn effects(&self) -> &super::TransactionEffects {
            self.effects
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::TransactionEffects::default_instance() as _)
        }
        ///If `effects` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn effects_opt_mut(&mut self) -> Option<&mut super::TransactionEffects> {
            self.effects.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `effects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn effects_mut(&mut self) -> &mut super::TransactionEffects {
            self.effects.get_or_insert_default()
        }
        ///If `effects` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn effects_opt(&self) -> Option<&super::TransactionEffects> {
            self.effects.as_ref().map(|field| field as _)
        }
        ///Sets `effects` with the provided value.
        pub fn set_effects<T: Into<super::TransactionEffects>>(&mut self, field: T) {
            self.effects = Some(field.into().into());
        }
        ///Sets `effects` with the provided value.
        pub fn with_effects<T: Into<super::TransactionEffects>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_effects(field.into());
            self
        }
        ///Returns the value of `events`, or the default value if `events` is unset.
        pub fn events(&self) -> &super::TransactionEvents {
            self.events
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::TransactionEvents::default_instance() as _)
        }
        ///If `events` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn events_opt_mut(&mut self) -> Option<&mut super::TransactionEvents> {
            self.events.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `events`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn events_mut(&mut self) -> &mut super::TransactionEvents {
            self.events.get_or_insert_default()
        }
        ///If `events` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn events_opt(&self) -> Option<&super::TransactionEvents> {
            self.events.as_ref().map(|field| field as _)
        }
        ///Sets `events` with the provided value.
        pub fn set_events<T: Into<super::TransactionEvents>>(&mut self, field: T) {
            self.events = Some(field.into().into());
        }
        ///Sets `events` with the provided value.
        pub fn with_events<T: Into<super::TransactionEvents>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_events(field.into());
            self
        }
        ///If `checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn checkpoint_mut(&mut self) -> &mut u64 {
            self.checkpoint.get_or_insert_default()
        }
        ///If `checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn checkpoint_opt(&self) -> Option<u64> {
            self.checkpoint.as_ref().map(|field| *field)
        }
        ///Sets `checkpoint` with the provided value.
        pub fn set_checkpoint(&mut self, field: u64) {
            self.checkpoint = Some(field);
        }
        ///Sets `checkpoint` with the provided value.
        pub fn with_checkpoint(mut self, field: u64) -> Self {
            self.set_checkpoint(field);
            self
        }
        ///If `timestamp` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn timestamp_opt_mut(&mut self) -> Option<&mut ::prost_types::Timestamp> {
            self.timestamp.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `timestamp`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.timestamp.get_or_insert_default()
        }
        ///If `timestamp` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.timestamp.as_ref().map(|field| field as _)
        }
        ///Sets `timestamp` with the provided value.
        pub fn set_timestamp<T: Into<::prost_types::Timestamp>>(&mut self, field: T) {
            self.timestamp = Some(field.into().into());
        }
        ///Sets `timestamp` with the provided value.
        pub fn with_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_timestamp(field.into());
            self
        }
        ///Returns the value of `balance_changes`, or the default value if `balance_changes` is unset.
        pub fn balance_changes(&self) -> &[super::BalanceChange] {
            &self.balance_changes
        }
        ///Returns a mutable reference to `balance_changes`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn balance_changes_mut(&mut self) -> &mut Vec<super::BalanceChange> {
            &mut self.balance_changes
        }
        ///Sets `balance_changes` with the provided value.
        pub fn set_balance_changes(&mut self, field: Vec<super::BalanceChange>) {
            self.balance_changes = field;
        }
        ///Sets `balance_changes` with the provided value.
        pub fn with_balance_changes(mut self, field: Vec<super::BalanceChange>) -> Self {
            self.set_balance_changes(field);
            self
        }
        ///Returns the value of `objects`, or the default value if `objects` is unset.
        pub fn objects(&self) -> &super::ObjectSet {
            self.objects
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ObjectSet::default_instance() as _)
        }
        ///If `objects` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn objects_opt_mut(&mut self) -> Option<&mut super::ObjectSet> {
            self.objects.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn objects_mut(&mut self) -> &mut super::ObjectSet {
            self.objects.get_or_insert_default()
        }
        ///If `objects` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn objects_opt(&self) -> Option<&super::ObjectSet> {
            self.objects.as_ref().map(|field| field as _)
        }
        ///Sets `objects` with the provided value.
        pub fn set_objects<T: Into<super::ObjectSet>>(&mut self, field: T) {
            self.objects = Some(field.into().into());
        }
        ///Sets `objects` with the provided value.
        pub fn with_objects<T: Into<super::ObjectSet>>(mut self, field: T) -> Self {
            self.set_objects(field.into());
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
        ///If `description` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn description_opt_mut(&mut self) -> Option<&mut String> {
            self.description.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `description`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn description_mut(&mut self) -> &mut String {
            self.description.get_or_insert_default()
        }
        ///If `description` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn description_opt(&self) -> Option<&str> {
            self.description.as_ref().map(|field| field as _)
        }
        ///Sets `description` with the provided value.
        pub fn set_description<T: Into<String>>(&mut self, field: T) {
            self.description = Some(field.into().into());
        }
        ///Sets `description` with the provided value.
        pub fn with_description<T: Into<String>>(mut self, field: T) -> Self {
            self.set_description(field.into());
            self
        }
        ///If `command` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn command_opt_mut(&mut self) -> Option<&mut u64> {
            self.command.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `command`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn command_mut(&mut self) -> &mut u64 {
            self.command.get_or_insert_default()
        }
        ///If `command` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn command_opt(&self) -> Option<u64> {
            self.command.as_ref().map(|field| *field)
        }
        ///Sets `command` with the provided value.
        pub fn set_command(&mut self, field: u64) {
            self.command = Some(field);
        }
        ///Sets `command` with the provided value.
        pub fn with_command(mut self, field: u64) -> Self {
            self.set_command(field);
            self
        }
        ///Sets `kind` with the provided value.
        pub fn with_kind<T: Into<super::execution_error::ExecutionErrorKind>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_kind(field.into());
            self
        }
        ///Returns the value of `abort`, or the default value if `abort` is unset.
        pub fn abort(&self) -> &super::MoveAbort {
            if let Some(super::execution_error::ErrorDetails::Abort(field)) = &self
                .error_details
            {
                field as _
            } else {
                super::MoveAbort::default_instance() as _
            }
        }
        ///If `abort` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn abort_opt(&self) -> Option<&super::MoveAbort> {
            if let Some(super::execution_error::ErrorDetails::Abort(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `abort` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn abort_opt_mut(&mut self) -> Option<&mut super::MoveAbort> {
            if let Some(super::execution_error::ErrorDetails::Abort(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `abort`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `abort` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_abort<T: Into<super::MoveAbort>>(&mut self, field: T) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::Abort(field.into().into()),
            );
        }
        ///Sets `abort` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_abort<T: Into<super::MoveAbort>>(mut self, field: T) -> Self {
            self.set_abort(field.into());
            self
        }
        ///Returns the value of `size_error`, or the default value if `size_error` is unset.
        pub fn size_error(&self) -> &super::SizeError {
            if let Some(super::execution_error::ErrorDetails::SizeError(field)) = &self
                .error_details
            {
                field as _
            } else {
                super::SizeError::default_instance() as _
            }
        }
        ///If `size_error` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn size_error_opt(&self) -> Option<&super::SizeError> {
            if let Some(super::execution_error::ErrorDetails::SizeError(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `size_error` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn size_error_opt_mut(&mut self) -> Option<&mut super::SizeError> {
            if let Some(super::execution_error::ErrorDetails::SizeError(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `size_error`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `size_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_size_error<T: Into<super::SizeError>>(&mut self, field: T) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::SizeError(field.into().into()),
            );
        }
        ///Sets `size_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_size_error<T: Into<super::SizeError>>(mut self, field: T) -> Self {
            self.set_size_error(field.into());
            self
        }
        ///Returns the value of `command_argument_error`, or the default value if `command_argument_error` is unset.
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
        ///If `command_argument_error` is set, returns [`Some`] with the value; otherwise returns [`None`].
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
        ///If `command_argument_error` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `command_argument_error`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `command_argument_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `command_argument_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_command_argument_error<T: Into<super::CommandArgumentError>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_command_argument_error(field.into());
            self
        }
        ///Returns the value of `type_argument_error`, or the default value if `type_argument_error` is unset.
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
        ///If `type_argument_error` is set, returns [`Some`] with the value; otherwise returns [`None`].
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
        ///If `type_argument_error` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `type_argument_error`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `type_argument_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `type_argument_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_type_argument_error<T: Into<super::TypeArgumentError>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_type_argument_error(field.into());
            self
        }
        ///Returns the value of `package_upgrade_error`, or the default value if `package_upgrade_error` is unset.
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
        ///If `package_upgrade_error` is set, returns [`Some`] with the value; otherwise returns [`None`].
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
        ///If `package_upgrade_error` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `package_upgrade_error`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `package_upgrade_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `package_upgrade_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_package_upgrade_error<T: Into<super::PackageUpgradeError>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_package_upgrade_error(field.into());
            self
        }
        ///Returns the value of `index_error`, or the default value if `index_error` is unset.
        pub fn index_error(&self) -> &super::IndexError {
            if let Some(super::execution_error::ErrorDetails::IndexError(field)) = &self
                .error_details
            {
                field as _
            } else {
                super::IndexError::default_instance() as _
            }
        }
        ///If `index_error` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn index_error_opt(&self) -> Option<&super::IndexError> {
            if let Some(super::execution_error::ErrorDetails::IndexError(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `index_error` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn index_error_opt_mut(&mut self) -> Option<&mut super::IndexError> {
            if let Some(super::execution_error::ErrorDetails::IndexError(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `index_error`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `index_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_index_error<T: Into<super::IndexError>>(&mut self, field: T) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::IndexError(field.into().into()),
            );
        }
        ///Sets `index_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_index_error<T: Into<super::IndexError>>(mut self, field: T) -> Self {
            self.set_index_error(field.into());
            self
        }
        ///Returns the value of `object_id`, or the default value if `object_id` is unset.
        pub fn object_id(&self) -> &str {
            if let Some(super::execution_error::ErrorDetails::ObjectId(field)) = &self
                .error_details
            {
                field as _
            } else {
                ""
            }
        }
        ///If `object_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_id_opt(&self) -> Option<&str> {
            if let Some(super::execution_error::ErrorDetails::ObjectId(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `object_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(super::execution_error::ErrorDetails::ObjectId(field)) = &mut self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `object_id`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn object_id_mut(&mut self) -> &mut String {
            if self.object_id_opt_mut().is_none() {
                self.error_details = Some(
                    super::execution_error::ErrorDetails::ObjectId(String::default()),
                );
            }
            self.object_id_opt_mut().unwrap()
        }
        ///Sets `object_id` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.error_details = Some(
                super::execution_error::ErrorDetails::ObjectId(field.into().into()),
            );
        }
        ///Sets `object_id` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field.into());
            self
        }
        ///Returns the value of `coin_deny_list_error`, or the default value if `coin_deny_list_error` is unset.
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
        ///If `coin_deny_list_error` is set, returns [`Some`] with the value; otherwise returns [`None`].
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
        ///If `coin_deny_list_error` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `coin_deny_list_error`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `coin_deny_list_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `coin_deny_list_error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_coin_deny_list_error<T: Into<super::CoinDenyListError>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_coin_deny_list_error(field.into());
            self
        }
        ///Returns the value of `congested_objects`, or the default value if `congested_objects` is unset.
        pub fn congested_objects(&self) -> &super::CongestedObjects {
            if let Some(super::execution_error::ErrorDetails::CongestedObjects(field)) = &self
                .error_details
            {
                field as _
            } else {
                super::CongestedObjects::default_instance() as _
            }
        }
        ///If `congested_objects` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn congested_objects_opt(&self) -> Option<&super::CongestedObjects> {
            if let Some(super::execution_error::ErrorDetails::CongestedObjects(field)) = &self
                .error_details
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `congested_objects` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `congested_objects`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `congested_objects` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `congested_objects` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_congested_objects<T: Into<super::CongestedObjects>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_congested_objects(field.into());
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
        ///If `success` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn success_opt_mut(&mut self) -> Option<&mut bool> {
            self.success.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `success`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn success_mut(&mut self) -> &mut bool {
            self.success.get_or_insert_default()
        }
        ///If `success` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn success_opt(&self) -> Option<bool> {
            self.success.as_ref().map(|field| *field)
        }
        ///Sets `success` with the provided value.
        pub fn set_success(&mut self, field: bool) {
            self.success = Some(field);
        }
        ///Sets `success` with the provided value.
        pub fn with_success(mut self, field: bool) -> Self {
            self.set_success(field);
            self
        }
        ///Returns the value of `error`, or the default value if `error` is unset.
        pub fn error(&self) -> &super::ExecutionError {
            self.error
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ExecutionError::default_instance() as _)
        }
        ///If `error` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn error_opt_mut(&mut self) -> Option<&mut super::ExecutionError> {
            self.error.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `error`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn error_mut(&mut self) -> &mut super::ExecutionError {
            self.error.get_or_insert_default()
        }
        ///If `error` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn error_opt(&self) -> Option<&super::ExecutionError> {
            self.error.as_ref().map(|field| field as _)
        }
        ///Sets `error` with the provided value.
        pub fn set_error<T: Into<super::ExecutionError>>(&mut self, field: T) {
            self.error = Some(field.into().into());
        }
        ///Sets `error` with the provided value.
        pub fn with_error<T: Into<super::ExecutionError>>(mut self, field: T) -> Self {
            self.set_error(field.into());
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
        ///Sets `kind` with the provided value.
        pub fn with_kind<
            T: Into<super::execution_time_observation::ExecutionTimeObservationKind>,
        >(mut self, field: T) -> Self {
            self.set_kind(field.into());
            self
        }
        ///Returns the value of `move_entry_point`, or the default value if `move_entry_point` is unset.
        pub fn move_entry_point(&self) -> &super::MoveCall {
            self.move_entry_point
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveCall::default_instance() as _)
        }
        ///If `move_entry_point` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn move_entry_point_opt_mut(&mut self) -> Option<&mut super::MoveCall> {
            self.move_entry_point.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `move_entry_point`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn move_entry_point_mut(&mut self) -> &mut super::MoveCall {
            self.move_entry_point.get_or_insert_default()
        }
        ///If `move_entry_point` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn move_entry_point_opt(&self) -> Option<&super::MoveCall> {
            self.move_entry_point.as_ref().map(|field| field as _)
        }
        ///Sets `move_entry_point` with the provided value.
        pub fn set_move_entry_point<T: Into<super::MoveCall>>(&mut self, field: T) {
            self.move_entry_point = Some(field.into().into());
        }
        ///Sets `move_entry_point` with the provided value.
        pub fn with_move_entry_point<T: Into<super::MoveCall>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_move_entry_point(field.into());
            self
        }
        ///Returns the value of `validator_observations`, or the default value if `validator_observations` is unset.
        pub fn validator_observations(
            &self,
        ) -> &[super::ValidatorExecutionTimeObservation] {
            &self.validator_observations
        }
        ///Returns a mutable reference to `validator_observations`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn validator_observations_mut(
            &mut self,
        ) -> &mut Vec<super::ValidatorExecutionTimeObservation> {
            &mut self.validator_observations
        }
        ///Sets `validator_observations` with the provided value.
        pub fn set_validator_observations(
            &mut self,
            field: Vec<super::ValidatorExecutionTimeObservation>,
        ) {
            self.validator_observations = field;
        }
        ///Sets `validator_observations` with the provided value.
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
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut i32> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut i32 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<i32> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: i32) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: i32) -> Self {
            self.set_version(field);
            self
        }
        ///Returns the value of `observations`, or the default value if `observations` is unset.
        pub fn observations(&self) -> &[super::ExecutionTimeObservation] {
            &self.observations
        }
        ///Returns a mutable reference to `observations`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn observations_mut(&mut self) -> &mut Vec<super::ExecutionTimeObservation> {
            &mut self.observations
        }
        ///Sets `observations` with the provided value.
        pub fn set_observations(&mut self, field: Vec<super::ExecutionTimeObservation>) {
            self.observations = field;
        }
        ///Sets `observations` with the provided value.
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
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
            self
        }
        ///If `position` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn position_opt_mut(&mut self) -> Option<&mut u32> {
            self.position.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `position`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn position_mut(&mut self) -> &mut u32 {
            self.position.get_or_insert_default()
        }
        ///If `position` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn position_opt(&self) -> Option<u32> {
            self.position.as_ref().map(|field| *field)
        }
        ///Sets `position` with the provided value.
        pub fn set_position(&mut self, field: u32) {
            self.position = Some(field);
        }
        ///Sets `position` with the provided value.
        pub fn with_position(mut self, field: u32) -> Self {
            self.set_position(field);
            self
        }
        ///Returns the value of `r#type`, or the default value if `r#type` is unset.
        pub fn r#type(&self) -> &super::OpenSignatureBody {
            self.r#type
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::OpenSignatureBody::default_instance() as _)
        }
        ///If `r#type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn type_opt_mut(&mut self) -> Option<&mut super::OpenSignatureBody> {
            self.r#type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `r#type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn type_mut(&mut self) -> &mut super::OpenSignatureBody {
            self.r#type.get_or_insert_default()
        }
        ///If `r#type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn type_opt(&self) -> Option<&super::OpenSignatureBody> {
            self.r#type.as_ref().map(|field| field as _)
        }
        ///Sets `r#type` with the provided value.
        pub fn set_type<T: Into<super::OpenSignatureBody>>(&mut self, field: T) {
            self.r#type = Some(field.into().into());
        }
        ///Sets `r#type` with the provided value.
        pub fn with_type<T: Into<super::OpenSignatureBody>>(mut self, field: T) -> Self {
            self.set_type(field.into());
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
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
            self
        }
        ///Sets `visibility` with the provided value.
        pub fn with_visibility<T: Into<super::function_descriptor::Visibility>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_visibility(field.into());
            self
        }
        ///If `is_entry` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn is_entry_opt_mut(&mut self) -> Option<&mut bool> {
            self.is_entry.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `is_entry`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn is_entry_mut(&mut self) -> &mut bool {
            self.is_entry.get_or_insert_default()
        }
        ///If `is_entry` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn is_entry_opt(&self) -> Option<bool> {
            self.is_entry.as_ref().map(|field| *field)
        }
        ///Sets `is_entry` with the provided value.
        pub fn set_is_entry(&mut self, field: bool) {
            self.is_entry = Some(field);
        }
        ///Sets `is_entry` with the provided value.
        pub fn with_is_entry(mut self, field: bool) -> Self {
            self.set_is_entry(field);
            self
        }
        ///Returns the value of `type_parameters`, or the default value if `type_parameters` is unset.
        pub fn type_parameters(&self) -> &[super::TypeParameter] {
            &self.type_parameters
        }
        ///Returns a mutable reference to `type_parameters`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn type_parameters_mut(&mut self) -> &mut Vec<super::TypeParameter> {
            &mut self.type_parameters
        }
        ///Sets `type_parameters` with the provided value.
        pub fn set_type_parameters(&mut self, field: Vec<super::TypeParameter>) {
            self.type_parameters = field;
        }
        ///Sets `type_parameters` with the provided value.
        pub fn with_type_parameters(mut self, field: Vec<super::TypeParameter>) -> Self {
            self.set_type_parameters(field);
            self
        }
        ///Returns the value of `parameters`, or the default value if `parameters` is unset.
        pub fn parameters(&self) -> &[super::OpenSignature] {
            &self.parameters
        }
        ///Returns a mutable reference to `parameters`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn parameters_mut(&mut self) -> &mut Vec<super::OpenSignature> {
            &mut self.parameters
        }
        ///Sets `parameters` with the provided value.
        pub fn set_parameters(&mut self, field: Vec<super::OpenSignature>) {
            self.parameters = field;
        }
        ///Sets `parameters` with the provided value.
        pub fn with_parameters(mut self, field: Vec<super::OpenSignature>) -> Self {
            self.set_parameters(field);
            self
        }
        ///Returns the value of `returns`, or the default value if `returns` is unset.
        pub fn returns(&self) -> &[super::OpenSignature] {
            &self.returns
        }
        ///Returns a mutable reference to `returns`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn returns_mut(&mut self) -> &mut Vec<super::OpenSignature> {
            &mut self.returns
        }
        ///Sets `returns` with the provided value.
        pub fn set_returns(&mut self, field: Vec<super::OpenSignature>) {
            self.returns = field;
        }
        ///Sets `returns` with the provided value.
        pub fn with_returns(mut self, field: Vec<super::OpenSignature>) -> Self {
            self.set_returns(field);
            self
        }
    }
    impl super::FundsWithdrawal {
        pub const fn const_default() -> Self {
            Self {
                amount: None,
                coin_type: None,
                source: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::FundsWithdrawal = super::FundsWithdrawal::const_default();
            &DEFAULT
        }
        ///If `amount` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn amount_opt_mut(&mut self) -> Option<&mut u64> {
            self.amount.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `amount`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn amount_mut(&mut self) -> &mut u64 {
            self.amount.get_or_insert_default()
        }
        ///If `amount` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn amount_opt(&self) -> Option<u64> {
            self.amount.as_ref().map(|field| *field)
        }
        ///Sets `amount` with the provided value.
        pub fn set_amount(&mut self, field: u64) {
            self.amount = Some(field);
        }
        ///Sets `amount` with the provided value.
        pub fn with_amount(mut self, field: u64) -> Self {
            self.set_amount(field);
            self
        }
        ///If `coin_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `coin_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        ///If `coin_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        ///Sets `coin_type` with the provided value.
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        ///Sets `coin_type` with the provided value.
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field.into());
            self
        }
        ///Sets `source` with the provided value.
        pub fn with_source<T: Into<super::funds_withdrawal::Source>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_source(field.into());
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
        ///If `computation_cost` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn computation_cost_opt_mut(&mut self) -> Option<&mut u64> {
            self.computation_cost.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `computation_cost`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn computation_cost_mut(&mut self) -> &mut u64 {
            self.computation_cost.get_or_insert_default()
        }
        ///If `computation_cost` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn computation_cost_opt(&self) -> Option<u64> {
            self.computation_cost.as_ref().map(|field| *field)
        }
        ///Sets `computation_cost` with the provided value.
        pub fn set_computation_cost(&mut self, field: u64) {
            self.computation_cost = Some(field);
        }
        ///Sets `computation_cost` with the provided value.
        pub fn with_computation_cost(mut self, field: u64) -> Self {
            self.set_computation_cost(field);
            self
        }
        ///If `storage_cost` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn storage_cost_opt_mut(&mut self) -> Option<&mut u64> {
            self.storage_cost.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `storage_cost`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn storage_cost_mut(&mut self) -> &mut u64 {
            self.storage_cost.get_or_insert_default()
        }
        ///If `storage_cost` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn storage_cost_opt(&self) -> Option<u64> {
            self.storage_cost.as_ref().map(|field| *field)
        }
        ///Sets `storage_cost` with the provided value.
        pub fn set_storage_cost(&mut self, field: u64) {
            self.storage_cost = Some(field);
        }
        ///Sets `storage_cost` with the provided value.
        pub fn with_storage_cost(mut self, field: u64) -> Self {
            self.set_storage_cost(field);
            self
        }
        ///If `storage_rebate` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn storage_rebate_opt_mut(&mut self) -> Option<&mut u64> {
            self.storage_rebate.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `storage_rebate`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn storage_rebate_mut(&mut self) -> &mut u64 {
            self.storage_rebate.get_or_insert_default()
        }
        ///If `storage_rebate` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn storage_rebate_opt(&self) -> Option<u64> {
            self.storage_rebate.as_ref().map(|field| *field)
        }
        ///Sets `storage_rebate` with the provided value.
        pub fn set_storage_rebate(&mut self, field: u64) {
            self.storage_rebate = Some(field);
        }
        ///Sets `storage_rebate` with the provided value.
        pub fn with_storage_rebate(mut self, field: u64) -> Self {
            self.set_storage_rebate(field);
            self
        }
        ///If `non_refundable_storage_fee` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn non_refundable_storage_fee_opt_mut(&mut self) -> Option<&mut u64> {
            self.non_refundable_storage_fee.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `non_refundable_storage_fee`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn non_refundable_storage_fee_mut(&mut self) -> &mut u64 {
            self.non_refundable_storage_fee.get_or_insert_default()
        }
        ///If `non_refundable_storage_fee` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn non_refundable_storage_fee_opt(&self) -> Option<u64> {
            self.non_refundable_storage_fee.as_ref().map(|field| *field)
        }
        ///Sets `non_refundable_storage_fee` with the provided value.
        pub fn set_non_refundable_storage_fee(&mut self, field: u64) {
            self.non_refundable_storage_fee = Some(field);
        }
        ///Sets `non_refundable_storage_fee` with the provided value.
        pub fn with_non_refundable_storage_fee(mut self, field: u64) -> Self {
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
        ///Returns the value of `objects`, or the default value if `objects` is unset.
        pub fn objects(&self) -> &[super::ObjectReference] {
            &self.objects
        }
        ///Returns a mutable reference to `objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn objects_mut(&mut self) -> &mut Vec<super::ObjectReference> {
            &mut self.objects
        }
        ///Sets `objects` with the provided value.
        pub fn set_objects(&mut self, field: Vec<super::ObjectReference>) {
            self.objects = field;
        }
        ///Sets `objects` with the provided value.
        pub fn with_objects(mut self, field: Vec<super::ObjectReference>) -> Self {
            self.set_objects(field);
            self
        }
        ///If `owner` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn owner_opt_mut(&mut self) -> Option<&mut String> {
            self.owner.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `owner`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn owner_mut(&mut self) -> &mut String {
            self.owner.get_or_insert_default()
        }
        ///If `owner` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn owner_opt(&self) -> Option<&str> {
            self.owner.as_ref().map(|field| field as _)
        }
        ///Sets `owner` with the provided value.
        pub fn set_owner<T: Into<String>>(&mut self, field: T) {
            self.owner = Some(field.into().into());
        }
        ///Sets `owner` with the provided value.
        pub fn with_owner<T: Into<String>>(mut self, field: T) -> Self {
            self.set_owner(field.into());
            self
        }
        ///If `price` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn price_opt_mut(&mut self) -> Option<&mut u64> {
            self.price.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `price`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn price_mut(&mut self) -> &mut u64 {
            self.price.get_or_insert_default()
        }
        ///If `price` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn price_opt(&self) -> Option<u64> {
            self.price.as_ref().map(|field| *field)
        }
        ///Sets `price` with the provided value.
        pub fn set_price(&mut self, field: u64) {
            self.price = Some(field);
        }
        ///Sets `price` with the provided value.
        pub fn with_price(mut self, field: u64) -> Self {
            self.set_price(field);
            self
        }
        ///If `budget` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn budget_opt_mut(&mut self) -> Option<&mut u64> {
            self.budget.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `budget`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn budget_mut(&mut self) -> &mut u64 {
            self.budget.get_or_insert_default()
        }
        ///If `budget` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn budget_opt(&self) -> Option<u64> {
            self.budget.as_ref().map(|field| *field)
        }
        ///Sets `budget` with the provided value.
        pub fn set_budget(&mut self, field: u64) {
            self.budget = Some(field);
        }
        ///Sets `budget` with the provided value.
        pub fn with_budget(mut self, field: u64) -> Self {
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
        ///Returns the value of `objects`, or the default value if `objects` is unset.
        pub fn objects(&self) -> &[super::Object] {
            &self.objects
        }
        ///Returns a mutable reference to `objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn objects_mut(&mut self) -> &mut Vec<super::Object> {
            &mut self.objects
        }
        ///Sets `objects` with the provided value.
        pub fn set_objects(&mut self, field: Vec<super::Object>) {
            self.objects = field;
        }
        ///Sets `objects` with the provided value.
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
        ///If `owner` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn owner_opt_mut(&mut self) -> Option<&mut String> {
            self.owner.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `owner`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn owner_mut(&mut self) -> &mut String {
            self.owner.get_or_insert_default()
        }
        ///If `owner` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn owner_opt(&self) -> Option<&str> {
            self.owner.as_ref().map(|field| field as _)
        }
        ///Sets `owner` with the provided value.
        pub fn set_owner<T: Into<String>>(&mut self, field: T) {
            self.owner = Some(field.into().into());
        }
        ///Sets `owner` with the provided value.
        pub fn with_owner<T: Into<String>>(mut self, field: T) -> Self {
            self.set_owner(field.into());
            self
        }
        ///If `coin_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `coin_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        ///If `coin_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        ///Sets `coin_type` with the provided value.
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        ///Sets `coin_type` with the provided value.
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field.into());
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
        ///Returns the value of `balance`, or the default value if `balance` is unset.
        pub fn balance(&self) -> &super::Balance {
            self.balance
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Balance::default_instance() as _)
        }
        ///If `balance` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn balance_opt_mut(&mut self) -> Option<&mut super::Balance> {
            self.balance.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `balance`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn balance_mut(&mut self) -> &mut super::Balance {
            self.balance.get_or_insert_default()
        }
        ///If `balance` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn balance_opt(&self) -> Option<&super::Balance> {
            self.balance.as_ref().map(|field| field as _)
        }
        ///Sets `balance` with the provided value.
        pub fn set_balance<T: Into<super::Balance>>(&mut self, field: T) {
            self.balance = Some(field.into().into());
        }
        ///Sets `balance` with the provided value.
        pub fn with_balance<T: Into<super::Balance>>(mut self, field: T) -> Self {
            self.set_balance(field.into());
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
        ///If `read_mask` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `read_mask`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        ///If `read_mask` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        ///Sets `read_mask` with the provided value.
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        ///Sets `read_mask` with the provided value.
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field.into());
            self
        }
        ///Returns the value of `sequence_number`, or the default value if `sequence_number` is unset.
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
        ///If `sequence_number` is set, returns [`Some`] with the value; otherwise returns [`None`].
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
        ///If `sequence_number` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `sequence_number`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `sequence_number` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_sequence_number(&mut self, field: u64) {
            self.checkpoint_id = Some(
                super::get_checkpoint_request::CheckpointId::SequenceNumber(field),
            );
        }
        ///Sets `sequence_number` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_sequence_number(mut self, field: u64) -> Self {
            self.set_sequence_number(field);
            self
        }
        ///Returns the value of `digest`, or the default value if `digest` is unset.
        pub fn digest(&self) -> &str {
            if let Some(super::get_checkpoint_request::CheckpointId::Digest(field)) = &self
                .checkpoint_id
            {
                field as _
            } else {
                ""
            }
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            if let Some(super::get_checkpoint_request::CheckpointId::Digest(field)) = &self
                .checkpoint_id
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            if let Some(super::get_checkpoint_request::CheckpointId::Digest(field)) = &mut self
                .checkpoint_id
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `digest` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.checkpoint_id = Some(
                super::get_checkpoint_request::CheckpointId::Digest(field.into().into()),
            );
        }
        ///Sets `digest` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
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
        ///Returns the value of `checkpoint`, or the default value if `checkpoint` is unset.
        pub fn checkpoint(&self) -> &super::Checkpoint {
            self.checkpoint
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Checkpoint::default_instance() as _)
        }
        ///If `checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn checkpoint_opt_mut(&mut self) -> Option<&mut super::Checkpoint> {
            self.checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn checkpoint_mut(&mut self) -> &mut super::Checkpoint {
            self.checkpoint.get_or_insert_default()
        }
        ///If `checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn checkpoint_opt(&self) -> Option<&super::Checkpoint> {
            self.checkpoint.as_ref().map(|field| field as _)
        }
        ///Sets `checkpoint` with the provided value.
        pub fn set_checkpoint<T: Into<super::Checkpoint>>(&mut self, field: T) {
            self.checkpoint = Some(field.into().into());
        }
        ///Sets `checkpoint` with the provided value.
        pub fn with_checkpoint<T: Into<super::Checkpoint>>(mut self, field: T) -> Self {
            self.set_checkpoint(field.into());
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
        ///If `coin_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `coin_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        ///If `coin_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        ///Sets `coin_type` with the provided value.
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        ///Sets `coin_type` with the provided value.
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field.into());
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
        ///If `coin_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn coin_type_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `coin_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coin_type_mut(&mut self) -> &mut String {
            self.coin_type.get_or_insert_default()
        }
        ///If `coin_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn coin_type_opt(&self) -> Option<&str> {
            self.coin_type.as_ref().map(|field| field as _)
        }
        ///Sets `coin_type` with the provided value.
        pub fn set_coin_type<T: Into<String>>(&mut self, field: T) {
            self.coin_type = Some(field.into().into());
        }
        ///Sets `coin_type` with the provided value.
        pub fn with_coin_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_type(field.into());
            self
        }
        ///Returns the value of `metadata`, or the default value if `metadata` is unset.
        pub fn metadata(&self) -> &super::CoinMetadata {
            self.metadata
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CoinMetadata::default_instance() as _)
        }
        ///If `metadata` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn metadata_opt_mut(&mut self) -> Option<&mut super::CoinMetadata> {
            self.metadata.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `metadata`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn metadata_mut(&mut self) -> &mut super::CoinMetadata {
            self.metadata.get_or_insert_default()
        }
        ///If `metadata` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn metadata_opt(&self) -> Option<&super::CoinMetadata> {
            self.metadata.as_ref().map(|field| field as _)
        }
        ///Sets `metadata` with the provided value.
        pub fn set_metadata<T: Into<super::CoinMetadata>>(&mut self, field: T) {
            self.metadata = Some(field.into().into());
        }
        ///Sets `metadata` with the provided value.
        pub fn with_metadata<T: Into<super::CoinMetadata>>(mut self, field: T) -> Self {
            self.set_metadata(field.into());
            self
        }
        ///Returns the value of `treasury`, or the default value if `treasury` is unset.
        pub fn treasury(&self) -> &super::CoinTreasury {
            self.treasury
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CoinTreasury::default_instance() as _)
        }
        ///If `treasury` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn treasury_opt_mut(&mut self) -> Option<&mut super::CoinTreasury> {
            self.treasury.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `treasury`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn treasury_mut(&mut self) -> &mut super::CoinTreasury {
            self.treasury.get_or_insert_default()
        }
        ///If `treasury` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn treasury_opt(&self) -> Option<&super::CoinTreasury> {
            self.treasury.as_ref().map(|field| field as _)
        }
        ///Sets `treasury` with the provided value.
        pub fn set_treasury<T: Into<super::CoinTreasury>>(&mut self, field: T) {
            self.treasury = Some(field.into().into());
        }
        ///Sets `treasury` with the provided value.
        pub fn with_treasury<T: Into<super::CoinTreasury>>(mut self, field: T) -> Self {
            self.set_treasury(field.into());
            self
        }
        ///Returns the value of `regulated_metadata`, or the default value if `regulated_metadata` is unset.
        pub fn regulated_metadata(&self) -> &super::RegulatedCoinMetadata {
            self.regulated_metadata
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::RegulatedCoinMetadata::default_instance() as _)
        }
        ///If `regulated_metadata` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn regulated_metadata_opt_mut(
            &mut self,
        ) -> Option<&mut super::RegulatedCoinMetadata> {
            self.regulated_metadata.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `regulated_metadata`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn regulated_metadata_mut(&mut self) -> &mut super::RegulatedCoinMetadata {
            self.regulated_metadata.get_or_insert_default()
        }
        ///If `regulated_metadata` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn regulated_metadata_opt(&self) -> Option<&super::RegulatedCoinMetadata> {
            self.regulated_metadata.as_ref().map(|field| field as _)
        }
        ///Sets `regulated_metadata` with the provided value.
        pub fn set_regulated_metadata<T: Into<super::RegulatedCoinMetadata>>(
            &mut self,
            field: T,
        ) {
            self.regulated_metadata = Some(field.into().into());
        }
        ///Sets `regulated_metadata` with the provided value.
        pub fn with_regulated_metadata<T: Into<super::RegulatedCoinMetadata>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_regulated_metadata(field.into());
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
        ///If `package_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        ///If `package_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        ///Sets `package_id` with the provided value.
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        ///Sets `package_id` with the provided value.
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field.into());
            self
        }
        ///If `module_name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn module_name_opt_mut(&mut self) -> Option<&mut String> {
            self.module_name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `module_name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn module_name_mut(&mut self) -> &mut String {
            self.module_name.get_or_insert_default()
        }
        ///If `module_name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn module_name_opt(&self) -> Option<&str> {
            self.module_name.as_ref().map(|field| field as _)
        }
        ///Sets `module_name` with the provided value.
        pub fn set_module_name<T: Into<String>>(&mut self, field: T) {
            self.module_name = Some(field.into().into());
        }
        ///Sets `module_name` with the provided value.
        pub fn with_module_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module_name(field.into());
            self
        }
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
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
        ///Returns the value of `datatype`, or the default value if `datatype` is unset.
        pub fn datatype(&self) -> &super::DatatypeDescriptor {
            self.datatype
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::DatatypeDescriptor::default_instance() as _)
        }
        ///If `datatype` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn datatype_opt_mut(&mut self) -> Option<&mut super::DatatypeDescriptor> {
            self.datatype.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `datatype`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn datatype_mut(&mut self) -> &mut super::DatatypeDescriptor {
            self.datatype.get_or_insert_default()
        }
        ///If `datatype` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn datatype_opt(&self) -> Option<&super::DatatypeDescriptor> {
            self.datatype.as_ref().map(|field| field as _)
        }
        ///Sets `datatype` with the provided value.
        pub fn set_datatype<T: Into<super::DatatypeDescriptor>>(&mut self, field: T) {
            self.datatype = Some(field.into().into());
        }
        ///Sets `datatype` with the provided value.
        pub fn with_datatype<T: Into<super::DatatypeDescriptor>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_datatype(field.into());
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
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///If `read_mask` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `read_mask`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        ///If `read_mask` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        ///Sets `read_mask` with the provided value.
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        ///Sets `read_mask` with the provided value.
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field.into());
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
        ///Returns the value of `epoch`, or the default value if `epoch` is unset.
        pub fn epoch(&self) -> &super::Epoch {
            self.epoch
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Epoch::default_instance() as _)
        }
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut super::Epoch> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut super::Epoch {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<&super::Epoch> {
            self.epoch.as_ref().map(|field| field as _)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch<T: Into<super::Epoch>>(&mut self, field: T) {
            self.epoch = Some(field.into().into());
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch<T: Into<super::Epoch>>(mut self, field: T) -> Self {
            self.set_epoch(field.into());
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
        ///If `package_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        ///If `package_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        ///Sets `package_id` with the provided value.
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        ///Sets `package_id` with the provided value.
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field.into());
            self
        }
        ///If `module_name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn module_name_opt_mut(&mut self) -> Option<&mut String> {
            self.module_name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `module_name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn module_name_mut(&mut self) -> &mut String {
            self.module_name.get_or_insert_default()
        }
        ///If `module_name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn module_name_opt(&self) -> Option<&str> {
            self.module_name.as_ref().map(|field| field as _)
        }
        ///Sets `module_name` with the provided value.
        pub fn set_module_name<T: Into<String>>(&mut self, field: T) {
            self.module_name = Some(field.into().into());
        }
        ///Sets `module_name` with the provided value.
        pub fn with_module_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module_name(field.into());
            self
        }
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
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
        ///Returns the value of `function`, or the default value if `function` is unset.
        pub fn function(&self) -> &super::FunctionDescriptor {
            self.function
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::FunctionDescriptor::default_instance() as _)
        }
        ///If `function` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn function_opt_mut(&mut self) -> Option<&mut super::FunctionDescriptor> {
            self.function.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `function`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn function_mut(&mut self) -> &mut super::FunctionDescriptor {
            self.function.get_or_insert_default()
        }
        ///If `function` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn function_opt(&self) -> Option<&super::FunctionDescriptor> {
            self.function.as_ref().map(|field| field as _)
        }
        ///Sets `function` with the provided value.
        pub fn set_function<T: Into<super::FunctionDescriptor>>(&mut self, field: T) {
            self.function = Some(field.into().into());
        }
        ///Sets `function` with the provided value.
        pub fn with_function<T: Into<super::FunctionDescriptor>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_function(field.into());
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
        ///If `object_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        ///If `object_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        ///Sets `object_id` with the provided value.
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        ///Sets `object_id` with the provided value.
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field.into());
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
            self.set_version(field);
            self
        }
        ///If `read_mask` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `read_mask`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        ///If `read_mask` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        ///Sets `read_mask` with the provided value.
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        ///Sets `read_mask` with the provided value.
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field.into());
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
        ///Returns the value of `object`, or the default value if `object` is unset.
        pub fn object(&self) -> &super::Object {
            self.object
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Object::default_instance() as _)
        }
        ///If `object` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_opt_mut(&mut self) -> Option<&mut super::Object> {
            self.object.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_mut(&mut self) -> &mut super::Object {
            self.object.get_or_insert_default()
        }
        ///If `object` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_opt(&self) -> Option<&super::Object> {
            self.object.as_ref().map(|field| field as _)
        }
        ///Sets `object` with the provided value.
        pub fn set_object<T: Into<super::Object>>(&mut self, field: T) {
            self.object = Some(field.into().into());
        }
        ///Sets `object` with the provided value.
        pub fn with_object<T: Into<super::Object>>(mut self, field: T) -> Self {
            self.set_object(field.into());
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
        ///Returns the value of `object`, or the default value if `object` is unset.
        pub fn object(&self) -> &super::Object {
            if let Some(super::get_object_result::Result::Object(field)) = &self.result {
                field as _
            } else {
                super::Object::default_instance() as _
            }
        }
        ///If `object` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_opt(&self) -> Option<&super::Object> {
            if let Some(super::get_object_result::Result::Object(field)) = &self.result {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `object` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_opt_mut(&mut self) -> Option<&mut super::Object> {
            if let Some(super::get_object_result::Result::Object(field)) = &mut self
                .result
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `object`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn object_mut(&mut self) -> &mut super::Object {
            if self.object_opt_mut().is_none() {
                self.result = Some(
                    super::get_object_result::Result::Object(super::Object::default()),
                );
            }
            self.object_opt_mut().unwrap()
        }
        ///Sets `object` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_object<T: Into<super::Object>>(&mut self, field: T) {
            self.result = Some(
                super::get_object_result::Result::Object(field.into().into()),
            );
        }
        ///Sets `object` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_object<T: Into<super::Object>>(mut self, field: T) -> Self {
            self.set_object(field.into());
            self
        }
        ///Returns the value of `error`, or the default value if `error` is unset.
        pub fn error(&self) -> &super::super::super::super::google::rpc::Status {
            if let Some(super::get_object_result::Result::Error(field)) = &self.result {
                field as _
            } else {
                super::super::super::super::google::rpc::Status::default_instance() as _
            }
        }
        ///If `error` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn error_opt(
            &self,
        ) -> Option<&super::super::super::super::google::rpc::Status> {
            if let Some(super::get_object_result::Result::Error(field)) = &self.result {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `error` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `error`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_error<T: Into<super::super::super::super::google::rpc::Status>>(
            &mut self,
            field: T,
        ) {
            self.result = Some(
                super::get_object_result::Result::Error(field.into().into()),
            );
        }
        ///Sets `error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_error<T: Into<super::super::super::super::google::rpc::Status>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_error(field.into());
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
        ///If `package_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        ///If `package_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        ///Sets `package_id` with the provided value.
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        ///Sets `package_id` with the provided value.
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field.into());
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
        ///Returns the value of `package`, or the default value if `package` is unset.
        pub fn package(&self) -> &super::Package {
            self.package
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Package::default_instance() as _)
        }
        ///If `package` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_opt_mut(&mut self) -> Option<&mut super::Package> {
            self.package.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_mut(&mut self) -> &mut super::Package {
            self.package.get_or_insert_default()
        }
        ///If `package` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_opt(&self) -> Option<&super::Package> {
            self.package.as_ref().map(|field| field as _)
        }
        ///Sets `package` with the provided value.
        pub fn set_package<T: Into<super::Package>>(&mut self, field: T) {
            self.package = Some(field.into().into());
        }
        ///Sets `package` with the provided value.
        pub fn with_package<T: Into<super::Package>>(mut self, field: T) -> Self {
            self.set_package(field.into());
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
        ///If `chain_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn chain_id_opt_mut(&mut self) -> Option<&mut String> {
            self.chain_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `chain_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn chain_id_mut(&mut self) -> &mut String {
            self.chain_id.get_or_insert_default()
        }
        ///If `chain_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn chain_id_opt(&self) -> Option<&str> {
            self.chain_id.as_ref().map(|field| field as _)
        }
        ///Sets `chain_id` with the provided value.
        pub fn set_chain_id<T: Into<String>>(&mut self, field: T) {
            self.chain_id = Some(field.into().into());
        }
        ///Sets `chain_id` with the provided value.
        pub fn with_chain_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_chain_id(field.into());
            self
        }
        ///If `chain` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn chain_opt_mut(&mut self) -> Option<&mut String> {
            self.chain.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `chain`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn chain_mut(&mut self) -> &mut String {
            self.chain.get_or_insert_default()
        }
        ///If `chain` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn chain_opt(&self) -> Option<&str> {
            self.chain.as_ref().map(|field| field as _)
        }
        ///Sets `chain` with the provided value.
        pub fn set_chain<T: Into<String>>(&mut self, field: T) {
            self.chain = Some(field.into().into());
        }
        ///Sets `chain` with the provided value.
        pub fn with_chain<T: Into<String>>(mut self, field: T) -> Self {
            self.set_chain(field.into());
            self
        }
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///If `checkpoint_height` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn checkpoint_height_opt_mut(&mut self) -> Option<&mut u64> {
            self.checkpoint_height.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `checkpoint_height`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn checkpoint_height_mut(&mut self) -> &mut u64 {
            self.checkpoint_height.get_or_insert_default()
        }
        ///If `checkpoint_height` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn checkpoint_height_opt(&self) -> Option<u64> {
            self.checkpoint_height.as_ref().map(|field| *field)
        }
        ///Sets `checkpoint_height` with the provided value.
        pub fn set_checkpoint_height(&mut self, field: u64) {
            self.checkpoint_height = Some(field);
        }
        ///Sets `checkpoint_height` with the provided value.
        pub fn with_checkpoint_height(mut self, field: u64) -> Self {
            self.set_checkpoint_height(field);
            self
        }
        ///If `timestamp` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn timestamp_opt_mut(&mut self) -> Option<&mut ::prost_types::Timestamp> {
            self.timestamp.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `timestamp`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.timestamp.get_or_insert_default()
        }
        ///If `timestamp` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.timestamp.as_ref().map(|field| field as _)
        }
        ///Sets `timestamp` with the provided value.
        pub fn set_timestamp<T: Into<::prost_types::Timestamp>>(&mut self, field: T) {
            self.timestamp = Some(field.into().into());
        }
        ///Sets `timestamp` with the provided value.
        pub fn with_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_timestamp(field.into());
            self
        }
        ///If `lowest_available_checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn lowest_available_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.lowest_available_checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `lowest_available_checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn lowest_available_checkpoint_mut(&mut self) -> &mut u64 {
            self.lowest_available_checkpoint.get_or_insert_default()
        }
        ///If `lowest_available_checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn lowest_available_checkpoint_opt(&self) -> Option<u64> {
            self.lowest_available_checkpoint.as_ref().map(|field| *field)
        }
        ///Sets `lowest_available_checkpoint` with the provided value.
        pub fn set_lowest_available_checkpoint(&mut self, field: u64) {
            self.lowest_available_checkpoint = Some(field);
        }
        ///Sets `lowest_available_checkpoint` with the provided value.
        pub fn with_lowest_available_checkpoint(mut self, field: u64) -> Self {
            self.set_lowest_available_checkpoint(field);
            self
        }
        ///If `lowest_available_checkpoint_objects` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn lowest_available_checkpoint_objects_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.lowest_available_checkpoint_objects.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `lowest_available_checkpoint_objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn lowest_available_checkpoint_objects_mut(&mut self) -> &mut u64 {
            self.lowest_available_checkpoint_objects.get_or_insert_default()
        }
        ///If `lowest_available_checkpoint_objects` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn lowest_available_checkpoint_objects_opt(&self) -> Option<u64> {
            self.lowest_available_checkpoint_objects.as_ref().map(|field| *field)
        }
        ///Sets `lowest_available_checkpoint_objects` with the provided value.
        pub fn set_lowest_available_checkpoint_objects(&mut self, field: u64) {
            self.lowest_available_checkpoint_objects = Some(field);
        }
        ///Sets `lowest_available_checkpoint_objects` with the provided value.
        pub fn with_lowest_available_checkpoint_objects(mut self, field: u64) -> Self {
            self.set_lowest_available_checkpoint_objects(field);
            self
        }
        ///If `server` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn server_opt_mut(&mut self) -> Option<&mut String> {
            self.server.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `server`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn server_mut(&mut self) -> &mut String {
            self.server.get_or_insert_default()
        }
        ///If `server` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn server_opt(&self) -> Option<&str> {
            self.server.as_ref().map(|field| field as _)
        }
        ///Sets `server` with the provided value.
        pub fn set_server<T: Into<String>>(&mut self, field: T) {
            self.server = Some(field.into().into());
        }
        ///Sets `server` with the provided value.
        pub fn with_server<T: Into<String>>(mut self, field: T) -> Self {
            self.set_server(field.into());
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
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///If `read_mask` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `read_mask`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        ///If `read_mask` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        ///Sets `read_mask` with the provided value.
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        ///Sets `read_mask` with the provided value.
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field.into());
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
        ///Returns the value of `transaction`, or the default value if `transaction` is unset.
        pub fn transaction(&self) -> &super::ExecutedTransaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ExecutedTransaction::default_instance() as _)
        }
        ///If `transaction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn transaction_opt_mut(
            &mut self,
        ) -> Option<&mut super::ExecutedTransaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `transaction`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transaction_mut(&mut self) -> &mut super::ExecutedTransaction {
            self.transaction.get_or_insert_default()
        }
        ///If `transaction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn transaction_opt(&self) -> Option<&super::ExecutedTransaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        ///Sets `transaction` with the provided value.
        pub fn set_transaction<T: Into<super::ExecutedTransaction>>(
            &mut self,
            field: T,
        ) {
            self.transaction = Some(field.into().into());
        }
        ///Sets `transaction` with the provided value.
        pub fn with_transaction<T: Into<super::ExecutedTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field.into());
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
        ///Returns the value of `transaction`, or the default value if `transaction` is unset.
        pub fn transaction(&self) -> &super::ExecutedTransaction {
            if let Some(super::get_transaction_result::Result::Transaction(field)) = &self
                .result
            {
                field as _
            } else {
                super::ExecutedTransaction::default_instance() as _
            }
        }
        ///If `transaction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn transaction_opt(&self) -> Option<&super::ExecutedTransaction> {
            if let Some(super::get_transaction_result::Result::Transaction(field)) = &self
                .result
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `transaction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `transaction`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `transaction` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_transaction<T: Into<super::ExecutedTransaction>>(
            &mut self,
            field: T,
        ) {
            self.result = Some(
                super::get_transaction_result::Result::Transaction(field.into().into()),
            );
        }
        ///Sets `transaction` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_transaction<T: Into<super::ExecutedTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field.into());
            self
        }
        ///Returns the value of `error`, or the default value if `error` is unset.
        pub fn error(&self) -> &super::super::super::super::google::rpc::Status {
            if let Some(super::get_transaction_result::Result::Error(field)) = &self
                .result
            {
                field as _
            } else {
                super::super::super::super::google::rpc::Status::default_instance() as _
            }
        }
        ///If `error` is set, returns [`Some`] with the value; otherwise returns [`None`].
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
        ///If `error` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `error`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_error<T: Into<super::super::super::super::google::rpc::Status>>(
            &mut self,
            field: T,
        ) {
            self.result = Some(
                super::get_transaction_result::Result::Error(field.into().into()),
            );
        }
        ///Sets `error` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_error<T: Into<super::super::super::super::google::rpc::Status>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_error(field.into());
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
        ///If `index` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn index_opt_mut(&mut self) -> Option<&mut u32> {
            self.index.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `index`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn index_mut(&mut self) -> &mut u32 {
            self.index.get_or_insert_default()
        }
        ///If `index` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn index_opt(&self) -> Option<u32> {
            self.index.as_ref().map(|field| *field)
        }
        ///Sets `index` with the provided value.
        pub fn set_index(&mut self, field: u32) {
            self.index = Some(field);
        }
        ///Sets `index` with the provided value.
        pub fn with_index(mut self, field: u32) -> Self {
            self.set_index(field);
            self
        }
        ///If `subresult` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn subresult_opt_mut(&mut self) -> Option<&mut u32> {
            self.subresult.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `subresult`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn subresult_mut(&mut self) -> &mut u32 {
            self.subresult.get_or_insert_default()
        }
        ///If `subresult` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn subresult_opt(&self) -> Option<u32> {
            self.subresult.as_ref().map(|field| *field)
        }
        ///Sets `subresult` with the provided value.
        pub fn set_subresult(&mut self, field: u32) {
            self.subresult = Some(field);
        }
        ///Sets `subresult` with the provided value.
        pub fn with_subresult(mut self, field: u32) -> Self {
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
                mutability: None,
                funds_withdrawal: None,
                literal: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Input = super::Input::const_default();
            &DEFAULT
        }
        ///Sets `kind` with the provided value.
        pub fn with_kind<T: Into<super::input::InputKind>>(mut self, field: T) -> Self {
            self.set_kind(field.into());
            self
        }
        ///If `pure` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn pure_opt(&self) -> Option<&[u8]> {
            self.pure.as_ref().map(|field| field as _)
        }
        ///Sets `pure` with the provided value.
        pub fn set_pure<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.pure = Some(field.into().into());
        }
        ///Sets `pure` with the provided value.
        pub fn with_pure<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_pure(field.into());
            self
        }
        ///If `object_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        ///If `object_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        ///Sets `object_id` with the provided value.
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        ///Sets `object_id` with the provided value.
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field.into());
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
            self.set_version(field);
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///If `mutable` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn mutable_opt_mut(&mut self) -> Option<&mut bool> {
            self.mutable.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `mutable`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn mutable_mut(&mut self) -> &mut bool {
            self.mutable.get_or_insert_default()
        }
        ///If `mutable` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn mutable_opt(&self) -> Option<bool> {
            self.mutable.as_ref().map(|field| *field)
        }
        ///Sets `mutable` with the provided value.
        pub fn set_mutable(&mut self, field: bool) {
            self.mutable = Some(field);
        }
        ///Sets `mutable` with the provided value.
        pub fn with_mutable(mut self, field: bool) -> Self {
            self.set_mutable(field);
            self
        }
        ///Sets `mutability` with the provided value.
        pub fn with_mutability<T: Into<super::input::Mutability>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_mutability(field.into());
            self
        }
        ///Returns the value of `funds_withdrawal`, or the default value if `funds_withdrawal` is unset.
        pub fn funds_withdrawal(&self) -> &super::FundsWithdrawal {
            self.funds_withdrawal
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::FundsWithdrawal::default_instance() as _)
        }
        ///If `funds_withdrawal` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn funds_withdrawal_opt_mut(
            &mut self,
        ) -> Option<&mut super::FundsWithdrawal> {
            self.funds_withdrawal.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `funds_withdrawal`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn funds_withdrawal_mut(&mut self) -> &mut super::FundsWithdrawal {
            self.funds_withdrawal.get_or_insert_default()
        }
        ///If `funds_withdrawal` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn funds_withdrawal_opt(&self) -> Option<&super::FundsWithdrawal> {
            self.funds_withdrawal.as_ref().map(|field| field as _)
        }
        ///Sets `funds_withdrawal` with the provided value.
        pub fn set_funds_withdrawal<T: Into<super::FundsWithdrawal>>(
            &mut self,
            field: T,
        ) {
            self.funds_withdrawal = Some(field.into().into());
        }
        ///Sets `funds_withdrawal` with the provided value.
        pub fn with_funds_withdrawal<T: Into<super::FundsWithdrawal>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_funds_withdrawal(field.into());
            self
        }
        ///If `literal` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn literal_opt_mut(&mut self) -> Option<&mut ::prost_types::Value> {
            self.literal.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `literal`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn literal_mut(&mut self) -> &mut ::prost_types::Value {
            self.literal.get_or_insert_default()
        }
        ///If `literal` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn literal_opt(&self) -> Option<&::prost_types::Value> {
            self.literal.as_ref().map(|field| field as _)
        }
        ///Sets `literal` with the provided value.
        pub fn set_literal<T: Into<::prost_types::Value>>(&mut self, field: T) {
            self.literal = Some(field.into().into());
        }
        ///Sets `literal` with the provided value.
        pub fn with_literal<T: Into<::prost_types::Value>>(mut self, field: T) -> Self {
            self.set_literal(field.into());
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
        ///If `kty` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn kty_opt_mut(&mut self) -> Option<&mut String> {
            self.kty.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `kty`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn kty_mut(&mut self) -> &mut String {
            self.kty.get_or_insert_default()
        }
        ///If `kty` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn kty_opt(&self) -> Option<&str> {
            self.kty.as_ref().map(|field| field as _)
        }
        ///Sets `kty` with the provided value.
        pub fn set_kty<T: Into<String>>(&mut self, field: T) {
            self.kty = Some(field.into().into());
        }
        ///Sets `kty` with the provided value.
        pub fn with_kty<T: Into<String>>(mut self, field: T) -> Self {
            self.set_kty(field.into());
            self
        }
        ///If `e` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn e_opt_mut(&mut self) -> Option<&mut String> {
            self.e.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `e`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn e_mut(&mut self) -> &mut String {
            self.e.get_or_insert_default()
        }
        ///If `e` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn e_opt(&self) -> Option<&str> {
            self.e.as_ref().map(|field| field as _)
        }
        ///Sets `e` with the provided value.
        pub fn set_e<T: Into<String>>(&mut self, field: T) {
            self.e = Some(field.into().into());
        }
        ///Sets `e` with the provided value.
        pub fn with_e<T: Into<String>>(mut self, field: T) -> Self {
            self.set_e(field.into());
            self
        }
        ///If `n` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn n_opt_mut(&mut self) -> Option<&mut String> {
            self.n.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `n`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn n_mut(&mut self) -> &mut String {
            self.n.get_or_insert_default()
        }
        ///If `n` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn n_opt(&self) -> Option<&str> {
            self.n.as_ref().map(|field| field as _)
        }
        ///Sets `n` with the provided value.
        pub fn set_n<T: Into<String>>(&mut self, field: T) {
            self.n = Some(field.into().into());
        }
        ///Sets `n` with the provided value.
        pub fn with_n<T: Into<String>>(mut self, field: T) -> Self {
            self.set_n(field.into());
            self
        }
        ///If `alg` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn alg_opt_mut(&mut self) -> Option<&mut String> {
            self.alg.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `alg`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn alg_mut(&mut self) -> &mut String {
            self.alg.get_or_insert_default()
        }
        ///If `alg` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn alg_opt(&self) -> Option<&str> {
            self.alg.as_ref().map(|field| field as _)
        }
        ///Sets `alg` with the provided value.
        pub fn set_alg<T: Into<String>>(&mut self, field: T) {
            self.alg = Some(field.into().into());
        }
        ///Sets `alg` with the provided value.
        pub fn with_alg<T: Into<String>>(mut self, field: T) -> Self {
            self.set_alg(field.into());
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
        ///If `iss` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn iss_opt_mut(&mut self) -> Option<&mut String> {
            self.iss.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `iss`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn iss_mut(&mut self) -> &mut String {
            self.iss.get_or_insert_default()
        }
        ///If `iss` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn iss_opt(&self) -> Option<&str> {
            self.iss.as_ref().map(|field| field as _)
        }
        ///Sets `iss` with the provided value.
        pub fn set_iss<T: Into<String>>(&mut self, field: T) {
            self.iss = Some(field.into().into());
        }
        ///Sets `iss` with the provided value.
        pub fn with_iss<T: Into<String>>(mut self, field: T) -> Self {
            self.set_iss(field.into());
            self
        }
        ///If `kid` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn kid_opt_mut(&mut self) -> Option<&mut String> {
            self.kid.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `kid`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn kid_mut(&mut self) -> &mut String {
            self.kid.get_or_insert_default()
        }
        ///If `kid` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn kid_opt(&self) -> Option<&str> {
            self.kid.as_ref().map(|field| field as _)
        }
        ///Sets `kid` with the provided value.
        pub fn set_kid<T: Into<String>>(&mut self, field: T) {
            self.kid = Some(field.into().into());
        }
        ///Sets `kid` with the provided value.
        pub fn with_kid<T: Into<String>>(mut self, field: T) -> Self {
            self.set_kid(field.into());
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
        ///If `original_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn original_id_opt_mut(&mut self) -> Option<&mut String> {
            self.original_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `original_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn original_id_mut(&mut self) -> &mut String {
            self.original_id.get_or_insert_default()
        }
        ///If `original_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn original_id_opt(&self) -> Option<&str> {
            self.original_id.as_ref().map(|field| field as _)
        }
        ///Sets `original_id` with the provided value.
        pub fn set_original_id<T: Into<String>>(&mut self, field: T) {
            self.original_id = Some(field.into().into());
        }
        ///Sets `original_id` with the provided value.
        pub fn with_original_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_original_id(field.into());
            self
        }
        ///If `upgraded_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn upgraded_id_opt_mut(&mut self) -> Option<&mut String> {
            self.upgraded_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `upgraded_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn upgraded_id_mut(&mut self) -> &mut String {
            self.upgraded_id.get_or_insert_default()
        }
        ///If `upgraded_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn upgraded_id_opt(&self) -> Option<&str> {
            self.upgraded_id.as_ref().map(|field| field as _)
        }
        ///Sets `upgraded_id` with the provided value.
        pub fn set_upgraded_id<T: Into<String>>(&mut self, field: T) {
            self.upgraded_id = Some(field.into().into());
        }
        ///Sets `upgraded_id` with the provided value.
        pub fn with_upgraded_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_upgraded_id(field.into());
            self
        }
        ///If `upgraded_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn upgraded_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.upgraded_version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `upgraded_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn upgraded_version_mut(&mut self) -> &mut u64 {
            self.upgraded_version.get_or_insert_default()
        }
        ///If `upgraded_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn upgraded_version_opt(&self) -> Option<u64> {
            self.upgraded_version.as_ref().map(|field| *field)
        }
        ///Sets `upgraded_version` with the provided value.
        pub fn set_upgraded_version(&mut self, field: u64) {
            self.upgraded_version = Some(field);
        }
        ///Sets `upgraded_version` with the provided value.
        pub fn with_upgraded_version(mut self, field: u64) -> Self {
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
        ///If `owner` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn owner_opt_mut(&mut self) -> Option<&mut String> {
            self.owner.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `owner`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn owner_mut(&mut self) -> &mut String {
            self.owner.get_or_insert_default()
        }
        ///If `owner` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn owner_opt(&self) -> Option<&str> {
            self.owner.as_ref().map(|field| field as _)
        }
        ///Sets `owner` with the provided value.
        pub fn set_owner<T: Into<String>>(&mut self, field: T) {
            self.owner = Some(field.into().into());
        }
        ///Sets `owner` with the provided value.
        pub fn with_owner<T: Into<String>>(mut self, field: T) -> Self {
            self.set_owner(field.into());
            self
        }
        ///If `page_size` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn page_size_opt_mut(&mut self) -> Option<&mut u32> {
            self.page_size.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `page_size`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn page_size_mut(&mut self) -> &mut u32 {
            self.page_size.get_or_insert_default()
        }
        ///If `page_size` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn page_size_opt(&self) -> Option<u32> {
            self.page_size.as_ref().map(|field| *field)
        }
        ///Sets `page_size` with the provided value.
        pub fn set_page_size(&mut self, field: u32) {
            self.page_size = Some(field);
        }
        ///Sets `page_size` with the provided value.
        pub fn with_page_size(mut self, field: u32) -> Self {
            self.set_page_size(field);
            self
        }
        ///If `page_token` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn page_token_opt(&self) -> Option<&[u8]> {
            self.page_token.as_ref().map(|field| field as _)
        }
        ///Sets `page_token` with the provided value.
        pub fn set_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.page_token = Some(field.into().into());
        }
        ///Sets `page_token` with the provided value.
        pub fn with_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_page_token(field.into());
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
        ///Returns the value of `balances`, or the default value if `balances` is unset.
        pub fn balances(&self) -> &[super::Balance] {
            &self.balances
        }
        ///Returns a mutable reference to `balances`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn balances_mut(&mut self) -> &mut Vec<super::Balance> {
            &mut self.balances
        }
        ///Sets `balances` with the provided value.
        pub fn set_balances(&mut self, field: Vec<super::Balance>) {
            self.balances = field;
        }
        ///Sets `balances` with the provided value.
        pub fn with_balances(mut self, field: Vec<super::Balance>) -> Self {
            self.set_balances(field);
            self
        }
        ///If `next_page_token` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_page_token_opt(&self) -> Option<&[u8]> {
            self.next_page_token.as_ref().map(|field| field as _)
        }
        ///Sets `next_page_token` with the provided value.
        pub fn set_next_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.next_page_token = Some(field.into().into());
        }
        ///Sets `next_page_token` with the provided value.
        pub fn with_next_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_page_token(field.into());
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
        ///If `parent` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn parent_opt_mut(&mut self) -> Option<&mut String> {
            self.parent.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `parent`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn parent_mut(&mut self) -> &mut String {
            self.parent.get_or_insert_default()
        }
        ///If `parent` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn parent_opt(&self) -> Option<&str> {
            self.parent.as_ref().map(|field| field as _)
        }
        ///Sets `parent` with the provided value.
        pub fn set_parent<T: Into<String>>(&mut self, field: T) {
            self.parent = Some(field.into().into());
        }
        ///Sets `parent` with the provided value.
        pub fn with_parent<T: Into<String>>(mut self, field: T) -> Self {
            self.set_parent(field.into());
            self
        }
        ///If `page_size` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn page_size_opt_mut(&mut self) -> Option<&mut u32> {
            self.page_size.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `page_size`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn page_size_mut(&mut self) -> &mut u32 {
            self.page_size.get_or_insert_default()
        }
        ///If `page_size` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn page_size_opt(&self) -> Option<u32> {
            self.page_size.as_ref().map(|field| *field)
        }
        ///Sets `page_size` with the provided value.
        pub fn set_page_size(&mut self, field: u32) {
            self.page_size = Some(field);
        }
        ///Sets `page_size` with the provided value.
        pub fn with_page_size(mut self, field: u32) -> Self {
            self.set_page_size(field);
            self
        }
        ///If `page_token` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn page_token_opt(&self) -> Option<&[u8]> {
            self.page_token.as_ref().map(|field| field as _)
        }
        ///Sets `page_token` with the provided value.
        pub fn set_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.page_token = Some(field.into().into());
        }
        ///Sets `page_token` with the provided value.
        pub fn with_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_page_token(field.into());
            self
        }
        ///If `read_mask` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `read_mask`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        ///If `read_mask` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        ///Sets `read_mask` with the provided value.
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        ///Sets `read_mask` with the provided value.
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field.into());
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
        ///Returns the value of `dynamic_fields`, or the default value if `dynamic_fields` is unset.
        pub fn dynamic_fields(&self) -> &[super::DynamicField] {
            &self.dynamic_fields
        }
        ///Returns a mutable reference to `dynamic_fields`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn dynamic_fields_mut(&mut self) -> &mut Vec<super::DynamicField> {
            &mut self.dynamic_fields
        }
        ///Sets `dynamic_fields` with the provided value.
        pub fn set_dynamic_fields(&mut self, field: Vec<super::DynamicField>) {
            self.dynamic_fields = field;
        }
        ///Sets `dynamic_fields` with the provided value.
        pub fn with_dynamic_fields(mut self, field: Vec<super::DynamicField>) -> Self {
            self.set_dynamic_fields(field);
            self
        }
        ///If `next_page_token` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_page_token_opt(&self) -> Option<&[u8]> {
            self.next_page_token.as_ref().map(|field| field as _)
        }
        ///Sets `next_page_token` with the provided value.
        pub fn set_next_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.next_page_token = Some(field.into().into());
        }
        ///Sets `next_page_token` with the provided value.
        pub fn with_next_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_page_token(field.into());
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
        ///If `owner` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn owner_opt_mut(&mut self) -> Option<&mut String> {
            self.owner.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `owner`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn owner_mut(&mut self) -> &mut String {
            self.owner.get_or_insert_default()
        }
        ///If `owner` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn owner_opt(&self) -> Option<&str> {
            self.owner.as_ref().map(|field| field as _)
        }
        ///Sets `owner` with the provided value.
        pub fn set_owner<T: Into<String>>(&mut self, field: T) {
            self.owner = Some(field.into().into());
        }
        ///Sets `owner` with the provided value.
        pub fn with_owner<T: Into<String>>(mut self, field: T) -> Self {
            self.set_owner(field.into());
            self
        }
        ///If `page_size` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn page_size_opt_mut(&mut self) -> Option<&mut u32> {
            self.page_size.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `page_size`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn page_size_mut(&mut self) -> &mut u32 {
            self.page_size.get_or_insert_default()
        }
        ///If `page_size` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn page_size_opt(&self) -> Option<u32> {
            self.page_size.as_ref().map(|field| *field)
        }
        ///Sets `page_size` with the provided value.
        pub fn set_page_size(&mut self, field: u32) {
            self.page_size = Some(field);
        }
        ///Sets `page_size` with the provided value.
        pub fn with_page_size(mut self, field: u32) -> Self {
            self.set_page_size(field);
            self
        }
        ///If `page_token` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn page_token_opt(&self) -> Option<&[u8]> {
            self.page_token.as_ref().map(|field| field as _)
        }
        ///Sets `page_token` with the provided value.
        pub fn set_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.page_token = Some(field.into().into());
        }
        ///Sets `page_token` with the provided value.
        pub fn with_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_page_token(field.into());
            self
        }
        ///If `read_mask` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `read_mask`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        ///If `read_mask` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        ///Sets `read_mask` with the provided value.
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        ///Sets `read_mask` with the provided value.
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field.into());
            self
        }
        ///If `object_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_type_opt_mut(&mut self) -> Option<&mut String> {
            self.object_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_type_mut(&mut self) -> &mut String {
            self.object_type.get_or_insert_default()
        }
        ///If `object_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_type_opt(&self) -> Option<&str> {
            self.object_type.as_ref().map(|field| field as _)
        }
        ///Sets `object_type` with the provided value.
        pub fn set_object_type<T: Into<String>>(&mut self, field: T) {
            self.object_type = Some(field.into().into());
        }
        ///Sets `object_type` with the provided value.
        pub fn with_object_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_type(field.into());
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
        ///Returns the value of `objects`, or the default value if `objects` is unset.
        pub fn objects(&self) -> &[super::Object] {
            &self.objects
        }
        ///Returns a mutable reference to `objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn objects_mut(&mut self) -> &mut Vec<super::Object> {
            &mut self.objects
        }
        ///Sets `objects` with the provided value.
        pub fn set_objects(&mut self, field: Vec<super::Object>) {
            self.objects = field;
        }
        ///Sets `objects` with the provided value.
        pub fn with_objects(mut self, field: Vec<super::Object>) -> Self {
            self.set_objects(field);
            self
        }
        ///If `next_page_token` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_page_token_opt(&self) -> Option<&[u8]> {
            self.next_page_token.as_ref().map(|field| field as _)
        }
        ///Sets `next_page_token` with the provided value.
        pub fn set_next_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.next_page_token = Some(field.into().into());
        }
        ///Sets `next_page_token` with the provided value.
        pub fn with_next_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_page_token(field.into());
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
        ///If `package_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        ///If `package_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        ///Sets `package_id` with the provided value.
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        ///Sets `package_id` with the provided value.
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field.into());
            self
        }
        ///If `page_size` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn page_size_opt_mut(&mut self) -> Option<&mut u32> {
            self.page_size.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `page_size`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn page_size_mut(&mut self) -> &mut u32 {
            self.page_size.get_or_insert_default()
        }
        ///If `page_size` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn page_size_opt(&self) -> Option<u32> {
            self.page_size.as_ref().map(|field| *field)
        }
        ///Sets `page_size` with the provided value.
        pub fn set_page_size(&mut self, field: u32) {
            self.page_size = Some(field);
        }
        ///Sets `page_size` with the provided value.
        pub fn with_page_size(mut self, field: u32) -> Self {
            self.set_page_size(field);
            self
        }
        ///If `page_token` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn page_token_opt(&self) -> Option<&[u8]> {
            self.page_token.as_ref().map(|field| field as _)
        }
        ///Sets `page_token` with the provided value.
        pub fn set_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.page_token = Some(field.into().into());
        }
        ///Sets `page_token` with the provided value.
        pub fn with_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_page_token(field.into());
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
        ///Returns the value of `versions`, or the default value if `versions` is unset.
        pub fn versions(&self) -> &[super::PackageVersion] {
            &self.versions
        }
        ///Returns a mutable reference to `versions`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn versions_mut(&mut self) -> &mut Vec<super::PackageVersion> {
            &mut self.versions
        }
        ///Sets `versions` with the provided value.
        pub fn set_versions(&mut self, field: Vec<super::PackageVersion>) {
            self.versions = field;
        }
        ///Sets `versions` with the provided value.
        pub fn with_versions(mut self, field: Vec<super::PackageVersion>) -> Self {
            self.set_versions(field);
            self
        }
        ///If `next_page_token` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_page_token_opt(&self) -> Option<&[u8]> {
            self.next_page_token.as_ref().map(|field| field as _)
        }
        ///Sets `next_page_token` with the provided value.
        pub fn set_next_page_token<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.next_page_token = Some(field.into().into());
        }
        ///Sets `next_page_token` with the provided value.
        pub fn with_next_page_token<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_page_token(field.into());
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
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
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
        ///Returns the value of `record`, or the default value if `record` is unset.
        pub fn record(&self) -> &super::NameRecord {
            self.record
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::NameRecord::default_instance() as _)
        }
        ///If `record` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn record_opt_mut(&mut self) -> Option<&mut super::NameRecord> {
            self.record.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `record`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn record_mut(&mut self) -> &mut super::NameRecord {
            self.record.get_or_insert_default()
        }
        ///If `record` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn record_opt(&self) -> Option<&super::NameRecord> {
            self.record.as_ref().map(|field| field as _)
        }
        ///Sets `record` with the provided value.
        pub fn set_record<T: Into<super::NameRecord>>(&mut self, field: T) {
            self.record = Some(field.into().into());
        }
        ///Sets `record` with the provided value.
        pub fn with_record<T: Into<super::NameRecord>>(mut self, field: T) -> Self {
            self.set_record(field.into());
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
        ///If `element_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn element_type_opt_mut(&mut self) -> Option<&mut String> {
            self.element_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `element_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn element_type_mut(&mut self) -> &mut String {
            self.element_type.get_or_insert_default()
        }
        ///If `element_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn element_type_opt(&self) -> Option<&str> {
            self.element_type.as_ref().map(|field| field as _)
        }
        ///Sets `element_type` with the provided value.
        pub fn set_element_type<T: Into<String>>(&mut self, field: T) {
            self.element_type = Some(field.into().into());
        }
        ///Sets `element_type` with the provided value.
        pub fn with_element_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_element_type(field.into());
            self
        }
        ///Returns the value of `elements`, or the default value if `elements` is unset.
        pub fn elements(&self) -> &[super::Argument] {
            &self.elements
        }
        ///Returns a mutable reference to `elements`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn elements_mut(&mut self) -> &mut Vec<super::Argument> {
            &mut self.elements
        }
        ///Sets `elements` with the provided value.
        pub fn set_elements(&mut self, field: Vec<super::Argument>) {
            self.elements = field;
        }
        ///Sets `elements` with the provided value.
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
        ///Returns the value of `coin`, or the default value if `coin` is unset.
        pub fn coin(&self) -> &super::Argument {
            self.coin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Argument::default_instance() as _)
        }
        ///If `coin` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn coin_opt_mut(&mut self) -> Option<&mut super::Argument> {
            self.coin.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `coin`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coin_mut(&mut self) -> &mut super::Argument {
            self.coin.get_or_insert_default()
        }
        ///If `coin` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn coin_opt(&self) -> Option<&super::Argument> {
            self.coin.as_ref().map(|field| field as _)
        }
        ///Sets `coin` with the provided value.
        pub fn set_coin<T: Into<super::Argument>>(&mut self, field: T) {
            self.coin = Some(field.into().into());
        }
        ///Sets `coin` with the provided value.
        pub fn with_coin<T: Into<super::Argument>>(mut self, field: T) -> Self {
            self.set_coin(field.into());
            self
        }
        ///Returns the value of `coins_to_merge`, or the default value if `coins_to_merge` is unset.
        pub fn coins_to_merge(&self) -> &[super::Argument] {
            &self.coins_to_merge
        }
        ///Returns a mutable reference to `coins_to_merge`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coins_to_merge_mut(&mut self) -> &mut Vec<super::Argument> {
            &mut self.coins_to_merge
        }
        ///Sets `coins_to_merge` with the provided value.
        pub fn set_coins_to_merge(&mut self, field: Vec<super::Argument>) {
            self.coins_to_merge = field;
        }
        ///Sets `coins_to_merge` with the provided value.
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
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
            self
        }
        ///If `contents` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn contents_opt(&self) -> Option<&[u8]> {
            self.contents.as_ref().map(|field| field as _)
        }
        ///Sets `contents` with the provided value.
        pub fn set_contents<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.contents = Some(field.into().into());
        }
        ///Sets `contents` with the provided value.
        pub fn with_contents<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_contents(field.into());
            self
        }
        ///Returns the value of `datatypes`, or the default value if `datatypes` is unset.
        pub fn datatypes(&self) -> &[super::DatatypeDescriptor] {
            &self.datatypes
        }
        ///Returns a mutable reference to `datatypes`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn datatypes_mut(&mut self) -> &mut Vec<super::DatatypeDescriptor> {
            &mut self.datatypes
        }
        ///Sets `datatypes` with the provided value.
        pub fn set_datatypes(&mut self, field: Vec<super::DatatypeDescriptor>) {
            self.datatypes = field;
        }
        ///Sets `datatypes` with the provided value.
        pub fn with_datatypes(mut self, field: Vec<super::DatatypeDescriptor>) -> Self {
            self.set_datatypes(field);
            self
        }
        ///Returns the value of `functions`, or the default value if `functions` is unset.
        pub fn functions(&self) -> &[super::FunctionDescriptor] {
            &self.functions
        }
        ///Returns a mutable reference to `functions`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn functions_mut(&mut self) -> &mut Vec<super::FunctionDescriptor> {
            &mut self.functions
        }
        ///Sets `functions` with the provided value.
        pub fn set_functions(&mut self, field: Vec<super::FunctionDescriptor>) {
            self.functions = field;
        }
        ///Sets `functions` with the provided value.
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
        ///If `abort_code` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn abort_code_opt_mut(&mut self) -> Option<&mut u64> {
            self.abort_code.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `abort_code`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn abort_code_mut(&mut self) -> &mut u64 {
            self.abort_code.get_or_insert_default()
        }
        ///If `abort_code` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn abort_code_opt(&self) -> Option<u64> {
            self.abort_code.as_ref().map(|field| *field)
        }
        ///Sets `abort_code` with the provided value.
        pub fn set_abort_code(&mut self, field: u64) {
            self.abort_code = Some(field);
        }
        ///Sets `abort_code` with the provided value.
        pub fn with_abort_code(mut self, field: u64) -> Self {
            self.set_abort_code(field);
            self
        }
        ///Returns the value of `location`, or the default value if `location` is unset.
        pub fn location(&self) -> &super::MoveLocation {
            self.location
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveLocation::default_instance() as _)
        }
        ///If `location` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn location_opt_mut(&mut self) -> Option<&mut super::MoveLocation> {
            self.location.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `location`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn location_mut(&mut self) -> &mut super::MoveLocation {
            self.location.get_or_insert_default()
        }
        ///If `location` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn location_opt(&self) -> Option<&super::MoveLocation> {
            self.location.as_ref().map(|field| field as _)
        }
        ///Sets `location` with the provided value.
        pub fn set_location<T: Into<super::MoveLocation>>(&mut self, field: T) {
            self.location = Some(field.into().into());
        }
        ///Sets `location` with the provided value.
        pub fn with_location<T: Into<super::MoveLocation>>(mut self, field: T) -> Self {
            self.set_location(field.into());
            self
        }
        ///Returns the value of `clever_error`, or the default value if `clever_error` is unset.
        pub fn clever_error(&self) -> &super::CleverError {
            self.clever_error
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CleverError::default_instance() as _)
        }
        ///If `clever_error` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn clever_error_opt_mut(&mut self) -> Option<&mut super::CleverError> {
            self.clever_error.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `clever_error`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn clever_error_mut(&mut self) -> &mut super::CleverError {
            self.clever_error.get_or_insert_default()
        }
        ///If `clever_error` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn clever_error_opt(&self) -> Option<&super::CleverError> {
            self.clever_error.as_ref().map(|field| field as _)
        }
        ///Sets `clever_error` with the provided value.
        pub fn set_clever_error<T: Into<super::CleverError>>(&mut self, field: T) {
            self.clever_error = Some(field.into().into());
        }
        ///Sets `clever_error` with the provided value.
        pub fn with_clever_error<T: Into<super::CleverError>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_clever_error(field.into());
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
        ///If `package` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_opt_mut(&mut self) -> Option<&mut String> {
            self.package.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_mut(&mut self) -> &mut String {
            self.package.get_or_insert_default()
        }
        ///If `package` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_opt(&self) -> Option<&str> {
            self.package.as_ref().map(|field| field as _)
        }
        ///Sets `package` with the provided value.
        pub fn set_package<T: Into<String>>(&mut self, field: T) {
            self.package = Some(field.into().into());
        }
        ///Sets `package` with the provided value.
        pub fn with_package<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package(field.into());
            self
        }
        ///If `module` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn module_opt_mut(&mut self) -> Option<&mut String> {
            self.module.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `module`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn module_mut(&mut self) -> &mut String {
            self.module.get_or_insert_default()
        }
        ///If `module` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn module_opt(&self) -> Option<&str> {
            self.module.as_ref().map(|field| field as _)
        }
        ///Sets `module` with the provided value.
        pub fn set_module<T: Into<String>>(&mut self, field: T) {
            self.module = Some(field.into().into());
        }
        ///Sets `module` with the provided value.
        pub fn with_module<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module(field.into());
            self
        }
        ///If `function` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn function_opt_mut(&mut self) -> Option<&mut String> {
            self.function.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `function`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn function_mut(&mut self) -> &mut String {
            self.function.get_or_insert_default()
        }
        ///If `function` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn function_opt(&self) -> Option<&str> {
            self.function.as_ref().map(|field| field as _)
        }
        ///Sets `function` with the provided value.
        pub fn set_function<T: Into<String>>(&mut self, field: T) {
            self.function = Some(field.into().into());
        }
        ///Sets `function` with the provided value.
        pub fn with_function<T: Into<String>>(mut self, field: T) -> Self {
            self.set_function(field.into());
            self
        }
        ///Returns the value of `type_arguments`, or the default value if `type_arguments` is unset.
        pub fn type_arguments(&self) -> &[String] {
            &self.type_arguments
        }
        ///Returns a mutable reference to `type_arguments`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn type_arguments_mut(&mut self) -> &mut Vec<String> {
            &mut self.type_arguments
        }
        ///Sets `type_arguments` with the provided value.
        pub fn set_type_arguments(&mut self, field: Vec<String>) {
            self.type_arguments = field;
        }
        ///Sets `type_arguments` with the provided value.
        pub fn with_type_arguments(mut self, field: Vec<String>) -> Self {
            self.set_type_arguments(field);
            self
        }
        ///Returns the value of `arguments`, or the default value if `arguments` is unset.
        pub fn arguments(&self) -> &[super::Argument] {
            &self.arguments
        }
        ///Returns a mutable reference to `arguments`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn arguments_mut(&mut self) -> &mut Vec<super::Argument> {
            &mut self.arguments
        }
        ///Sets `arguments` with the provided value.
        pub fn set_arguments(&mut self, field: Vec<super::Argument>) {
            self.arguments = field;
        }
        ///Sets `arguments` with the provided value.
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
        ///If `package` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_opt_mut(&mut self) -> Option<&mut String> {
            self.package.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_mut(&mut self) -> &mut String {
            self.package.get_or_insert_default()
        }
        ///If `package` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_opt(&self) -> Option<&str> {
            self.package.as_ref().map(|field| field as _)
        }
        ///Sets `package` with the provided value.
        pub fn set_package<T: Into<String>>(&mut self, field: T) {
            self.package = Some(field.into().into());
        }
        ///Sets `package` with the provided value.
        pub fn with_package<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package(field.into());
            self
        }
        ///If `module` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn module_opt_mut(&mut self) -> Option<&mut String> {
            self.module.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `module`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn module_mut(&mut self) -> &mut String {
            self.module.get_or_insert_default()
        }
        ///If `module` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn module_opt(&self) -> Option<&str> {
            self.module.as_ref().map(|field| field as _)
        }
        ///Sets `module` with the provided value.
        pub fn set_module<T: Into<String>>(&mut self, field: T) {
            self.module = Some(field.into().into());
        }
        ///Sets `module` with the provided value.
        pub fn with_module<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module(field.into());
            self
        }
        ///If `function` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn function_opt_mut(&mut self) -> Option<&mut u32> {
            self.function.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `function`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn function_mut(&mut self) -> &mut u32 {
            self.function.get_or_insert_default()
        }
        ///If `function` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn function_opt(&self) -> Option<u32> {
            self.function.as_ref().map(|field| *field)
        }
        ///Sets `function` with the provided value.
        pub fn set_function(&mut self, field: u32) {
            self.function = Some(field);
        }
        ///Sets `function` with the provided value.
        pub fn with_function(mut self, field: u32) -> Self {
            self.set_function(field);
            self
        }
        ///If `instruction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn instruction_opt_mut(&mut self) -> Option<&mut u32> {
            self.instruction.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `instruction`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn instruction_mut(&mut self) -> &mut u32 {
            self.instruction.get_or_insert_default()
        }
        ///If `instruction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn instruction_opt(&self) -> Option<u32> {
            self.instruction.as_ref().map(|field| *field)
        }
        ///Sets `instruction` with the provided value.
        pub fn set_instruction(&mut self, field: u32) {
            self.instruction = Some(field);
        }
        ///Sets `instruction` with the provided value.
        pub fn with_instruction(mut self, field: u32) -> Self {
            self.set_instruction(field);
            self
        }
        ///If `function_name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn function_name_opt_mut(&mut self) -> Option<&mut String> {
            self.function_name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `function_name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn function_name_mut(&mut self) -> &mut String {
            self.function_name.get_or_insert_default()
        }
        ///If `function_name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn function_name_opt(&self) -> Option<&str> {
            self.function_name.as_ref().map(|field| field as _)
        }
        ///Sets `function_name` with the provided value.
        pub fn set_function_name<T: Into<String>>(&mut self, field: T) {
            self.function_name = Some(field.into().into());
        }
        ///Sets `function_name` with the provided value.
        pub fn with_function_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_function_name(field.into());
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
        ///If `id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        ///If `id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        ///Sets `id` with the provided value.
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        ///Sets `id` with the provided value.
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field.into());
            self
        }
        ///If `size` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn size_opt_mut(&mut self) -> Option<&mut u64> {
            self.size.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `size`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn size_mut(&mut self) -> &mut u64 {
            self.size.get_or_insert_default()
        }
        ///If `size` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn size_opt(&self) -> Option<u64> {
            self.size.as_ref().map(|field| *field)
        }
        ///Sets `size` with the provided value.
        pub fn set_size(&mut self, field: u64) {
            self.size = Some(field);
        }
        ///Sets `size` with the provided value.
        pub fn with_size(mut self, field: u64) -> Self {
            self.set_size(field);
            self
        }
    }
    impl super::MultisigAggregatedSignature {
        pub const fn const_default() -> Self {
            Self {
                signatures: Vec::new(),
                bitmap: None,
                legacy_bitmap: None,
                committee: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MultisigAggregatedSignature = super::MultisigAggregatedSignature::const_default();
            &DEFAULT
        }
        ///Returns the value of `signatures`, or the default value if `signatures` is unset.
        pub fn signatures(&self) -> &[super::MultisigMemberSignature] {
            &self.signatures
        }
        ///Returns a mutable reference to `signatures`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn signatures_mut(&mut self) -> &mut Vec<super::MultisigMemberSignature> {
            &mut self.signatures
        }
        ///Sets `signatures` with the provided value.
        pub fn set_signatures(&mut self, field: Vec<super::MultisigMemberSignature>) {
            self.signatures = field;
        }
        ///Sets `signatures` with the provided value.
        pub fn with_signatures(
            mut self,
            field: Vec<super::MultisigMemberSignature>,
        ) -> Self {
            self.set_signatures(field);
            self
        }
        ///If `bitmap` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn bitmap_opt_mut(&mut self) -> Option<&mut u32> {
            self.bitmap.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `bitmap`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn bitmap_mut(&mut self) -> &mut u32 {
            self.bitmap.get_or_insert_default()
        }
        ///If `bitmap` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn bitmap_opt(&self) -> Option<u32> {
            self.bitmap.as_ref().map(|field| *field)
        }
        ///Sets `bitmap` with the provided value.
        pub fn set_bitmap(&mut self, field: u32) {
            self.bitmap = Some(field);
        }
        ///Sets `bitmap` with the provided value.
        pub fn with_bitmap(mut self, field: u32) -> Self {
            self.set_bitmap(field);
            self
        }
        ///If `legacy_bitmap` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn legacy_bitmap_opt(&self) -> Option<&[u8]> {
            self.legacy_bitmap.as_ref().map(|field| field as _)
        }
        ///Sets `legacy_bitmap` with the provided value.
        pub fn set_legacy_bitmap<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.legacy_bitmap = Some(field.into().into());
        }
        ///Sets `legacy_bitmap` with the provided value.
        pub fn with_legacy_bitmap<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_legacy_bitmap(field.into());
            self
        }
        ///Returns the value of `committee`, or the default value if `committee` is unset.
        pub fn committee(&self) -> &super::MultisigCommittee {
            self.committee
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MultisigCommittee::default_instance() as _)
        }
        ///If `committee` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn committee_opt_mut(&mut self) -> Option<&mut super::MultisigCommittee> {
            self.committee.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `committee`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn committee_mut(&mut self) -> &mut super::MultisigCommittee {
            self.committee.get_or_insert_default()
        }
        ///If `committee` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn committee_opt(&self) -> Option<&super::MultisigCommittee> {
            self.committee.as_ref().map(|field| field as _)
        }
        ///Sets `committee` with the provided value.
        pub fn set_committee<T: Into<super::MultisigCommittee>>(&mut self, field: T) {
            self.committee = Some(field.into().into());
        }
        ///Sets `committee` with the provided value.
        pub fn with_committee<T: Into<super::MultisigCommittee>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_committee(field.into());
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
        ///Returns the value of `members`, or the default value if `members` is unset.
        pub fn members(&self) -> &[super::MultisigMember] {
            &self.members
        }
        ///Returns a mutable reference to `members`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn members_mut(&mut self) -> &mut Vec<super::MultisigMember> {
            &mut self.members
        }
        ///Sets `members` with the provided value.
        pub fn set_members(&mut self, field: Vec<super::MultisigMember>) {
            self.members = field;
        }
        ///Sets `members` with the provided value.
        pub fn with_members(mut self, field: Vec<super::MultisigMember>) -> Self {
            self.set_members(field);
            self
        }
        ///If `threshold` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn threshold_opt_mut(&mut self) -> Option<&mut u32> {
            self.threshold.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `threshold`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn threshold_mut(&mut self) -> &mut u32 {
            self.threshold.get_or_insert_default()
        }
        ///If `threshold` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn threshold_opt(&self) -> Option<u32> {
            self.threshold.as_ref().map(|field| *field)
        }
        ///Sets `threshold` with the provided value.
        pub fn set_threshold(&mut self, field: u32) {
            self.threshold = Some(field);
        }
        ///Sets `threshold` with the provided value.
        pub fn with_threshold(mut self, field: u32) -> Self {
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
        ///Returns the value of `public_key`, or the default value if `public_key` is unset.
        pub fn public_key(&self) -> &super::MultisigMemberPublicKey {
            self.public_key
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::MultisigMemberPublicKey::default_instance() as _
                })
        }
        ///If `public_key` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn public_key_opt_mut(
            &mut self,
        ) -> Option<&mut super::MultisigMemberPublicKey> {
            self.public_key.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `public_key`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn public_key_mut(&mut self) -> &mut super::MultisigMemberPublicKey {
            self.public_key.get_or_insert_default()
        }
        ///If `public_key` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn public_key_opt(&self) -> Option<&super::MultisigMemberPublicKey> {
            self.public_key.as_ref().map(|field| field as _)
        }
        ///Sets `public_key` with the provided value.
        pub fn set_public_key<T: Into<super::MultisigMemberPublicKey>>(
            &mut self,
            field: T,
        ) {
            self.public_key = Some(field.into().into());
        }
        ///Sets `public_key` with the provided value.
        pub fn with_public_key<T: Into<super::MultisigMemberPublicKey>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_public_key(field.into());
            self
        }
        ///If `weight` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn weight_opt_mut(&mut self) -> Option<&mut u32> {
            self.weight.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `weight`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn weight_mut(&mut self) -> &mut u32 {
            self.weight.get_or_insert_default()
        }
        ///If `weight` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn weight_opt(&self) -> Option<u32> {
            self.weight.as_ref().map(|field| *field)
        }
        ///Sets `weight` with the provided value.
        pub fn set_weight(&mut self, field: u32) {
            self.weight = Some(field);
        }
        ///Sets `weight` with the provided value.
        pub fn with_weight(mut self, field: u32) -> Self {
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
        ///Sets `scheme` with the provided value.
        pub fn with_scheme<T: Into<super::SignatureScheme>>(mut self, field: T) -> Self {
            self.set_scheme(field.into());
            self
        }
        ///If `public_key` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn public_key_opt(&self) -> Option<&[u8]> {
            self.public_key.as_ref().map(|field| field as _)
        }
        ///Sets `public_key` with the provided value.
        pub fn set_public_key<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.public_key = Some(field.into().into());
        }
        ///Sets `public_key` with the provided value.
        pub fn with_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_public_key(field.into());
            self
        }
        ///Returns the value of `zklogin`, or the default value if `zklogin` is unset.
        pub fn zklogin(&self) -> &super::ZkLoginPublicIdentifier {
            self.zklogin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::ZkLoginPublicIdentifier::default_instance() as _
                })
        }
        ///If `zklogin` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn zklogin_opt_mut(
            &mut self,
        ) -> Option<&mut super::ZkLoginPublicIdentifier> {
            self.zklogin.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `zklogin`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn zklogin_mut(&mut self) -> &mut super::ZkLoginPublicIdentifier {
            self.zklogin.get_or_insert_default()
        }
        ///If `zklogin` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn zklogin_opt(&self) -> Option<&super::ZkLoginPublicIdentifier> {
            self.zklogin.as_ref().map(|field| field as _)
        }
        ///Sets `zklogin` with the provided value.
        pub fn set_zklogin<T: Into<super::ZkLoginPublicIdentifier>>(
            &mut self,
            field: T,
        ) {
            self.zklogin = Some(field.into().into());
        }
        ///Sets `zklogin` with the provided value.
        pub fn with_zklogin<T: Into<super::ZkLoginPublicIdentifier>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_zklogin(field.into());
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
        ///Sets `scheme` with the provided value.
        pub fn with_scheme<T: Into<super::SignatureScheme>>(mut self, field: T) -> Self {
            self.set_scheme(field.into());
            self
        }
        ///If `signature` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn signature_opt(&self) -> Option<&[u8]> {
            self.signature.as_ref().map(|field| field as _)
        }
        ///Sets `signature` with the provided value.
        pub fn set_signature<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        ///Sets `signature` with the provided value.
        pub fn with_signature<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field.into());
            self
        }
        ///Returns the value of `zklogin`, or the default value if `zklogin` is unset.
        pub fn zklogin(&self) -> &super::ZkLoginAuthenticator {
            self.zklogin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ZkLoginAuthenticator::default_instance() as _)
        }
        ///If `zklogin` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn zklogin_opt_mut(&mut self) -> Option<&mut super::ZkLoginAuthenticator> {
            self.zklogin.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `zklogin`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn zklogin_mut(&mut self) -> &mut super::ZkLoginAuthenticator {
            self.zklogin.get_or_insert_default()
        }
        ///If `zklogin` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn zklogin_opt(&self) -> Option<&super::ZkLoginAuthenticator> {
            self.zklogin.as_ref().map(|field| field as _)
        }
        ///Sets `zklogin` with the provided value.
        pub fn set_zklogin<T: Into<super::ZkLoginAuthenticator>>(&mut self, field: T) {
            self.zklogin = Some(field.into().into());
        }
        ///Sets `zklogin` with the provided value.
        pub fn with_zklogin<T: Into<super::ZkLoginAuthenticator>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_zklogin(field.into());
            self
        }
        ///Returns the value of `passkey`, or the default value if `passkey` is unset.
        pub fn passkey(&self) -> &super::PasskeyAuthenticator {
            self.passkey
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::PasskeyAuthenticator::default_instance() as _)
        }
        ///If `passkey` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn passkey_opt_mut(&mut self) -> Option<&mut super::PasskeyAuthenticator> {
            self.passkey.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `passkey`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn passkey_mut(&mut self) -> &mut super::PasskeyAuthenticator {
            self.passkey.get_or_insert_default()
        }
        ///If `passkey` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn passkey_opt(&self) -> Option<&super::PasskeyAuthenticator> {
            self.passkey.as_ref().map(|field| field as _)
        }
        ///Sets `passkey` with the provided value.
        pub fn set_passkey<T: Into<super::PasskeyAuthenticator>>(&mut self, field: T) {
            self.passkey = Some(field.into().into());
        }
        ///Sets `passkey` with the provided value.
        pub fn with_passkey<T: Into<super::PasskeyAuthenticator>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_passkey(field.into());
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
        ///If `id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        ///If `id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        ///Sets `id` with the provided value.
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        ///Sets `id` with the provided value.
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field.into());
            self
        }
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
            self
        }
        ///If `registration_nft_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn registration_nft_id_opt_mut(&mut self) -> Option<&mut String> {
            self.registration_nft_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `registration_nft_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn registration_nft_id_mut(&mut self) -> &mut String {
            self.registration_nft_id.get_or_insert_default()
        }
        ///If `registration_nft_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn registration_nft_id_opt(&self) -> Option<&str> {
            self.registration_nft_id.as_ref().map(|field| field as _)
        }
        ///Sets `registration_nft_id` with the provided value.
        pub fn set_registration_nft_id<T: Into<String>>(&mut self, field: T) {
            self.registration_nft_id = Some(field.into().into());
        }
        ///Sets `registration_nft_id` with the provided value.
        pub fn with_registration_nft_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_registration_nft_id(field.into());
            self
        }
        ///If `expiration_timestamp` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn expiration_timestamp_opt_mut(
            &mut self,
        ) -> Option<&mut ::prost_types::Timestamp> {
            self.expiration_timestamp.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `expiration_timestamp`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn expiration_timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.expiration_timestamp.get_or_insert_default()
        }
        ///If `expiration_timestamp` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn expiration_timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.expiration_timestamp.as_ref().map(|field| field as _)
        }
        ///Sets `expiration_timestamp` with the provided value.
        pub fn set_expiration_timestamp<T: Into<::prost_types::Timestamp>>(
            &mut self,
            field: T,
        ) {
            self.expiration_timestamp = Some(field.into().into());
        }
        ///Sets `expiration_timestamp` with the provided value.
        pub fn with_expiration_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_expiration_timestamp(field.into());
            self
        }
        ///If `target_address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn target_address_opt_mut(&mut self) -> Option<&mut String> {
            self.target_address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `target_address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn target_address_mut(&mut self) -> &mut String {
            self.target_address.get_or_insert_default()
        }
        ///If `target_address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn target_address_opt(&self) -> Option<&str> {
            self.target_address.as_ref().map(|field| field as _)
        }
        ///Sets `target_address` with the provided value.
        pub fn set_target_address<T: Into<String>>(&mut self, field: T) {
            self.target_address = Some(field.into().into());
        }
        ///Sets `target_address` with the provided value.
        pub fn with_target_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_target_address(field.into());
            self
        }
        ///Returns the value of `data`, or the default value if `data` is unset.
        pub fn data(&self) -> &::std::collections::BTreeMap<String, String> {
            &self.data
        }
        ///Returns a mutable reference to `data`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn data_mut(&mut self) -> &mut ::std::collections::BTreeMap<String, String> {
            &mut self.data
        }
        ///Sets `data` with the provided value.
        pub fn set_data(&mut self, field: ::std::collections::BTreeMap<String, String>) {
            self.data = field;
        }
        ///Sets `data` with the provided value.
        pub fn with_data(
            mut self,
            field: ::std::collections::BTreeMap<String, String>,
        ) -> Self {
            self.set_data(field);
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
                display: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Object = super::Object::const_default();
            &DEFAULT
        }
        ///Returns the value of `bcs`, or the default value if `bcs` is unset.
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `bcs` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `bcs`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        ///If `bcs` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        ///Sets `bcs` with the provided value.
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        ///Sets `bcs` with the provided value.
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field.into());
            self
        }
        ///If `object_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        ///If `object_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        ///Sets `object_id` with the provided value.
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        ///Sets `object_id` with the provided value.
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field.into());
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
            self.set_version(field);
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///Returns the value of `owner`, or the default value if `owner` is unset.
        pub fn owner(&self) -> &super::Owner {
            self.owner
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Owner::default_instance() as _)
        }
        ///If `owner` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn owner_opt_mut(&mut self) -> Option<&mut super::Owner> {
            self.owner.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `owner`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn owner_mut(&mut self) -> &mut super::Owner {
            self.owner.get_or_insert_default()
        }
        ///If `owner` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn owner_opt(&self) -> Option<&super::Owner> {
            self.owner.as_ref().map(|field| field as _)
        }
        ///Sets `owner` with the provided value.
        pub fn set_owner<T: Into<super::Owner>>(&mut self, field: T) {
            self.owner = Some(field.into().into());
        }
        ///Sets `owner` with the provided value.
        pub fn with_owner<T: Into<super::Owner>>(mut self, field: T) -> Self {
            self.set_owner(field.into());
            self
        }
        ///If `object_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_type_opt_mut(&mut self) -> Option<&mut String> {
            self.object_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_type_mut(&mut self) -> &mut String {
            self.object_type.get_or_insert_default()
        }
        ///If `object_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_type_opt(&self) -> Option<&str> {
            self.object_type.as_ref().map(|field| field as _)
        }
        ///Sets `object_type` with the provided value.
        pub fn set_object_type<T: Into<String>>(&mut self, field: T) {
            self.object_type = Some(field.into().into());
        }
        ///Sets `object_type` with the provided value.
        pub fn with_object_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_type(field.into());
            self
        }
        ///If `has_public_transfer` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn has_public_transfer_opt_mut(&mut self) -> Option<&mut bool> {
            self.has_public_transfer.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `has_public_transfer`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn has_public_transfer_mut(&mut self) -> &mut bool {
            self.has_public_transfer.get_or_insert_default()
        }
        ///If `has_public_transfer` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn has_public_transfer_opt(&self) -> Option<bool> {
            self.has_public_transfer.as_ref().map(|field| *field)
        }
        ///Sets `has_public_transfer` with the provided value.
        pub fn set_has_public_transfer(&mut self, field: bool) {
            self.has_public_transfer = Some(field);
        }
        ///Sets `has_public_transfer` with the provided value.
        pub fn with_has_public_transfer(mut self, field: bool) -> Self {
            self.set_has_public_transfer(field);
            self
        }
        ///Returns the value of `contents`, or the default value if `contents` is unset.
        pub fn contents(&self) -> &super::Bcs {
            self.contents
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `contents` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn contents_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.contents.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `contents`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn contents_mut(&mut self) -> &mut super::Bcs {
            self.contents.get_or_insert_default()
        }
        ///If `contents` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn contents_opt(&self) -> Option<&super::Bcs> {
            self.contents.as_ref().map(|field| field as _)
        }
        ///Sets `contents` with the provided value.
        pub fn set_contents<T: Into<super::Bcs>>(&mut self, field: T) {
            self.contents = Some(field.into().into());
        }
        ///Sets `contents` with the provided value.
        pub fn with_contents<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_contents(field.into());
            self
        }
        ///Returns the value of `package`, or the default value if `package` is unset.
        pub fn package(&self) -> &super::Package {
            self.package
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Package::default_instance() as _)
        }
        ///If `package` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_opt_mut(&mut self) -> Option<&mut super::Package> {
            self.package.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_mut(&mut self) -> &mut super::Package {
            self.package.get_or_insert_default()
        }
        ///If `package` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_opt(&self) -> Option<&super::Package> {
            self.package.as_ref().map(|field| field as _)
        }
        ///Sets `package` with the provided value.
        pub fn set_package<T: Into<super::Package>>(&mut self, field: T) {
            self.package = Some(field.into().into());
        }
        ///Sets `package` with the provided value.
        pub fn with_package<T: Into<super::Package>>(mut self, field: T) -> Self {
            self.set_package(field.into());
            self
        }
        ///If `previous_transaction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn previous_transaction_opt_mut(&mut self) -> Option<&mut String> {
            self.previous_transaction.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `previous_transaction`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn previous_transaction_mut(&mut self) -> &mut String {
            self.previous_transaction.get_or_insert_default()
        }
        ///If `previous_transaction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn previous_transaction_opt(&self) -> Option<&str> {
            self.previous_transaction.as_ref().map(|field| field as _)
        }
        ///Sets `previous_transaction` with the provided value.
        pub fn set_previous_transaction<T: Into<String>>(&mut self, field: T) {
            self.previous_transaction = Some(field.into().into());
        }
        ///Sets `previous_transaction` with the provided value.
        pub fn with_previous_transaction<T: Into<String>>(mut self, field: T) -> Self {
            self.set_previous_transaction(field.into());
            self
        }
        ///If `storage_rebate` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn storage_rebate_opt_mut(&mut self) -> Option<&mut u64> {
            self.storage_rebate.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `storage_rebate`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn storage_rebate_mut(&mut self) -> &mut u64 {
            self.storage_rebate.get_or_insert_default()
        }
        ///If `storage_rebate` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn storage_rebate_opt(&self) -> Option<u64> {
            self.storage_rebate.as_ref().map(|field| *field)
        }
        ///Sets `storage_rebate` with the provided value.
        pub fn set_storage_rebate(&mut self, field: u64) {
            self.storage_rebate = Some(field);
        }
        ///Sets `storage_rebate` with the provided value.
        pub fn with_storage_rebate(mut self, field: u64) -> Self {
            self.set_storage_rebate(field);
            self
        }
        ///If `json` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn json_opt_mut(&mut self) -> Option<&mut ::prost_types::Value> {
            self.json.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `json`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn json_mut(&mut self) -> &mut ::prost_types::Value {
            self.json.get_or_insert_default()
        }
        ///If `json` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn json_opt(&self) -> Option<&::prost_types::Value> {
            self.json.as_ref().map(|field| field as _)
        }
        ///Sets `json` with the provided value.
        pub fn set_json<T: Into<::prost_types::Value>>(&mut self, field: T) {
            self.json = Some(field.into().into());
        }
        ///Sets `json` with the provided value.
        pub fn with_json<T: Into<::prost_types::Value>>(mut self, field: T) -> Self {
            self.set_json(field.into());
            self
        }
        ///If `balance` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.balance.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `balance`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn balance_mut(&mut self) -> &mut u64 {
            self.balance.get_or_insert_default()
        }
        ///If `balance` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn balance_opt(&self) -> Option<u64> {
            self.balance.as_ref().map(|field| *field)
        }
        ///Sets `balance` with the provided value.
        pub fn set_balance(&mut self, field: u64) {
            self.balance = Some(field);
        }
        ///Sets `balance` with the provided value.
        pub fn with_balance(mut self, field: u64) -> Self {
            self.set_balance(field);
            self
        }
        ///Returns the value of `display`, or the default value if `display` is unset.
        pub fn display(&self) -> &super::Display {
            self.display
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Display::default_instance() as _)
        }
        ///If `display` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn display_opt_mut(&mut self) -> Option<&mut super::Display> {
            self.display.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `display`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn display_mut(&mut self) -> &mut super::Display {
            self.display.get_or_insert_default()
        }
        ///If `display` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn display_opt(&self) -> Option<&super::Display> {
            self.display.as_ref().map(|field| field as _)
        }
        ///Sets `display` with the provided value.
        pub fn set_display<T: Into<super::Display>>(&mut self, field: T) {
            self.display = Some(field.into().into());
        }
        ///Sets `display` with the provided value.
        pub fn with_display<T: Into<super::Display>>(mut self, field: T) -> Self {
            self.set_display(field.into());
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
        ///If `object_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        ///If `object_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        ///Sets `object_id` with the provided value.
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        ///Sets `object_id` with the provided value.
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field.into());
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
            self.set_version(field);
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
    }
    impl super::ObjectSet {
        pub const fn const_default() -> Self {
            Self { objects: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ObjectSet = super::ObjectSet::const_default();
            &DEFAULT
        }
        ///Returns the value of `objects`, or the default value if `objects` is unset.
        pub fn objects(&self) -> &[super::Object] {
            &self.objects
        }
        ///Returns a mutable reference to `objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn objects_mut(&mut self) -> &mut Vec<super::Object> {
            &mut self.objects
        }
        ///Sets `objects` with the provided value.
        pub fn set_objects(&mut self, field: Vec<super::Object>) {
            self.objects = field;
        }
        ///Sets `objects` with the provided value.
        pub fn with_objects(mut self, field: Vec<super::Object>) -> Self {
            self.set_objects(field);
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
        ///Sets `reference` with the provided value.
        pub fn with_reference<T: Into<super::open_signature::Reference>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_reference(field.into());
            self
        }
        ///Returns the value of `body`, or the default value if `body` is unset.
        pub fn body(&self) -> &super::OpenSignatureBody {
            self.body
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::OpenSignatureBody::default_instance() as _)
        }
        ///If `body` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn body_opt_mut(&mut self) -> Option<&mut super::OpenSignatureBody> {
            self.body.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `body`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn body_mut(&mut self) -> &mut super::OpenSignatureBody {
            self.body.get_or_insert_default()
        }
        ///If `body` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn body_opt(&self) -> Option<&super::OpenSignatureBody> {
            self.body.as_ref().map(|field| field as _)
        }
        ///Sets `body` with the provided value.
        pub fn set_body<T: Into<super::OpenSignatureBody>>(&mut self, field: T) {
            self.body = Some(field.into().into());
        }
        ///Sets `body` with the provided value.
        pub fn with_body<T: Into<super::OpenSignatureBody>>(mut self, field: T) -> Self {
            self.set_body(field.into());
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
        ///Sets `r#type` with the provided value.
        pub fn with_type<T: Into<super::open_signature_body::Type>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_type(field.into());
            self
        }
        ///If `type_name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn type_name_opt_mut(&mut self) -> Option<&mut String> {
            self.type_name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `type_name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn type_name_mut(&mut self) -> &mut String {
            self.type_name.get_or_insert_default()
        }
        ///If `type_name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn type_name_opt(&self) -> Option<&str> {
            self.type_name.as_ref().map(|field| field as _)
        }
        ///Sets `type_name` with the provided value.
        pub fn set_type_name<T: Into<String>>(&mut self, field: T) {
            self.type_name = Some(field.into().into());
        }
        ///Sets `type_name` with the provided value.
        pub fn with_type_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_type_name(field.into());
            self
        }
        ///Returns the value of `type_parameter_instantiation`, or the default value if `type_parameter_instantiation` is unset.
        pub fn type_parameter_instantiation(&self) -> &[super::OpenSignatureBody] {
            &self.type_parameter_instantiation
        }
        ///Returns a mutable reference to `type_parameter_instantiation`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn type_parameter_instantiation_mut(
            &mut self,
        ) -> &mut Vec<super::OpenSignatureBody> {
            &mut self.type_parameter_instantiation
        }
        ///Sets `type_parameter_instantiation` with the provided value.
        pub fn set_type_parameter_instantiation(
            &mut self,
            field: Vec<super::OpenSignatureBody>,
        ) {
            self.type_parameter_instantiation = field;
        }
        ///Sets `type_parameter_instantiation` with the provided value.
        pub fn with_type_parameter_instantiation(
            mut self,
            field: Vec<super::OpenSignatureBody>,
        ) -> Self {
            self.set_type_parameter_instantiation(field);
            self
        }
        ///If `type_parameter` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn type_parameter_opt_mut(&mut self) -> Option<&mut u32> {
            self.type_parameter.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `type_parameter`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn type_parameter_mut(&mut self) -> &mut u32 {
            self.type_parameter.get_or_insert_default()
        }
        ///If `type_parameter` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn type_parameter_opt(&self) -> Option<u32> {
            self.type_parameter.as_ref().map(|field| *field)
        }
        ///Sets `type_parameter` with the provided value.
        pub fn set_type_parameter(&mut self, field: u32) {
            self.type_parameter = Some(field);
        }
        ///Sets `type_parameter` with the provided value.
        pub fn with_type_parameter(mut self, field: u32) -> Self {
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
        ///Sets `kind` with the provided value.
        pub fn with_kind<T: Into<super::owner::OwnerKind>>(mut self, field: T) -> Self {
            self.set_kind(field.into());
            self
        }
        ///If `address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        ///If `address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        ///Sets `address` with the provided value.
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        ///Sets `address` with the provided value.
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field.into());
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
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
        ///If `storage_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn storage_id_opt_mut(&mut self) -> Option<&mut String> {
            self.storage_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `storage_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn storage_id_mut(&mut self) -> &mut String {
            self.storage_id.get_or_insert_default()
        }
        ///If `storage_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn storage_id_opt(&self) -> Option<&str> {
            self.storage_id.as_ref().map(|field| field as _)
        }
        ///Sets `storage_id` with the provided value.
        pub fn set_storage_id<T: Into<String>>(&mut self, field: T) {
            self.storage_id = Some(field.into().into());
        }
        ///Sets `storage_id` with the provided value.
        pub fn with_storage_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_storage_id(field.into());
            self
        }
        ///If `original_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn original_id_opt_mut(&mut self) -> Option<&mut String> {
            self.original_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `original_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn original_id_mut(&mut self) -> &mut String {
            self.original_id.get_or_insert_default()
        }
        ///If `original_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn original_id_opt(&self) -> Option<&str> {
            self.original_id.as_ref().map(|field| field as _)
        }
        ///Sets `original_id` with the provided value.
        pub fn set_original_id<T: Into<String>>(&mut self, field: T) {
            self.original_id = Some(field.into().into());
        }
        ///Sets `original_id` with the provided value.
        pub fn with_original_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_original_id(field.into());
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
            self.set_version(field);
            self
        }
        ///Returns the value of `modules`, or the default value if `modules` is unset.
        pub fn modules(&self) -> &[super::Module] {
            &self.modules
        }
        ///Returns a mutable reference to `modules`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn modules_mut(&mut self) -> &mut Vec<super::Module> {
            &mut self.modules
        }
        ///Sets `modules` with the provided value.
        pub fn set_modules(&mut self, field: Vec<super::Module>) {
            self.modules = field;
        }
        ///Sets `modules` with the provided value.
        pub fn with_modules(mut self, field: Vec<super::Module>) -> Self {
            self.set_modules(field);
            self
        }
        ///Returns the value of `type_origins`, or the default value if `type_origins` is unset.
        pub fn type_origins(&self) -> &[super::TypeOrigin] {
            &self.type_origins
        }
        ///Returns a mutable reference to `type_origins`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn type_origins_mut(&mut self) -> &mut Vec<super::TypeOrigin> {
            &mut self.type_origins
        }
        ///Sets `type_origins` with the provided value.
        pub fn set_type_origins(&mut self, field: Vec<super::TypeOrigin>) {
            self.type_origins = field;
        }
        ///Sets `type_origins` with the provided value.
        pub fn with_type_origins(mut self, field: Vec<super::TypeOrigin>) -> Self {
            self.set_type_origins(field);
            self
        }
        ///Returns the value of `linkage`, or the default value if `linkage` is unset.
        pub fn linkage(&self) -> &[super::Linkage] {
            &self.linkage
        }
        ///Returns a mutable reference to `linkage`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn linkage_mut(&mut self) -> &mut Vec<super::Linkage> {
            &mut self.linkage
        }
        ///Sets `linkage` with the provided value.
        pub fn set_linkage(&mut self, field: Vec<super::Linkage>) {
            self.linkage = field;
        }
        ///Sets `linkage` with the provided value.
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
        ///Sets `kind` with the provided value.
        pub fn with_kind<T: Into<super::package_upgrade_error::PackageUpgradeErrorKind>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_kind(field.into());
            self
        }
        ///If `package_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        ///If `package_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        ///Sets `package_id` with the provided value.
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        ///Sets `package_id` with the provided value.
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field.into());
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///If `policy` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn policy_opt_mut(&mut self) -> Option<&mut u32> {
            self.policy.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `policy`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn policy_mut(&mut self) -> &mut u32 {
            self.policy.get_or_insert_default()
        }
        ///If `policy` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn policy_opt(&self) -> Option<u32> {
            self.policy.as_ref().map(|field| *field)
        }
        ///Sets `policy` with the provided value.
        pub fn set_policy(&mut self, field: u32) {
            self.policy = Some(field);
        }
        ///Sets `policy` with the provided value.
        pub fn with_policy(mut self, field: u32) -> Self {
            self.set_policy(field);
            self
        }
        ///If `ticket_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn ticket_id_opt_mut(&mut self) -> Option<&mut String> {
            self.ticket_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `ticket_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn ticket_id_mut(&mut self) -> &mut String {
            self.ticket_id.get_or_insert_default()
        }
        ///If `ticket_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn ticket_id_opt(&self) -> Option<&str> {
            self.ticket_id.as_ref().map(|field| field as _)
        }
        ///Sets `ticket_id` with the provided value.
        pub fn set_ticket_id<T: Into<String>>(&mut self, field: T) {
            self.ticket_id = Some(field.into().into());
        }
        ///Sets `ticket_id` with the provided value.
        pub fn with_ticket_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_ticket_id(field.into());
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
        ///If `package_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        ///If `package_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        ///Sets `package_id` with the provided value.
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        ///Sets `package_id` with the provided value.
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field.into());
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
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
        ///If `authenticator_data` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn authenticator_data_opt(&self) -> Option<&[u8]> {
            self.authenticator_data.as_ref().map(|field| field as _)
        }
        ///Sets `authenticator_data` with the provided value.
        pub fn set_authenticator_data<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.authenticator_data = Some(field.into().into());
        }
        ///Sets `authenticator_data` with the provided value.
        pub fn with_authenticator_data<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_authenticator_data(field.into());
            self
        }
        ///If `client_data_json` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn client_data_json_opt_mut(&mut self) -> Option<&mut String> {
            self.client_data_json.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `client_data_json`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn client_data_json_mut(&mut self) -> &mut String {
            self.client_data_json.get_or_insert_default()
        }
        ///If `client_data_json` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn client_data_json_opt(&self) -> Option<&str> {
            self.client_data_json.as_ref().map(|field| field as _)
        }
        ///Sets `client_data_json` with the provided value.
        pub fn set_client_data_json<T: Into<String>>(&mut self, field: T) {
            self.client_data_json = Some(field.into().into());
        }
        ///Sets `client_data_json` with the provided value.
        pub fn with_client_data_json<T: Into<String>>(mut self, field: T) -> Self {
            self.set_client_data_json(field.into());
            self
        }
        ///Returns the value of `signature`, or the default value if `signature` is unset.
        pub fn signature(&self) -> &super::SimpleSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::SimpleSignature::default_instance() as _)
        }
        ///If `signature` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn signature_opt_mut(&mut self) -> Option<&mut super::SimpleSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `signature`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn signature_mut(&mut self) -> &mut super::SimpleSignature {
            self.signature.get_or_insert_default()
        }
        ///If `signature` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn signature_opt(&self) -> Option<&super::SimpleSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        ///Sets `signature` with the provided value.
        pub fn set_signature<T: Into<super::SimpleSignature>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        ///Sets `signature` with the provided value.
        pub fn with_signature<T: Into<super::SimpleSignature>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field.into());
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
        ///Returns the value of `inputs`, or the default value if `inputs` is unset.
        pub fn inputs(&self) -> &[super::Input] {
            &self.inputs
        }
        ///Returns a mutable reference to `inputs`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn inputs_mut(&mut self) -> &mut Vec<super::Input> {
            &mut self.inputs
        }
        ///Sets `inputs` with the provided value.
        pub fn set_inputs(&mut self, field: Vec<super::Input>) {
            self.inputs = field;
        }
        ///Sets `inputs` with the provided value.
        pub fn with_inputs(mut self, field: Vec<super::Input>) -> Self {
            self.set_inputs(field);
            self
        }
        ///Returns the value of `commands`, or the default value if `commands` is unset.
        pub fn commands(&self) -> &[super::Command] {
            &self.commands
        }
        ///Returns a mutable reference to `commands`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn commands_mut(&mut self) -> &mut Vec<super::Command> {
            &mut self.commands
        }
        ///Sets `commands` with the provided value.
        pub fn set_commands(&mut self, field: Vec<super::Command>) {
            self.commands = field;
        }
        ///Sets `commands` with the provided value.
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
        ///If `protocol_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn protocol_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.protocol_version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `protocol_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn protocol_version_mut(&mut self) -> &mut u64 {
            self.protocol_version.get_or_insert_default()
        }
        ///If `protocol_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn protocol_version_opt(&self) -> Option<u64> {
            self.protocol_version.as_ref().map(|field| *field)
        }
        ///Sets `protocol_version` with the provided value.
        pub fn set_protocol_version(&mut self, field: u64) {
            self.protocol_version = Some(field);
        }
        ///Sets `protocol_version` with the provided value.
        pub fn with_protocol_version(mut self, field: u64) -> Self {
            self.set_protocol_version(field);
            self
        }
        ///Returns the value of `feature_flags`, or the default value if `feature_flags` is unset.
        pub fn feature_flags(&self) -> &::std::collections::BTreeMap<String, bool> {
            &self.feature_flags
        }
        ///Returns a mutable reference to `feature_flags`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn feature_flags_mut(
            &mut self,
        ) -> &mut ::std::collections::BTreeMap<String, bool> {
            &mut self.feature_flags
        }
        ///Sets `feature_flags` with the provided value.
        pub fn set_feature_flags(
            &mut self,
            field: ::std::collections::BTreeMap<String, bool>,
        ) {
            self.feature_flags = field;
        }
        ///Sets `feature_flags` with the provided value.
        pub fn with_feature_flags(
            mut self,
            field: ::std::collections::BTreeMap<String, bool>,
        ) -> Self {
            self.set_feature_flags(field);
            self
        }
        ///Returns the value of `attributes`, or the default value if `attributes` is unset.
        pub fn attributes(&self) -> &::std::collections::BTreeMap<String, String> {
            &self.attributes
        }
        ///Returns a mutable reference to `attributes`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn attributes_mut(
            &mut self,
        ) -> &mut ::std::collections::BTreeMap<String, String> {
            &mut self.attributes
        }
        ///Sets `attributes` with the provided value.
        pub fn set_attributes(
            &mut self,
            field: ::std::collections::BTreeMap<String, String>,
        ) {
            self.attributes = field;
        }
        ///Sets `attributes` with the provided value.
        pub fn with_attributes(
            mut self,
            field: ::std::collections::BTreeMap<String, String>,
        ) -> Self {
            self.set_attributes(field);
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
        ///Returns the value of `modules`, or the default value if `modules` is unset.
        pub fn modules(&self) -> &[::prost::bytes::Bytes] {
            &self.modules
        }
        ///Returns a mutable reference to `modules`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn modules_mut(&mut self) -> &mut Vec<::prost::bytes::Bytes> {
            &mut self.modules
        }
        ///Sets `modules` with the provided value.
        pub fn set_modules(&mut self, field: Vec<::prost::bytes::Bytes>) {
            self.modules = field;
        }
        ///Sets `modules` with the provided value.
        pub fn with_modules(mut self, field: Vec<::prost::bytes::Bytes>) -> Self {
            self.set_modules(field);
            self
        }
        ///Returns the value of `dependencies`, or the default value if `dependencies` is unset.
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        ///Returns a mutable reference to `dependencies`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn dependencies_mut(&mut self) -> &mut Vec<String> {
            &mut self.dependencies
        }
        ///Sets `dependencies` with the provided value.
        pub fn set_dependencies(&mut self, field: Vec<String>) {
            self.dependencies = field;
        }
        ///Sets `dependencies` with the provided value.
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
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///If `randomness_round` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn randomness_round_opt_mut(&mut self) -> Option<&mut u64> {
            self.randomness_round.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `randomness_round`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn randomness_round_mut(&mut self) -> &mut u64 {
            self.randomness_round.get_or_insert_default()
        }
        ///If `randomness_round` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn randomness_round_opt(&self) -> Option<u64> {
            self.randomness_round.as_ref().map(|field| *field)
        }
        ///Sets `randomness_round` with the provided value.
        pub fn set_randomness_round(&mut self, field: u64) {
            self.randomness_round = Some(field);
        }
        ///Sets `randomness_round` with the provided value.
        pub fn with_randomness_round(mut self, field: u64) -> Self {
            self.set_randomness_round(field);
            self
        }
        ///If `random_bytes` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn random_bytes_opt(&self) -> Option<&[u8]> {
            self.random_bytes.as_ref().map(|field| field as _)
        }
        ///Sets `random_bytes` with the provided value.
        pub fn set_random_bytes<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.random_bytes = Some(field.into().into());
        }
        ///Sets `random_bytes` with the provided value.
        pub fn with_random_bytes<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_random_bytes(field.into());
            self
        }
        ///If `randomness_object_initial_shared_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn randomness_object_initial_shared_version_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.randomness_object_initial_shared_version
                .as_mut()
                .map(|field| field as _)
        }
        ///Returns a mutable reference to `randomness_object_initial_shared_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn randomness_object_initial_shared_version_mut(&mut self) -> &mut u64 {
            self.randomness_object_initial_shared_version.get_or_insert_default()
        }
        ///If `randomness_object_initial_shared_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn randomness_object_initial_shared_version_opt(&self) -> Option<u64> {
            self.randomness_object_initial_shared_version.as_ref().map(|field| *field)
        }
        ///Sets `randomness_object_initial_shared_version` with the provided value.
        pub fn set_randomness_object_initial_shared_version(&mut self, field: u64) {
            self.randomness_object_initial_shared_version = Some(field);
        }
        ///Sets `randomness_object_initial_shared_version` with the provided value.
        pub fn with_randomness_object_initial_shared_version(
            mut self,
            field: u64,
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
                allow_global_pause: None,
                variant: None,
                coin_regulated_state: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::RegulatedCoinMetadata = super::RegulatedCoinMetadata::const_default();
            &DEFAULT
        }
        ///If `id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        ///If `id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        ///Sets `id` with the provided value.
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        ///Sets `id` with the provided value.
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field.into());
            self
        }
        ///If `coin_metadata_object` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn coin_metadata_object_opt_mut(&mut self) -> Option<&mut String> {
            self.coin_metadata_object.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `coin_metadata_object`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coin_metadata_object_mut(&mut self) -> &mut String {
            self.coin_metadata_object.get_or_insert_default()
        }
        ///If `coin_metadata_object` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn coin_metadata_object_opt(&self) -> Option<&str> {
            self.coin_metadata_object.as_ref().map(|field| field as _)
        }
        ///Sets `coin_metadata_object` with the provided value.
        pub fn set_coin_metadata_object<T: Into<String>>(&mut self, field: T) {
            self.coin_metadata_object = Some(field.into().into());
        }
        ///Sets `coin_metadata_object` with the provided value.
        pub fn with_coin_metadata_object<T: Into<String>>(mut self, field: T) -> Self {
            self.set_coin_metadata_object(field.into());
            self
        }
        ///If `deny_cap_object` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn deny_cap_object_opt_mut(&mut self) -> Option<&mut String> {
            self.deny_cap_object.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `deny_cap_object`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn deny_cap_object_mut(&mut self) -> &mut String {
            self.deny_cap_object.get_or_insert_default()
        }
        ///If `deny_cap_object` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn deny_cap_object_opt(&self) -> Option<&str> {
            self.deny_cap_object.as_ref().map(|field| field as _)
        }
        ///Sets `deny_cap_object` with the provided value.
        pub fn set_deny_cap_object<T: Into<String>>(&mut self, field: T) {
            self.deny_cap_object = Some(field.into().into());
        }
        ///Sets `deny_cap_object` with the provided value.
        pub fn with_deny_cap_object<T: Into<String>>(mut self, field: T) -> Self {
            self.set_deny_cap_object(field.into());
            self
        }
        ///If `allow_global_pause` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn allow_global_pause_opt_mut(&mut self) -> Option<&mut bool> {
            self.allow_global_pause.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `allow_global_pause`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn allow_global_pause_mut(&mut self) -> &mut bool {
            self.allow_global_pause.get_or_insert_default()
        }
        ///If `allow_global_pause` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn allow_global_pause_opt(&self) -> Option<bool> {
            self.allow_global_pause.as_ref().map(|field| *field)
        }
        ///Sets `allow_global_pause` with the provided value.
        pub fn set_allow_global_pause(&mut self, field: bool) {
            self.allow_global_pause = Some(field);
        }
        ///Sets `allow_global_pause` with the provided value.
        pub fn with_allow_global_pause(mut self, field: bool) -> Self {
            self.set_allow_global_pause(field);
            self
        }
        ///If `variant` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn variant_opt_mut(&mut self) -> Option<&mut u32> {
            self.variant.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `variant`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn variant_mut(&mut self) -> &mut u32 {
            self.variant.get_or_insert_default()
        }
        ///If `variant` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn variant_opt(&self) -> Option<u32> {
            self.variant.as_ref().map(|field| *field)
        }
        ///Sets `variant` with the provided value.
        pub fn set_variant(&mut self, field: u32) {
            self.variant = Some(field);
        }
        ///Sets `variant` with the provided value.
        pub fn with_variant(mut self, field: u32) -> Self {
            self.set_variant(field);
            self
        }
        ///Sets `coin_regulated_state` with the provided value.
        pub fn with_coin_regulated_state<
            T: Into<super::regulated_coin_metadata::CoinRegulatedState>,
        >(mut self, field: T) -> Self {
            self.set_coin_regulated_state(field.into());
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
        ///If `address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        ///If `address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        ///Sets `address` with the provided value.
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        ///Sets `address` with the provided value.
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field.into());
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
        ///Returns the value of `record`, or the default value if `record` is unset.
        pub fn record(&self) -> &super::NameRecord {
            self.record
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::NameRecord::default_instance() as _)
        }
        ///If `record` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn record_opt_mut(&mut self) -> Option<&mut super::NameRecord> {
            self.record.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `record`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn record_mut(&mut self) -> &mut super::NameRecord {
            self.record.get_or_insert_default()
        }
        ///If `record` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn record_opt(&self) -> Option<&super::NameRecord> {
            self.record.as_ref().map(|field| field as _)
        }
        ///Sets `record` with the provided value.
        pub fn set_record<T: Into<super::NameRecord>>(&mut self, field: T) {
            self.record = Some(field.into().into());
        }
        ///Sets `record` with the provided value.
        pub fn with_record<T: Into<super::NameRecord>>(mut self, field: T) -> Self {
            self.set_record(field.into());
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
        ///Sets `scheme` with the provided value.
        pub fn with_scheme<T: Into<super::SignatureScheme>>(mut self, field: T) -> Self {
            self.set_scheme(field.into());
            self
        }
        ///If `signature` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn signature_opt(&self) -> Option<&[u8]> {
            self.signature.as_ref().map(|field| field as _)
        }
        ///Sets `signature` with the provided value.
        pub fn set_signature<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        ///Sets `signature` with the provided value.
        pub fn with_signature<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field.into());
            self
        }
        ///If `public_key` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn public_key_opt(&self) -> Option<&[u8]> {
            self.public_key.as_ref().map(|field| field as _)
        }
        ///Sets `public_key` with the provided value.
        pub fn set_public_key<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.public_key = Some(field.into().into());
        }
        ///Sets `public_key` with the provided value.
        pub fn with_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_public_key(field.into());
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
        ///Returns the value of `transaction`, or the default value if `transaction` is unset.
        pub fn transaction(&self) -> &super::Transaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Transaction::default_instance() as _)
        }
        ///If `transaction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn transaction_opt_mut(&mut self) -> Option<&mut super::Transaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `transaction`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transaction_mut(&mut self) -> &mut super::Transaction {
            self.transaction.get_or_insert_default()
        }
        ///If `transaction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn transaction_opt(&self) -> Option<&super::Transaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        ///Sets `transaction` with the provided value.
        pub fn set_transaction<T: Into<super::Transaction>>(&mut self, field: T) {
            self.transaction = Some(field.into().into());
        }
        ///Sets `transaction` with the provided value.
        pub fn with_transaction<T: Into<super::Transaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field.into());
            self
        }
        ///If `read_mask` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `read_mask`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        ///If `read_mask` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        ///Sets `read_mask` with the provided value.
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        ///Sets `read_mask` with the provided value.
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field.into());
            self
        }
        ///Sets `checks` with the provided value.
        pub fn with_checks<
            T: Into<super::simulate_transaction_request::TransactionChecks>,
        >(mut self, field: T) -> Self {
            self.set_checks(field.into());
            self
        }
        ///If `do_gas_selection` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn do_gas_selection_opt_mut(&mut self) -> Option<&mut bool> {
            self.do_gas_selection.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `do_gas_selection`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn do_gas_selection_mut(&mut self) -> &mut bool {
            self.do_gas_selection.get_or_insert_default()
        }
        ///If `do_gas_selection` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn do_gas_selection_opt(&self) -> Option<bool> {
            self.do_gas_selection.as_ref().map(|field| *field)
        }
        ///Sets `do_gas_selection` with the provided value.
        pub fn set_do_gas_selection(&mut self, field: bool) {
            self.do_gas_selection = Some(field);
        }
        ///Sets `do_gas_selection` with the provided value.
        pub fn with_do_gas_selection(mut self, field: bool) -> Self {
            self.set_do_gas_selection(field);
            self
        }
    }
    impl super::SimulateTransactionResponse {
        pub const fn const_default() -> Self {
            Self {
                transaction: None,
                command_outputs: Vec::new(),
                suggested_gas_price: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SimulateTransactionResponse = super::SimulateTransactionResponse::const_default();
            &DEFAULT
        }
        ///Returns the value of `transaction`, or the default value if `transaction` is unset.
        pub fn transaction(&self) -> &super::ExecutedTransaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ExecutedTransaction::default_instance() as _)
        }
        ///If `transaction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn transaction_opt_mut(
            &mut self,
        ) -> Option<&mut super::ExecutedTransaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `transaction`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transaction_mut(&mut self) -> &mut super::ExecutedTransaction {
            self.transaction.get_or_insert_default()
        }
        ///If `transaction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn transaction_opt(&self) -> Option<&super::ExecutedTransaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        ///Sets `transaction` with the provided value.
        pub fn set_transaction<T: Into<super::ExecutedTransaction>>(
            &mut self,
            field: T,
        ) {
            self.transaction = Some(field.into().into());
        }
        ///Sets `transaction` with the provided value.
        pub fn with_transaction<T: Into<super::ExecutedTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field.into());
            self
        }
        ///Returns the value of `command_outputs`, or the default value if `command_outputs` is unset.
        pub fn command_outputs(&self) -> &[super::CommandResult] {
            &self.command_outputs
        }
        ///Returns a mutable reference to `command_outputs`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn command_outputs_mut(&mut self) -> &mut Vec<super::CommandResult> {
            &mut self.command_outputs
        }
        ///Sets `command_outputs` with the provided value.
        pub fn set_command_outputs(&mut self, field: Vec<super::CommandResult>) {
            self.command_outputs = field;
        }
        ///Sets `command_outputs` with the provided value.
        pub fn with_command_outputs(mut self, field: Vec<super::CommandResult>) -> Self {
            self.set_command_outputs(field);
            self
        }
        ///If `suggested_gas_price` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn suggested_gas_price_opt_mut(&mut self) -> Option<&mut u64> {
            self.suggested_gas_price.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `suggested_gas_price`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn suggested_gas_price_mut(&mut self) -> &mut u64 {
            self.suggested_gas_price.get_or_insert_default()
        }
        ///If `suggested_gas_price` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn suggested_gas_price_opt(&self) -> Option<u64> {
            self.suggested_gas_price.as_ref().map(|field| *field)
        }
        ///Sets `suggested_gas_price` with the provided value.
        pub fn set_suggested_gas_price(&mut self, field: u64) {
            self.suggested_gas_price = Some(field);
        }
        ///Sets `suggested_gas_price` with the provided value.
        pub fn with_suggested_gas_price(mut self, field: u64) -> Self {
            self.set_suggested_gas_price(field);
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
        ///If `size` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn size_opt_mut(&mut self) -> Option<&mut u64> {
            self.size.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `size`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn size_mut(&mut self) -> &mut u64 {
            self.size.get_or_insert_default()
        }
        ///If `size` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn size_opt(&self) -> Option<u64> {
            self.size.as_ref().map(|field| *field)
        }
        ///Sets `size` with the provided value.
        pub fn set_size(&mut self, field: u64) {
            self.size = Some(field);
        }
        ///Sets `size` with the provided value.
        pub fn with_size(mut self, field: u64) -> Self {
            self.set_size(field);
            self
        }
        ///If `max_size` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn max_size_opt_mut(&mut self) -> Option<&mut u64> {
            self.max_size.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `max_size`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn max_size_mut(&mut self) -> &mut u64 {
            self.max_size.get_or_insert_default()
        }
        ///If `max_size` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn max_size_opt(&self) -> Option<u64> {
            self.max_size.as_ref().map(|field| *field)
        }
        ///Sets `max_size` with the provided value.
        pub fn set_max_size(&mut self, field: u64) {
            self.max_size = Some(field);
        }
        ///Sets `max_size` with the provided value.
        pub fn with_max_size(mut self, field: u64) -> Self {
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
        ///Returns the value of `coin`, or the default value if `coin` is unset.
        pub fn coin(&self) -> &super::Argument {
            self.coin
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Argument::default_instance() as _)
        }
        ///If `coin` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn coin_opt_mut(&mut self) -> Option<&mut super::Argument> {
            self.coin.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `coin`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn coin_mut(&mut self) -> &mut super::Argument {
            self.coin.get_or_insert_default()
        }
        ///If `coin` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn coin_opt(&self) -> Option<&super::Argument> {
            self.coin.as_ref().map(|field| field as _)
        }
        ///Sets `coin` with the provided value.
        pub fn set_coin<T: Into<super::Argument>>(&mut self, field: T) {
            self.coin = Some(field.into().into());
        }
        ///Sets `coin` with the provided value.
        pub fn with_coin<T: Into<super::Argument>>(mut self, field: T) -> Self {
            self.set_coin(field.into());
            self
        }
        ///Returns the value of `amounts`, or the default value if `amounts` is unset.
        pub fn amounts(&self) -> &[super::Argument] {
            &self.amounts
        }
        ///Returns a mutable reference to `amounts`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn amounts_mut(&mut self) -> &mut Vec<super::Argument> {
            &mut self.amounts
        }
        ///Sets `amounts` with the provided value.
        pub fn set_amounts(&mut self, field: Vec<super::Argument>) {
            self.amounts = field;
        }
        ///Sets `amounts` with the provided value.
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
        ///If `balance` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.balance.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `balance`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn balance_mut(&mut self) -> &mut u64 {
            self.balance.get_or_insert_default()
        }
        ///If `balance` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn balance_opt(&self) -> Option<u64> {
            self.balance.as_ref().map(|field| *field)
        }
        ///Sets `balance` with the provided value.
        pub fn set_balance(&mut self, field: u64) {
            self.balance = Some(field);
        }
        ///Sets `balance` with the provided value.
        pub fn with_balance(mut self, field: u64) -> Self {
            self.set_balance(field);
            self
        }
        ///If `distribution_counter` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn distribution_counter_opt_mut(&mut self) -> Option<&mut u64> {
            self.distribution_counter.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `distribution_counter`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn distribution_counter_mut(&mut self) -> &mut u64 {
            self.distribution_counter.get_or_insert_default()
        }
        ///If `distribution_counter` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn distribution_counter_opt(&self) -> Option<u64> {
            self.distribution_counter.as_ref().map(|field| *field)
        }
        ///Sets `distribution_counter` with the provided value.
        pub fn set_distribution_counter(&mut self, field: u64) {
            self.distribution_counter = Some(field);
        }
        ///Sets `distribution_counter` with the provided value.
        pub fn with_distribution_counter(mut self, field: u64) -> Self {
            self.set_distribution_counter(field);
            self
        }
        ///If `current_distribution_amount` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn current_distribution_amount_opt_mut(&mut self) -> Option<&mut u64> {
            self.current_distribution_amount.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `current_distribution_amount`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn current_distribution_amount_mut(&mut self) -> &mut u64 {
            self.current_distribution_amount.get_or_insert_default()
        }
        ///If `current_distribution_amount` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn current_distribution_amount_opt(&self) -> Option<u64> {
            self.current_distribution_amount.as_ref().map(|field| *field)
        }
        ///Sets `current_distribution_amount` with the provided value.
        pub fn set_current_distribution_amount(&mut self, field: u64) {
            self.current_distribution_amount = Some(field);
        }
        ///Sets `current_distribution_amount` with the provided value.
        pub fn with_current_distribution_amount(mut self, field: u64) -> Self {
            self.set_current_distribution_amount(field);
            self
        }
        ///If `stake_subsidy_period_length` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn stake_subsidy_period_length_opt_mut(&mut self) -> Option<&mut u64> {
            self.stake_subsidy_period_length.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `stake_subsidy_period_length`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn stake_subsidy_period_length_mut(&mut self) -> &mut u64 {
            self.stake_subsidy_period_length.get_or_insert_default()
        }
        ///If `stake_subsidy_period_length` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn stake_subsidy_period_length_opt(&self) -> Option<u64> {
            self.stake_subsidy_period_length.as_ref().map(|field| *field)
        }
        ///Sets `stake_subsidy_period_length` with the provided value.
        pub fn set_stake_subsidy_period_length(&mut self, field: u64) {
            self.stake_subsidy_period_length = Some(field);
        }
        ///Sets `stake_subsidy_period_length` with the provided value.
        pub fn with_stake_subsidy_period_length(mut self, field: u64) -> Self {
            self.set_stake_subsidy_period_length(field);
            self
        }
        ///If `stake_subsidy_decrease_rate` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn stake_subsidy_decrease_rate_opt_mut(&mut self) -> Option<&mut u32> {
            self.stake_subsidy_decrease_rate.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `stake_subsidy_decrease_rate`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn stake_subsidy_decrease_rate_mut(&mut self) -> &mut u32 {
            self.stake_subsidy_decrease_rate.get_or_insert_default()
        }
        ///If `stake_subsidy_decrease_rate` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn stake_subsidy_decrease_rate_opt(&self) -> Option<u32> {
            self.stake_subsidy_decrease_rate.as_ref().map(|field| *field)
        }
        ///Sets `stake_subsidy_decrease_rate` with the provided value.
        pub fn set_stake_subsidy_decrease_rate(&mut self, field: u32) {
            self.stake_subsidy_decrease_rate = Some(field);
        }
        ///Sets `stake_subsidy_decrease_rate` with the provided value.
        pub fn with_stake_subsidy_decrease_rate(mut self, field: u32) -> Self {
            self.set_stake_subsidy_decrease_rate(field);
            self
        }
        ///Returns the value of `extra_fields`, or the default value if `extra_fields` is unset.
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `extra_fields` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `extra_fields`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        ///If `extra_fields` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        ///Sets `extra_fields` with the provided value.
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        ///Sets `extra_fields` with the provided value.
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field.into());
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
        ///If `id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn id_opt_mut(&mut self) -> Option<&mut String> {
            self.id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn id_mut(&mut self) -> &mut String {
            self.id.get_or_insert_default()
        }
        ///If `id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn id_opt(&self) -> Option<&str> {
            self.id.as_ref().map(|field| field as _)
        }
        ///Sets `id` with the provided value.
        pub fn set_id<T: Into<String>>(&mut self, field: T) {
            self.id = Some(field.into().into());
        }
        ///Sets `id` with the provided value.
        pub fn with_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_id(field.into());
            self
        }
        ///If `activation_epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn activation_epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.activation_epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `activation_epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn activation_epoch_mut(&mut self) -> &mut u64 {
            self.activation_epoch.get_or_insert_default()
        }
        ///If `activation_epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn activation_epoch_opt(&self) -> Option<u64> {
            self.activation_epoch.as_ref().map(|field| *field)
        }
        ///Sets `activation_epoch` with the provided value.
        pub fn set_activation_epoch(&mut self, field: u64) {
            self.activation_epoch = Some(field);
        }
        ///Sets `activation_epoch` with the provided value.
        pub fn with_activation_epoch(mut self, field: u64) -> Self {
            self.set_activation_epoch(field);
            self
        }
        ///If `deactivation_epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn deactivation_epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.deactivation_epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `deactivation_epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn deactivation_epoch_mut(&mut self) -> &mut u64 {
            self.deactivation_epoch.get_or_insert_default()
        }
        ///If `deactivation_epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn deactivation_epoch_opt(&self) -> Option<u64> {
            self.deactivation_epoch.as_ref().map(|field| *field)
        }
        ///Sets `deactivation_epoch` with the provided value.
        pub fn set_deactivation_epoch(&mut self, field: u64) {
            self.deactivation_epoch = Some(field);
        }
        ///Sets `deactivation_epoch` with the provided value.
        pub fn with_deactivation_epoch(mut self, field: u64) -> Self {
            self.set_deactivation_epoch(field);
            self
        }
        ///If `sui_balance` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn sui_balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.sui_balance.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `sui_balance`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn sui_balance_mut(&mut self) -> &mut u64 {
            self.sui_balance.get_or_insert_default()
        }
        ///If `sui_balance` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn sui_balance_opt(&self) -> Option<u64> {
            self.sui_balance.as_ref().map(|field| *field)
        }
        ///Sets `sui_balance` with the provided value.
        pub fn set_sui_balance(&mut self, field: u64) {
            self.sui_balance = Some(field);
        }
        ///Sets `sui_balance` with the provided value.
        pub fn with_sui_balance(mut self, field: u64) -> Self {
            self.set_sui_balance(field);
            self
        }
        ///If `rewards_pool` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn rewards_pool_opt_mut(&mut self) -> Option<&mut u64> {
            self.rewards_pool.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `rewards_pool`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn rewards_pool_mut(&mut self) -> &mut u64 {
            self.rewards_pool.get_or_insert_default()
        }
        ///If `rewards_pool` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn rewards_pool_opt(&self) -> Option<u64> {
            self.rewards_pool.as_ref().map(|field| *field)
        }
        ///Sets `rewards_pool` with the provided value.
        pub fn set_rewards_pool(&mut self, field: u64) {
            self.rewards_pool = Some(field);
        }
        ///Sets `rewards_pool` with the provided value.
        pub fn with_rewards_pool(mut self, field: u64) -> Self {
            self.set_rewards_pool(field);
            self
        }
        ///If `pool_token_balance` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn pool_token_balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.pool_token_balance.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `pool_token_balance`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn pool_token_balance_mut(&mut self) -> &mut u64 {
            self.pool_token_balance.get_or_insert_default()
        }
        ///If `pool_token_balance` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn pool_token_balance_opt(&self) -> Option<u64> {
            self.pool_token_balance.as_ref().map(|field| *field)
        }
        ///Sets `pool_token_balance` with the provided value.
        pub fn set_pool_token_balance(&mut self, field: u64) {
            self.pool_token_balance = Some(field);
        }
        ///Sets `pool_token_balance` with the provided value.
        pub fn with_pool_token_balance(mut self, field: u64) -> Self {
            self.set_pool_token_balance(field);
            self
        }
        ///Returns the value of `exchange_rates`, or the default value if `exchange_rates` is unset.
        pub fn exchange_rates(&self) -> &super::MoveTable {
            self.exchange_rates
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `exchange_rates` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn exchange_rates_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.exchange_rates.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `exchange_rates`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn exchange_rates_mut(&mut self) -> &mut super::MoveTable {
            self.exchange_rates.get_or_insert_default()
        }
        ///If `exchange_rates` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn exchange_rates_opt(&self) -> Option<&super::MoveTable> {
            self.exchange_rates.as_ref().map(|field| field as _)
        }
        ///Sets `exchange_rates` with the provided value.
        pub fn set_exchange_rates<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.exchange_rates = Some(field.into().into());
        }
        ///Sets `exchange_rates` with the provided value.
        pub fn with_exchange_rates<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_exchange_rates(field.into());
            self
        }
        ///If `pending_stake` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn pending_stake_opt_mut(&mut self) -> Option<&mut u64> {
            self.pending_stake.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `pending_stake`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn pending_stake_mut(&mut self) -> &mut u64 {
            self.pending_stake.get_or_insert_default()
        }
        ///If `pending_stake` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn pending_stake_opt(&self) -> Option<u64> {
            self.pending_stake.as_ref().map(|field| *field)
        }
        ///Sets `pending_stake` with the provided value.
        pub fn set_pending_stake(&mut self, field: u64) {
            self.pending_stake = Some(field);
        }
        ///Sets `pending_stake` with the provided value.
        pub fn with_pending_stake(mut self, field: u64) -> Self {
            self.set_pending_stake(field);
            self
        }
        ///If `pending_total_sui_withdraw` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn pending_total_sui_withdraw_opt_mut(&mut self) -> Option<&mut u64> {
            self.pending_total_sui_withdraw.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `pending_total_sui_withdraw`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn pending_total_sui_withdraw_mut(&mut self) -> &mut u64 {
            self.pending_total_sui_withdraw.get_or_insert_default()
        }
        ///If `pending_total_sui_withdraw` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn pending_total_sui_withdraw_opt(&self) -> Option<u64> {
            self.pending_total_sui_withdraw.as_ref().map(|field| *field)
        }
        ///Sets `pending_total_sui_withdraw` with the provided value.
        pub fn set_pending_total_sui_withdraw(&mut self, field: u64) {
            self.pending_total_sui_withdraw = Some(field);
        }
        ///Sets `pending_total_sui_withdraw` with the provided value.
        pub fn with_pending_total_sui_withdraw(mut self, field: u64) -> Self {
            self.set_pending_total_sui_withdraw(field);
            self
        }
        ///If `pending_pool_token_withdraw` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn pending_pool_token_withdraw_opt_mut(&mut self) -> Option<&mut u64> {
            self.pending_pool_token_withdraw.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `pending_pool_token_withdraw`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn pending_pool_token_withdraw_mut(&mut self) -> &mut u64 {
            self.pending_pool_token_withdraw.get_or_insert_default()
        }
        ///If `pending_pool_token_withdraw` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn pending_pool_token_withdraw_opt(&self) -> Option<u64> {
            self.pending_pool_token_withdraw.as_ref().map(|field| *field)
        }
        ///Sets `pending_pool_token_withdraw` with the provided value.
        pub fn set_pending_pool_token_withdraw(&mut self, field: u64) {
            self.pending_pool_token_withdraw = Some(field);
        }
        ///Sets `pending_pool_token_withdraw` with the provided value.
        pub fn with_pending_pool_token_withdraw(mut self, field: u64) -> Self {
            self.set_pending_pool_token_withdraw(field);
            self
        }
        ///Returns the value of `extra_fields`, or the default value if `extra_fields` is unset.
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `extra_fields` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `extra_fields`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        ///If `extra_fields` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        ///Sets `extra_fields` with the provided value.
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        ///Sets `extra_fields` with the provided value.
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field.into());
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
        ///If `total_object_storage_rebates` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn total_object_storage_rebates_opt_mut(&mut self) -> Option<&mut u64> {
            self.total_object_storage_rebates.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `total_object_storage_rebates`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn total_object_storage_rebates_mut(&mut self) -> &mut u64 {
            self.total_object_storage_rebates.get_or_insert_default()
        }
        ///If `total_object_storage_rebates` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn total_object_storage_rebates_opt(&self) -> Option<u64> {
            self.total_object_storage_rebates.as_ref().map(|field| *field)
        }
        ///Sets `total_object_storage_rebates` with the provided value.
        pub fn set_total_object_storage_rebates(&mut self, field: u64) {
            self.total_object_storage_rebates = Some(field);
        }
        ///Sets `total_object_storage_rebates` with the provided value.
        pub fn with_total_object_storage_rebates(mut self, field: u64) -> Self {
            self.set_total_object_storage_rebates(field);
            self
        }
        ///If `non_refundable_balance` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn non_refundable_balance_opt_mut(&mut self) -> Option<&mut u64> {
            self.non_refundable_balance.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `non_refundable_balance`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn non_refundable_balance_mut(&mut self) -> &mut u64 {
            self.non_refundable_balance.get_or_insert_default()
        }
        ///If `non_refundable_balance` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn non_refundable_balance_opt(&self) -> Option<u64> {
            self.non_refundable_balance.as_ref().map(|field| *field)
        }
        ///Sets `non_refundable_balance` with the provided value.
        pub fn set_non_refundable_balance(&mut self, field: u64) {
            self.non_refundable_balance = Some(field);
        }
        ///Sets `non_refundable_balance` with the provided value.
        pub fn with_non_refundable_balance(mut self, field: u64) -> Self {
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
        ///If `read_mask` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn read_mask_opt_mut(&mut self) -> Option<&mut ::prost_types::FieldMask> {
            self.read_mask.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `read_mask`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn read_mask_mut(&mut self) -> &mut ::prost_types::FieldMask {
            self.read_mask.get_or_insert_default()
        }
        ///If `read_mask` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn read_mask_opt(&self) -> Option<&::prost_types::FieldMask> {
            self.read_mask.as_ref().map(|field| field as _)
        }
        ///Sets `read_mask` with the provided value.
        pub fn set_read_mask<T: Into<::prost_types::FieldMask>>(&mut self, field: T) {
            self.read_mask = Some(field.into().into());
        }
        ///Sets `read_mask` with the provided value.
        pub fn with_read_mask<T: Into<::prost_types::FieldMask>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_read_mask(field.into());
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
        ///If `cursor` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn cursor_opt_mut(&mut self) -> Option<&mut u64> {
            self.cursor.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `cursor`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn cursor_mut(&mut self) -> &mut u64 {
            self.cursor.get_or_insert_default()
        }
        ///If `cursor` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn cursor_opt(&self) -> Option<u64> {
            self.cursor.as_ref().map(|field| *field)
        }
        ///Sets `cursor` with the provided value.
        pub fn set_cursor(&mut self, field: u64) {
            self.cursor = Some(field);
        }
        ///Sets `cursor` with the provided value.
        pub fn with_cursor(mut self, field: u64) -> Self {
            self.set_cursor(field);
            self
        }
        ///Returns the value of `checkpoint`, or the default value if `checkpoint` is unset.
        pub fn checkpoint(&self) -> &super::Checkpoint {
            self.checkpoint
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Checkpoint::default_instance() as _)
        }
        ///If `checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn checkpoint_opt_mut(&mut self) -> Option<&mut super::Checkpoint> {
            self.checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn checkpoint_mut(&mut self) -> &mut super::Checkpoint {
            self.checkpoint.get_or_insert_default()
        }
        ///If `checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn checkpoint_opt(&self) -> Option<&super::Checkpoint> {
            self.checkpoint.as_ref().map(|field| field as _)
        }
        ///Sets `checkpoint` with the provided value.
        pub fn set_checkpoint<T: Into<super::Checkpoint>>(&mut self, field: T) {
            self.checkpoint = Some(field.into().into());
        }
        ///Sets `checkpoint` with the provided value.
        pub fn with_checkpoint<T: Into<super::Checkpoint>>(mut self, field: T) -> Self {
            self.set_checkpoint(field.into());
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
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
            self.set_version(field);
            self
        }
        ///Returns the value of `modules`, or the default value if `modules` is unset.
        pub fn modules(&self) -> &[::prost::bytes::Bytes] {
            &self.modules
        }
        ///Returns a mutable reference to `modules`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn modules_mut(&mut self) -> &mut Vec<::prost::bytes::Bytes> {
            &mut self.modules
        }
        ///Sets `modules` with the provided value.
        pub fn set_modules(&mut self, field: Vec<::prost::bytes::Bytes>) {
            self.modules = field;
        }
        ///Sets `modules` with the provided value.
        pub fn with_modules(mut self, field: Vec<::prost::bytes::Bytes>) -> Self {
            self.set_modules(field);
            self
        }
        ///Returns the value of `dependencies`, or the default value if `dependencies` is unset.
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        ///Returns a mutable reference to `dependencies`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn dependencies_mut(&mut self) -> &mut Vec<String> {
            &mut self.dependencies
        }
        ///Sets `dependencies` with the provided value.
        pub fn set_dependencies(&mut self, field: Vec<String>) {
            self.dependencies = field;
        }
        ///Sets `dependencies` with the provided value.
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
        ///If `epoch_duration_ms` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_duration_ms_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch_duration_ms.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch_duration_ms`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_duration_ms_mut(&mut self) -> &mut u64 {
            self.epoch_duration_ms.get_or_insert_default()
        }
        ///If `epoch_duration_ms` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_duration_ms_opt(&self) -> Option<u64> {
            self.epoch_duration_ms.as_ref().map(|field| *field)
        }
        ///Sets `epoch_duration_ms` with the provided value.
        pub fn set_epoch_duration_ms(&mut self, field: u64) {
            self.epoch_duration_ms = Some(field);
        }
        ///Sets `epoch_duration_ms` with the provided value.
        pub fn with_epoch_duration_ms(mut self, field: u64) -> Self {
            self.set_epoch_duration_ms(field);
            self
        }
        ///If `stake_subsidy_start_epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn stake_subsidy_start_epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.stake_subsidy_start_epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `stake_subsidy_start_epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn stake_subsidy_start_epoch_mut(&mut self) -> &mut u64 {
            self.stake_subsidy_start_epoch.get_or_insert_default()
        }
        ///If `stake_subsidy_start_epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn stake_subsidy_start_epoch_opt(&self) -> Option<u64> {
            self.stake_subsidy_start_epoch.as_ref().map(|field| *field)
        }
        ///Sets `stake_subsidy_start_epoch` with the provided value.
        pub fn set_stake_subsidy_start_epoch(&mut self, field: u64) {
            self.stake_subsidy_start_epoch = Some(field);
        }
        ///Sets `stake_subsidy_start_epoch` with the provided value.
        pub fn with_stake_subsidy_start_epoch(mut self, field: u64) -> Self {
            self.set_stake_subsidy_start_epoch(field);
            self
        }
        ///If `min_validator_count` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn min_validator_count_opt_mut(&mut self) -> Option<&mut u64> {
            self.min_validator_count.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `min_validator_count`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn min_validator_count_mut(&mut self) -> &mut u64 {
            self.min_validator_count.get_or_insert_default()
        }
        ///If `min_validator_count` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn min_validator_count_opt(&self) -> Option<u64> {
            self.min_validator_count.as_ref().map(|field| *field)
        }
        ///Sets `min_validator_count` with the provided value.
        pub fn set_min_validator_count(&mut self, field: u64) {
            self.min_validator_count = Some(field);
        }
        ///Sets `min_validator_count` with the provided value.
        pub fn with_min_validator_count(mut self, field: u64) -> Self {
            self.set_min_validator_count(field);
            self
        }
        ///If `max_validator_count` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn max_validator_count_opt_mut(&mut self) -> Option<&mut u64> {
            self.max_validator_count.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `max_validator_count`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn max_validator_count_mut(&mut self) -> &mut u64 {
            self.max_validator_count.get_or_insert_default()
        }
        ///If `max_validator_count` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn max_validator_count_opt(&self) -> Option<u64> {
            self.max_validator_count.as_ref().map(|field| *field)
        }
        ///Sets `max_validator_count` with the provided value.
        pub fn set_max_validator_count(&mut self, field: u64) {
            self.max_validator_count = Some(field);
        }
        ///Sets `max_validator_count` with the provided value.
        pub fn with_max_validator_count(mut self, field: u64) -> Self {
            self.set_max_validator_count(field);
            self
        }
        ///If `min_validator_joining_stake` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn min_validator_joining_stake_opt_mut(&mut self) -> Option<&mut u64> {
            self.min_validator_joining_stake.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `min_validator_joining_stake`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn min_validator_joining_stake_mut(&mut self) -> &mut u64 {
            self.min_validator_joining_stake.get_or_insert_default()
        }
        ///If `min_validator_joining_stake` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn min_validator_joining_stake_opt(&self) -> Option<u64> {
            self.min_validator_joining_stake.as_ref().map(|field| *field)
        }
        ///Sets `min_validator_joining_stake` with the provided value.
        pub fn set_min_validator_joining_stake(&mut self, field: u64) {
            self.min_validator_joining_stake = Some(field);
        }
        ///Sets `min_validator_joining_stake` with the provided value.
        pub fn with_min_validator_joining_stake(mut self, field: u64) -> Self {
            self.set_min_validator_joining_stake(field);
            self
        }
        ///If `validator_low_stake_threshold` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn validator_low_stake_threshold_opt_mut(&mut self) -> Option<&mut u64> {
            self.validator_low_stake_threshold.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `validator_low_stake_threshold`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn validator_low_stake_threshold_mut(&mut self) -> &mut u64 {
            self.validator_low_stake_threshold.get_or_insert_default()
        }
        ///If `validator_low_stake_threshold` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn validator_low_stake_threshold_opt(&self) -> Option<u64> {
            self.validator_low_stake_threshold.as_ref().map(|field| *field)
        }
        ///Sets `validator_low_stake_threshold` with the provided value.
        pub fn set_validator_low_stake_threshold(&mut self, field: u64) {
            self.validator_low_stake_threshold = Some(field);
        }
        ///Sets `validator_low_stake_threshold` with the provided value.
        pub fn with_validator_low_stake_threshold(mut self, field: u64) -> Self {
            self.set_validator_low_stake_threshold(field);
            self
        }
        ///If `validator_very_low_stake_threshold` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn validator_very_low_stake_threshold_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.validator_very_low_stake_threshold.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `validator_very_low_stake_threshold`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn validator_very_low_stake_threshold_mut(&mut self) -> &mut u64 {
            self.validator_very_low_stake_threshold.get_or_insert_default()
        }
        ///If `validator_very_low_stake_threshold` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn validator_very_low_stake_threshold_opt(&self) -> Option<u64> {
            self.validator_very_low_stake_threshold.as_ref().map(|field| *field)
        }
        ///Sets `validator_very_low_stake_threshold` with the provided value.
        pub fn set_validator_very_low_stake_threshold(&mut self, field: u64) {
            self.validator_very_low_stake_threshold = Some(field);
        }
        ///Sets `validator_very_low_stake_threshold` with the provided value.
        pub fn with_validator_very_low_stake_threshold(mut self, field: u64) -> Self {
            self.set_validator_very_low_stake_threshold(field);
            self
        }
        ///If `validator_low_stake_grace_period` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn validator_low_stake_grace_period_opt_mut(&mut self) -> Option<&mut u64> {
            self.validator_low_stake_grace_period.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `validator_low_stake_grace_period`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn validator_low_stake_grace_period_mut(&mut self) -> &mut u64 {
            self.validator_low_stake_grace_period.get_or_insert_default()
        }
        ///If `validator_low_stake_grace_period` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn validator_low_stake_grace_period_opt(&self) -> Option<u64> {
            self.validator_low_stake_grace_period.as_ref().map(|field| *field)
        }
        ///Sets `validator_low_stake_grace_period` with the provided value.
        pub fn set_validator_low_stake_grace_period(&mut self, field: u64) {
            self.validator_low_stake_grace_period = Some(field);
        }
        ///Sets `validator_low_stake_grace_period` with the provided value.
        pub fn with_validator_low_stake_grace_period(mut self, field: u64) -> Self {
            self.set_validator_low_stake_grace_period(field);
            self
        }
        ///Returns the value of `extra_fields`, or the default value if `extra_fields` is unset.
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `extra_fields` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `extra_fields`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        ///If `extra_fields` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        ///Sets `extra_fields` with the provided value.
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        ///Sets `extra_fields` with the provided value.
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field.into());
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
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
            self.set_version(field);
            self
        }
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///If `protocol_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn protocol_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.protocol_version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `protocol_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn protocol_version_mut(&mut self) -> &mut u64 {
            self.protocol_version.get_or_insert_default()
        }
        ///If `protocol_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn protocol_version_opt(&self) -> Option<u64> {
            self.protocol_version.as_ref().map(|field| *field)
        }
        ///Sets `protocol_version` with the provided value.
        pub fn set_protocol_version(&mut self, field: u64) {
            self.protocol_version = Some(field);
        }
        ///Sets `protocol_version` with the provided value.
        pub fn with_protocol_version(mut self, field: u64) -> Self {
            self.set_protocol_version(field);
            self
        }
        ///Returns the value of `validators`, or the default value if `validators` is unset.
        pub fn validators(&self) -> &super::ValidatorSet {
            self.validators
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ValidatorSet::default_instance() as _)
        }
        ///If `validators` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn validators_opt_mut(&mut self) -> Option<&mut super::ValidatorSet> {
            self.validators.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `validators`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn validators_mut(&mut self) -> &mut super::ValidatorSet {
            self.validators.get_or_insert_default()
        }
        ///If `validators` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn validators_opt(&self) -> Option<&super::ValidatorSet> {
            self.validators.as_ref().map(|field| field as _)
        }
        ///Sets `validators` with the provided value.
        pub fn set_validators<T: Into<super::ValidatorSet>>(&mut self, field: T) {
            self.validators = Some(field.into().into());
        }
        ///Sets `validators` with the provided value.
        pub fn with_validators<T: Into<super::ValidatorSet>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_validators(field.into());
            self
        }
        ///Returns the value of `storage_fund`, or the default value if `storage_fund` is unset.
        pub fn storage_fund(&self) -> &super::StorageFund {
            self.storage_fund
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::StorageFund::default_instance() as _)
        }
        ///If `storage_fund` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn storage_fund_opt_mut(&mut self) -> Option<&mut super::StorageFund> {
            self.storage_fund.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `storage_fund`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn storage_fund_mut(&mut self) -> &mut super::StorageFund {
            self.storage_fund.get_or_insert_default()
        }
        ///If `storage_fund` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn storage_fund_opt(&self) -> Option<&super::StorageFund> {
            self.storage_fund.as_ref().map(|field| field as _)
        }
        ///Sets `storage_fund` with the provided value.
        pub fn set_storage_fund<T: Into<super::StorageFund>>(&mut self, field: T) {
            self.storage_fund = Some(field.into().into());
        }
        ///Sets `storage_fund` with the provided value.
        pub fn with_storage_fund<T: Into<super::StorageFund>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_storage_fund(field.into());
            self
        }
        ///Returns the value of `parameters`, or the default value if `parameters` is unset.
        pub fn parameters(&self) -> &super::SystemParameters {
            self.parameters
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::SystemParameters::default_instance() as _)
        }
        ///If `parameters` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn parameters_opt_mut(&mut self) -> Option<&mut super::SystemParameters> {
            self.parameters.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `parameters`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn parameters_mut(&mut self) -> &mut super::SystemParameters {
            self.parameters.get_or_insert_default()
        }
        ///If `parameters` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn parameters_opt(&self) -> Option<&super::SystemParameters> {
            self.parameters.as_ref().map(|field| field as _)
        }
        ///Sets `parameters` with the provided value.
        pub fn set_parameters<T: Into<super::SystemParameters>>(&mut self, field: T) {
            self.parameters = Some(field.into().into());
        }
        ///Sets `parameters` with the provided value.
        pub fn with_parameters<T: Into<super::SystemParameters>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_parameters(field.into());
            self
        }
        ///If `reference_gas_price` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn reference_gas_price_opt_mut(&mut self) -> Option<&mut u64> {
            self.reference_gas_price.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `reference_gas_price`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn reference_gas_price_mut(&mut self) -> &mut u64 {
            self.reference_gas_price.get_or_insert_default()
        }
        ///If `reference_gas_price` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn reference_gas_price_opt(&self) -> Option<u64> {
            self.reference_gas_price.as_ref().map(|field| *field)
        }
        ///Sets `reference_gas_price` with the provided value.
        pub fn set_reference_gas_price(&mut self, field: u64) {
            self.reference_gas_price = Some(field);
        }
        ///Sets `reference_gas_price` with the provided value.
        pub fn with_reference_gas_price(mut self, field: u64) -> Self {
            self.set_reference_gas_price(field);
            self
        }
        ///Returns the value of `validator_report_records`, or the default value if `validator_report_records` is unset.
        pub fn validator_report_records(&self) -> &[super::ValidatorReportRecord] {
            &self.validator_report_records
        }
        ///Returns a mutable reference to `validator_report_records`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn validator_report_records_mut(
            &mut self,
        ) -> &mut Vec<super::ValidatorReportRecord> {
            &mut self.validator_report_records
        }
        ///Sets `validator_report_records` with the provided value.
        pub fn set_validator_report_records(
            &mut self,
            field: Vec<super::ValidatorReportRecord>,
        ) {
            self.validator_report_records = field;
        }
        ///Sets `validator_report_records` with the provided value.
        pub fn with_validator_report_records(
            mut self,
            field: Vec<super::ValidatorReportRecord>,
        ) -> Self {
            self.set_validator_report_records(field);
            self
        }
        ///Returns the value of `stake_subsidy`, or the default value if `stake_subsidy` is unset.
        pub fn stake_subsidy(&self) -> &super::StakeSubsidy {
            self.stake_subsidy
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::StakeSubsidy::default_instance() as _)
        }
        ///If `stake_subsidy` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn stake_subsidy_opt_mut(&mut self) -> Option<&mut super::StakeSubsidy> {
            self.stake_subsidy.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `stake_subsidy`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn stake_subsidy_mut(&mut self) -> &mut super::StakeSubsidy {
            self.stake_subsidy.get_or_insert_default()
        }
        ///If `stake_subsidy` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn stake_subsidy_opt(&self) -> Option<&super::StakeSubsidy> {
            self.stake_subsidy.as_ref().map(|field| field as _)
        }
        ///Sets `stake_subsidy` with the provided value.
        pub fn set_stake_subsidy<T: Into<super::StakeSubsidy>>(&mut self, field: T) {
            self.stake_subsidy = Some(field.into().into());
        }
        ///Sets `stake_subsidy` with the provided value.
        pub fn with_stake_subsidy<T: Into<super::StakeSubsidy>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_stake_subsidy(field.into());
            self
        }
        ///If `safe_mode` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn safe_mode_opt_mut(&mut self) -> Option<&mut bool> {
            self.safe_mode.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `safe_mode`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn safe_mode_mut(&mut self) -> &mut bool {
            self.safe_mode.get_or_insert_default()
        }
        ///If `safe_mode` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn safe_mode_opt(&self) -> Option<bool> {
            self.safe_mode.as_ref().map(|field| *field)
        }
        ///Sets `safe_mode` with the provided value.
        pub fn set_safe_mode(&mut self, field: bool) {
            self.safe_mode = Some(field);
        }
        ///Sets `safe_mode` with the provided value.
        pub fn with_safe_mode(mut self, field: bool) -> Self {
            self.set_safe_mode(field);
            self
        }
        ///If `safe_mode_storage_rewards` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn safe_mode_storage_rewards_opt_mut(&mut self) -> Option<&mut u64> {
            self.safe_mode_storage_rewards.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `safe_mode_storage_rewards`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn safe_mode_storage_rewards_mut(&mut self) -> &mut u64 {
            self.safe_mode_storage_rewards.get_or_insert_default()
        }
        ///If `safe_mode_storage_rewards` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn safe_mode_storage_rewards_opt(&self) -> Option<u64> {
            self.safe_mode_storage_rewards.as_ref().map(|field| *field)
        }
        ///Sets `safe_mode_storage_rewards` with the provided value.
        pub fn set_safe_mode_storage_rewards(&mut self, field: u64) {
            self.safe_mode_storage_rewards = Some(field);
        }
        ///Sets `safe_mode_storage_rewards` with the provided value.
        pub fn with_safe_mode_storage_rewards(mut self, field: u64) -> Self {
            self.set_safe_mode_storage_rewards(field);
            self
        }
        ///If `safe_mode_computation_rewards` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn safe_mode_computation_rewards_opt_mut(&mut self) -> Option<&mut u64> {
            self.safe_mode_computation_rewards.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `safe_mode_computation_rewards`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn safe_mode_computation_rewards_mut(&mut self) -> &mut u64 {
            self.safe_mode_computation_rewards.get_or_insert_default()
        }
        ///If `safe_mode_computation_rewards` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn safe_mode_computation_rewards_opt(&self) -> Option<u64> {
            self.safe_mode_computation_rewards.as_ref().map(|field| *field)
        }
        ///Sets `safe_mode_computation_rewards` with the provided value.
        pub fn set_safe_mode_computation_rewards(&mut self, field: u64) {
            self.safe_mode_computation_rewards = Some(field);
        }
        ///Sets `safe_mode_computation_rewards` with the provided value.
        pub fn with_safe_mode_computation_rewards(mut self, field: u64) -> Self {
            self.set_safe_mode_computation_rewards(field);
            self
        }
        ///If `safe_mode_storage_rebates` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn safe_mode_storage_rebates_opt_mut(&mut self) -> Option<&mut u64> {
            self.safe_mode_storage_rebates.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `safe_mode_storage_rebates`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn safe_mode_storage_rebates_mut(&mut self) -> &mut u64 {
            self.safe_mode_storage_rebates.get_or_insert_default()
        }
        ///If `safe_mode_storage_rebates` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn safe_mode_storage_rebates_opt(&self) -> Option<u64> {
            self.safe_mode_storage_rebates.as_ref().map(|field| *field)
        }
        ///Sets `safe_mode_storage_rebates` with the provided value.
        pub fn set_safe_mode_storage_rebates(&mut self, field: u64) {
            self.safe_mode_storage_rebates = Some(field);
        }
        ///Sets `safe_mode_storage_rebates` with the provided value.
        pub fn with_safe_mode_storage_rebates(mut self, field: u64) -> Self {
            self.set_safe_mode_storage_rebates(field);
            self
        }
        ///If `safe_mode_non_refundable_storage_fee` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn safe_mode_non_refundable_storage_fee_opt_mut(
            &mut self,
        ) -> Option<&mut u64> {
            self.safe_mode_non_refundable_storage_fee.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `safe_mode_non_refundable_storage_fee`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn safe_mode_non_refundable_storage_fee_mut(&mut self) -> &mut u64 {
            self.safe_mode_non_refundable_storage_fee.get_or_insert_default()
        }
        ///If `safe_mode_non_refundable_storage_fee` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn safe_mode_non_refundable_storage_fee_opt(&self) -> Option<u64> {
            self.safe_mode_non_refundable_storage_fee.as_ref().map(|field| *field)
        }
        ///Sets `safe_mode_non_refundable_storage_fee` with the provided value.
        pub fn set_safe_mode_non_refundable_storage_fee(&mut self, field: u64) {
            self.safe_mode_non_refundable_storage_fee = Some(field);
        }
        ///Sets `safe_mode_non_refundable_storage_fee` with the provided value.
        pub fn with_safe_mode_non_refundable_storage_fee(mut self, field: u64) -> Self {
            self.set_safe_mode_non_refundable_storage_fee(field);
            self
        }
        ///If `epoch_start_timestamp_ms` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_start_timestamp_ms_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch_start_timestamp_ms.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch_start_timestamp_ms`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_start_timestamp_ms_mut(&mut self) -> &mut u64 {
            self.epoch_start_timestamp_ms.get_or_insert_default()
        }
        ///If `epoch_start_timestamp_ms` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_start_timestamp_ms_opt(&self) -> Option<u64> {
            self.epoch_start_timestamp_ms.as_ref().map(|field| *field)
        }
        ///Sets `epoch_start_timestamp_ms` with the provided value.
        pub fn set_epoch_start_timestamp_ms(&mut self, field: u64) {
            self.epoch_start_timestamp_ms = Some(field);
        }
        ///Sets `epoch_start_timestamp_ms` with the provided value.
        pub fn with_epoch_start_timestamp_ms(mut self, field: u64) -> Self {
            self.set_epoch_start_timestamp_ms(field);
            self
        }
        ///Returns the value of `extra_fields`, or the default value if `extra_fields` is unset.
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `extra_fields` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `extra_fields`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        ///If `extra_fields` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        ///Sets `extra_fields` with the provided value.
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        ///Sets `extra_fields` with the provided value.
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field.into());
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
        ///Returns the value of `bcs`, or the default value if `bcs` is unset.
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `bcs` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `bcs`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        ///If `bcs` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        ///Sets `bcs` with the provided value.
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        ///Sets `bcs` with the provided value.
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field.into());
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut i32> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut i32 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<i32> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: i32) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: i32) -> Self {
            self.set_version(field);
            self
        }
        ///Returns the value of `kind`, or the default value if `kind` is unset.
        pub fn kind(&self) -> &super::TransactionKind {
            self.kind
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::TransactionKind::default_instance() as _)
        }
        ///If `kind` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn kind_opt_mut(&mut self) -> Option<&mut super::TransactionKind> {
            self.kind.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `kind`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn kind_mut(&mut self) -> &mut super::TransactionKind {
            self.kind.get_or_insert_default()
        }
        ///If `kind` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn kind_opt(&self) -> Option<&super::TransactionKind> {
            self.kind.as_ref().map(|field| field as _)
        }
        ///Sets `kind` with the provided value.
        pub fn set_kind<T: Into<super::TransactionKind>>(&mut self, field: T) {
            self.kind = Some(field.into().into());
        }
        ///Sets `kind` with the provided value.
        pub fn with_kind<T: Into<super::TransactionKind>>(mut self, field: T) -> Self {
            self.set_kind(field.into());
            self
        }
        ///If `sender` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn sender_opt_mut(&mut self) -> Option<&mut String> {
            self.sender.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `sender`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn sender_mut(&mut self) -> &mut String {
            self.sender.get_or_insert_default()
        }
        ///If `sender` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn sender_opt(&self) -> Option<&str> {
            self.sender.as_ref().map(|field| field as _)
        }
        ///Sets `sender` with the provided value.
        pub fn set_sender<T: Into<String>>(&mut self, field: T) {
            self.sender = Some(field.into().into());
        }
        ///Sets `sender` with the provided value.
        pub fn with_sender<T: Into<String>>(mut self, field: T) -> Self {
            self.set_sender(field.into());
            self
        }
        ///Returns the value of `gas_payment`, or the default value if `gas_payment` is unset.
        pub fn gas_payment(&self) -> &super::GasPayment {
            self.gas_payment
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::GasPayment::default_instance() as _)
        }
        ///If `gas_payment` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn gas_payment_opt_mut(&mut self) -> Option<&mut super::GasPayment> {
            self.gas_payment.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `gas_payment`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn gas_payment_mut(&mut self) -> &mut super::GasPayment {
            self.gas_payment.get_or_insert_default()
        }
        ///If `gas_payment` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn gas_payment_opt(&self) -> Option<&super::GasPayment> {
            self.gas_payment.as_ref().map(|field| field as _)
        }
        ///Sets `gas_payment` with the provided value.
        pub fn set_gas_payment<T: Into<super::GasPayment>>(&mut self, field: T) {
            self.gas_payment = Some(field.into().into());
        }
        ///Sets `gas_payment` with the provided value.
        pub fn with_gas_payment<T: Into<super::GasPayment>>(mut self, field: T) -> Self {
            self.set_gas_payment(field.into());
            self
        }
        ///Returns the value of `expiration`, or the default value if `expiration` is unset.
        pub fn expiration(&self) -> &super::TransactionExpiration {
            self.expiration
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::TransactionExpiration::default_instance() as _)
        }
        ///If `expiration` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn expiration_opt_mut(
            &mut self,
        ) -> Option<&mut super::TransactionExpiration> {
            self.expiration.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `expiration`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn expiration_mut(&mut self) -> &mut super::TransactionExpiration {
            self.expiration.get_or_insert_default()
        }
        ///If `expiration` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn expiration_opt(&self) -> Option<&super::TransactionExpiration> {
            self.expiration.as_ref().map(|field| field as _)
        }
        ///Sets `expiration` with the provided value.
        pub fn set_expiration<T: Into<super::TransactionExpiration>>(
            &mut self,
            field: T,
        ) {
            self.expiration = Some(field.into().into());
        }
        ///Sets `expiration` with the provided value.
        pub fn with_expiration<T: Into<super::TransactionExpiration>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_expiration(field.into());
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
                unchanged_loaded_runtime_objects: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionEffects = super::TransactionEffects::const_default();
            &DEFAULT
        }
        ///Returns the value of `bcs`, or the default value if `bcs` is unset.
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `bcs` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `bcs`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        ///If `bcs` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        ///Sets `bcs` with the provided value.
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        ///Sets `bcs` with the provided value.
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field.into());
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut i32> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut i32 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<i32> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: i32) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: i32) -> Self {
            self.set_version(field);
            self
        }
        ///Returns the value of `status`, or the default value if `status` is unset.
        pub fn status(&self) -> &super::ExecutionStatus {
            self.status
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ExecutionStatus::default_instance() as _)
        }
        ///If `status` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn status_opt_mut(&mut self) -> Option<&mut super::ExecutionStatus> {
            self.status.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `status`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn status_mut(&mut self) -> &mut super::ExecutionStatus {
            self.status.get_or_insert_default()
        }
        ///If `status` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn status_opt(&self) -> Option<&super::ExecutionStatus> {
            self.status.as_ref().map(|field| field as _)
        }
        ///Sets `status` with the provided value.
        pub fn set_status<T: Into<super::ExecutionStatus>>(&mut self, field: T) {
            self.status = Some(field.into().into());
        }
        ///Sets `status` with the provided value.
        pub fn with_status<T: Into<super::ExecutionStatus>>(mut self, field: T) -> Self {
            self.set_status(field.into());
            self
        }
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///Returns the value of `gas_used`, or the default value if `gas_used` is unset.
        pub fn gas_used(&self) -> &super::GasCostSummary {
            self.gas_used
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::GasCostSummary::default_instance() as _)
        }
        ///If `gas_used` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn gas_used_opt_mut(&mut self) -> Option<&mut super::GasCostSummary> {
            self.gas_used.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `gas_used`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn gas_used_mut(&mut self) -> &mut super::GasCostSummary {
            self.gas_used.get_or_insert_default()
        }
        ///If `gas_used` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn gas_used_opt(&self) -> Option<&super::GasCostSummary> {
            self.gas_used.as_ref().map(|field| field as _)
        }
        ///Sets `gas_used` with the provided value.
        pub fn set_gas_used<T: Into<super::GasCostSummary>>(&mut self, field: T) {
            self.gas_used = Some(field.into().into());
        }
        ///Sets `gas_used` with the provided value.
        pub fn with_gas_used<T: Into<super::GasCostSummary>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_gas_used(field.into());
            self
        }
        ///If `transaction_digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn transaction_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.transaction_digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `transaction_digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transaction_digest_mut(&mut self) -> &mut String {
            self.transaction_digest.get_or_insert_default()
        }
        ///If `transaction_digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn transaction_digest_opt(&self) -> Option<&str> {
            self.transaction_digest.as_ref().map(|field| field as _)
        }
        ///Sets `transaction_digest` with the provided value.
        pub fn set_transaction_digest<T: Into<String>>(&mut self, field: T) {
            self.transaction_digest = Some(field.into().into());
        }
        ///Sets `transaction_digest` with the provided value.
        pub fn with_transaction_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_transaction_digest(field.into());
            self
        }
        ///Returns the value of `gas_object`, or the default value if `gas_object` is unset.
        pub fn gas_object(&self) -> &super::ChangedObject {
            self.gas_object
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ChangedObject::default_instance() as _)
        }
        ///If `gas_object` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn gas_object_opt_mut(&mut self) -> Option<&mut super::ChangedObject> {
            self.gas_object.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `gas_object`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn gas_object_mut(&mut self) -> &mut super::ChangedObject {
            self.gas_object.get_or_insert_default()
        }
        ///If `gas_object` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn gas_object_opt(&self) -> Option<&super::ChangedObject> {
            self.gas_object.as_ref().map(|field| field as _)
        }
        ///Sets `gas_object` with the provided value.
        pub fn set_gas_object<T: Into<super::ChangedObject>>(&mut self, field: T) {
            self.gas_object = Some(field.into().into());
        }
        ///Sets `gas_object` with the provided value.
        pub fn with_gas_object<T: Into<super::ChangedObject>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_gas_object(field.into());
            self
        }
        ///If `events_digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn events_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.events_digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `events_digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn events_digest_mut(&mut self) -> &mut String {
            self.events_digest.get_or_insert_default()
        }
        ///If `events_digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn events_digest_opt(&self) -> Option<&str> {
            self.events_digest.as_ref().map(|field| field as _)
        }
        ///Sets `events_digest` with the provided value.
        pub fn set_events_digest<T: Into<String>>(&mut self, field: T) {
            self.events_digest = Some(field.into().into());
        }
        ///Sets `events_digest` with the provided value.
        pub fn with_events_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_events_digest(field.into());
            self
        }
        ///Returns the value of `dependencies`, or the default value if `dependencies` is unset.
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        ///Returns a mutable reference to `dependencies`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn dependencies_mut(&mut self) -> &mut Vec<String> {
            &mut self.dependencies
        }
        ///Sets `dependencies` with the provided value.
        pub fn set_dependencies(&mut self, field: Vec<String>) {
            self.dependencies = field;
        }
        ///Sets `dependencies` with the provided value.
        pub fn with_dependencies(mut self, field: Vec<String>) -> Self {
            self.set_dependencies(field);
            self
        }
        ///If `lamport_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn lamport_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.lamport_version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `lamport_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn lamport_version_mut(&mut self) -> &mut u64 {
            self.lamport_version.get_or_insert_default()
        }
        ///If `lamport_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn lamport_version_opt(&self) -> Option<u64> {
            self.lamport_version.as_ref().map(|field| *field)
        }
        ///Sets `lamport_version` with the provided value.
        pub fn set_lamport_version(&mut self, field: u64) {
            self.lamport_version = Some(field);
        }
        ///Sets `lamport_version` with the provided value.
        pub fn with_lamport_version(mut self, field: u64) -> Self {
            self.set_lamport_version(field);
            self
        }
        ///Returns the value of `changed_objects`, or the default value if `changed_objects` is unset.
        pub fn changed_objects(&self) -> &[super::ChangedObject] {
            &self.changed_objects
        }
        ///Returns a mutable reference to `changed_objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn changed_objects_mut(&mut self) -> &mut Vec<super::ChangedObject> {
            &mut self.changed_objects
        }
        ///Sets `changed_objects` with the provided value.
        pub fn set_changed_objects(&mut self, field: Vec<super::ChangedObject>) {
            self.changed_objects = field;
        }
        ///Sets `changed_objects` with the provided value.
        pub fn with_changed_objects(mut self, field: Vec<super::ChangedObject>) -> Self {
            self.set_changed_objects(field);
            self
        }
        ///Returns the value of `unchanged_consensus_objects`, or the default value if `unchanged_consensus_objects` is unset.
        pub fn unchanged_consensus_objects(&self) -> &[super::UnchangedConsensusObject] {
            &self.unchanged_consensus_objects
        }
        ///Returns a mutable reference to `unchanged_consensus_objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn unchanged_consensus_objects_mut(
            &mut self,
        ) -> &mut Vec<super::UnchangedConsensusObject> {
            &mut self.unchanged_consensus_objects
        }
        ///Sets `unchanged_consensus_objects` with the provided value.
        pub fn set_unchanged_consensus_objects(
            &mut self,
            field: Vec<super::UnchangedConsensusObject>,
        ) {
            self.unchanged_consensus_objects = field;
        }
        ///Sets `unchanged_consensus_objects` with the provided value.
        pub fn with_unchanged_consensus_objects(
            mut self,
            field: Vec<super::UnchangedConsensusObject>,
        ) -> Self {
            self.set_unchanged_consensus_objects(field);
            self
        }
        ///If `auxiliary_data_digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn auxiliary_data_digest_opt_mut(&mut self) -> Option<&mut String> {
            self.auxiliary_data_digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `auxiliary_data_digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn auxiliary_data_digest_mut(&mut self) -> &mut String {
            self.auxiliary_data_digest.get_or_insert_default()
        }
        ///If `auxiliary_data_digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn auxiliary_data_digest_opt(&self) -> Option<&str> {
            self.auxiliary_data_digest.as_ref().map(|field| field as _)
        }
        ///Sets `auxiliary_data_digest` with the provided value.
        pub fn set_auxiliary_data_digest<T: Into<String>>(&mut self, field: T) {
            self.auxiliary_data_digest = Some(field.into().into());
        }
        ///Sets `auxiliary_data_digest` with the provided value.
        pub fn with_auxiliary_data_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_auxiliary_data_digest(field.into());
            self
        }
        ///Returns the value of `unchanged_loaded_runtime_objects`, or the default value if `unchanged_loaded_runtime_objects` is unset.
        pub fn unchanged_loaded_runtime_objects(&self) -> &[super::ObjectReference] {
            &self.unchanged_loaded_runtime_objects
        }
        ///Returns a mutable reference to `unchanged_loaded_runtime_objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn unchanged_loaded_runtime_objects_mut(
            &mut self,
        ) -> &mut Vec<super::ObjectReference> {
            &mut self.unchanged_loaded_runtime_objects
        }
        ///Sets `unchanged_loaded_runtime_objects` with the provided value.
        pub fn set_unchanged_loaded_runtime_objects(
            &mut self,
            field: Vec<super::ObjectReference>,
        ) {
            self.unchanged_loaded_runtime_objects = field;
        }
        ///Sets `unchanged_loaded_runtime_objects` with the provided value.
        pub fn with_unchanged_loaded_runtime_objects(
            mut self,
            field: Vec<super::ObjectReference>,
        ) -> Self {
            self.set_unchanged_loaded_runtime_objects(field);
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
        ///Returns the value of `bcs`, or the default value if `bcs` is unset.
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `bcs` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `bcs`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        ///If `bcs` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        ///Sets `bcs` with the provided value.
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        ///Sets `bcs` with the provided value.
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field.into());
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///Returns the value of `events`, or the default value if `events` is unset.
        pub fn events(&self) -> &[super::Event] {
            &self.events
        }
        ///Returns a mutable reference to `events`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn events_mut(&mut self) -> &mut Vec<super::Event> {
            &mut self.events
        }
        ///Sets `events` with the provided value.
        pub fn set_events(&mut self, field: Vec<super::Event>) {
            self.events = field;
        }
        ///Sets `events` with the provided value.
        pub fn with_events(mut self, field: Vec<super::Event>) -> Self {
            self.set_events(field);
            self
        }
    }
    impl super::TransactionExpiration {
        pub const fn const_default() -> Self {
            Self {
                kind: None,
                epoch: None,
                min_epoch: None,
                min_timestamp: None,
                max_timestamp: None,
                chain: None,
                nonce: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionExpiration = super::TransactionExpiration::const_default();
            &DEFAULT
        }
        ///Sets `kind` with the provided value.
        pub fn with_kind<
            T: Into<super::transaction_expiration::TransactionExpirationKind>,
        >(mut self, field: T) -> Self {
            self.set_kind(field.into());
            self
        }
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///If `min_epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn min_epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.min_epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `min_epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn min_epoch_mut(&mut self) -> &mut u64 {
            self.min_epoch.get_or_insert_default()
        }
        ///If `min_epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn min_epoch_opt(&self) -> Option<u64> {
            self.min_epoch.as_ref().map(|field| *field)
        }
        ///Sets `min_epoch` with the provided value.
        pub fn set_min_epoch(&mut self, field: u64) {
            self.min_epoch = Some(field);
        }
        ///Sets `min_epoch` with the provided value.
        pub fn with_min_epoch(mut self, field: u64) -> Self {
            self.set_min_epoch(field);
            self
        }
        ///If `min_timestamp` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn min_timestamp_opt_mut(
            &mut self,
        ) -> Option<&mut ::prost_types::Timestamp> {
            self.min_timestamp.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `min_timestamp`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn min_timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.min_timestamp.get_or_insert_default()
        }
        ///If `min_timestamp` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn min_timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.min_timestamp.as_ref().map(|field| field as _)
        }
        ///Sets `min_timestamp` with the provided value.
        pub fn set_min_timestamp<T: Into<::prost_types::Timestamp>>(
            &mut self,
            field: T,
        ) {
            self.min_timestamp = Some(field.into().into());
        }
        ///Sets `min_timestamp` with the provided value.
        pub fn with_min_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_min_timestamp(field.into());
            self
        }
        ///If `max_timestamp` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn max_timestamp_opt_mut(
            &mut self,
        ) -> Option<&mut ::prost_types::Timestamp> {
            self.max_timestamp.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `max_timestamp`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn max_timestamp_mut(&mut self) -> &mut ::prost_types::Timestamp {
            self.max_timestamp.get_or_insert_default()
        }
        ///If `max_timestamp` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn max_timestamp_opt(&self) -> Option<&::prost_types::Timestamp> {
            self.max_timestamp.as_ref().map(|field| field as _)
        }
        ///Sets `max_timestamp` with the provided value.
        pub fn set_max_timestamp<T: Into<::prost_types::Timestamp>>(
            &mut self,
            field: T,
        ) {
            self.max_timestamp = Some(field.into().into());
        }
        ///Sets `max_timestamp` with the provided value.
        pub fn with_max_timestamp<T: Into<::prost_types::Timestamp>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_max_timestamp(field.into());
            self
        }
        ///If `chain` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn chain_opt_mut(&mut self) -> Option<&mut String> {
            self.chain.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `chain`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn chain_mut(&mut self) -> &mut String {
            self.chain.get_or_insert_default()
        }
        ///If `chain` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn chain_opt(&self) -> Option<&str> {
            self.chain.as_ref().map(|field| field as _)
        }
        ///Sets `chain` with the provided value.
        pub fn set_chain<T: Into<String>>(&mut self, field: T) {
            self.chain = Some(field.into().into());
        }
        ///Sets `chain` with the provided value.
        pub fn with_chain<T: Into<String>>(mut self, field: T) -> Self {
            self.set_chain(field.into());
            self
        }
        ///If `nonce` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn nonce_opt_mut(&mut self) -> Option<&mut u32> {
            self.nonce.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `nonce`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn nonce_mut(&mut self) -> &mut u32 {
            self.nonce.get_or_insert_default()
        }
        ///If `nonce` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn nonce_opt(&self) -> Option<u32> {
            self.nonce.as_ref().map(|field| *field)
        }
        ///Sets `nonce` with the provided value.
        pub fn set_nonce(&mut self, field: u32) {
            self.nonce = Some(field);
        }
        ///Sets `nonce` with the provided value.
        pub fn with_nonce(mut self, field: u32) -> Self {
            self.set_nonce(field);
            self
        }
    }
    impl super::TransactionKind {
        pub const fn const_default() -> Self {
            Self { kind: None, data: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionKind = super::TransactionKind::const_default();
            &DEFAULT
        }
        ///Sets `kind` with the provided value.
        pub fn with_kind<T: Into<super::transaction_kind::Kind>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_kind(field.into());
            self
        }
        ///Returns the value of `programmable_transaction`, or the default value if `programmable_transaction` is unset.
        pub fn programmable_transaction(&self) -> &super::ProgrammableTransaction {
            if let Some(super::transaction_kind::Data::ProgrammableTransaction(field)) = &self
                .data
            {
                field as _
            } else {
                super::ProgrammableTransaction::default_instance() as _
            }
        }
        ///If `programmable_transaction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn programmable_transaction_opt(
            &self,
        ) -> Option<&super::ProgrammableTransaction> {
            if let Some(super::transaction_kind::Data::ProgrammableTransaction(field)) = &self
                .data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `programmable_transaction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn programmable_transaction_opt_mut(
            &mut self,
        ) -> Option<&mut super::ProgrammableTransaction> {
            if let Some(super::transaction_kind::Data::ProgrammableTransaction(field)) = &mut self
                .data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `programmable_transaction`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn programmable_transaction_mut(
            &mut self,
        ) -> &mut super::ProgrammableTransaction {
            if self.programmable_transaction_opt_mut().is_none() {
                self.data = Some(
                    super::transaction_kind::Data::ProgrammableTransaction(
                        super::ProgrammableTransaction::default(),
                    ),
                );
            }
            self.programmable_transaction_opt_mut().unwrap()
        }
        ///Sets `programmable_transaction` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_programmable_transaction<T: Into<super::ProgrammableTransaction>>(
            &mut self,
            field: T,
        ) {
            self.data = Some(
                super::transaction_kind::Data::ProgrammableTransaction(
                    field.into().into(),
                ),
            );
        }
        ///Sets `programmable_transaction` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_programmable_transaction<T: Into<super::ProgrammableTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_programmable_transaction(field.into());
            self
        }
        ///Returns the value of `change_epoch`, or the default value if `change_epoch` is unset.
        pub fn change_epoch(&self) -> &super::ChangeEpoch {
            if let Some(super::transaction_kind::Data::ChangeEpoch(field)) = &self.data {
                field as _
            } else {
                super::ChangeEpoch::default_instance() as _
            }
        }
        ///If `change_epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn change_epoch_opt(&self) -> Option<&super::ChangeEpoch> {
            if let Some(super::transaction_kind::Data::ChangeEpoch(field)) = &self.data {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `change_epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn change_epoch_opt_mut(&mut self) -> Option<&mut super::ChangeEpoch> {
            if let Some(super::transaction_kind::Data::ChangeEpoch(field)) = &mut self
                .data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `change_epoch`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn change_epoch_mut(&mut self) -> &mut super::ChangeEpoch {
            if self.change_epoch_opt_mut().is_none() {
                self.data = Some(
                    super::transaction_kind::Data::ChangeEpoch(
                        super::ChangeEpoch::default(),
                    ),
                );
            }
            self.change_epoch_opt_mut().unwrap()
        }
        ///Sets `change_epoch` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_change_epoch<T: Into<super::ChangeEpoch>>(&mut self, field: T) {
            self.data = Some(
                super::transaction_kind::Data::ChangeEpoch(field.into().into()),
            );
        }
        ///Sets `change_epoch` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_change_epoch<T: Into<super::ChangeEpoch>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_change_epoch(field.into());
            self
        }
        ///Returns the value of `genesis`, or the default value if `genesis` is unset.
        pub fn genesis(&self) -> &super::GenesisTransaction {
            if let Some(super::transaction_kind::Data::Genesis(field)) = &self.data {
                field as _
            } else {
                super::GenesisTransaction::default_instance() as _
            }
        }
        ///If `genesis` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn genesis_opt(&self) -> Option<&super::GenesisTransaction> {
            if let Some(super::transaction_kind::Data::Genesis(field)) = &self.data {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `genesis` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn genesis_opt_mut(&mut self) -> Option<&mut super::GenesisTransaction> {
            if let Some(super::transaction_kind::Data::Genesis(field)) = &mut self.data {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `genesis`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn genesis_mut(&mut self) -> &mut super::GenesisTransaction {
            if self.genesis_opt_mut().is_none() {
                self.data = Some(
                    super::transaction_kind::Data::Genesis(
                        super::GenesisTransaction::default(),
                    ),
                );
            }
            self.genesis_opt_mut().unwrap()
        }
        ///Sets `genesis` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_genesis<T: Into<super::GenesisTransaction>>(&mut self, field: T) {
            self.data = Some(
                super::transaction_kind::Data::Genesis(field.into().into()),
            );
        }
        ///Sets `genesis` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_genesis<T: Into<super::GenesisTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_genesis(field.into());
            self
        }
        ///Returns the value of `consensus_commit_prologue`, or the default value if `consensus_commit_prologue` is unset.
        pub fn consensus_commit_prologue(&self) -> &super::ConsensusCommitPrologue {
            if let Some(super::transaction_kind::Data::ConsensusCommitPrologue(field)) = &self
                .data
            {
                field as _
            } else {
                super::ConsensusCommitPrologue::default_instance() as _
            }
        }
        ///If `consensus_commit_prologue` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn consensus_commit_prologue_opt(
            &self,
        ) -> Option<&super::ConsensusCommitPrologue> {
            if let Some(super::transaction_kind::Data::ConsensusCommitPrologue(field)) = &self
                .data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `consensus_commit_prologue` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn consensus_commit_prologue_opt_mut(
            &mut self,
        ) -> Option<&mut super::ConsensusCommitPrologue> {
            if let Some(super::transaction_kind::Data::ConsensusCommitPrologue(field)) = &mut self
                .data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `consensus_commit_prologue`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn consensus_commit_prologue_mut(
            &mut self,
        ) -> &mut super::ConsensusCommitPrologue {
            if self.consensus_commit_prologue_opt_mut().is_none() {
                self.data = Some(
                    super::transaction_kind::Data::ConsensusCommitPrologue(
                        super::ConsensusCommitPrologue::default(),
                    ),
                );
            }
            self.consensus_commit_prologue_opt_mut().unwrap()
        }
        ///Sets `consensus_commit_prologue` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_consensus_commit_prologue<T: Into<super::ConsensusCommitPrologue>>(
            &mut self,
            field: T,
        ) {
            self.data = Some(
                super::transaction_kind::Data::ConsensusCommitPrologue(
                    field.into().into(),
                ),
            );
        }
        ///Sets `consensus_commit_prologue` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_consensus_commit_prologue<T: Into<super::ConsensusCommitPrologue>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_consensus_commit_prologue(field.into());
            self
        }
        ///Returns the value of `authenticator_state_update`, or the default value if `authenticator_state_update` is unset.
        pub fn authenticator_state_update(&self) -> &super::AuthenticatorStateUpdate {
            if let Some(
                super::transaction_kind::Data::AuthenticatorStateUpdate(field),
            ) = &self.data
            {
                field as _
            } else {
                super::AuthenticatorStateUpdate::default_instance() as _
            }
        }
        ///If `authenticator_state_update` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn authenticator_state_update_opt(
            &self,
        ) -> Option<&super::AuthenticatorStateUpdate> {
            if let Some(
                super::transaction_kind::Data::AuthenticatorStateUpdate(field),
            ) = &self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `authenticator_state_update` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn authenticator_state_update_opt_mut(
            &mut self,
        ) -> Option<&mut super::AuthenticatorStateUpdate> {
            if let Some(
                super::transaction_kind::Data::AuthenticatorStateUpdate(field),
            ) = &mut self.data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `authenticator_state_update`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn authenticator_state_update_mut(
            &mut self,
        ) -> &mut super::AuthenticatorStateUpdate {
            if self.authenticator_state_update_opt_mut().is_none() {
                self.data = Some(
                    super::transaction_kind::Data::AuthenticatorStateUpdate(
                        super::AuthenticatorStateUpdate::default(),
                    ),
                );
            }
            self.authenticator_state_update_opt_mut().unwrap()
        }
        ///Sets `authenticator_state_update` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_authenticator_state_update<T: Into<super::AuthenticatorStateUpdate>>(
            &mut self,
            field: T,
        ) {
            self.data = Some(
                super::transaction_kind::Data::AuthenticatorStateUpdate(
                    field.into().into(),
                ),
            );
        }
        ///Sets `authenticator_state_update` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_authenticator_state_update<T: Into<super::AuthenticatorStateUpdate>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_authenticator_state_update(field.into());
            self
        }
        ///Returns the value of `end_of_epoch`, or the default value if `end_of_epoch` is unset.
        pub fn end_of_epoch(&self) -> &super::EndOfEpochTransaction {
            if let Some(super::transaction_kind::Data::EndOfEpoch(field)) = &self.data {
                field as _
            } else {
                super::EndOfEpochTransaction::default_instance() as _
            }
        }
        ///If `end_of_epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn end_of_epoch_opt(&self) -> Option<&super::EndOfEpochTransaction> {
            if let Some(super::transaction_kind::Data::EndOfEpoch(field)) = &self.data {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `end_of_epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn end_of_epoch_opt_mut(
            &mut self,
        ) -> Option<&mut super::EndOfEpochTransaction> {
            if let Some(super::transaction_kind::Data::EndOfEpoch(field)) = &mut self
                .data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `end_of_epoch`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn end_of_epoch_mut(&mut self) -> &mut super::EndOfEpochTransaction {
            if self.end_of_epoch_opt_mut().is_none() {
                self.data = Some(
                    super::transaction_kind::Data::EndOfEpoch(
                        super::EndOfEpochTransaction::default(),
                    ),
                );
            }
            self.end_of_epoch_opt_mut().unwrap()
        }
        ///Sets `end_of_epoch` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_end_of_epoch<T: Into<super::EndOfEpochTransaction>>(
            &mut self,
            field: T,
        ) {
            self.data = Some(
                super::transaction_kind::Data::EndOfEpoch(field.into().into()),
            );
        }
        ///Sets `end_of_epoch` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_end_of_epoch<T: Into<super::EndOfEpochTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_end_of_epoch(field.into());
            self
        }
        ///Returns the value of `randomness_state_update`, or the default value if `randomness_state_update` is unset.
        pub fn randomness_state_update(&self) -> &super::RandomnessStateUpdate {
            if let Some(super::transaction_kind::Data::RandomnessStateUpdate(field)) = &self
                .data
            {
                field as _
            } else {
                super::RandomnessStateUpdate::default_instance() as _
            }
        }
        ///If `randomness_state_update` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn randomness_state_update_opt(
            &self,
        ) -> Option<&super::RandomnessStateUpdate> {
            if let Some(super::transaction_kind::Data::RandomnessStateUpdate(field)) = &self
                .data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `randomness_state_update` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn randomness_state_update_opt_mut(
            &mut self,
        ) -> Option<&mut super::RandomnessStateUpdate> {
            if let Some(super::transaction_kind::Data::RandomnessStateUpdate(field)) = &mut self
                .data
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `randomness_state_update`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn randomness_state_update_mut(
            &mut self,
        ) -> &mut super::RandomnessStateUpdate {
            if self.randomness_state_update_opt_mut().is_none() {
                self.data = Some(
                    super::transaction_kind::Data::RandomnessStateUpdate(
                        super::RandomnessStateUpdate::default(),
                    ),
                );
            }
            self.randomness_state_update_opt_mut().unwrap()
        }
        ///Sets `randomness_state_update` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_randomness_state_update<T: Into<super::RandomnessStateUpdate>>(
            &mut self,
            field: T,
        ) {
            self.data = Some(
                super::transaction_kind::Data::RandomnessStateUpdate(field.into().into()),
            );
        }
        ///Sets `randomness_state_update` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_randomness_state_update<T: Into<super::RandomnessStateUpdate>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_randomness_state_update(field.into());
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
        ///Returns the value of `objects`, or the default value if `objects` is unset.
        pub fn objects(&self) -> &[super::Argument] {
            &self.objects
        }
        ///Returns a mutable reference to `objects`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn objects_mut(&mut self) -> &mut Vec<super::Argument> {
            &mut self.objects
        }
        ///Sets `objects` with the provided value.
        pub fn set_objects(&mut self, field: Vec<super::Argument>) {
            self.objects = field;
        }
        ///Sets `objects` with the provided value.
        pub fn with_objects(mut self, field: Vec<super::Argument>) -> Self {
            self.set_objects(field);
            self
        }
        ///Returns the value of `address`, or the default value if `address` is unset.
        pub fn address(&self) -> &super::Argument {
            self.address
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Argument::default_instance() as _)
        }
        ///If `address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn address_opt_mut(&mut self) -> Option<&mut super::Argument> {
            self.address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_mut(&mut self) -> &mut super::Argument {
            self.address.get_or_insert_default()
        }
        ///If `address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn address_opt(&self) -> Option<&super::Argument> {
            self.address.as_ref().map(|field| field as _)
        }
        ///Sets `address` with the provided value.
        pub fn set_address<T: Into<super::Argument>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        ///Sets `address` with the provided value.
        pub fn with_address<T: Into<super::Argument>>(mut self, field: T) -> Self {
            self.set_address(field.into());
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
        ///If `type_argument` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn type_argument_opt_mut(&mut self) -> Option<&mut u32> {
            self.type_argument.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `type_argument`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn type_argument_mut(&mut self) -> &mut u32 {
            self.type_argument.get_or_insert_default()
        }
        ///If `type_argument` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn type_argument_opt(&self) -> Option<u32> {
            self.type_argument.as_ref().map(|field| *field)
        }
        ///Sets `type_argument` with the provided value.
        pub fn set_type_argument(&mut self, field: u32) {
            self.type_argument = Some(field);
        }
        ///Sets `type_argument` with the provided value.
        pub fn with_type_argument(mut self, field: u32) -> Self {
            self.set_type_argument(field);
            self
        }
        ///Sets `kind` with the provided value.
        pub fn with_kind<T: Into<super::type_argument_error::TypeArgumentErrorKind>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_kind(field.into());
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
        ///If `module_name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn module_name_opt_mut(&mut self) -> Option<&mut String> {
            self.module_name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `module_name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn module_name_mut(&mut self) -> &mut String {
            self.module_name.get_or_insert_default()
        }
        ///If `module_name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn module_name_opt(&self) -> Option<&str> {
            self.module_name.as_ref().map(|field| field as _)
        }
        ///Sets `module_name` with the provided value.
        pub fn set_module_name<T: Into<String>>(&mut self, field: T) {
            self.module_name = Some(field.into().into());
        }
        ///Sets `module_name` with the provided value.
        pub fn with_module_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_module_name(field.into());
            self
        }
        ///If `datatype_name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn datatype_name_opt_mut(&mut self) -> Option<&mut String> {
            self.datatype_name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `datatype_name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn datatype_name_mut(&mut self) -> &mut String {
            self.datatype_name.get_or_insert_default()
        }
        ///If `datatype_name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn datatype_name_opt(&self) -> Option<&str> {
            self.datatype_name.as_ref().map(|field| field as _)
        }
        ///Sets `datatype_name` with the provided value.
        pub fn set_datatype_name<T: Into<String>>(&mut self, field: T) {
            self.datatype_name = Some(field.into().into());
        }
        ///Sets `datatype_name` with the provided value.
        pub fn with_datatype_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_datatype_name(field.into());
            self
        }
        ///If `package_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_id_opt_mut(&mut self) -> Option<&mut String> {
            self.package_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_id_mut(&mut self) -> &mut String {
            self.package_id.get_or_insert_default()
        }
        ///If `package_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_id_opt(&self) -> Option<&str> {
            self.package_id.as_ref().map(|field| field as _)
        }
        ///Sets `package_id` with the provided value.
        pub fn set_package_id<T: Into<String>>(&mut self, field: T) {
            self.package_id = Some(field.into().into());
        }
        ///Sets `package_id` with the provided value.
        pub fn with_package_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package_id(field.into());
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
        ///If `is_phantom` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn is_phantom_opt_mut(&mut self) -> Option<&mut bool> {
            self.is_phantom.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `is_phantom`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn is_phantom_mut(&mut self) -> &mut bool {
            self.is_phantom.get_or_insert_default()
        }
        ///If `is_phantom` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn is_phantom_opt(&self) -> Option<bool> {
            self.is_phantom.as_ref().map(|field| *field)
        }
        ///Sets `is_phantom` with the provided value.
        pub fn set_is_phantom(&mut self, field: bool) {
            self.is_phantom = Some(field);
        }
        ///Sets `is_phantom` with the provided value.
        pub fn with_is_phantom(mut self, field: bool) -> Self {
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
        ///Sets `kind` with the provided value.
        pub fn with_kind<
            T: Into<super::unchanged_consensus_object::UnchangedConsensusObjectKind>,
        >(mut self, field: T) -> Self {
            self.set_kind(field.into());
            self
        }
        ///If `object_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        ///If `object_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        ///Sets `object_id` with the provided value.
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        ///Sets `object_id` with the provided value.
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field.into());
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
            self.set_version(field);
            self
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut String> {
            self.digest.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn digest_mut(&mut self) -> &mut String {
            self.digest.get_or_insert_default()
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&str> {
            self.digest.as_ref().map(|field| field as _)
        }
        ///Sets `digest` with the provided value.
        pub fn set_digest<T: Into<String>>(&mut self, field: T) {
            self.digest = Some(field.into().into());
        }
        ///Sets `digest` with the provided value.
        pub fn with_digest<T: Into<String>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
        ///If `object_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_type_opt_mut(&mut self) -> Option<&mut String> {
            self.object_type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_type_mut(&mut self) -> &mut String {
            self.object_type.get_or_insert_default()
        }
        ///If `object_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_type_opt(&self) -> Option<&str> {
            self.object_type.as_ref().map(|field| field as _)
        }
        ///Sets `object_type` with the provided value.
        pub fn set_object_type<T: Into<String>>(&mut self, field: T) {
            self.object_type = Some(field.into().into());
        }
        ///Sets `object_type` with the provided value.
        pub fn with_object_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_type(field.into());
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
        ///Returns the value of `modules`, or the default value if `modules` is unset.
        pub fn modules(&self) -> &[::prost::bytes::Bytes] {
            &self.modules
        }
        ///Returns a mutable reference to `modules`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn modules_mut(&mut self) -> &mut Vec<::prost::bytes::Bytes> {
            &mut self.modules
        }
        ///Sets `modules` with the provided value.
        pub fn set_modules(&mut self, field: Vec<::prost::bytes::Bytes>) {
            self.modules = field;
        }
        ///Sets `modules` with the provided value.
        pub fn with_modules(mut self, field: Vec<::prost::bytes::Bytes>) -> Self {
            self.set_modules(field);
            self
        }
        ///Returns the value of `dependencies`, or the default value if `dependencies` is unset.
        pub fn dependencies(&self) -> &[String] {
            &self.dependencies
        }
        ///Returns a mutable reference to `dependencies`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn dependencies_mut(&mut self) -> &mut Vec<String> {
            &mut self.dependencies
        }
        ///Sets `dependencies` with the provided value.
        pub fn set_dependencies(&mut self, field: Vec<String>) {
            self.dependencies = field;
        }
        ///Sets `dependencies` with the provided value.
        pub fn with_dependencies(mut self, field: Vec<String>) -> Self {
            self.set_dependencies(field);
            self
        }
        ///If `package` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_opt_mut(&mut self) -> Option<&mut String> {
            self.package.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `package`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn package_mut(&mut self) -> &mut String {
            self.package.get_or_insert_default()
        }
        ///If `package` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_opt(&self) -> Option<&str> {
            self.package.as_ref().map(|field| field as _)
        }
        ///Sets `package` with the provided value.
        pub fn set_package<T: Into<String>>(&mut self, field: T) {
            self.package = Some(field.into().into());
        }
        ///Sets `package` with the provided value.
        pub fn with_package<T: Into<String>>(mut self, field: T) -> Self {
            self.set_package(field.into());
            self
        }
        ///Returns the value of `ticket`, or the default value if `ticket` is unset.
        pub fn ticket(&self) -> &super::Argument {
            self.ticket
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Argument::default_instance() as _)
        }
        ///If `ticket` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn ticket_opt_mut(&mut self) -> Option<&mut super::Argument> {
            self.ticket.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `ticket`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn ticket_mut(&mut self) -> &mut super::Argument {
            self.ticket.get_or_insert_default()
        }
        ///If `ticket` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn ticket_opt(&self) -> Option<&super::Argument> {
            self.ticket.as_ref().map(|field| field as _)
        }
        ///Sets `ticket` with the provided value.
        pub fn set_ticket<T: Into<super::Argument>>(&mut self, field: T) {
            self.ticket = Some(field.into().into());
        }
        ///Sets `ticket` with the provided value.
        pub fn with_ticket<T: Into<super::Argument>>(mut self, field: T) -> Self {
            self.set_ticket(field.into());
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
        ///Returns the value of `bcs`, or the default value if `bcs` is unset.
        pub fn bcs(&self) -> &super::Bcs {
            self.bcs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `bcs` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn bcs_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.bcs.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `bcs`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn bcs_mut(&mut self) -> &mut super::Bcs {
            self.bcs.get_or_insert_default()
        }
        ///If `bcs` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn bcs_opt(&self) -> Option<&super::Bcs> {
            self.bcs.as_ref().map(|field| field as _)
        }
        ///Sets `bcs` with the provided value.
        pub fn set_bcs<T: Into<super::Bcs>>(&mut self, field: T) {
            self.bcs = Some(field.into().into());
        }
        ///Sets `bcs` with the provided value.
        pub fn with_bcs<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_bcs(field.into());
            self
        }
        ///Sets `scheme` with the provided value.
        pub fn with_scheme<T: Into<super::SignatureScheme>>(mut self, field: T) -> Self {
            self.set_scheme(field.into());
            self
        }
        ///Returns the value of `simple`, or the default value if `simple` is unset.
        pub fn simple(&self) -> &super::SimpleSignature {
            if let Some(super::user_signature::Signature::Simple(field)) = &self
                .signature
            {
                field as _
            } else {
                super::SimpleSignature::default_instance() as _
            }
        }
        ///If `simple` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn simple_opt(&self) -> Option<&super::SimpleSignature> {
            if let Some(super::user_signature::Signature::Simple(field)) = &self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `simple` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn simple_opt_mut(&mut self) -> Option<&mut super::SimpleSignature> {
            if let Some(super::user_signature::Signature::Simple(field)) = &mut self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `simple`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `simple` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_simple<T: Into<super::SimpleSignature>>(&mut self, field: T) {
            self.signature = Some(
                super::user_signature::Signature::Simple(field.into().into()),
            );
        }
        ///Sets `simple` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_simple<T: Into<super::SimpleSignature>>(mut self, field: T) -> Self {
            self.set_simple(field.into());
            self
        }
        ///Returns the value of `multisig`, or the default value if `multisig` is unset.
        pub fn multisig(&self) -> &super::MultisigAggregatedSignature {
            if let Some(super::user_signature::Signature::Multisig(field)) = &self
                .signature
            {
                field as _
            } else {
                super::MultisigAggregatedSignature::default_instance() as _
            }
        }
        ///If `multisig` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn multisig_opt(&self) -> Option<&super::MultisigAggregatedSignature> {
            if let Some(super::user_signature::Signature::Multisig(field)) = &self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `multisig` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
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
        ///Returns a mutable reference to `multisig`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `multisig` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_multisig<T: Into<super::MultisigAggregatedSignature>>(
            &mut self,
            field: T,
        ) {
            self.signature = Some(
                super::user_signature::Signature::Multisig(field.into().into()),
            );
        }
        ///Sets `multisig` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_multisig<T: Into<super::MultisigAggregatedSignature>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_multisig(field.into());
            self
        }
        ///Returns the value of `zklogin`, or the default value if `zklogin` is unset.
        pub fn zklogin(&self) -> &super::ZkLoginAuthenticator {
            if let Some(super::user_signature::Signature::Zklogin(field)) = &self
                .signature
            {
                field as _
            } else {
                super::ZkLoginAuthenticator::default_instance() as _
            }
        }
        ///If `zklogin` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn zklogin_opt(&self) -> Option<&super::ZkLoginAuthenticator> {
            if let Some(super::user_signature::Signature::Zklogin(field)) = &self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `zklogin` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn zklogin_opt_mut(&mut self) -> Option<&mut super::ZkLoginAuthenticator> {
            if let Some(super::user_signature::Signature::Zklogin(field)) = &mut self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `zklogin`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `zklogin` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_zklogin<T: Into<super::ZkLoginAuthenticator>>(&mut self, field: T) {
            self.signature = Some(
                super::user_signature::Signature::Zklogin(field.into().into()),
            );
        }
        ///Sets `zklogin` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_zklogin<T: Into<super::ZkLoginAuthenticator>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_zklogin(field.into());
            self
        }
        ///Returns the value of `passkey`, or the default value if `passkey` is unset.
        pub fn passkey(&self) -> &super::PasskeyAuthenticator {
            if let Some(super::user_signature::Signature::Passkey(field)) = &self
                .signature
            {
                field as _
            } else {
                super::PasskeyAuthenticator::default_instance() as _
            }
        }
        ///If `passkey` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn passkey_opt(&self) -> Option<&super::PasskeyAuthenticator> {
            if let Some(super::user_signature::Signature::Passkey(field)) = &self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `passkey` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn passkey_opt_mut(&mut self) -> Option<&mut super::PasskeyAuthenticator> {
            if let Some(super::user_signature::Signature::Passkey(field)) = &mut self
                .signature
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `passkey`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
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
        ///Sets `passkey` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_passkey<T: Into<super::PasskeyAuthenticator>>(&mut self, field: T) {
            self.signature = Some(
                super::user_signature::Signature::Passkey(field.into().into()),
            );
        }
        ///Sets `passkey` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_passkey<T: Into<super::PasskeyAuthenticator>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_passkey(field.into());
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
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
            self
        }
        ///If `address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        ///If `address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        ///Sets `address` with the provided value.
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        ///Sets `address` with the provided value.
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field.into());
            self
        }
        ///If `description` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn description_opt_mut(&mut self) -> Option<&mut String> {
            self.description.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `description`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn description_mut(&mut self) -> &mut String {
            self.description.get_or_insert_default()
        }
        ///If `description` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn description_opt(&self) -> Option<&str> {
            self.description.as_ref().map(|field| field as _)
        }
        ///Sets `description` with the provided value.
        pub fn set_description<T: Into<String>>(&mut self, field: T) {
            self.description = Some(field.into().into());
        }
        ///Sets `description` with the provided value.
        pub fn with_description<T: Into<String>>(mut self, field: T) -> Self {
            self.set_description(field.into());
            self
        }
        ///If `image_url` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn image_url_opt_mut(&mut self) -> Option<&mut String> {
            self.image_url.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `image_url`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn image_url_mut(&mut self) -> &mut String {
            self.image_url.get_or_insert_default()
        }
        ///If `image_url` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn image_url_opt(&self) -> Option<&str> {
            self.image_url.as_ref().map(|field| field as _)
        }
        ///Sets `image_url` with the provided value.
        pub fn set_image_url<T: Into<String>>(&mut self, field: T) {
            self.image_url = Some(field.into().into());
        }
        ///Sets `image_url` with the provided value.
        pub fn with_image_url<T: Into<String>>(mut self, field: T) -> Self {
            self.set_image_url(field.into());
            self
        }
        ///If `project_url` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn project_url_opt_mut(&mut self) -> Option<&mut String> {
            self.project_url.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `project_url`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn project_url_mut(&mut self) -> &mut String {
            self.project_url.get_or_insert_default()
        }
        ///If `project_url` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn project_url_opt(&self) -> Option<&str> {
            self.project_url.as_ref().map(|field| field as _)
        }
        ///Sets `project_url` with the provided value.
        pub fn set_project_url<T: Into<String>>(&mut self, field: T) {
            self.project_url = Some(field.into().into());
        }
        ///Sets `project_url` with the provided value.
        pub fn with_project_url<T: Into<String>>(mut self, field: T) -> Self {
            self.set_project_url(field.into());
            self
        }
        ///If `protocol_public_key` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn protocol_public_key_opt(&self) -> Option<&[u8]> {
            self.protocol_public_key.as_ref().map(|field| field as _)
        }
        ///Sets `protocol_public_key` with the provided value.
        pub fn set_protocol_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.protocol_public_key = Some(field.into().into());
        }
        ///Sets `protocol_public_key` with the provided value.
        pub fn with_protocol_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_protocol_public_key(field.into());
            self
        }
        ///If `proof_of_possession` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn proof_of_possession_opt(&self) -> Option<&[u8]> {
            self.proof_of_possession.as_ref().map(|field| field as _)
        }
        ///Sets `proof_of_possession` with the provided value.
        pub fn set_proof_of_possession<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.proof_of_possession = Some(field.into().into());
        }
        ///Sets `proof_of_possession` with the provided value.
        pub fn with_proof_of_possession<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_proof_of_possession(field.into());
            self
        }
        ///If `network_public_key` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn network_public_key_opt(&self) -> Option<&[u8]> {
            self.network_public_key.as_ref().map(|field| field as _)
        }
        ///Sets `network_public_key` with the provided value.
        pub fn set_network_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.network_public_key = Some(field.into().into());
        }
        ///Sets `network_public_key` with the provided value.
        pub fn with_network_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_network_public_key(field.into());
            self
        }
        ///If `worker_public_key` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn worker_public_key_opt(&self) -> Option<&[u8]> {
            self.worker_public_key.as_ref().map(|field| field as _)
        }
        ///Sets `worker_public_key` with the provided value.
        pub fn set_worker_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.worker_public_key = Some(field.into().into());
        }
        ///Sets `worker_public_key` with the provided value.
        pub fn with_worker_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_worker_public_key(field.into());
            self
        }
        ///If `network_address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn network_address_opt_mut(&mut self) -> Option<&mut String> {
            self.network_address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `network_address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn network_address_mut(&mut self) -> &mut String {
            self.network_address.get_or_insert_default()
        }
        ///If `network_address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn network_address_opt(&self) -> Option<&str> {
            self.network_address.as_ref().map(|field| field as _)
        }
        ///Sets `network_address` with the provided value.
        pub fn set_network_address<T: Into<String>>(&mut self, field: T) {
            self.network_address = Some(field.into().into());
        }
        ///Sets `network_address` with the provided value.
        pub fn with_network_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_network_address(field.into());
            self
        }
        ///If `p2p_address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn p2p_address_opt_mut(&mut self) -> Option<&mut String> {
            self.p2p_address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `p2p_address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn p2p_address_mut(&mut self) -> &mut String {
            self.p2p_address.get_or_insert_default()
        }
        ///If `p2p_address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn p2p_address_opt(&self) -> Option<&str> {
            self.p2p_address.as_ref().map(|field| field as _)
        }
        ///Sets `p2p_address` with the provided value.
        pub fn set_p2p_address<T: Into<String>>(&mut self, field: T) {
            self.p2p_address = Some(field.into().into());
        }
        ///Sets `p2p_address` with the provided value.
        pub fn with_p2p_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_p2p_address(field.into());
            self
        }
        ///If `primary_address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn primary_address_opt_mut(&mut self) -> Option<&mut String> {
            self.primary_address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `primary_address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn primary_address_mut(&mut self) -> &mut String {
            self.primary_address.get_or_insert_default()
        }
        ///If `primary_address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn primary_address_opt(&self) -> Option<&str> {
            self.primary_address.as_ref().map(|field| field as _)
        }
        ///Sets `primary_address` with the provided value.
        pub fn set_primary_address<T: Into<String>>(&mut self, field: T) {
            self.primary_address = Some(field.into().into());
        }
        ///Sets `primary_address` with the provided value.
        pub fn with_primary_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_primary_address(field.into());
            self
        }
        ///If `worker_address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn worker_address_opt_mut(&mut self) -> Option<&mut String> {
            self.worker_address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `worker_address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn worker_address_mut(&mut self) -> &mut String {
            self.worker_address.get_or_insert_default()
        }
        ///If `worker_address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn worker_address_opt(&self) -> Option<&str> {
            self.worker_address.as_ref().map(|field| field as _)
        }
        ///Sets `worker_address` with the provided value.
        pub fn set_worker_address<T: Into<String>>(&mut self, field: T) {
            self.worker_address = Some(field.into().into());
        }
        ///Sets `worker_address` with the provided value.
        pub fn with_worker_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_worker_address(field.into());
            self
        }
        ///If `next_epoch_protocol_public_key` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_protocol_public_key_opt(&self) -> Option<&[u8]> {
            self.next_epoch_protocol_public_key.as_ref().map(|field| field as _)
        }
        ///Sets `next_epoch_protocol_public_key` with the provided value.
        pub fn set_next_epoch_protocol_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.next_epoch_protocol_public_key = Some(field.into().into());
        }
        ///Sets `next_epoch_protocol_public_key` with the provided value.
        pub fn with_next_epoch_protocol_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_protocol_public_key(field.into());
            self
        }
        ///If `next_epoch_proof_of_possession` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_proof_of_possession_opt(&self) -> Option<&[u8]> {
            self.next_epoch_proof_of_possession.as_ref().map(|field| field as _)
        }
        ///Sets `next_epoch_proof_of_possession` with the provided value.
        pub fn set_next_epoch_proof_of_possession<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.next_epoch_proof_of_possession = Some(field.into().into());
        }
        ///Sets `next_epoch_proof_of_possession` with the provided value.
        pub fn with_next_epoch_proof_of_possession<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_proof_of_possession(field.into());
            self
        }
        ///If `next_epoch_network_public_key` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_network_public_key_opt(&self) -> Option<&[u8]> {
            self.next_epoch_network_public_key.as_ref().map(|field| field as _)
        }
        ///Sets `next_epoch_network_public_key` with the provided value.
        pub fn set_next_epoch_network_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.next_epoch_network_public_key = Some(field.into().into());
        }
        ///Sets `next_epoch_network_public_key` with the provided value.
        pub fn with_next_epoch_network_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_network_public_key(field.into());
            self
        }
        ///If `next_epoch_worker_public_key` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_worker_public_key_opt(&self) -> Option<&[u8]> {
            self.next_epoch_worker_public_key.as_ref().map(|field| field as _)
        }
        ///Sets `next_epoch_worker_public_key` with the provided value.
        pub fn set_next_epoch_worker_public_key<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.next_epoch_worker_public_key = Some(field.into().into());
        }
        ///Sets `next_epoch_worker_public_key` with the provided value.
        pub fn with_next_epoch_worker_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_worker_public_key(field.into());
            self
        }
        ///If `next_epoch_network_address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn next_epoch_network_address_opt_mut(&mut self) -> Option<&mut String> {
            self.next_epoch_network_address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `next_epoch_network_address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn next_epoch_network_address_mut(&mut self) -> &mut String {
            self.next_epoch_network_address.get_or_insert_default()
        }
        ///If `next_epoch_network_address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_network_address_opt(&self) -> Option<&str> {
            self.next_epoch_network_address.as_ref().map(|field| field as _)
        }
        ///Sets `next_epoch_network_address` with the provided value.
        pub fn set_next_epoch_network_address<T: Into<String>>(&mut self, field: T) {
            self.next_epoch_network_address = Some(field.into().into());
        }
        ///Sets `next_epoch_network_address` with the provided value.
        pub fn with_next_epoch_network_address<T: Into<String>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_network_address(field.into());
            self
        }
        ///If `next_epoch_p2p_address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn next_epoch_p2p_address_opt_mut(&mut self) -> Option<&mut String> {
            self.next_epoch_p2p_address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `next_epoch_p2p_address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn next_epoch_p2p_address_mut(&mut self) -> &mut String {
            self.next_epoch_p2p_address.get_or_insert_default()
        }
        ///If `next_epoch_p2p_address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_p2p_address_opt(&self) -> Option<&str> {
            self.next_epoch_p2p_address.as_ref().map(|field| field as _)
        }
        ///Sets `next_epoch_p2p_address` with the provided value.
        pub fn set_next_epoch_p2p_address<T: Into<String>>(&mut self, field: T) {
            self.next_epoch_p2p_address = Some(field.into().into());
        }
        ///Sets `next_epoch_p2p_address` with the provided value.
        pub fn with_next_epoch_p2p_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_next_epoch_p2p_address(field.into());
            self
        }
        ///If `next_epoch_primary_address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn next_epoch_primary_address_opt_mut(&mut self) -> Option<&mut String> {
            self.next_epoch_primary_address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `next_epoch_primary_address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn next_epoch_primary_address_mut(&mut self) -> &mut String {
            self.next_epoch_primary_address.get_or_insert_default()
        }
        ///If `next_epoch_primary_address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_primary_address_opt(&self) -> Option<&str> {
            self.next_epoch_primary_address.as_ref().map(|field| field as _)
        }
        ///Sets `next_epoch_primary_address` with the provided value.
        pub fn set_next_epoch_primary_address<T: Into<String>>(&mut self, field: T) {
            self.next_epoch_primary_address = Some(field.into().into());
        }
        ///Sets `next_epoch_primary_address` with the provided value.
        pub fn with_next_epoch_primary_address<T: Into<String>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_primary_address(field.into());
            self
        }
        ///If `next_epoch_worker_address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn next_epoch_worker_address_opt_mut(&mut self) -> Option<&mut String> {
            self.next_epoch_worker_address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `next_epoch_worker_address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn next_epoch_worker_address_mut(&mut self) -> &mut String {
            self.next_epoch_worker_address.get_or_insert_default()
        }
        ///If `next_epoch_worker_address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_worker_address_opt(&self) -> Option<&str> {
            self.next_epoch_worker_address.as_ref().map(|field| field as _)
        }
        ///Sets `next_epoch_worker_address` with the provided value.
        pub fn set_next_epoch_worker_address<T: Into<String>>(&mut self, field: T) {
            self.next_epoch_worker_address = Some(field.into().into());
        }
        ///Sets `next_epoch_worker_address` with the provided value.
        pub fn with_next_epoch_worker_address<T: Into<String>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_epoch_worker_address(field.into());
            self
        }
        ///Returns the value of `metadata_extra_fields`, or the default value if `metadata_extra_fields` is unset.
        pub fn metadata_extra_fields(&self) -> &super::MoveTable {
            self.metadata_extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `metadata_extra_fields` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn metadata_extra_fields_opt_mut(
            &mut self,
        ) -> Option<&mut super::MoveTable> {
            self.metadata_extra_fields.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `metadata_extra_fields`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn metadata_extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.metadata_extra_fields.get_or_insert_default()
        }
        ///If `metadata_extra_fields` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn metadata_extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.metadata_extra_fields.as_ref().map(|field| field as _)
        }
        ///Sets `metadata_extra_fields` with the provided value.
        pub fn set_metadata_extra_fields<T: Into<super::MoveTable>>(
            &mut self,
            field: T,
        ) {
            self.metadata_extra_fields = Some(field.into().into());
        }
        ///Sets `metadata_extra_fields` with the provided value.
        pub fn with_metadata_extra_fields<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_metadata_extra_fields(field.into());
            self
        }
        ///If `voting_power` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn voting_power_opt_mut(&mut self) -> Option<&mut u64> {
            self.voting_power.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `voting_power`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn voting_power_mut(&mut self) -> &mut u64 {
            self.voting_power.get_or_insert_default()
        }
        ///If `voting_power` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn voting_power_opt(&self) -> Option<u64> {
            self.voting_power.as_ref().map(|field| *field)
        }
        ///Sets `voting_power` with the provided value.
        pub fn set_voting_power(&mut self, field: u64) {
            self.voting_power = Some(field);
        }
        ///Sets `voting_power` with the provided value.
        pub fn with_voting_power(mut self, field: u64) -> Self {
            self.set_voting_power(field);
            self
        }
        ///If `operation_cap_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn operation_cap_id_opt_mut(&mut self) -> Option<&mut String> {
            self.operation_cap_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `operation_cap_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn operation_cap_id_mut(&mut self) -> &mut String {
            self.operation_cap_id.get_or_insert_default()
        }
        ///If `operation_cap_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn operation_cap_id_opt(&self) -> Option<&str> {
            self.operation_cap_id.as_ref().map(|field| field as _)
        }
        ///Sets `operation_cap_id` with the provided value.
        pub fn set_operation_cap_id<T: Into<String>>(&mut self, field: T) {
            self.operation_cap_id = Some(field.into().into());
        }
        ///Sets `operation_cap_id` with the provided value.
        pub fn with_operation_cap_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_operation_cap_id(field.into());
            self
        }
        ///If `gas_price` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn gas_price_opt_mut(&mut self) -> Option<&mut u64> {
            self.gas_price.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `gas_price`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn gas_price_mut(&mut self) -> &mut u64 {
            self.gas_price.get_or_insert_default()
        }
        ///If `gas_price` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn gas_price_opt(&self) -> Option<u64> {
            self.gas_price.as_ref().map(|field| *field)
        }
        ///Sets `gas_price` with the provided value.
        pub fn set_gas_price(&mut self, field: u64) {
            self.gas_price = Some(field);
        }
        ///Sets `gas_price` with the provided value.
        pub fn with_gas_price(mut self, field: u64) -> Self {
            self.set_gas_price(field);
            self
        }
        ///Returns the value of `staking_pool`, or the default value if `staking_pool` is unset.
        pub fn staking_pool(&self) -> &super::StakingPool {
            self.staking_pool
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::StakingPool::default_instance() as _)
        }
        ///If `staking_pool` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn staking_pool_opt_mut(&mut self) -> Option<&mut super::StakingPool> {
            self.staking_pool.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `staking_pool`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn staking_pool_mut(&mut self) -> &mut super::StakingPool {
            self.staking_pool.get_or_insert_default()
        }
        ///If `staking_pool` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn staking_pool_opt(&self) -> Option<&super::StakingPool> {
            self.staking_pool.as_ref().map(|field| field as _)
        }
        ///Sets `staking_pool` with the provided value.
        pub fn set_staking_pool<T: Into<super::StakingPool>>(&mut self, field: T) {
            self.staking_pool = Some(field.into().into());
        }
        ///Sets `staking_pool` with the provided value.
        pub fn with_staking_pool<T: Into<super::StakingPool>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_staking_pool(field.into());
            self
        }
        ///If `commission_rate` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn commission_rate_opt_mut(&mut self) -> Option<&mut u64> {
            self.commission_rate.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `commission_rate`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn commission_rate_mut(&mut self) -> &mut u64 {
            self.commission_rate.get_or_insert_default()
        }
        ///If `commission_rate` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn commission_rate_opt(&self) -> Option<u64> {
            self.commission_rate.as_ref().map(|field| *field)
        }
        ///Sets `commission_rate` with the provided value.
        pub fn set_commission_rate(&mut self, field: u64) {
            self.commission_rate = Some(field);
        }
        ///Sets `commission_rate` with the provided value.
        pub fn with_commission_rate(mut self, field: u64) -> Self {
            self.set_commission_rate(field);
            self
        }
        ///If `next_epoch_stake` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn next_epoch_stake_opt_mut(&mut self) -> Option<&mut u64> {
            self.next_epoch_stake.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `next_epoch_stake`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn next_epoch_stake_mut(&mut self) -> &mut u64 {
            self.next_epoch_stake.get_or_insert_default()
        }
        ///If `next_epoch_stake` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_stake_opt(&self) -> Option<u64> {
            self.next_epoch_stake.as_ref().map(|field| *field)
        }
        ///Sets `next_epoch_stake` with the provided value.
        pub fn set_next_epoch_stake(&mut self, field: u64) {
            self.next_epoch_stake = Some(field);
        }
        ///Sets `next_epoch_stake` with the provided value.
        pub fn with_next_epoch_stake(mut self, field: u64) -> Self {
            self.set_next_epoch_stake(field);
            self
        }
        ///If `next_epoch_gas_price` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn next_epoch_gas_price_opt_mut(&mut self) -> Option<&mut u64> {
            self.next_epoch_gas_price.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `next_epoch_gas_price`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn next_epoch_gas_price_mut(&mut self) -> &mut u64 {
            self.next_epoch_gas_price.get_or_insert_default()
        }
        ///If `next_epoch_gas_price` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_gas_price_opt(&self) -> Option<u64> {
            self.next_epoch_gas_price.as_ref().map(|field| *field)
        }
        ///Sets `next_epoch_gas_price` with the provided value.
        pub fn set_next_epoch_gas_price(&mut self, field: u64) {
            self.next_epoch_gas_price = Some(field);
        }
        ///Sets `next_epoch_gas_price` with the provided value.
        pub fn with_next_epoch_gas_price(mut self, field: u64) -> Self {
            self.set_next_epoch_gas_price(field);
            self
        }
        ///If `next_epoch_commission_rate` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn next_epoch_commission_rate_opt_mut(&mut self) -> Option<&mut u64> {
            self.next_epoch_commission_rate.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `next_epoch_commission_rate`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn next_epoch_commission_rate_mut(&mut self) -> &mut u64 {
            self.next_epoch_commission_rate.get_or_insert_default()
        }
        ///If `next_epoch_commission_rate` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_epoch_commission_rate_opt(&self) -> Option<u64> {
            self.next_epoch_commission_rate.as_ref().map(|field| *field)
        }
        ///Sets `next_epoch_commission_rate` with the provided value.
        pub fn set_next_epoch_commission_rate(&mut self, field: u64) {
            self.next_epoch_commission_rate = Some(field);
        }
        ///Sets `next_epoch_commission_rate` with the provided value.
        pub fn with_next_epoch_commission_rate(mut self, field: u64) -> Self {
            self.set_next_epoch_commission_rate(field);
            self
        }
        ///Returns the value of `extra_fields`, or the default value if `extra_fields` is unset.
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `extra_fields` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `extra_fields`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        ///If `extra_fields` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        ///Sets `extra_fields` with the provided value.
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        ///Sets `extra_fields` with the provided value.
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field.into());
            self
        }
    }
    impl super::ValidatorAggregatedSignature {
        pub const fn const_default() -> Self {
            Self {
                epoch: None,
                signature: None,
                bitmap: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ValidatorAggregatedSignature = super::ValidatorAggregatedSignature::const_default();
            &DEFAULT
        }
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///If `signature` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn signature_opt(&self) -> Option<&[u8]> {
            self.signature.as_ref().map(|field| field as _)
        }
        ///Sets `signature` with the provided value.
        pub fn set_signature<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        ///Sets `signature` with the provided value.
        pub fn with_signature<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field.into());
            self
        }
        ///If `bitmap` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn bitmap_opt(&self) -> Option<&[u8]> {
            self.bitmap.as_ref().map(|field| field as _)
        }
        ///Sets `bitmap` with the provided value.
        pub fn set_bitmap<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.bitmap = Some(field.into().into());
        }
        ///Sets `bitmap` with the provided value.
        pub fn with_bitmap<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_bitmap(field.into());
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
        ///If `epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn epoch_mut(&mut self) -> &mut u64 {
            self.epoch.get_or_insert_default()
        }
        ///If `epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn epoch_opt(&self) -> Option<u64> {
            self.epoch.as_ref().map(|field| *field)
        }
        ///Sets `epoch` with the provided value.
        pub fn set_epoch(&mut self, field: u64) {
            self.epoch = Some(field);
        }
        ///Sets `epoch` with the provided value.
        pub fn with_epoch(mut self, field: u64) -> Self {
            self.set_epoch(field);
            self
        }
        ///Returns the value of `members`, or the default value if `members` is unset.
        pub fn members(&self) -> &[super::ValidatorCommitteeMember] {
            &self.members
        }
        ///Returns a mutable reference to `members`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn members_mut(&mut self) -> &mut Vec<super::ValidatorCommitteeMember> {
            &mut self.members
        }
        ///Sets `members` with the provided value.
        pub fn set_members(&mut self, field: Vec<super::ValidatorCommitteeMember>) {
            self.members = field;
        }
        ///Sets `members` with the provided value.
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
        ///If `public_key` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn public_key_opt(&self) -> Option<&[u8]> {
            self.public_key.as_ref().map(|field| field as _)
        }
        ///Sets `public_key` with the provided value.
        pub fn set_public_key<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.public_key = Some(field.into().into());
        }
        ///Sets `public_key` with the provided value.
        pub fn with_public_key<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_public_key(field.into());
            self
        }
        ///If `weight` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn weight_opt_mut(&mut self) -> Option<&mut u64> {
            self.weight.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `weight`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn weight_mut(&mut self) -> &mut u64 {
            self.weight.get_or_insert_default()
        }
        ///If `weight` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn weight_opt(&self) -> Option<u64> {
            self.weight.as_ref().map(|field| *field)
        }
        ///Sets `weight` with the provided value.
        pub fn set_weight(&mut self, field: u64) {
            self.weight = Some(field);
        }
        ///Sets `weight` with the provided value.
        pub fn with_weight(mut self, field: u64) -> Self {
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
        ///If `validator` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn validator_opt(&self) -> Option<&[u8]> {
            self.validator.as_ref().map(|field| field as _)
        }
        ///Sets `validator` with the provided value.
        pub fn set_validator<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.validator = Some(field.into().into());
        }
        ///Sets `validator` with the provided value.
        pub fn with_validator<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_validator(field.into());
            self
        }
        ///If `duration` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn duration_opt_mut(&mut self) -> Option<&mut ::prost_types::Duration> {
            self.duration.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `duration`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn duration_mut(&mut self) -> &mut ::prost_types::Duration {
            self.duration.get_or_insert_default()
        }
        ///If `duration` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn duration_opt(&self) -> Option<&::prost_types::Duration> {
            self.duration.as_ref().map(|field| field as _)
        }
        ///Sets `duration` with the provided value.
        pub fn set_duration<T: Into<::prost_types::Duration>>(&mut self, field: T) {
            self.duration = Some(field.into().into());
        }
        ///Sets `duration` with the provided value.
        pub fn with_duration<T: Into<::prost_types::Duration>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_duration(field.into());
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
        ///If `reported` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn reported_opt_mut(&mut self) -> Option<&mut String> {
            self.reported.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `reported`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn reported_mut(&mut self) -> &mut String {
            self.reported.get_or_insert_default()
        }
        ///If `reported` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn reported_opt(&self) -> Option<&str> {
            self.reported.as_ref().map(|field| field as _)
        }
        ///Sets `reported` with the provided value.
        pub fn set_reported<T: Into<String>>(&mut self, field: T) {
            self.reported = Some(field.into().into());
        }
        ///Sets `reported` with the provided value.
        pub fn with_reported<T: Into<String>>(mut self, field: T) -> Self {
            self.set_reported(field.into());
            self
        }
        ///Returns the value of `reporters`, or the default value if `reporters` is unset.
        pub fn reporters(&self) -> &[String] {
            &self.reporters
        }
        ///Returns a mutable reference to `reporters`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn reporters_mut(&mut self) -> &mut Vec<String> {
            &mut self.reporters
        }
        ///Sets `reporters` with the provided value.
        pub fn set_reporters(&mut self, field: Vec<String>) {
            self.reporters = field;
        }
        ///Sets `reporters` with the provided value.
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
        ///If `total_stake` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn total_stake_opt_mut(&mut self) -> Option<&mut u64> {
            self.total_stake.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `total_stake`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn total_stake_mut(&mut self) -> &mut u64 {
            self.total_stake.get_or_insert_default()
        }
        ///If `total_stake` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn total_stake_opt(&self) -> Option<u64> {
            self.total_stake.as_ref().map(|field| *field)
        }
        ///Sets `total_stake` with the provided value.
        pub fn set_total_stake(&mut self, field: u64) {
            self.total_stake = Some(field);
        }
        ///Sets `total_stake` with the provided value.
        pub fn with_total_stake(mut self, field: u64) -> Self {
            self.set_total_stake(field);
            self
        }
        ///Returns the value of `active_validators`, or the default value if `active_validators` is unset.
        pub fn active_validators(&self) -> &[super::Validator] {
            &self.active_validators
        }
        ///Returns a mutable reference to `active_validators`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn active_validators_mut(&mut self) -> &mut Vec<super::Validator> {
            &mut self.active_validators
        }
        ///Sets `active_validators` with the provided value.
        pub fn set_active_validators(&mut self, field: Vec<super::Validator>) {
            self.active_validators = field;
        }
        ///Sets `active_validators` with the provided value.
        pub fn with_active_validators(mut self, field: Vec<super::Validator>) -> Self {
            self.set_active_validators(field);
            self
        }
        ///Returns the value of `pending_active_validators`, or the default value if `pending_active_validators` is unset.
        pub fn pending_active_validators(&self) -> &super::MoveTable {
            self.pending_active_validators
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `pending_active_validators` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn pending_active_validators_opt_mut(
            &mut self,
        ) -> Option<&mut super::MoveTable> {
            self.pending_active_validators.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `pending_active_validators`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn pending_active_validators_mut(&mut self) -> &mut super::MoveTable {
            self.pending_active_validators.get_or_insert_default()
        }
        ///If `pending_active_validators` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn pending_active_validators_opt(&self) -> Option<&super::MoveTable> {
            self.pending_active_validators.as_ref().map(|field| field as _)
        }
        ///Sets `pending_active_validators` with the provided value.
        pub fn set_pending_active_validators<T: Into<super::MoveTable>>(
            &mut self,
            field: T,
        ) {
            self.pending_active_validators = Some(field.into().into());
        }
        ///Sets `pending_active_validators` with the provided value.
        pub fn with_pending_active_validators<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_pending_active_validators(field.into());
            self
        }
        ///Returns the value of `pending_removals`, or the default value if `pending_removals` is unset.
        pub fn pending_removals(&self) -> &[u64] {
            &self.pending_removals
        }
        ///Returns a mutable reference to `pending_removals`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn pending_removals_mut(&mut self) -> &mut Vec<u64> {
            &mut self.pending_removals
        }
        ///Sets `pending_removals` with the provided value.
        pub fn set_pending_removals(&mut self, field: Vec<u64>) {
            self.pending_removals = field;
        }
        ///Sets `pending_removals` with the provided value.
        pub fn with_pending_removals(mut self, field: Vec<u64>) -> Self {
            self.set_pending_removals(field);
            self
        }
        ///Returns the value of `staking_pool_mappings`, or the default value if `staking_pool_mappings` is unset.
        pub fn staking_pool_mappings(&self) -> &super::MoveTable {
            self.staking_pool_mappings
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `staking_pool_mappings` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn staking_pool_mappings_opt_mut(
            &mut self,
        ) -> Option<&mut super::MoveTable> {
            self.staking_pool_mappings.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `staking_pool_mappings`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn staking_pool_mappings_mut(&mut self) -> &mut super::MoveTable {
            self.staking_pool_mappings.get_or_insert_default()
        }
        ///If `staking_pool_mappings` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn staking_pool_mappings_opt(&self) -> Option<&super::MoveTable> {
            self.staking_pool_mappings.as_ref().map(|field| field as _)
        }
        ///Sets `staking_pool_mappings` with the provided value.
        pub fn set_staking_pool_mappings<T: Into<super::MoveTable>>(
            &mut self,
            field: T,
        ) {
            self.staking_pool_mappings = Some(field.into().into());
        }
        ///Sets `staking_pool_mappings` with the provided value.
        pub fn with_staking_pool_mappings<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_staking_pool_mappings(field.into());
            self
        }
        ///Returns the value of `inactive_validators`, or the default value if `inactive_validators` is unset.
        pub fn inactive_validators(&self) -> &super::MoveTable {
            self.inactive_validators
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `inactive_validators` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn inactive_validators_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.inactive_validators.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `inactive_validators`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn inactive_validators_mut(&mut self) -> &mut super::MoveTable {
            self.inactive_validators.get_or_insert_default()
        }
        ///If `inactive_validators` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn inactive_validators_opt(&self) -> Option<&super::MoveTable> {
            self.inactive_validators.as_ref().map(|field| field as _)
        }
        ///Sets `inactive_validators` with the provided value.
        pub fn set_inactive_validators<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.inactive_validators = Some(field.into().into());
        }
        ///Sets `inactive_validators` with the provided value.
        pub fn with_inactive_validators<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_inactive_validators(field.into());
            self
        }
        ///Returns the value of `validator_candidates`, or the default value if `validator_candidates` is unset.
        pub fn validator_candidates(&self) -> &super::MoveTable {
            self.validator_candidates
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `validator_candidates` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn validator_candidates_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.validator_candidates.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `validator_candidates`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn validator_candidates_mut(&mut self) -> &mut super::MoveTable {
            self.validator_candidates.get_or_insert_default()
        }
        ///If `validator_candidates` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn validator_candidates_opt(&self) -> Option<&super::MoveTable> {
            self.validator_candidates.as_ref().map(|field| field as _)
        }
        ///Sets `validator_candidates` with the provided value.
        pub fn set_validator_candidates<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.validator_candidates = Some(field.into().into());
        }
        ///Sets `validator_candidates` with the provided value.
        pub fn with_validator_candidates<T: Into<super::MoveTable>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_validator_candidates(field.into());
            self
        }
        ///Returns the value of `at_risk_validators`, or the default value if `at_risk_validators` is unset.
        pub fn at_risk_validators(&self) -> &::std::collections::BTreeMap<String, u64> {
            &self.at_risk_validators
        }
        ///Returns a mutable reference to `at_risk_validators`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn at_risk_validators_mut(
            &mut self,
        ) -> &mut ::std::collections::BTreeMap<String, u64> {
            &mut self.at_risk_validators
        }
        ///Sets `at_risk_validators` with the provided value.
        pub fn set_at_risk_validators(
            &mut self,
            field: ::std::collections::BTreeMap<String, u64>,
        ) {
            self.at_risk_validators = field;
        }
        ///Sets `at_risk_validators` with the provided value.
        pub fn with_at_risk_validators(
            mut self,
            field: ::std::collections::BTreeMap<String, u64>,
        ) -> Self {
            self.set_at_risk_validators(field);
            self
        }
        ///Returns the value of `extra_fields`, or the default value if `extra_fields` is unset.
        pub fn extra_fields(&self) -> &super::MoveTable {
            self.extra_fields
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MoveTable::default_instance() as _)
        }
        ///If `extra_fields` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn extra_fields_opt_mut(&mut self) -> Option<&mut super::MoveTable> {
            self.extra_fields.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `extra_fields`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn extra_fields_mut(&mut self) -> &mut super::MoveTable {
            self.extra_fields.get_or_insert_default()
        }
        ///If `extra_fields` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn extra_fields_opt(&self) -> Option<&super::MoveTable> {
            self.extra_fields.as_ref().map(|field| field as _)
        }
        ///Sets `extra_fields` with the provided value.
        pub fn set_extra_fields<T: Into<super::MoveTable>>(&mut self, field: T) {
            self.extra_fields = Some(field.into().into());
        }
        ///Sets `extra_fields` with the provided value.
        pub fn with_extra_fields<T: Into<super::MoveTable>>(mut self, field: T) -> Self {
            self.set_extra_fields(field.into());
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
        ///If `name` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn name_opt_mut(&mut self) -> Option<&mut String> {
            self.name.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `name`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn name_mut(&mut self) -> &mut String {
            self.name.get_or_insert_default()
        }
        ///If `name` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn name_opt(&self) -> Option<&str> {
            self.name.as_ref().map(|field| field as _)
        }
        ///Sets `name` with the provided value.
        pub fn set_name<T: Into<String>>(&mut self, field: T) {
            self.name = Some(field.into().into());
        }
        ///Sets `name` with the provided value.
        pub fn with_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_name(field.into());
            self
        }
        ///If `position` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn position_opt_mut(&mut self) -> Option<&mut u32> {
            self.position.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `position`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn position_mut(&mut self) -> &mut u32 {
            self.position.get_or_insert_default()
        }
        ///If `position` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn position_opt(&self) -> Option<u32> {
            self.position.as_ref().map(|field| *field)
        }
        ///Sets `position` with the provided value.
        pub fn set_position(&mut self, field: u32) {
            self.position = Some(field);
        }
        ///Sets `position` with the provided value.
        pub fn with_position(mut self, field: u32) -> Self {
            self.set_position(field);
            self
        }
        ///Returns the value of `fields`, or the default value if `fields` is unset.
        pub fn fields(&self) -> &[super::FieldDescriptor] {
            &self.fields
        }
        ///Returns a mutable reference to `fields`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn fields_mut(&mut self) -> &mut Vec<super::FieldDescriptor> {
            &mut self.fields
        }
        ///Sets `fields` with the provided value.
        pub fn set_fields(&mut self, field: Vec<super::FieldDescriptor>) {
            self.fields = field;
        }
        ///Sets `fields` with the provided value.
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
        ///Returns the value of `message`, or the default value if `message` is unset.
        pub fn message(&self) -> &super::Bcs {
            self.message
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Bcs::default_instance() as _)
        }
        ///If `message` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn message_opt_mut(&mut self) -> Option<&mut super::Bcs> {
            self.message.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `message`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn message_mut(&mut self) -> &mut super::Bcs {
            self.message.get_or_insert_default()
        }
        ///If `message` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn message_opt(&self) -> Option<&super::Bcs> {
            self.message.as_ref().map(|field| field as _)
        }
        ///Sets `message` with the provided value.
        pub fn set_message<T: Into<super::Bcs>>(&mut self, field: T) {
            self.message = Some(field.into().into());
        }
        ///Sets `message` with the provided value.
        pub fn with_message<T: Into<super::Bcs>>(mut self, field: T) -> Self {
            self.set_message(field.into());
            self
        }
        ///Returns the value of `signature`, or the default value if `signature` is unset.
        pub fn signature(&self) -> &super::UserSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::UserSignature::default_instance() as _)
        }
        ///If `signature` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn signature_opt_mut(&mut self) -> Option<&mut super::UserSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `signature`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn signature_mut(&mut self) -> &mut super::UserSignature {
            self.signature.get_or_insert_default()
        }
        ///If `signature` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn signature_opt(&self) -> Option<&super::UserSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        ///Sets `signature` with the provided value.
        pub fn set_signature<T: Into<super::UserSignature>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        ///Sets `signature` with the provided value.
        pub fn with_signature<T: Into<super::UserSignature>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field.into());
            self
        }
        ///If `address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn address_opt_mut(&mut self) -> Option<&mut String> {
            self.address.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `address`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_mut(&mut self) -> &mut String {
            self.address.get_or_insert_default()
        }
        ///If `address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn address_opt(&self) -> Option<&str> {
            self.address.as_ref().map(|field| field as _)
        }
        ///Sets `address` with the provided value.
        pub fn set_address<T: Into<String>>(&mut self, field: T) {
            self.address = Some(field.into().into());
        }
        ///Sets `address` with the provided value.
        pub fn with_address<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address(field.into());
            self
        }
        ///Returns the value of `jwks`, or the default value if `jwks` is unset.
        pub fn jwks(&self) -> &[super::ActiveJwk] {
            &self.jwks
        }
        ///Returns a mutable reference to `jwks`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn jwks_mut(&mut self) -> &mut Vec<super::ActiveJwk> {
            &mut self.jwks
        }
        ///Sets `jwks` with the provided value.
        pub fn set_jwks(&mut self, field: Vec<super::ActiveJwk>) {
            self.jwks = field;
        }
        ///Sets `jwks` with the provided value.
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
        ///If `is_valid` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn is_valid_opt_mut(&mut self) -> Option<&mut bool> {
            self.is_valid.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `is_valid`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn is_valid_mut(&mut self) -> &mut bool {
            self.is_valid.get_or_insert_default()
        }
        ///If `is_valid` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn is_valid_opt(&self) -> Option<bool> {
            self.is_valid.as_ref().map(|field| *field)
        }
        ///Sets `is_valid` with the provided value.
        pub fn set_is_valid(&mut self, field: bool) {
            self.is_valid = Some(field);
        }
        ///Sets `is_valid` with the provided value.
        pub fn with_is_valid(mut self, field: bool) -> Self {
            self.set_is_valid(field);
            self
        }
        ///If `reason` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn reason_opt_mut(&mut self) -> Option<&mut String> {
            self.reason.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `reason`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn reason_mut(&mut self) -> &mut String {
            self.reason.get_or_insert_default()
        }
        ///If `reason` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn reason_opt(&self) -> Option<&str> {
            self.reason.as_ref().map(|field| field as _)
        }
        ///Sets `reason` with the provided value.
        pub fn set_reason<T: Into<String>>(&mut self, field: T) {
            self.reason = Some(field.into().into());
        }
        ///Sets `reason` with the provided value.
        pub fn with_reason<T: Into<String>>(mut self, field: T) -> Self {
            self.set_reason(field.into());
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
        ///If `object_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        ///If `object_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        ///Sets `object_id` with the provided value.
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        ///Sets `object_id` with the provided value.
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field.into());
            self
        }
        ///If `start_version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn start_version_opt_mut(&mut self) -> Option<&mut u64> {
            self.start_version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `start_version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn start_version_mut(&mut self) -> &mut u64 {
            self.start_version.get_or_insert_default()
        }
        ///If `start_version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn start_version_opt(&self) -> Option<u64> {
            self.start_version.as_ref().map(|field| *field)
        }
        ///Sets `start_version` with the provided value.
        pub fn set_start_version(&mut self, field: u64) {
            self.start_version = Some(field);
        }
        ///Sets `start_version` with the provided value.
        pub fn with_start_version(mut self, field: u64) -> Self {
            self.set_start_version(field);
            self
        }
        ///If `version` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn version_opt_mut(&mut self) -> Option<&mut u64> {
            self.version.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `version`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn version_mut(&mut self) -> &mut u64 {
            self.version.get_or_insert_default()
        }
        ///If `version` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn version_opt(&self) -> Option<u64> {
            self.version.as_ref().map(|field| *field)
        }
        ///Sets `version` with the provided value.
        pub fn set_version(&mut self, field: u64) {
            self.version = Some(field);
        }
        ///Sets `version` with the provided value.
        pub fn with_version(mut self, field: u64) -> Self {
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
                public_identifier: None,
                jwk_id: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ZkLoginAuthenticator = super::ZkLoginAuthenticator::const_default();
            &DEFAULT
        }
        ///Returns the value of `inputs`, or the default value if `inputs` is unset.
        pub fn inputs(&self) -> &super::ZkLoginInputs {
            self.inputs
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ZkLoginInputs::default_instance() as _)
        }
        ///If `inputs` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn inputs_opt_mut(&mut self) -> Option<&mut super::ZkLoginInputs> {
            self.inputs.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `inputs`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn inputs_mut(&mut self) -> &mut super::ZkLoginInputs {
            self.inputs.get_or_insert_default()
        }
        ///If `inputs` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn inputs_opt(&self) -> Option<&super::ZkLoginInputs> {
            self.inputs.as_ref().map(|field| field as _)
        }
        ///Sets `inputs` with the provided value.
        pub fn set_inputs<T: Into<super::ZkLoginInputs>>(&mut self, field: T) {
            self.inputs = Some(field.into().into());
        }
        ///Sets `inputs` with the provided value.
        pub fn with_inputs<T: Into<super::ZkLoginInputs>>(mut self, field: T) -> Self {
            self.set_inputs(field.into());
            self
        }
        ///If `max_epoch` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn max_epoch_opt_mut(&mut self) -> Option<&mut u64> {
            self.max_epoch.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `max_epoch`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn max_epoch_mut(&mut self) -> &mut u64 {
            self.max_epoch.get_or_insert_default()
        }
        ///If `max_epoch` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn max_epoch_opt(&self) -> Option<u64> {
            self.max_epoch.as_ref().map(|field| *field)
        }
        ///Sets `max_epoch` with the provided value.
        pub fn set_max_epoch(&mut self, field: u64) {
            self.max_epoch = Some(field);
        }
        ///Sets `max_epoch` with the provided value.
        pub fn with_max_epoch(mut self, field: u64) -> Self {
            self.set_max_epoch(field);
            self
        }
        ///Returns the value of `signature`, or the default value if `signature` is unset.
        pub fn signature(&self) -> &super::SimpleSignature {
            self.signature
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::SimpleSignature::default_instance() as _)
        }
        ///If `signature` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn signature_opt_mut(&mut self) -> Option<&mut super::SimpleSignature> {
            self.signature.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `signature`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn signature_mut(&mut self) -> &mut super::SimpleSignature {
            self.signature.get_or_insert_default()
        }
        ///If `signature` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn signature_opt(&self) -> Option<&super::SimpleSignature> {
            self.signature.as_ref().map(|field| field as _)
        }
        ///Sets `signature` with the provided value.
        pub fn set_signature<T: Into<super::SimpleSignature>>(&mut self, field: T) {
            self.signature = Some(field.into().into());
        }
        ///Sets `signature` with the provided value.
        pub fn with_signature<T: Into<super::SimpleSignature>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_signature(field.into());
            self
        }
        ///Returns the value of `public_identifier`, or the default value if `public_identifier` is unset.
        pub fn public_identifier(&self) -> &super::ZkLoginPublicIdentifier {
            self.public_identifier
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::ZkLoginPublicIdentifier::default_instance() as _
                })
        }
        ///If `public_identifier` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn public_identifier_opt_mut(
            &mut self,
        ) -> Option<&mut super::ZkLoginPublicIdentifier> {
            self.public_identifier.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `public_identifier`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn public_identifier_mut(&mut self) -> &mut super::ZkLoginPublicIdentifier {
            self.public_identifier.get_or_insert_default()
        }
        ///If `public_identifier` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn public_identifier_opt(&self) -> Option<&super::ZkLoginPublicIdentifier> {
            self.public_identifier.as_ref().map(|field| field as _)
        }
        ///Sets `public_identifier` with the provided value.
        pub fn set_public_identifier<T: Into<super::ZkLoginPublicIdentifier>>(
            &mut self,
            field: T,
        ) {
            self.public_identifier = Some(field.into().into());
        }
        ///Sets `public_identifier` with the provided value.
        pub fn with_public_identifier<T: Into<super::ZkLoginPublicIdentifier>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_public_identifier(field.into());
            self
        }
        ///Returns the value of `jwk_id`, or the default value if `jwk_id` is unset.
        pub fn jwk_id(&self) -> &super::JwkId {
            self.jwk_id
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::JwkId::default_instance() as _)
        }
        ///If `jwk_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn jwk_id_opt_mut(&mut self) -> Option<&mut super::JwkId> {
            self.jwk_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `jwk_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn jwk_id_mut(&mut self) -> &mut super::JwkId {
            self.jwk_id.get_or_insert_default()
        }
        ///If `jwk_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn jwk_id_opt(&self) -> Option<&super::JwkId> {
            self.jwk_id.as_ref().map(|field| field as _)
        }
        ///Sets `jwk_id` with the provided value.
        pub fn set_jwk_id<T: Into<super::JwkId>>(&mut self, field: T) {
            self.jwk_id = Some(field.into().into());
        }
        ///Sets `jwk_id` with the provided value.
        pub fn with_jwk_id<T: Into<super::JwkId>>(mut self, field: T) -> Self {
            self.set_jwk_id(field.into());
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
        ///If `value` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn value_opt_mut(&mut self) -> Option<&mut String> {
            self.value.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `value`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn value_mut(&mut self) -> &mut String {
            self.value.get_or_insert_default()
        }
        ///If `value` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn value_opt(&self) -> Option<&str> {
            self.value.as_ref().map(|field| field as _)
        }
        ///Sets `value` with the provided value.
        pub fn set_value<T: Into<String>>(&mut self, field: T) {
            self.value = Some(field.into().into());
        }
        ///Sets `value` with the provided value.
        pub fn with_value<T: Into<String>>(mut self, field: T) -> Self {
            self.set_value(field.into());
            self
        }
        ///If `index_mod_4` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn index_mod_4_opt_mut(&mut self) -> Option<&mut u32> {
            self.index_mod_4.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `index_mod_4`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn index_mod_4_mut(&mut self) -> &mut u32 {
            self.index_mod_4.get_or_insert_default()
        }
        ///If `index_mod_4` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn index_mod_4_opt(&self) -> Option<u32> {
            self.index_mod_4.as_ref().map(|field| *field)
        }
        ///Sets `index_mod_4` with the provided value.
        pub fn set_index_mod_4(&mut self, field: u32) {
            self.index_mod_4 = Some(field);
        }
        ///Sets `index_mod_4` with the provided value.
        pub fn with_index_mod_4(mut self, field: u32) -> Self {
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
        ///Returns the value of `proof_points`, or the default value if `proof_points` is unset.
        pub fn proof_points(&self) -> &super::ZkLoginProof {
            self.proof_points
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ZkLoginProof::default_instance() as _)
        }
        ///If `proof_points` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn proof_points_opt_mut(&mut self) -> Option<&mut super::ZkLoginProof> {
            self.proof_points.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `proof_points`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn proof_points_mut(&mut self) -> &mut super::ZkLoginProof {
            self.proof_points.get_or_insert_default()
        }
        ///If `proof_points` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn proof_points_opt(&self) -> Option<&super::ZkLoginProof> {
            self.proof_points.as_ref().map(|field| field as _)
        }
        ///Sets `proof_points` with the provided value.
        pub fn set_proof_points<T: Into<super::ZkLoginProof>>(&mut self, field: T) {
            self.proof_points = Some(field.into().into());
        }
        ///Sets `proof_points` with the provided value.
        pub fn with_proof_points<T: Into<super::ZkLoginProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_proof_points(field.into());
            self
        }
        ///Returns the value of `iss_base64_details`, or the default value if `iss_base64_details` is unset.
        pub fn iss_base64_details(&self) -> &super::ZkLoginClaim {
            self.iss_base64_details
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::ZkLoginClaim::default_instance() as _)
        }
        ///If `iss_base64_details` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn iss_base64_details_opt_mut(
            &mut self,
        ) -> Option<&mut super::ZkLoginClaim> {
            self.iss_base64_details.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `iss_base64_details`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn iss_base64_details_mut(&mut self) -> &mut super::ZkLoginClaim {
            self.iss_base64_details.get_or_insert_default()
        }
        ///If `iss_base64_details` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn iss_base64_details_opt(&self) -> Option<&super::ZkLoginClaim> {
            self.iss_base64_details.as_ref().map(|field| field as _)
        }
        ///Sets `iss_base64_details` with the provided value.
        pub fn set_iss_base64_details<T: Into<super::ZkLoginClaim>>(
            &mut self,
            field: T,
        ) {
            self.iss_base64_details = Some(field.into().into());
        }
        ///Sets `iss_base64_details` with the provided value.
        pub fn with_iss_base64_details<T: Into<super::ZkLoginClaim>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_iss_base64_details(field.into());
            self
        }
        ///If `header_base64` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn header_base64_opt_mut(&mut self) -> Option<&mut String> {
            self.header_base64.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `header_base64`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn header_base64_mut(&mut self) -> &mut String {
            self.header_base64.get_or_insert_default()
        }
        ///If `header_base64` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn header_base64_opt(&self) -> Option<&str> {
            self.header_base64.as_ref().map(|field| field as _)
        }
        ///Sets `header_base64` with the provided value.
        pub fn set_header_base64<T: Into<String>>(&mut self, field: T) {
            self.header_base64 = Some(field.into().into());
        }
        ///Sets `header_base64` with the provided value.
        pub fn with_header_base64<T: Into<String>>(mut self, field: T) -> Self {
            self.set_header_base64(field.into());
            self
        }
        ///If `address_seed` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn address_seed_opt_mut(&mut self) -> Option<&mut String> {
            self.address_seed.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `address_seed`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_seed_mut(&mut self) -> &mut String {
            self.address_seed.get_or_insert_default()
        }
        ///If `address_seed` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn address_seed_opt(&self) -> Option<&str> {
            self.address_seed.as_ref().map(|field| field as _)
        }
        ///Sets `address_seed` with the provided value.
        pub fn set_address_seed<T: Into<String>>(&mut self, field: T) {
            self.address_seed = Some(field.into().into());
        }
        ///Sets `address_seed` with the provided value.
        pub fn with_address_seed<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address_seed(field.into());
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
        ///Returns the value of `a`, or the default value if `a` is unset.
        pub fn a(&self) -> &super::CircomG1 {
            self.a
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CircomG1::default_instance() as _)
        }
        ///If `a` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn a_opt_mut(&mut self) -> Option<&mut super::CircomG1> {
            self.a.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `a`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn a_mut(&mut self) -> &mut super::CircomG1 {
            self.a.get_or_insert_default()
        }
        ///If `a` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn a_opt(&self) -> Option<&super::CircomG1> {
            self.a.as_ref().map(|field| field as _)
        }
        ///Sets `a` with the provided value.
        pub fn set_a<T: Into<super::CircomG1>>(&mut self, field: T) {
            self.a = Some(field.into().into());
        }
        ///Sets `a` with the provided value.
        pub fn with_a<T: Into<super::CircomG1>>(mut self, field: T) -> Self {
            self.set_a(field.into());
            self
        }
        ///Returns the value of `b`, or the default value if `b` is unset.
        pub fn b(&self) -> &super::CircomG2 {
            self.b
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CircomG2::default_instance() as _)
        }
        ///If `b` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn b_opt_mut(&mut self) -> Option<&mut super::CircomG2> {
            self.b.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `b`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn b_mut(&mut self) -> &mut super::CircomG2 {
            self.b.get_or_insert_default()
        }
        ///If `b` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn b_opt(&self) -> Option<&super::CircomG2> {
            self.b.as_ref().map(|field| field as _)
        }
        ///Sets `b` with the provided value.
        pub fn set_b<T: Into<super::CircomG2>>(&mut self, field: T) {
            self.b = Some(field.into().into());
        }
        ///Sets `b` with the provided value.
        pub fn with_b<T: Into<super::CircomG2>>(mut self, field: T) -> Self {
            self.set_b(field.into());
            self
        }
        ///Returns the value of `c`, or the default value if `c` is unset.
        pub fn c(&self) -> &super::CircomG1 {
            self.c
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::CircomG1::default_instance() as _)
        }
        ///If `c` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn c_opt_mut(&mut self) -> Option<&mut super::CircomG1> {
            self.c.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `c`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn c_mut(&mut self) -> &mut super::CircomG1 {
            self.c.get_or_insert_default()
        }
        ///If `c` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn c_opt(&self) -> Option<&super::CircomG1> {
            self.c.as_ref().map(|field| field as _)
        }
        ///Sets `c` with the provided value.
        pub fn set_c<T: Into<super::CircomG1>>(&mut self, field: T) {
            self.c = Some(field.into().into());
        }
        ///Sets `c` with the provided value.
        pub fn with_c<T: Into<super::CircomG1>>(mut self, field: T) -> Self {
            self.set_c(field.into());
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
        ///If `iss` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn iss_opt_mut(&mut self) -> Option<&mut String> {
            self.iss.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `iss`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn iss_mut(&mut self) -> &mut String {
            self.iss.get_or_insert_default()
        }
        ///If `iss` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn iss_opt(&self) -> Option<&str> {
            self.iss.as_ref().map(|field| field as _)
        }
        ///Sets `iss` with the provided value.
        pub fn set_iss<T: Into<String>>(&mut self, field: T) {
            self.iss = Some(field.into().into());
        }
        ///Sets `iss` with the provided value.
        pub fn with_iss<T: Into<String>>(mut self, field: T) -> Self {
            self.set_iss(field.into());
            self
        }
        ///If `address_seed` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn address_seed_opt_mut(&mut self) -> Option<&mut String> {
            self.address_seed.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `address_seed`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn address_seed_mut(&mut self) -> &mut String {
            self.address_seed.get_or_insert_default()
        }
        ///If `address_seed` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn address_seed_opt(&self) -> Option<&str> {
            self.address_seed.as_ref().map(|field| field as _)
        }
        ///Sets `address_seed` with the provided value.
        pub fn set_address_seed<T: Into<String>>(&mut self, field: T) {
            self.address_seed = Some(field.into().into());
        }
        ///Sets `address_seed` with the provided value.
        pub fn with_address_seed<T: Into<String>>(mut self, field: T) -> Self {
            self.set_address_seed(field.into());
            self
        }
    }
}
