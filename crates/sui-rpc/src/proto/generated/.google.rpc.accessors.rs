mod _getter_impls {
    #![allow(clippy::useless_conversion)]
    use super::*;
    impl BadRequest {
        pub const fn const_default() -> Self {
            Self {
                field_violations: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl bad_request::FieldViolation {
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl DebugInfo {
        pub const fn const_default() -> Self {
            Self {
                stack_entries: Vec::new(),
                detail: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl ErrorInfo {
        pub const fn const_default() -> Self {
            Self {
                reason: String::new(),
                domain: String::new(),
                metadata: std::collections::BTreeMap::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl Help {
        pub const fn const_default() -> Self {
            Self { links: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl help::Link {
        pub const fn const_default() -> Self {
            Self {
                description: String::new(),
                url: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl LocalizedMessage {
        pub const fn const_default() -> Self {
            Self {
                locale: String::new(),
                message: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl PreconditionFailure {
        pub const fn const_default() -> Self {
            Self { violations: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl precondition_failure::Violation {
        pub const fn const_default() -> Self {
            Self {
                r#type: String::new(),
                subject: String::new(),
                description: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl QuotaFailure {
        pub const fn const_default() -> Self {
            Self { violations: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl quota_failure::Violation {
        pub const fn const_default() -> Self {
            Self {
                subject: String::new(),
                description: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl RequestInfo {
        pub const fn const_default() -> Self {
            Self {
                request_id: String::new(),
                serving_data: String::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl ResourceInfo {
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
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl RetryInfo {
        pub const fn const_default() -> Self {
            Self { retry_delay: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
    impl Status {
        pub const fn const_default() -> Self {
            Self {
                code: 0,
                message: String::new(),
                details: Vec::new(),
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: Self = Self::const_default();
            &DEFAULT
        }
    }
}
