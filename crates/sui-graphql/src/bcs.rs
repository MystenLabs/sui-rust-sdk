//! BCS decoding utilities.

use base64ct::Base64;
use base64ct::Encoding;
use serde::de::DeserializeOwned;

use crate::error::Error;

/// A wrapper for BCS-decoded values.
///
/// Provides a convenient way to decode Base64-encoded BCS data.
pub struct Bcs<T>(T);

impl<T: DeserializeOwned> Bcs<T> {
    /// Decode a Base64-encoded BCS string into `T`.
    pub fn decode(base64_data: &str) -> Result<Self, Error> {
        let bytes = Base64::decode_vec(base64_data)?;
        let value = bcs::from_bytes(&bytes)?;
        Ok(Bcs(value))
    }
}

impl<T> Bcs<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
