pub(crate) mod _field_impls {
    #![allow(clippy::wrong_self_convention)]
    use super::*;
    use crate::field::MessageFields;
    use crate::field::MessageField;
    impl MerkleNode {
        pub const EMPTY_FIELD: &'static MessageField = &MessageField {
            name: "empty",
            json_name: "empty",
            number: 1i32,
            message_fields: None,
        };
        pub const DIGEST_FIELD: &'static MessageField = &MessageField {
            name: "digest",
            json_name: "digest",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for MerkleNode {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::EMPTY_FIELD,
            Self::DIGEST_FIELD,
        ];
    }
    impl MerkleNode {
        pub fn path_builder() -> MerkleNodeFieldPathBuilder {
            MerkleNodeFieldPathBuilder::new()
        }
    }
    pub struct MerkleNodeFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MerkleNodeFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn empty(mut self) -> String {
            self.path.push(MerkleNode::EMPTY_FIELD.name);
            self.finish()
        }
        pub fn digest(mut self) -> String {
            self.path.push(MerkleNode::DIGEST_FIELD.name);
            self.finish()
        }
    }
    impl MerkleProof {
        pub const PATH_FIELD: &'static MessageField = &MessageField {
            name: "path",
            json_name: "path",
            number: 1i32,
            message_fields: Some(MerkleNode::FIELDS),
        };
    }
    impl MessageFields for MerkleProof {
        const FIELDS: &'static [&'static MessageField] = &[Self::PATH_FIELD];
    }
    impl MerkleProof {
        pub fn path_builder() -> MerkleProofFieldPathBuilder {
            MerkleProofFieldPathBuilder::new()
        }
    }
    pub struct MerkleProofFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MerkleProofFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn path(mut self) -> MerkleNodeFieldPathBuilder {
            self.path.push(MerkleProof::PATH_FIELD.name);
            MerkleNodeFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MerkleNonInclusionProof {
        pub const INDEX_FIELD: &'static MessageField = &MessageField {
            name: "index",
            json_name: "index",
            number: 1i32,
            message_fields: None,
        };
        pub const LEFT_LEAF_FIELD: &'static MessageField = &MessageField {
            name: "left_leaf",
            json_name: "leftLeaf",
            number: 2i32,
            message_fields: Some(MerkleNeighbourLeaf::FIELDS),
        };
        pub const RIGHT_LEAF_FIELD: &'static MessageField = &MessageField {
            name: "right_leaf",
            json_name: "rightLeaf",
            number: 3i32,
            message_fields: Some(MerkleNeighbourLeaf::FIELDS),
        };
    }
    impl MessageFields for MerkleNonInclusionProof {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::INDEX_FIELD,
            Self::LEFT_LEAF_FIELD,
            Self::RIGHT_LEAF_FIELD,
        ];
    }
    impl MerkleNonInclusionProof {
        pub fn path_builder() -> MerkleNonInclusionProofFieldPathBuilder {
            MerkleNonInclusionProofFieldPathBuilder::new()
        }
    }
    pub struct MerkleNonInclusionProofFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MerkleNonInclusionProofFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn index(mut self) -> String {
            self.path.push(MerkleNonInclusionProof::INDEX_FIELD.name);
            self.finish()
        }
        pub fn left_leaf(mut self) -> MerkleNeighbourLeafFieldPathBuilder {
            self.path.push(MerkleNonInclusionProof::LEFT_LEAF_FIELD.name);
            MerkleNeighbourLeafFieldPathBuilder::new_with_base(self.path)
        }
        pub fn right_leaf(mut self) -> MerkleNeighbourLeafFieldPathBuilder {
            self.path.push(MerkleNonInclusionProof::RIGHT_LEAF_FIELD.name);
            MerkleNeighbourLeafFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl MerkleNeighbourLeaf {
        pub const LEAF_FIELD: &'static MessageField = &MessageField {
            name: "leaf",
            json_name: "leaf",
            number: 1i32,
            message_fields: Some(ObjectReference::FIELDS),
        };
        pub const MERKLE_PROOF_FIELD: &'static MessageField = &MessageField {
            name: "merkle_proof",
            json_name: "merkleProof",
            number: 2i32,
            message_fields: Some(MerkleProof::FIELDS),
        };
    }
    impl MessageFields for MerkleNeighbourLeaf {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::LEAF_FIELD,
            Self::MERKLE_PROOF_FIELD,
        ];
    }
    impl MerkleNeighbourLeaf {
        pub fn path_builder() -> MerkleNeighbourLeafFieldPathBuilder {
            MerkleNeighbourLeafFieldPathBuilder::new()
        }
    }
    pub struct MerkleNeighbourLeafFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl MerkleNeighbourLeafFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn leaf(mut self) -> ObjectReferenceFieldPathBuilder {
            self.path.push(MerkleNeighbourLeaf::LEAF_FIELD.name);
            ObjectReferenceFieldPathBuilder::new_with_base(self.path)
        }
        pub fn merkle_proof(mut self) -> MerkleProofFieldPathBuilder {
            self.path.push(MerkleNeighbourLeaf::MERKLE_PROOF_FIELD.name);
            MerkleProofFieldPathBuilder::new_with_base(self.path)
        }
    }
    impl OcsInclusionProof {
        pub const OBJECT_REF_FIELD: &'static MessageField = &MessageField {
            name: "object_ref",
            json_name: "objectRef",
            number: 1i32,
            message_fields: Some(ObjectReference::FIELDS),
        };
        pub const MERKLE_PROOF_FIELD: &'static MessageField = &MessageField {
            name: "merkle_proof",
            json_name: "merkleProof",
            number: 2i32,
            message_fields: Some(MerkleProof::FIELDS),
        };
        pub const LEAF_INDEX_FIELD: &'static MessageField = &MessageField {
            name: "leaf_index",
            json_name: "leafIndex",
            number: 3i32,
            message_fields: None,
        };
        pub const TREE_ROOT_FIELD: &'static MessageField = &MessageField {
            name: "tree_root",
            json_name: "treeRoot",
            number: 4i32,
            message_fields: None,
        };
        pub const OBJECT_DATA_FIELD: &'static MessageField = &MessageField {
            name: "object_data",
            json_name: "objectData",
            number: 5i32,
            message_fields: None,
        };
    }
    impl MessageFields for OcsInclusionProof {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECT_REF_FIELD,
            Self::MERKLE_PROOF_FIELD,
            Self::LEAF_INDEX_FIELD,
            Self::TREE_ROOT_FIELD,
            Self::OBJECT_DATA_FIELD,
        ];
    }
    impl OcsInclusionProof {
        pub fn path_builder() -> OcsInclusionProofFieldPathBuilder {
            OcsInclusionProofFieldPathBuilder::new()
        }
    }
    pub struct OcsInclusionProofFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl OcsInclusionProofFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn object_ref(mut self) -> ObjectReferenceFieldPathBuilder {
            self.path.push(OcsInclusionProof::OBJECT_REF_FIELD.name);
            ObjectReferenceFieldPathBuilder::new_with_base(self.path)
        }
        pub fn merkle_proof(mut self) -> MerkleProofFieldPathBuilder {
            self.path.push(OcsInclusionProof::MERKLE_PROOF_FIELD.name);
            MerkleProofFieldPathBuilder::new_with_base(self.path)
        }
        pub fn leaf_index(mut self) -> String {
            self.path.push(OcsInclusionProof::LEAF_INDEX_FIELD.name);
            self.finish()
        }
        pub fn tree_root(mut self) -> String {
            self.path.push(OcsInclusionProof::TREE_ROOT_FIELD.name);
            self.finish()
        }
        pub fn object_data(mut self) -> String {
            self.path.push(OcsInclusionProof::OBJECT_DATA_FIELD.name);
            self.finish()
        }
    }
    impl OcsNonInclusionProof {
        pub const NON_INCLUSION_PROOF_FIELD: &'static MessageField = &MessageField {
            name: "non_inclusion_proof",
            json_name: "nonInclusionProof",
            number: 1i32,
            message_fields: Some(MerkleNonInclusionProof::FIELDS),
        };
        pub const TREE_ROOT_FIELD: &'static MessageField = &MessageField {
            name: "tree_root",
            json_name: "treeRoot",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for OcsNonInclusionProof {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::NON_INCLUSION_PROOF_FIELD,
            Self::TREE_ROOT_FIELD,
        ];
    }
    impl OcsNonInclusionProof {
        pub fn path_builder() -> OcsNonInclusionProofFieldPathBuilder {
            OcsNonInclusionProofFieldPathBuilder::new()
        }
    }
    pub struct OcsNonInclusionProofFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl OcsNonInclusionProofFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn non_inclusion_proof(mut self) -> MerkleNonInclusionProofFieldPathBuilder {
            self.path.push(OcsNonInclusionProof::NON_INCLUSION_PROOF_FIELD.name);
            MerkleNonInclusionProofFieldPathBuilder::new_with_base(self.path)
        }
        pub fn tree_root(mut self) -> String {
            self.path.push(OcsNonInclusionProof::TREE_ROOT_FIELD.name);
            self.finish()
        }
    }
    impl GetCheckpointObjectProofRequest {
        pub const OBJECT_ID_FIELD: &'static MessageField = &MessageField {
            name: "object_id",
            json_name: "objectId",
            number: 1i32,
            message_fields: None,
        };
        pub const CHECKPOINT_FIELD: &'static MessageField = &MessageField {
            name: "checkpoint",
            json_name: "checkpoint",
            number: 2i32,
            message_fields: None,
        };
    }
    impl MessageFields for GetCheckpointObjectProofRequest {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::OBJECT_ID_FIELD,
            Self::CHECKPOINT_FIELD,
        ];
    }
    impl GetCheckpointObjectProofRequest {
        pub fn path_builder() -> GetCheckpointObjectProofRequestFieldPathBuilder {
            GetCheckpointObjectProofRequestFieldPathBuilder::new()
        }
    }
    pub struct GetCheckpointObjectProofRequestFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetCheckpointObjectProofRequestFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn object_id(mut self) -> String {
            self.path.push(GetCheckpointObjectProofRequest::OBJECT_ID_FIELD.name);
            self.finish()
        }
        pub fn checkpoint(mut self) -> String {
            self.path.push(GetCheckpointObjectProofRequest::CHECKPOINT_FIELD.name);
            self.finish()
        }
    }
    impl GetCheckpointObjectProofResponse {
        pub const CHECKPOINT_SUMMARY_FIELD: &'static MessageField = &MessageField {
            name: "checkpoint_summary",
            json_name: "checkpointSummary",
            number: 1i32,
            message_fields: None,
        };
        pub const INCLUSION_FIELD: &'static MessageField = &MessageField {
            name: "inclusion",
            json_name: "inclusion",
            number: 2i32,
            message_fields: Some(OcsInclusionProof::FIELDS),
        };
        pub const NON_INCLUSION_FIELD: &'static MessageField = &MessageField {
            name: "non_inclusion",
            json_name: "nonInclusion",
            number: 3i32,
            message_fields: Some(OcsNonInclusionProof::FIELDS),
        };
    }
    impl MessageFields for GetCheckpointObjectProofResponse {
        const FIELDS: &'static [&'static MessageField] = &[
            Self::CHECKPOINT_SUMMARY_FIELD,
            Self::INCLUSION_FIELD,
            Self::NON_INCLUSION_FIELD,
        ];
    }
    impl GetCheckpointObjectProofResponse {
        pub fn path_builder() -> GetCheckpointObjectProofResponseFieldPathBuilder {
            GetCheckpointObjectProofResponseFieldPathBuilder::new()
        }
    }
    pub struct GetCheckpointObjectProofResponseFieldPathBuilder {
        path: Vec<&'static str>,
    }
    impl GetCheckpointObjectProofResponseFieldPathBuilder {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self { path: Default::default() }
        }
        #[doc(hidden)]
        pub fn new_with_base(base: Vec<&'static str>) -> Self {
            Self { path: base }
        }
        pub fn finish(self) -> String {
            self.path.join(".")
        }
        pub fn checkpoint_summary(mut self) -> String {
            self.path
                .push(GetCheckpointObjectProofResponse::CHECKPOINT_SUMMARY_FIELD.name);
            self.finish()
        }
        pub fn inclusion(mut self) -> OcsInclusionProofFieldPathBuilder {
            self.path.push(GetCheckpointObjectProofResponse::INCLUSION_FIELD.name);
            OcsInclusionProofFieldPathBuilder::new_with_base(self.path)
        }
        pub fn non_inclusion(mut self) -> OcsNonInclusionProofFieldPathBuilder {
            self.path.push(GetCheckpointObjectProofResponse::NON_INCLUSION_FIELD.name);
            OcsNonInclusionProofFieldPathBuilder::new_with_base(self.path)
        }
    }
}
