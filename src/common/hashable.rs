use sha2::{Sha256, Digest};

pub trait Sha256Hashable {
    fn hash(&self) -> Vec<u8>;
    fn concat_hash(&self, other: &Self) -> Vec<u8>;
    fn hash_to_hex(&self) -> String;
}

impl<T: AsRef<[u8]>> Sha256Hashable for T {
    fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.as_ref());
        hasher.finalize().as_slice().to_vec()
    }

    fn hash_to_hex(&self) -> String {
        hex::encode(self.hash())
    }

    fn concat_hash(&self, other: &Self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.as_ref());
        hasher.update(other.as_ref());
        hasher.finalize().as_slice().to_vec()
    }
}

