pub(crate) mod _field_impls {
    #![allow(clippy::wrong_self_convention)]
    use super::*;
    use crate::field::MessageFields;
    use crate::field::MessageField;
    impl TransactionFilter {
        pub const TERMS_FIELD: &'static MessageField = &MessageField {
            name: "terms",
            json_name: "terms",
            number: 1i32,
            message_fields: Some(TransactionTerm::FIELDS),
        };
    }
    impl MessageFields for TransactionFilter {
        const FIELDS: &'static [&'static MessageField] = &[Self::TERMS_FIELD];
    }
    impl TransactionFilter {
        pub fn path_builder() -> TransactionFilterFieldPathBuilder {
            TransactionFilterFieldPathBuilder::new()
        }
    }
    pub struct TransactionFilterFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TransactionFilterFieldPathBuilder {
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
        pub fn terms(mut self) -> TransactionTermFieldPathBuilder {
            self.path.push(TransactionFilter::TERMS_FIELD.name);
            TransactionTermFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl TransactionTerm {
        pub const LITERALS_FIELD: &'static MessageField = &MessageField {
            name: "literals",
            json_name: "literals",
            number: 1i32,
            message_fields: Some(TransactionLiteral::FIELDS),
        };
    }
    impl MessageFields for TransactionTerm {
        const FIELDS: &'static [&'static MessageField] = &[Self::LITERALS_FIELD];
    }
    impl TransactionTerm {
        pub fn path_builder() -> TransactionTermFieldPathBuilder {
            TransactionTermFieldPathBuilder::new()
        }
    }
    pub struct TransactionTermFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TransactionTermFieldPathBuilder {
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
        pub fn literals(mut self) -> TransactionLiteralFieldPathBuilder {
            self.path.push(TransactionTerm::LITERALS_FIELD.name);
            TransactionLiteralFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl TransactionLiteral {
        pub const INCLUDE_FIELD: &'static MessageField = &MessageField {
            name: "include",
            json_name: "include",
            number: 1i32,
            message_fields: Some(TransactionPredicate::FIELDS),
        };
        pub const EXCLUDE_FIELD: &'static MessageField = &MessageField {
            name: "exclude",
            json_name: "exclude",
            number: 2i32,
            message_fields: Some(TransactionPredicate::FIELDS),
        };
    }
    impl MessageFields for TransactionLiteral {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::INCLUDE_FIELD,
            Self::EXCLUDE_FIELD,
        ];
    }
    impl TransactionLiteral {
        pub fn path_builder() -> TransactionLiteralFieldPathBuilder {
            TransactionLiteralFieldPathBuilder::new()
        }
    }
    pub struct TransactionLiteralFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TransactionLiteralFieldPathBuilder {
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
        pub fn include(mut self) -> TransactionPredicateFieldPathBuilder {
            self.path.push(TransactionLiteral::INCLUDE_FIELD.name);
            TransactionPredicateFieldPathBuilder::new_with_base(self.path)
        }
        pub fn exclude(mut self) -> TransactionPredicateFieldPathBuilder {
            self.path.push(TransactionLiteral::EXCLUDE_FIELD.name);
            TransactionPredicateFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl TransactionPredicate {
        pub const SENDER_FIELD: &'static MessageField = &MessageField {
            name: "sender",
            json_name: "sender",
            number: 1i32,
            message_fields: Some(SenderFilter::FIELDS),
        };
        pub const AFFECTED_ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "affected_address",
            json_name: "affectedAddress",
            number: 2i32,
            message_fields: Some(AffectedAddressFilter::FIELDS),
        };
        pub const AFFECTED_OBJECT_FIELD: &'static MessageField = &MessageField {
            name: "affected_object",
            json_name: "affectedObject",
            number: 3i32,
            message_fields: Some(AffectedObjectFilter::FIELDS),
        };
        pub const MOVE_CALL_FIELD: &'static MessageField = &MessageField {
            name: "move_call",
            json_name: "moveCall",
            number: 4i32,
            message_fields: Some(MoveCallFilter::FIELDS),
        };
        pub const EMIT_MODULE_FIELD: &'static MessageField = &MessageField {
            name: "emit_module",
            json_name: "emitModule",
            number: 5i32,
            message_fields: Some(EmitModuleFilter::FIELDS),
        };
        pub const EVENT_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "event_type",
            json_name: "eventType",
            number: 6i32,
            message_fields: Some(EventTypeFilter::FIELDS),
        };
        pub const EVENT_STREAM_HEAD_FIELD: &'static MessageField = &MessageField {
            name: "event_stream_head",
            json_name: "eventStreamHead",
            number: 7i32,
            message_fields: Some(EventStreamHeadFilter::FIELDS),
        };
    }
    impl MessageFields for TransactionPredicate {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::SENDER_FIELD,
            Self::AFFECTED_ADDRESS_FIELD,
            Self::AFFECTED_OBJECT_FIELD,
            Self::MOVE_CALL_FIELD,
            Self::EMIT_MODULE_FIELD,
            Self::EVENT_TYPE_FIELD,
            Self::EVENT_STREAM_HEAD_FIELD,
        ];
    }
    impl TransactionPredicate {
        pub fn path_builder() -> TransactionPredicateFieldPathBuilder {
            TransactionPredicateFieldPathBuilder::new()
        }
    }
    pub struct TransactionPredicateFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TransactionPredicateFieldPathBuilder {
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
        pub fn sender(mut self) -> SenderFilterFieldPathBuilder {
            self.path.push(TransactionPredicate::SENDER_FIELD.name);
            SenderFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn affected_address(mut self) -> AffectedAddressFilterFieldPathBuilder {
            self.path.push(TransactionPredicate::AFFECTED_ADDRESS_FIELD.name);
            AffectedAddressFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn affected_object(mut self) -> AffectedObjectFilterFieldPathBuilder {
            self.path.push(TransactionPredicate::AFFECTED_OBJECT_FIELD.name);
            AffectedObjectFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn move_call(mut self) -> MoveCallFilterFieldPathBuilder {
            self.path.push(TransactionPredicate::MOVE_CALL_FIELD.name);
            MoveCallFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn emit_module(mut self) -> EmitModuleFilterFieldPathBuilder {
            self.path.push(TransactionPredicate::EMIT_MODULE_FIELD.name);
            EmitModuleFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn event_type(mut self) -> EventTypeFilterFieldPathBuilder {
            self.path.push(TransactionPredicate::EVENT_TYPE_FIELD.name);
            EventTypeFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn event_stream_head(mut self) -> EventStreamHeadFilterFieldPathBuilder {
            self.path.push(TransactionPredicate::EVENT_STREAM_HEAD_FIELD.name);
            EventStreamHeadFilterFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl EventFilter {
        pub const TERMS_FIELD: &'static MessageField = &MessageField {
            name: "terms",
            json_name: "terms",
            number: 1i32,
            message_fields: Some(EventTerm::FIELDS),
        };
    }
    impl MessageFields for EventFilter {
        const FIELDS: &'static [&'static MessageField] = &[Self::TERMS_FIELD];
    }
    impl EventFilter {
        pub fn path_builder() -> EventFilterFieldPathBuilder {
            EventFilterFieldPathBuilder::new()
        }
    }
    pub struct EventFilterFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EventFilterFieldPathBuilder {
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
        pub fn terms(mut self) -> EventTermFieldPathBuilder {
            self.path.push(EventFilter::TERMS_FIELD.name);
            EventTermFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl EventTerm {
        pub const LITERALS_FIELD: &'static MessageField = &MessageField {
            name: "literals",
            json_name: "literals",
            number: 1i32,
            message_fields: Some(EventLiteral::FIELDS),
        };
    }
    impl MessageFields for EventTerm {
        const FIELDS: &'static [&'static MessageField] = &[Self::LITERALS_FIELD];
    }
    impl EventTerm {
        pub fn path_builder() -> EventTermFieldPathBuilder {
            EventTermFieldPathBuilder::new()
        }
    }
    pub struct EventTermFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EventTermFieldPathBuilder {
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
        pub fn literals(mut self) -> EventLiteralFieldPathBuilder {
            self.path.push(EventTerm::LITERALS_FIELD.name);
            EventLiteralFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl EventLiteral {
        pub const INCLUDE_FIELD: &'static MessageField = &MessageField {
            name: "include",
            json_name: "include",
            number: 1i32,
            message_fields: Some(EventPredicate::FIELDS),
        };
        pub const EXCLUDE_FIELD: &'static MessageField = &MessageField {
            name: "exclude",
            json_name: "exclude",
            number: 2i32,
            message_fields: Some(EventPredicate::FIELDS),
        };
    }
    impl MessageFields for EventLiteral {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::INCLUDE_FIELD,
            Self::EXCLUDE_FIELD,
        ];
    }
    impl EventLiteral {
        pub fn path_builder() -> EventLiteralFieldPathBuilder {
            EventLiteralFieldPathBuilder::new()
        }
    }
    pub struct EventLiteralFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EventLiteralFieldPathBuilder {
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
        pub fn include(mut self) -> EventPredicateFieldPathBuilder {
            self.path.push(EventLiteral::INCLUDE_FIELD.name);
            EventPredicateFieldPathBuilder::new_with_base(self.path)
        }
        pub fn exclude(mut self) -> EventPredicateFieldPathBuilder {
            self.path.push(EventLiteral::EXCLUDE_FIELD.name);
            EventPredicateFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl EventPredicate {
        pub const SENDER_FIELD: &'static MessageField = &MessageField {
            name: "sender",
            json_name: "sender",
            number: 1i32,
            message_fields: Some(SenderFilter::FIELDS),
        };
        pub const AFFECTED_OBJECT_FIELD: &'static MessageField = &MessageField {
            name: "affected_object",
            json_name: "affectedObject",
            number: 2i32,
            message_fields: Some(AffectedObjectFilter::FIELDS),
        };
        pub const EMIT_MODULE_FIELD: &'static MessageField = &MessageField {
            name: "emit_module",
            json_name: "emitModule",
            number: 3i32,
            message_fields: Some(EmitModuleFilter::FIELDS),
        };
        pub const EVENT_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "event_type",
            json_name: "eventType",
            number: 4i32,
            message_fields: Some(EventTypeFilter::FIELDS),
        };
        pub const EVENT_STREAM_HEAD_FIELD: &'static MessageField = &MessageField {
            name: "event_stream_head",
            json_name: "eventStreamHead",
            number: 5i32,
            message_fields: Some(EventStreamHeadFilter::FIELDS),
        };
    }
    impl MessageFields for EventPredicate {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::SENDER_FIELD,
            Self::AFFECTED_OBJECT_FIELD,
            Self::EMIT_MODULE_FIELD,
            Self::EVENT_TYPE_FIELD,
            Self::EVENT_STREAM_HEAD_FIELD,
        ];
    }
    impl EventPredicate {
        pub fn path_builder() -> EventPredicateFieldPathBuilder {
            EventPredicateFieldPathBuilder::new()
        }
    }
    pub struct EventPredicateFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EventPredicateFieldPathBuilder {
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
        pub fn sender(mut self) -> SenderFilterFieldPathBuilder {
            self.path.push(EventPredicate::SENDER_FIELD.name);
            SenderFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn affected_object(mut self) -> AffectedObjectFilterFieldPathBuilder {
            self.path.push(EventPredicate::AFFECTED_OBJECT_FIELD.name);
            AffectedObjectFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn emit_module(mut self) -> EmitModuleFilterFieldPathBuilder {
            self.path.push(EventPredicate::EMIT_MODULE_FIELD.name);
            EmitModuleFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn event_type(mut self) -> EventTypeFilterFieldPathBuilder {
            self.path.push(EventPredicate::EVENT_TYPE_FIELD.name);
            EventTypeFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn event_stream_head(mut self) -> EventStreamHeadFilterFieldPathBuilder {
            self.path.push(EventPredicate::EVENT_STREAM_HEAD_FIELD.name);
            EventStreamHeadFilterFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl SenderFilter {
        pub const ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "address",
            json_name: "address",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for SenderFilter {
        const FIELDS: &'static [&'static MessageField] = &[Self::ADDRESS_FIELD];
    }
    impl SenderFilter {
        pub fn path_builder() -> SenderFilterFieldPathBuilder {
            SenderFilterFieldPathBuilder::new()
        }
    }
    pub struct SenderFilterFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SenderFilterFieldPathBuilder {
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
            self.path.push(SenderFilter::ADDRESS_FIELD.name);
            self.finish()
        }
    }
    impl AffectedAddressFilter {
        pub const ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "address",
            json_name: "address",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for AffectedAddressFilter {
        const FIELDS: &'static [&'static MessageField] = &[Self::ADDRESS_FIELD];
    }
    impl AffectedAddressFilter {
        pub fn path_builder() -> AffectedAddressFilterFieldPathBuilder {
            AffectedAddressFilterFieldPathBuilder::new()
        }
    }
    pub struct AffectedAddressFilterFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl AffectedAddressFilterFieldPathBuilder {
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
            self.path.push(AffectedAddressFilter::ADDRESS_FIELD.name);
            self.finish()
        }
    }
    impl AffectedObjectFilter {
        pub const OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "object_id",
            json_name: "objectId",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for AffectedObjectFilter {
        const FIELDS: &'static [&'static MessageField] = &[Self::OBJECT_ID_FIELD];
    }
    impl AffectedObjectFilter {
        pub fn path_builder() -> AffectedObjectFilterFieldPathBuilder {
            AffectedObjectFilterFieldPathBuilder::new()
        }
    }
    pub struct AffectedObjectFilterFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl AffectedObjectFilterFieldPathBuilder {
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
            self.path.push(AffectedObjectFilter::OBJECT_ID_FIELD.name);
            self.finish()
        }
    }
    impl MoveCallFilter {
        pub const FUNCTION_FIELD: &'static MessageField = &MessageField {
            name: "function",
            json_name: "function",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for MoveCallFilter {
        const FIELDS: &'static [&'static MessageField] = &[Self::FUNCTION_FIELD];
    }
    impl MoveCallFilter {
        pub fn path_builder() -> MoveCallFilterFieldPathBuilder {
            MoveCallFilterFieldPathBuilder::new()
        }
    }
    pub struct MoveCallFilterFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MoveCallFilterFieldPathBuilder {
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
        pub fn function(mut self) -> String {
            self.path.push(MoveCallFilter::FUNCTION_FIELD.name);
            self.finish()
        }
    }
    impl EmitModuleFilter {
        pub const MODULE_FIELD: &'static MessageField = &MessageField {
            name: "module",
            json_name: "module",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for EmitModuleFilter {
        const FIELDS: &'static [&'static MessageField] = &[Self::MODULE_FIELD];
    }
    impl EmitModuleFilter {
        pub fn path_builder() -> EmitModuleFilterFieldPathBuilder {
            EmitModuleFilterFieldPathBuilder::new()
        }
    }
    pub struct EmitModuleFilterFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EmitModuleFilterFieldPathBuilder {
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
        pub fn module(mut self) -> String {
            self.path.push(EmitModuleFilter::MODULE_FIELD.name);
            self.finish()
        }
    }
    impl EventTypeFilter {
        pub const TYPE_FIELD: &'static MessageField = &MessageField {
            name: "type",
            json_name: "type",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for EventTypeFilter {
        const FIELDS: &'static [&'static MessageField] = &[Self::TYPE_FIELD];
    }
    impl EventTypeFilter {
        pub fn path_builder() -> EventTypeFilterFieldPathBuilder {
            EventTypeFilterFieldPathBuilder::new()
        }
    }
    pub struct EventTypeFilterFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EventTypeFilterFieldPathBuilder {
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
            self.path.push(EventTypeFilter::TYPE_FIELD.name);
            self.finish()
        }
    }
    impl EventStreamHeadFilter {
        pub const STREAM_ID_FIELD: &'static MessageField = &MessageField {
            name: "stream_id",
            json_name: "streamId",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for EventStreamHeadFilter {
        const FIELDS: &'static [&'static MessageField] = &[Self::STREAM_ID_FIELD];
    }
    impl EventStreamHeadFilter {
        pub fn path_builder() -> EventStreamHeadFilterFieldPathBuilder {
            EventStreamHeadFilterFieldPathBuilder::new()
        }
    }
    pub struct EventStreamHeadFilterFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EventStreamHeadFilterFieldPathBuilder {
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
        pub fn stream_id(mut self) -> String {
            self.path.push(EventStreamHeadFilter::STREAM_ID_FIELD.name);
            self.finish()
        }
    }
    impl ListCheckpointsRequest {
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 1i32,
            message_fields: None,
        };
        pub const START_CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "start_checkpoint",
            json_name: "startCheckpoint",
            number: 2i32,
            message_fields: None,
        };
        pub const END_CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "end_checkpoint",
            json_name: "endCheckpoint",
            number: 3i32,
            message_fields: None,
        };
        pub const FILTER_FIELD: &'static MessageField = &MessageField {
            name: "filter",
            json_name: "filter",
            number: 4i32,
            message_fields: Some(TransactionFilter::FIELDS),
        };
        pub const PAGINATION_FIELD: &'static MessageField = &MessageField {
            name: "pagination",
            json_name: "pagination",
            number: 5i32,
            message_fields: Some(Pagination::FIELDS),
        };
    }
    impl MessageFields for ListCheckpointsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::READ_MASK_FIELD,
            Self::START_CHECKPOINT_FIELD,
            Self::END_CHECKPOINT_FIELD,
            Self::FILTER_FIELD,
            Self::PAGINATION_FIELD,
        ];
    }
    impl ListCheckpointsRequest {
        pub fn path_builder() -> ListCheckpointsRequestFieldPathBuilder {
            ListCheckpointsRequestFieldPathBuilder::new()
        }
    }
    pub struct ListCheckpointsRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListCheckpointsRequestFieldPathBuilder {
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
            self.path.push(ListCheckpointsRequest::READ_MASK_FIELD.name);
            self.finish()
        }
        pub fn start_checkpoint(mut self) -> String {
            self.path.push(ListCheckpointsRequest::START_CHECKPOINT_FIELD.name);
            self.finish()
        }
        pub fn end_checkpoint(mut self) -> String {
            self.path.push(ListCheckpointsRequest::END_CHECKPOINT_FIELD.name);
            self.finish()
        }
        pub fn filter(mut self) -> TransactionFilterFieldPathBuilder {
            self.path.push(ListCheckpointsRequest::FILTER_FIELD.name);
            TransactionFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn pagination(mut self) -> PaginationFieldPathBuilder {
            self.path.push(ListCheckpointsRequest::PAGINATION_FIELD.name);
            PaginationFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl CheckpointItem {
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
    impl MessageFields for CheckpointItem {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CURSOR_FIELD,
            Self::CHECKPOINT_FIELD,
        ];
    }
    impl CheckpointItem {
        pub fn path_builder() -> CheckpointItemFieldPathBuilder {
            CheckpointItemFieldPathBuilder::new()
        }
    }
    pub struct CheckpointItemFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl CheckpointItemFieldPathBuilder {
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
            self.path.push(CheckpointItem::CURSOR_FIELD.name);
            self.finish()
        }
        pub fn checkpoint(mut self) -> CheckpointFieldPathBuilder {
            self.path.push(CheckpointItem::CHECKPOINT_FIELD.name);
            CheckpointFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ListCheckpointsResponse {
        pub const ITEM_FIELD: &'static MessageField = &MessageField {
            name: "item",
            json_name: "item",
            number: 1i32,
            message_fields: Some(CheckpointItem::FIELDS),
        };
        pub const PAGE_INFO_FIELD: &'static MessageField = &MessageField {
            name: "page_info",
            json_name: "pageInfo",
            number: 2i32,
            message_fields: Some(PageInfo::FIELDS),
        };
    }
    impl MessageFields for ListCheckpointsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ITEM_FIELD,
            Self::PAGE_INFO_FIELD,
        ];
    }
    impl ListCheckpointsResponse {
        pub fn path_builder() -> ListCheckpointsResponseFieldPathBuilder {
            ListCheckpointsResponseFieldPathBuilder::new()
        }
    }
    pub struct ListCheckpointsResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListCheckpointsResponseFieldPathBuilder {
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
        pub fn item(mut self) -> CheckpointItemFieldPathBuilder {
            self.path.push(ListCheckpointsResponse::ITEM_FIELD.name);
            CheckpointItemFieldPathBuilder::new_with_base(self.path)
        }
        pub fn page_info(mut self) -> PageInfoFieldPathBuilder {
            self.path.push(ListCheckpointsResponse::PAGE_INFO_FIELD.name);
            PageInfoFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ListTransactionsRequest {
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 1i32,
            message_fields: None,
        };
        pub const START_CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "start_checkpoint",
            json_name: "startCheckpoint",
            number: 2i32,
            message_fields: None,
        };
        pub const END_CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "end_checkpoint",
            json_name: "endCheckpoint",
            number: 3i32,
            message_fields: None,
        };
        pub const FILTER_FIELD: &'static MessageField = &MessageField {
            name: "filter",
            json_name: "filter",
            number: 4i32,
            message_fields: Some(TransactionFilter::FIELDS),
        };
        pub const PAGINATION_FIELD: &'static MessageField = &MessageField {
            name: "pagination",
            json_name: "pagination",
            number: 5i32,
            message_fields: Some(Pagination::FIELDS),
        };
    }
    impl MessageFields for ListTransactionsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::READ_MASK_FIELD,
            Self::START_CHECKPOINT_FIELD,
            Self::END_CHECKPOINT_FIELD,
            Self::FILTER_FIELD,
            Self::PAGINATION_FIELD,
        ];
    }
    impl ListTransactionsRequest {
        pub fn path_builder() -> ListTransactionsRequestFieldPathBuilder {
            ListTransactionsRequestFieldPathBuilder::new()
        }
    }
    pub struct ListTransactionsRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListTransactionsRequestFieldPathBuilder {
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
            self.path.push(ListTransactionsRequest::READ_MASK_FIELD.name);
            self.finish()
        }
        pub fn start_checkpoint(mut self) -> String {
            self.path.push(ListTransactionsRequest::START_CHECKPOINT_FIELD.name);
            self.finish()
        }
        pub fn end_checkpoint(mut self) -> String {
            self.path.push(ListTransactionsRequest::END_CHECKPOINT_FIELD.name);
            self.finish()
        }
        pub fn filter(mut self) -> TransactionFilterFieldPathBuilder {
            self.path.push(ListTransactionsRequest::FILTER_FIELD.name);
            TransactionFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn pagination(mut self) -> PaginationFieldPathBuilder {
            self.path.push(ListTransactionsRequest::PAGINATION_FIELD.name);
            PaginationFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl TransactionItem {
        pub const CURSOR_FIELD: &'static MessageField = &MessageField {
            name: "cursor",
            json_name: "cursor",
            number: 1i32,
            message_fields: None,
        };
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 2i32,
            message_fields: Some(ExecutedTransaction::FIELDS),
        };
    }
    impl MessageFields for TransactionItem {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CURSOR_FIELD,
            Self::TRANSACTION_FIELD,
        ];
    }
    impl TransactionItem {
        pub fn path_builder() -> TransactionItemFieldPathBuilder {
            TransactionItemFieldPathBuilder::new()
        }
    }
    pub struct TransactionItemFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl TransactionItemFieldPathBuilder {
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
            self.path.push(TransactionItem::CURSOR_FIELD.name);
            self.finish()
        }
        pub fn transaction(mut self) -> ExecutedTransactionFieldPathBuilder {
            self.path.push(TransactionItem::TRANSACTION_FIELD.name);
            ExecutedTransactionFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ListTransactionsResponse {
        pub const ITEM_FIELD: &'static MessageField = &MessageField {
            name: "item",
            json_name: "item",
            number: 1i32,
            message_fields: Some(TransactionItem::FIELDS),
        };
        pub const PAGE_INFO_FIELD: &'static MessageField = &MessageField {
            name: "page_info",
            json_name: "pageInfo",
            number: 2i32,
            message_fields: Some(PageInfo::FIELDS),
        };
    }
    impl MessageFields for ListTransactionsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ITEM_FIELD,
            Self::PAGE_INFO_FIELD,
        ];
    }
    impl ListTransactionsResponse {
        pub fn path_builder() -> ListTransactionsResponseFieldPathBuilder {
            ListTransactionsResponseFieldPathBuilder::new()
        }
    }
    pub struct ListTransactionsResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListTransactionsResponseFieldPathBuilder {
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
        pub fn item(mut self) -> TransactionItemFieldPathBuilder {
            self.path.push(ListTransactionsResponse::ITEM_FIELD.name);
            TransactionItemFieldPathBuilder::new_with_base(self.path)
        }
        pub fn page_info(mut self) -> PageInfoFieldPathBuilder {
            self.path.push(ListTransactionsResponse::PAGE_INFO_FIELD.name);
            PageInfoFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ListEventsRequest {
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 1i32,
            message_fields: None,
        };
        pub const START_CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "start_checkpoint",
            json_name: "startCheckpoint",
            number: 2i32,
            message_fields: None,
        };
        pub const END_CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "end_checkpoint",
            json_name: "endCheckpoint",
            number: 3i32,
            message_fields: None,
        };
        pub const FILTER_FIELD: &'static MessageField = &MessageField {
            name: "filter",
            json_name: "filter",
            number: 4i32,
            message_fields: Some(EventFilter::FIELDS),
        };
        pub const PAGINATION_FIELD: &'static MessageField = &MessageField {
            name: "pagination",
            json_name: "pagination",
            number: 5i32,
            message_fields: Some(Pagination::FIELDS),
        };
    }
    impl MessageFields for ListEventsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::READ_MASK_FIELD,
            Self::START_CHECKPOINT_FIELD,
            Self::END_CHECKPOINT_FIELD,
            Self::FILTER_FIELD,
            Self::PAGINATION_FIELD,
        ];
    }
    impl ListEventsRequest {
        pub fn path_builder() -> ListEventsRequestFieldPathBuilder {
            ListEventsRequestFieldPathBuilder::new()
        }
    }
    pub struct ListEventsRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListEventsRequestFieldPathBuilder {
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
            self.path.push(ListEventsRequest::READ_MASK_FIELD.name);
            self.finish()
        }
        pub fn start_checkpoint(mut self) -> String {
            self.path.push(ListEventsRequest::START_CHECKPOINT_FIELD.name);
            self.finish()
        }
        pub fn end_checkpoint(mut self) -> String {
            self.path.push(ListEventsRequest::END_CHECKPOINT_FIELD.name);
            self.finish()
        }
        pub fn filter(mut self) -> EventFilterFieldPathBuilder {
            self.path.push(ListEventsRequest::FILTER_FIELD.name);
            EventFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn pagination(mut self) -> PaginationFieldPathBuilder {
            self.path.push(ListEventsRequest::PAGINATION_FIELD.name);
            PaginationFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl EventItem {
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
            message_fields: None,
        };
        pub const EVENT_INDEX_FIELD: &'static MessageField = &MessageField {
            name: "event_index",
            json_name: "eventIndex",
            number: 3i32,
            message_fields: None,
        };
        pub const TRANSACTION_DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "transaction_digest",
            json_name: "transactionDigest",
            number: 4i32,
            message_fields: None,
        };
        pub const EVENT_FIELD: &'static MessageField = &MessageField {
            name: "event",
            json_name: "event",
            number: 5i32,
            message_fields: Some(Event::FIELDS),
        };
    }
    impl MessageFields for EventItem {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CURSOR_FIELD,
            Self::CHECKPOINT_FIELD,
            Self::EVENT_INDEX_FIELD,
            Self::TRANSACTION_DIGEST_FIELD,
            Self::EVENT_FIELD,
        ];
    }
    impl EventItem {
        pub fn path_builder() -> EventItemFieldPathBuilder {
            EventItemFieldPathBuilder::new()
        }
    }
    pub struct EventItemFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl EventItemFieldPathBuilder {
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
            self.path.push(EventItem::CURSOR_FIELD.name);
            self.finish()
        }
        pub fn checkpoint(mut self) -> String {
            self.path.push(EventItem::CHECKPOINT_FIELD.name);
            self.finish()
        }
        pub fn event_index(mut self) -> String {
            self.path.push(EventItem::EVENT_INDEX_FIELD.name);
            self.finish()
        }
        pub fn transaction_digest(mut self) -> String {
            self.path.push(EventItem::TRANSACTION_DIGEST_FIELD.name);
            self.finish()
        }
        pub fn event(mut self) -> EventFieldPathBuilder {
            self.path.push(EventItem::EVENT_FIELD.name);
            EventFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ListEventsResponse {
        pub const ITEM_FIELD: &'static MessageField = &MessageField {
            name: "item",
            json_name: "item",
            number: 1i32,
            message_fields: Some(EventItem::FIELDS),
        };
        pub const PAGE_INFO_FIELD: &'static MessageField = &MessageField {
            name: "page_info",
            json_name: "pageInfo",
            number: 2i32,
            message_fields: Some(PageInfo::FIELDS),
        };
    }
    impl MessageFields for ListEventsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ITEM_FIELD,
            Self::PAGE_INFO_FIELD,
        ];
    }
    impl ListEventsResponse {
        pub fn path_builder() -> ListEventsResponseFieldPathBuilder {
            ListEventsResponseFieldPathBuilder::new()
        }
    }
    pub struct ListEventsResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl ListEventsResponseFieldPathBuilder {
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
        pub fn item(mut self) -> EventItemFieldPathBuilder {
            self.path.push(ListEventsResponse::ITEM_FIELD.name);
            EventItemFieldPathBuilder::new_with_base(self.path)
        }
        pub fn page_info(mut self) -> PageInfoFieldPathBuilder {
            self.path.push(ListEventsResponse::PAGE_INFO_FIELD.name);
            PageInfoFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl Pagination {
        pub const PAGE_SIZE_FIELD: &'static MessageField = &MessageField {
            name: "page_size",
            json_name: "pageSize",
            number: 1i32,
            message_fields: None,
        };
        pub const CURSOR_FIELD: &'static MessageField = &MessageField {
            name: "cursor",
            json_name: "cursor",
            number: 2i32,
            message_fields: None,
        };
        pub const ORDERING_FIELD: &'static MessageField = &MessageField {
            name: "ordering",
            json_name: "ordering",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for Pagination {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::PAGE_SIZE_FIELD,
            Self::CURSOR_FIELD,
            Self::ORDERING_FIELD,
        ];
    }
    impl Pagination {
        pub fn path_builder() -> PaginationFieldPathBuilder {
            PaginationFieldPathBuilder::new()
        }
    }
    pub struct PaginationFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl PaginationFieldPathBuilder {
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
        pub fn page_size(mut self) -> String {
            self.path.push(Pagination::PAGE_SIZE_FIELD.name);
            self.finish()
        }
        pub fn cursor(mut self) -> String {
            self.path.push(Pagination::CURSOR_FIELD.name);
            self.finish()
        }
        pub fn ordering(mut self) -> String {
            self.path.push(Pagination::ORDERING_FIELD.name);
            self.finish()
        }
    }
    impl PageInfo {
        pub const NEXT_CURSOR_FIELD: &'static MessageField = &MessageField {
            name: "next_cursor",
            json_name: "nextCursor",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for PageInfo {
        const FIELDS: &'static [&'static MessageField] = &[Self::NEXT_CURSOR_FIELD];
    }
    impl PageInfo {
        pub fn path_builder() -> PageInfoFieldPathBuilder {
            PageInfoFieldPathBuilder::new()
        }
    }
    pub struct PageInfoFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl PageInfoFieldPathBuilder {
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
        pub fn next_cursor(mut self) -> String {
            self.path.push(PageInfo::NEXT_CURSOR_FIELD.name);
            self.finish()
        }
    }
}
