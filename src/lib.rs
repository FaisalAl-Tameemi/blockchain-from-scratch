pub mod crypto;
pub mod common;
pub mod block;
pub mod transaction;

// Re-export some structs for easier access
pub use crypto::merkle::MerkleTree;
pub use transaction::transaction::Transaction;
