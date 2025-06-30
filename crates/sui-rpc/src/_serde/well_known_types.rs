use super::base64;
use super::BytesDeserialize;
use super::NumberDeserialize;
use prost_types::Any;
use prost_types::Duration;
use prost_types::FieldMask;
use prost_types::ListValue;
use prost_types::NullValue;
use prost_types::Struct;
use prost_types::Timestamp;
use prost_types::Value;

pub struct EmptySerializer<'a>(pub &'a ());

impl serde::Serialize for EmptySerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        serializer.serialize_map(None)?.end()
    }
}

pub struct EmptyDeserializer(pub ());

impl<'de> serde::Deserialize<'de> for EmptyDeserializer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ();

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an empty message")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                if map.next_key::<serde::de::IgnoredAny>()?.is_some() {
                    return Err(<A::Error as serde::de::Error>::invalid_length(
                        1,
                        &"an empty map",
                    ));
                }
                Ok(())
            }
        }

        deserializer.deserialize_map(Visitor).map(EmptyDeserializer)
    }
}

pub struct AnySerializer<'a>(pub &'a Any);

impl serde::Serialize for AnySerializer<'_> {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.0.type_url.is_empty() {
            len += 1;
        }
        if !self.0.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.Any", len)?;
        if !self.0.type_url.is_empty() {
            struct_ser.serialize_field("@type", &self.0.type_url)?;
        }
        if !self.0.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("value", base64::encode(&self.0.value).as_str())?;
        }
        struct_ser.end()
    }
}

pub struct AnyDeserializer(pub Any);

impl<'de> serde::Deserialize<'de> for AnyDeserializer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["type_url", "typeUrl", "@type", "value"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypeUrl,
            Value,
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

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "typeUrl" | "type_url" | "@type" => Ok(GeneratedField::TypeUrl),
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Any;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.Any")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Any, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut type_url__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value::<BytesDeserialize<_>>()?.0);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Any {
                    type_url: type_url__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        Ok(Self(deserializer.deserialize_struct(
            "google.protobuf.Any",
            FIELDS,
            GeneratedVisitor,
        )?))
    }
}

pub struct DurationSerializer<'a>(pub &'a Duration);

fn is_duration_valid(duration: &Duration) -> bool {
    duration.seconds >= -315_576_000_000 && duration.seconds <= 315_576_000_000
}

impl serde::Serialize for DurationSerializer<'_> {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if !is_duration_valid(self.0) {
            return Err(serde::ser::Error::custom(format!(
                "duration is invalid: d={:?}",
                self.0
            )));
        }

        serializer.serialize_str(&self.0.to_string())
    }
}

pub struct DurationDeserializer(pub Duration);

impl<'de> serde::Deserialize<'de> for DurationDeserializer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["seconds", "nanos"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seconds,
            Nanos,
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

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "seconds" => Ok(GeneratedField::Seconds),
                            "nanos" => Ok(GeneratedField::Nanos),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("google.protobuf.Duration")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.parse::<Duration>() {
                    Ok(val) if is_duration_valid(&val) => Ok(val),
                    Ok(_) | Err(_) => Err(E::invalid_value(
                        serde::de::Unexpected::Str(v),
                        &"a valid duration string",
                    )),
                }
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Duration, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut seconds__ = None;
                let mut nanos__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Seconds => {
                            if seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seconds"));
                            }
                            seconds__ = Some(map_.next_value::<NumberDeserialize<_>>()?.0);
                        }
                        GeneratedField::Nanos => {
                            if nanos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos__ = Some(map_.next_value::<NumberDeserialize<_>>()?.0);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Duration {
                    seconds: seconds__.unwrap_or_default(),
                    nanos: nanos__.unwrap_or_default(),
                })
            }
        }
        Ok(Self(deserializer.deserialize_any(GeneratedVisitor)?))
    }
}

pub struct FieldMaskSerializer<'a>(pub &'a FieldMask);

impl serde::Serialize for FieldMaskSerializer<'_> {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf = String::with_capacity(self.0.paths.iter().map(|path| path.len()).sum());
        let mut paths = self.0.paths.iter().peekable();

        while let Some(path) = paths.next() {
            let mut path_chars = path.chars().peekable();

            while let Some(chr) = path_chars.next() {
                match chr {
                    'A'..='Z' => {
                        return Err(<S::Error as serde::ser::Error>::custom(
                            "field mask element may not have upper-case letters",
                        ))
                    }
                    '_' => {
                        let Some(next_chr) =
                            path_chars.next().filter(|chr| chr.is_ascii_lowercase())
                        else {
                            return Err(<S::Error as serde::ser::Error>::custom(
                                "underscore in field mask element must be followed by lower-case letter",
                            ));
                        };
                        buf.push(next_chr.to_ascii_uppercase());
                    }
                    _ => buf.push(chr),
                }
            }

            if paths.peek().is_some() {
                buf.push(',');
            }
        }

        serializer.serialize_str(&buf)
    }
}

pub struct FieldMaskDeserializer(pub FieldMask);

impl<'de> serde::Deserialize<'de> for FieldMaskDeserializer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["paths"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Paths,
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

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "paths" => Ok(GeneratedField::Paths),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldMask;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("google.protobuf.FieldMask")
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                fn convert_path(path: &str) -> Result<String, &'static str> {
                    let underscores_required =
                        path.chars().filter(|chr| chr.is_ascii_uppercase()).count();

                    let mut buf = String::with_capacity(path.len() + underscores_required);

                    for chr in path.chars() {
                        match chr {
                            'A'..='Z' => {
                                buf.push('_');
                                buf.push(chr.to_ascii_lowercase());
                            }
                            '_' => return Err("field mask element may not contain underscores"),
                            'a'..='z' | '0'..='9' => buf.push(chr),
                            _ => {
                                return Err(
                                    "field mask element may not contain non ascii alphabetic letters or digits",
                                )
                            }
                        }
                    }

                    Ok(buf)
                }

                let paths = val
                    .split(',')
                    .map(|path| path.trim())
                    .filter(|path| !path.is_empty())
                    .map(convert_path)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|err| {
                        E::invalid_value(
                            serde::de::Unexpected::Str(val),
                            &&*format!("a valid fieldmask string ({err})"),
                        )
                    })?;

                Ok(FieldMask { paths })
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldMask, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut paths__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Paths => {
                            if paths__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paths"));
                            }
                            paths__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(FieldMask {
                    paths: paths__.unwrap_or_default(),
                })
            }
        }

        Ok(Self(deserializer.deserialize_any(GeneratedVisitor)?))
    }
}

pub struct ListValueSerializer<'a>(pub &'a ListValue);

impl serde::Serialize for ListValueSerializer<'_> {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.0.values.iter().map(ValueSerializer))
    }
}

pub struct ListValueDeserializer(pub ListValue);

impl<'de> serde::Deserialize<'de> for ListValueDeserializer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.ListValue")
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                deserialize_list_value(seq)
            }
        }

        Ok(Self(deserializer.deserialize_any(GeneratedVisitor)?))
    }
}

pub struct NullValueSerializer<'a>(pub &'a NullValue);

impl serde::Serialize for NullValueSerializer<'_> {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_none()
    }
}

pub struct NullValueDeserializer(pub NullValue);

impl<'de> serde::Deserialize<'de> for NullValueDeserializer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["NULL_VALUE"];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NullValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected a null value")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NullValue::NullValue)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
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
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NULL_VALUE" => Ok(NullValue::NullValue),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        Ok(Self(deserializer.deserialize_any(GeneratedVisitor)?))
    }
}

pub struct StructSerializer<'a>(pub &'a Struct);

impl serde::Serialize for StructSerializer<'_> {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(self.0.fields.len()))?;
        for (key, value) in &self.0.fields {
            map.serialize_entry(key, &ValueSerializer(value))?;
        }
        map.end()
    }
}

pub struct StructDeserializer(pub Struct);

impl<'de> serde::Deserialize<'de> for StructDeserializer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Struct;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("google.protobuf.Struct")
            }

            fn visit_map<V>(self, map: V) -> std::result::Result<Struct, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                deserialize_struct(map)
            }
        }
        Ok(Self(deserializer.deserialize_any(GeneratedVisitor)?))
    }
}

pub struct TimestampSerializer<'a>(pub &'a Timestamp);

impl serde::Serialize for TimestampSerializer<'_> {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

pub struct TimestampDeserializer(pub Timestamp);

impl<'de> serde::Deserialize<'de> for TimestampDeserializer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["seconds", "nanos"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seconds,
            Nanos,
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

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "seconds" => Ok(GeneratedField::Seconds),
                            "nanos" => Ok(GeneratedField::Nanos),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Timestamp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("google.protobuf.Timestamp")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.parse::<Timestamp>() {
                    Ok(val) => Ok(val),
                    Err(_) => Err(E::invalid_value(
                        serde::de::Unexpected::Str(v),
                        &"a valid timestamp string",
                    )),
                }
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Timestamp, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut seconds__ = None;
                let mut nanos__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Seconds => {
                            if seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seconds"));
                            }
                            seconds__ = Some(map_.next_value::<NumberDeserialize<_>>()?.0);
                        }
                        GeneratedField::Nanos => {
                            if nanos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos__ = Some(map_.next_value::<NumberDeserialize<_>>()?.0);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Timestamp {
                    seconds: seconds__.unwrap_or_default(),
                    nanos: nanos__.unwrap_or_default(),
                })
            }
        }
        Ok(Self(deserializer.deserialize_any(GeneratedVisitor)?))
    }
}

const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

pub struct ValueSerializer<'a>(pub &'a Value);

impl serde::Serialize for ValueSerializer<'_> {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use prost_types::value::Kind;
        match self.0.kind.as_ref() {
            Some(Kind::NullValue(_)) | None => serializer.serialize_none(),
            Some(Kind::NumberValue(val)) => {
                if val.is_nan() || val.is_infinite() {
                    return Err(serde::ser::Error::custom(format!(
                        "serializing a value::Kind::NumberValue, which is {val}, is not possible"
                    )));
                }
                if val.trunc() == *val {
                    let val_u64 = *val as u64;
                    if val_u64 <= MAX_SAFE_INTEGER {
                        serializer.serialize_u64(val_u64)
                    } else {
                        serializer.serialize_f64(*val)
                    }
                } else {
                    serializer.serialize_f64(*val)
                }
            }
            Some(Kind::StringValue(val)) => serializer.serialize_str(val),
            Some(Kind::BoolValue(val)) => serializer.serialize_bool(*val),
            Some(Kind::StructValue(val)) => StructSerializer(val).serialize(serializer),
            Some(Kind::ListValue(val)) => ListValueSerializer(val).serialize(serializer),
        }
    }
}

pub struct ValueDeserializer(pub Value);

impl<'de> serde::Deserialize<'de> for ValueDeserializer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use prost_types::value;
        use std::fmt;

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Value")
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value {
                    kind: Some(value::Kind::NullValue(0)),
                })
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value {
                    kind: Some(value::Kind::NullValue(0)),
                })
            }

            #[inline]
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value {
                    kind: Some(value::Kind::NumberValue(v as f64)),
                })
            }

            #[inline]
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value {
                    kind: Some(value::Kind::NumberValue(v as f64)),
                })
            }

            #[inline]
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value {
                    kind: Some(value::Kind::NumberValue(v)),
                })
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value {
                    kind: Some(value::Kind::StringValue(v.to_owned())),
                })
            }

            #[inline]
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value {
                    kind: Some(value::Kind::BoolValue(v)),
                })
            }

            #[inline]
            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let value = deserialize_struct(map)?;
                Ok(Value {
                    kind: Some(value::Kind::StructValue(value)),
                })
            }

            #[inline]
            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let value = deserialize_list_value(seq)?;
                Ok(Value {
                    kind: Some(value::Kind::ListValue(value)),
                })
            }
        }

        Ok(Self(deserializer.deserialize_any(Visitor)?))
    }
}

fn deserialize_list_value<'de, A>(mut seq: A) -> Result<ListValue, A::Error>
where
    A: serde::de::SeqAccess<'de>,
{
    let mut values = vec![];
    while let Some(value) = seq.next_element::<ValueDeserializer>()? {
        values.push(value.0);
    }
    Ok(ListValue { values })
}

fn deserialize_struct<'de, A>(mut map: A) -> Result<Struct, A::Error>
where
    A: serde::de::MapAccess<'de>,
{
    let mut fields = std::collections::BTreeMap::new();
    while let Some(key) = map.next_key::<String>()? {
        let value = map.next_value::<ValueDeserializer>()?;
        fields.insert(key, value.0);
    }
    Ok(Struct { fields })
}
