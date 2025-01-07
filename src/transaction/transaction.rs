use crate::common::error::Error;

// The struct of a proof-of-stake transaction
pub struct Transaction {
    // Transaction metadata
    pub version: u32,   // Version of the transaction format
    pub timestamp: u64, // Time when transaction was created
    pub size: u64,      // Size of transaction in bytes

    // Sender and receiver
    pub sender_address: String,
    pub receiver_address: String,

    // Transaction state
    pub status: TransactionStatus, // Current status of transaction
    pub fee: u64,                  // Transaction fee
    pub nonce: u64,                // Transaction sequence number

    // Blockchain location
    pub hash: String,       // Hash of this transaction
    pub block_hash: String, // Hash of containing block
    pub block_height: u64,  // Height of containing block
}

pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

impl Transaction {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        unimplemented!()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        unimplemented!()
    }
}
