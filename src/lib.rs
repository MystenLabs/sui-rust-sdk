#![cfg_attr(doc_cfg, feature(doc_cfg))]

pub mod types;

#[cfg(feature = "hash")]
pub mod hash;

#[cfg(feature = "serde")]
mod _serde {
    use base64ct::{Base64, Encoding};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_with::{Bytes, DeserializeAs, SerializeAs};
    use std::borrow::Cow;

    pub(crate) type ReadableDisplay =
        ::serde_with::As<::serde_with::IfIsHumanReadable<::serde_with::DisplayFromStr>>;

    pub(crate) type ReadableBase64Encoded =
        ::serde_with::As<::serde_with::IfIsHumanReadable<Base64Encoded, ::serde_with::Bytes>>;

    pub(crate) struct Base64Encoded;

    impl<T: AsRef<[u8]>> SerializeAs<T> for Base64Encoded {
        fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let bytes = source.as_ref();
            let b64 = Base64::encode_string(bytes);
            b64.serialize(serializer)
        }
    }

    impl<'de, T: TryFrom<Vec<u8>>> DeserializeAs<'de, T> for Base64Encoded {
        fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
        {
            let b64: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
            let bytes = Base64::decode_vec(&b64).map_err(serde::de::Error::custom)?;
            let length = bytes.len();
            T::try_from(bytes).map_err(|_| {
                serde::de::Error::custom(format_args!(
                    "Can't convert a Byte Vector of length {length} to the output type."
                ))
            })
        }
    }

    /// Serializes a bitmap according to the roaring bitmap on-disk standard.
    /// <https://github.com/RoaringBitmap/RoaringFormatSpec>
    pub(crate) struct BinaryRoaringBitmap;

    impl SerializeAs<roaring::RoaringBitmap> for BinaryRoaringBitmap {
        fn serialize_as<S>(
            source: &roaring::RoaringBitmap,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut bytes = vec![];

            source
                .serialize_into(&mut bytes)
                .map_err(serde::ser::Error::custom)?;
            Bytes::serialize_as(&bytes, serializer)
        }
    }

    impl<'de> DeserializeAs<'de, roaring::RoaringBitmap> for BinaryRoaringBitmap {
        fn deserialize_as<D>(deserializer: D) -> Result<roaring::RoaringBitmap, D::Error>
        where
            D: Deserializer<'de>,
        {
            let bytes: Cow<'de, [u8]> = Bytes::deserialize_as(deserializer)?;
            roaring::RoaringBitmap::deserialize_from(&bytes[..]).map_err(serde::de::Error::custom)
        }
    }

    pub(crate) struct Base64RoaringBitmap;

    impl SerializeAs<roaring::RoaringBitmap> for Base64RoaringBitmap {
        fn serialize_as<S>(
            source: &roaring::RoaringBitmap,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut bytes = vec![];

            source
                .serialize_into(&mut bytes)
                .map_err(serde::ser::Error::custom)?;
            let b64 = Base64::encode_string(&bytes);
            b64.serialize(serializer)
        }
    }

    impl<'de> DeserializeAs<'de, roaring::RoaringBitmap> for Base64RoaringBitmap {
        fn deserialize_as<D>(deserializer: D) -> Result<roaring::RoaringBitmap, D::Error>
        where
            D: Deserializer<'de>,
        {
            let b64: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
            let bytes = Base64::decode_vec(&b64).map_err(serde::de::Error::custom)?;
            roaring::RoaringBitmap::deserialize_from(&bytes[..]).map_err(serde::de::Error::custom)
        }
    }
}
