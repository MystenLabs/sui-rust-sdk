//! BCS encoding/decoding utilities.

use std::borrow::Cow;

use base64ct::Base64;
use base64ct::Encoding;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::Error;

/// A wrapper for BCS-encoded values.
///
/// - Deserializes from a Base64-encoded BCS string.
/// - Serializes to a Base64-encoded BCS string.
pub struct Bcs<T>(pub T);

impl<'de, T: DeserializeOwned> Deserialize<'de> for Bcs<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let b64 = <Cow<'_, str>>::deserialize(deserializer)?;
        let bytes = Base64::decode_vec(&b64).map_err(serde::de::Error::custom)?;
        let value = bcs::from_bytes(&bytes).map_err(serde::de::Error::custom)?;
        Ok(Bcs(value))
    }
}

impl<T: Serialize> Serialize for Bcs<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = bcs::to_bytes(&self.0).map_err(serde::ser::Error::custom)?;
        let b64 = Base64::encode_string(&bytes);
        b64.serialize(serializer)
    }
}

/// Raw BCS bytes that can be deserialized into any type.
///
/// This type deserializes from a Base64-encoded string and stores the raw bytes.
/// Use `deserialize::<T>()` to convert to a concrete type.
#[derive(Debug, Clone)]
pub struct BcsBytes(pub Vec<u8>);

impl BcsBytes {
    /// Deserialize the BCS bytes into a concrete type.
    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T, Error> {
        bcs::from_bytes(&self.0).map_err(|e| Error::Deserialization(format!("bcs decode: {e}")))
    }
}

impl<'de> Deserialize<'de> for BcsBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let b64 = <Cow<'_, str>>::deserialize(deserializer)?;
        let bytes = Base64::decode_vec(&b64).map_err(serde::de::Error::custom)?;
        Ok(BcsBytes(bytes))
    }
}
