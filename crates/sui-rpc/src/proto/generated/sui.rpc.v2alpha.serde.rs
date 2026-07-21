impl serde::Serialize for GetCheckpointObjectProofRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.object_id.is_some() {
            len += 1;
        }
        if self.checkpoint.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.GetCheckpointObjectProofRequest", len)?;
        if let Some(v) = self.object_id.as_ref() {
            struct_ser.serialize_field("objectId", v)?;
        }
        if let Some(v) = self.checkpoint.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("checkpoint", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCheckpointObjectProofRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["object_id", "objectId", "checkpoint"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectId,
            Checkpoint,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objectId" | "object_id" => Ok(GeneratedField::ObjectId),
                            "checkpoint" => Ok(GeneratedField::Checkpoint),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCheckpointObjectProofRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct sui.rpc.v2alpha.GetCheckpointObjectProofRequest")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<GetCheckpointObjectProofRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut object_id__ = None;
                let mut checkpoint__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ObjectId => {
                            if object_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectId"));
                            }
                            object_id__ = map_.next_value()?;
                        }
                        GeneratedField::Checkpoint => {
                            if checkpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkpoint"));
                            }
                            checkpoint__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetCheckpointObjectProofRequest {
                    object_id: object_id__,
                    checkpoint: checkpoint__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.GetCheckpointObjectProofRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for GetCheckpointObjectProofResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.checkpoint_summary.is_some() {
            len += 1;
        }
        if self.proof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.GetCheckpointObjectProofResponse", len)?;
        if let Some(v) = self.checkpoint_summary.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field(
                    "checkpointSummary",
                    crate::_serde::base64::encode(&v).as_str(),
                )?;
        }
        if let Some(v) = self.proof.as_ref() {
            match v {
                get_checkpoint_object_proof_response::Proof::Inclusion(v) => {
                    struct_ser.serialize_field("inclusion", v)?;
                }
                get_checkpoint_object_proof_response::Proof::NonInclusion(v) => {
                    struct_ser.serialize_field("nonInclusion", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCheckpointObjectProofResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "checkpoint_summary",
            "checkpointSummary",
            "inclusion",
            "non_inclusion",
            "nonInclusion",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CheckpointSummary,
            Inclusion,
            NonInclusion,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "checkpointSummary" | "checkpoint_summary" => {
                                Ok(GeneratedField::CheckpointSummary)
                            }
                            "inclusion" => Ok(GeneratedField::Inclusion),
                            "nonInclusion" | "non_inclusion" => {
                                Ok(GeneratedField::NonInclusion)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCheckpointObjectProofResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct sui.rpc.v2alpha.GetCheckpointObjectProofResponse")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<GetCheckpointObjectProofResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut checkpoint_summary__ = None;
                let mut proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CheckpointSummary => {
                            if checkpoint_summary__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("checkpointSummary"),
                                );
                            }
                            checkpoint_summary__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::Inclusion => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inclusion"));
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(
                                    get_checkpoint_object_proof_response::Proof::Inclusion,
                                );
                        }
                        GeneratedField::NonInclusion => {
                            if proof__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("nonInclusion"),
                                );
                            }
                            proof__ = map_
                                .next_value::<::std::option::Option<_>>()?
                                .map(
                                    get_checkpoint_object_proof_response::Proof::NonInclusion,
                                );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetCheckpointObjectProofResponse {
                    checkpoint_summary: checkpoint_summary__,
                    proof: proof__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.GetCheckpointObjectProofResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MerkleNeighbourLeaf {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.leaf.is_some() {
            len += 1;
        }
        if self.merkle_proof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.MerkleNeighbourLeaf", len)?;
        if let Some(v) = self.leaf.as_ref() {
            struct_ser.serialize_field("leaf", v)?;
        }
        if let Some(v) = self.merkle_proof.as_ref() {
            struct_ser.serialize_field("merkleProof", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleNeighbourLeaf {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["leaf", "merkle_proof", "merkleProof"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Leaf,
            MerkleProof,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "leaf" => Ok(GeneratedField::Leaf),
                            "merkleProof" | "merkle_proof" => {
                                Ok(GeneratedField::MerkleProof)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerkleNeighbourLeaf;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.MerkleNeighbourLeaf")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MerkleNeighbourLeaf, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut leaf__ = None;
                let mut merkle_proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Leaf => {
                            if leaf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leaf"));
                            }
                            leaf__ = map_.next_value()?;
                        }
                        GeneratedField::MerkleProof => {
                            if merkle_proof__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("merkleProof"),
                                );
                            }
                            merkle_proof__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MerkleNeighbourLeaf {
                    leaf: leaf__,
                    merkle_proof: merkle_proof__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.MerkleNeighbourLeaf",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MerkleNode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.node.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.MerkleNode", len)?;
        if let Some(v) = self.node.as_ref() {
            match v {
                merkle_node::Node::Empty(v) => {
                    struct_ser
                        .serialize_field("empty", &crate::_serde::EmptySerializer(v))?;
                }
                merkle_node::Node::Digest(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser
                        .serialize_field(
                            "digest",
                            crate::_serde::base64::encode(&v).as_str(),
                        )?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleNode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["empty", "digest"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Empty,
            Digest,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "empty" => Ok(GeneratedField::Empty),
                            "digest" => Ok(GeneratedField::Digest),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerkleNode;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.MerkleNode")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MerkleNode, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Empty => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("empty"));
                            }
                            node__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::EmptyDeserializer>,
                                >()?
                                .map(|x| merkle_node::Node::Empty(x.0));
                        }
                        GeneratedField::Digest => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digest"));
                            }
                            node__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| merkle_node::Node::Digest(x.0));
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MerkleNode { node: node__ })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.MerkleNode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MerkleNonInclusionProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.index.is_some() {
            len += 1;
        }
        if self.left_leaf.is_some() {
            len += 1;
        }
        if self.right_leaf.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.MerkleNonInclusionProof", len)?;
        if let Some(v) = self.index.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("index", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.left_leaf.as_ref() {
            struct_ser.serialize_field("leftLeaf", v)?;
        }
        if let Some(v) = self.right_leaf.as_ref() {
            struct_ser.serialize_field("rightLeaf", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleNonInclusionProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "index",
            "left_leaf",
            "leftLeaf",
            "right_leaf",
            "rightLeaf",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Index,
            LeftLeaf,
            RightLeaf,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "index" => Ok(GeneratedField::Index),
                            "leftLeaf" | "left_leaf" => Ok(GeneratedField::LeftLeaf),
                            "rightLeaf" | "right_leaf" => Ok(GeneratedField::RightLeaf),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerkleNonInclusionProof;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.MerkleNonInclusionProof")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MerkleNonInclusionProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut index__ = None;
                let mut left_leaf__ = None;
                let mut right_leaf__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::LeftLeaf => {
                            if left_leaf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leftLeaf"));
                            }
                            left_leaf__ = map_.next_value()?;
                        }
                        GeneratedField::RightLeaf => {
                            if right_leaf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rightLeaf"));
                            }
                            right_leaf__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MerkleNonInclusionProof {
                    index: index__,
                    left_leaf: left_leaf__,
                    right_leaf: right_leaf__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.MerkleNonInclusionProof",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MerkleProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.MerkleProof", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["path"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerkleProof;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.MerkleProof")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MerkleProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MerkleProof {
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct("sui.rpc.v2alpha.MerkleProof", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OcsInclusionProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.object_ref.is_some() {
            len += 1;
        }
        if self.merkle_proof.is_some() {
            len += 1;
        }
        if self.leaf_index.is_some() {
            len += 1;
        }
        if self.tree_root.is_some() {
            len += 1;
        }
        if self.object_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.OcsInclusionProof", len)?;
        if let Some(v) = self.object_ref.as_ref() {
            struct_ser.serialize_field("objectRef", v)?;
        }
        if let Some(v) = self.merkle_proof.as_ref() {
            struct_ser.serialize_field("merkleProof", v)?;
        }
        if let Some(v) = self.leaf_index.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("leafIndex", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.tree_root.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field(
                    "treeRoot",
                    crate::_serde::base64::encode(&v).as_str(),
                )?;
        }
        if let Some(v) = self.object_data.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field(
                    "objectData",
                    crate::_serde::base64::encode(&v).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OcsInclusionProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_ref",
            "objectRef",
            "merkle_proof",
            "merkleProof",
            "leaf_index",
            "leafIndex",
            "tree_root",
            "treeRoot",
            "object_data",
            "objectData",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectRef,
            MerkleProof,
            LeafIndex,
            TreeRoot,
            ObjectData,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objectRef" | "object_ref" => Ok(GeneratedField::ObjectRef),
                            "merkleProof" | "merkle_proof" => {
                                Ok(GeneratedField::MerkleProof)
                            }
                            "leafIndex" | "leaf_index" => Ok(GeneratedField::LeafIndex),
                            "treeRoot" | "tree_root" => Ok(GeneratedField::TreeRoot),
                            "objectData" | "object_data" => {
                                Ok(GeneratedField::ObjectData)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OcsInclusionProof;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.OcsInclusionProof")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<OcsInclusionProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut object_ref__ = None;
                let mut merkle_proof__ = None;
                let mut leaf_index__ = None;
                let mut tree_root__ = None;
                let mut object_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ObjectRef => {
                            if object_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectRef"));
                            }
                            object_ref__ = map_.next_value()?;
                        }
                        GeneratedField::MerkleProof => {
                            if merkle_proof__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("merkleProof"),
                                );
                            }
                            merkle_proof__ = map_.next_value()?;
                        }
                        GeneratedField::LeafIndex => {
                            if leaf_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leafIndex"));
                            }
                            leaf_index__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::NumberDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::TreeRoot => {
                            if tree_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treeRoot"));
                            }
                            tree_root__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::ObjectData => {
                            if object_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectData"));
                            }
                            object_data__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(OcsInclusionProof {
                    object_ref: object_ref__,
                    merkle_proof: merkle_proof__,
                    leaf_index: leaf_index__,
                    tree_root: tree_root__,
                    object_data: object_data__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.OcsInclusionProof",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for OcsNonInclusionProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0usize;
        if self.non_inclusion_proof.is_some() {
            len += 1;
        }
        if self.tree_root.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("sui.rpc.v2alpha.OcsNonInclusionProof", len)?;
        if let Some(v) = self.non_inclusion_proof.as_ref() {
            struct_ser.serialize_field("nonInclusionProof", v)?;
        }
        if let Some(v) = self.tree_root.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser
                .serialize_field(
                    "treeRoot",
                    crate::_serde::base64::encode(&v).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OcsNonInclusionProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "non_inclusion_proof",
            "nonInclusionProof",
            "tree_root",
            "treeRoot",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NonInclusionProof,
            TreeRoot,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nonInclusionProof" | "non_inclusion_proof" => {
                                Ok(GeneratedField::NonInclusionProof)
                            }
                            "treeRoot" | "tree_root" => Ok(GeneratedField::TreeRoot),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OcsNonInclusionProof;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct sui.rpc.v2alpha.OcsNonInclusionProof")
            }
            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<OcsNonInclusionProof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut non_inclusion_proof__ = None;
                let mut tree_root__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NonInclusionProof => {
                            if non_inclusion_proof__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("nonInclusionProof"),
                                );
                            }
                            non_inclusion_proof__ = map_.next_value()?;
                        }
                        GeneratedField::TreeRoot => {
                            if tree_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treeRoot"));
                            }
                            tree_root__ = map_
                                .next_value::<
                                    ::std::option::Option<crate::_serde::BytesDeserialize<_>>,
                                >()?
                                .map(|x| x.0);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(OcsNonInclusionProof {
                    non_inclusion_proof: non_inclusion_proof__,
                    tree_root: tree_root__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "sui.rpc.v2alpha.OcsNonInclusionProof",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
