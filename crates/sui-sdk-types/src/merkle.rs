//! Blake2b256 Merkle tree.
//!
//! This module implements a binary Merkle tree using Blake2b256 and the
//! RFC-6962-style domain-separation scheme:
//!
//! - leaves are hashed as `BLAKE2b-256(0x00 || leaf_bytes)`,
//! - inner nodes are hashed as `BLAKE2b-256(0x01 || left_bytes || right_bytes)`,
//! - empty subtrees are represented as the all-zero 32-byte node.
//!
//! Levels with an odd number of nodes are padded with one [`Node::Empty`]
//! sibling on the right.
//!
//! None of the types in this module implement `Serialize` or `Deserialize`:
//! the Merkle tree and its proofs are in-memory constructs. Code that needs
//! to transport a proof across a process boundary should define its own wire
//! representation and convert at the boundary.

use serde::Serialize;

use crate::Digest;
use crate::hash::Hasher;

/// Byte length of a Merkle digest (32 bytes; Blake2b256).
pub const DIGEST_LEN: usize = 32;

/// Domain-separation prefix applied to every leaf before hashing.
pub const LEAF_PREFIX: [u8; 1] = [0x00];

/// Domain-separation prefix applied to every inner node before hashing.
pub const INNER_PREFIX: [u8; 1] = [0x01];

/// The 32-byte representation of an empty subtree.
pub const EMPTY_NODE: [u8; DIGEST_LEN] = [0u8; DIGEST_LEN];

/// A node in the Merkle tree.
///
/// `Empty` represents an empty subtree (used to pad odd-sized levels and to
/// represent the root of a tree built from zero leaves). `Digest` carries the
/// 32-byte Blake2b256 hash of either a leaf or an inner node.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    /// A subtree with no leaves underneath it.
    Empty,
    /// A non-empty node carrying its 32-byte hash.
    Digest([u8; DIGEST_LEN]),
}

impl Node {
    /// Returns the 32-byte representation of this node. `Empty` is rendered
    /// as the all-zero digest.
    pub fn bytes(&self) -> [u8; DIGEST_LEN] {
        match self {
            Self::Empty => EMPTY_NODE,
            Self::Digest(value) => *value,
        }
    }
}

impl AsRef<[u8]> for Node {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Empty => &EMPTY_NODE,
            Self::Digest(value) => value.as_ref(),
        }
    }
}

impl From<[u8; DIGEST_LEN]> for Node {
    fn from(value: [u8; DIGEST_LEN]) -> Self {
        Self::Digest(value)
    }
}

impl From<Digest> for Node {
    fn from(value: Digest) -> Self {
        Self::Digest(value.into_inner())
    }
}

/// An error returned by Merkle tree construction or proof verification.
#[derive(Debug, PartialEq, Eq)]
pub enum MerkleError {
    /// The proof does not authenticate the leaf at the given index against
    /// the provided root.
    InvalidProof,
    /// The caller supplied an invalid argument (e.g. a leaf index out of
    /// bounds, or a leaf that could not be BCS-serialized).
    InvalidInput,
}

impl std::fmt::Display for MerkleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidProof => f.write_str("invalid merkle proof"),
            Self::InvalidInput => f.write_str("invalid merkle input"),
        }
    }
}

impl std::error::Error for MerkleError {}

/// An inclusion proof for a leaf in a [`MerkleTree`].
///
/// The path lists each sibling hash on the way from the leaf up to the root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerkleProof {
    path: Vec<Node>,
}

impl MerkleProof {
    /// Construct a proof directly from a sibling path. Mostly useful for
    /// tests and for callers reconstructing a proof from out-of-band data;
    /// most users obtain proofs via [`MerkleTree::get_proof`].
    pub fn new(path: Vec<Node>) -> Self {
        Self { path }
    }

    /// The sibling hashes on the path from the leaf to the root, leaf-side
    /// first.
    pub fn path(&self) -> &[Node] {
        &self.path
    }

    /// Verify that the leaf identified by the given canonical bytes at
    /// position `leaf_index` is included in the tree whose root is `root`.
    ///
    /// `leaf_bytes` must be the same byte representation that was hashed
    /// into the tree when it was built — for trees built via
    /// [`MerkleTree::build_from_unserialized`] that is the leaf's BCS
    /// encoding. Most callers that already have the leaf in its structured
    /// form should use [`verify_proof`] instead, which performs that BCS
    /// step internally.
    ///
    /// [`verify_proof`]: MerkleProof::verify_proof
    pub fn verify_proof_with_leaf_bytes(
        &self,
        root: &Node,
        leaf_bytes: &[u8],
        leaf_index: usize,
    ) -> Result<(), MerkleError> {
        match self.compute_root(leaf_bytes, leaf_index) {
            Some(computed) if &computed == root => Ok(()),
            _ => Err(MerkleError::InvalidProof),
        }
    }

    /// Verify that `leaf` at position `leaf_index` is included in the tree
    /// whose root is `root`.
    ///
    /// The leaf is BCS-encoded internally to obtain the canonical byte
    /// representation that the tree commits to. Use
    /// [`verify_proof_with_leaf_bytes`] when the caller already has those
    /// bytes.
    ///
    /// [`verify_proof_with_leaf_bytes`]: MerkleProof::verify_proof_with_leaf_bytes
    pub fn verify_proof<L: Serialize>(
        &self,
        root: &Node,
        leaf: &L,
        leaf_index: usize,
    ) -> Result<(), MerkleError> {
        let bytes = bcs::to_bytes(leaf).map_err(|_| MerkleError::InvalidInput)?;
        self.verify_proof_with_leaf_bytes(root, &bytes, leaf_index)
    }

    /// Recompute the root from the proof and `leaf` at `leaf_index`.
    ///
    /// Returns `None` if `leaf_index` cannot fit in a tree of
    /// `self.path.len()` levels (which would imply the proof was tampered
    /// with or never matched this index).
    pub fn compute_root(&self, leaf: &[u8], leaf_index: usize) -> Option<Node> {
        if leaf_index >> self.path.len() != 0 {
            return None;
        }
        let mut current = leaf_hash(leaf);
        let mut level_index = leaf_index;
        for sibling in &self.path {
            current = if level_index.is_multiple_of(2) {
                inner_hash(&current, sibling)
            } else {
                inner_hash(sibling, &current)
            };
            level_index /= 2;
        }
        Some(current)
    }

    /// Whether this proof identifies the right-most leaf in its tree.
    ///
    /// A right-most proof has the property that every step where the leaf is
    /// a left child carries an `Empty` sibling — the only way to be on the
    /// extreme right of a padded power-of-two tree.
    pub fn is_right_most(&self, leaf_index: usize) -> bool {
        let mut level_index = leaf_index;
        for sibling in &self.path {
            if level_index.is_multiple_of(2) && sibling.as_ref() != EMPTY_NODE.as_ref() {
                return false;
            }
            level_index /= 2;
        }
        true
    }
}

/// A Blake2b256 Merkle tree.
///
/// The tree is built once from a set of leaves and then queried for its
/// [`root`] or for [`MerkleProof`]s.
///
/// [`root`]: MerkleTree::root
#[derive(Debug)]
pub struct MerkleTree {
    /// Nodes laid out level by level, leaves first. Padding `Empty` nodes
    /// are pushed in-band so the buffer is contiguous.
    nodes: Vec<Node>,
    n_leaves: usize,
}

impl MerkleTree {
    /// Build a tree by hashing each input as a leaf.
    pub fn build_from_serialized<I>(iter: I) -> Self
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
        I::Item: AsRef<[u8]>,
    {
        let leaf_hashes = iter
            .into_iter()
            .map(|leaf| leaf_hash(leaf.as_ref()))
            .collect::<Vec<_>>();
        Self::build_from_leaf_hashes(leaf_hashes)
    }

    /// Build a tree by BCS-encoding each input and using its hash as a leaf.
    ///
    /// Returns [`MerkleError::InvalidInput`] if any leaf fails BCS
    /// serialization, which in practice can only happen for unusual `Serialize`
    /// impls (e.g. infinite recursion); ordinary value types do not fail.
    pub fn build_from_unserialized<I>(iter: I) -> Result<Self, MerkleError>
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
        I::Item: Serialize,
    {
        let leaf_hashes = iter
            .into_iter()
            .map(|leaf| {
                bcs::to_bytes(&leaf)
                    .map_err(|_| MerkleError::InvalidInput)
                    .map(|bytes| leaf_hash(&bytes))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::build_from_leaf_hashes(leaf_hashes))
    }

    /// Build a tree directly from already-hashed leaves.
    ///
    /// Each input node is interpreted as a leaf hash; callers are responsible
    /// for applying the [`LEAF_PREFIX`] before hashing.
    pub fn build_from_leaf_hashes<I>(iter: I) -> Self
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator<Item = Node>,
    {
        let iter = iter.into_iter();
        let mut nodes = Vec::with_capacity(n_nodes(iter.len()));
        nodes.extend(iter);

        let n_leaves = nodes.len();
        let mut level_nodes = n_leaves;
        let mut prev_level_index = 0;

        while level_nodes > 1 {
            if level_nodes.is_multiple_of(2) {
                // Even level: nothing to do.
            } else {
                nodes.push(Node::Empty);
                level_nodes += 1;
            }
            let new_level_index = prev_level_index + level_nodes;
            (prev_level_index..new_level_index)
                .step_by(2)
                .for_each(|index| nodes.push(inner_hash(&nodes[index], &nodes[index + 1])));
            prev_level_index = new_level_index;
            level_nodes /= 2;
        }

        Self { nodes, n_leaves }
    }

    /// The Merkle root.
    ///
    /// An empty tree (built from zero leaves) returns [`Node::Empty`].
    pub fn root(&self) -> Node {
        self.nodes.last().copied().unwrap_or(Node::Empty)
    }

    /// The number of leaves the tree was built from.
    pub fn n_leaves(&self) -> usize {
        self.n_leaves
    }

    /// Build an inclusion proof for the leaf at `leaf_index`.
    ///
    /// Returns [`MerkleError::InvalidInput`] if `leaf_index >= n_leaves`.
    pub fn get_proof(&self, leaf_index: usize) -> Result<MerkleProof, MerkleError> {
        if leaf_index >= self.n_leaves {
            return Err(MerkleError::InvalidInput);
        }
        let path_capacity = self
            .n_leaves
            .checked_ilog2()
            .map(|log| log as usize + 1)
            .unwrap_or(0);
        let mut path = Vec::with_capacity(path_capacity);
        let mut level_index = leaf_index;
        let mut n_level = self.n_leaves;
        let mut level_base_index = 0;
        while n_level > 1 {
            // Every level has an even number of nodes (padding is in-band).
            n_level = n_level.next_multiple_of(2);
            let sibling_index = if level_index.is_multiple_of(2) {
                level_base_index + level_index + 1
            } else {
                level_base_index + level_index - 1
            };
            path.push(self.nodes[sibling_index]);
            level_index /= 2;
            level_base_index += n_level;
            n_level /= 2;
        }
        Ok(MerkleProof { path })
    }

    /// Build a non-inclusion proof showing that `target` does not appear in
    /// this tree.
    ///
    /// The tree must have been built from `sorted_leaves` in sorted order;
    /// the caller is responsible for passing the same sorted slice it built
    /// the tree from. Returns [`MerkleError::InvalidInput`] if `target` is
    /// actually present in `sorted_leaves`.
    pub fn compute_non_inclusion_proof<L>(
        &self,
        sorted_leaves: &[L],
        target: &L,
    ) -> Result<MerkleNonInclusionProof<L>, MerkleError>
    where
        L: Ord + Clone,
    {
        let position = sorted_leaves.partition_point(|leaf| leaf <= target);
        if position > 0 && &sorted_leaves[position - 1] == target {
            return Err(MerkleError::InvalidInput);
        }

        let left_leaf = if position > 0 {
            Some((
                sorted_leaves[position - 1].clone(),
                self.get_proof(position - 1)?,
            ))
        } else {
            None
        };

        let right_leaf = if position < sorted_leaves.len() {
            Some((sorted_leaves[position].clone(), self.get_proof(position)?))
        } else {
            None
        };

        Ok(MerkleNonInclusionProof {
            index: position,
            left_leaf,
            right_leaf,
        })
    }
}

/// A non-inclusion proof for a tree built over **sorted** leaves.
///
/// The proof shows that a target leaf does not appear in the tree by
/// presenting its two neighbours in sorted order (with their inclusion
/// proofs) and asserting `left < target < right`.
///
/// Edge cases:
/// - For an empty tree, both neighbours are `None` and `index` is `0`; any
///   target is automatically non-included.
/// - When the target sorts before every leaf, `left_leaf` is `None`,
///   `right_leaf` is the first leaf, and `index` is `0`.
/// - When the target sorts after every leaf, `right_leaf` is `None` and
///   `left_leaf` is the right-most leaf in the tree (which is detected via
///   [`MerkleProof::is_right_most`] during verification).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerkleNonInclusionProof<L> {
    /// The position where `target` would be inserted into the sorted leaf
    /// list to keep it sorted. `left_leaf` lives at `index - 1` (when
    /// present), `right_leaf` at `index`.
    index: usize,
    /// Inclusion proof for the leaf immediately less than `target`, or
    /// `None` if `target` sorts before every leaf.
    left_leaf: Option<(L, MerkleProof)>,
    /// Inclusion proof for the leaf immediately greater than `target`, or
    /// `None` if `target` sorts after every leaf.
    right_leaf: Option<(L, MerkleProof)>,
}

impl<L> MerkleNonInclusionProof<L> {
    /// Construct a proof directly from its parts. Mostly useful for
    /// reconstructing a proof from out-of-band data; the usual way to
    /// produce a proof is [`MerkleTree::compute_non_inclusion_proof`].
    pub fn new(
        index: usize,
        left_leaf: Option<(L, MerkleProof)>,
        right_leaf: Option<(L, MerkleProof)>,
    ) -> Self {
        Self {
            index,
            left_leaf,
            right_leaf,
        }
    }

    /// The position where `target` would be inserted to keep the leaf list
    /// sorted.
    pub fn index(&self) -> usize {
        self.index
    }

    /// The left neighbour leaf and its inclusion proof, if any.
    pub fn left_leaf(&self) -> Option<&(L, MerkleProof)> {
        self.left_leaf.as_ref()
    }

    /// The right neighbour leaf and its inclusion proof, if any.
    pub fn right_leaf(&self) -> Option<&(L, MerkleProof)> {
        self.right_leaf.as_ref()
    }
}

impl<L> MerkleNonInclusionProof<L>
where
    L: Ord + Serialize,
{
    /// Verify that `target` is not in the tree whose root is `root`.
    pub fn verify_proof(&self, root: &Node, target: &L) -> Result<(), MerkleError> {
        // An empty tree contains nothing, so non-inclusion is trivial.
        if root.as_ref() == EMPTY_NODE.as_ref() {
            return Ok(());
        }

        let right_leaf_index = self.index;
        let left_leaf_with_index = self.left_leaf.as_ref().zip(self.index.checked_sub(1));

        if let Some(((left_leaf, left_proof), left_leaf_index)) = left_leaf_with_index {
            left_proof.verify_proof(root, left_leaf, left_leaf_index)?;
            if left_leaf >= target {
                return Err(MerkleError::InvalidProof);
            }
        } else if right_leaf_index != 0 || self.right_leaf.is_none() {
            // Without a left neighbour we must be inserting at the very
            // start of the leaf list and there must be a right neighbour
            // to compare against.
            return Err(MerkleError::InvalidProof);
        }

        if let Some((right_leaf, right_proof)) = &self.right_leaf {
            right_proof.verify_proof(root, right_leaf, right_leaf_index)?;
            if right_leaf <= target {
                return Err(MerkleError::InvalidProof);
            }
        } else if let Some(((_, left_proof), left_leaf_index)) = left_leaf_with_index {
            // Without a right neighbour the left neighbour must be the
            // tree's right-most leaf, otherwise the gap is fictional.
            if !left_proof.is_right_most(left_leaf_index) {
                return Err(MerkleError::InvalidProof);
            }
        } else {
            return Err(MerkleError::InvalidProof);
        }

        Ok(())
    }
}

/// Hash a leaf with the [`LEAF_PREFIX`] domain separator.
pub(crate) fn leaf_hash(input: &[u8]) -> Node {
    let mut hasher = Hasher::new();
    hasher.update(LEAF_PREFIX);
    hasher.update(input);
    Node::Digest(hasher.finalize().into_inner())
}

/// Hash two child nodes with the [`INNER_PREFIX`] domain separator.
fn inner_hash(left: &Node, right: &Node) -> Node {
    let mut hasher = Hasher::new();
    hasher.update(INNER_PREFIX);
    hasher.update(left.bytes());
    hasher.update(right.bytes());
    Node::Digest(hasher.finalize().into_inner())
}

/// Total number of nodes stored when building a tree of `n_leaves` leaves
/// (including in-band `Empty` padding on every level).
pub(crate) fn n_nodes(n_leaves: usize) -> usize {
    let mut lvl_nodes = n_leaves;
    let mut tot_nodes = 0;
    while lvl_nodes > 1 {
        lvl_nodes += lvl_nodes % 2;
        tot_nodes += lvl_nodes;
        lvl_nodes /= 2;
    }
    tot_nodes + lvl_nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    const TEST_INPUT: [&[u8]; 9] = [
        b"foo", b"bar", b"fizz", b"baz", b"buzz", b"fizz", b"foobar", b"walrus", b"fizz",
    ];

    #[test]
    fn n_nodes_formula() {
        assert_eq!(n_nodes(0), 0);
        assert_eq!(n_nodes(1), 1);
        assert_eq!(n_nodes(2), 3);
        assert_eq!(n_nodes(3), 7);
        assert_eq!(n_nodes(4), 7);
        assert_eq!(n_nodes(5), 13);
        assert_eq!(n_nodes(6), 13);
        assert_eq!(n_nodes(7), 15);
        assert_eq!(n_nodes(8), 15);
        assert_eq!(n_nodes(9), 23);
    }

    #[test]
    fn empty_tree_root_is_empty_node() {
        let tree = MerkleTree::build_from_serialized::<[&[u8]; 0]>([]);
        assert_eq!(tree.root().bytes(), EMPTY_NODE);
    }

    #[test]
    fn single_element_tree_root_is_leaf_hash() {
        let leaf = b"Test";
        let tree = MerkleTree::build_from_serialized([leaf.as_ref()]);
        // Direct comparison with the leaf hash construction.
        let mut hasher = Hasher::new();
        hasher.update(LEAF_PREFIX);
        hasher.update(leaf);
        assert_eq!(tree.root().bytes(), hasher.finalize().into_inner());
    }

    #[test]
    fn empty_element_tree_root_is_hash_of_empty_leaf() {
        let tree = MerkleTree::build_from_serialized([&[][..]]);
        let mut hasher = Hasher::new();
        hasher.update(LEAF_PREFIX);
        hasher.update::<&[u8]>(&[]);
        assert_eq!(tree.root().bytes(), hasher.finalize().into_inner());
    }

    #[test]
    fn get_proof_out_of_bounds() {
        for i in 0..TEST_INPUT.len() {
            let tree = MerkleTree::build_from_serialized(&TEST_INPUT[..i]);
            assert_eq!(
                tree.get_proof(i.next_power_of_two()),
                Err(MerkleError::InvalidInput),
            );
        }
    }

    #[test]
    fn every_proof_round_trips() {
        for i in 0..TEST_INPUT.len() {
            let tree = MerkleTree::build_from_serialized(&TEST_INPUT[..i]);
            for (index, leaf) in TEST_INPUT[..i].iter().enumerate() {
                let proof = tree.get_proof(index).unwrap();
                proof
                    .verify_proof_with_leaf_bytes(&tree.root(), leaf, index)
                    .unwrap();
            }
        }
    }

    #[test]
    fn proof_fails_for_wrong_index() {
        for i in 1..TEST_INPUT.len() {
            let tree = MerkleTree::build_from_serialized(&TEST_INPUT[..i]);
            for (index, leaf) in TEST_INPUT[..i].iter().enumerate() {
                let proof = tree.get_proof(index).unwrap();
                assert_eq!(
                    proof.verify_proof_with_leaf_bytes(&tree.root(), leaf, index + 1),
                    Err(MerkleError::InvalidProof),
                );
            }
        }
    }

    #[test]
    fn proof_fails_for_tampered_leaf() {
        let tree = MerkleTree::build_from_serialized(TEST_INPUT);
        let proof = tree.get_proof(3).unwrap();
        let tampered = b"not-the-real-leaf";
        assert_eq!(
            proof.verify_proof_with_leaf_bytes(&tree.root(), tampered, 3),
            Err(MerkleError::InvalidProof),
        );
    }

    #[test]
    fn proof_fails_against_wrong_root() {
        let tree = MerkleTree::build_from_serialized(TEST_INPUT);
        let proof = tree.get_proof(2).unwrap();
        let wrong_root = Node::Digest([0xab; DIGEST_LEN]);
        assert_eq!(
            proof.verify_proof_with_leaf_bytes(&wrong_root, TEST_INPUT[2], 2),
            Err(MerkleError::InvalidProof),
        );
    }

    #[test]
    fn is_right_most_detects_last_leaf() {
        for i in 1..TEST_INPUT.len() {
            let tree = MerkleTree::build_from_serialized(&TEST_INPUT[..i]);
            for j in 0..i {
                let proof = tree.get_proof(j).unwrap();
                let expected = j == i - 1;
                assert_eq!(proof.is_right_most(j), expected);
            }
        }
    }

    /// Non-inclusion proof for an empty tree: any target is trivially
    /// non-included, both neighbours are absent, and the index is zero.
    #[test]
    fn non_inclusion_empty_tree() {
        let tree = MerkleTree::build_from_unserialized::<[&[u8]; 0]>([]).unwrap();
        let leaves: [&[u8]; 0] = [];
        let proof = tree
            .compute_non_inclusion_proof(&leaves, &b"foo".as_ref())
            .unwrap();
        assert!(proof.left_leaf().is_none());
        assert!(proof.right_leaf().is_none());
        assert_eq!(proof.index(), 0);
        proof.verify_proof(&tree.root(), &b"foo".as_ref()).unwrap();
        proof.verify_proof(&tree.root(), &b"bar".as_ref()).unwrap();
    }

    /// Single-leaf tree: a different target is non-included; the leaf
    /// itself cannot have a non-inclusion proof constructed.
    #[test]
    fn non_inclusion_single_leaf() {
        let leaves: [&[u8]; 1] = [b"foo"];
        let tree = MerkleTree::build_from_unserialized(&leaves).unwrap();

        let proof = tree
            .compute_non_inclusion_proof(&leaves, &b"bar".as_ref())
            .unwrap();
        proof.verify_proof(&tree.root(), &b"bar".as_ref()).unwrap();

        // Re-using a proof against a leaf that is in the tree must fail.
        assert_eq!(
            proof.verify_proof(&tree.root(), &b"foo".as_ref()),
            Err(MerkleError::InvalidProof),
        );

        // Asking for a non-inclusion proof of a leaf that is in the tree
        // is a caller error.
        assert_eq!(
            tree.compute_non_inclusion_proof(&leaves, &b"foo".as_ref()),
            Err(MerkleError::InvalidInput),
        );
    }

    /// Many-leaf tree: every non-tree target verifies as non-included, and
    /// every tree-member target is rejected by the constructor.
    #[test]
    fn non_inclusion_multiple_leaves() {
        const RAW: [&str; 9] = [
            "foo", "bar", "fizz", "baz", "buzz", "fizz", "foobar", "walrus", "fizz",
        ];
        let mut sorted: Vec<&str> = RAW.to_vec();
        sorted.sort();
        sorted.dedup();
        let tree = MerkleTree::build_from_unserialized(&sorted).unwrap();

        // Mix of targets that are not in the tree and targets that are.
        let probes = ["fuzz", "yankee", "aloha", "foo", "bar", "fizz", "walrus"];
        for probe in probes {
            let result = tree.compute_non_inclusion_proof(&sorted, &probe);
            if sorted.contains(&probe) {
                assert_eq!(result, Err(MerkleError::InvalidInput));
            } else {
                let proof = result.unwrap();
                proof.verify_proof(&tree.root(), &probe).unwrap();
            }
        }
    }

    /// Targets at the extremes of the sort order land in the two
    /// asymmetric branches (no left neighbour, or no right neighbour with
    /// the left neighbour being the right-most leaf).
    #[test]
    fn non_inclusion_at_extremes() {
        let sorted: [&str; 3] = ["bar", "foo", "qux"];
        let tree = MerkleTree::build_from_unserialized(&sorted).unwrap();

        // Before everything.
        let before = tree.compute_non_inclusion_proof(&sorted, &"aaa").unwrap();
        assert!(before.left_leaf().is_none());
        assert!(before.right_leaf().is_some());
        assert_eq!(before.index(), 0);
        before.verify_proof(&tree.root(), &"aaa").unwrap();

        // After everything.
        let after = tree.compute_non_inclusion_proof(&sorted, &"zzz").unwrap();
        assert!(after.left_leaf().is_some());
        assert!(after.right_leaf().is_none());
        assert_eq!(after.index(), sorted.len());
        after.verify_proof(&tree.root(), &"zzz").unwrap();
    }

    /// Forged proof that claims `index = 0` while also supplying a left
    /// neighbour does not panic and does not verify. Mirrors upstream's
    /// regression test for `is_valid_neighbor` returning sensibly.
    #[test]
    fn non_inclusion_forged_zero_index_with_left_leaf() {
        let leaves: [&[u8]; 1] = [b"foo"];
        let tree = MerkleTree::build_from_unserialized(&leaves).unwrap();
        let fake: &[u8] = b"fake";

        let forged = MerkleNonInclusionProof::new(
            0,
            Some((fake, tree.get_proof(0).unwrap())),
            Some((fake, tree.get_proof(0).unwrap())),
        );
        assert_eq!(
            forged.verify_proof(&tree.root(), &fake),
            Err(MerkleError::InvalidProof),
        );
    }

    /// Pinned root for `TEST_INPUT` under our `LEAF_PREFIX`/`INNER_PREFIX`/
    /// `EMPTY_NODE` scheme. This was captured from upstream
    /// `fastcrypto::merkle::MerkleTree<Blake2b256>::root()`, so a regression
    /// in any of the load-bearing pieces of the hash construction (leaf
    /// prefix, inner prefix, empty-node bytes, padding rule) would change
    /// this value and fail the test.
    #[test]
    fn root_matches_upstream_for_known_input() {
        const EXPECTED_ROOT: [u8; DIGEST_LEN] = [
            0x8d, 0x01, 0x06, 0x76, 0xde, 0x3d, 0x66, 0x08, 0x77, 0xcc, 0x8c, 0x27, 0xa4, 0x2d,
            0xcf, 0xf9, 0xc1, 0x15, 0x97, 0x20, 0x36, 0x1a, 0x82, 0x36, 0xd2, 0xd2, 0x07, 0xb6,
            0x8b, 0x72, 0x9b, 0x0c,
        ];

        let tree = MerkleTree::build_from_serialized(TEST_INPUT);
        assert_eq!(tree.root().bytes(), EXPECTED_ROOT);
    }
}
