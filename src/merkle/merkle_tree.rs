use crate::common::hashable::Sha256Hashable;

/// A Merkle Tree struct that holds the root and the leafs.
/// The leafs are the hashable items that will be used to build the tree and calculate the root.
#[derive(Debug)]
pub struct MerkleTree<T: Sha256Hashable> {
    pub root: Option<String>,
    pub leafs: Vec<T>,
}

impl<T: Sha256Hashable> MerkleTree<T> {
    pub fn new(leafs: Option<Vec<T>>) -> Self {
        Self { root: None, leafs: leafs.unwrap_or_default() }
    }

    pub fn add_leaf(&mut self, leaf: T) {
        self.leafs.push(leaf);
    }

    pub fn remove_leaf(&mut self, index: usize) {
        self.leafs.remove(index);
    }

    pub fn calculate_root(&self) {
        todo!()
    }

    pub fn verify(&self, proof: Vec<T>, target: T) -> bool {
        todo!()
    }

    pub fn get_proof(&self, target: T) -> Vec<T> {
        todo!()
    }
}

impl<T: Sha256Hashable> From<Vec<T>> for MerkleTree<T> {
    fn from(leafs: Vec<T>) -> Self {
        Self { root: None, leafs }
    }
}

impl<T: Sha256Hashable + Clone> From<&[T]> for MerkleTree<T> {
    fn from(leafs: &[T]) -> Self {
        Self { root: None, leafs: leafs.to_vec() }
    }
}
