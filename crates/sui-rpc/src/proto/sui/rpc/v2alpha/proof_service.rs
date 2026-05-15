//! Conversions between the v2alpha `ProofService` proto messages and the
//! in-memory Merkle types in `sui_sdk_types::merkle`.
//!
//! The proto wire format encodes Merkle proofs natively — each node is a
//! `oneof { Empty empty, bytes digest }` — rather than carrying a single
//! BCS-encoded blob. The conversions in this module unpack that proto
//! representation into the SDK's in-memory `Node` and `MerkleProof`
//! types, validating that every digest is exactly 32 bytes and that
//! every `MerkleNode` has its `oneof` populated.

use super::*;
use crate::proto::TryFromProtoError;

use sui_sdk_types::merkle::Node;

const DIGEST_LEN: usize = 32;

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
}
