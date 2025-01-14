use crate::crypto::hasher::Hasher;
use crate::common::error::Error;

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

    pub fn hash(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.version.to_le_bytes());
        bytes.extend_from_slice(&self.timestamp.to_le_bytes());
        bytes.extend_from_slice(&self.size.to_le_bytes());
        bytes.extend_from_slice(&self.sender_address.as_bytes());
        bytes.extend_from_slice(&self.receiver_address.as_bytes());
        bytes.extend_from_slice(&self.fee.to_le_bytes());
        bytes.extend_from_slice(&self.nonce.to_le_bytes());
        bytes.extend_from_slice(&self.hash.as_bytes());
        bytes.hash()
    }

    pub fn verify(&self) -> Result<(), Error> {
        unimplemented!()
    }
}