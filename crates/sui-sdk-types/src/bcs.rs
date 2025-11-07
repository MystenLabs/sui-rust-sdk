pub use bcs::Error;

pub trait FromBcs {
    fn from_bcs<'de>(bytes: &'de [u8]) -> Result<Self, Error>
    where
        Self: serde::de::Deserialize<'de>,
    {
        bcs::from_bytes(bytes)
    }

    fn from_bcs_base64(base64: &str) -> Result<Self, Error>
    where
        Self: serde::de::DeserializeOwned,
    {
        let bytes = <base64ct::Base64 as base64ct::Encoding>::decode_vec(base64)
            .map_err(|e| bcs::Error::Custom(format!("invalid base64: {e}")))?;
        Self::from_bcs(&bytes)
    }
}

impl<'de, T> FromBcs for T where T: serde::de::Deserialize<'de> {}

pub trait ToBcs: serde::ser::Serialize {
    fn to_bcs(&self) -> Result<Vec<u8>, Error> {
        bcs::to_bytes(self)
    }

    fn to_bcs_base64(&self) -> Result<String, Error> {
        let bytes = self.to_bcs()?;
        Ok(<base64ct::Base64 as base64ct::Encoding>::encode_string(
            &bytes,
        ))
    }
}

impl<T> ToBcs for T where T: serde::ser::Serialize {}

// Maybe have a name trait?
// pub trait BcsName;
