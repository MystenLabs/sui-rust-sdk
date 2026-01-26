//! BCS decoding utilities.

use std::borrow::Cow;

use base64ct::Base64;
use base64ct::Encoding;
use serde::Deserialize;
use serde::de::DeserializeOwned;

/// A wrapper for BCS-decoded values.
///
/// Deserializes from a Base64-encoded BCS string.
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
