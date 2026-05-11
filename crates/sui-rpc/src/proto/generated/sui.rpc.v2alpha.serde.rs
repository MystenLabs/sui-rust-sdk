impl serde::Serialize for AffectedAddressFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.AffectedAddressFilter", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AffectedAddressFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
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
            type Value = AffectedAddressFilter;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.AffectedAddressFilter")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<AffectedAddressFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AffectedAddressFilter {
                    address: address__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.AffectedAddressFilter",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for AffectedObjectFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.object_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.AffectedObjectFilter", len)?;
        if let Some(v) = self.object_id.as_ref() {
            struct_ser.serialize_field("objectId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AffectedObjectFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["object_id", "objectId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objectId" | "object_id" => Ok(GeneratedField::ObjectId),
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
            type Value = AffectedObjectFilter;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.AffectedObjectFilter")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<AffectedObjectFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut object_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ObjectId => {
                            if object_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectId"));
                            }
                            object_id__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AffectedObjectFilter {
                    object_id: object_id__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.AffectedObjectFilter",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for CheckpointItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.cursor.is_some() {
            len += 1;
        }
        if self.checkpoint.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.CheckpointItem", len)?;
        if let Some(v) = self.cursor.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("cursor", crate::_serde::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.checkpoint.as_ref() {
            struct_ser.serialize_field("checkpoint", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckpointItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["cursor", "checkpoint"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cursor,
            Checkpoint,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cursor" => Ok(GeneratedField::Cursor),
                            "checkpoint" => Ok(GeneratedField::Checkpoint),
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
            type Value = CheckpointItem;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.CheckpointItem")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<CheckpointItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut cursor__ = None;
                let mut checkpoint__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cursor => {
                            if cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cursor"));
                            }
                            cursor__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Checkpoint => {
                            if checkpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkpoint"));
                            }
                            checkpoint__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CheckpointItem {
                    cursor: cursor__,
                    checkpoint: checkpoint__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.CheckpointItem",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for EmitModuleFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.module.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.EmitModuleFilter", len)?;
        if let Some(v) = self.module.as_ref() {
            struct_ser.serialize_field("module", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EmitModuleFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["module"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Module,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "module" => Ok(GeneratedField::Module),
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
            type Value = EmitModuleFilter;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.EmitModuleFilter")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EmitModuleFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut module__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Module => {
                            if module__.is_some() {
                                return Err(serde::de::Error::duplicate_field("module"));
                            }
                            module__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EmitModuleFilter {
                    module: module__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.EmitModuleFilter",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for EventFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if !self.terms.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.EventFilter", len)?;
        if !self.terms.is_empty() {
            struct_ser.serialize_field("terms", &self.terms)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["terms"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Terms,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "terms" => Ok(GeneratedField::Terms),
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
            type Value = EventFilter;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.EventFilter")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut terms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Terms => {
                            if terms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terms"));
                            }
                            terms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EventFilter {
                    terms: terms__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.EventFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.cursor.is_some() {
            len += 1;
        }
        if self.checkpoint.is_some() {
            len += 1;
        }
        if self.event_index.is_some() {
            len += 1;
        }
        if self.transaction_digest.is_some() {
            len += 1;
        }
        if self.event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.EventItem", len)?;
        if let Some(v) = self.cursor.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("cursor", crate::_serde::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.checkpoint.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("checkpoint", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.event_index.as_ref() {
            struct_ser.serialize_field("eventIndex", v)?;
        }
        if let Some(v) = self.transaction_digest.as_ref() {
            struct_ser.serialize_field("transactionDigest", v)?;
        }
        if let Some(v) = self.event.as_ref() {
            struct_ser.serialize_field("event", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cursor",
            "checkpoint",
            "event_index",
            "eventIndex",
            "transaction_digest",
            "transactionDigest",
            "event",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cursor,
            Checkpoint,
            EventIndex,
            TransactionDigest,
            Event,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cursor" => Ok(GeneratedField::Cursor),
                            "checkpoint" => Ok(GeneratedField::Checkpoint),
                            "eventIndex" | "event_index" => {
                                Ok(GeneratedField::EventIndex)
                            }
                            "transactionDigest" | "transaction_digest" => {
                                Ok(GeneratedField::TransactionDigest)
                            }
                            "event" => Ok(GeneratedField::Event),
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
            type Value = EventItem;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.EventItem")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut cursor__ = None;
                let mut checkpoint__ = None;
                let mut event_index__ = None;
                let mut transaction_digest__ = None;
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cursor => {
                            if cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cursor"));
                            }
                            cursor__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Checkpoint => {
                            if checkpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkpoint"));
                            }
                            checkpoint__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::EventIndex => {
                            if event_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventIndex"));
                            }
                            event_index__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::TransactionDigest => {
                            if transaction_digest__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("transactionDigest"),
                                );
                            }
                            transaction_digest__ = map_.next_value()?;
                        }
                        GeneratedField::Event => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            event__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EventItem {
                    cursor: cursor__,
                    checkpoint: checkpoint__,
                    event_index: event_index__,
                    transaction_digest: transaction_digest__,
                    event: event__,
                })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.EventItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventLiteral {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.polarity.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.EventLiteral", len)?;
        if let Some(v) = self.polarity.as_ref() {
            match v {
                event_literal::Polarity::Include(v) => {
                    struct_ser.serialize_field("include", v)?;
                }
                event_literal::Polarity::Exclude(v) => {
                    struct_ser.serialize_field("exclude", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventLiteral {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["include", "exclude"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Include,
            Exclude,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "include" => Ok(GeneratedField::Include),
                            "exclude" => Ok(GeneratedField::Exclude),
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
            type Value = EventLiteral;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.EventLiteral")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventLiteral, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut polarity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Include => {
                            if polarity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("include"));
                            }
                            polarity__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(event_literal::Polarity::Include);
                        }
                        GeneratedField::Exclude => {
                            if polarity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exclude"));
                            }
                            polarity__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(event_literal::Polarity::Exclude);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EventLiteral {
                    polarity: polarity__,
                })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.EventLiteral", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventPredicate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.predicate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.EventPredicate", len)?;
        if let Some(v) = self.predicate.as_ref() {
            match v {
                event_predicate::Predicate::Sender(v) => {
                    struct_ser.serialize_field("sender", v)?;
                }
                event_predicate::Predicate::AffectedObject(v) => {
                    struct_ser.serialize_field("affectedObject", v)?;
                }
                event_predicate::Predicate::EmitModule(v) => {
                    struct_ser.serialize_field("emitModule", v)?;
                }
                event_predicate::Predicate::EventType(v) => {
                    struct_ser.serialize_field("eventType", v)?;
                }
                event_predicate::Predicate::EventStreamHead(v) => {
                    struct_ser.serialize_field("eventStreamHead", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventPredicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "affected_object",
            "affectedObject",
            "emit_module",
            "emitModule",
            "event_type",
            "eventType",
            "event_stream_head",
            "eventStreamHead",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            AffectedObject,
            EmitModule,
            EventType,
            EventStreamHead,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "affectedObject" | "affected_object" => {
                                Ok(GeneratedField::AffectedObject)
                            }
                            "emitModule" | "emit_module" => {
                                Ok(GeneratedField::EmitModule)
                            }
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "eventStreamHead" | "event_stream_head" => {
                                Ok(GeneratedField::EventStreamHead)
                            }
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
            type Value = EventPredicate;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.EventPredicate")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventPredicate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut predicate__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(event_predicate::Predicate::Sender);
                        }
                        GeneratedField::AffectedObject => {
                            if predicate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("affectedObject"),
                                );
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(event_predicate::Predicate::AffectedObject);
                        }
                        GeneratedField::EmitModule => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emitModule"));
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(event_predicate::Predicate::EmitModule);
                        }
                        GeneratedField::EventType => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(event_predicate::Predicate::EventType);
                        }
                        GeneratedField::EventStreamHead => {
                            if predicate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("eventStreamHead"),
                                );
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(event_predicate::Predicate::EventStreamHead);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EventPredicate {
                    predicate: predicate__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.EventPredicate",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for EventStreamHeadFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.stream_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.EventStreamHeadFilter", len)?;
        if let Some(v) = self.stream_id.as_ref() {
            struct_ser.serialize_field("streamId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventStreamHeadFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["stream_id", "streamId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StreamId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "streamId" | "stream_id" => Ok(GeneratedField::StreamId),
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
            type Value = EventStreamHeadFilter;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.EventStreamHeadFilter")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventStreamHeadFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut stream_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StreamId => {
                            if stream_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamId"));
                            }
                            stream_id__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EventStreamHeadFilter {
                    stream_id: stream_id__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.EventStreamHeadFilter",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for EventTerm {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if !self.literals.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.EventTerm", len)?;
        if !self.literals.is_empty() {
            struct_ser.serialize_field("literals", &self.literals)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventTerm {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["literals"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Literals,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "literals" => Ok(GeneratedField::Literals),
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
            type Value = EventTerm;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.EventTerm")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventTerm, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut literals__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Literals => {
                            if literals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("literals"));
                            }
                            literals__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EventTerm {
                    literals: literals__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.EventTerm", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventTypeFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.EventTypeFilter", len)?;
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventTypeFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["type"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
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
            type Value = EventTypeFilter;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.EventTypeFilter")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventTypeFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            type__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EventTypeFilter { r#type: type__ })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.EventTypeFilter",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for ListCheckpointsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.read_mask.is_some() {
            len += 1;
        }
        if self.start_checkpoint.is_some() {
            len += 1;
        }
        if self.end_checkpoint.is_some() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.ListCheckpointsRequest", len)?;
        if let Some(v) = self.read_mask.as_ref() {
            struct_ser
                .serialize_field("readMask", &crate::_serde::FieldMaskSerializer(v))?;
        }
        if let Some(v) = self.start_checkpoint.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("startCheckpoint", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.end_checkpoint.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("endCheckpoint", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCheckpointsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "read_mask",
            "readMask",
            "start_checkpoint",
            "startCheckpoint",
            "end_checkpoint",
            "endCheckpoint",
            "filter",
            "pagination",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadMask,
            StartCheckpoint,
            EndCheckpoint,
            Filter,
            Pagination,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "readMask" | "read_mask" => Ok(GeneratedField::ReadMask),
                            "startCheckpoint" | "start_checkpoint" => {
                                Ok(GeneratedField::StartCheckpoint)
                            }
                            "endCheckpoint" | "end_checkpoint" => {
                                Ok(GeneratedField::EndCheckpoint)
                            }
                            "filter" => Ok(GeneratedField::Filter),
                            "pagination" => Ok(GeneratedField::Pagination),
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
            type Value = ListCheckpointsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.ListCheckpointsRequest")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ListCheckpointsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut read_mask__ = None;
                let mut start_checkpoint__ = None;
                let mut end_checkpoint__ = None;
                let mut filter__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReadMask => {
                            if read_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readMask"));
                            }
                            read_mask__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::FieldMaskDeserializer>,
                                >()?
                                .map(|x| x.0.into());
                        }
                        GeneratedField::StartCheckpoint => {
                            if start_checkpoint__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("startCheckpoint"),
                                );
                            }
                            start_checkpoint__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::EndCheckpoint => {
                            if end_checkpoint__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("endCheckpoint"),
                                );
                            }
                            end_checkpoint__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListCheckpointsRequest {
                    read_mask: read_mask__,
                    start_checkpoint: start_checkpoint__,
                    end_checkpoint: end_checkpoint__,
                    filter: filter__,
                    pagination: pagination__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.ListCheckpointsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for ListCheckpointsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.ListCheckpointsResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            match v {
                list_checkpoints_response::Response::Item(v) => {
                    struct_ser.serialize_field("item", v)?;
                }
                list_checkpoints_response::Response::PageInfo(v) => {
                    struct_ser.serialize_field("pageInfo", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCheckpointsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["item", "page_info", "pageInfo"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Item,
            PageInfo,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "item" => Ok(GeneratedField::Item),
                            "pageInfo" | "page_info" => Ok(GeneratedField::PageInfo),
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
            type Value = ListCheckpointsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.ListCheckpointsResponse")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ListCheckpointsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Item => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_checkpoints_response::Response::Item);
                        }
                        GeneratedField::PageInfo => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageInfo"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_checkpoints_response::Response::PageInfo);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListCheckpointsResponse {
                    response: response__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.ListCheckpointsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for ListEventsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.read_mask.is_some() {
            len += 1;
        }
        if self.start_checkpoint.is_some() {
            len += 1;
        }
        if self.end_checkpoint.is_some() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.ListEventsRequest", len)?;
        if let Some(v) = self.read_mask.as_ref() {
            struct_ser
                .serialize_field("readMask", &crate::_serde::FieldMaskSerializer(v))?;
        }
        if let Some(v) = self.start_checkpoint.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("startCheckpoint", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.end_checkpoint.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("endCheckpoint", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListEventsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "read_mask",
            "readMask",
            "start_checkpoint",
            "startCheckpoint",
            "end_checkpoint",
            "endCheckpoint",
            "filter",
            "pagination",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadMask,
            StartCheckpoint,
            EndCheckpoint,
            Filter,
            Pagination,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "readMask" | "read_mask" => Ok(GeneratedField::ReadMask),
                            "startCheckpoint" | "start_checkpoint" => {
                                Ok(GeneratedField::StartCheckpoint)
                            }
                            "endCheckpoint" | "end_checkpoint" => {
                                Ok(GeneratedField::EndCheckpoint)
                            }
                            "filter" => Ok(GeneratedField::Filter),
                            "pagination" => Ok(GeneratedField::Pagination),
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
            type Value = ListEventsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.ListEventsRequest")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ListEventsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut read_mask__ = None;
                let mut start_checkpoint__ = None;
                let mut end_checkpoint__ = None;
                let mut filter__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReadMask => {
                            if read_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readMask"));
                            }
                            read_mask__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::FieldMaskDeserializer>,
                                >()?
                                .map(|x| x.0.into());
                        }
                        GeneratedField::StartCheckpoint => {
                            if start_checkpoint__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("startCheckpoint"),
                                );
                            }
                            start_checkpoint__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::EndCheckpoint => {
                            if end_checkpoint__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("endCheckpoint"),
                                );
                            }
                            end_checkpoint__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListEventsRequest {
                    read_mask: read_mask__,
                    start_checkpoint: start_checkpoint__,
                    end_checkpoint: end_checkpoint__,
                    filter: filter__,
                    pagination: pagination__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.ListEventsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for ListEventsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.ListEventsResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            match v {
                list_events_response::Response::Item(v) => {
                    struct_ser.serialize_field("item", v)?;
                }
                list_events_response::Response::PageInfo(v) => {
                    struct_ser.serialize_field("pageInfo", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListEventsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["item", "page_info", "pageInfo"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Item,
            PageInfo,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "item" => Ok(GeneratedField::Item),
                            "pageInfo" | "page_info" => Ok(GeneratedField::PageInfo),
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
            type Value = ListEventsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.ListEventsResponse")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ListEventsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Item => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_events_response::Response::Item);
                        }
                        GeneratedField::PageInfo => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageInfo"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_events_response::Response::PageInfo);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListEventsResponse {
                    response: response__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.ListEventsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for ListTransactionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.read_mask.is_some() {
            len += 1;
        }
        if self.start_checkpoint.is_some() {
            len += 1;
        }
        if self.end_checkpoint.is_some() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.ListTransactionsRequest", len)?;
        if let Some(v) = self.read_mask.as_ref() {
            struct_ser
                .serialize_field("readMask", &crate::_serde::FieldMaskSerializer(v))?;
        }
        if let Some(v) = self.start_checkpoint.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("startCheckpoint", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.end_checkpoint.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("endCheckpoint", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTransactionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "read_mask",
            "readMask",
            "start_checkpoint",
            "startCheckpoint",
            "end_checkpoint",
            "endCheckpoint",
            "filter",
            "pagination",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadMask,
            StartCheckpoint,
            EndCheckpoint,
            Filter,
            Pagination,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "readMask" | "read_mask" => Ok(GeneratedField::ReadMask),
                            "startCheckpoint" | "start_checkpoint" => {
                                Ok(GeneratedField::StartCheckpoint)
                            }
                            "endCheckpoint" | "end_checkpoint" => {
                                Ok(GeneratedField::EndCheckpoint)
                            }
                            "filter" => Ok(GeneratedField::Filter),
                            "pagination" => Ok(GeneratedField::Pagination),
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
            type Value = ListTransactionsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.ListTransactionsRequest")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ListTransactionsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut read_mask__ = None;
                let mut start_checkpoint__ = None;
                let mut end_checkpoint__ = None;
                let mut filter__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReadMask => {
                            if read_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readMask"));
                            }
                            read_mask__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::FieldMaskDeserializer>,
                                >()?
                                .map(|x| x.0.into());
                        }
                        GeneratedField::StartCheckpoint => {
                            if start_checkpoint__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("startCheckpoint"),
                                );
                            }
                            start_checkpoint__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::EndCheckpoint => {
                            if end_checkpoint__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("endCheckpoint"),
                                );
                            }
                            end_checkpoint__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListTransactionsRequest {
                    read_mask: read_mask__,
                    start_checkpoint: start_checkpoint__,
                    end_checkpoint: end_checkpoint__,
                    filter: filter__,
                    pagination: pagination__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.ListTransactionsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for ListTransactionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.ListTransactionsResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            match v {
                list_transactions_response::Response::Item(v) => {
                    struct_ser.serialize_field("item", v)?;
                }
                list_transactions_response::Response::PageInfo(v) => {
                    struct_ser.serialize_field("pageInfo", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTransactionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["item", "page_info", "pageInfo"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Item,
            PageInfo,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "item" => Ok(GeneratedField::Item),
                            "pageInfo" | "page_info" => Ok(GeneratedField::PageInfo),
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
            type Value = ListTransactionsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.ListTransactionsResponse")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ListTransactionsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Item => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_transactions_response::Response::Item);
                        }
                        GeneratedField::PageInfo => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageInfo"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_transactions_response::Response::PageInfo);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListTransactionsResponse {
                    response: response__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.ListTransactionsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MoveCallFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.function.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.MoveCallFilter", len)?;
        if let Some(v) = self.function.as_ref() {
            struct_ser.serialize_field("function", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MoveCallFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["function"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Function,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "function" => Ok(GeneratedField::Function),
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
            type Value = MoveCallFilter;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.MoveCallFilter")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MoveCallFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut function__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Function => {
                            if function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            function__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MoveCallFilter {
                    function: function__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.MoveCallFilter",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for PageInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.PageInfo", len)?;
        if let Some(v) = self.next_page_token.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field(
                    "nextPageToken",
                    crate::_serde::base64::encode(&v).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PageInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["next_page_token", "nextPageToken"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NextPageToken,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nextPageToken" | "next_page_token" => {
                                Ok(GeneratedField::NextPageToken)
                            }
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
            type Value = PageInfo;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.PageInfo")
            }
            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PageInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("nextPageToken"),
                                );
                            }
                            next_page_token__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PageInfo {
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.PageInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Pagination {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.page_size.is_some() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        if self.ordering != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.Pagination", len)?;
        if let Some(v) = self.page_size.as_ref() {
            struct_ser.serialize_field("pageSize", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field(
                    "pageToken",
                    crate::_serde::base64::encode(&v).as_str(),
                )?;
        }
        if self.ordering != 0 {
            let v = PaginationOrdering::try_from(self.ordering)
                .map_err(|_| serde::ser::Error::custom(
                    format!("Invalid variant {}", self.ordering),
                ))?;
            struct_ser.serialize_field("ordering", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Pagination {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "ordering",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Ordering,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "ordering" => Ok(GeneratedField::Ordering),
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
            type Value = Pagination;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.Pagination")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<Pagination, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Ordering => {
                            if ordering__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ordering"));
                            }
                            ordering__ = Some(
                                map_.next_value::<PaginationOrdering>()? as i32,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Pagination {
                    page_size: page_size__,
                    page_token: page_token__,
                    ordering: ordering__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.Pagination", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PaginationOrdering {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Ascending => "PAGINATION_ORDERING_ASCENDING",
            Self::Descending => "PAGINATION_ORDERING_DESCENDING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PaginationOrdering {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PAGINATION_ORDERING_ASCENDING",
            "PAGINATION_ORDERING_DESCENDING",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PaginationOrdering;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", & FIELDS)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Signed(v),
                            &self,
                        )
                    })
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Unsigned(v),
                            &self,
                        )
                    })
            }
            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PAGINATION_ORDERING_ASCENDING" => Ok(PaginationOrdering::Ascending),
                    "PAGINATION_ORDERING_DESCENDING" => {
                        Ok(PaginationOrdering::Descending)
                    }
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SenderFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.SenderFilter", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SenderFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
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
            type Value = SenderFilter;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.SenderFilter")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SenderFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(SenderFilter { address: address__ })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.SenderFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransactionFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if !self.terms.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.TransactionFilter", len)?;
        if !self.terms.is_empty() {
            struct_ser.serialize_field("terms", &self.terms)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["terms"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Terms,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "terms" => Ok(GeneratedField::Terms),
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
            type Value = TransactionFilter;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.TransactionFilter")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<TransactionFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut terms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Terms => {
                            if terms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terms"));
                            }
                            terms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TransactionFilter {
                    terms: terms__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.TransactionFilter",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for TransactionItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.cursor.is_some() {
            len += 1;
        }
        if self.transaction.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.TransactionItem", len)?;
        if let Some(v) = self.cursor.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("cursor", crate::_serde::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.transaction.as_ref() {
            struct_ser.serialize_field("transaction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["cursor", "transaction"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cursor,
            Transaction,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cursor" => Ok(GeneratedField::Cursor),
                            "transaction" => Ok(GeneratedField::Transaction),
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
            type Value = TransactionItem;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.TransactionItem")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<TransactionItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut cursor__ = None;
                let mut transaction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cursor => {
                            if cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cursor"));
                            }
                            cursor__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Transaction => {
                            if transaction__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("transaction"),
                                );
                            }
                            transaction__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TransactionItem {
                    cursor: cursor__,
                    transaction: transaction__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.TransactionItem",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for TransactionLiteral {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.polarity.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.TransactionLiteral", len)?;
        if let Some(v) = self.polarity.as_ref() {
            match v {
                transaction_literal::Polarity::Include(v) => {
                    struct_ser.serialize_field("include", v)?;
                }
                transaction_literal::Polarity::Exclude(v) => {
                    struct_ser.serialize_field("exclude", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionLiteral {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["include", "exclude"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Include,
            Exclude,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "include" => Ok(GeneratedField::Include),
                            "exclude" => Ok(GeneratedField::Exclude),
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
            type Value = TransactionLiteral;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.TransactionLiteral")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<TransactionLiteral, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut polarity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Include => {
                            if polarity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("include"));
                            }
                            polarity__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(transaction_literal::Polarity::Include);
                        }
                        GeneratedField::Exclude => {
                            if polarity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exclude"));
                            }
                            polarity__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(transaction_literal::Polarity::Exclude);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TransactionLiteral {
                    polarity: polarity__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.TransactionLiteral",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for TransactionPredicate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.predicate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.TransactionPredicate", len)?;
        if let Some(v) = self.predicate.as_ref() {
            match v {
                transaction_predicate::Predicate::Sender(v) => {
                    struct_ser.serialize_field("sender", v)?;
                }
                transaction_predicate::Predicate::AffectedAddress(v) => {
                    struct_ser.serialize_field("affectedAddress", v)?;
                }
                transaction_predicate::Predicate::AffectedObject(v) => {
                    struct_ser.serialize_field("affectedObject", v)?;
                }
                transaction_predicate::Predicate::MoveCall(v) => {
                    struct_ser.serialize_field("moveCall", v)?;
                }
                transaction_predicate::Predicate::EmitModule(v) => {
                    struct_ser.serialize_field("emitModule", v)?;
                }
                transaction_predicate::Predicate::EventType(v) => {
                    struct_ser.serialize_field("eventType", v)?;
                }
                transaction_predicate::Predicate::EventStreamHead(v) => {
                    struct_ser.serialize_field("eventStreamHead", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionPredicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "affected_address",
            "affectedAddress",
            "affected_object",
            "affectedObject",
            "move_call",
            "moveCall",
            "emit_module",
            "emitModule",
            "event_type",
            "eventType",
            "event_stream_head",
            "eventStreamHead",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            AffectedAddress,
            AffectedObject,
            MoveCall,
            EmitModule,
            EventType,
            EventStreamHead,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "affectedAddress" | "affected_address" => {
                                Ok(GeneratedField::AffectedAddress)
                            }
                            "affectedObject" | "affected_object" => {
                                Ok(GeneratedField::AffectedObject)
                            }
                            "moveCall" | "move_call" => Ok(GeneratedField::MoveCall),
                            "emitModule" | "emit_module" => {
                                Ok(GeneratedField::EmitModule)
                            }
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "eventStreamHead" | "event_stream_head" => {
                                Ok(GeneratedField::EventStreamHead)
                            }
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
            type Value = TransactionPredicate;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.TransactionPredicate")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<TransactionPredicate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut predicate__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(transaction_predicate::Predicate::Sender);
                        }
                        GeneratedField::AffectedAddress => {
                            if predicate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("affectedAddress"),
                                );
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(transaction_predicate::Predicate::AffectedAddress);
                        }
                        GeneratedField::AffectedObject => {
                            if predicate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("affectedObject"),
                                );
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(transaction_predicate::Predicate::AffectedObject);
                        }
                        GeneratedField::MoveCall => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moveCall"));
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(transaction_predicate::Predicate::MoveCall);
                        }
                        GeneratedField::EmitModule => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emitModule"));
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(transaction_predicate::Predicate::EmitModule);
                        }
                        GeneratedField::EventType => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(transaction_predicate::Predicate::EventType);
                        }
                        GeneratedField::EventStreamHead => {
                            if predicate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("eventStreamHead"),
                                );
                            }
                            predicate__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(transaction_predicate::Predicate::EventStreamHead);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TransactionPredicate {
                    predicate: predicate__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.TransactionPredicate",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for TransactionTerm {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if !self.literals.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.TransactionTerm", len)?;
        if !self.literals.is_empty() {
            struct_ser.serialize_field("literals", &self.literals)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionTerm {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["literals"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Literals,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "literals" => Ok(GeneratedField::Literals),
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
            type Value = TransactionTerm;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.TransactionTerm")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<TransactionTerm, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut literals__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Literals => {
                            if literals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("literals"));
                            }
                            literals__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TransactionTerm {
                    literals: literals__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.TransactionTerm",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
