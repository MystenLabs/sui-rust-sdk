#![cfg_attr(doc_cfg, feature(doc_cfg))]

pub mod types;

#[cfg(feature = "hash")]
pub mod hash;

#[cfg(feature = "serde")]
mod _serde {
    use base64ct::{Base64, Encoding};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_with::{DeserializeAs, SerializeAs};

    pub(crate) type ReadableDisplay =
        ::serde_with::As<::serde_with::IfIsHumanReadable<::serde_with::DisplayFromStr>>;

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
            let b64: std::borrow::Cow<'de, str> = Deserialize::deserialize(deserializer)?;
            let bytes = Base64::decode_vec(&b64).map_err(serde::de::Error::custom)?;
            let length = bytes.len();
            T::try_from(bytes).map_err(|_| {
                serde::de::Error::custom(format_args!(
                    "Can't convert a Byte Vector of length {length} to the output type."
                ))
            })
        }
    }
}
