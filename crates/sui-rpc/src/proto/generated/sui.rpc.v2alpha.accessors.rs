mod _accessor_impls {
    #![allow(clippy::useless_conversion)]
    impl super::AffectedAddressFilter {
        pub const fn const_default() -> Self {
            Self { address: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::AffectedAddressFilter = super::AffectedAddressFilter::const_default();
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
    impl super::AffectedObjectFilter {
        pub const fn const_default() -> Self {
            Self { object_id: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::AffectedObjectFilter = super::AffectedObjectFilter::const_default();
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
    }
    impl super::CheckpointItem {
        pub const fn const_default() -> Self {
            Self {
                cursor: None,
                checkpoint: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CheckpointItem = super::CheckpointItem::const_default();
            &DEFAULT
        }
        ///If `cursor` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn cursor_opt(&self) -> Option<&[u8]> {
            self.cursor.as_ref().map(|field| field as _)
        }
        ///Sets `cursor` with the provided value.
        pub fn set_cursor<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.cursor = Some(field.into().into());
        }
        ///Sets `cursor` with the provided value.
        pub fn with_cursor<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_cursor(field.into());
            self
        }
        ///Returns the value of `checkpoint`, or the default value if `checkpoint` is unset.
        pub fn checkpoint(&self) -> &super::super::v2::Checkpoint {
            self.checkpoint
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::super::v2::Checkpoint::default_instance() as _)
        }
        ///If `checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn checkpoint_opt_mut(
            &mut self,
        ) -> Option<&mut super::super::v2::Checkpoint> {
            self.checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn checkpoint_mut(&mut self) -> &mut super::super::v2::Checkpoint {
            self.checkpoint.get_or_insert_default()
        }
        ///If `checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn checkpoint_opt(&self) -> Option<&super::super::v2::Checkpoint> {
            self.checkpoint.as_ref().map(|field| field as _)
        }
        ///Sets `checkpoint` with the provided value.
        pub fn set_checkpoint<T: Into<super::super::v2::Checkpoint>>(
            &mut self,
            field: T,
        ) {
            self.checkpoint = Some(field.into().into());
        }
        ///Sets `checkpoint` with the provided value.
        pub fn with_checkpoint<T: Into<super::super::v2::Checkpoint>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_checkpoint(field.into());
            self
        }
    }
    impl super::EmitModuleFilter {
        pub const fn const_default() -> Self {
            Self { module: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EmitModuleFilter = super::EmitModuleFilter::const_default();
            &DEFAULT
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
    }
    impl super::EventFilter {
        pub const fn const_default() -> Self {
            Self { terms: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EventFilter = super::EventFilter::const_default();
            &DEFAULT
        }
        ///Returns the value of `terms`, or the default value if `terms` is unset.
        pub fn terms(&self) -> &[super::EventTerm] {
            &self.terms
        }
        ///Returns a mutable reference to `terms`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn terms_mut(&mut self) -> &mut Vec<super::EventTerm> {
            &mut self.terms
        }
        ///Sets `terms` with the provided value.
        pub fn set_terms(&mut self, field: Vec<super::EventTerm>) {
            self.terms = field;
        }
        ///Sets `terms` with the provided value.
        pub fn with_terms(mut self, field: Vec<super::EventTerm>) -> Self {
            self.set_terms(field);
            self
        }
    }
    impl super::EventItem {
        pub const fn const_default() -> Self {
            Self {
                cursor: None,
                checkpoint: None,
                event_index: None,
                transaction_digest: None,
                event: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EventItem = super::EventItem::const_default();
            &DEFAULT
        }
        ///If `cursor` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn cursor_opt(&self) -> Option<&[u8]> {
            self.cursor.as_ref().map(|field| field as _)
        }
        ///Sets `cursor` with the provided value.
        pub fn set_cursor<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.cursor = Some(field.into().into());
        }
        ///Sets `cursor` with the provided value.
        pub fn with_cursor<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_cursor(field.into());
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
        ///If `event_index` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn event_index_opt_mut(&mut self) -> Option<&mut u32> {
            self.event_index.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `event_index`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn event_index_mut(&mut self) -> &mut u32 {
            self.event_index.get_or_insert_default()
        }
        ///If `event_index` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn event_index_opt(&self) -> Option<u32> {
            self.event_index.as_ref().map(|field| *field)
        }
        ///Sets `event_index` with the provided value.
        pub fn set_event_index(&mut self, field: u32) {
            self.event_index = Some(field);
        }
        ///Sets `event_index` with the provided value.
        pub fn with_event_index(mut self, field: u32) -> Self {
            self.set_event_index(field);
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
        ///Returns the value of `event`, or the default value if `event` is unset.
        pub fn event(&self) -> &super::super::v2::Event {
            self.event
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::super::v2::Event::default_instance() as _)
        }
        ///If `event` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn event_opt_mut(&mut self) -> Option<&mut super::super::v2::Event> {
            self.event.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `event`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn event_mut(&mut self) -> &mut super::super::v2::Event {
            self.event.get_or_insert_default()
        }
        ///If `event` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn event_opt(&self) -> Option<&super::super::v2::Event> {
            self.event.as_ref().map(|field| field as _)
        }
        ///Sets `event` with the provided value.
        pub fn set_event<T: Into<super::super::v2::Event>>(&mut self, field: T) {
            self.event = Some(field.into().into());
        }
        ///Sets `event` with the provided value.
        pub fn with_event<T: Into<super::super::v2::Event>>(mut self, field: T) -> Self {
            self.set_event(field.into());
            self
        }
    }
    impl super::EventLiteral {
        pub const fn const_default() -> Self {
            Self { polarity: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EventLiteral = super::EventLiteral::const_default();
            &DEFAULT
        }
        ///Returns the value of `include`, or the default value if `include` is unset.
        pub fn include(&self) -> &super::EventPredicate {
            if let Some(super::event_literal::Polarity::Include(field)) = &self.polarity
            {
                field as _
            } else {
                super::EventPredicate::default_instance() as _
            }
        }
        ///If `include` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn include_opt(&self) -> Option<&super::EventPredicate> {
            if let Some(super::event_literal::Polarity::Include(field)) = &self.polarity
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `include` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn include_opt_mut(&mut self) -> Option<&mut super::EventPredicate> {
            if let Some(super::event_literal::Polarity::Include(field)) = &mut self
                .polarity
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `include`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn include_mut(&mut self) -> &mut super::EventPredicate {
            if self.include_opt_mut().is_none() {
                self.polarity = Some(
                    super::event_literal::Polarity::Include(
                        super::EventPredicate::default(),
                    ),
                );
            }
            self.include_opt_mut().unwrap()
        }
        ///Sets `include` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_include<T: Into<super::EventPredicate>>(&mut self, field: T) {
            self.polarity = Some(
                super::event_literal::Polarity::Include(field.into().into()),
            );
        }
        ///Sets `include` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_include<T: Into<super::EventPredicate>>(mut self, field: T) -> Self {
            self.set_include(field.into());
            self
        }
        ///Returns the value of `exclude`, or the default value if `exclude` is unset.
        pub fn exclude(&self) -> &super::EventPredicate {
            if let Some(super::event_literal::Polarity::Exclude(field)) = &self.polarity
            {
                field as _
            } else {
                super::EventPredicate::default_instance() as _
            }
        }
        ///If `exclude` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn exclude_opt(&self) -> Option<&super::EventPredicate> {
            if let Some(super::event_literal::Polarity::Exclude(field)) = &self.polarity
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `exclude` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn exclude_opt_mut(&mut self) -> Option<&mut super::EventPredicate> {
            if let Some(super::event_literal::Polarity::Exclude(field)) = &mut self
                .polarity
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `exclude`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn exclude_mut(&mut self) -> &mut super::EventPredicate {
            if self.exclude_opt_mut().is_none() {
                self.polarity = Some(
                    super::event_literal::Polarity::Exclude(
                        super::EventPredicate::default(),
                    ),
                );
            }
            self.exclude_opt_mut().unwrap()
        }
        ///Sets `exclude` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_exclude<T: Into<super::EventPredicate>>(&mut self, field: T) {
            self.polarity = Some(
                super::event_literal::Polarity::Exclude(field.into().into()),
            );
        }
        ///Sets `exclude` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_exclude<T: Into<super::EventPredicate>>(mut self, field: T) -> Self {
            self.set_exclude(field.into());
            self
        }
    }
    impl super::EventPredicate {
        pub const fn const_default() -> Self {
            Self { predicate: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EventPredicate = super::EventPredicate::const_default();
            &DEFAULT
        }
        ///Returns the value of `sender`, or the default value if `sender` is unset.
        pub fn sender(&self) -> &super::SenderFilter {
            if let Some(super::event_predicate::Predicate::Sender(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::SenderFilter::default_instance() as _
            }
        }
        ///If `sender` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn sender_opt(&self) -> Option<&super::SenderFilter> {
            if let Some(super::event_predicate::Predicate::Sender(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `sender` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn sender_opt_mut(&mut self) -> Option<&mut super::SenderFilter> {
            if let Some(super::event_predicate::Predicate::Sender(field)) = &mut self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `sender`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn sender_mut(&mut self) -> &mut super::SenderFilter {
            if self.sender_opt_mut().is_none() {
                self.predicate = Some(
                    super::event_predicate::Predicate::Sender(
                        super::SenderFilter::default(),
                    ),
                );
            }
            self.sender_opt_mut().unwrap()
        }
        ///Sets `sender` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_sender<T: Into<super::SenderFilter>>(&mut self, field: T) {
            self.predicate = Some(
                super::event_predicate::Predicate::Sender(field.into().into()),
            );
        }
        ///Sets `sender` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_sender<T: Into<super::SenderFilter>>(mut self, field: T) -> Self {
            self.set_sender(field.into());
            self
        }
        ///Returns the value of `affected_object`, or the default value if `affected_object` is unset.
        pub fn affected_object(&self) -> &super::AffectedObjectFilter {
            if let Some(super::event_predicate::Predicate::AffectedObject(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::AffectedObjectFilter::default_instance() as _
            }
        }
        ///If `affected_object` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn affected_object_opt(&self) -> Option<&super::AffectedObjectFilter> {
            if let Some(super::event_predicate::Predicate::AffectedObject(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `affected_object` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn affected_object_opt_mut(
            &mut self,
        ) -> Option<&mut super::AffectedObjectFilter> {
            if let Some(super::event_predicate::Predicate::AffectedObject(field)) = &mut self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `affected_object`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn affected_object_mut(&mut self) -> &mut super::AffectedObjectFilter {
            if self.affected_object_opt_mut().is_none() {
                self.predicate = Some(
                    super::event_predicate::Predicate::AffectedObject(
                        super::AffectedObjectFilter::default(),
                    ),
                );
            }
            self.affected_object_opt_mut().unwrap()
        }
        ///Sets `affected_object` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_affected_object<T: Into<super::AffectedObjectFilter>>(
            &mut self,
            field: T,
        ) {
            self.predicate = Some(
                super::event_predicate::Predicate::AffectedObject(field.into().into()),
            );
        }
        ///Sets `affected_object` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_affected_object<T: Into<super::AffectedObjectFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_affected_object(field.into());
            self
        }
        ///Returns the value of `emit_module`, or the default value if `emit_module` is unset.
        pub fn emit_module(&self) -> &super::EmitModuleFilter {
            if let Some(super::event_predicate::Predicate::EmitModule(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::EmitModuleFilter::default_instance() as _
            }
        }
        ///If `emit_module` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn emit_module_opt(&self) -> Option<&super::EmitModuleFilter> {
            if let Some(super::event_predicate::Predicate::EmitModule(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `emit_module` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn emit_module_opt_mut(&mut self) -> Option<&mut super::EmitModuleFilter> {
            if let Some(super::event_predicate::Predicate::EmitModule(field)) = &mut self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `emit_module`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn emit_module_mut(&mut self) -> &mut super::EmitModuleFilter {
            if self.emit_module_opt_mut().is_none() {
                self.predicate = Some(
                    super::event_predicate::Predicate::EmitModule(
                        super::EmitModuleFilter::default(),
                    ),
                );
            }
            self.emit_module_opt_mut().unwrap()
        }
        ///Sets `emit_module` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_emit_module<T: Into<super::EmitModuleFilter>>(&mut self, field: T) {
            self.predicate = Some(
                super::event_predicate::Predicate::EmitModule(field.into().into()),
            );
        }
        ///Sets `emit_module` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_emit_module<T: Into<super::EmitModuleFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_emit_module(field.into());
            self
        }
        ///Returns the value of `event_type`, or the default value if `event_type` is unset.
        pub fn event_type(&self) -> &super::EventTypeFilter {
            if let Some(super::event_predicate::Predicate::EventType(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::EventTypeFilter::default_instance() as _
            }
        }
        ///If `event_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn event_type_opt(&self) -> Option<&super::EventTypeFilter> {
            if let Some(super::event_predicate::Predicate::EventType(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `event_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn event_type_opt_mut(&mut self) -> Option<&mut super::EventTypeFilter> {
            if let Some(super::event_predicate::Predicate::EventType(field)) = &mut self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `event_type`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn event_type_mut(&mut self) -> &mut super::EventTypeFilter {
            if self.event_type_opt_mut().is_none() {
                self.predicate = Some(
                    super::event_predicate::Predicate::EventType(
                        super::EventTypeFilter::default(),
                    ),
                );
            }
            self.event_type_opt_mut().unwrap()
        }
        ///Sets `event_type` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_event_type<T: Into<super::EventTypeFilter>>(&mut self, field: T) {
            self.predicate = Some(
                super::event_predicate::Predicate::EventType(field.into().into()),
            );
        }
        ///Sets `event_type` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_event_type<T: Into<super::EventTypeFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_event_type(field.into());
            self
        }
        ///Returns the value of `event_stream_head`, or the default value if `event_stream_head` is unset.
        pub fn event_stream_head(&self) -> &super::EventStreamHeadFilter {
            if let Some(super::event_predicate::Predicate::EventStreamHead(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::EventStreamHeadFilter::default_instance() as _
            }
        }
        ///If `event_stream_head` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn event_stream_head_opt(&self) -> Option<&super::EventStreamHeadFilter> {
            if let Some(super::event_predicate::Predicate::EventStreamHead(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `event_stream_head` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn event_stream_head_opt_mut(
            &mut self,
        ) -> Option<&mut super::EventStreamHeadFilter> {
            if let Some(super::event_predicate::Predicate::EventStreamHead(field)) = &mut self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `event_stream_head`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn event_stream_head_mut(&mut self) -> &mut super::EventStreamHeadFilter {
            if self.event_stream_head_opt_mut().is_none() {
                self.predicate = Some(
                    super::event_predicate::Predicate::EventStreamHead(
                        super::EventStreamHeadFilter::default(),
                    ),
                );
            }
            self.event_stream_head_opt_mut().unwrap()
        }
        ///Sets `event_stream_head` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_event_stream_head<T: Into<super::EventStreamHeadFilter>>(
            &mut self,
            field: T,
        ) {
            self.predicate = Some(
                super::event_predicate::Predicate::EventStreamHead(field.into().into()),
            );
        }
        ///Sets `event_stream_head` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_event_stream_head<T: Into<super::EventStreamHeadFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_event_stream_head(field.into());
            self
        }
    }
    impl super::EventStreamHeadFilter {
        pub const fn const_default() -> Self {
            Self { stream_id: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EventStreamHeadFilter = super::EventStreamHeadFilter::const_default();
            &DEFAULT
        }
        ///If `stream_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn stream_id_opt_mut(&mut self) -> Option<&mut String> {
            self.stream_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `stream_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn stream_id_mut(&mut self) -> &mut String {
            self.stream_id.get_or_insert_default()
        }
        ///If `stream_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn stream_id_opt(&self) -> Option<&str> {
            self.stream_id.as_ref().map(|field| field as _)
        }
        ///Sets `stream_id` with the provided value.
        pub fn set_stream_id<T: Into<String>>(&mut self, field: T) {
            self.stream_id = Some(field.into().into());
        }
        ///Sets `stream_id` with the provided value.
        pub fn with_stream_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_stream_id(field.into());
            self
        }
    }
    impl super::EventTerm {
        pub const fn const_default() -> Self {
            Self { literals: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EventTerm = super::EventTerm::const_default();
            &DEFAULT
        }
        ///Returns the value of `literals`, or the default value if `literals` is unset.
        pub fn literals(&self) -> &[super::EventLiteral] {
            &self.literals
        }
        ///Returns a mutable reference to `literals`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn literals_mut(&mut self) -> &mut Vec<super::EventLiteral> {
            &mut self.literals
        }
        ///Sets `literals` with the provided value.
        pub fn set_literals(&mut self, field: Vec<super::EventLiteral>) {
            self.literals = field;
        }
        ///Sets `literals` with the provided value.
        pub fn with_literals(mut self, field: Vec<super::EventLiteral>) -> Self {
            self.set_literals(field);
            self
        }
    }
    impl super::EventTypeFilter {
        pub const fn const_default() -> Self {
            Self { r#type: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EventTypeFilter = super::EventTypeFilter::const_default();
            &DEFAULT
        }
        ///If `r#type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn type_opt_mut(&mut self) -> Option<&mut String> {
            self.r#type.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `r#type`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn type_mut(&mut self) -> &mut String {
            self.r#type.get_or_insert_default()
        }
        ///If `r#type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn type_opt(&self) -> Option<&str> {
            self.r#type.as_ref().map(|field| field as _)
        }
        ///Sets `r#type` with the provided value.
        pub fn set_type<T: Into<String>>(&mut self, field: T) {
            self.r#type = Some(field.into().into());
        }
        ///Sets `r#type` with the provided value.
        pub fn with_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_type(field.into());
            self
        }
    }
    impl super::ListCheckpointsRequest {
        pub const fn const_default() -> Self {
            Self {
                read_mask: None,
                start_checkpoint: None,
                end_checkpoint: None,
                filter: None,
                pagination: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListCheckpointsRequest = super::ListCheckpointsRequest::const_default();
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
        ///If `start_checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn start_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.start_checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `start_checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn start_checkpoint_mut(&mut self) -> &mut u64 {
            self.start_checkpoint.get_or_insert_default()
        }
        ///If `start_checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn start_checkpoint_opt(&self) -> Option<u64> {
            self.start_checkpoint.as_ref().map(|field| *field)
        }
        ///Sets `start_checkpoint` with the provided value.
        pub fn set_start_checkpoint(&mut self, field: u64) {
            self.start_checkpoint = Some(field);
        }
        ///Sets `start_checkpoint` with the provided value.
        pub fn with_start_checkpoint(mut self, field: u64) -> Self {
            self.set_start_checkpoint(field);
            self
        }
        ///If `end_checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn end_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.end_checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `end_checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn end_checkpoint_mut(&mut self) -> &mut u64 {
            self.end_checkpoint.get_or_insert_default()
        }
        ///If `end_checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn end_checkpoint_opt(&self) -> Option<u64> {
            self.end_checkpoint.as_ref().map(|field| *field)
        }
        ///Sets `end_checkpoint` with the provided value.
        pub fn set_end_checkpoint(&mut self, field: u64) {
            self.end_checkpoint = Some(field);
        }
        ///Sets `end_checkpoint` with the provided value.
        pub fn with_end_checkpoint(mut self, field: u64) -> Self {
            self.set_end_checkpoint(field);
            self
        }
        ///Returns the value of `filter`, or the default value if `filter` is unset.
        pub fn filter(&self) -> &super::TransactionFilter {
            self.filter
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::TransactionFilter::default_instance() as _)
        }
        ///If `filter` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn filter_opt_mut(&mut self) -> Option<&mut super::TransactionFilter> {
            self.filter.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `filter`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn filter_mut(&mut self) -> &mut super::TransactionFilter {
            self.filter.get_or_insert_default()
        }
        ///If `filter` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn filter_opt(&self) -> Option<&super::TransactionFilter> {
            self.filter.as_ref().map(|field| field as _)
        }
        ///Sets `filter` with the provided value.
        pub fn set_filter<T: Into<super::TransactionFilter>>(&mut self, field: T) {
            self.filter = Some(field.into().into());
        }
        ///Sets `filter` with the provided value.
        pub fn with_filter<T: Into<super::TransactionFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_filter(field.into());
            self
        }
        ///Returns the value of `pagination`, or the default value if `pagination` is unset.
        pub fn pagination(&self) -> &super::Pagination {
            self.pagination
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Pagination::default_instance() as _)
        }
        ///If `pagination` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn pagination_opt_mut(&mut self) -> Option<&mut super::Pagination> {
            self.pagination.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `pagination`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn pagination_mut(&mut self) -> &mut super::Pagination {
            self.pagination.get_or_insert_default()
        }
        ///If `pagination` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn pagination_opt(&self) -> Option<&super::Pagination> {
            self.pagination.as_ref().map(|field| field as _)
        }
        ///Sets `pagination` with the provided value.
        pub fn set_pagination<T: Into<super::Pagination>>(&mut self, field: T) {
            self.pagination = Some(field.into().into());
        }
        ///Sets `pagination` with the provided value.
        pub fn with_pagination<T: Into<super::Pagination>>(mut self, field: T) -> Self {
            self.set_pagination(field.into());
            self
        }
    }
    impl super::ListCheckpointsResponse {
        pub const fn const_default() -> Self {
            Self { response: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListCheckpointsResponse = super::ListCheckpointsResponse::const_default();
            &DEFAULT
        }
        ///Returns the value of `item`, or the default value if `item` is unset.
        pub fn item(&self) -> &super::CheckpointItem {
            if let Some(super::list_checkpoints_response::Response::Item(field)) = &self
                .response
            {
                field as _
            } else {
                super::CheckpointItem::default_instance() as _
            }
        }
        ///If `item` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn item_opt(&self) -> Option<&super::CheckpointItem> {
            if let Some(super::list_checkpoints_response::Response::Item(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `item` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn item_opt_mut(&mut self) -> Option<&mut super::CheckpointItem> {
            if let Some(super::list_checkpoints_response::Response::Item(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `item`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn item_mut(&mut self) -> &mut super::CheckpointItem {
            if self.item_opt_mut().is_none() {
                self.response = Some(
                    super::list_checkpoints_response::Response::Item(
                        super::CheckpointItem::default(),
                    ),
                );
            }
            self.item_opt_mut().unwrap()
        }
        ///Sets `item` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_item<T: Into<super::CheckpointItem>>(&mut self, field: T) {
            self.response = Some(
                super::list_checkpoints_response::Response::Item(field.into().into()),
            );
        }
        ///Sets `item` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_item<T: Into<super::CheckpointItem>>(mut self, field: T) -> Self {
            self.set_item(field.into());
            self
        }
        ///Returns the value of `page_info`, or the default value if `page_info` is unset.
        pub fn page_info(&self) -> &super::PageInfo {
            if let Some(super::list_checkpoints_response::Response::PageInfo(field)) = &self
                .response
            {
                field as _
            } else {
                super::PageInfo::default_instance() as _
            }
        }
        ///If `page_info` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn page_info_opt(&self) -> Option<&super::PageInfo> {
            if let Some(super::list_checkpoints_response::Response::PageInfo(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `page_info` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn page_info_opt_mut(&mut self) -> Option<&mut super::PageInfo> {
            if let Some(super::list_checkpoints_response::Response::PageInfo(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `page_info`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn page_info_mut(&mut self) -> &mut super::PageInfo {
            if self.page_info_opt_mut().is_none() {
                self.response = Some(
                    super::list_checkpoints_response::Response::PageInfo(
                        super::PageInfo::default(),
                    ),
                );
            }
            self.page_info_opt_mut().unwrap()
        }
        ///Sets `page_info` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_page_info<T: Into<super::PageInfo>>(&mut self, field: T) {
            self.response = Some(
                super::list_checkpoints_response::Response::PageInfo(field.into().into()),
            );
        }
        ///Sets `page_info` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_page_info<T: Into<super::PageInfo>>(mut self, field: T) -> Self {
            self.set_page_info(field.into());
            self
        }
    }
    impl super::ListEventsRequest {
        pub const fn const_default() -> Self {
            Self {
                read_mask: None,
                start_checkpoint: None,
                end_checkpoint: None,
                filter: None,
                pagination: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListEventsRequest = super::ListEventsRequest::const_default();
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
        ///If `start_checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn start_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.start_checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `start_checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn start_checkpoint_mut(&mut self) -> &mut u64 {
            self.start_checkpoint.get_or_insert_default()
        }
        ///If `start_checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn start_checkpoint_opt(&self) -> Option<u64> {
            self.start_checkpoint.as_ref().map(|field| *field)
        }
        ///Sets `start_checkpoint` with the provided value.
        pub fn set_start_checkpoint(&mut self, field: u64) {
            self.start_checkpoint = Some(field);
        }
        ///Sets `start_checkpoint` with the provided value.
        pub fn with_start_checkpoint(mut self, field: u64) -> Self {
            self.set_start_checkpoint(field);
            self
        }
        ///If `end_checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn end_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.end_checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `end_checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn end_checkpoint_mut(&mut self) -> &mut u64 {
            self.end_checkpoint.get_or_insert_default()
        }
        ///If `end_checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn end_checkpoint_opt(&self) -> Option<u64> {
            self.end_checkpoint.as_ref().map(|field| *field)
        }
        ///Sets `end_checkpoint` with the provided value.
        pub fn set_end_checkpoint(&mut self, field: u64) {
            self.end_checkpoint = Some(field);
        }
        ///Sets `end_checkpoint` with the provided value.
        pub fn with_end_checkpoint(mut self, field: u64) -> Self {
            self.set_end_checkpoint(field);
            self
        }
        ///Returns the value of `filter`, or the default value if `filter` is unset.
        pub fn filter(&self) -> &super::EventFilter {
            self.filter
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::EventFilter::default_instance() as _)
        }
        ///If `filter` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn filter_opt_mut(&mut self) -> Option<&mut super::EventFilter> {
            self.filter.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `filter`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn filter_mut(&mut self) -> &mut super::EventFilter {
            self.filter.get_or_insert_default()
        }
        ///If `filter` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn filter_opt(&self) -> Option<&super::EventFilter> {
            self.filter.as_ref().map(|field| field as _)
        }
        ///Sets `filter` with the provided value.
        pub fn set_filter<T: Into<super::EventFilter>>(&mut self, field: T) {
            self.filter = Some(field.into().into());
        }
        ///Sets `filter` with the provided value.
        pub fn with_filter<T: Into<super::EventFilter>>(mut self, field: T) -> Self {
            self.set_filter(field.into());
            self
        }
        ///Returns the value of `pagination`, or the default value if `pagination` is unset.
        pub fn pagination(&self) -> &super::Pagination {
            self.pagination
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Pagination::default_instance() as _)
        }
        ///If `pagination` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn pagination_opt_mut(&mut self) -> Option<&mut super::Pagination> {
            self.pagination.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `pagination`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn pagination_mut(&mut self) -> &mut super::Pagination {
            self.pagination.get_or_insert_default()
        }
        ///If `pagination` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn pagination_opt(&self) -> Option<&super::Pagination> {
            self.pagination.as_ref().map(|field| field as _)
        }
        ///Sets `pagination` with the provided value.
        pub fn set_pagination<T: Into<super::Pagination>>(&mut self, field: T) {
            self.pagination = Some(field.into().into());
        }
        ///Sets `pagination` with the provided value.
        pub fn with_pagination<T: Into<super::Pagination>>(mut self, field: T) -> Self {
            self.set_pagination(field.into());
            self
        }
    }
    impl super::ListEventsResponse {
        pub const fn const_default() -> Self {
            Self { response: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListEventsResponse = super::ListEventsResponse::const_default();
            &DEFAULT
        }
        ///Returns the value of `item`, or the default value if `item` is unset.
        pub fn item(&self) -> &super::EventItem {
            if let Some(super::list_events_response::Response::Item(field)) = &self
                .response
            {
                field as _
            } else {
                super::EventItem::default_instance() as _
            }
        }
        ///If `item` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn item_opt(&self) -> Option<&super::EventItem> {
            if let Some(super::list_events_response::Response::Item(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `item` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn item_opt_mut(&mut self) -> Option<&mut super::EventItem> {
            if let Some(super::list_events_response::Response::Item(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `item`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn item_mut(&mut self) -> &mut super::EventItem {
            if self.item_opt_mut().is_none() {
                self.response = Some(
                    super::list_events_response::Response::Item(
                        super::EventItem::default(),
                    ),
                );
            }
            self.item_opt_mut().unwrap()
        }
        ///Sets `item` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_item<T: Into<super::EventItem>>(&mut self, field: T) {
            self.response = Some(
                super::list_events_response::Response::Item(field.into().into()),
            );
        }
        ///Sets `item` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_item<T: Into<super::EventItem>>(mut self, field: T) -> Self {
            self.set_item(field.into());
            self
        }
        ///Returns the value of `page_info`, or the default value if `page_info` is unset.
        pub fn page_info(&self) -> &super::PageInfo {
            if let Some(super::list_events_response::Response::PageInfo(field)) = &self
                .response
            {
                field as _
            } else {
                super::PageInfo::default_instance() as _
            }
        }
        ///If `page_info` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn page_info_opt(&self) -> Option<&super::PageInfo> {
            if let Some(super::list_events_response::Response::PageInfo(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `page_info` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn page_info_opt_mut(&mut self) -> Option<&mut super::PageInfo> {
            if let Some(super::list_events_response::Response::PageInfo(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `page_info`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn page_info_mut(&mut self) -> &mut super::PageInfo {
            if self.page_info_opt_mut().is_none() {
                self.response = Some(
                    super::list_events_response::Response::PageInfo(
                        super::PageInfo::default(),
                    ),
                );
            }
            self.page_info_opt_mut().unwrap()
        }
        ///Sets `page_info` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_page_info<T: Into<super::PageInfo>>(&mut self, field: T) {
            self.response = Some(
                super::list_events_response::Response::PageInfo(field.into().into()),
            );
        }
        ///Sets `page_info` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_page_info<T: Into<super::PageInfo>>(mut self, field: T) -> Self {
            self.set_page_info(field.into());
            self
        }
    }
    impl super::ListTransactionsRequest {
        pub const fn const_default() -> Self {
            Self {
                read_mask: None,
                start_checkpoint: None,
                end_checkpoint: None,
                filter: None,
                pagination: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListTransactionsRequest = super::ListTransactionsRequest::const_default();
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
        ///If `start_checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn start_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.start_checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `start_checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn start_checkpoint_mut(&mut self) -> &mut u64 {
            self.start_checkpoint.get_or_insert_default()
        }
        ///If `start_checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn start_checkpoint_opt(&self) -> Option<u64> {
            self.start_checkpoint.as_ref().map(|field| *field)
        }
        ///Sets `start_checkpoint` with the provided value.
        pub fn set_start_checkpoint(&mut self, field: u64) {
            self.start_checkpoint = Some(field);
        }
        ///Sets `start_checkpoint` with the provided value.
        pub fn with_start_checkpoint(mut self, field: u64) -> Self {
            self.set_start_checkpoint(field);
            self
        }
        ///If `end_checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn end_checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.end_checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `end_checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn end_checkpoint_mut(&mut self) -> &mut u64 {
            self.end_checkpoint.get_or_insert_default()
        }
        ///If `end_checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn end_checkpoint_opt(&self) -> Option<u64> {
            self.end_checkpoint.as_ref().map(|field| *field)
        }
        ///Sets `end_checkpoint` with the provided value.
        pub fn set_end_checkpoint(&mut self, field: u64) {
            self.end_checkpoint = Some(field);
        }
        ///Sets `end_checkpoint` with the provided value.
        pub fn with_end_checkpoint(mut self, field: u64) -> Self {
            self.set_end_checkpoint(field);
            self
        }
        ///Returns the value of `filter`, or the default value if `filter` is unset.
        pub fn filter(&self) -> &super::TransactionFilter {
            self.filter
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::TransactionFilter::default_instance() as _)
        }
        ///If `filter` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn filter_opt_mut(&mut self) -> Option<&mut super::TransactionFilter> {
            self.filter.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `filter`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn filter_mut(&mut self) -> &mut super::TransactionFilter {
            self.filter.get_or_insert_default()
        }
        ///If `filter` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn filter_opt(&self) -> Option<&super::TransactionFilter> {
            self.filter.as_ref().map(|field| field as _)
        }
        ///Sets `filter` with the provided value.
        pub fn set_filter<T: Into<super::TransactionFilter>>(&mut self, field: T) {
            self.filter = Some(field.into().into());
        }
        ///Sets `filter` with the provided value.
        pub fn with_filter<T: Into<super::TransactionFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_filter(field.into());
            self
        }
        ///Returns the value of `pagination`, or the default value if `pagination` is unset.
        pub fn pagination(&self) -> &super::Pagination {
            self.pagination
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Pagination::default_instance() as _)
        }
        ///If `pagination` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn pagination_opt_mut(&mut self) -> Option<&mut super::Pagination> {
            self.pagination.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `pagination`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn pagination_mut(&mut self) -> &mut super::Pagination {
            self.pagination.get_or_insert_default()
        }
        ///If `pagination` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn pagination_opt(&self) -> Option<&super::Pagination> {
            self.pagination.as_ref().map(|field| field as _)
        }
        ///Sets `pagination` with the provided value.
        pub fn set_pagination<T: Into<super::Pagination>>(&mut self, field: T) {
            self.pagination = Some(field.into().into());
        }
        ///Sets `pagination` with the provided value.
        pub fn with_pagination<T: Into<super::Pagination>>(mut self, field: T) -> Self {
            self.set_pagination(field.into());
            self
        }
    }
    impl super::ListTransactionsResponse {
        pub const fn const_default() -> Self {
            Self { response: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ListTransactionsResponse = super::ListTransactionsResponse::const_default();
            &DEFAULT
        }
        ///Returns the value of `item`, or the default value if `item` is unset.
        pub fn item(&self) -> &super::TransactionItem {
            if let Some(super::list_transactions_response::Response::Item(field)) = &self
                .response
            {
                field as _
            } else {
                super::TransactionItem::default_instance() as _
            }
        }
        ///If `item` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn item_opt(&self) -> Option<&super::TransactionItem> {
            if let Some(super::list_transactions_response::Response::Item(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `item` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn item_opt_mut(&mut self) -> Option<&mut super::TransactionItem> {
            if let Some(super::list_transactions_response::Response::Item(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `item`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn item_mut(&mut self) -> &mut super::TransactionItem {
            if self.item_opt_mut().is_none() {
                self.response = Some(
                    super::list_transactions_response::Response::Item(
                        super::TransactionItem::default(),
                    ),
                );
            }
            self.item_opt_mut().unwrap()
        }
        ///Sets `item` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_item<T: Into<super::TransactionItem>>(&mut self, field: T) {
            self.response = Some(
                super::list_transactions_response::Response::Item(field.into().into()),
            );
        }
        ///Sets `item` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_item<T: Into<super::TransactionItem>>(mut self, field: T) -> Self {
            self.set_item(field.into());
            self
        }
        ///Returns the value of `page_info`, or the default value if `page_info` is unset.
        pub fn page_info(&self) -> &super::PageInfo {
            if let Some(super::list_transactions_response::Response::PageInfo(field)) = &self
                .response
            {
                field as _
            } else {
                super::PageInfo::default_instance() as _
            }
        }
        ///If `page_info` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn page_info_opt(&self) -> Option<&super::PageInfo> {
            if let Some(super::list_transactions_response::Response::PageInfo(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `page_info` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn page_info_opt_mut(&mut self) -> Option<&mut super::PageInfo> {
            if let Some(super::list_transactions_response::Response::PageInfo(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `page_info`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn page_info_mut(&mut self) -> &mut super::PageInfo {
            if self.page_info_opt_mut().is_none() {
                self.response = Some(
                    super::list_transactions_response::Response::PageInfo(
                        super::PageInfo::default(),
                    ),
                );
            }
            self.page_info_opt_mut().unwrap()
        }
        ///Sets `page_info` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_page_info<T: Into<super::PageInfo>>(&mut self, field: T) {
            self.response = Some(
                super::list_transactions_response::Response::PageInfo(
                    field.into().into(),
                ),
            );
        }
        ///Sets `page_info` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_page_info<T: Into<super::PageInfo>>(mut self, field: T) -> Self {
            self.set_page_info(field.into());
            self
        }
    }
    impl super::MoveCallFilter {
        pub const fn const_default() -> Self {
            Self { function: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MoveCallFilter = super::MoveCallFilter::const_default();
            &DEFAULT
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
    }
    impl super::PageInfo {
        pub const fn const_default() -> Self {
            Self { next_cursor: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::PageInfo = super::PageInfo::const_default();
            &DEFAULT
        }
        ///If `next_cursor` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn next_cursor_opt(&self) -> Option<&[u8]> {
            self.next_cursor.as_ref().map(|field| field as _)
        }
        ///Sets `next_cursor` with the provided value.
        pub fn set_next_cursor<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.next_cursor = Some(field.into().into());
        }
        ///Sets `next_cursor` with the provided value.
        pub fn with_next_cursor<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_next_cursor(field.into());
            self
        }
    }
    impl super::Pagination {
        pub const fn const_default() -> Self {
            Self {
                page_size: None,
                cursor: None,
                ordering: 0,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Pagination = super::Pagination::const_default();
            &DEFAULT
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
        ///If `cursor` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn cursor_opt(&self) -> Option<&[u8]> {
            self.cursor.as_ref().map(|field| field as _)
        }
        ///Sets `cursor` with the provided value.
        pub fn set_cursor<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.cursor = Some(field.into().into());
        }
        ///Sets `cursor` with the provided value.
        pub fn with_cursor<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_cursor(field.into());
            self
        }
    }
    impl super::SenderFilter {
        pub const fn const_default() -> Self {
            Self { address: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SenderFilter = super::SenderFilter::const_default();
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
    impl super::TransactionFilter {
        pub const fn const_default() -> Self {
            Self { terms: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionFilter = super::TransactionFilter::const_default();
            &DEFAULT
        }
        ///Returns the value of `terms`, or the default value if `terms` is unset.
        pub fn terms(&self) -> &[super::TransactionTerm] {
            &self.terms
        }
        ///Returns a mutable reference to `terms`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn terms_mut(&mut self) -> &mut Vec<super::TransactionTerm> {
            &mut self.terms
        }
        ///Sets `terms` with the provided value.
        pub fn set_terms(&mut self, field: Vec<super::TransactionTerm>) {
            self.terms = field;
        }
        ///Sets `terms` with the provided value.
        pub fn with_terms(mut self, field: Vec<super::TransactionTerm>) -> Self {
            self.set_terms(field);
            self
        }
    }
    impl super::TransactionItem {
        pub const fn const_default() -> Self {
            Self {
                cursor: None,
                transaction: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionItem = super::TransactionItem::const_default();
            &DEFAULT
        }
        ///If `cursor` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn cursor_opt(&self) -> Option<&[u8]> {
            self.cursor.as_ref().map(|field| field as _)
        }
        ///Sets `cursor` with the provided value.
        pub fn set_cursor<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.cursor = Some(field.into().into());
        }
        ///Sets `cursor` with the provided value.
        pub fn with_cursor<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_cursor(field.into());
            self
        }
        ///Returns the value of `transaction`, or the default value if `transaction` is unset.
        pub fn transaction(&self) -> &super::super::v2::ExecutedTransaction {
            self.transaction
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::super::v2::ExecutedTransaction::default_instance() as _
                })
        }
        ///If `transaction` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn transaction_opt_mut(
            &mut self,
        ) -> Option<&mut super::super::v2::ExecutedTransaction> {
            self.transaction.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `transaction`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn transaction_mut(&mut self) -> &mut super::super::v2::ExecutedTransaction {
            self.transaction.get_or_insert_default()
        }
        ///If `transaction` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn transaction_opt(&self) -> Option<&super::super::v2::ExecutedTransaction> {
            self.transaction.as_ref().map(|field| field as _)
        }
        ///Sets `transaction` with the provided value.
        pub fn set_transaction<T: Into<super::super::v2::ExecutedTransaction>>(
            &mut self,
            field: T,
        ) {
            self.transaction = Some(field.into().into());
        }
        ///Sets `transaction` with the provided value.
        pub fn with_transaction<T: Into<super::super::v2::ExecutedTransaction>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_transaction(field.into());
            self
        }
    }
    impl super::TransactionLiteral {
        pub const fn const_default() -> Self {
            Self { polarity: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionLiteral = super::TransactionLiteral::const_default();
            &DEFAULT
        }
        ///Returns the value of `include`, or the default value if `include` is unset.
        pub fn include(&self) -> &super::TransactionPredicate {
            if let Some(super::transaction_literal::Polarity::Include(field)) = &self
                .polarity
            {
                field as _
            } else {
                super::TransactionPredicate::default_instance() as _
            }
        }
        ///If `include` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn include_opt(&self) -> Option<&super::TransactionPredicate> {
            if let Some(super::transaction_literal::Polarity::Include(field)) = &self
                .polarity
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `include` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn include_opt_mut(&mut self) -> Option<&mut super::TransactionPredicate> {
            if let Some(super::transaction_literal::Polarity::Include(field)) = &mut self
                .polarity
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `include`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn include_mut(&mut self) -> &mut super::TransactionPredicate {
            if self.include_opt_mut().is_none() {
                self.polarity = Some(
                    super::transaction_literal::Polarity::Include(
                        super::TransactionPredicate::default(),
                    ),
                );
            }
            self.include_opt_mut().unwrap()
        }
        ///Sets `include` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_include<T: Into<super::TransactionPredicate>>(&mut self, field: T) {
            self.polarity = Some(
                super::transaction_literal::Polarity::Include(field.into().into()),
            );
        }
        ///Sets `include` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_include<T: Into<super::TransactionPredicate>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_include(field.into());
            self
        }
        ///Returns the value of `exclude`, or the default value if `exclude` is unset.
        pub fn exclude(&self) -> &super::TransactionPredicate {
            if let Some(super::transaction_literal::Polarity::Exclude(field)) = &self
                .polarity
            {
                field as _
            } else {
                super::TransactionPredicate::default_instance() as _
            }
        }
        ///If `exclude` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn exclude_opt(&self) -> Option<&super::TransactionPredicate> {
            if let Some(super::transaction_literal::Polarity::Exclude(field)) = &self
                .polarity
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `exclude` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn exclude_opt_mut(&mut self) -> Option<&mut super::TransactionPredicate> {
            if let Some(super::transaction_literal::Polarity::Exclude(field)) = &mut self
                .polarity
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `exclude`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn exclude_mut(&mut self) -> &mut super::TransactionPredicate {
            if self.exclude_opt_mut().is_none() {
                self.polarity = Some(
                    super::transaction_literal::Polarity::Exclude(
                        super::TransactionPredicate::default(),
                    ),
                );
            }
            self.exclude_opt_mut().unwrap()
        }
        ///Sets `exclude` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_exclude<T: Into<super::TransactionPredicate>>(&mut self, field: T) {
            self.polarity = Some(
                super::transaction_literal::Polarity::Exclude(field.into().into()),
            );
        }
        ///Sets `exclude` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_exclude<T: Into<super::TransactionPredicate>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_exclude(field.into());
            self
        }
    }
    impl super::TransactionPredicate {
        pub const fn const_default() -> Self {
            Self { predicate: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionPredicate = super::TransactionPredicate::const_default();
            &DEFAULT
        }
        ///Returns the value of `sender`, or the default value if `sender` is unset.
        pub fn sender(&self) -> &super::SenderFilter {
            if let Some(super::transaction_predicate::Predicate::Sender(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::SenderFilter::default_instance() as _
            }
        }
        ///If `sender` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn sender_opt(&self) -> Option<&super::SenderFilter> {
            if let Some(super::transaction_predicate::Predicate::Sender(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `sender` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn sender_opt_mut(&mut self) -> Option<&mut super::SenderFilter> {
            if let Some(super::transaction_predicate::Predicate::Sender(field)) = &mut self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `sender`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn sender_mut(&mut self) -> &mut super::SenderFilter {
            if self.sender_opt_mut().is_none() {
                self.predicate = Some(
                    super::transaction_predicate::Predicate::Sender(
                        super::SenderFilter::default(),
                    ),
                );
            }
            self.sender_opt_mut().unwrap()
        }
        ///Sets `sender` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_sender<T: Into<super::SenderFilter>>(&mut self, field: T) {
            self.predicate = Some(
                super::transaction_predicate::Predicate::Sender(field.into().into()),
            );
        }
        ///Sets `sender` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_sender<T: Into<super::SenderFilter>>(mut self, field: T) -> Self {
            self.set_sender(field.into());
            self
        }
        ///Returns the value of `affected_address`, or the default value if `affected_address` is unset.
        pub fn affected_address(&self) -> &super::AffectedAddressFilter {
            if let Some(
                super::transaction_predicate::Predicate::AffectedAddress(field),
            ) = &self.predicate
            {
                field as _
            } else {
                super::AffectedAddressFilter::default_instance() as _
            }
        }
        ///If `affected_address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn affected_address_opt(&self) -> Option<&super::AffectedAddressFilter> {
            if let Some(
                super::transaction_predicate::Predicate::AffectedAddress(field),
            ) = &self.predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `affected_address` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn affected_address_opt_mut(
            &mut self,
        ) -> Option<&mut super::AffectedAddressFilter> {
            if let Some(
                super::transaction_predicate::Predicate::AffectedAddress(field),
            ) = &mut self.predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `affected_address`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn affected_address_mut(&mut self) -> &mut super::AffectedAddressFilter {
            if self.affected_address_opt_mut().is_none() {
                self.predicate = Some(
                    super::transaction_predicate::Predicate::AffectedAddress(
                        super::AffectedAddressFilter::default(),
                    ),
                );
            }
            self.affected_address_opt_mut().unwrap()
        }
        ///Sets `affected_address` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_affected_address<T: Into<super::AffectedAddressFilter>>(
            &mut self,
            field: T,
        ) {
            self.predicate = Some(
                super::transaction_predicate::Predicate::AffectedAddress(
                    field.into().into(),
                ),
            );
        }
        ///Sets `affected_address` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_affected_address<T: Into<super::AffectedAddressFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_affected_address(field.into());
            self
        }
        ///Returns the value of `affected_object`, or the default value if `affected_object` is unset.
        pub fn affected_object(&self) -> &super::AffectedObjectFilter {
            if let Some(
                super::transaction_predicate::Predicate::AffectedObject(field),
            ) = &self.predicate
            {
                field as _
            } else {
                super::AffectedObjectFilter::default_instance() as _
            }
        }
        ///If `affected_object` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn affected_object_opt(&self) -> Option<&super::AffectedObjectFilter> {
            if let Some(
                super::transaction_predicate::Predicate::AffectedObject(field),
            ) = &self.predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `affected_object` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn affected_object_opt_mut(
            &mut self,
        ) -> Option<&mut super::AffectedObjectFilter> {
            if let Some(
                super::transaction_predicate::Predicate::AffectedObject(field),
            ) = &mut self.predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `affected_object`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn affected_object_mut(&mut self) -> &mut super::AffectedObjectFilter {
            if self.affected_object_opt_mut().is_none() {
                self.predicate = Some(
                    super::transaction_predicate::Predicate::AffectedObject(
                        super::AffectedObjectFilter::default(),
                    ),
                );
            }
            self.affected_object_opt_mut().unwrap()
        }
        ///Sets `affected_object` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_affected_object<T: Into<super::AffectedObjectFilter>>(
            &mut self,
            field: T,
        ) {
            self.predicate = Some(
                super::transaction_predicate::Predicate::AffectedObject(
                    field.into().into(),
                ),
            );
        }
        ///Sets `affected_object` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_affected_object<T: Into<super::AffectedObjectFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_affected_object(field.into());
            self
        }
        ///Returns the value of `move_call`, or the default value if `move_call` is unset.
        pub fn move_call(&self) -> &super::MoveCallFilter {
            if let Some(super::transaction_predicate::Predicate::MoveCall(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::MoveCallFilter::default_instance() as _
            }
        }
        ///If `move_call` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn move_call_opt(&self) -> Option<&super::MoveCallFilter> {
            if let Some(super::transaction_predicate::Predicate::MoveCall(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `move_call` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn move_call_opt_mut(&mut self) -> Option<&mut super::MoveCallFilter> {
            if let Some(super::transaction_predicate::Predicate::MoveCall(field)) = &mut self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `move_call`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn move_call_mut(&mut self) -> &mut super::MoveCallFilter {
            if self.move_call_opt_mut().is_none() {
                self.predicate = Some(
                    super::transaction_predicate::Predicate::MoveCall(
                        super::MoveCallFilter::default(),
                    ),
                );
            }
            self.move_call_opt_mut().unwrap()
        }
        ///Sets `move_call` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_move_call<T: Into<super::MoveCallFilter>>(&mut self, field: T) {
            self.predicate = Some(
                super::transaction_predicate::Predicate::MoveCall(field.into().into()),
            );
        }
        ///Sets `move_call` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_move_call<T: Into<super::MoveCallFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_move_call(field.into());
            self
        }
        ///Returns the value of `emit_module`, or the default value if `emit_module` is unset.
        pub fn emit_module(&self) -> &super::EmitModuleFilter {
            if let Some(super::transaction_predicate::Predicate::EmitModule(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::EmitModuleFilter::default_instance() as _
            }
        }
        ///If `emit_module` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn emit_module_opt(&self) -> Option<&super::EmitModuleFilter> {
            if let Some(super::transaction_predicate::Predicate::EmitModule(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `emit_module` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn emit_module_opt_mut(&mut self) -> Option<&mut super::EmitModuleFilter> {
            if let Some(super::transaction_predicate::Predicate::EmitModule(field)) = &mut self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `emit_module`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn emit_module_mut(&mut self) -> &mut super::EmitModuleFilter {
            if self.emit_module_opt_mut().is_none() {
                self.predicate = Some(
                    super::transaction_predicate::Predicate::EmitModule(
                        super::EmitModuleFilter::default(),
                    ),
                );
            }
            self.emit_module_opt_mut().unwrap()
        }
        ///Sets `emit_module` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_emit_module<T: Into<super::EmitModuleFilter>>(&mut self, field: T) {
            self.predicate = Some(
                super::transaction_predicate::Predicate::EmitModule(field.into().into()),
            );
        }
        ///Sets `emit_module` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_emit_module<T: Into<super::EmitModuleFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_emit_module(field.into());
            self
        }
        ///Returns the value of `event_type`, or the default value if `event_type` is unset.
        pub fn event_type(&self) -> &super::EventTypeFilter {
            if let Some(super::transaction_predicate::Predicate::EventType(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::EventTypeFilter::default_instance() as _
            }
        }
        ///If `event_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn event_type_opt(&self) -> Option<&super::EventTypeFilter> {
            if let Some(super::transaction_predicate::Predicate::EventType(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `event_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn event_type_opt_mut(&mut self) -> Option<&mut super::EventTypeFilter> {
            if let Some(super::transaction_predicate::Predicate::EventType(field)) = &mut self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `event_type`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn event_type_mut(&mut self) -> &mut super::EventTypeFilter {
            if self.event_type_opt_mut().is_none() {
                self.predicate = Some(
                    super::transaction_predicate::Predicate::EventType(
                        super::EventTypeFilter::default(),
                    ),
                );
            }
            self.event_type_opt_mut().unwrap()
        }
        ///Sets `event_type` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_event_type<T: Into<super::EventTypeFilter>>(&mut self, field: T) {
            self.predicate = Some(
                super::transaction_predicate::Predicate::EventType(field.into().into()),
            );
        }
        ///Sets `event_type` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_event_type<T: Into<super::EventTypeFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_event_type(field.into());
            self
        }
        ///Returns the value of `event_stream_head`, or the default value if `event_stream_head` is unset.
        pub fn event_stream_head(&self) -> &super::EventStreamHeadFilter {
            if let Some(
                super::transaction_predicate::Predicate::EventStreamHead(field),
            ) = &self.predicate
            {
                field as _
            } else {
                super::EventStreamHeadFilter::default_instance() as _
            }
        }
        ///If `event_stream_head` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn event_stream_head_opt(&self) -> Option<&super::EventStreamHeadFilter> {
            if let Some(
                super::transaction_predicate::Predicate::EventStreamHead(field),
            ) = &self.predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `event_stream_head` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn event_stream_head_opt_mut(
            &mut self,
        ) -> Option<&mut super::EventStreamHeadFilter> {
            if let Some(
                super::transaction_predicate::Predicate::EventStreamHead(field),
            ) = &mut self.predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `event_stream_head`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn event_stream_head_mut(&mut self) -> &mut super::EventStreamHeadFilter {
            if self.event_stream_head_opt_mut().is_none() {
                self.predicate = Some(
                    super::transaction_predicate::Predicate::EventStreamHead(
                        super::EventStreamHeadFilter::default(),
                    ),
                );
            }
            self.event_stream_head_opt_mut().unwrap()
        }
        ///Sets `event_stream_head` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_event_stream_head<T: Into<super::EventStreamHeadFilter>>(
            &mut self,
            field: T,
        ) {
            self.predicate = Some(
                super::transaction_predicate::Predicate::EventStreamHead(
                    field.into().into(),
                ),
            );
        }
        ///Sets `event_stream_head` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_event_stream_head<T: Into<super::EventStreamHeadFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_event_stream_head(field.into());
            self
        }
    }
    impl super::TransactionTerm {
        pub const fn const_default() -> Self {
            Self { literals: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionTerm = super::TransactionTerm::const_default();
            &DEFAULT
        }
        ///Returns the value of `literals`, or the default value if `literals` is unset.
        pub fn literals(&self) -> &[super::TransactionLiteral] {
            &self.literals
        }
        ///Returns a mutable reference to `literals`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn literals_mut(&mut self) -> &mut Vec<super::TransactionLiteral> {
            &mut self.literals
        }
        ///Sets `literals` with the provided value.
        pub fn set_literals(&mut self, field: Vec<super::TransactionLiteral>) {
            self.literals = field;
        }
        ///Sets `literals` with the provided value.
        pub fn with_literals(mut self, field: Vec<super::TransactionLiteral>) -> Self {
            self.set_literals(field);
            self
        }
    }
}
