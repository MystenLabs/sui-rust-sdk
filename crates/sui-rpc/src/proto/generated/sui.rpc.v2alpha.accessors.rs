mod _accessor_impls {
    #![allow(clippy::useless_conversion)]
    impl super::GetCheckpointObjectProofRequest {
        pub const fn const_default() -> Self {
            Self {
                object_id: None,
                checkpoint: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetCheckpointObjectProofRequest = super::GetCheckpointObjectProofRequest::const_default();
            &DEFAULT
        }
        ///If `object_id` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_id_opt_mut(&mut self) -> Option<&mut String> {
            self.object_id.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_id`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_id_mut(&mut self) -> &mut String {
            self.object_id.get_or_insert_default()
        }
        ///If `object_id` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_id_opt(&self) -> Option<&str> {
            self.object_id.as_ref().map(|field| field as _)
        }
        ///Sets `object_id` with the provided value.
        pub fn set_object_id<T: Into<String>>(&mut self, field: T) {
            self.object_id = Some(field.into().into());
        }
        ///Sets `object_id` with the provided value.
        pub fn with_object_id<T: Into<String>>(mut self, field: T) -> Self {
            self.set_object_id(field.into());
            self
        }
        ///If `checkpoint` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn checkpoint_opt_mut(&mut self) -> Option<&mut u64> {
            self.checkpoint.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `checkpoint`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn checkpoint_mut(&mut self) -> &mut u64 {
            self.checkpoint.get_or_insert_default()
        }
        ///If `checkpoint` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn checkpoint_opt(&self) -> Option<u64> {
            self.checkpoint.as_ref().map(|field| *field)
        }
        ///Sets `checkpoint` with the provided value.
        pub fn set_checkpoint(&mut self, field: u64) {
            self.checkpoint = Some(field);
        }
        ///Sets `checkpoint` with the provided value.
        pub fn with_checkpoint(mut self, field: u64) -> Self {
            self.set_checkpoint(field);
            self
        }
    }
    impl super::GetCheckpointObjectProofResponse {
        pub const fn const_default() -> Self {
            Self {
                checkpoint_summary: None,
                proof: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::GetCheckpointObjectProofResponse = super::GetCheckpointObjectProofResponse::const_default();
            &DEFAULT
        }
        ///If `checkpoint_summary` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn checkpoint_summary_opt(&self) -> Option<&[u8]> {
            self.checkpoint_summary.as_ref().map(|field| field as _)
        }
        ///Sets `checkpoint_summary` with the provided value.
        pub fn set_checkpoint_summary<T: Into<::prost::bytes::Bytes>>(
            &mut self,
            field: T,
        ) {
            self.checkpoint_summary = Some(field.into().into());
        }
        ///Sets `checkpoint_summary` with the provided value.
        pub fn with_checkpoint_summary<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_checkpoint_summary(field.into());
            self
        }
        ///Returns the value of `inclusion`, or the default value if `inclusion` is unset.
        pub fn inclusion(&self) -> &super::OcsInclusionProof {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::Inclusion(field),
            ) = &self.proof
            {
                field as _
            } else {
                super::OcsInclusionProof::default_instance() as _
            }
        }
        ///If `inclusion` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn inclusion_opt(&self) -> Option<&super::OcsInclusionProof> {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::Inclusion(field),
            ) = &self.proof
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `inclusion` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn inclusion_opt_mut(&mut self) -> Option<&mut super::OcsInclusionProof> {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::Inclusion(field),
            ) = &mut self.proof
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `inclusion`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn inclusion_mut(&mut self) -> &mut super::OcsInclusionProof {
            if self.inclusion_opt_mut().is_none() {
                self.proof = Some(
                    super::get_checkpoint_object_proof_response::Proof::Inclusion(
                        super::OcsInclusionProof::default(),
                    ),
                );
            }
            self.inclusion_opt_mut().unwrap()
        }
        ///Sets `inclusion` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_inclusion<T: Into<super::OcsInclusionProof>>(&mut self, field: T) {
            self.proof = Some(
                super::get_checkpoint_object_proof_response::Proof::Inclusion(
                    field.into().into(),
                ),
            );
        }
        ///Sets `inclusion` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_inclusion<T: Into<super::OcsInclusionProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_inclusion(field.into());
            self
        }
        ///Returns the value of `non_inclusion`, or the default value if `non_inclusion` is unset.
        pub fn non_inclusion(&self) -> &super::OcsNonInclusionProof {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::NonInclusion(field),
            ) = &self.proof
            {
                field as _
            } else {
                super::OcsNonInclusionProof::default_instance() as _
            }
        }
        ///If `non_inclusion` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn non_inclusion_opt(&self) -> Option<&super::OcsNonInclusionProof> {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::NonInclusion(field),
            ) = &self.proof
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `non_inclusion` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn non_inclusion_opt_mut(
            &mut self,
        ) -> Option<&mut super::OcsNonInclusionProof> {
            if let Some(
                super::get_checkpoint_object_proof_response::Proof::NonInclusion(field),
            ) = &mut self.proof
            {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `non_inclusion`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn non_inclusion_mut(&mut self) -> &mut super::OcsNonInclusionProof {
            if self.non_inclusion_opt_mut().is_none() {
                self.proof = Some(
                    super::get_checkpoint_object_proof_response::Proof::NonInclusion(
                        super::OcsNonInclusionProof::default(),
                    ),
                );
            }
            self.non_inclusion_opt_mut().unwrap()
        }
        ///Sets `non_inclusion` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_non_inclusion<T: Into<super::OcsNonInclusionProof>>(
            &mut self,
            field: T,
        ) {
            self.proof = Some(
                super::get_checkpoint_object_proof_response::Proof::NonInclusion(
                    field.into().into(),
                ),
            );
        }
        ///Sets `non_inclusion` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_non_inclusion<T: Into<super::OcsNonInclusionProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_non_inclusion(field.into());
            self
        }
    }
    impl super::MerkleNeighbourLeaf {
        pub const fn const_default() -> Self {
            Self {
                leaf: None,
                merkle_proof: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MerkleNeighbourLeaf = super::MerkleNeighbourLeaf::const_default();
            &DEFAULT
        }
        ///Returns the value of `leaf`, or the default value if `leaf` is unset.
        pub fn leaf(&self) -> &super::super::v2::ObjectReference {
            self.leaf
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::super::v2::ObjectReference::default_instance() as _
                })
        }
        ///If `leaf` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn leaf_opt_mut(
            &mut self,
        ) -> Option<&mut super::super::v2::ObjectReference> {
            self.leaf.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `leaf`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn leaf_mut(&mut self) -> &mut super::super::v2::ObjectReference {
            self.leaf.get_or_insert_default()
        }
        ///If `leaf` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn leaf_opt(&self) -> Option<&super::super::v2::ObjectReference> {
            self.leaf.as_ref().map(|field| field as _)
        }
        ///Sets `leaf` with the provided value.
        pub fn set_leaf<T: Into<super::super::v2::ObjectReference>>(
            &mut self,
            field: T,
        ) {
            self.leaf = Some(field.into().into());
        }
        ///Sets `leaf` with the provided value.
        pub fn with_leaf<T: Into<super::super::v2::ObjectReference>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_leaf(field.into());
            self
        }
        ///Returns the value of `merkle_proof`, or the default value if `merkle_proof` is unset.
        pub fn merkle_proof(&self) -> &super::MerkleProof {
            self.merkle_proof
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MerkleProof::default_instance() as _)
        }
        ///If `merkle_proof` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn merkle_proof_opt_mut(&mut self) -> Option<&mut super::MerkleProof> {
            self.merkle_proof.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `merkle_proof`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn merkle_proof_mut(&mut self) -> &mut super::MerkleProof {
            self.merkle_proof.get_or_insert_default()
        }
        ///If `merkle_proof` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn merkle_proof_opt(&self) -> Option<&super::MerkleProof> {
            self.merkle_proof.as_ref().map(|field| field as _)
        }
        ///Sets `merkle_proof` with the provided value.
        pub fn set_merkle_proof<T: Into<super::MerkleProof>>(&mut self, field: T) {
            self.merkle_proof = Some(field.into().into());
        }
        ///Sets `merkle_proof` with the provided value.
        pub fn with_merkle_proof<T: Into<super::MerkleProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_merkle_proof(field.into());
            self
        }
    }
    impl super::MerkleNode {
        pub const fn const_default() -> Self {
            Self { node: None }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MerkleNode = super::MerkleNode::const_default();
            &DEFAULT
        }
        ///Returns the value of `digest`, or the default value if `digest` is unset.
        pub fn digest(&self) -> &[u8] {
            if let Some(super::merkle_node::Node::Digest(field)) = &self.node {
                field as _
            } else {
                &[]
            }
        }
        ///If `digest` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn digest_opt(&self) -> Option<&[u8]> {
            if let Some(super::merkle_node::Node::Digest(field)) = &self.node {
                Some(field as _)
            } else {
                None
            }
        }
        ///If `digest` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn digest_opt_mut(&mut self) -> Option<&mut ::prost::bytes::Bytes> {
            if let Some(super::merkle_node::Node::Digest(field)) = &mut self.node {
                Some(field as _)
            } else {
                None
            }
        }
        ///Returns a mutable reference to `digest`.
        ///If the field is unset, it is first initialized with the default value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn digest_mut(&mut self) -> &mut ::prost::bytes::Bytes {
            if self.digest_opt_mut().is_none() {
                self.node = Some(
                    super::merkle_node::Node::Digest(::prost::bytes::Bytes::default()),
                );
            }
            self.digest_opt_mut().unwrap()
        }
        ///Sets `digest` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn set_digest<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.node = Some(super::merkle_node::Node::Digest(field.into().into()));
        }
        ///Sets `digest` with the provided value.
        ///If any other oneof field in the same oneof is set, it will be cleared.
        pub fn with_digest<T: Into<::prost::bytes::Bytes>>(mut self, field: T) -> Self {
            self.set_digest(field.into());
            self
        }
    }
    impl super::MerkleNonInclusionProof {
        pub const fn const_default() -> Self {
            Self {
                index: None,
                left_leaf: None,
                right_leaf: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MerkleNonInclusionProof = super::MerkleNonInclusionProof::const_default();
            &DEFAULT
        }
        ///If `index` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn index_opt_mut(&mut self) -> Option<&mut u64> {
            self.index.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `index`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn index_mut(&mut self) -> &mut u64 {
            self.index.get_or_insert_default()
        }
        ///If `index` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn index_opt(&self) -> Option<u64> {
            self.index.as_ref().map(|field| *field)
        }
        ///Sets `index` with the provided value.
        pub fn set_index(&mut self, field: u64) {
            self.index = Some(field);
        }
        ///Sets `index` with the provided value.
        pub fn with_index(mut self, field: u64) -> Self {
            self.set_index(field);
            self
        }
        ///Returns the value of `left_leaf`, or the default value if `left_leaf` is unset.
        pub fn left_leaf(&self) -> &super::MerkleNeighbourLeaf {
            self.left_leaf
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MerkleNeighbourLeaf::default_instance() as _)
        }
        ///If `left_leaf` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn left_leaf_opt_mut(&mut self) -> Option<&mut super::MerkleNeighbourLeaf> {
            self.left_leaf.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `left_leaf`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn left_leaf_mut(&mut self) -> &mut super::MerkleNeighbourLeaf {
            self.left_leaf.get_or_insert_default()
        }
        ///If `left_leaf` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn left_leaf_opt(&self) -> Option<&super::MerkleNeighbourLeaf> {
            self.left_leaf.as_ref().map(|field| field as _)
        }
        ///Sets `left_leaf` with the provided value.
        pub fn set_left_leaf<T: Into<super::MerkleNeighbourLeaf>>(&mut self, field: T) {
            self.left_leaf = Some(field.into().into());
        }
        ///Sets `left_leaf` with the provided value.
        pub fn with_left_leaf<T: Into<super::MerkleNeighbourLeaf>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_left_leaf(field.into());
            self
        }
        ///Returns the value of `right_leaf`, or the default value if `right_leaf` is unset.
        pub fn right_leaf(&self) -> &super::MerkleNeighbourLeaf {
            self.right_leaf
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MerkleNeighbourLeaf::default_instance() as _)
        }
        ///If `right_leaf` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn right_leaf_opt_mut(&mut self) -> Option<&mut super::MerkleNeighbourLeaf> {
            self.right_leaf.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `right_leaf`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn right_leaf_mut(&mut self) -> &mut super::MerkleNeighbourLeaf {
            self.right_leaf.get_or_insert_default()
        }
        ///If `right_leaf` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn right_leaf_opt(&self) -> Option<&super::MerkleNeighbourLeaf> {
            self.right_leaf.as_ref().map(|field| field as _)
        }
        ///Sets `right_leaf` with the provided value.
        pub fn set_right_leaf<T: Into<super::MerkleNeighbourLeaf>>(&mut self, field: T) {
            self.right_leaf = Some(field.into().into());
        }
        ///Sets `right_leaf` with the provided value.
        pub fn with_right_leaf<T: Into<super::MerkleNeighbourLeaf>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_right_leaf(field.into());
            self
        }
    }
    impl super::MerkleProof {
        pub const fn const_default() -> Self {
            Self { path: Vec::new() }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::MerkleProof = super::MerkleProof::const_default();
            &DEFAULT
        }
        ///Returns the value of `path`, or the default value if `path` is unset.
        pub fn path(&self) -> &[super::MerkleNode] {
            &self.path
        }
        ///Returns a mutable reference to `path`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn path_mut(&mut self) -> &mut Vec<super::MerkleNode> {
            &mut self.path
        }
        ///Sets `path` with the provided value.
        pub fn set_path(&mut self, field: Vec<super::MerkleNode>) {
            self.path = field;
        }
        ///Sets `path` with the provided value.
        pub fn with_path(mut self, field: Vec<super::MerkleNode>) -> Self {
            self.set_path(field);
            self
        }
    }
    impl super::OcsInclusionProof {
        pub const fn const_default() -> Self {
            Self {
                object_ref: None,
                merkle_proof: None,
                leaf_index: None,
                tree_root: None,
                object_data: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::OcsInclusionProof = super::OcsInclusionProof::const_default();
            &DEFAULT
        }
        ///Returns the value of `object_ref`, or the default value if `object_ref` is unset.
        pub fn object_ref(&self) -> &super::super::v2::ObjectReference {
            self.object_ref
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::super::v2::ObjectReference::default_instance() as _
                })
        }
        ///If `object_ref` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn object_ref_opt_mut(
            &mut self,
        ) -> Option<&mut super::super::v2::ObjectReference> {
            self.object_ref.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `object_ref`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn object_ref_mut(&mut self) -> &mut super::super::v2::ObjectReference {
            self.object_ref.get_or_insert_default()
        }
        ///If `object_ref` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_ref_opt(&self) -> Option<&super::super::v2::ObjectReference> {
            self.object_ref.as_ref().map(|field| field as _)
        }
        ///Sets `object_ref` with the provided value.
        pub fn set_object_ref<T: Into<super::super::v2::ObjectReference>>(
            &mut self,
            field: T,
        ) {
            self.object_ref = Some(field.into().into());
        }
        ///Sets `object_ref` with the provided value.
        pub fn with_object_ref<T: Into<super::super::v2::ObjectReference>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_object_ref(field.into());
            self
        }
        ///Returns the value of `merkle_proof`, or the default value if `merkle_proof` is unset.
        pub fn merkle_proof(&self) -> &super::MerkleProof {
            self.merkle_proof
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| super::MerkleProof::default_instance() as _)
        }
        ///If `merkle_proof` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn merkle_proof_opt_mut(&mut self) -> Option<&mut super::MerkleProof> {
            self.merkle_proof.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `merkle_proof`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn merkle_proof_mut(&mut self) -> &mut super::MerkleProof {
            self.merkle_proof.get_or_insert_default()
        }
        ///If `merkle_proof` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn merkle_proof_opt(&self) -> Option<&super::MerkleProof> {
            self.merkle_proof.as_ref().map(|field| field as _)
        }
        ///Sets `merkle_proof` with the provided value.
        pub fn set_merkle_proof<T: Into<super::MerkleProof>>(&mut self, field: T) {
            self.merkle_proof = Some(field.into().into());
        }
        ///Sets `merkle_proof` with the provided value.
        pub fn with_merkle_proof<T: Into<super::MerkleProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_merkle_proof(field.into());
            self
        }
        ///If `leaf_index` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn leaf_index_opt_mut(&mut self) -> Option<&mut u64> {
            self.leaf_index.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `leaf_index`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn leaf_index_mut(&mut self) -> &mut u64 {
            self.leaf_index.get_or_insert_default()
        }
        ///If `leaf_index` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn leaf_index_opt(&self) -> Option<u64> {
            self.leaf_index.as_ref().map(|field| *field)
        }
        ///Sets `leaf_index` with the provided value.
        pub fn set_leaf_index(&mut self, field: u64) {
            self.leaf_index = Some(field);
        }
        ///Sets `leaf_index` with the provided value.
        pub fn with_leaf_index(mut self, field: u64) -> Self {
            self.set_leaf_index(field);
            self
        }
        ///If `tree_root` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn tree_root_opt(&self) -> Option<&[u8]> {
            self.tree_root.as_ref().map(|field| field as _)
        }
        ///Sets `tree_root` with the provided value.
        pub fn set_tree_root<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.tree_root = Some(field.into().into());
        }
        ///Sets `tree_root` with the provided value.
        pub fn with_tree_root<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_tree_root(field.into());
            self
        }
        ///If `object_data` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn object_data_opt(&self) -> Option<&[u8]> {
            self.object_data.as_ref().map(|field| field as _)
        }
        ///Sets `object_data` with the provided value.
        pub fn set_object_data<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.object_data = Some(field.into().into());
        }
        ///Sets `object_data` with the provided value.
        pub fn with_object_data<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_object_data(field.into());
            self
        }
    }
    impl super::OcsNonInclusionProof {
        pub const fn const_default() -> Self {
            Self {
                non_inclusion_proof: None,
                tree_root: None,
            }
        }
        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: super::OcsNonInclusionProof = super::OcsNonInclusionProof::const_default();
            &DEFAULT
        }
        ///Returns the value of `non_inclusion_proof`, or the default value if `non_inclusion_proof` is unset.
        pub fn non_inclusion_proof(&self) -> &super::MerkleNonInclusionProof {
            self.non_inclusion_proof
                .as_ref()
                .map(|field| field as _)
                .unwrap_or_else(|| {
                    super::MerkleNonInclusionProof::default_instance() as _
                })
        }
        ///If `non_inclusion_proof` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`].
        pub fn non_inclusion_proof_opt_mut(
            &mut self,
        ) -> Option<&mut super::MerkleNonInclusionProof> {
            self.non_inclusion_proof.as_mut().map(|field| field as _)
        }
        ///Returns a mutable reference to `non_inclusion_proof`.
        ///If the field is unset, it is first initialized with the default value.
        pub fn non_inclusion_proof_mut(
            &mut self,
        ) -> &mut super::MerkleNonInclusionProof {
            self.non_inclusion_proof.get_or_insert_default()
        }
        ///If `non_inclusion_proof` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn non_inclusion_proof_opt(
            &self,
        ) -> Option<&super::MerkleNonInclusionProof> {
            self.non_inclusion_proof.as_ref().map(|field| field as _)
        }
        ///Sets `non_inclusion_proof` with the provided value.
        pub fn set_non_inclusion_proof<T: Into<super::MerkleNonInclusionProof>>(
            &mut self,
            field: T,
        ) {
            self.non_inclusion_proof = Some(field.into().into());
        }
        ///Sets `non_inclusion_proof` with the provided value.
        pub fn with_non_inclusion_proof<T: Into<super::MerkleNonInclusionProof>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_non_inclusion_proof(field.into());
            self
        }
        ///If `tree_root` is set, returns [`Some`] with the value; otherwise returns [`None`].
        pub fn tree_root_opt(&self) -> Option<&[u8]> {
            self.tree_root.as_ref().map(|field| field as _)
        }
        ///Sets `tree_root` with the provided value.
        pub fn set_tree_root<T: Into<::prost::bytes::Bytes>>(&mut self, field: T) {
            self.tree_root = Some(field.into().into());
        }
        ///Sets `tree_root` with the provided value.
        pub fn with_tree_root<T: Into<::prost::bytes::Bytes>>(
            mut self,
            field: T,
        ) -> Self {
            self.set_tree_root(field.into());
            self
        }
    }
}
