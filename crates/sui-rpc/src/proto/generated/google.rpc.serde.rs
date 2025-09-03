impl serde::Serialize for BadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field_violations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.BadRequest", len)?;
        if !self.field_violations.is_empty() {
            struct_ser.serialize_field("fieldViolations", &self.field_violations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_violations",
            "fieldViolations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldViolations,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fieldViolations" | "field_violations" => Ok(GeneratedField::FieldViolations),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.BadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_violations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FieldViolations => {
                            if field_violations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldViolations"));
                            }
                            field_violations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(BadRequest {
                    field_violations: field_violations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.BadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bad_request::FieldViolation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        if self.localized_message.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.BadRequest.FieldViolation", len)?;
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if let Some(v) = self.localized_message.as_ref() {
            struct_ser.serialize_field("localizedMessage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for bad_request::FieldViolation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "description",
            "reason",
            "localized_message",
            "localizedMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Description,
            Reason,
            LocalizedMessage,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "field" => Ok(GeneratedField::Field),
                            "description" => Ok(GeneratedField::Description),
                            "reason" => Ok(GeneratedField::Reason),
                            "localizedMessage" | "localized_message" => Ok(GeneratedField::LocalizedMessage),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bad_request::FieldViolation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.BadRequest.FieldViolation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<bad_request::FieldViolation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut description__ = None;
                let mut reason__ = None;
                let mut localized_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocalizedMessage => {
                            if localized_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localizedMessage"));
                            }
                            localized_message__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(bad_request::FieldViolation {
                    field: field__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                    localized_message: localized_message__,
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.BadRequest.FieldViolation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DebugInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stack_entries.is_empty() {
            len += 1;
        }
        if !self.detail.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.DebugInfo", len)?;
        if !self.stack_entries.is_empty() {
            struct_ser.serialize_field("stackEntries", &self.stack_entries)?;
        }
        if !self.detail.is_empty() {
            struct_ser.serialize_field("detail", &self.detail)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DebugInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stack_entries",
            "stackEntries",
            "detail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StackEntries,
            Detail,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stackEntries" | "stack_entries" => Ok(GeneratedField::StackEntries),
                            "detail" => Ok(GeneratedField::Detail),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DebugInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.DebugInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DebugInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stack_entries__ = None;
                let mut detail__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StackEntries => {
                            if stack_entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackEntries"));
                            }
                            stack_entries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Detail => {
                            if detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            detail__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DebugInfo {
                    stack_entries: stack_entries__.unwrap_or_default(),
                    detail: detail__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.DebugInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reason.is_empty() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.ErrorInfo", len)?;
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ErrorInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
            "domain",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
            Domain,
            Metadata,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reason" => Ok(GeneratedField::Reason),
                            "domain" => Ok(GeneratedField::Domain),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.ErrorInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ErrorInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                let mut domain__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(
                                map_.next_value::<std::collections::BTreeMap<_, _>>()?
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ErrorInfo {
                    reason: reason__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.ErrorInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Help {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.links.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.Help", len)?;
        if !self.links.is_empty() {
            struct_ser.serialize_field("links", &self.links)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Help {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "links",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Links,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "links" => Ok(GeneratedField::Links),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Help;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.Help")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Help, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut links__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Links => {
                            if links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("links"));
                            }
                            links__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Help {
                    links: links__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.Help", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for help::Link {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.Help.Link", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for help::Link {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "url",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Url,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "description" => Ok(GeneratedField::Description),
                            "url" => Ok(GeneratedField::Url),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = help::Link;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.Help.Link")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<help::Link, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(help::Link {
                    description: description__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.Help.Link", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalizedMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.locale.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.LocalizedMessage", len)?;
        if !self.locale.is_empty() {
            struct_ser.serialize_field("locale", &self.locale)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalizedMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locale",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locale,
            Message,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "locale" => Ok(GeneratedField::Locale),
                            "message" => Ok(GeneratedField::Message),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalizedMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.LocalizedMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LocalizedMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locale__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locale => {
                            if locale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            locale__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(LocalizedMessage {
                    locale: locale__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.LocalizedMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PreconditionFailure {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.violations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.PreconditionFailure", len)?;
        if !self.violations.is_empty() {
            struct_ser.serialize_field("violations", &self.violations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PreconditionFailure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "violations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Violations,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "violations" => Ok(GeneratedField::Violations),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PreconditionFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.PreconditionFailure")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PreconditionFailure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut violations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Violations => {
                            if violations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("violations"));
                            }
                            violations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PreconditionFailure {
                    violations: violations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.PreconditionFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for precondition_failure::Violation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.subject.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.PreconditionFailure.Violation", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.subject.is_empty() {
            struct_ser.serialize_field("subject", &self.subject)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for precondition_failure::Violation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "subject",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Subject,
            Description,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "subject" => Ok(GeneratedField::Subject),
                            "description" => Ok(GeneratedField::Description),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = precondition_failure::Violation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.PreconditionFailure.Violation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<precondition_failure::Violation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut subject__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(precondition_failure::Violation {
                    r#type: r#type__.unwrap_or_default(),
                    subject: subject__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.PreconditionFailure.Violation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaFailure {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.violations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.QuotaFailure", len)?;
        if !self.violations.is_empty() {
            struct_ser.serialize_field("violations", &self.violations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaFailure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "violations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Violations,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "violations" => Ok(GeneratedField::Violations),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.QuotaFailure")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaFailure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut violations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Violations => {
                            if violations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("violations"));
                            }
                            violations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QuotaFailure {
                    violations: violations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.QuotaFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for quota_failure::Violation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subject.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.QuotaFailure.Violation", len)?;
        if !self.subject.is_empty() {
            struct_ser.serialize_field("subject", &self.subject)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for quota_failure::Violation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subject,
            Description,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subject" => Ok(GeneratedField::Subject),
                            "description" => Ok(GeneratedField::Description),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = quota_failure::Violation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.QuotaFailure.Violation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<quota_failure::Violation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(quota_failure::Violation {
                    subject: subject__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.QuotaFailure.Violation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_id.is_empty() {
            len += 1;
        }
        if !self.serving_data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.RequestInfo", len)?;
        if !self.request_id.is_empty() {
            struct_ser.serialize_field("requestId", &self.request_id)?;
        }
        if !self.serving_data.is_empty() {
            struct_ser.serialize_field("servingData", &self.serving_data)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_id",
            "requestId",
            "serving_data",
            "servingData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestId,
            ServingData,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            "servingData" | "serving_data" => Ok(GeneratedField::ServingData),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.RequestInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_id__ = None;
                let mut serving_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServingData => {
                            if serving_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("servingData"));
                            }
                            serving_data__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RequestInfo {
                    request_id: request_id__.unwrap_or_default(),
                    serving_data: serving_data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.RequestInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_type.is_empty() {
            len += 1;
        }
        if !self.resource_name.is_empty() {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.ResourceInfo", len)?;
        if !self.resource_type.is_empty() {
            struct_ser.serialize_field("resourceType", &self.resource_type)?;
        }
        if !self.resource_name.is_empty() {
            struct_ser.serialize_field("resourceName", &self.resource_name)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_type",
            "resourceType",
            "resource_name",
            "resourceName",
            "owner",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceType,
            ResourceName,
            Owner,
            Description,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resourceType" | "resource_type" => Ok(GeneratedField::ResourceType),
                            "resourceName" | "resource_name" => Ok(GeneratedField::ResourceName),
                            "owner" => Ok(GeneratedField::Owner),
                            "description" => Ok(GeneratedField::Description),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.ResourceInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_type__ = None;
                let mut resource_name__ = None;
                let mut owner__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceType"));
                            }
                            resource_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceName"));
                            }
                            resource_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ResourceInfo {
                    resource_type: resource_type__.unwrap_or_default(),
                    resource_name: resource_name__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.ResourceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RetryInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.retry_delay.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.RetryInfo", len)?;
        if let Some(v) = self.retry_delay.as_ref() {
            struct_ser.serialize_field("retryDelay", &crate::_serde::DurationSerializer(v))?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "retry_delay",
            "retryDelay",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RetryDelay,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "retryDelay" | "retry_delay" => Ok(GeneratedField::RetryDelay),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.RetryInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetryInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut retry_delay__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RetryDelay => {
                            if retry_delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryDelay"));
                            }
                            retry_delay__ = map_.next_value::<::std::option::Option<crate::_serde::DurationDeserializer>>()?.map(|x| x.0.into());
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RetryInfo {
                    retry_delay: retry_delay__,
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.RetryInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.Status", len)?;
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details.iter().map(crate::_serde::AnySerializer).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
            Details,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            "details" => Ok(GeneratedField::Details),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Status;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.Status")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Status, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<crate::_serde::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = Some(map_.next_value::<Vec<crate::_serde::AnyDeserializer>>()?.into_iter().map(|x| x.0.into()).collect());
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Status {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    details: details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.Status", FIELDS, GeneratedVisitor)
    }
}
