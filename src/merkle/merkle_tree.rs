use crate::common::hashable::Sha256Hashable;

/// A Merkle Tree struct that holds the root and the leafs.
/// The leafs are the hashable items that will be used to build the tree and calculate the root.
#[derive(Debug)]
pub struct MerkleTree<T: Sha256Hashable + Clone> {
    pub root: Option<String>,
    // a tree of hashes where each level is represented by a tuple of (index, vector of hashes / nodes)
    // the index is the level within the tree
    pub tree: Vec<(usize, Vec<Vec<u8>>)>,
    pub leafs: Vec<T>,
}

impl<T: Sha256Hashable + Clone> MerkleTree<T> {
    pub fn new(leafs: Option<Vec<T>>) -> Self {
        Self { root: None, leafs: leafs.unwrap_or_default(), tree: vec![] }
    }

    pub fn add_leaf(&mut self, leaf: T) {
        self.leafs.push(leaf);
    }

    pub fn remove_leaf(&mut self, index: usize) {
        self.leafs.remove(index);
    }

    /// Calculate the root of the Merkle Tree.
    /// This is done by hashing every leaf node within the tree and then concatenating the hashes of the nodes.
    /// The process is repeated until there is only one hash left, which is the root.
    /// If there is an odd number of leaves, a node full of zeros is added to the end of the list to make it even.
    /// 
    /// Returns the root hash as a `Vec<u8>`.
    pub fn calculate_root(&self) -> Option<Vec<u8>> {
        // hash every leaf node within the tree
        let mut leafs = self.leafs
            .iter()
            .map(|leaf| leaf.hash())
            .collect::<Vec<_>>();

        while leafs.len() >= 2 {
            // push a node full of zeros to the end of the list to make it even
            if leafs.len() % 2 != 0 {
                leafs.push([0u8; 32].to_vec());
            }
            
            leafs = leafs
                .chunks(2)
                .map(|chunk| chunk[0].concat_hash(&chunk[1]))
                .collect();
        }

        leafs.get(0).cloned()
    }

    pub fn verify(&self, proof: Vec<T>, target: T) -> bool {
        todo!()
    }

    /// Get the proof for a given leaf index such that if the proof is folded
    /// iteratively starting with the leaf at the given index, the root of the tree is obtained.
    /// 
    /// To generate the proof, we simply need to create a vector of the sibling hash nodes that are needed to
    /// fold the tree to the root starting with the leaf at the given index.
    /// 
    /// Returns the proof as a `Vec<Vec<u8>>`.
    pub fn get_proof(&self, leaf_index: usize) -> Option<Vec<Vec<u8>>> {
        let mut leafs = self.leafs
            .iter()
            .map(|leaf| leaf.hash())
            .collect::<Vec<_>>();
        let mut proof = vec![];
        let mut next_leaf_index = leaf_index;

        // push the leaf at the given index to the proof
        proof.push(leafs.get(leaf_index)?.clone());

        while leafs.len() >= 2 {
            // push a node full of zeros to the end of the list to make it even
            if leafs.len() % 2 != 0 {
                leafs.push([0u8; 32].to_vec());
            }

            let sibling_index = match next_leaf_index % 2 {
                0 => next_leaf_index + 1, // if the index is even, the sibling is the next index
                _ => next_leaf_index - 1, // if the index is odd, the sibling is the previous index
            };
            proof.push(leafs.get(sibling_index)?.clone());
            next_leaf_index = match next_leaf_index % 2 {
                0 => next_leaf_index / 2, // if the index is even, the next leaf index is the index of the sibling
                _ => (next_leaf_index - 1) / 2, // if the index is odd, the next leaf index is the index of the sibling
            };

            leafs = leafs
                .chunks(2)
                .map(|chunk| chunk[0].concat_hash(&chunk[1]))
                .collect();
        }

        // push the root to the proof
        proof.push(leafs.get(0)?.clone());

        Some(proof)
    }
}

impl<T: Sha256Hashable + Clone> From<Vec<T>> for MerkleTree<T> {
    fn from(leafs: Vec<T>) -> Self {
        Self { root: None, leafs, tree: vec![] }
    }
}

impl<T: Sha256Hashable + Clone> From<&[T]> for MerkleTree<T> {
    fn from(leafs: &[T]) -> Self {
        Self { root: None, leafs: leafs.to_vec(), tree: vec![] }
    }
}

/// 
/// Tests
/// 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_root() {
        let leafs = vec![
            b"first item".to_vec(),
            b"second item".to_vec(),
            b"third item".to_vec(),
            b"fourth item".to_vec()
        ];
        let tree = MerkleTree::from(leafs);
        let root = tree.calculate_root();
        
        assert_eq!(
            hex::encode(root.unwrap()),
            "91d8059a205751e1abf0f79f2a5d318f3074c10fa002e887b1878898987aedf8"
        );
    }

    #[test]
    fn test_calculate_root_with_odd_number_of_leaves() {
        let leafs = vec![
            b"first item".to_vec(),
            b"second item".to_vec(),
            b"third item".to_vec(),
        ];
        let tree = MerkleTree::from(leafs);
        let root = tree.calculate_root();
        
        assert_eq!(root.is_some(), true);
    }

    #[test]
    fn test_get_proof() {
        let leafs = vec![
            b"first item".to_vec(),
            b"second item".to_vec(),
            b"third item".to_vec(),
            b"fourth item".to_vec(),
            b"fifth item".to_vec(),
        ];
        let tree = MerkleTree::from(leafs);
        let proof = tree.get_proof(1);
        let root = tree.calculate_root().unwrap();

        assert_eq!(proof.is_some(), true);
        assert_eq!(
            proof.unwrap(),
            vec![
                b"second item".hash(),
                b"first item".hash(),
                b"third item".to_vec().concat_hash(&b"fourth item".to_vec()),
                root,
            ]
        );
    }
}
