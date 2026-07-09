#![allow(clippy::uninlined_format_args)]

use super::v2::_field_impls::ObjectReferenceFieldPathBuilder;
use super::v2::ObjectReference;

// Include the generated proto definitions
include!("../../../generated/sui.rpc.v2alpha.rs");

// Include generated field info impls
include!("../../../generated/sui.rpc.v2alpha.field_info.rs");

// Include generated serde impls
include!("../../../generated/sui.rpc.v2alpha.serde.rs");
include!("../../../generated/sui.rpc.v2alpha.accessors.rs");

mod proof_service;

pub use descriptor::FILE_DESCRIPTOR_SET;
mod descriptor {
    /// Byte encoded FILE_DESCRIPTOR_SET.
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        include_bytes!("../../../generated/sui.rpc.v2alpha.fds.bin");

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
