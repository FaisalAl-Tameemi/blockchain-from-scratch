use crate::{MerkleTree, Transaction};

pub struct Body {
    pub transactions: Vec<Transaction>,
}

impl Body {
    /// The hash of the block is composed of the merkle root of the transactions.
    pub fn hash(&self) -> Vec<u8> {
        let hashes: Vec<Vec<u8>> = self.transactions
            .iter()
            .map(|t| t.hash())
            .collect();
        let leafs: Vec<&[u8]> = hashes.iter()
            .map(|h| h.as_slice())
            .collect();
        let tree = MerkleTree::from(leafs);
        tree.calculate_root();
        tree.root.unwrap().to_vec()
    }
}
