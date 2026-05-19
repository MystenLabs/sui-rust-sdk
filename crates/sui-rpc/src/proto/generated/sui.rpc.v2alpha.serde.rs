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
        if self.checkpoint.is_some() {
            len += 1;
        }
        if self.watermark.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.CheckpointItem", len)?;
        if let Some(v) = self.checkpoint.as_ref() {
            struct_ser.serialize_field("checkpoint", v)?;
        }
        if let Some(v) = self.watermark.as_ref() {
            struct_ser.serialize_field("watermark", v)?;
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
        const FIELDS: &[&str] = &["checkpoint", "watermark"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Checkpoint,
            Watermark,
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
                            "checkpoint" => Ok(GeneratedField::Checkpoint),
                            "watermark" => Ok(GeneratedField::Watermark),
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
                let mut checkpoint__ = None;
                let mut watermark__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Checkpoint => {
                            if checkpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkpoint"));
                            }
                            checkpoint__ = map_.next_value()?;
                        }
                        GeneratedField::Watermark => {
                            if watermark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watermark"));
                            }
                            watermark__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CheckpointItem {
                    checkpoint: checkpoint__,
                    watermark: watermark__,
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
        if self.transaction_index.is_some() {
            len += 1;
        }
        if self.watermark.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.EventItem", len)?;
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
        if let Some(v) = self.transaction_index.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("transactionIndex", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.watermark.as_ref() {
            struct_ser.serialize_field("watermark", v)?;
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
            "checkpoint",
            "event_index",
            "eventIndex",
            "transaction_digest",
            "transactionDigest",
            "event",
            "transaction_index",
            "transactionIndex",
            "watermark",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Checkpoint,
            EventIndex,
            TransactionDigest,
            Event,
            TransactionIndex,
            Watermark,
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
                            "checkpoint" => Ok(GeneratedField::Checkpoint),
                            "eventIndex" | "event_index" => {
                                Ok(GeneratedField::EventIndex)
                            }
                            "transactionDigest" | "transaction_digest" => {
                                Ok(GeneratedField::TransactionDigest)
                            }
                            "event" => Ok(GeneratedField::Event),
                            "transactionIndex" | "transaction_index" => {
                                Ok(GeneratedField::TransactionIndex)
                            }
                            "watermark" => Ok(GeneratedField::Watermark),
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
                let mut checkpoint__ = None;
                let mut event_index__ = None;
                let mut transaction_digest__ = None;
                let mut event__ = None;
                let mut transaction_index__ = None;
                let mut watermark__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                        GeneratedField::TransactionIndex => {
                            if transaction_index__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("transactionIndex"),
                                );
                            }
                            transaction_index__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Watermark => {
                            if watermark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watermark"));
                            }
                            watermark__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EventItem {
                    checkpoint: checkpoint__,
                    event_index: event_index__,
                    transaction_digest: transaction_digest__,
                    event: event__,
                    transaction_index: transaction_index__,
                    watermark: watermark__,
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
impl serde::Serialize for GetCheckpointObjectProofRequest {
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
        if self.checkpoint.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.GetCheckpointObjectProofRequest", len)?;
        if let Some(v) = self.object_id.as_ref() {
            struct_ser.serialize_field("objectId", v)?;
        }
        if let Some(v) = self.checkpoint.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("checkpoint", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCheckpointObjectProofRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["object_id", "objectId", "checkpoint"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectId,
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
                            "objectId" | "object_id" => Ok(GeneratedField::ObjectId),
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
            type Value = GetCheckpointObjectProofRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct sui.rpc.v2alpha.GetCheckpointObjectProofRequest")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<GetCheckpointObjectProofRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut object_id__ = None;
                let mut checkpoint__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ObjectId => {
                            if object_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectId"));
                            }
                            object_id__ = map_.next_value()?;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetCheckpointObjectProofRequest {
                    object_id: object_id__,
                    checkpoint: checkpoint__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.GetCheckpointObjectProofRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for GetCheckpointObjectProofResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.checkpoint_summary.is_some() {
            len += 1;
        }
        if self.proof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.GetCheckpointObjectProofResponse", len)?;
        if let Some(v) = self.checkpoint_summary.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field(
                    "checkpointSummary",
                    crate::_serde::base64::encode(&v).as_str(),
                )?;
        }
        if let Some(v) = self.proof.as_ref() {
            match v {
                get_checkpoint_object_proof_response::Proof::Inclusion(v) => {
                    struct_ser.serialize_field("inclusion", v)?;
                }
                get_checkpoint_object_proof_response::Proof::NonInclusion(v) => {
                    struct_ser.serialize_field("nonInclusion", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCheckpointObjectProofResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "checkpoint_summary",
            "checkpointSummary",
            "inclusion",
            "non_inclusion",
            "nonInclusion",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CheckpointSummary,
            Inclusion,
            NonInclusion,
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
                            "checkpointSummary" | "checkpoint_summary" => {
                                Ok(GeneratedField::CheckpointSummary)
                            }
                            "inclusion" => Ok(GeneratedField::Inclusion),
                            "nonInclusion" | "non_inclusion" => {
                                Ok(GeneratedField::NonInclusion)
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
            type Value = GetCheckpointObjectProofResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct sui.rpc.v2alpha.GetCheckpointObjectProofResponse")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<GetCheckpointObjectProofResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut checkpoint_summary__ = None;
                let mut proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CheckpointSummary => {
                            if checkpoint_summary__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("checkpointSummary"),
                                );
                            }
                            checkpoint_summary__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Inclusion => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inclusion"));
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(
                                    get_checkpoint_object_proof_response::Proof::Inclusion,
                                );
                        }
                        GeneratedField::NonInclusion => {
                            if proof__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("nonInclusion"),
                                );
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(
                                    get_checkpoint_object_proof_response::Proof::NonInclusion,
                                );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetCheckpointObjectProofResponse {
                    checkpoint_summary: checkpoint_summary__,
                    proof: proof__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.GetCheckpointObjectProofResponse",
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
        if self.options.is_some() {
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
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
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
            "options",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadMask,
            StartCheckpoint,
            EndCheckpoint,
            Filter,
            Options,
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
                            "options" => Ok(GeneratedField::Options),
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
                let mut options__ = None;
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
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map_.next_value()?;
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
                    options: options__,
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
                list_checkpoints_response::Response::Watermark(v) => {
                    struct_ser.serialize_field("watermark", v)?;
                }
                list_checkpoints_response::Response::End(v) => {
                    struct_ser.serialize_field("end", v)?;
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
        const FIELDS: &[&str] = &["item", "watermark", "end"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Item,
            Watermark,
            End,
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
                            "watermark" => Ok(GeneratedField::Watermark),
                            "end" => Ok(GeneratedField::End),
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
                        GeneratedField::Watermark => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watermark"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_checkpoints_response::Response::Watermark);
                        }
                        GeneratedField::End => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_checkpoints_response::Response::End);
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
        if self.options.is_some() {
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
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
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
            "options",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadMask,
            StartCheckpoint,
            EndCheckpoint,
            Filter,
            Options,
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
                            "options" => Ok(GeneratedField::Options),
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
                let mut options__ = None;
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
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map_.next_value()?;
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
                    options: options__,
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
                list_events_response::Response::Watermark(v) => {
                    struct_ser.serialize_field("watermark", v)?;
                }
                list_events_response::Response::End(v) => {
                    struct_ser.serialize_field("end", v)?;
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
        const FIELDS: &[&str] = &["item", "watermark", "end"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Item,
            Watermark,
            End,
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
                            "watermark" => Ok(GeneratedField::Watermark),
                            "end" => Ok(GeneratedField::End),
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
                        GeneratedField::Watermark => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watermark"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_events_response::Response::Watermark);
                        }
                        GeneratedField::End => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_events_response::Response::End);
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
        if self.options.is_some() {
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
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
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
            "options",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadMask,
            StartCheckpoint,
            EndCheckpoint,
            Filter,
            Options,
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
                            "options" => Ok(GeneratedField::Options),
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
                let mut options__ = None;
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
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map_.next_value()?;
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
                    options: options__,
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
                list_transactions_response::Response::Watermark(v) => {
                    struct_ser.serialize_field("watermark", v)?;
                }
                list_transactions_response::Response::End(v) => {
                    struct_ser.serialize_field("end", v)?;
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
        const FIELDS: &[&str] = &["item", "watermark", "end"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Item,
            Watermark,
            End,
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
                            "watermark" => Ok(GeneratedField::Watermark),
                            "end" => Ok(GeneratedField::End),
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
                        GeneratedField::Watermark => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watermark"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_transactions_response::Response::Watermark);
                        }
                        GeneratedField::End => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            response__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(list_transactions_response::Response::End);
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
impl serde::Serialize for MerkleNeighbourLeaf {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.leaf.is_some() {
            len += 1;
        }
        if self.merkle_proof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.MerkleNeighbourLeaf", len)?;
        if let Some(v) = self.leaf.as_ref() {
            struct_ser.serialize_field("leaf", v)?;
        }
        if let Some(v) = self.merkle_proof.as_ref() {
            struct_ser.serialize_field("merkleProof", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleNeighbourLeaf {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["leaf", "merkle_proof", "merkleProof"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Leaf,
            MerkleProof,
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
                            "leaf" => Ok(GeneratedField::Leaf),
                            "merkleProof" | "merkle_proof" => {
                                Ok(GeneratedField::MerkleProof)
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
            type Value = MerkleNeighbourLeaf;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.MerkleNeighbourLeaf")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MerkleNeighbourLeaf, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut leaf__ = None;
                let mut merkle_proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Leaf => {
                            if leaf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leaf"));
                            }
                            leaf__ = map_.next_value()?;
                        }
                        GeneratedField::MerkleProof => {
                            if merkle_proof__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("merkleProof"),
                                );
                            }
                            merkle_proof__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MerkleNeighbourLeaf {
                    leaf: leaf__,
                    merkle_proof: merkle_proof__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.MerkleNeighbourLeaf",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MerkleNode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.node.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.MerkleNode", len)?;
        if let Some(v) = self.node.as_ref() {
            match v {
                merkle_node::Node::Empty(v) => {
                    struct_ser
                        .serialize_field("empty", &crate::_serde::EmptySerializer(v))?;
                }
                merkle_node::Node::Digest(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser
                        .serialize_field(
                            "digest",
                            crate::_serde::base64::encode(&v).as_str(),
                        )?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleNode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["empty", "digest"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Empty,
            Digest,
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
                            "empty" => Ok(GeneratedField::Empty),
                            "digest" => Ok(GeneratedField::Digest),
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
            type Value = MerkleNode;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.MerkleNode")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MerkleNode, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Empty => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("empty"));
                            }
                            node__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::EmptyDeserializer>,
                                >()?
                                .map(|x| merkle_node::Node::Empty(x.0));
                        }
                        GeneratedField::Digest => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digest"));
                            }
                            node__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| merkle_node::Node::Digest(x.0));
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MerkleNode { node: node__ })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.MerkleNode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MerkleNonInclusionProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.index.is_some() {
            len += 1;
        }
        if self.left_leaf.is_some() {
            len += 1;
        }
        if self.right_leaf.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.MerkleNonInclusionProof", len)?;
        if let Some(v) = self.index.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("index", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.left_leaf.as_ref() {
            struct_ser.serialize_field("leftLeaf", v)?;
        }
        if let Some(v) = self.right_leaf.as_ref() {
            struct_ser.serialize_field("rightLeaf", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleNonInclusionProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "index",
            "left_leaf",
            "leftLeaf",
            "right_leaf",
            "rightLeaf",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Index,
            LeftLeaf,
            RightLeaf,
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
                            "index" => Ok(GeneratedField::Index),
                            "leftLeaf" | "left_leaf" => Ok(GeneratedField::LeftLeaf),
                            "rightLeaf" | "right_leaf" => Ok(GeneratedField::RightLeaf),
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
            type Value = MerkleNonInclusionProof;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.MerkleNonInclusionProof")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MerkleNonInclusionProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut index__ = None;
                let mut left_leaf__ = None;
                let mut right_leaf__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::LeftLeaf => {
                            if left_leaf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leftLeaf"));
                            }
                            left_leaf__ = map_.next_value()?;
                        }
                        GeneratedField::RightLeaf => {
                            if right_leaf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rightLeaf"));
                            }
                            right_leaf__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MerkleNonInclusionProof {
                    index: index__,
                    left_leaf: left_leaf__,
                    right_leaf: right_leaf__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.MerkleNonInclusionProof",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MerkleProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.MerkleProof", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["path"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
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
                            "path" => Ok(GeneratedField::Path),
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
            type Value = MerkleProof;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.MerkleProof")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MerkleProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MerkleProof {
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.MerkleProof", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for OcsInclusionProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.object_ref.is_some() {
            len += 1;
        }
        if self.merkle_proof.is_some() {
            len += 1;
        }
        if self.leaf_index.is_some() {
            len += 1;
        }
        if self.tree_root.is_some() {
            len += 1;
        }
        if self.object_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.OcsInclusionProof", len)?;
        if let Some(v) = self.object_ref.as_ref() {
            struct_ser.serialize_field("objectRef", v)?;
        }
        if let Some(v) = self.merkle_proof.as_ref() {
            struct_ser.serialize_field("merkleProof", v)?;
        }
        if let Some(v) = self.leaf_index.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("leafIndex", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.tree_root.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field(
                    "treeRoot",
                    crate::_serde::base64::encode(&v).as_str(),
                )?;
        }
        if let Some(v) = self.object_data.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field(
                    "objectData",
                    crate::_serde::base64::encode(&v).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OcsInclusionProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_ref",
            "objectRef",
            "merkle_proof",
            "merkleProof",
            "leaf_index",
            "leafIndex",
            "tree_root",
            "treeRoot",
            "object_data",
            "objectData",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectRef,
            MerkleProof,
            LeafIndex,
            TreeRoot,
            ObjectData,
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
                            "objectRef" | "object_ref" => Ok(GeneratedField::ObjectRef),
                            "merkleProof" | "merkle_proof" => {
                                Ok(GeneratedField::MerkleProof)
                            }
                            "leafIndex" | "leaf_index" => Ok(GeneratedField::LeafIndex),
                            "treeRoot" | "tree_root" => Ok(GeneratedField::TreeRoot),
                            "objectData" | "object_data" => {
                                Ok(GeneratedField::ObjectData)
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
            type Value = OcsInclusionProof;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.OcsInclusionProof")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<OcsInclusionProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut object_ref__ = None;
                let mut merkle_proof__ = None;
                let mut leaf_index__ = None;
                let mut tree_root__ = None;
                let mut object_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ObjectRef => {
                            if object_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectRef"));
                            }
                            object_ref__ = map_.next_value()?;
                        }
                        GeneratedField::MerkleProof => {
                            if merkle_proof__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("merkleProof"),
                                );
                            }
                            merkle_proof__ = map_.next_value()?;
                        }
                        GeneratedField::LeafIndex => {
                            if leaf_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leafIndex"));
                            }
                            leaf_index__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::TreeRoot => {
                            if tree_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treeRoot"));
                            }
                            tree_root__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::ObjectData => {
                            if object_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectData"));
                            }
                            object_data__ = map_
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
                Ok(OcsInclusionProof {
                    object_ref: object_ref__,
                    merkle_proof: merkle_proof__,
                    leaf_index: leaf_index__,
                    tree_root: tree_root__,
                    object_data: object_data__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.OcsInclusionProof",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for OcsNonInclusionProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.non_inclusion_proof.is_some() {
            len += 1;
        }
        if self.tree_root.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.OcsNonInclusionProof", len)?;
        if let Some(v) = self.non_inclusion_proof.as_ref() {
            struct_ser.serialize_field("nonInclusionProof", v)?;
        }
        if let Some(v) = self.tree_root.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field(
                    "treeRoot",
                    crate::_serde::base64::encode(&v).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OcsNonInclusionProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "non_inclusion_proof",
            "nonInclusionProof",
            "tree_root",
            "treeRoot",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NonInclusionProof,
            TreeRoot,
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
                            "nonInclusionProof" | "non_inclusion_proof" => {
                                Ok(GeneratedField::NonInclusionProof)
                            }
                            "treeRoot" | "tree_root" => Ok(GeneratedField::TreeRoot),
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
            type Value = OcsNonInclusionProof;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.OcsNonInclusionProof")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<OcsNonInclusionProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut non_inclusion_proof__ = None;
                let mut tree_root__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NonInclusionProof => {
                            if non_inclusion_proof__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("nonInclusionProof"),
                                );
                            }
                            non_inclusion_proof__ = map_.next_value()?;
                        }
                        GeneratedField::TreeRoot => {
                            if tree_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treeRoot"));
                            }
                            tree_root__ = map_
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
                Ok(OcsNonInclusionProof {
                    non_inclusion_proof: non_inclusion_proof__,
                    tree_root: tree_root__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.OcsNonInclusionProof",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for Ordering {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Ascending => "ORDERING_ASCENDING",
            Self::Descending => "ORDERING_DESCENDING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Ordering {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["ORDERING_ASCENDING", "ORDERING_DESCENDING"];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Ordering;
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
                    "ORDERING_ASCENDING" => Ok(Ordering::Ascending),
                    "ORDERING_DESCENDING" => Ok(Ordering::Descending),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for QueryEnd {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.reason != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.QueryEnd", len)?;
        if self.reason != 0 {
            let v = QueryEndReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(
                    format!("Invalid variant {}", self.reason),
                ))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryEnd {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["reason"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
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
                            "reason" => Ok(GeneratedField::Reason),
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
            type Value = QueryEnd;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.QueryEnd")
            }
            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryEnd, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value::<QueryEndReason>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryEnd {
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.QueryEnd", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryEndReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "QUERY_END_REASON_UNSPECIFIED",
            Self::ItemLimit => "QUERY_END_REASON_ITEM_LIMIT",
            Self::ScanLimit => "QUERY_END_REASON_SCAN_LIMIT",
            Self::CheckpointBound => "QUERY_END_REASON_CHECKPOINT_BOUND",
            Self::CursorBound => "QUERY_END_REASON_CURSOR_BOUND",
            Self::LedgerTip => "QUERY_END_REASON_LEDGER_TIP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for QueryEndReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "QUERY_END_REASON_UNSPECIFIED",
            "QUERY_END_REASON_ITEM_LIMIT",
            "QUERY_END_REASON_SCAN_LIMIT",
            "QUERY_END_REASON_CHECKPOINT_BOUND",
            "QUERY_END_REASON_CURSOR_BOUND",
            "QUERY_END_REASON_LEDGER_TIP",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryEndReason;
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
                    "QUERY_END_REASON_UNSPECIFIED" => Ok(QueryEndReason::Unspecified),
                    "QUERY_END_REASON_ITEM_LIMIT" => Ok(QueryEndReason::ItemLimit),
                    "QUERY_END_REASON_SCAN_LIMIT" => Ok(QueryEndReason::ScanLimit),
                    "QUERY_END_REASON_CHECKPOINT_BOUND" => {
                        Ok(QueryEndReason::CheckpointBound)
                    }
                    "QUERY_END_REASON_CURSOR_BOUND" => Ok(QueryEndReason::CursorBound),
                    "QUERY_END_REASON_LEDGER_TIP" => Ok(QueryEndReason::LedgerTip),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for QueryOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.limit_items.is_some() {
            len += 1;
        }
        if self.after.is_some() {
            len += 1;
        }
        if self.before.is_some() {
            len += 1;
        }
        if self.ordering != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.QueryOptions", len)?;
        if let Some(v) = self.limit_items.as_ref() {
            struct_ser.serialize_field("limitItems", v)?;
        }
        if let Some(v) = self.after.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("after", crate::_serde::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.before.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("before", crate::_serde::base64::encode(&v).as_str())?;
        }
        if self.ordering != 0 {
            let v = Ordering::try_from(self.ordering)
                .map_err(|_| serde::ser::Error::custom(
                    format!("Invalid variant {}", self.ordering),
                ))?;
            struct_ser.serialize_field("ordering", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limit_items",
            "limitItems",
            "after",
            "before",
            "ordering",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LimitItems,
            After,
            Before,
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
                            "limitItems" | "limit_items" => {
                                Ok(GeneratedField::LimitItems)
                            }
                            "after" => Ok(GeneratedField::After),
                            "before" => Ok(GeneratedField::Before),
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
            type Value = QueryOptions;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.QueryOptions")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOptions, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut limit_items__ = None;
                let mut after__ = None;
                let mut before__ = None;
                let mut ordering__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LimitItems => {
                            if limit_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limitItems"));
                            }
                            limit_items__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::After => {
                            if after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("after"));
                            }
                            after__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Before => {
                            if before__.is_some() {
                                return Err(serde::de::Error::duplicate_field("before"));
                            }
                            before__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Ordering => {
                            if ordering__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ordering"));
                            }
                            ordering__ = Some(map_.next_value::<Ordering>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryOptions {
                    limit_items: limit_items__,
                    after: after__,
                    before: before__,
                    ordering: ordering__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.QueryOptions", FIELDS, GeneratedVisitor)
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
        if self.transaction.is_some() {
            len += 1;
        }
        if self.watermark.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.TransactionItem", len)?;
        if let Some(v) = self.transaction.as_ref() {
            struct_ser.serialize_field("transaction", v)?;
        }
        if let Some(v) = self.watermark.as_ref() {
            struct_ser.serialize_field("watermark", v)?;
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
        const FIELDS: &[&str] = &["transaction", "watermark"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transaction,
            Watermark,
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
                            "transaction" => Ok(GeneratedField::Transaction),
                            "watermark" => Ok(GeneratedField::Watermark),
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
                let mut transaction__ = None;
                let mut watermark__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Transaction => {
                            if transaction__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("transaction"),
                                );
                            }
                            transaction__ = map_.next_value()?;
                        }
                        GeneratedField::Watermark => {
                            if watermark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watermark"));
                            }
                            watermark__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TransactionItem {
                    transaction: transaction__,
                    watermark: watermark__,
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
impl serde::Serialize for Watermark {
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
        if self.checkpoint_hi.is_some() {
            len += 1;
        }
        if self.checkpoint_lo.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.Watermark", len)?;
        if let Some(v) = self.cursor.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("cursor", crate::_serde::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.checkpoint_hi.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("checkpointHi", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.checkpoint_lo.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field("checkpointLo", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Watermark {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cursor",
            "checkpoint_hi",
            "checkpointHi",
            "checkpoint_lo",
            "checkpointLo",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cursor,
            CheckpointHi,
            CheckpointLo,
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
                            "checkpointHi" | "checkpoint_hi" => {
                                Ok(GeneratedField::CheckpointHi)
                            }
                            "checkpointLo" | "checkpoint_lo" => {
                                Ok(GeneratedField::CheckpointLo)
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
            type Value = Watermark;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.Watermark")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<Watermark, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut cursor__ = None;
                let mut checkpoint_hi__ = None;
                let mut checkpoint_lo__ = None;
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
                        GeneratedField::CheckpointHi => {
                            if checkpoint_hi__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("checkpointHi"),
                                );
                            }
                            checkpoint_hi__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::CheckpointLo => {
                            if checkpoint_lo__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("checkpointLo"),
                                );
                            }
                            checkpoint_lo__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Watermark {
                    cursor: cursor__,
                    checkpoint_hi: checkpoint_hi__,
                    checkpoint_lo: checkpoint_lo__,
                })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.Watermark", FIELDS, GeneratedVisitor)
    }
}
