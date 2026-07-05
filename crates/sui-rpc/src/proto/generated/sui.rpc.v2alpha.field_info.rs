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
        pub const NEGATED_FIELD: &'static MessageField = &MessageField {
            name: "negated",
            json_name: "negated",
            number: 1i32,
            message_fields: None,
        };
        pub const SENDER_FIELD: &'static MessageField = &MessageField {
            name: "sender",
            json_name: "sender",
            number: 2i32,
            message_fields: Some(SenderFilter::FIELDS),
        };
        pub const AFFECTED_ADDRESS_FIELD: &'static MessageField = &MessageField {
            name: "affected_address",
            json_name: "affectedAddress",
            number: 3i32,
            message_fields: Some(AffectedAddressFilter::FIELDS),
        };
        pub const AFFECTED_OBJECT_FIELD: &'static MessageField = &MessageField {
            name: "affected_object",
            json_name: "affectedObject",
            number: 4i32,
            message_fields: Some(AffectedObjectFilter::FIELDS),
        };
        pub const MOVE_CALL_FIELD: &'static MessageField = &MessageField {
            name: "move_call",
            json_name: "moveCall",
            number: 5i32,
            message_fields: Some(MoveCallFilter::FIELDS),
        };
        pub const EMIT_MODULE_FIELD: &'static MessageField = &MessageField {
            name: "emit_module",
            json_name: "emitModule",
            number: 6i32,
            message_fields: Some(EmitModuleFilter::FIELDS),
        };
        pub const EVENT_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "event_type",
            json_name: "eventType",
            number: 7i32,
            message_fields: Some(EventTypeFilter::FIELDS),
        };
        pub const EVENT_STREAM_HEAD_FIELD: &'static MessageField = &MessageField {
            name: "event_stream_head",
            json_name: "eventStreamHead",
            number: 8i32,
            message_fields: Some(EventStreamHeadFilter::FIELDS),
        };
        pub const PACKAGE_WRITE_FIELD: &'static MessageField = &MessageField {
            name: "package_write",
            json_name: "packageWrite",
            number: 9i32,
            message_fields: Some(PackageWriteFilter::FIELDS),
        };
    }
    impl MessageFields for TransactionLiteral {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::NEGATED_FIELD,
            Self::SENDER_FIELD,
            Self::AFFECTED_ADDRESS_FIELD,
            Self::AFFECTED_OBJECT_FIELD,
            Self::MOVE_CALL_FIELD,
            Self::EMIT_MODULE_FIELD,
            Self::EVENT_TYPE_FIELD,
            Self::EVENT_STREAM_HEAD_FIELD,
            Self::PACKAGE_WRITE_FIELD,
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
        pub fn negated(mut self) -> String {
            self.path.push(TransactionLiteral::NEGATED_FIELD.name);
            self.finish()
        }
        pub fn sender(mut self) -> SenderFilterFieldPathBuilder {
            self.path.push(TransactionLiteral::SENDER_FIELD.name);
            SenderFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn affected_address(mut self) -> AffectedAddressFilterFieldPathBuilder {
            self.path.push(TransactionLiteral::AFFECTED_ADDRESS_FIELD.name);
            AffectedAddressFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn affected_object(mut self) -> AffectedObjectFilterFieldPathBuilder {
            self.path.push(TransactionLiteral::AFFECTED_OBJECT_FIELD.name);
            AffectedObjectFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn move_call(mut self) -> MoveCallFilterFieldPathBuilder {
            self.path.push(TransactionLiteral::MOVE_CALL_FIELD.name);
            MoveCallFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn emit_module(mut self) -> EmitModuleFilterFieldPathBuilder {
            self.path.push(TransactionLiteral::EMIT_MODULE_FIELD.name);
            EmitModuleFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn event_type(mut self) -> EventTypeFilterFieldPathBuilder {
            self.path.push(TransactionLiteral::EVENT_TYPE_FIELD.name);
            EventTypeFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn event_stream_head(mut self) -> EventStreamHeadFilterFieldPathBuilder {
            self.path.push(TransactionLiteral::EVENT_STREAM_HEAD_FIELD.name);
            EventStreamHeadFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn package_write(mut self) -> PackageWriteFilterFieldPathBuilder {
            self.path.push(TransactionLiteral::PACKAGE_WRITE_FIELD.name);
            PackageWriteFilterFieldPathBuilder::new_with_base(self.path)
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
        pub const NEGATED_FIELD: &'static MessageField = &MessageField {
            name: "negated",
            json_name: "negated",
            number: 1i32,
            message_fields: None,
        };
        pub const SENDER_FIELD: &'static MessageField = &MessageField {
            name: "sender",
            json_name: "sender",
            number: 2i32,
            message_fields: Some(SenderFilter::FIELDS),
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
    impl MessageFields for EventLiteral {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::NEGATED_FIELD,
            Self::SENDER_FIELD,
            Self::EMIT_MODULE_FIELD,
            Self::EVENT_TYPE_FIELD,
            Self::EVENT_STREAM_HEAD_FIELD,
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
        pub fn negated(mut self) -> String {
            self.path.push(EventLiteral::NEGATED_FIELD.name);
            self.finish()
        }
        pub fn sender(mut self) -> SenderFilterFieldPathBuilder {
            self.path.push(EventLiteral::SENDER_FIELD.name);
            SenderFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn emit_module(mut self) -> EmitModuleFilterFieldPathBuilder {
            self.path.push(EventLiteral::EMIT_MODULE_FIELD.name);
            EmitModuleFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn event_type(mut self) -> EventTypeFilterFieldPathBuilder {
            self.path.push(EventLiteral::EVENT_TYPE_FIELD.name);
            EventTypeFilterFieldPathBuilder::new_with_base(self.path)
        }
        pub fn event_stream_head(mut self) -> EventStreamHeadFilterFieldPathBuilder {
            self.path.push(EventLiteral::EVENT_STREAM_HEAD_FIELD.name);
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
        pub const EVENT_TYPE_FIELD: &'static MessageField = &MessageField {
            name: "event_type",
            json_name: "eventType",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for EventTypeFilter {
        const FIELDS: &'static [&'static MessageField] = &[Self::EVENT_TYPE_FIELD];
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
        pub fn event_type(mut self) -> String {
            self.path.push(EventTypeFilter::EVENT_TYPE_FIELD.name);
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
    impl PackageWriteFilter {}
    impl MessageFields for PackageWriteFilter {
        const FIELDS: &'static [&'static MessageField] = &[];
    }
    impl PackageWriteFilter {
        pub fn path_builder() -> PackageWriteFilterFieldPathBuilder {
            PackageWriteFilterFieldPathBuilder::new()
        }
    }
    pub struct PackageWriteFilterFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl PackageWriteFilterFieldPathBuilder {
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
        pub const OPTIONS_FIELD: &'static MessageField = &MessageField {
            name: "options",
            json_name: "options",
            number: 5i32,
            message_fields: Some(QueryOptions::FIELDS),
        };
    }
    impl MessageFields for ListCheckpointsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::READ_MASK_FIELD,
            Self::START_CHECKPOINT_FIELD,
            Self::END_CHECKPOINT_FIELD,
            Self::FILTER_FIELD,
            Self::OPTIONS_FIELD,
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
        pub fn options(mut self) -> QueryOptionsFieldPathBuilder {
            self.path.push(ListCheckpointsRequest::OPTIONS_FIELD.name);
            QueryOptionsFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl CheckpointItem {
        pub const CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "checkpoint",
            json_name: "checkpoint",
            number: 1i32,
            message_fields: Some(Checkpoint::FIELDS),
        };
        pub const WATERMARK_FIELD: &'static MessageField = &MessageField {
            name: "watermark",
            json_name: "watermark",
            number: 2i32,
            message_fields: Some(Watermark::FIELDS),
        };
    }
    impl MessageFields for CheckpointItem {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CHECKPOINT_FIELD,
            Self::WATERMARK_FIELD,
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
        pub fn checkpoint(mut self) -> CheckpointFieldPathBuilder {
            self.path.push(CheckpointItem::CHECKPOINT_FIELD.name);
            CheckpointFieldPathBuilder::new_with_base(self.path)
        }
        pub fn watermark(mut self) -> WatermarkFieldPathBuilder {
            self.path.push(CheckpointItem::WATERMARK_FIELD.name);
            WatermarkFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ListCheckpointsResponse {
        pub const ITEM_FIELD: &'static MessageField = &MessageField {
            name: "item",
            json_name: "item",
            number: 1i32,
            message_fields: Some(CheckpointItem::FIELDS),
        };
        pub const WATERMARK_FIELD: &'static MessageField = &MessageField {
            name: "watermark",
            json_name: "watermark",
            number: 2i32,
            message_fields: Some(Watermark::FIELDS),
        };
        pub const END_FIELD: &'static MessageField = &MessageField {
            name: "end",
            json_name: "end",
            number: 3i32,
            message_fields: Some(QueryEnd::FIELDS),
        };
    }
    impl MessageFields for ListCheckpointsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ITEM_FIELD,
            Self::WATERMARK_FIELD,
            Self::END_FIELD,
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
        pub fn watermark(mut self) -> WatermarkFieldPathBuilder {
            self.path.push(ListCheckpointsResponse::WATERMARK_FIELD.name);
            WatermarkFieldPathBuilder::new_with_base(self.path)
        }
        pub fn end(mut self) -> QueryEndFieldPathBuilder {
            self.path.push(ListCheckpointsResponse::END_FIELD.name);
            QueryEndFieldPathBuilder::new_with_base(self.path)
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
        pub const OPTIONS_FIELD: &'static MessageField = &MessageField {
            name: "options",
            json_name: "options",
            number: 5i32,
            message_fields: Some(QueryOptions::FIELDS),
        };
    }
    impl MessageFields for ListTransactionsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::READ_MASK_FIELD,
            Self::START_CHECKPOINT_FIELD,
            Self::END_CHECKPOINT_FIELD,
            Self::FILTER_FIELD,
            Self::OPTIONS_FIELD,
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
        pub fn options(mut self) -> QueryOptionsFieldPathBuilder {
            self.path.push(ListTransactionsRequest::OPTIONS_FIELD.name);
            QueryOptionsFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl TransactionItem {
        pub const TRANSACTION_FIELD: &'static MessageField = &MessageField {
            name: "transaction",
            json_name: "transaction",
            number: 1i32,
            message_fields: Some(ExecutedTransaction::FIELDS),
        };
        pub const WATERMARK_FIELD: &'static MessageField = &MessageField {
            name: "watermark",
            json_name: "watermark",
            number: 2i32,
            message_fields: Some(Watermark::FIELDS),
        };
    }
    impl MessageFields for TransactionItem {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::TRANSACTION_FIELD,
            Self::WATERMARK_FIELD,
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
        pub fn transaction(mut self) -> ExecutedTransactionFieldPathBuilder {
            self.path.push(TransactionItem::TRANSACTION_FIELD.name);
            ExecutedTransactionFieldPathBuilder::new_with_base(self.path)
        }
        pub fn watermark(mut self) -> WatermarkFieldPathBuilder {
            self.path.push(TransactionItem::WATERMARK_FIELD.name);
            WatermarkFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ListTransactionsResponse {
        pub const ITEM_FIELD: &'static MessageField = &MessageField {
            name: "item",
            json_name: "item",
            number: 1i32,
            message_fields: Some(TransactionItem::FIELDS),
        };
        pub const WATERMARK_FIELD: &'static MessageField = &MessageField {
            name: "watermark",
            json_name: "watermark",
            number: 2i32,
            message_fields: Some(Watermark::FIELDS),
        };
        pub const END_FIELD: &'static MessageField = &MessageField {
            name: "end",
            json_name: "end",
            number: 3i32,
            message_fields: Some(QueryEnd::FIELDS),
        };
    }
    impl MessageFields for ListTransactionsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ITEM_FIELD,
            Self::WATERMARK_FIELD,
            Self::END_FIELD,
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
        pub fn watermark(mut self) -> WatermarkFieldPathBuilder {
            self.path.push(ListTransactionsResponse::WATERMARK_FIELD.name);
            WatermarkFieldPathBuilder::new_with_base(self.path)
        }
        pub fn end(mut self) -> QueryEndFieldPathBuilder {
            self.path.push(ListTransactionsResponse::END_FIELD.name);
            QueryEndFieldPathBuilder::new_with_base(self.path)
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
        pub const OPTIONS_FIELD: &'static MessageField = &MessageField {
            name: "options",
            json_name: "options",
            number: 5i32,
            message_fields: Some(QueryOptions::FIELDS),
        };
    }
    impl MessageFields for ListEventsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::READ_MASK_FIELD,
            Self::START_CHECKPOINT_FIELD,
            Self::END_CHECKPOINT_FIELD,
            Self::FILTER_FIELD,
            Self::OPTIONS_FIELD,
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
        pub fn options(mut self) -> QueryOptionsFieldPathBuilder {
            self.path.push(ListEventsRequest::OPTIONS_FIELD.name);
            QueryOptionsFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl EventItem {
        pub const EVENT_FIELD: &'static MessageField = &MessageField {
            name: "event",
            json_name: "event",
            number: 1i32,
            message_fields: Some(Event::FIELDS),
        };
        pub const WATERMARK_FIELD: &'static MessageField = &MessageField {
            name: "watermark",
            json_name: "watermark",
            number: 2i32,
            message_fields: Some(Watermark::FIELDS),
        };
    }
    impl MessageFields for EventItem {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EVENT_FIELD,
            Self::WATERMARK_FIELD,
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
        pub fn event(mut self) -> EventFieldPathBuilder {
            self.path.push(EventItem::EVENT_FIELD.name);
            EventFieldPathBuilder::new_with_base(self.path)
        }
        pub fn watermark(mut self) -> WatermarkFieldPathBuilder {
            self.path.push(EventItem::WATERMARK_FIELD.name);
            WatermarkFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl ListEventsResponse {
        pub const ITEM_FIELD: &'static MessageField = &MessageField {
            name: "item",
            json_name: "item",
            number: 1i32,
            message_fields: Some(EventItem::FIELDS),
        };
        pub const WATERMARK_FIELD: &'static MessageField = &MessageField {
            name: "watermark",
            json_name: "watermark",
            number: 2i32,
            message_fields: Some(Watermark::FIELDS),
        };
        pub const END_FIELD: &'static MessageField = &MessageField {
            name: "end",
            json_name: "end",
            number: 3i32,
            message_fields: Some(QueryEnd::FIELDS),
        };
    }
    impl MessageFields for ListEventsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ITEM_FIELD,
            Self::WATERMARK_FIELD,
            Self::END_FIELD,
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
        pub fn watermark(mut self) -> WatermarkFieldPathBuilder {
            self.path.push(ListEventsResponse::WATERMARK_FIELD.name);
            WatermarkFieldPathBuilder::new_with_base(self.path)
        }
        pub fn end(mut self) -> QueryEndFieldPathBuilder {
            self.path.push(ListEventsResponse::END_FIELD.name);
            QueryEndFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MerkleNode {
        pub const EMPTY_FIELD: &'static MessageField = &MessageField {
            name: "empty",
            json_name: "empty",
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
    impl MessageFields for MerkleNode {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EMPTY_FIELD,
            Self::DIGEST_FIELD,
        ];
    }
    impl MerkleNode {
        pub fn path_builder() -> MerkleNodeFieldPathBuilder {
            MerkleNodeFieldPathBuilder::new()
        }
    }
    pub struct MerkleNodeFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MerkleNodeFieldPathBuilder {
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
        pub fn empty(mut self) -> String {
            self.path.push(MerkleNode::EMPTY_FIELD.name);
            self.finish()
        }
        pub fn digest(mut self) -> String {
            self.path.push(MerkleNode::DIGEST_FIELD.name);
            self.finish()
        }
    }
    impl MerkleProof {
        pub const PATH_FIELD: &'static MessageField = &MessageField {
            name: "path",
            json_name: "path",
            number: 1i32,
            message_fields: Some(MerkleNode::FIELDS),
        };
    }
    impl MessageFields for MerkleProof {
        const FIELDS: &'static [&'static MessageField] = &[Self::PATH_FIELD];
    }
    impl MerkleProof {
        pub fn path_builder() -> MerkleProofFieldPathBuilder {
            MerkleProofFieldPathBuilder::new()
        }
    }
    pub struct MerkleProofFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MerkleProofFieldPathBuilder {
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
        pub fn path(mut self) -> MerkleNodeFieldPathBuilder {
            self.path.push(MerkleProof::PATH_FIELD.name);
            MerkleNodeFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MerkleNonInclusionProof {
        pub const INDEX_FIELD: &'static MessageField = &MessageField {
            name: "index",
            json_name: "index",
            number: 1i32,
            message_fields: None,
        };
        pub const LEFT_LEAF_FIELD: &'static MessageField = &MessageField {
            name: "left_leaf",
            json_name: "leftLeaf",
            number: 2i32,
            message_fields: Some(MerkleNeighbourLeaf::FIELDS),
        };
        pub const RIGHT_LEAF_FIELD: &'static MessageField = &MessageField {
            name: "right_leaf",
            json_name: "rightLeaf",
            number: 3i32,
            message_fields: Some(MerkleNeighbourLeaf::FIELDS),
        };
    }
    impl MessageFields for MerkleNonInclusionProof {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::INDEX_FIELD,
            Self::LEFT_LEAF_FIELD,
            Self::RIGHT_LEAF_FIELD,
        ];
    }
    impl MerkleNonInclusionProof {
        pub fn path_builder() -> MerkleNonInclusionProofFieldPathBuilder {
            MerkleNonInclusionProofFieldPathBuilder::new()
        }
    }
    pub struct MerkleNonInclusionProofFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MerkleNonInclusionProofFieldPathBuilder {
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
            self.path.push(MerkleNonInclusionProof::INDEX_FIELD.name);
            self.finish()
        }
        pub fn left_leaf(mut self) -> MerkleNeighbourLeafFieldPathBuilder {
            self.path.push(MerkleNonInclusionProof::LEFT_LEAF_FIELD.name);
            MerkleNeighbourLeafFieldPathBuilder::new_with_base(self.path)
        }
        pub fn right_leaf(mut self) -> MerkleNeighbourLeafFieldPathBuilder {
            self.path.push(MerkleNonInclusionProof::RIGHT_LEAF_FIELD.name);
            MerkleNeighbourLeafFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MerkleNeighbourLeaf {
        pub const LEAF_FIELD: &'static MessageField = &MessageField {
            name: "leaf",
            json_name: "leaf",
            number: 1i32,
            message_fields: Some(ObjectReference::FIELDS),
        };
        pub const MERKLE_PROOF_FIELD: &'static MessageField = &MessageField {
            name: "merkle_proof",
            json_name: "merkleProof",
            number: 2i32,
            message_fields: Some(MerkleProof::FIELDS),
        };
    }
    impl MessageFields for MerkleNeighbourLeaf {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::LEAF_FIELD,
            Self::MERKLE_PROOF_FIELD,
        ];
    }
    impl MerkleNeighbourLeaf {
        pub fn path_builder() -> MerkleNeighbourLeafFieldPathBuilder {
            MerkleNeighbourLeafFieldPathBuilder::new()
        }
    }
    pub struct MerkleNeighbourLeafFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MerkleNeighbourLeafFieldPathBuilder {
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
        pub fn leaf(mut self) -> ObjectReferenceFieldPathBuilder {
            self.path.push(MerkleNeighbourLeaf::LEAF_FIELD.name);
            ObjectReferenceFieldPathBuilder::new_with_base(self.path)
        }
        pub fn merkle_proof(mut self) -> MerkleProofFieldPathBuilder {
            self.path.push(MerkleNeighbourLeaf::MERKLE_PROOF_FIELD.name);
            MerkleProofFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl OcsInclusionProof {
        pub const OBJECT_REF_FIELD: &'static MessageField = &MessageField {
            name: "object_ref",
            json_name: "objectRef",
            number: 1i32,
            message_fields: Some(ObjectReference::FIELDS),
        };
        pub const MERKLE_PROOF_FIELD: &'static MessageField = &MessageField {
            name: "merkle_proof",
            json_name: "merkleProof",
            number: 2i32,
            message_fields: Some(MerkleProof::FIELDS),
        };
        pub const LEAF_INDEX_FIELD: &'static MessageField = &MessageField {
            name: "leaf_index",
            json_name: "leafIndex",
            number: 3i32,
            message_fields: None,
        };
        pub const TREE_ROOT_FIELD: &'static MessageField = &MessageField {
            name: "tree_root",
            json_name: "treeRoot",
            number: 4i32,
            message_fields: None,
        };
        pub const OBJECT_DATA_FIELD: &'static MessageField = &MessageField {
            name: "object_data",
            json_name: "objectData",
            number: 5i32,
            message_fields: None,
        };
    }
    impl MessageFields for OcsInclusionProof {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECT_REF_FIELD,
            Self::MERKLE_PROOF_FIELD,
            Self::LEAF_INDEX_FIELD,
            Self::TREE_ROOT_FIELD,
            Self::OBJECT_DATA_FIELD,
        ];
    }
    impl OcsInclusionProof {
        pub fn path_builder() -> OcsInclusionProofFieldPathBuilder {
            OcsInclusionProofFieldPathBuilder::new()
        }
    }
    pub struct OcsInclusionProofFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl OcsInclusionProofFieldPathBuilder {
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
        pub fn object_ref(mut self) -> ObjectReferenceFieldPathBuilder {
            self.path.push(OcsInclusionProof::OBJECT_REF_FIELD.name);
            ObjectReferenceFieldPathBuilder::new_with_base(self.path)
        }
        pub fn merkle_proof(mut self) -> MerkleProofFieldPathBuilder {
            self.path.push(OcsInclusionProof::MERKLE_PROOF_FIELD.name);
            MerkleProofFieldPathBuilder::new_with_base(self.path)
        }
        pub fn leaf_index(mut self) -> String {
            self.path.push(OcsInclusionProof::LEAF_INDEX_FIELD.name);
            self.finish()
        }
        pub fn tree_root(mut self) -> String {
            self.path.push(OcsInclusionProof::TREE_ROOT_FIELD.name);
            self.finish()
        }
        pub fn object_data(mut self) -> String {
            self.path.push(OcsInclusionProof::OBJECT_DATA_FIELD.name);
            self.finish()
        }
    }
    impl OcsNonInclusionProof {
        pub const NON_INCLUSION_PROOF_FIELD: &'static MessageField = &MessageField {
            name: "non_inclusion_proof",
            json_name: "nonInclusionProof",
            number: 1i32,
            message_fields: Some(MerkleNonInclusionProof::FIELDS),
        };
        pub const TREE_ROOT_FIELD: &'static MessageField = &MessageField {
            name: "tree_root",
            json_name: "treeRoot",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for OcsNonInclusionProof {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::NON_INCLUSION_PROOF_FIELD,
            Self::TREE_ROOT_FIELD,
        ];
    }
    impl OcsNonInclusionProof {
        pub fn path_builder() -> OcsNonInclusionProofFieldPathBuilder {
            OcsNonInclusionProofFieldPathBuilder::new()
        }
    }
    pub struct OcsNonInclusionProofFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl OcsNonInclusionProofFieldPathBuilder {
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
        pub fn non_inclusion_proof(mut self) -> MerkleNonInclusionProofFieldPathBuilder {
            self.path.push(OcsNonInclusionProof::NON_INCLUSION_PROOF_FIELD.name);
            MerkleNonInclusionProofFieldPathBuilder::new_with_base(self.path)
        }
        pub fn tree_root(mut self) -> String {
            self.path.push(OcsNonInclusionProof::TREE_ROOT_FIELD.name);
            self.finish()
        }
    }
    impl GetCheckpointObjectProofRequest {
        pub const OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "object_id",
            json_name: "objectId",
            number: 1i32,
            message_fields: None,
        };
        pub const CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "checkpoint",
            json_name: "checkpoint",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetCheckpointObjectProofRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECT_ID_FIELD,
            Self::CHECKPOINT_FIELD,
        ];
    }
    impl GetCheckpointObjectProofRequest {
        pub fn path_builder() -> GetCheckpointObjectProofRequestFieldPathBuilder {
            GetCheckpointObjectProofRequestFieldPathBuilder::new()
        }
    }
    pub struct GetCheckpointObjectProofRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetCheckpointObjectProofRequestFieldPathBuilder {
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
            self.path.push(GetCheckpointObjectProofRequest::OBJECT_ID_FIELD.name);
            self.finish()
        }
        pub fn checkpoint(mut self) -> String {
            self.path.push(GetCheckpointObjectProofRequest::CHECKPOINT_FIELD.name);
            self.finish()
        }
    }
    impl GetCheckpointObjectProofResponse {
        pub const CHECKPOINT_SUMMARY_FIELD: &'static MessageField = &MessageField {
            name: "checkpoint_summary",
            json_name: "checkpointSummary",
            number: 1i32,
            message_fields: None,
        };
        pub const INCLUSION_FIELD: &'static MessageField = &MessageField {
            name: "inclusion",
            json_name: "inclusion",
            number: 2i32,
            message_fields: Some(OcsInclusionProof::FIELDS),
        };
        pub const NON_INCLUSION_FIELD: &'static MessageField = &MessageField {
            name: "non_inclusion",
            json_name: "nonInclusion",
            number: 3i32,
            message_fields: Some(OcsNonInclusionProof::FIELDS),
        };
    }
    impl MessageFields for GetCheckpointObjectProofResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CHECKPOINT_SUMMARY_FIELD,
            Self::INCLUSION_FIELD,
            Self::NON_INCLUSION_FIELD,
        ];
    }
    impl GetCheckpointObjectProofResponse {
        pub fn path_builder() -> GetCheckpointObjectProofResponseFieldPathBuilder {
            GetCheckpointObjectProofResponseFieldPathBuilder::new()
        }
    }
    pub struct GetCheckpointObjectProofResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetCheckpointObjectProofResponseFieldPathBuilder {
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
        pub fn checkpoint_summary(mut self) -> String {
            self.path
                .push(GetCheckpointObjectProofResponse::CHECKPOINT_SUMMARY_FIELD.name);
            self.finish()
        }
        pub fn inclusion(mut self) -> OcsInclusionProofFieldPathBuilder {
            self.path.push(GetCheckpointObjectProofResponse::INCLUSION_FIELD.name);
            OcsInclusionProofFieldPathBuilder::new_with_base(self.path)
        }
        pub fn non_inclusion(mut self) -> OcsNonInclusionProofFieldPathBuilder {
            self.path.push(GetCheckpointObjectProofResponse::NON_INCLUSION_FIELD.name);
            OcsNonInclusionProofFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl QueryOptions {
        pub const LIMIT_FIELD: &'static MessageField = &MessageField {
            name: "limit",
            json_name: "limit",
            number: 1i32,
            message_fields: None,
        };
        pub const AFTER_FIELD: &'static MessageField = &MessageField {
            name: "after",
            json_name: "after",
            number: 2i32,
            message_fields: None,
        };
        pub const BEFORE_FIELD: &'static MessageField = &MessageField {
            name: "before",
            json_name: "before",
            number: 3i32,
            message_fields: None,
        };
        pub const ORDERING_FIELD: &'static MessageField = &MessageField {
            name: "ordering",
            json_name: "ordering",
            number: 4i32,
            message_fields: None,
        };
    }
    impl MessageFields for QueryOptions {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::LIMIT_FIELD,
            Self::AFTER_FIELD,
            Self::BEFORE_FIELD,
            Self::ORDERING_FIELD,
        ];
    }
    impl QueryOptions {
        pub fn path_builder() -> QueryOptionsFieldPathBuilder {
            QueryOptionsFieldPathBuilder::new()
        }
    }
    pub struct QueryOptionsFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl QueryOptionsFieldPathBuilder {
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
        pub fn limit(mut self) -> String {
            self.path.push(QueryOptions::LIMIT_FIELD.name);
            self.finish()
        }
        pub fn after(mut self) -> String {
            self.path.push(QueryOptions::AFTER_FIELD.name);
            self.finish()
        }
        pub fn before(mut self) -> String {
            self.path.push(QueryOptions::BEFORE_FIELD.name);
            self.finish()
        }
        pub fn ordering(mut self) -> String {
            self.path.push(QueryOptions::ORDERING_FIELD.name);
            self.finish()
        }
    }
    impl Watermark {
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
    }
    impl MessageFields for Watermark {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CURSOR_FIELD,
            Self::CHECKPOINT_FIELD,
        ];
    }
    impl Watermark {
        pub fn path_builder() -> WatermarkFieldPathBuilder {
            WatermarkFieldPathBuilder::new()
        }
    }
    pub struct WatermarkFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl WatermarkFieldPathBuilder {
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
            self.path.push(Watermark::CURSOR_FIELD.name);
            self.finish()
        }
        pub fn checkpoint(mut self) -> String {
            self.path.push(Watermark::CHECKPOINT_FIELD.name);
            self.finish()
        }
    }
    impl QueryEnd {
        pub const REASON_FIELD: &'static MessageField = &MessageField {
            name: "reason",
            json_name: "reason",
            number: 1i32,
            message_fields: None,
        };
    }
    impl MessageFields for QueryEnd {
        const FIELDS: &'static [&'static MessageField] = &[Self::REASON_FIELD];
    }
    impl QueryEnd {
        pub fn path_builder() -> QueryEndFieldPathBuilder {
            QueryEndFieldPathBuilder::new()
        }
    }
    pub struct QueryEndFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl QueryEndFieldPathBuilder {
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
        pub fn reason(mut self) -> String {
            self.path.push(QueryEnd::REASON_FIELD.name);
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
        pub const FILTER_FIELD: &'static MessageField = &MessageField {
            name: "filter",
            json_name: "filter",
            number: 2i32,
            message_fields: Some(TransactionFilter::FIELDS),
        };
    }
    impl MessageFields for SubscribeCheckpointsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::READ_MASK_FIELD,
            Self::FILTER_FIELD,
        ];
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
        pub fn filter(mut self) -> TransactionFilterFieldPathBuilder {
            self.path.push(SubscribeCheckpointsRequest::FILTER_FIELD.name);
            TransactionFilterFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl SubscribeCheckpointsResponse {
        pub const ITEM_FIELD: &'static MessageField = &MessageField {
            name: "item",
            json_name: "item",
            number: 1i32,
            message_fields: Some(CheckpointItem::FIELDS),
        };
        pub const WATERMARK_FIELD: &'static MessageField = &MessageField {
            name: "watermark",
            json_name: "watermark",
            number: 2i32,
            message_fields: Some(Watermark::FIELDS),
        };
    }
    impl MessageFields for SubscribeCheckpointsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ITEM_FIELD,
            Self::WATERMARK_FIELD,
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
        pub fn item(mut self) -> CheckpointItemFieldPathBuilder {
            self.path.push(SubscribeCheckpointsResponse::ITEM_FIELD.name);
            CheckpointItemFieldPathBuilder::new_with_base(self.path)
        }
        pub fn watermark(mut self) -> WatermarkFieldPathBuilder {
            self.path.push(SubscribeCheckpointsResponse::WATERMARK_FIELD.name);
            WatermarkFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl SubscribeTransactionsRequest {
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 1i32,
            message_fields: None,
        };
        pub const FILTER_FIELD: &'static MessageField = &MessageField {
            name: "filter",
            json_name: "filter",
            number: 2i32,
            message_fields: Some(TransactionFilter::FIELDS),
        };
    }
    impl MessageFields for SubscribeTransactionsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::READ_MASK_FIELD,
            Self::FILTER_FIELD,
        ];
    }
    impl SubscribeTransactionsRequest {
        pub fn path_builder() -> SubscribeTransactionsRequestFieldPathBuilder {
            SubscribeTransactionsRequestFieldPathBuilder::new()
        }
    }
    pub struct SubscribeTransactionsRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SubscribeTransactionsRequestFieldPathBuilder {
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
            self.path.push(SubscribeTransactionsRequest::READ_MASK_FIELD.name);
            self.finish()
        }
        pub fn filter(mut self) -> TransactionFilterFieldPathBuilder {
            self.path.push(SubscribeTransactionsRequest::FILTER_FIELD.name);
            TransactionFilterFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl SubscribeTransactionsResponse {
        pub const ITEM_FIELD: &'static MessageField = &MessageField {
            name: "item",
            json_name: "item",
            number: 1i32,
            message_fields: Some(TransactionItem::FIELDS),
        };
        pub const WATERMARK_FIELD: &'static MessageField = &MessageField {
            name: "watermark",
            json_name: "watermark",
            number: 2i32,
            message_fields: Some(Watermark::FIELDS),
        };
    }
    impl MessageFields for SubscribeTransactionsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ITEM_FIELD,
            Self::WATERMARK_FIELD,
        ];
    }
    impl SubscribeTransactionsResponse {
        pub fn path_builder() -> SubscribeTransactionsResponseFieldPathBuilder {
            SubscribeTransactionsResponseFieldPathBuilder::new()
        }
    }
    pub struct SubscribeTransactionsResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SubscribeTransactionsResponseFieldPathBuilder {
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
            self.path.push(SubscribeTransactionsResponse::ITEM_FIELD.name);
            TransactionItemFieldPathBuilder::new_with_base(self.path)
        }
        pub fn watermark(mut self) -> WatermarkFieldPathBuilder {
            self.path.push(SubscribeTransactionsResponse::WATERMARK_FIELD.name);
            WatermarkFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl SubscribeEventsRequest {
        pub const READ_MASK_FIELD: &'static MessageField = &MessageField {
            name: "read_mask",
            json_name: "readMask",
            number: 1i32,
            message_fields: None,
        };
        pub const FILTER_FIELD: &'static MessageField = &MessageField {
            name: "filter",
            json_name: "filter",
            number: 2i32,
            message_fields: Some(EventFilter::FIELDS),
        };
    }
    impl MessageFields for SubscribeEventsRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::READ_MASK_FIELD,
            Self::FILTER_FIELD,
        ];
    }
    impl SubscribeEventsRequest {
        pub fn path_builder() -> SubscribeEventsRequestFieldPathBuilder {
            SubscribeEventsRequestFieldPathBuilder::new()
        }
    }
    pub struct SubscribeEventsRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SubscribeEventsRequestFieldPathBuilder {
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
            self.path.push(SubscribeEventsRequest::READ_MASK_FIELD.name);
            self.finish()
        }
        pub fn filter(mut self) -> EventFilterFieldPathBuilder {
            self.path.push(SubscribeEventsRequest::FILTER_FIELD.name);
            EventFilterFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl SubscribeEventsResponse {
        pub const ITEM_FIELD: &'static MessageField = &MessageField {
            name: "item",
            json_name: "item",
            number: 1i32,
            message_fields: Some(EventItem::FIELDS),
        };
        pub const WATERMARK_FIELD: &'static MessageField = &MessageField {
            name: "watermark",
            json_name: "watermark",
            number: 2i32,
            message_fields: Some(Watermark::FIELDS),
        };
    }
    impl MessageFields for SubscribeEventsResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::ITEM_FIELD,
            Self::WATERMARK_FIELD,
        ];
    }
    impl SubscribeEventsResponse {
        pub fn path_builder() -> SubscribeEventsResponseFieldPathBuilder {
            SubscribeEventsResponseFieldPathBuilder::new()
        }
    }
    pub struct SubscribeEventsResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl SubscribeEventsResponseFieldPathBuilder {
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
            self.path.push(SubscribeEventsResponse::ITEM_FIELD.name);
            EventItemFieldPathBuilder::new_with_base(self.path)
        }
        pub fn watermark(mut self) -> WatermarkFieldPathBuilder {
            self.path.push(SubscribeEventsResponse::WATERMARK_FIELD.name);
            WatermarkFieldPathBuilder::new_with_base(self.path)
        }
    }
}
