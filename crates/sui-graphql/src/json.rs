//! JSON utilities.

use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::error::Error;

/// Raw JSON value that can be deserialized into any type.
///
/// This type wraps a `serde_json::Value` and provides a `deserialize::<T>()` method
/// for converting to a concrete type.
#[derive(Debug, Clone)]
pub struct JsonValue(pub serde_json::Value);

impl JsonValue {
    /// Deserialize the JSON into a concrete type.
    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T, Error> {
        serde_json::from_value(self.0.clone())
            .map_err(|e| Error::Deserialization(format!("json decode: {e}")))
    }
}

impl<'de> Deserialize<'de> for JsonValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let json = serde_json::Value::deserialize(deserializer)?;
        Ok(JsonValue(json))
    }
}
