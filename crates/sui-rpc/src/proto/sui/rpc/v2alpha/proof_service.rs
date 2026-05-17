//! Conversions between the v2alpha `ProofService` proto messages and the
//! in-memory proof / Merkle types in `sui_sdk_types`.
//!
//! The proto wire format encodes Merkle proofs natively — each node is a
//! `oneof { Empty empty, bytes digest }` — rather than carrying a single
//! BCS-encoded blob. The conversions in this module unpack that proto
//! representation into the SDK's in-memory `Node` and `MerkleProof`
//! types, validating that every digest is exactly 32 bytes and that
//! every `MerkleNode` has its `oneof` populated.
//!
//! Conversions for the higher-level `OcsInclusionProof` and
//! `OcsNonInclusionProof` envelopes layer on top of those primitives and
//! the v2 `ObjectReference` conversions.

use super::*;
use crate::proto::TryFromProtoError;

use sui_sdk_types::Digest;
use sui_sdk_types::ObjectReference;
use sui_sdk_types::merkle::MerkleNonInclusionProof as SdkMerkleNonInclusionProof;
use sui_sdk_types::merkle::Node;
use sui_sdk_types::proof::OcsInclusionProof as SdkOcsInclusionProof;
use sui_sdk_types::proof::OcsNonInclusionProof as SdkOcsNonInclusionProof;

const DIGEST_LEN: usize = 32;

/// Decode a 32-byte field from a `Bytes` blob into a `Digest`, with a
/// `TryFromProtoError` pointing at the named field on failure.
fn try_digest_from_bytes(
    bytes: &prost::bytes::Bytes,
    field: &'static str,
) -> Result<Digest, TryFromProtoError> {
    let len = bytes.len();
    let arr: [u8; DIGEST_LEN] = bytes.as_ref().try_into().map_err(|_| {
        TryFromProtoError::invalid(field, format!("expected {DIGEST_LEN} bytes, got {len}"))
    })?;
    Ok(Digest::new(arr))
}

impl From<Node> for MerkleNode {
    fn from(value: Node) -> Self {
        let node = match value {
            Node::Empty => merkle_node::Node::Empty(()),
            Node::Digest(digest) => {
                merkle_node::Node::Digest(prost::bytes::Bytes::copy_from_slice(&digest))
            }
        };
        Self { node: Some(node) }
    }
}

impl From<&Node> for MerkleNode {
    fn from(value: &Node) -> Self {
        (*value).into()
    }
}

impl TryFrom<&MerkleNode> for Node {
    type Error = TryFromProtoError;

    fn try_from(value: &MerkleNode) -> Result<Self, Self::Error> {
        match value.node.as_ref() {
            Some(merkle_node::Node::Empty(_)) => Ok(Node::Empty),
            Some(merkle_node::Node::Digest(bytes)) => {
                let len = bytes.len();
                let digest: [u8; DIGEST_LEN] = bytes.as_ref().try_into().map_err(|_| {
                    TryFromProtoError::invalid(
                        MerkleNode::DIGEST_FIELD.name,
                        format!("expected {DIGEST_LEN} bytes, got {len}"),
                    )
                })?;
                Ok(Node::Digest(digest))
            }
            None => Err(TryFromProtoError::missing("node")),
        }
    }
}

impl From<&sui_sdk_types::merkle::MerkleProof> for MerkleProof {
    fn from(value: &sui_sdk_types::merkle::MerkleProof) -> Self {
        Self {
            path: value.path().iter().copied().map(MerkleNode::from).collect(),
        }
    }
}

impl From<sui_sdk_types::merkle::MerkleProof> for MerkleProof {
    fn from(value: sui_sdk_types::merkle::MerkleProof) -> Self {
        (&value).into()
    }
}

impl TryFrom<&MerkleProof> for sui_sdk_types::merkle::MerkleProof {
    type Error = TryFromProtoError;

    fn try_from(value: &MerkleProof) -> Result<Self, Self::Error> {
        let path = value
            .path
            .iter()
            .enumerate()
            .map(|(i, node)| {
                Node::try_from(node).map_err(|e| e.nested_at(MerkleProof::PATH_FIELD.name, i))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(sui_sdk_types::merkle::MerkleProof::new(path))
    }
}

impl From<&(ObjectReference, sui_sdk_types::merkle::MerkleProof)> for MerkleNeighbourLeaf {
    fn from(value: &(ObjectReference, sui_sdk_types::merkle::MerkleProof)) -> Self {
        let (leaf, proof) = value;
        Self {
            leaf: Some(leaf.clone().into()),
            merkle_proof: Some(proof.into()),
        }
    }
}

impl TryFrom<&MerkleNeighbourLeaf> for (ObjectReference, sui_sdk_types::merkle::MerkleProof) {
    type Error = TryFromProtoError;

    fn try_from(value: &MerkleNeighbourLeaf) -> Result<Self, Self::Error> {
        let leaf_proto = value
            .leaf
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing(MerkleNeighbourLeaf::LEAF_FIELD.name))?;
        let leaf: ObjectReference = leaf_proto
            .try_into()
            .map_err(|e: TryFromProtoError| e.nested(MerkleNeighbourLeaf::LEAF_FIELD.name))?;
        let proof_proto = value.merkle_proof.as_ref().ok_or_else(|| {
            TryFromProtoError::missing(MerkleNeighbourLeaf::MERKLE_PROOF_FIELD.name)
        })?;
        let proof = sui_sdk_types::merkle::MerkleProof::try_from(proof_proto)
            .map_err(|e| e.nested(MerkleNeighbourLeaf::MERKLE_PROOF_FIELD.name))?;
        Ok((leaf, proof))
    }
}

impl From<&SdkMerkleNonInclusionProof<ObjectReference>> for MerkleNonInclusionProof {
    fn from(value: &SdkMerkleNonInclusionProof<ObjectReference>) -> Self {
        Self {
            index: Some(value.index() as u64),
            left_leaf: value.left_leaf().map(MerkleNeighbourLeaf::from),
            right_leaf: value.right_leaf().map(MerkleNeighbourLeaf::from),
        }
    }
}

impl From<SdkMerkleNonInclusionProof<ObjectReference>> for MerkleNonInclusionProof {
    fn from(value: SdkMerkleNonInclusionProof<ObjectReference>) -> Self {
        (&value).into()
    }
}

impl TryFrom<&MerkleNonInclusionProof> for SdkMerkleNonInclusionProof<ObjectReference> {
    type Error = TryFromProtoError;

    fn try_from(value: &MerkleNonInclusionProof) -> Result<Self, Self::Error> {
        let index_u64 = value
            .index
            .ok_or_else(|| TryFromProtoError::missing(MerkleNonInclusionProof::INDEX_FIELD.name))?;
        let index = usize::try_from(index_u64).map_err(|e| {
            TryFromProtoError::invalid(MerkleNonInclusionProof::INDEX_FIELD.name, e)
        })?;
        let left_leaf = value
            .left_leaf
            .as_ref()
            .map(|l| {
                <(ObjectReference, sui_sdk_types::merkle::MerkleProof)>::try_from(l)
                    .map_err(|e| e.nested(MerkleNonInclusionProof::LEFT_LEAF_FIELD.name))
            })
            .transpose()?;
        let right_leaf = value
            .right_leaf
            .as_ref()
            .map(|l| {
                <(ObjectReference, sui_sdk_types::merkle::MerkleProof)>::try_from(l)
                    .map_err(|e| e.nested(MerkleNonInclusionProof::RIGHT_LEAF_FIELD.name))
            })
            .transpose()?;
        Ok(SdkMerkleNonInclusionProof::new(
            index, left_leaf, right_leaf,
        ))
    }
}

impl From<&SdkOcsInclusionProof> for OcsInclusionProof {
    fn from(value: &SdkOcsInclusionProof) -> Self {
        Self {
            // `object_ref` and `object_data` are carried alongside the
            // proof on the wire (so the server can hand the SDK an
            // authenticated reference and the corresponding object
            // bytes in a single round trip), but they're populated by
            // the `LightClient` glue from the response fields rather
            // than by `OcsInclusionProof` itself. This conversion only
            // touches the cryptographic-proof fields.
            object_ref: None,
            merkle_proof: Some((&value.merkle_proof).into()),
            leaf_index: Some(value.leaf_index),
            tree_root: Some(prost::bytes::Bytes::copy_from_slice(
                value.tree_root.inner(),
            )),
            object_data: None,
        }
    }
}

impl From<SdkOcsInclusionProof> for OcsInclusionProof {
    fn from(value: SdkOcsInclusionProof) -> Self {
        (&value).into()
    }
}

impl TryFrom<&OcsInclusionProof> for SdkOcsInclusionProof {
    type Error = TryFromProtoError;

    fn try_from(value: &OcsInclusionProof) -> Result<Self, Self::Error> {
        let merkle_proof_proto = value.merkle_proof.as_ref().ok_or_else(|| {
            TryFromProtoError::missing(OcsInclusionProof::MERKLE_PROOF_FIELD.name)
        })?;
        let merkle_proof = sui_sdk_types::merkle::MerkleProof::try_from(merkle_proof_proto)
            .map_err(|e| e.nested(OcsInclusionProof::MERKLE_PROOF_FIELD.name))?;
        let leaf_index = value
            .leaf_index
            .ok_or_else(|| TryFromProtoError::missing(OcsInclusionProof::LEAF_INDEX_FIELD.name))?;
        let tree_root_bytes = value
            .tree_root
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing(OcsInclusionProof::TREE_ROOT_FIELD.name))?;
        let tree_root =
            try_digest_from_bytes(tree_root_bytes, OcsInclusionProof::TREE_ROOT_FIELD.name)?;
        Ok(SdkOcsInclusionProof {
            merkle_proof,
            leaf_index,
            tree_root,
        })
    }
}

impl From<&SdkOcsNonInclusionProof> for OcsNonInclusionProof {
    fn from(value: &SdkOcsNonInclusionProof) -> Self {
        Self {
            non_inclusion_proof: Some((&value.non_inclusion_proof).into()),
            tree_root: Some(prost::bytes::Bytes::copy_from_slice(
                value.tree_root.inner(),
            )),
        }
    }
}

impl From<SdkOcsNonInclusionProof> for OcsNonInclusionProof {
    fn from(value: SdkOcsNonInclusionProof) -> Self {
        (&value).into()
    }
}

impl TryFrom<&OcsNonInclusionProof> for SdkOcsNonInclusionProof {
    type Error = TryFromProtoError;

    fn try_from(value: &OcsNonInclusionProof) -> Result<Self, Self::Error> {
        let inner_proto = value.non_inclusion_proof.as_ref().ok_or_else(|| {
            TryFromProtoError::missing(OcsNonInclusionProof::NON_INCLUSION_PROOF_FIELD.name)
        })?;
        let non_inclusion_proof =
            SdkMerkleNonInclusionProof::<ObjectReference>::try_from(inner_proto)
                .map_err(|e| e.nested(OcsNonInclusionProof::NON_INCLUSION_PROOF_FIELD.name))?;
        let tree_root_bytes = value.tree_root.as_ref().ok_or_else(|| {
            TryFromProtoError::missing(OcsNonInclusionProof::TREE_ROOT_FIELD.name)
        })?;
        let tree_root =
            try_digest_from_bytes(tree_root_bytes, OcsNonInclusionProof::TREE_ROOT_FIELD.name)?;
        Ok(SdkOcsNonInclusionProof {
            non_inclusion_proof,
            tree_root,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use prost::bytes::Bytes;
    use sui_sdk_types::merkle::MerkleTree;

    /// A round-trip through the proto types preserves the proof's path
    /// node-by-node and the recovered proof verifies against the same tree
    /// root.
    #[test]
    fn merkle_proof_round_trip_via_proto() {
        const LEAVES: [&[u8]; 9] = [
            b"foo", b"bar", b"fizz", b"baz", b"buzz", b"fizz", b"foobar", b"walrus", b"fizz",
        ];

        let tree = MerkleTree::build_from_serialized(LEAVES);
        for (index, leaf) in LEAVES.iter().enumerate() {
            let original = tree.get_proof(index).unwrap();

            let proto: MerkleProof = (&original).into();
            let round_tripped: sui_sdk_types::merkle::MerkleProof = (&proto).try_into().unwrap();

            assert_eq!(round_tripped, original);
            round_tripped
                .verify_proof_with_leaf_bytes(&tree.root(), leaf, index)
                .unwrap();
        }
    }

    #[test]
    fn merkle_node_empty_round_trip() {
        let proto: MerkleNode = Node::Empty.into();
        assert!(matches!(proto.node, Some(merkle_node::Node::Empty(_))));
        let back: Node = (&proto).try_into().unwrap();
        assert_eq!(back, Node::Empty);
    }

    #[test]
    fn merkle_node_digest_round_trip() {
        let raw = [0xab; DIGEST_LEN];
        let proto: MerkleNode = Node::Digest(raw).into();
        match &proto.node {
            Some(merkle_node::Node::Digest(bytes)) => assert_eq!(bytes.as_ref(), raw),
            other => panic!("expected Digest variant, got {other:?}"),
        }
        let back: Node = (&proto).try_into().unwrap();
        assert_eq!(back, Node::Digest(raw));
    }

    #[test]
    fn merkle_node_missing_oneof_rejected() {
        let proto = MerkleNode { node: None };
        let err = Node::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "node");
    }

    #[test]
    fn merkle_node_short_digest_rejected() {
        let proto = MerkleNode {
            node: Some(merkle_node::Node::Digest(Bytes::from_static(&[0u8; 16]))),
        };
        let err = Node::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "digest");
        assert!(
            err.to_string().contains("expected 32 bytes"),
            "error should mention expected length: {err}"
        );
    }

    #[test]
    fn merkle_node_long_digest_rejected() {
        let proto = MerkleNode {
            node: Some(merkle_node::Node::Digest(Bytes::from_static(&[0u8; 64]))),
        };
        let err = Node::try_from(&proto).unwrap_err();
        assert_eq!(err.field_violation().field, "digest");
    }

    /// A malformed inner node's field path includes its position in the
    /// outer `path` list so the failure points at the bad index.
    #[test]
    fn malformed_inner_node_reports_path_index() {
        let mut tree_proof: MerkleProof = (&MerkleTree::build_from_serialized([b"a", b"b"])
            .get_proof(1)
            .unwrap())
            .into();
        // Corrupt the second sibling by stripping the oneof.
        tree_proof.path[0].node = None;

        let err = sui_sdk_types::merkle::MerkleProof::try_from(&tree_proof).unwrap_err();
        let field = &err.field_violation().field;
        assert!(
            field.contains("path[0]") || field.contains("path.0"),
            "field path should reference path index, got {field}"
        );
    }

    /// Build a small set of `ObjectReference` leaves sorted by id; used
    /// by the inclusion / non-inclusion round-trip tests.
    fn synthetic_refs(count: u8) -> Vec<ObjectReference> {
        (0..count)
            .map(|i| {
                let mut addr = [0u8; 32];
                addr[31] = i;
                let mut digest = [0u8; 32];
                digest[0] = i ^ 0x42;
                ObjectReference::new(
                    sui_sdk_types::Address::new(addr),
                    u64::from(i) + 1,
                    Digest::new(digest),
                )
            })
            .collect()
    }

    /// `OcsInclusionProof` round-trips through its proto representation:
    /// the recovered proof equals the original (modulo the wire-only
    /// `object_ref` and `object_data` slots that this conversion does
    /// not own — see the `OcsInclusionProof::from` doc).
    #[test]
    fn ocs_inclusion_proof_round_trip_via_proto() {
        let refs = synthetic_refs(5);
        let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
        let tree_root = Digest::new(tree.root().bytes());

        for (index, _) in refs.iter().enumerate() {
            let original = SdkOcsInclusionProof {
                merkle_proof: tree.get_proof(index).unwrap(),
                leaf_index: index as u64,
                tree_root,
            };
            let proto: OcsInclusionProof = (&original).into();
            let round_tripped: SdkOcsInclusionProof = (&proto).try_into().unwrap();
            assert_eq!(round_tripped, original);
        }
    }

    /// `OcsNonInclusionProof` round-trips through its proto
    /// representation across the three structural shapes the
    /// underlying `MerkleNonInclusionProof` admits: both neighbours
    /// present (interior target), no left neighbour (target sorts
    /// before all leaves), and no right neighbour (target sorts after
    /// all leaves).
    #[test]
    fn ocs_non_inclusion_proof_round_trip_via_proto() {
        let refs = synthetic_refs(5);
        let tree = MerkleTree::build_from_unserialized(&refs).unwrap();
        let tree_root = Digest::new(tree.root().bytes());

        // Interior: a synthetic ref whose id sorts between two leaves.
        let interior = {
            let mut addr = [0u8; 32];
            addr[31] = 0x02;
            ObjectReference::new(
                sui_sdk_types::Address::new(addr),
                10,
                Digest::new([0xaa; 32]),
            )
        };
        // Before everything.
        let before = ObjectReference::new(
            sui_sdk_types::Address::new([0u8; 32]),
            0,
            Digest::new([0u8; 32]),
        );
        // After everything.
        let after = {
            let mut addr = [0u8; 32];
            addr[31] = 0xff;
            ObjectReference::new(
                sui_sdk_types::Address::new(addr),
                999,
                Digest::new([0xff; 32]),
            )
        };

        for target in [interior, before, after] {
            assert!(!refs.contains(&target));
            let inner = tree.compute_non_inclusion_proof(&refs, &target).unwrap();
            let original = SdkOcsNonInclusionProof {
                non_inclusion_proof: inner,
                tree_root,
            };
            let proto: OcsNonInclusionProof = (&original).into();
            let round_tripped: SdkOcsNonInclusionProof = (&proto).try_into().unwrap();
            assert_eq!(round_tripped, original);
        }
    }

    /// A `MerkleNeighbourLeaf` missing its `leaf` field is rejected with
    /// a field-path-attributed error.
    #[test]
    fn merkle_neighbour_leaf_missing_leaf_rejected() {
        let proto = MerkleNeighbourLeaf {
            leaf: None,
            merkle_proof: Some(MerkleProof::default()),
        };
        let err =
            <(ObjectReference, sui_sdk_types::merkle::MerkleProof)>::try_from(&proto).unwrap_err();
        assert_eq!(
            err.field_violation().field,
            MerkleNeighbourLeaf::LEAF_FIELD.name
        );
    }

    /// A `MerkleNeighbourLeaf` missing its `merkle_proof` field is
    /// rejected with a field-path-attributed error.
    #[test]
    fn merkle_neighbour_leaf_missing_merkle_proof_rejected() {
        let leaf_ref = synthetic_refs(1).pop().unwrap();
        let proto = MerkleNeighbourLeaf {
            leaf: Some(leaf_ref.into()),
            merkle_proof: None,
        };
        let err =
            <(ObjectReference, sui_sdk_types::merkle::MerkleProof)>::try_from(&proto).unwrap_err();
        assert_eq!(
            err.field_violation().field,
            MerkleNeighbourLeaf::MERKLE_PROOF_FIELD.name,
        );
    }

    /// A `MerkleNonInclusionProof` missing its `index` is rejected.
    #[test]
    fn merkle_non_inclusion_proof_missing_index_rejected() {
        let proto = MerkleNonInclusionProof::default();
        let err = SdkMerkleNonInclusionProof::<ObjectReference>::try_from(&proto).unwrap_err();
        assert_eq!(
            err.field_violation().field,
            MerkleNonInclusionProof::INDEX_FIELD.name,
        );
    }
}
