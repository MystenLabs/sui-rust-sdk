mod _accessor_impls {
    #![allow(clippy::useless_conversion)]
    impl super::BadRequest {
        pub const fn const_default() -> Self {
            Self {
                field_violations: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::BadRequest = super::BadRequest::const_default();
            &DEFAULT
        }
        pub fn field_violations(&self) -> &[super::bad_request::FieldViolation] {
            &self.field_violations
        }
        pub fn field_violations_mut(
            &mut self,
        ) -> &mut Vec<super::bad_request::FieldViolation> {
            &mut self.field_violations
        }
        pub fn set_field_violations(
            &mut self,
            field: Vec<super::bad_request::FieldViolation>,
        ) {
            self.field_violations = field;
        }
        pub fn with_field_violations(
            mut self,
            field: Vec<super::bad_request::FieldViolation>,
        ) -> Self {
            self.set_field_violations(field);
            self
        }
    }
    impl super::bad_request::FieldViolation {
        pub const fn const_default() -> Self {
            Self {
                field: String::new(),
                description: String::new(),
                reason: String::new(),
                localized_message: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::bad_request::FieldViolation = super::bad_request::FieldViolation::const_default();
            &DEFAULT
        }
        pub fn field_mut(&mut self) -> &mut String {
            &mut self.field
        }
        pub fn set_field<T: Into<String>>(&mut self, field: T) {
            self.field = field.into().into();
        }
        pub fn with_field<T: Into<String>>(mut self, field: T) -> Self {
            self.set_field(field.into());
            self
        }
        pub fn description_mut(&mut self) -> &mut String {
            &mut self.description
        }
        pub fn set_description<T: Into<String>>(&mut self, field: T) {
            self.description = field.into().into();
        }
        pub fn with_description<T: Into<String>>(mut self, field: T) -> Self {
            self.set_description(field.into());
            self
        }
        pub fn reason_mut(&mut self) -> &mut String {
            &mut self.reason
        }
        pub fn set_reason<T: Into<String>>(&mut self, field: T) {
            self.reason = field.into().into();
        }
        pub fn with_reason<T: Into<String>>(mut self, field: T) -> Self {
            self.set_reason(field.into());
            self
        }
        pub fn localized_message(&self) -> &super::LocalizedMessage {
            self.localized_message
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::LocalizedMessage::default_instance() as _)
        }
        pub fn localized_message_opt_mut(
            &mut self,
        ) -> Option<&mut super::LocalizedMessage> {
            self.localized_message.as_mut().map(|field| field as _)
        }
        pub fn localized_message_mut(&mut self) -> &mut super::LocalizedMessage {
            self.localized_message.get_or_insert_default()
        }
        pub fn localized_message_opt(&self) -> Option<&super::LocalizedMessage> {
            self.localized_message.as_ref().map(|field| field as _)
        }
        pub fn set_localized_message<T: Into<super::LocalizedMessage>>(
            &mut self,
            field: T,
        ) {
            self.localized_message = Some(field.into().into());
        }
        pub fn with_localized_message<T: Into<super::LocalizedMessage>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_localized_message(field.into());
            self
        }
    }
    impl super::DebugInfo {
        pub const fn const_default() -> Self {
            Self {
                stack_entries: Vec::new(),
                detail: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::DebugInfo = super::DebugInfo::const_default();
            &DEFAULT
        }
        pub fn stack_entries(&self) -> &[String] {
            &self.stack_entries
        }
        pub fn stack_entries_mut(&mut self) -> &mut Vec<String> {
            &mut self.stack_entries
        }
        pub fn set_stack_entries(&mut self, field: Vec<String>) {
            self.stack_entries = field;
        }
        pub fn with_stack_entries(mut self, field: Vec<String>) -> Self {
            self.set_stack_entries(field);
            self
        }
        pub fn detail_mut(&mut self) -> &mut String {
            &mut self.detail
        }
        pub fn set_detail<T: Into<String>>(&mut self, field: T) {
            self.detail = field.into().into();
        }
        pub fn with_detail<T: Into<String>>(mut self, field: T) -> Self {
            self.set_detail(field.into());
            self
        }
    }
    impl super::ErrorInfo {
        pub const fn const_default() -> Self {
            Self {
                reason: String::new(),
                domain: String::new(),
                metadata: std::collections::BTreeMap::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ErrorInfo = super::ErrorInfo::const_default();
            &DEFAULT
        }
        pub fn reason_mut(&mut self) -> &mut String {
            &mut self.reason
        }
        pub fn set_reason<T: Into<String>>(&mut self, field: T) {
            self.reason = field.into().into();
        }
        pub fn with_reason<T: Into<String>>(mut self, field: T) -> Self {
            self.set_reason(field.into());
            self
        }
        pub fn domain_mut(&mut self) -> &mut String {
            &mut self.domain
        }
        pub fn set_domain<T: Into<String>>(&mut self, field: T) {
            self.domain = field.into().into();
        }
        pub fn with_domain<T: Into<String>>(mut self, field: T) -> Self {
            self.set_domain(field.into());
            self
        }
        pub fn metadata(&self) -> &::std::collections::BTreeMap<String, String> {
            &self.metadata
        }
        pub fn metadata_mut(
            &mut self,
        ) -> &mut ::std::collections::BTreeMap<String, String> {
            &mut self.metadata
        }
        pub fn set_metadata(
            &mut self,
            field: ::std::collections::BTreeMap<String, String>,
        ) {
            self.metadata = field;
        }
        pub fn with_metadata(
            mut self,
            field: ::std::collections::BTreeMap<String, String>,
        ) -> Self {
            self.set_metadata(field);
            self
        }
    }
    impl super::Help {
        pub const fn const_default() -> Self {
            Self { links: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Help = super::Help::const_default();
            &DEFAULT
        }
        pub fn links(&self) -> &[super::help::Link] {
            &self.links
        }
        pub fn links_mut(&mut self) -> &mut Vec<super::help::Link> {
            &mut self.links
        }
        pub fn set_links(&mut self, field: Vec<super::help::Link>) {
            self.links = field;
        }
        pub fn with_links(mut self, field: Vec<super::help::Link>) -> Self {
            self.set_links(field);
            self
        }
    }
    impl super::help::Link {
        pub const fn const_default() -> Self {
            Self {
                description: String::new(),
                url: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::help::Link = super::help::Link::const_default();
            &DEFAULT
        }
        pub fn description_mut(&mut self) -> &mut String {
            &mut self.description
        }
        pub fn set_description<T: Into<String>>(&mut self, field: T) {
            self.description = field.into().into();
        }
        pub fn with_description<T: Into<String>>(mut self, field: T) -> Self {
            self.set_description(field.into());
            self
        }
        pub fn url_mut(&mut self) -> &mut String {
            &mut self.url
        }
        pub fn set_url<T: Into<String>>(&mut self, field: T) {
            self.url = field.into().into();
        }
        pub fn with_url<T: Into<String>>(mut self, field: T) -> Self {
            self.set_url(field.into());
            self
        }
    }
    impl super::LocalizedMessage {
        pub const fn const_default() -> Self {
            Self {
                locale: String::new(),
                message: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::LocalizedMessage = super::LocalizedMessage::const_default();
            &DEFAULT
        }
        pub fn locale_mut(&mut self) -> &mut String {
            &mut self.locale
        }
        pub fn set_locale<T: Into<String>>(&mut self, field: T) {
            self.locale = field.into().into();
        }
        pub fn with_locale<T: Into<String>>(mut self, field: T) -> Self {
            self.set_locale(field.into());
            self
        }
        pub fn message_mut(&mut self) -> &mut String {
            &mut self.message
        }
        pub fn set_message<T: Into<String>>(&mut self, field: T) {
            self.message = field.into().into();
        }
        pub fn with_message<T: Into<String>>(mut self, field: T) -> Self {
            self.set_message(field.into());
            self
        }
    }
    impl super::PreconditionFailure {
        pub const fn const_default() -> Self {
            Self { violations: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::PreconditionFailure = super::PreconditionFailure::const_default();
            &DEFAULT
        }
        pub fn violations(&self) -> &[super::precondition_failure::Violation] {
            &self.violations
        }
        pub fn violations_mut(
            &mut self,
        ) -> &mut Vec<super::precondition_failure::Violation> {
            &mut self.violations
        }
        pub fn set_violations(
            &mut self,
            field: Vec<super::precondition_failure::Violation>,
        ) {
            self.violations = field;
        }
        pub fn with_violations(
            mut self,
            field: Vec<super::precondition_failure::Violation>,
        ) -> Self {
            self.set_violations(field);
            self
        }
    }
    impl super::precondition_failure::Violation {
        pub const fn const_default() -> Self {
            Self {
                r#type: String::new(),
                subject: String::new(),
                description: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::precondition_failure::Violation = super::precondition_failure::Violation::const_default();
            &DEFAULT
        }
        pub fn type_mut(&mut self) -> &mut String {
            &mut self.r#type
        }
        pub fn set_type<T: Into<String>>(&mut self, field: T) {
            self.r#type = field.into().into();
        }
        pub fn with_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_type(field.into());
            self
        }
        pub fn subject_mut(&mut self) -> &mut String {
            &mut self.subject
        }
        pub fn set_subject<T: Into<String>>(&mut self, field: T) {
            self.subject = field.into().into();
        }
        pub fn with_subject<T: Into<String>>(mut self, field: T) -> Self {
            self.set_subject(field.into());
            self
        }
        pub fn description_mut(&mut self) -> &mut String {
            &mut self.description
        }
        pub fn set_description<T: Into<String>>(&mut self, field: T) {
            self.description = field.into().into();
        }
        pub fn with_description<T: Into<String>>(mut self, field: T) -> Self {
            self.set_description(field.into());
            self
        }
    }
    impl super::QuotaFailure {
        pub const fn const_default() -> Self {
            Self { violations: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::QuotaFailure = super::QuotaFailure::const_default();
            &DEFAULT
        }
        pub fn violations(&self) -> &[super::quota_failure::Violation] {
            &self.violations
        }
        pub fn violations_mut(&mut self) -> &mut Vec<super::quota_failure::Violation> {
            &mut self.violations
        }
        pub fn set_violations(&mut self, field: Vec<super::quota_failure::Violation>) {
            self.violations = field;
        }
        pub fn with_violations(
            mut self,
            field: Vec<super::quota_failure::Violation>,
        ) -> Self {
            self.set_violations(field);
            self
        }
    }
    impl super::quota_failure::Violation {
        pub const fn const_default() -> Self {
            Self {
                subject: String::new(),
                description: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::quota_failure::Violation = super::quota_failure::Violation::const_default();
            &DEFAULT
        }
        pub fn subject_mut(&mut self) -> &mut String {
            &mut self.subject
        }
        pub fn set_subject<T: Into<String>>(&mut self, field: T) {
            self.subject = field.into().into();
        }
        pub fn with_subject<T: Into<String>>(mut self, field: T) -> Self {
            self.set_subject(field.into());
            self
        }
        pub fn description_mut(&mut self) -> &mut String {
            &mut self.description
        }
        pub fn set_description<T: Into<String>>(&mut self, field: T) {
            self.description = field.into().into();
        }
        pub fn with_description<T: Into<String>>(mut self, field: T) -> Self {
            self.set_description(field.into());
            self
        }
    }
    impl super::RequestInfo {
        pub const fn const_default() -> Self {
            Self {
                request_id: String::new(),
                serving_data: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::RequestInfo = super::RequestInfo::const_default();
            &DEFAULT
        }
        pub fn request_id_mut(&mut self) -> &mut String {
            &mut self.request_id
        }
        pub fn set_request_id<T: Into<String>>(&mut self, field: T) {
            self.request_id = field.into().into();
        }
        pub fn with_request_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_request_id(field.into());
            self
        }
        pub fn serving_data_mut(&mut self) -> &mut String {
            &mut self.serving_data
        }
        pub fn set_serving_data<T: Into<String>>(&mut self, field: T) {
            self.serving_data = field.into().into();
        }
        pub fn with_serving_data<T: Into<String>>(mut self, field: T) -> Self {
            self.set_serving_data(field.into());
            self
        }
    }
    impl super::ResourceInfo {
        pub const fn const_default() -> Self {
            Self {
                resource_type: String::new(),
                resource_name: String::new(),
                owner: String::new(),
                description: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::ResourceInfo = super::ResourceInfo::const_default();
            &DEFAULT
        }
        pub fn resource_type_mut(&mut self) -> &mut String {
            &mut self.resource_type
        }
        pub fn set_resource_type<T: Into<String>>(&mut self, field: T) {
            self.resource_type = field.into().into();
        }
        pub fn with_resource_type<T: Into<String>>(mut self, field: T) -> Self {
            self.set_resource_type(field.into());
            self
        }
        pub fn resource_name_mut(&mut self) -> &mut String {
            &mut self.resource_name
        }
        pub fn set_resource_name<T: Into<String>>(&mut self, field: T) {
            self.resource_name = field.into().into();
        }
        pub fn with_resource_name<T: Into<String>>(mut self, field: T) -> Self {
            self.set_resource_name(field.into());
            self
        }
        pub fn owner_mut(&mut self) -> &mut String {
            &mut self.owner
        }
        pub fn set_owner<T: Into<String>>(&mut self, field: T) {
            self.owner = field.into().into();
        }
        pub fn with_owner<T: Into<String>>(mut self, field: T) -> Self {
            self.set_owner(field.into());
            self
        }
        pub fn description_mut(&mut self) -> &mut String {
            &mut self.description
        }
        pub fn set_description<T: Into<String>>(&mut self, field: T) {
            self.description = field.into().into();
        }
        pub fn with_description<T: Into<String>>(mut self, field: T) -> Self {
            self.set_description(field.into());
            self
        }
    }
    impl super::RetryInfo {
        pub const fn const_default() -> Self {
            Self { retry_delay: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::RetryInfo = super::RetryInfo::const_default();
            &DEFAULT
        }
        pub fn retry_delay_opt_mut(&mut self) -> Option<&mut ::prost_types::Duration> {
            self.retry_delay.as_mut().map(|field| field as _)
        }
        pub fn retry_delay_mut(&mut self) -> &mut ::prost_types::Duration {
            self.retry_delay.get_or_insert_default()
        }
        pub fn retry_delay_opt(&self) -> Option<&::prost_types::Duration> {
            self.retry_delay.as_ref().map(|field| field as _)
        }
        pub fn set_retry_delay<T: Into<::prost_types::Duration>>(&mut self, field: T) {
            self.retry_delay = Some(field.into().into());
        }
        pub fn with_retry_delay<T: Into<::prost_types::Duration>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_retry_delay(field.into());
            self
        }
    }
    impl super::Status {
        pub const fn const_default() -> Self {
            Self {
                code: 0,
                message: String::new(),
                details: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::Status = super::Status::const_default();
            &DEFAULT
        }
        pub fn code_mut(&mut self) -> &mut i32 {
            &mut self.code
        }
        pub fn set_code<T: Into<i32>>(&mut self, field: T) {
            self.code = field.into().into();
        }
        pub fn with_code<T: Into<i32>>(mut self, field: T) -> Self {
            self.set_code(field.into());
            self
        }
        pub fn message_mut(&mut self) -> &mut String {
            &mut self.message
        }
        pub fn set_message<T: Into<String>>(&mut self, field: T) {
            self.message = field.into().into();
        }
        pub fn with_message<T: Into<String>>(mut self, field: T) -> Self {
            self.set_message(field.into());
            self
        }
        pub fn details(&self) -> &[::prost_types::Any] {
            &self.details
        }
        pub fn details_mut(&mut self) -> &mut Vec<::prost_types::Any> {
            &mut self.details
        }
        pub fn set_details(&mut self, field: Vec<::prost_types::Any>) {
            self.details = field;
        }
        pub fn with_details(mut self, field: Vec<::prost_types::Any>) -> Self {
            self.set_details(field);
            self
        }
    }
}
