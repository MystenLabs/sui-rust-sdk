#![allow(clippy::uninlined_format_args)]

// Include the generated proto definitions
include!("../../../generated/sui.rpc.v2.rs");

// Include generated field info impls
include!("../../../generated/sui.rpc.v2.field_info.rs");

// Include generated serde impls
include!("../../../generated/sui.rpc.v2.serde.rs");
include!("../../../generated/sui.rpc.v2.accessors.rs");

pub use descriptor::FILE_DESCRIPTOR_SET;
mod descriptor {
    /// Byte encoded FILE_DESCRIPTOR_SET.
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        include_bytes!("../../../generated/sui.rpc.v2.fds.bin");

    #[cfg(test)]
    mod tests {
        use super::FILE_DESCRIPTOR_SET;
        use prost::Message as _;

        #[test]
        fn file_descriptor_set_is_valid() {
            prost_types::FileDescriptorSet::decode(FILE_DESCRIPTOR_SET).unwrap();
        }
    }
}

#[cfg(test)]
mod proptests;

mod balance_change;
mod checkpoint;
mod effects;
mod epoch;
mod events;
mod executed_transaction;
mod execution_status;
mod ledger_service;
mod move_package_service;
mod name_service;
mod object;
mod signatures;
mod transaction;
mod transaction_execution_service;

//
// Bcs
//

impl Bcs {
    pub fn serialize<T: serde::Serialize>(value: &T) -> Result<Self, bcs::Error> {
        bcs::to_bytes(value).map(|bcs| Self {
            name: None,
            value: Some(bcs.into()),
        })
    }

    pub fn deserialize<'de, T: serde::Deserialize<'de>>(&'de self) -> Result<T, bcs::Error> {
        bcs::from_bytes(self.value.as_deref().unwrap_or(&[]))
    }
}

impl From<Vec<u8>> for Bcs {
    fn from(value: Vec<u8>) -> Self {
        Self {
            name: None,
            value: Some(value.into()),
        }
    }
}

impl From<&Bcs> for Vec<u8> {
    fn from(value: &Bcs) -> Self {
        value
            .value
            .as_ref()
            .map(|bytes| bytes.to_vec())
            .unwrap_or_default()
    }
}

impl From<Bcs> for Vec<u8> {
    fn from(value: Bcs) -> Self {
        value
            .value
            .as_ref()
            .map(|bytes| bytes.to_vec())
            .unwrap_or_default()
    }
}

impl From<prost::bytes::Bytes> for Bcs {
    fn from(value: prost::bytes::Bytes) -> Self {
        Self {
            name: None,
            value: Some(value),
        }
    }
}

impl From<&Bcs> for prost::bytes::Bytes {
    fn from(value: &Bcs) -> Self {
        value.value.clone().unwrap_or_default()
    }
}

impl From<Bcs> for prost::bytes::Bytes {
    fn from(value: Bcs) -> Self {
        value.value.unwrap_or_default()
    }
}

impl AsRef<str> for ErrorReason {
    fn as_ref(&self) -> &str {
        self.as_str_name()
    }
}

impl From<ErrorReason> for String {
    fn from(value: ErrorReason) -> Self {
        value.as_ref().into()
    }
}
