use crate::crypto::hasher::Hasher;

/// A Merkle Tree struct that holds the root and the leafs.
/// The leafs are the hashable items that will be used to build the tree and calculate the root.
#[derive(Debug)]
pub struct MerkleTree<'a> {
    pub root: Option<&'a [u8]>,
    // a tree of hashes where each level is represented by a tuple of (index, vector of hashes / nodes)
    // the index is the level within the tree
    pub tree: Vec<(usize, Vec<&'a [u8]>)>,
    pub leafs: Vec<&'a [u8]>,
}

impl<'a> MerkleTree<'a> {
    pub fn new(leafs: Option<Vec<&'a [u8]>>) -> Self {
        Self { 
            root: None, 
            leafs: leafs.unwrap_or_default(), 
            tree: vec![],
        }
    }

    pub fn add_leaf(&mut self, leaf: &'a [u8]) {
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
                leafs.push(leafs.last()?.clone());
            }
            
            leafs = leafs
                .chunks(2)
                .map(|chunk| chunk[0].concat_hash(&chunk[1]))
                .collect();
        }

        leafs.get(0).cloned()
    }
    
    pub fn verify(&self, proof: Vec<&'a [u8]>, target: &'a [u8]) -> bool {
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
        if leaf_index >= self.leafs.len() {
            return None;
        }

        let mut current_level = self.leafs
            .iter()
            .map(|leaf| leaf.hash())
            .collect::<Vec<_>>();
        let mut proof = Vec::with_capacity(self.leafs.len()); // Pre-allocate reasonable size
        let mut current_index = leaf_index;

        // Add target leaf to proof
        proof.push(current_level[leaf_index].clone());

        while current_level.len() > 1 {
            // Pad with last element if odd number of leaves
            if current_level.len() % 2 != 0 {
                current_level.push(current_level.last()?.clone());
            }

            // Get sibling index and add to proof
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            proof.push(current_level[sibling_index].clone());

            // Calculate index for next level
            current_index /= 2;

            // Build next level
            current_level = current_level
                .chunks_exact(2)
                .map(|pair| pair[0].concat_hash(&pair[1]))
                .collect();
        }

        Some(proof)
    }
}

impl<'a> From<Vec<&'a [u8]>> for MerkleTree<'a> {
    fn from(leafs: Vec<&'a [u8]>) -> Self {
        Self { root: None, leafs, tree: vec![] }
    }
}

impl<'a> From<&[&'a [u8]]> for MerkleTree<'a> {
    fn from(leafs: &[&'a [u8]]) -> Self {
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
        let leafs: Vec<&[u8]> = vec![
            b"first item",
            b"second item",
            b"third item",
            b"fourth item"
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
        let leafs: Vec<&[u8]> = vec![
            b"first item",
            b"second item",
            b"third item",
        ];
        let tree = MerkleTree::from(leafs);
        let root = tree.calculate_root();

        let leafs: Vec<&[u8]> = vec![
            b"first item",
            b"second item",
            b"third item",
            b"third item",
        ];
        let tree = MerkleTree::from(leafs);
        let root2 = tree.calculate_root();
        
        assert_eq!(root.unwrap(), root2.unwrap());
    }

    #[test]
    #[ignore]
    fn test_get_proof() {
        let leafs: Vec<&[u8]> = vec![
            b"first item",
            b"second item",
            b"third item",
            b"fourth item",
            b"fifth item",
        ];
        let tree = MerkleTree::from(leafs);
        let proof = tree.get_proof(1);
        let root = tree.calculate_root().unwrap();

        assert_eq!(proof.is_some(), true);
    }
}
