use crate::crypto::hasher::Hasher;
use super::header::Header;
use super::body::Body;

pub struct Block {
    pub header: Header,
    pub body: Body,
}

impl Block {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.header.as_bytes());
        bytes.extend_from_slice(&self.body.hash());
        bytes
    }

    pub fn hash(&self) -> Vec<u8> {
        self.as_bytes().hash()
    }
}
