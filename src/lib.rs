pub mod merkle;
pub mod common;

// Re-export the MerkleTree struct for easier access
pub use merkle::merkle_tree::MerkleTree;
pub use common::hashable::Sha256Hashable;