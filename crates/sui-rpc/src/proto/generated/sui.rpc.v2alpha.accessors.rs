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
                checkpoint: None,
                watermark: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::CheckpointItem = super::CheckpointItem::const_default();
            &DEFAULT
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
        ///Returns the value of `watermark`, or the default value if `watermark` is unset.
        pub fn watermark(&self) -> &super::Watermark {
            self.watermark
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Watermark::default_instance() as _)
        }
        ///If `watermark` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn watermark_opt_mut(&mut self) -> Option<&mut super::Watermark> {
            self.watermark.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `watermark`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn watermark_mut(&mut self) -> &mut super::Watermark {
            self.watermark.get_or_insert_default()
        }
        ///If `watermark` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn watermark_opt(&self) -> Option<&super::Watermark> {
            self.watermark.as_ref().map(|field| field as _)
        }
        ///Sets `watermark` with the provided value.
        pub fn set_watermark<T: Into<super::Watermark>>(&mut self, field: T) {
            self.watermark = Some(field.into().into());
        }
        ///Sets `watermark` with the provided value.
        pub fn with_watermark<T: Into<super::Watermark>>(mut self, field: T) -> Self {
            self.set_watermark(field.into());
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
                event: None,
                watermark: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EventItem = super::EventItem::const_default();
            &DEFAULT
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
        ///Returns the value of `watermark`, or the default value if `watermark` is unset.
        pub fn watermark(&self) -> &super::Watermark {
            self.watermark
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Watermark::default_instance() as _)
        }
        ///If `watermark` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn watermark_opt_mut(&mut self) -> Option<&mut super::Watermark> {
            self.watermark.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `watermark`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn watermark_mut(&mut self) -> &mut super::Watermark {
            self.watermark.get_or_insert_default()
        }
        ///If `watermark` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn watermark_opt(&self) -> Option<&super::Watermark> {
            self.watermark.as_ref().map(|field| field as _)
        }
        ///Sets `watermark` with the provided value.
        pub fn set_watermark<T: Into<super::Watermark>>(&mut self, field: T) {
            self.watermark = Some(field.into().into());
        }
        ///Sets `watermark` with the provided value.
        pub fn with_watermark<T: Into<super::Watermark>>(mut self, field: T) -> Self {
            self.set_watermark(field.into());
            self
        }
    }
    impl super::EventLiteral {
        pub const fn const_default() -> Self {
            Self {
                negated: false,
                predicate: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EventLiteral = super::EventLiteral::const_default();
            &DEFAULT
        }
        ///Returns a mutable reference to `negated`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn negated_mut(&mut self) -> &mut bool {
            &mut self.negated
        }
        ///Sets `negated` with the provided value.
        pub fn set_negated(&mut self, field: bool) {
            self.negated = field;
        }
        ///Sets `negated` with the provided value.
        pub fn with_negated(mut self, field: bool) -> Self {
            self.set_negated(field);
            self
        }
        ///Returns the value of `sender`, or the default value if `sender` is unset.
        pub fn sender(&self) -> &super::SenderFilter {
            if let Some(super::event_literal::Predicate::Sender(field)) = &self.predicate
            {
                field as _
            } else {
                super::SenderFilter::default_instance() as _
            }
        }
        ///If `sender` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn sender_opt(&self) -> Option<&super::SenderFilter> {
            if let Some(super::event_literal::Predicate::Sender(field)) = &self.predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `sender` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn sender_opt_mut(&mut self) -> Option<&mut super::SenderFilter> {
            if let Some(super::event_literal::Predicate::Sender(field)) = &mut self
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
                    super::event_literal::Predicate::Sender(
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
                super::event_literal::Predicate::Sender(field.into().into()),
            );
        }
        ///Sets `sender` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_sender<T: Into<super::SenderFilter>>(mut self, field: T) -> Self {
            self.set_sender(field.into());
            self
        }
        ///Returns the value of `emit_module`, or the default value if `emit_module` is unset.
        pub fn emit_module(&self) -> &super::EmitModuleFilter {
            if let Some(super::event_literal::Predicate::EmitModule(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::EmitModuleFilter::default_instance() as _
            }
        }
        ///If `emit_module` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn emit_module_opt(&self) -> Option<&super::EmitModuleFilter> {
            if let Some(super::event_literal::Predicate::EmitModule(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `emit_module` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn emit_module_opt_mut(&mut self) -> Option<&mut super::EmitModuleFilter> {
            if let Some(super::event_literal::Predicate::EmitModule(field)) = &mut self
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
                    super::event_literal::Predicate::EmitModule(
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
                super::event_literal::Predicate::EmitModule(field.into().into()),
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
            if let Some(super::event_literal::Predicate::EventType(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::EventTypeFilter::default_instance() as _
            }
        }
        ///If `event_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn event_type_opt(&self) -> Option<&super::EventTypeFilter> {
            if let Some(super::event_literal::Predicate::EventType(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `event_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn event_type_opt_mut(&mut self) -> Option<&mut super::EventTypeFilter> {
            if let Some(super::event_literal::Predicate::EventType(field)) = &mut self
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
                    super::event_literal::Predicate::EventType(
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
                super::event_literal::Predicate::EventType(field.into().into()),
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
            if let Some(super::event_literal::Predicate::EventStreamHead(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::EventStreamHeadFilter::default_instance() as _
            }
        }
        ///If `event_stream_head` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn event_stream_head_opt(&self) -> Option<&super::EventStreamHeadFilter> {
            if let Some(super::event_literal::Predicate::EventStreamHead(field)) = &self
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
            if let Some(super::event_literal::Predicate::EventStreamHead(field)) = &mut self
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
                    super::event_literal::Predicate::EventStreamHead(
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
                super::event_literal::Predicate::EventStreamHead(field.into().into()),
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
            Self { event_type: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::EventTypeFilter = super::EventTypeFilter::const_default();
            &DEFAULT
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
    }
    impl super::GetCheckpointObjectProofRequest {
        pub const fn const_default() -> Self {
            Self {
                object_id: None,
                checkpoint: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetCheckpointObjectProofRequest = super::GetCheckpointObjectProofRequest::const_default();
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
    }
    impl super::GetCheckpointObjectProofResponse {
        pub const fn const_default() -> Self {
            Self {
                checkpoint_summary: None,
                proof: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetCheckpointObjectProofResponse = super::GetCheckpointObjectProofResponse::const_default();
            &DEFAULT
        }
        ///If `checkpoint_summary` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn checkpoint_summary_opt(&self) -> Option<&[u8]> {
            self.checkpoint_summary.as_ref().map(|field| field as _)
        }
        ///Sets `checkpoint_summary` with the provided value.
        pub fn set_checkpoint_summary<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.checkpoint_summary = Some(field.into().into());
        }
        ///Sets `checkpoint_summary` with the provided value.
        pub fn with_checkpoint_summary<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_checkpoint_summary(field.into());
            self
        }
        ///Returns the value of `inclusion`, or the default value if `inclusion` is unset.
        pub fn inclusion(&self) -> &super::OcsInclusionProof {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::Inclusion(field),
            ) = &self.proof
            {
                field as _
            } else {
                super::OcsInclusionProof::default_instance() as _
            }
        }
        ///If `inclusion` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn inclusion_opt(&self) -> Option<&super::OcsInclusionProof> {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::Inclusion(field),
            ) = &self.proof
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `inclusion` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn inclusion_opt_mut(&mut self) -> Option<&mut super::OcsInclusionProof> {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::Inclusion(field),
            ) = &mut self.proof
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `inclusion`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn inclusion_mut(&mut self) -> &mut super::OcsInclusionProof {
            if self.inclusion_opt_mut().is_none() {
                self.proof = Some(
                    super::get_checkpoint_object_proof_response::Proof::Inclusion(
                        super::OcsInclusionProof::default(),
                    ),
                );
            }
            self.inclusion_opt_mut().unwrap()
        }
        ///Sets `inclusion` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_inclusion<T: Into<super::OcsInclusionProof>>(&mut self, field: T) {
            self.proof = Some(
                super::get_checkpoint_object_proof_response::Proof::Inclusion(
                    field.into().into(),
                ),
            );
        }
        ///Sets `inclusion` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_inclusion<T: Into<super::OcsInclusionProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_inclusion(field.into());
            self
        }
        ///Returns the value of `non_inclusion`, or the default value if `non_inclusion` is unset.
        pub fn non_inclusion(&self) -> &super::OcsNonInclusionProof {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::NonInclusion(field),
            ) = &self.proof
            {
                field as _
            } else {
                super::OcsNonInclusionProof::default_instance() as _
            }
        }
        ///If `non_inclusion` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn non_inclusion_opt(&self) -> Option<&super::OcsNonInclusionProof> {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::NonInclusion(field),
            ) = &self.proof
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `non_inclusion` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn non_inclusion_opt_mut(
            &mut self,
        ) -> Option<&mut super::OcsNonInclusionProof> {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::NonInclusion(field),
            ) = &mut self.proof
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `non_inclusion`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn non_inclusion_mut(&mut self) -> &mut super::OcsNonInclusionProof {
            if self.non_inclusion_opt_mut().is_none() {
                self.proof = Some(
                    super::get_checkpoint_object_proof_response::Proof::NonInclusion(
                        super::OcsNonInclusionProof::default(),
                    ),
                );
            }
            self.non_inclusion_opt_mut().unwrap()
        }
        ///Sets `non_inclusion` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_non_inclusion<T: Into<super::OcsNonInclusionProof>>(
            &mut self,
            field: T,
        ) {
            self.proof = Some(
                super::get_checkpoint_object_proof_response::Proof::NonInclusion(
                    field.into().into(),
                ),
            );
        }
        ///Sets `non_inclusion` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_non_inclusion<T: Into<super::OcsNonInclusionProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_non_inclusion(field.into());
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
                options: None,
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
        ///Returns the value of `options`, or the default value if `options` is unset.
        pub fn options(&self) -> &super::QueryOptions {
            self.options
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::QueryOptions::default_instance() as _)
        }
        ///If `options` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn options_opt_mut(&mut self) -> Option<&mut super::QueryOptions> {
            self.options.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `options`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn options_mut(&mut self) -> &mut super::QueryOptions {
            self.options.get_or_insert_default()
        }
        ///If `options` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn options_opt(&self) -> Option<&super::QueryOptions> {
            self.options.as_ref().map(|field| field as _)
        }
        ///Sets `options` with the provided value.
        pub fn set_options<T: Into<super::QueryOptions>>(&mut self, field: T) {
            self.options = Some(field.into().into());
        }
        ///Sets `options` with the provided value.
        pub fn with_options<T: Into<super::QueryOptions>>(mut self, field: T) -> Self {
            self.set_options(field.into());
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
        ///Returns the value of `watermark`, or the default value if `watermark` is unset.
        pub fn watermark(&self) -> &super::Watermark {
            if let Some(super::list_checkpoints_response::Response::Watermark(field)) = &self
                .response
            {
                field as _
            } else {
                super::Watermark::default_instance() as _
            }
        }
        ///If `watermark` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn watermark_opt(&self) -> Option<&super::Watermark> {
            if let Some(super::list_checkpoints_response::Response::Watermark(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `watermark` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn watermark_opt_mut(&mut self) -> Option<&mut super::Watermark> {
            if let Some(super::list_checkpoints_response::Response::Watermark(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `watermark`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn watermark_mut(&mut self) -> &mut super::Watermark {
            if self.watermark_opt_mut().is_none() {
                self.response = Some(
                    super::list_checkpoints_response::Response::Watermark(
                        super::Watermark::default(),
                    ),
                );
            }
            self.watermark_opt_mut().unwrap()
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_watermark<T: Into<super::Watermark>>(&mut self, field: T) {
            self.response = Some(
                super::list_checkpoints_response::Response::Watermark(
                    field.into().into(),
                ),
            );
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_watermark<T: Into<super::Watermark>>(mut self, field: T) -> Self {
            self.set_watermark(field.into());
            self
        }
        ///Returns the value of `end`, or the default value if `end` is unset.
        pub fn end(&self) -> &super::QueryEnd {
            if let Some(super::list_checkpoints_response::Response::End(field)) = &self
                .response
            {
                field as _
            } else {
                super::QueryEnd::default_instance() as _
            }
        }
        ///If `end` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn end_opt(&self) -> Option<&super::QueryEnd> {
            if let Some(super::list_checkpoints_response::Response::End(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `end` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn end_opt_mut(&mut self) -> Option<&mut super::QueryEnd> {
            if let Some(super::list_checkpoints_response::Response::End(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `end`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn end_mut(&mut self) -> &mut super::QueryEnd {
            if self.end_opt_mut().is_none() {
                self.response = Some(
                    super::list_checkpoints_response::Response::End(
                        super::QueryEnd::default(),
                    ),
                );
            }
            self.end_opt_mut().unwrap()
        }
        ///Sets `end` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_end<T: Into<super::QueryEnd>>(&mut self, field: T) {
            self.response = Some(
                super::list_checkpoints_response::Response::End(field.into().into()),
            );
        }
        ///Sets `end` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_end<T: Into<super::QueryEnd>>(mut self, field: T) -> Self {
            self.set_end(field.into());
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
                options: None,
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
        ///Returns the value of `options`, or the default value if `options` is unset.
        pub fn options(&self) -> &super::QueryOptions {
            self.options
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::QueryOptions::default_instance() as _)
        }
        ///If `options` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn options_opt_mut(&mut self) -> Option<&mut super::QueryOptions> {
            self.options.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `options`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn options_mut(&mut self) -> &mut super::QueryOptions {
            self.options.get_or_insert_default()
        }
        ///If `options` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn options_opt(&self) -> Option<&super::QueryOptions> {
            self.options.as_ref().map(|field| field as _)
        }
        ///Sets `options` with the provided value.
        pub fn set_options<T: Into<super::QueryOptions>>(&mut self, field: T) {
            self.options = Some(field.into().into());
        }
        ///Sets `options` with the provided value.
        pub fn with_options<T: Into<super::QueryOptions>>(mut self, field: T) -> Self {
            self.set_options(field.into());
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
        ///Returns the value of `watermark`, or the default value if `watermark` is unset.
        pub fn watermark(&self) -> &super::Watermark {
            if let Some(super::list_events_response::Response::Watermark(field)) = &self
                .response
            {
                field as _
            } else {
                super::Watermark::default_instance() as _
            }
        }
        ///If `watermark` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn watermark_opt(&self) -> Option<&super::Watermark> {
            if let Some(super::list_events_response::Response::Watermark(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `watermark` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn watermark_opt_mut(&mut self) -> Option<&mut super::Watermark> {
            if let Some(super::list_events_response::Response::Watermark(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `watermark`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn watermark_mut(&mut self) -> &mut super::Watermark {
            if self.watermark_opt_mut().is_none() {
                self.response = Some(
                    super::list_events_response::Response::Watermark(
                        super::Watermark::default(),
                    ),
                );
            }
            self.watermark_opt_mut().unwrap()
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_watermark<T: Into<super::Watermark>>(&mut self, field: T) {
            self.response = Some(
                super::list_events_response::Response::Watermark(field.into().into()),
            );
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_watermark<T: Into<super::Watermark>>(mut self, field: T) -> Self {
            self.set_watermark(field.into());
            self
        }
        ///Returns the value of `end`, or the default value if `end` is unset.
        pub fn end(&self) -> &super::QueryEnd {
            if let Some(super::list_events_response::Response::End(field)) = &self
                .response
            {
                field as _
            } else {
                super::QueryEnd::default_instance() as _
            }
        }
        ///If `end` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn end_opt(&self) -> Option<&super::QueryEnd> {
            if let Some(super::list_events_response::Response::End(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `end` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn end_opt_mut(&mut self) -> Option<&mut super::QueryEnd> {
            if let Some(super::list_events_response::Response::End(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `end`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn end_mut(&mut self) -> &mut super::QueryEnd {
            if self.end_opt_mut().is_none() {
                self.response = Some(
                    super::list_events_response::Response::End(
                        super::QueryEnd::default(),
                    ),
                );
            }
            self.end_opt_mut().unwrap()
        }
        ///Sets `end` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_end<T: Into<super::QueryEnd>>(&mut self, field: T) {
            self.response = Some(
                super::list_events_response::Response::End(field.into().into()),
            );
        }
        ///Sets `end` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_end<T: Into<super::QueryEnd>>(mut self, field: T) -> Self {
            self.set_end(field.into());
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
                options: None,
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
        ///Returns the value of `options`, or the default value if `options` is unset.
        pub fn options(&self) -> &super::QueryOptions {
            self.options
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::QueryOptions::default_instance() as _)
        }
        ///If `options` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn options_opt_mut(&mut self) -> Option<&mut super::QueryOptions> {
            self.options.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `options`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn options_mut(&mut self) -> &mut super::QueryOptions {
            self.options.get_or_insert_default()
        }
        ///If `options` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn options_opt(&self) -> Option<&super::QueryOptions> {
            self.options.as_ref().map(|field| field as _)
        }
        ///Sets `options` with the provided value.
        pub fn set_options<T: Into<super::QueryOptions>>(&mut self, field: T) {
            self.options = Some(field.into().into());
        }
        ///Sets `options` with the provided value.
        pub fn with_options<T: Into<super::QueryOptions>>(mut self, field: T) -> Self {
            self.set_options(field.into());
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
        ///Returns the value of `watermark`, or the default value if `watermark` is unset.
        pub fn watermark(&self) -> &super::Watermark {
            if let Some(super::list_transactions_response::Response::Watermark(field)) = &self
                .response
            {
                field as _
            } else {
                super::Watermark::default_instance() as _
            }
        }
        ///If `watermark` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn watermark_opt(&self) -> Option<&super::Watermark> {
            if let Some(super::list_transactions_response::Response::Watermark(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `watermark` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn watermark_opt_mut(&mut self) -> Option<&mut super::Watermark> {
            if let Some(super::list_transactions_response::Response::Watermark(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `watermark`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn watermark_mut(&mut self) -> &mut super::Watermark {
            if self.watermark_opt_mut().is_none() {
                self.response = Some(
                    super::list_transactions_response::Response::Watermark(
                        super::Watermark::default(),
                    ),
                );
            }
            self.watermark_opt_mut().unwrap()
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_watermark<T: Into<super::Watermark>>(&mut self, field: T) {
            self.response = Some(
                super::list_transactions_response::Response::Watermark(
                    field.into().into(),
                ),
            );
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_watermark<T: Into<super::Watermark>>(mut self, field: T) -> Self {
            self.set_watermark(field.into());
            self
        }
        ///Returns the value of `end`, or the default value if `end` is unset.
        pub fn end(&self) -> &super::QueryEnd {
            if let Some(super::list_transactions_response::Response::End(field)) = &self
                .response
            {
                field as _
            } else {
                super::QueryEnd::default_instance() as _
            }
        }
        ///If `end` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn end_opt(&self) -> Option<&super::QueryEnd> {
            if let Some(super::list_transactions_response::Response::End(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `end` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn end_opt_mut(&mut self) -> Option<&mut super::QueryEnd> {
            if let Some(super::list_transactions_response::Response::End(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `end`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn end_mut(&mut self) -> &mut super::QueryEnd {
            if self.end_opt_mut().is_none() {
                self.response = Some(
                    super::list_transactions_response::Response::End(
                        super::QueryEnd::default(),
                    ),
                );
            }
            self.end_opt_mut().unwrap()
        }
        ///Sets `end` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_end<T: Into<super::QueryEnd>>(&mut self, field: T) {
            self.response = Some(
                super::list_transactions_response::Response::End(field.into().into()),
            );
        }
        ///Sets `end` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_end<T: Into<super::QueryEnd>>(mut self, field: T) -> Self {
            self.set_end(field.into());
            self
        }
    }
    impl super::MerkleNeighbourLeaf {
        pub const fn const_default() -> Self {
            Self {
                leaf: None,
                merkle_proof: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MerkleNeighbourLeaf = super::MerkleNeighbourLeaf::const_default();
            &DEFAULT
        }
        ///Returns the value of `leaf`, or the default value if `leaf` is unset.
        pub fn leaf(&self) -> &super::super::v2::ObjectReference {
            self.leaf
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::super::v2::ObjectReference::default_instance() as _
                })
        }
        ///If `leaf` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn leaf_opt_mut(
            &mut self,
        ) -> Option<&mut super::super::v2::ObjectReference> {
            self.leaf.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `leaf`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn leaf_mut(&mut self) -> &mut super::super::v2::ObjectReference {
            self.leaf.get_or_insert_default()
        }
        ///If `leaf` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn leaf_opt(&self) -> Option<&super::super::v2::ObjectReference> {
            self.leaf.as_ref().map(|field| field as _)
        }
        ///Sets `leaf` with the provided value.
        pub fn set_leaf<T: Into<super::super::v2::ObjectReference>>(
            &mut self,
            field: T,
        ) {
            self.leaf = Some(field.into().into());
        }
        ///Sets `leaf` with the provided value.
        pub fn with_leaf<T: Into<super::super::v2::ObjectReference>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_leaf(field.into());
            self
        }
        ///Returns the value of `merkle_proof`, or the default value if `merkle_proof` is unset.
        pub fn merkle_proof(&self) -> &super::MerkleProof {
            self.merkle_proof
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MerkleProof::default_instance() as _)
        }
        ///If `merkle_proof` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn merkle_proof_opt_mut(&mut self) -> Option<&mut super::MerkleProof> {
            self.merkle_proof.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `merkle_proof`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn merkle_proof_mut(&mut self) -> &mut super::MerkleProof {
            self.merkle_proof.get_or_insert_default()
        }
        ///If `merkle_proof` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn merkle_proof_opt(&self) -> Option<&super::MerkleProof> {
            self.merkle_proof.as_ref().map(|field| field as _)
        }
        ///Sets `merkle_proof` with the provided value.
        pub fn set_merkle_proof<T: Into<super::MerkleProof>>(&mut self, field: T) {
            self.merkle_proof = Some(field.into().into());
        }
        ///Sets `merkle_proof` with the provided value.
        pub fn with_merkle_proof<T: Into<super::MerkleProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_merkle_proof(field.into());
            self
        }
    }
    impl super::MerkleNode {
        pub const fn const_default() -> Self {
            Self { node: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MerkleNode = super::MerkleNode::const_default();
            &DEFAULT
        }
        ///Returns the value of `digest`, or the default value if `digest` is unset.
        pub fn digest(&self) -> &[u8] {
            if let Some(super::merkle_node::Node::Digest(field)) = &self.node {
                field as _
            } else {
                &[]
            }
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&[u8]> {
            if let Some(super::merkle_node::Node::Digest(field)) = &self.node {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut ::prost::bytes::Bytes> {
            if let Some(super::merkle_node::Node::Digest(field)) = &mut self.node {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn digest_mut(&mut self) -> &mut ::prost::bytes::Bytes {
            if self.digest_opt_mut().is_none() {
                self.node = Some(
                    super::merkle_node::Node::Digest(::prost::bytes::Bytes::default()),
                );
            }
            self.digest_opt_mut().unwrap()
        }
        ///Sets `digest` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_digest<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.node = Some(super::merkle_node::Node::Digest(field.into().into()));
        }
        ///Sets `digest` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_digest<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
    }
    impl super::MerkleNonInclusionProof {
        pub const fn const_default() -> Self {
            Self {
                index: None,
                left_leaf: None,
                right_leaf: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MerkleNonInclusionProof = super::MerkleNonInclusionProof::const_default();
            &DEFAULT
        }
        ///If `index` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn index_opt_mut(&mut self) -> Option<&mut u64> {
            self.index.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `index`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn index_mut(&mut self) -> &mut u64 {
            self.index.get_or_insert_default()
        }
        ///If `index` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn index_opt(&self) -> Option<u64> {
            self.index.as_ref().map(|field| *field)
        }
        ///Sets `index` with the provided value.
        pub fn set_index(&mut self, field: u64) {
            self.index = Some(field);
        }
        ///Sets `index` with the provided value.
        pub fn with_index(mut self, field: u64) -> Self {
            self.set_index(field);
            self
        }
        ///Returns the value of `left_leaf`, or the default value if `left_leaf` is unset.
        pub fn left_leaf(&self) -> &super::MerkleNeighbourLeaf {
            self.left_leaf
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MerkleNeighbourLeaf::default_instance() as _)
        }
        ///If `left_leaf` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn left_leaf_opt_mut(&mut self) -> Option<&mut super::MerkleNeighbourLeaf> {
            self.left_leaf.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `left_leaf`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn left_leaf_mut(&mut self) -> &mut super::MerkleNeighbourLeaf {
            self.left_leaf.get_or_insert_default()
        }
        ///If `left_leaf` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn left_leaf_opt(&self) -> Option<&super::MerkleNeighbourLeaf> {
            self.left_leaf.as_ref().map(|field| field as _)
        }
        ///Sets `left_leaf` with the provided value.
        pub fn set_left_leaf<T: Into<super::MerkleNeighbourLeaf>>(&mut self, field: T) {
            self.left_leaf = Some(field.into().into());
        }
        ///Sets `left_leaf` with the provided value.
        pub fn with_left_leaf<T: Into<super::MerkleNeighbourLeaf>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_left_leaf(field.into());
            self
        }
        ///Returns the value of `right_leaf`, or the default value if `right_leaf` is unset.
        pub fn right_leaf(&self) -> &super::MerkleNeighbourLeaf {
            self.right_leaf
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MerkleNeighbourLeaf::default_instance() as _)
        }
        ///If `right_leaf` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn right_leaf_opt_mut(&mut self) -> Option<&mut super::MerkleNeighbourLeaf> {
            self.right_leaf.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `right_leaf`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn right_leaf_mut(&mut self) -> &mut super::MerkleNeighbourLeaf {
            self.right_leaf.get_or_insert_default()
        }
        ///If `right_leaf` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn right_leaf_opt(&self) -> Option<&super::MerkleNeighbourLeaf> {
            self.right_leaf.as_ref().map(|field| field as _)
        }
        ///Sets `right_leaf` with the provided value.
        pub fn set_right_leaf<T: Into<super::MerkleNeighbourLeaf>>(&mut self, field: T) {
            self.right_leaf = Some(field.into().into());
        }
        ///Sets `right_leaf` with the provided value.
        pub fn with_right_leaf<T: Into<super::MerkleNeighbourLeaf>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_right_leaf(field.into());
            self
        }
    }
    impl super::MerkleProof {
        pub const fn const_default() -> Self {
            Self { path: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MerkleProof = super::MerkleProof::const_default();
            &DEFAULT
        }
        ///Returns the value of `path`, or the default value if `path` is unset.
        pub fn path(&self) -> &[super::MerkleNode] {
            &self.path
        }
        ///Returns a mutable reference to `path`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn path_mut(&mut self) -> &mut Vec<super::MerkleNode> {
            &mut self.path
        }
        ///Sets `path` with the provided value.
        pub fn set_path(&mut self, field: Vec<super::MerkleNode>) {
            self.path = field;
        }
        ///Sets `path` with the provided value.
        pub fn with_path(mut self, field: Vec<super::MerkleNode>) -> Self {
            self.set_path(field);
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
    impl super::OcsInclusionProof {
        pub const fn const_default() -> Self {
            Self {
                object_ref: None,
                merkle_proof: None,
                leaf_index: None,
                tree_root: None,
                object_data: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::OcsInclusionProof = super::OcsInclusionProof::const_default();
            &DEFAULT
        }
        ///Returns the value of `object_ref`, or the default value if `object_ref` is unset.
        pub fn object_ref(&self) -> &super::super::v2::ObjectReference {
            self.object_ref
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::super::v2::ObjectReference::default_instance() as _
                })
        }
        ///If `object_ref` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_ref_opt_mut(
            &mut self,
        ) -> Option<&mut super::super::v2::ObjectReference> {
            self.object_ref.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_ref`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_ref_mut(&mut self) -> &mut super::super::v2::ObjectReference {
            self.object_ref.get_or_insert_default()
        }
        ///If `object_ref` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_ref_opt(&self) -> Option<&super::super::v2::ObjectReference> {
            self.object_ref.as_ref().map(|field| field as _)
        }
        ///Sets `object_ref` with the provided value.
        pub fn set_object_ref<T: Into<super::super::v2::ObjectReference>>(
            &mut self,
            field: T,
        ) {
            self.object_ref = Some(field.into().into());
        }
        ///Sets `object_ref` with the provided value.
        pub fn with_object_ref<T: Into<super::super::v2::ObjectReference>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_object_ref(field.into());
            self
        }
        ///Returns the value of `merkle_proof`, or the default value if `merkle_proof` is unset.
        pub fn merkle_proof(&self) -> &super::MerkleProof {
            self.merkle_proof
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MerkleProof::default_instance() as _)
        }
        ///If `merkle_proof` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn merkle_proof_opt_mut(&mut self) -> Option<&mut super::MerkleProof> {
            self.merkle_proof.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `merkle_proof`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn merkle_proof_mut(&mut self) -> &mut super::MerkleProof {
            self.merkle_proof.get_or_insert_default()
        }
        ///If `merkle_proof` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn merkle_proof_opt(&self) -> Option<&super::MerkleProof> {
            self.merkle_proof.as_ref().map(|field| field as _)
        }
        ///Sets `merkle_proof` with the provided value.
        pub fn set_merkle_proof<T: Into<super::MerkleProof>>(&mut self, field: T) {
            self.merkle_proof = Some(field.into().into());
        }
        ///Sets `merkle_proof` with the provided value.
        pub fn with_merkle_proof<T: Into<super::MerkleProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_merkle_proof(field.into());
            self
        }
        ///If `leaf_index` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn leaf_index_opt_mut(&mut self) -> Option<&mut u64> {
            self.leaf_index.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `leaf_index`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn leaf_index_mut(&mut self) -> &mut u64 {
            self.leaf_index.get_or_insert_default()
        }
        ///If `leaf_index` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn leaf_index_opt(&self) -> Option<u64> {
            self.leaf_index.as_ref().map(|field| *field)
        }
        ///Sets `leaf_index` with the provided value.
        pub fn set_leaf_index(&mut self, field: u64) {
            self.leaf_index = Some(field);
        }
        ///Sets `leaf_index` with the provided value.
        pub fn with_leaf_index(mut self, field: u64) -> Self {
            self.set_leaf_index(field);
            self
        }
        ///If `tree_root` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn tree_root_opt(&self) -> Option<&[u8]> {
            self.tree_root.as_ref().map(|field| field as _)
        }
        ///Sets `tree_root` with the provided value.
        pub fn set_tree_root<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.tree_root = Some(field.into().into());
        }
        ///Sets `tree_root` with the provided value.
        pub fn with_tree_root<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_tree_root(field.into());
            self
        }
        ///If `object_data` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_data_opt(&self) -> Option<&[u8]> {
            self.object_data.as_ref().map(|field| field as _)
        }
        ///Sets `object_data` with the provided value.
        pub fn set_object_data<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.object_data = Some(field.into().into());
        }
        ///Sets `object_data` with the provided value.
        pub fn with_object_data<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_object_data(field.into());
            self
        }
    }
    impl super::OcsNonInclusionProof {
        pub const fn const_default() -> Self {
            Self {
                non_inclusion_proof: None,
                tree_root: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::OcsNonInclusionProof = super::OcsNonInclusionProof::const_default();
            &DEFAULT
        }
        ///Returns the value of `non_inclusion_proof`, or the default value if `non_inclusion_proof` is unset.
        pub fn non_inclusion_proof(&self) -> &super::MerkleNonInclusionProof {
            self.non_inclusion_proof
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::MerkleNonInclusionProof::default_instance() as _
                })
        }
        ///If `non_inclusion_proof` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn non_inclusion_proof_opt_mut(
            &mut self,
        ) -> Option<&mut super::MerkleNonInclusionProof> {
            self.non_inclusion_proof.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `non_inclusion_proof`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn non_inclusion_proof_mut(
            &mut self,
        ) -> &mut super::MerkleNonInclusionProof {
            self.non_inclusion_proof.get_or_insert_default()
        }
        ///If `non_inclusion_proof` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn non_inclusion_proof_opt(
            &self,
        ) -> Option<&super::MerkleNonInclusionProof> {
            self.non_inclusion_proof.as_ref().map(|field| field as _)
        }
        ///Sets `non_inclusion_proof` with the provided value.
        pub fn set_non_inclusion_proof<T: Into<super::MerkleNonInclusionProof>>(
            &mut self,
            field: T,
        ) {
            self.non_inclusion_proof = Some(field.into().into());
        }
        ///Sets `non_inclusion_proof` with the provided value.
        pub fn with_non_inclusion_proof<T: Into<super::MerkleNonInclusionProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_non_inclusion_proof(field.into());
            self
        }
        ///If `tree_root` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn tree_root_opt(&self) -> Option<&[u8]> {
            self.tree_root.as_ref().map(|field| field as _)
        }
        ///Sets `tree_root` with the provided value.
        pub fn set_tree_root<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.tree_root = Some(field.into().into());
        }
        ///Sets `tree_root` with the provided value.
        pub fn with_tree_root<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_tree_root(field.into());
            self
        }
    }
    impl super::PackageWriteFilter {
        pub const fn const_default() -> Self {
            Self {}
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::PackageWriteFilter = super::PackageWriteFilter::const_default();
            &DEFAULT
        }
    }
    impl super::QueryEnd {
        pub const fn const_default() -> Self {
            Self { reason: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::QueryEnd = super::QueryEnd::const_default();
            &DEFAULT
        }
        ///Sets `reason` with the provided value.
        pub fn with_reason<T: Into<super::QueryEndReason>>(mut self, field: T) -> Self {
            self.set_reason(field.into());
            self
        }
    }
    impl super::QueryOptions {
        pub const fn const_default() -> Self {
            Self {
                limit: None,
                after: None,
                before: None,
                ordering: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::QueryOptions = super::QueryOptions::const_default();
            &DEFAULT
        }
        ///If `limit` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn limit_opt_mut(&mut self) -> Option<&mut u32> {
            self.limit.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `limit`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn limit_mut(&mut self) -> &mut u32 {
            self.limit.get_or_insert_default()
        }
        ///If `limit` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn limit_opt(&self) -> Option<u32> {
            self.limit.as_ref().map(|field| *field)
        }
        ///Sets `limit` with the provided value.
        pub fn set_limit(&mut self, field: u32) {
            self.limit = Some(field);
        }
        ///Sets `limit` with the provided value.
        pub fn with_limit(mut self, field: u32) -> Self {
            self.set_limit(field);
            self
        }
        ///If `after` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn after_opt(&self) -> Option<&[u8]> {
            self.after.as_ref().map(|field| field as _)
        }
        ///Sets `after` with the provided value.
        pub fn set_after<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.after = Some(field.into().into());
        }
        ///Sets `after` with the provided value.
        pub fn with_after<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_after(field.into());
            self
        }
        ///If `before` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn before_opt(&self) -> Option<&[u8]> {
            self.before.as_ref().map(|field| field as _)
        }
        ///Sets `before` with the provided value.
        pub fn set_before<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.before = Some(field.into().into());
        }
        ///Sets `before` with the provided value.
        pub fn with_before<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_before(field.into());
            self
        }
        ///Sets `ordering` with the provided value.
        pub fn with_ordering<T: Into<super::Ordering>>(mut self, field: T) -> Self {
            self.set_ordering(field.into());
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
    impl super::SubscribeCheckpointsRequest {
        pub const fn const_default() -> Self {
            Self {
                read_mask: None,
                filter: None,
            }
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
    }
    impl super::SubscribeCheckpointsResponse {
        pub const fn const_default() -> Self {
            Self { response: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SubscribeCheckpointsResponse = super::SubscribeCheckpointsResponse::const_default();
            &DEFAULT
        }
        ///Returns the value of `item`, or the default value if `item` is unset.
        pub fn item(&self) -> &super::CheckpointItem {
            if let Some(super::subscribe_checkpoints_response::Response::Item(field)) = &self
                .response
            {
                field as _
            } else {
                super::CheckpointItem::default_instance() as _
            }
        }
        ///If `item` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn item_opt(&self) -> Option<&super::CheckpointItem> {
            if let Some(super::subscribe_checkpoints_response::Response::Item(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `item` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn item_opt_mut(&mut self) -> Option<&mut super::CheckpointItem> {
            if let Some(super::subscribe_checkpoints_response::Response::Item(field)) = &mut self
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
                    super::subscribe_checkpoints_response::Response::Item(
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
                super::subscribe_checkpoints_response::Response::Item(
                    field.into().into(),
                ),
            );
        }
        ///Sets `item` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_item<T: Into<super::CheckpointItem>>(mut self, field: T) -> Self {
            self.set_item(field.into());
            self
        }
        ///Returns the value of `watermark`, or the default value if `watermark` is unset.
        pub fn watermark(&self) -> &super::Watermark {
            if let Some(
                super::subscribe_checkpoints_response::Response::Watermark(field),
            ) = &self.response
            {
                field as _
            } else {
                super::Watermark::default_instance() as _
            }
        }
        ///If `watermark` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn watermark_opt(&self) -> Option<&super::Watermark> {
            if let Some(
                super::subscribe_checkpoints_response::Response::Watermark(field),
            ) = &self.response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `watermark` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn watermark_opt_mut(&mut self) -> Option<&mut super::Watermark> {
            if let Some(
                super::subscribe_checkpoints_response::Response::Watermark(field),
            ) = &mut self.response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `watermark`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn watermark_mut(&mut self) -> &mut super::Watermark {
            if self.watermark_opt_mut().is_none() {
                self.response = Some(
                    super::subscribe_checkpoints_response::Response::Watermark(
                        super::Watermark::default(),
                    ),
                );
            }
            self.watermark_opt_mut().unwrap()
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_watermark<T: Into<super::Watermark>>(&mut self, field: T) {
            self.response = Some(
                super::subscribe_checkpoints_response::Response::Watermark(
                    field.into().into(),
                ),
            );
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_watermark<T: Into<super::Watermark>>(mut self, field: T) -> Self {
            self.set_watermark(field.into());
            self
        }
    }
    impl super::SubscribeEventsRequest {
        pub const fn const_default() -> Self {
            Self {
                read_mask: None,
                filter: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SubscribeEventsRequest = super::SubscribeEventsRequest::const_default();
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
    }
    impl super::SubscribeEventsResponse {
        pub const fn const_default() -> Self {
            Self { response: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SubscribeEventsResponse = super::SubscribeEventsResponse::const_default();
            &DEFAULT
        }
        ///Returns the value of `item`, or the default value if `item` is unset.
        pub fn item(&self) -> &super::EventItem {
            if let Some(super::subscribe_events_response::Response::Item(field)) = &self
                .response
            {
                field as _
            } else {
                super::EventItem::default_instance() as _
            }
        }
        ///If `item` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn item_opt(&self) -> Option<&super::EventItem> {
            if let Some(super::subscribe_events_response::Response::Item(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `item` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn item_opt_mut(&mut self) -> Option<&mut super::EventItem> {
            if let Some(super::subscribe_events_response::Response::Item(field)) = &mut self
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
                    super::subscribe_events_response::Response::Item(
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
                super::subscribe_events_response::Response::Item(field.into().into()),
            );
        }
        ///Sets `item` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_item<T: Into<super::EventItem>>(mut self, field: T) -> Self {
            self.set_item(field.into());
            self
        }
        ///Returns the value of `watermark`, or the default value if `watermark` is unset.
        pub fn watermark(&self) -> &super::Watermark {
            if let Some(super::subscribe_events_response::Response::Watermark(field)) = &self
                .response
            {
                field as _
            } else {
                super::Watermark::default_instance() as _
            }
        }
        ///If `watermark` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn watermark_opt(&self) -> Option<&super::Watermark> {
            if let Some(super::subscribe_events_response::Response::Watermark(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `watermark` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn watermark_opt_mut(&mut self) -> Option<&mut super::Watermark> {
            if let Some(super::subscribe_events_response::Response::Watermark(field)) = &mut self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `watermark`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn watermark_mut(&mut self) -> &mut super::Watermark {
            if self.watermark_opt_mut().is_none() {
                self.response = Some(
                    super::subscribe_events_response::Response::Watermark(
                        super::Watermark::default(),
                    ),
                );
            }
            self.watermark_opt_mut().unwrap()
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_watermark<T: Into<super::Watermark>>(&mut self, field: T) {
            self.response = Some(
                super::subscribe_events_response::Response::Watermark(
                    field.into().into(),
                ),
            );
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_watermark<T: Into<super::Watermark>>(mut self, field: T) -> Self {
            self.set_watermark(field.into());
            self
        }
    }
    impl super::SubscribeTransactionsRequest {
        pub const fn const_default() -> Self {
            Self {
                read_mask: None,
                filter: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SubscribeTransactionsRequest = super::SubscribeTransactionsRequest::const_default();
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
    }
    impl super::SubscribeTransactionsResponse {
        pub const fn const_default() -> Self {
            Self { response: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::SubscribeTransactionsResponse = super::SubscribeTransactionsResponse::const_default();
            &DEFAULT
        }
        ///Returns the value of `item`, or the default value if `item` is unset.
        pub fn item(&self) -> &super::TransactionItem {
            if let Some(super::subscribe_transactions_response::Response::Item(field)) = &self
                .response
            {
                field as _
            } else {
                super::TransactionItem::default_instance() as _
            }
        }
        ///If `item` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn item_opt(&self) -> Option<&super::TransactionItem> {
            if let Some(super::subscribe_transactions_response::Response::Item(field)) = &self
                .response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `item` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn item_opt_mut(&mut self) -> Option<&mut super::TransactionItem> {
            if let Some(super::subscribe_transactions_response::Response::Item(field)) = &mut self
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
                    super::subscribe_transactions_response::Response::Item(
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
                super::subscribe_transactions_response::Response::Item(
                    field.into().into(),
                ),
            );
        }
        ///Sets `item` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_item<T: Into<super::TransactionItem>>(mut self, field: T) -> Self {
            self.set_item(field.into());
            self
        }
        ///Returns the value of `watermark`, or the default value if `watermark` is unset.
        pub fn watermark(&self) -> &super::Watermark {
            if let Some(
                super::subscribe_transactions_response::Response::Watermark(field),
            ) = &self.response
            {
                field as _
            } else {
                super::Watermark::default_instance() as _
            }
        }
        ///If `watermark` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn watermark_opt(&self) -> Option<&super::Watermark> {
            if let Some(
                super::subscribe_transactions_response::Response::Watermark(field),
            ) = &self.response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `watermark` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn watermark_opt_mut(&mut self) -> Option<&mut super::Watermark> {
            if let Some(
                super::subscribe_transactions_response::Response::Watermark(field),
            ) = &mut self.response
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `watermark`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn watermark_mut(&mut self) -> &mut super::Watermark {
            if self.watermark_opt_mut().is_none() {
                self.response = Some(
                    super::subscribe_transactions_response::Response::Watermark(
                        super::Watermark::default(),
                    ),
                );
            }
            self.watermark_opt_mut().unwrap()
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_watermark<T: Into<super::Watermark>>(&mut self, field: T) {
            self.response = Some(
                super::subscribe_transactions_response::Response::Watermark(
                    field.into().into(),
                ),
            );
        }
        ///Sets `watermark` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_watermark<T: Into<super::Watermark>>(mut self, field: T) -> Self {
            self.set_watermark(field.into());
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
                transaction: None,
                watermark: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionItem = super::TransactionItem::const_default();
            &DEFAULT
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
        ///Returns the value of `watermark`, or the default value if `watermark` is unset.
        pub fn watermark(&self) -> &super::Watermark {
            self.watermark
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::Watermark::default_instance() as _)
        }
        ///If `watermark` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn watermark_opt_mut(&mut self) -> Option<&mut super::Watermark> {
            self.watermark.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `watermark`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn watermark_mut(&mut self) -> &mut super::Watermark {
            self.watermark.get_or_insert_default()
        }
        ///If `watermark` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn watermark_opt(&self) -> Option<&super::Watermark> {
            self.watermark.as_ref().map(|field| field as _)
        }
        ///Sets `watermark` with the provided value.
        pub fn set_watermark<T: Into<super::Watermark>>(&mut self, field: T) {
            self.watermark = Some(field.into().into());
        }
        ///Sets `watermark` with the provided value.
        pub fn with_watermark<T: Into<super::Watermark>>(mut self, field: T) -> Self {
            self.set_watermark(field.into());
            self
        }
    }
    impl super::TransactionLiteral {
        pub const fn const_default() -> Self {
            Self {
                negated: false,
                predicate: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::TransactionLiteral = super::TransactionLiteral::const_default();
            &DEFAULT
        }
        ///Returns a mutable reference to `negated`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn negated_mut(&mut self) -> &mut bool {
            &mut self.negated
        }
        ///Sets `negated` with the provided value.
        pub fn set_negated(&mut self, field: bool) {
            self.negated = field;
        }
        ///Sets `negated` with the provided value.
        pub fn with_negated(mut self, field: bool) -> Self {
            self.set_negated(field);
            self
        }
        ///Returns the value of `sender`, or the default value if `sender` is unset.
        pub fn sender(&self) -> &super::SenderFilter {
            if let Some(super::transaction_literal::Predicate::Sender(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::SenderFilter::default_instance() as _
            }
        }
        ///If `sender` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn sender_opt(&self) -> Option<&super::SenderFilter> {
            if let Some(super::transaction_literal::Predicate::Sender(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `sender` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn sender_opt_mut(&mut self) -> Option<&mut super::SenderFilter> {
            if let Some(super::transaction_literal::Predicate::Sender(field)) = &mut self
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
                    super::transaction_literal::Predicate::Sender(
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
                super::transaction_literal::Predicate::Sender(field.into().into()),
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
            if let Some(super::transaction_literal::Predicate::AffectedAddress(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::AffectedAddressFilter::default_instance() as _
            }
        }
        ///If `affected_address` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn affected_address_opt(&self) -> Option<&super::AffectedAddressFilter> {
            if let Some(super::transaction_literal::Predicate::AffectedAddress(field)) = &self
                .predicate
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
            if let Some(super::transaction_literal::Predicate::AffectedAddress(field)) = &mut self
                .predicate
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
                    super::transaction_literal::Predicate::AffectedAddress(
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
                super::transaction_literal::Predicate::AffectedAddress(
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
            if let Some(super::transaction_literal::Predicate::AffectedObject(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::AffectedObjectFilter::default_instance() as _
            }
        }
        ///If `affected_object` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn affected_object_opt(&self) -> Option<&super::AffectedObjectFilter> {
            if let Some(super::transaction_literal::Predicate::AffectedObject(field)) = &self
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
            if let Some(super::transaction_literal::Predicate::AffectedObject(field)) = &mut self
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
                    super::transaction_literal::Predicate::AffectedObject(
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
                super::transaction_literal::Predicate::AffectedObject(
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
            if let Some(super::transaction_literal::Predicate::MoveCall(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::MoveCallFilter::default_instance() as _
            }
        }
        ///If `move_call` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn move_call_opt(&self) -> Option<&super::MoveCallFilter> {
            if let Some(super::transaction_literal::Predicate::MoveCall(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `move_call` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn move_call_opt_mut(&mut self) -> Option<&mut super::MoveCallFilter> {
            if let Some(super::transaction_literal::Predicate::MoveCall(field)) = &mut self
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
                    super::transaction_literal::Predicate::MoveCall(
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
                super::transaction_literal::Predicate::MoveCall(field.into().into()),
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
            if let Some(super::transaction_literal::Predicate::EmitModule(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::EmitModuleFilter::default_instance() as _
            }
        }
        ///If `emit_module` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn emit_module_opt(&self) -> Option<&super::EmitModuleFilter> {
            if let Some(super::transaction_literal::Predicate::EmitModule(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `emit_module` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn emit_module_opt_mut(&mut self) -> Option<&mut super::EmitModuleFilter> {
            if let Some(super::transaction_literal::Predicate::EmitModule(field)) = &mut self
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
                    super::transaction_literal::Predicate::EmitModule(
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
                super::transaction_literal::Predicate::EmitModule(field.into().into()),
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
            if let Some(super::transaction_literal::Predicate::EventType(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::EventTypeFilter::default_instance() as _
            }
        }
        ///If `event_type` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn event_type_opt(&self) -> Option<&super::EventTypeFilter> {
            if let Some(super::transaction_literal::Predicate::EventType(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `event_type` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn event_type_opt_mut(&mut self) -> Option<&mut super::EventTypeFilter> {
            if let Some(super::transaction_literal::Predicate::EventType(field)) = &mut self
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
                    super::transaction_literal::Predicate::EventType(
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
                super::transaction_literal::Predicate::EventType(field.into().into()),
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
            if let Some(super::transaction_literal::Predicate::EventStreamHead(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::EventStreamHeadFilter::default_instance() as _
            }
        }
        ///If `event_stream_head` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn event_stream_head_opt(&self) -> Option<&super::EventStreamHeadFilter> {
            if let Some(super::transaction_literal::Predicate::EventStreamHead(field)) = &self
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
            if let Some(super::transaction_literal::Predicate::EventStreamHead(field)) = &mut self
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
                    super::transaction_literal::Predicate::EventStreamHead(
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
                super::transaction_literal::Predicate::EventStreamHead(
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
        ///Returns the value of `package_write`, or the default value if `package_write` is unset.
        pub fn package_write(&self) -> &super::PackageWriteFilter {
            if let Some(super::transaction_literal::Predicate::PackageWrite(field)) = &self
                .predicate
            {
                field as _
            } else {
                super::PackageWriteFilter::default_instance() as _
            }
        }
        ///If `package_write` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn package_write_opt(&self) -> Option<&super::PackageWriteFilter> {
            if let Some(super::transaction_literal::Predicate::PackageWrite(field)) = &self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `package_write` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn package_write_opt_mut(
            &mut self,
        ) -> Option<&mut super::PackageWriteFilter> {
            if let Some(super::transaction_literal::Predicate::PackageWrite(field)) = &mut self
                .predicate
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `package_write`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn package_write_mut(&mut self) -> &mut super::PackageWriteFilter {
            if self.package_write_opt_mut().is_none() {
                self.predicate = Some(
                    super::transaction_literal::Predicate::PackageWrite(
                        super::PackageWriteFilter::default(),
                    ),
                );
            }
            self.package_write_opt_mut().unwrap()
        }
        ///Sets `package_write` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_package_write<T: Into<super::PackageWriteFilter>>(
            &mut self,
            field: T,
        ) {
            self.predicate = Some(
                super::transaction_literal::Predicate::PackageWrite(field.into().into()),
            );
        }
        ///Sets `package_write` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_package_write<T: Into<super::PackageWriteFilter>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_package_write(field.into());
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
    impl super::Watermark {
        pub const fn const_default() -> Self {
            Self {
                cursor: None,
                checkpoint: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Watermark = super::Watermark::const_default();
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
    }
}
