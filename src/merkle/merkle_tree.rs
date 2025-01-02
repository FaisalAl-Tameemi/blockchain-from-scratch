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

    pub fn calculate_root(&self) -> Option<Vec<u8>> {
        // hash every leaf node within the tree
        let mut leafs = self.leafs
            .iter()
            .map(|leaf| leaf.hash())
            .collect::<Vec<_>>();
        let mut levels = vec![];

        while leafs.len() >= 2 {
            // push a node full of zeros to the end of the list to make it even
            if leafs.len() % 2 != 0 {
                leafs.push([0u8; 32].to_vec());
            }
            
            leafs = leafs
                .chunks(2)
                .map(|chunk| chunk[0].concat_hash(&chunk[1]))
                .collect();
            levels.push(leafs.clone());
        }

        leafs.get(0).cloned()
    }

    pub fn verify(&self, proof: Vec<T>, target: T) -> bool {
        todo!()
    }

    pub fn get_proof(&self, target: T) -> Vec<T> {
        todo!()
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
}
