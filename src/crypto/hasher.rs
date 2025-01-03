use sha2::{Sha256, Digest};

pub trait Hasher<T: AsRef<[u8]>> {
    fn hash(&self) -> Vec<u8>;
    fn concat_hash(&self, data: &T) -> Vec<u8>;
}

impl<T: AsRef<[u8]>> Hasher<T> for T {
    fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.as_ref());
        hasher.finalize().to_vec()
    }

    fn concat_hash(&self, data: &T) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.as_ref());
        hasher.update(data.as_ref());
        hasher.finalize().to_vec()
    }
}
