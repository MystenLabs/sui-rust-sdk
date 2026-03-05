// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use alloc::format;
use alloc::vec::Vec;
use core::fmt::Debug;

use blake2::digest::Digest as _;
use blake2::digest::consts::U32;
use serde::Deserialize;
use serde::Serialize;

pub const DIGEST_LEN: usize = 32;

pub const LEAF_PREFIX: [u8; 1] = [0];
pub const INNER_PREFIX: [u8; 1] = [1];
pub const EMPTY_NODE: [u8; DIGEST_LEN] = [0; DIGEST_LEN];

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum Node {
    Empty,
    Digest([u8; DIGEST_LEN]),
}

impl Node {
    pub fn bytes(&self) -> [u8; DIGEST_LEN] {
        match self {
            Node::Empty => EMPTY_NODE,
            Node::Digest(val) => *val,
        }
    }
}

impl From<[u8; DIGEST_LEN]> for Node {
    fn from(value: [u8; DIGEST_LEN]) -> Self {
        Self::Digest(value)
    }
}

impl AsRef<[u8]> for Node {
    fn as_ref(&self) -> &[u8] {
        match self {
            Node::Empty => EMPTY_NODE.as_ref(),
            Node::Digest(val) => val.as_ref(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct MerkleProof {
    path: Vec<Node>,
}

impl MerkleProof {
    pub fn new(path: &[Node]) -> Self {
        Self { path: path.into() }
    }

    pub fn verify_proof(
        &self,
        root: &Node,
        leaf: &[u8],
        leaf_index: usize,
    ) -> Result<(), MerkleError> {
        if self.compute_root(leaf, leaf_index).as_ref() != Some(root) {
            return Err(MerkleError::InvalidProof);
        }
        Ok(())
    }

    pub fn verify_proof_with_unserialized_leaf<L: Serialize>(
        &self,
        root: &Node,
        leaf: &L,
        leaf_index: usize,
    ) -> Result<(), MerkleError> {
        let bytes = bcs::to_bytes(leaf).map_err(|_| MerkleError::InvalidInput)?;
        self.verify_proof(root, &bytes, leaf_index)
    }

    pub fn compute_root(&self, leaf: &[u8], leaf_index: usize) -> Option<Node> {
        if leaf_index >> self.path.len() != 0 {
            return None;
        }
        let mut current_hash = leaf_hash(leaf);
        let mut level_index = leaf_index;
        for sibling in self.path.iter() {
            if level_index.is_multiple_of(2) {
                current_hash = inner_hash(&current_hash, sibling);
            } else {
                current_hash = inner_hash(sibling, &current_hash);
            };
            level_index /= 2;
        }
        Some(current_hash)
    }

    pub fn is_right_most(&self, leaf_index: usize) -> bool {
        let mut level_index = leaf_index;
        for sibling in self.path.iter() {
            if level_index.is_multiple_of(2) && sibling.as_ref() != EMPTY_NODE.as_ref() {
                return false;
            }
            level_index /= 2;
        }
        true
    }
}

#[derive(Serialize, Deserialize)]
#[serde(bound(
    serialize = "L: Serialize",
    deserialize = "L: serde::de::DeserializeOwned"
))]
pub struct MerkleNonInclusionProof<L>
where
    L: Ord + Serialize,
{
    pub index: usize,
    pub left_leaf: Option<(L, MerkleProof)>,
    pub right_leaf: Option<(L, MerkleProof)>,
}

impl<L> MerkleNonInclusionProof<L>
where
    L: Ord + Serialize,
{
    pub fn new(
        left_leaf: Option<(L, MerkleProof)>,
        right_leaf: Option<(L, MerkleProof)>,
        index: usize,
    ) -> Self {
        Self {
            left_leaf,
            right_leaf,
            index,
        }
    }
}

impl<L> Debug for MerkleNonInclusionProof<L>
where
    L: Debug + Ord + Serialize,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!(
            "MerkleNonInclusionProof<L={}>",
            core::any::type_name::<L>()
        ))
        .field("left_leaf", &self.left_leaf)
        .field("right_leaf", &self.right_leaf)
        .field("index", &self.index)
        .finish()
    }
}

impl<L> MerkleNonInclusionProof<L>
where
    L: Ord + Serialize,
{
    fn is_valid_neighbor(
        &self,
        neighbor: &L,
        proof: &MerkleProof,
        neighbor_index: usize,
        root: &Node,
    ) -> Result<(), MerkleError> {
        proof.verify_proof_with_unserialized_leaf(root, neighbor, neighbor_index)
    }

    pub fn verify_proof(&self, root: &Node, target_leaf: &L) -> Result<(), MerkleError> {
        if root.as_ref() == EMPTY_NODE.as_ref() {
            return Ok(());
        }

        let right_leaf_index = self.index;
        let left_leaf_with_idx = self.left_leaf.as_ref().zip(self.index.checked_sub(1));

        if let Some(((left_leaf, left_proof), left_leaf_index)) = &left_leaf_with_idx {
            self.is_valid_neighbor(left_leaf, left_proof, *left_leaf_index, root)?;
            if left_leaf >= target_leaf {
                return Err(MerkleError::InvalidProof);
            }
        } else if right_leaf_index != 0 || self.right_leaf.is_none() {
            return Err(MerkleError::InvalidProof);
        }

        if let Some((right_leaf, right_proof)) = &self.right_leaf {
            self.is_valid_neighbor(right_leaf, right_proof, right_leaf_index, root)?;
            if right_leaf <= target_leaf {
                return Err(MerkleError::InvalidProof);
            }
        } else if let Some(((_, left_proof), left_leaf_index)) = left_leaf_with_idx {
            if !left_proof.is_right_most(left_leaf_index) {
                return Err(MerkleError::InvalidProof);
            }
        } else {
            return Err(MerkleError::InvalidProof);
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct MerkleTree {
    nodes: Vec<Node>,
    n_leaves: usize,
}

impl Debug for MerkleTree {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("MerkleTree")
            .field("nodes", &self.nodes)
            .field("n_leaves", &self.n_leaves)
            .finish()
    }
}

impl MerkleTree {
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
            if level_nodes % 2 == 1 {
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

    pub fn verify_root(&self, root: &Node) -> bool {
        self.root() == *root
    }

    pub fn root(&self) -> Node {
        self.nodes.last().map_or(Node::Empty, |val| val.clone())
    }

    pub fn get_proof(&self, leaf_index: usize) -> Result<MerkleProof, MerkleError> {
        if leaf_index >= self.n_leaves {
            return Err(MerkleError::General(format!(
                "Leaf index out of bounds: {}",
                leaf_index
            )));
        }
        let mut path = Vec::with_capacity(
            usize::try_from(self.n_leaves.ilog2()).expect("this is smaller than `n_leaves`") + 1,
        );
        let mut level_index = leaf_index;
        let mut n_level = self.n_leaves;
        let mut level_base_index = 0;
        while n_level > 1 {
            n_level = n_level.next_multiple_of(2);
            let sibling_index = if level_index.is_multiple_of(2) {
                level_base_index + level_index + 1
            } else {
                level_base_index + level_index - 1
            };
            path.push(self.nodes[sibling_index].clone());
            level_index /= 2;
            level_base_index += n_level;
            n_level /= 2;
        }
        Ok(MerkleProof { path })
    }

    pub fn compute_non_inclusion_proof<L: Ord + Serialize + Clone>(
        &self,
        leaves: &[L],
        target_leaf: &L,
    ) -> Result<MerkleNonInclusionProof<L>, MerkleError> {
        let position = leaves.partition_point(|x| x <= target_leaf);
        if position > 0 && leaves[position - 1] == *target_leaf {
            return Err(MerkleError::General(
                "Target leaf is already in the tree".to_string(),
            ));
        }

        let left_leaf_proof = if position > 0 {
            Some((leaves[position - 1].clone(), self.get_proof(position - 1)?))
        } else {
            None
        };

        let right_leaf_proof = if position < leaves.len() {
            Some((leaves[position].clone(), self.get_proof(position)?))
        } else {
            None
        };

        Ok(MerkleNonInclusionProof::new(
            left_leaf_proof,
            right_leaf_proof,
            position,
        ))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MerkleError {
    #[error("invalid proof")]
    InvalidProof,
    #[error("invalid input")]
    InvalidInput,
    #[error("{0}")]
    General(String),
}

pub(crate) fn leaf_hash(input: &[u8]) -> Node {
    let mut hasher = blake2::Blake2b::<U32>::new();
    hasher.update(LEAF_PREFIX);
    hasher.update(input);
    let result: [u8; DIGEST_LEN] = hasher.finalize().into();
    result.into()
}

fn inner_hash(left: &Node, right: &Node) -> Node {
    let mut hasher = blake2::Blake2b::<U32>::new();
    hasher.update(INNER_PREFIX);
    hasher.update(left.bytes());
    hasher.update(right.bytes());
    let result: [u8; DIGEST_LEN] = hasher.finalize().into();
    result.into()
}

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

    const TEST_INPUT: [&[u8]; 9] = [
        b"foo", b"bar", b"fizz", b"baz", b"buzz", b"fizz", b"foobar", b"walrus", b"fizz",
    ];

    #[test]
    fn test_n_nodes() {
        assert!(n_nodes(0) == 0);
        assert!(n_nodes(1) == 1);
        assert!(n_nodes(2) == 3);
        assert!(n_nodes(3) == 7);
        assert!(n_nodes(4) == 7);
        assert!(n_nodes(5) == 13);
        assert!(n_nodes(6) == 13);
        assert!(n_nodes(7) == 15);
        assert!(n_nodes(8) == 15);
        assert!(n_nodes(9) == 23);
    }

    #[test]
    fn test_merkle_tree_empty() {
        let mt = MerkleTree::build_from_serialized::<[&[u8]; 0]>([]);
        assert_eq!(mt.root().bytes(), EMPTY_NODE);
    }

    #[test]
    fn test_merkle_tree_single_element() {
        let test_inp = "Test";
        let mt = MerkleTree::build_from_serialized(&[test_inp.as_bytes()]);
        let mut hasher = blake2::Blake2b::<U32>::new();
        hasher.update(LEAF_PREFIX);
        hasher.update(test_inp.as_bytes());
        let expected: [u8; DIGEST_LEN] = hasher.finalize().into();
        assert_eq!(mt.root().bytes(), expected);
    }

    #[test]
    fn test_merkle_tree_empty_element() {
        let mt = MerkleTree::build_from_serialized(&[&[] as &[u8]]);
        let mut hasher = blake2::Blake2b::<U32>::new();
        hasher.update(LEAF_PREFIX);
        hasher.update([]);
        let expected: [u8; DIGEST_LEN] = hasher.finalize().into();
        assert_eq!(mt.root().bytes(), expected);
    }

    #[test]
    fn test_get_path_out_of_bounds() {
        let test_inp: Vec<_> = [
            "foo", "bar", "fizz", "baz", "buzz", "fizz", "foobar", "walrus", "fizz",
        ]
        .iter()
        .map(|x| x.as_bytes())
        .collect();
        for i in 0..test_inp.len() {
            let mt = MerkleTree::build_from_serialized(&test_inp[..i]);
            assert!(mt.get_proof(i.next_power_of_two()).is_err());
        }
    }

    #[test]
    fn test_merkle_path_verify() {
        for i in 0..TEST_INPUT.len() {
            let mt = MerkleTree::build_from_serialized(&TEST_INPUT[..i]);
            for (index, leaf_data) in TEST_INPUT[..i].iter().enumerate() {
                let proof = mt.get_proof(index).unwrap();
                assert!(proof.verify_proof(&mt.root(), leaf_data, index).is_ok());
            }
        }
    }

    #[test]
    fn test_merkle_path_verify_fails_for_wrong_index() {
        for i in 0..TEST_INPUT.len() {
            let mt = MerkleTree::build_from_serialized(&TEST_INPUT[..i]);
            for (index, leaf_data) in TEST_INPUT[..i].iter().enumerate() {
                let proof = mt.get_proof(index).unwrap();
                assert!(
                    proof
                        .verify_proof(&mt.root(), leaf_data, index + 1)
                        .is_err()
                );
            }
        }
    }

    #[test]
    fn test_merkle_proof_is_right_most() {
        for i in 0..TEST_INPUT.len() {
            let mt = MerkleTree::build_from_serialized(&TEST_INPUT[..i]);
            for j in 0..i {
                let proof = mt.get_proof(j).unwrap();
                if j == i - 1 {
                    assert!(proof.is_right_most(j));
                } else {
                    assert!(!proof.is_right_most(j));
                }
            }
        }
    }

    #[test]
    fn test_non_inclusion_empty_tree() {
        let mt = MerkleTree::build_from_unserialized::<[&[u8]; 0]>([]).unwrap();
        let non_inclusion_proof = mt
            .compute_non_inclusion_proof(&[], &"foo".as_bytes())
            .unwrap();
        assert!(non_inclusion_proof.left_leaf.is_none());
        assert!(non_inclusion_proof.right_leaf.is_none());
        assert_eq!(non_inclusion_proof.index, 0);
        assert!(
            non_inclusion_proof
                .verify_proof(&mt.root(), &"foo".as_bytes())
                .is_ok()
        );
        assert!(
            non_inclusion_proof
                .verify_proof(&mt.root(), &"bar".as_bytes())
                .is_ok()
        );
    }

    #[test]
    fn test_non_inclusion_single_leaf() {
        let mt = MerkleTree::build_from_unserialized(&["foo".as_bytes()]).unwrap();

        let non_inclusion_proof = mt
            .compute_non_inclusion_proof(&["foo".as_bytes()], &"bar".as_bytes())
            .unwrap();
        assert!(
            non_inclusion_proof
                .verify_proof(&mt.root(), &"bar".as_bytes())
                .is_ok()
        );
        assert!(
            non_inclusion_proof
                .verify_proof(&mt.root(), &"foo".as_bytes())
                .is_err()
        );

        let non_inclusion_proof =
            mt.compute_non_inclusion_proof(&["foo".as_bytes()], &"foo".as_bytes());
        assert!(non_inclusion_proof.is_err());
    }

    #[test]
    fn test_non_inclusion_multiple_leaves() {
        const INPUT: [&str; 9] = [
            "foo", "bar", "fizz", "baz", "buzz", "fizz", "foobar", "walrus", "fizz",
        ];
        let mut sorted = INPUT.to_vec();
        sorted.sort();
        let mt = MerkleTree::build_from_unserialized(&sorted).unwrap();

        let test_cases = [["fuzz", "yankee", "aloha"].to_vec(), INPUT.to_vec()].concat();
        for item in test_cases {
            let non_inclusion_proof = mt.compute_non_inclusion_proof(&sorted, &item);
            if INPUT.contains(&item) {
                assert!(non_inclusion_proof.is_err());
            } else {
                let non_inclusion_proof = non_inclusion_proof.unwrap();
                assert!(non_inclusion_proof.verify_proof(&mt.root(), &item).is_ok());
            }
        }
    }

    #[test]
    fn test_non_inclusion_failure_zero_index_some_left_leaf() {
        let mt = MerkleTree::build_from_unserialized(&["foo".as_bytes()]).unwrap();
        let leaf = "fake_leaf".as_bytes();
        let fake_proof = MerkleNonInclusionProof {
            index: 0,
            left_leaf: Some((leaf, mt.get_proof(0).unwrap())),
            right_leaf: Some((leaf, mt.get_proof(0).unwrap())),
        };
        assert!(fake_proof.verify_proof(&mt.root(), &leaf).is_err());
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TestLeaf {
        pub id: u32,
        pub data: String,
    }

    #[test]
    fn test_serialization_with_blake2b256() {
        let leaf1 = TestLeaf {
            id: 1,
            data: "foo".to_string(),
        };
        let leaf2 = TestLeaf {
            id: 2,
            data: "bar".to_string(),
        };
        let leaf3 = TestLeaf {
            id: 3,
            data: "baz".to_string(),
        };

        let leaves = vec![leaf1.clone(), leaf2.clone(), leaf3.clone()];
        let mt = MerkleTree::build_from_unserialized(&leaves).unwrap();

        let inclusion_proof = mt.get_proof(1).unwrap();
        let serialized =
            serde_json::to_string(&inclusion_proof).expect("Failed to serialize inclusion proof");
        let deserialized: MerkleProof =
            serde_json::from_str(&serialized).expect("Failed to deserialize inclusion proof");

        let leaf2_bytes = bcs::to_bytes(&leaf2).unwrap();
        assert!(
            deserialized
                .verify_proof(&mt.root(), &leaf2_bytes, 1)
                .is_ok()
        );
        assert!(
            deserialized
                .verify_proof_with_unserialized_leaf(&mt.root(), &leaf2, 1)
                .is_ok()
        );

        let target_leaf = TestLeaf {
            id: 4,
            data: "missing".to_string(),
        };
        let non_inclusion_proof = mt
            .compute_non_inclusion_proof(&leaves, &target_leaf)
            .unwrap();

        let serialized = serde_json::to_string(&non_inclusion_proof)
            .expect("Failed to serialize non-inclusion proof");
        let deserialized: MerkleNonInclusionProof<TestLeaf> =
            serde_json::from_str(&serialized).expect("Failed to deserialize non-inclusion proof");

        assert!(deserialized.verify_proof(&mt.root(), &target_leaf).is_ok());
    }
}
