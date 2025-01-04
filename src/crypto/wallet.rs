use bip39::Mnemonic;
use ed25519_dalek::{SigningKey, VerifyingKey};
use hmac::{Hmac, Mac};
use sha2::Sha512;

use crate::common::error::Error;

const HARDENED_OFFSET: u32 = 0x80000000;
const FINGERPRINT_LENGTH: usize = 4;
const CHAIN_CODE_LENGTH: usize = 32;
const SEED_KEY: &[u8] = b"Bitcoin seed";

type HmacSha512 = Hmac<Sha512>;

#[derive(Clone)]
pub struct Keypair {
    pub secret: SigningKey,
    pub public: VerifyingKey,
}

#[derive(Clone)]
pub struct ExtendedKeypair {
    pub keypair: Keypair,
    pub chain_code: [u8; 32],
    pub depth: u8,
    pub parent_fingerprint: [u8; 4],
    pub child_number: u32,
}

pub struct Wallet {
    master_keypair: ExtendedKeypair,
    derived_keys: Vec<ExtendedKeypair>,
}

impl Wallet {
    pub fn generate_mnemonic_phrase() -> Mnemonic {
        Mnemonic::generate(24).unwrap()
    }

    pub fn generate_wallet_from_mnemonic(mnemonic: &str) -> Result<Self, Error> {
        let seed = Mnemonic::parse(mnemonic)?.to_seed("");
        
        // Create HMAC with "Bitcoin seed" key
        let mut mac = HmacSha512::new_from_slice(SEED_KEY)
            .map_err(|_| Error::InvalidKey)?;
        mac.update(&seed);
        let result = mac.finalize().into_bytes();

        // Split into key and chain code
        let secret = SigningKey::try_from(&result[..32])?;
        let public = VerifyingKey::from(&secret);
        let chain_code = result[32..].try_into().unwrap();

        let master_keypair = ExtendedKeypair {
            keypair: Keypair { secret, public },
            chain_code,
            depth: 0,
            parent_fingerprint: [0u8; 4],
            child_number: 0,
        };
        
        Ok(Self { 
            master_keypair,
            derived_keys: Vec::new(),
        })
    }

    pub fn derive_child_key(&mut self, index: u32) -> Result<&ExtendedKeypair, Error> {
        let parent = &self.master_keypair;
        let hardened = index >= 0x80000000;
        
        // Create HMAC with parent chain code
        let mut mac = HmacSha512::new_from_slice(&parent.chain_code)
            .map_err(|_| Error::InvalidKey)?;

        if hardened {
            // Hardened child: 0x00 || parent private key || index
            mac.update(&[0x00]);
            mac.update(parent.keypair.secret.as_bytes());
        } else {
            // Normal child: parent public key || index
            mac.update(parent.keypair.public.as_bytes());
        }
        mac.update(&index.to_be_bytes());

        let result = mac.finalize().into_bytes();

        // Split into key and chain code
        let mut child_key = SigningKey::try_from(&result[..32])?;
        
        // Add parent private key (mod n)
        // Note: This is a simplified version. In practice, you need to handle
        // field arithmetic properly and check for invalid keys
        let mut key_bytes = child_key.to_bytes();
        for (i, (child, parent)) in key_bytes.iter_mut()
            .zip(parent.keypair.secret.as_bytes())
            .enumerate() {
            *child = child.wrapping_add(*parent);
        }
        
        child_key = SigningKey::try_from(&key_bytes)
            .map_err(|_| Error::InvalidKey)?;
        let child_public = VerifyingKey::from(&child_key);
        let child_chain_code = result[32..].try_into().unwrap();

        // Calculate parent fingerprint (should use RIPEMD160 in real implementation)
        let parent_fingerprint = parent.keypair.public.as_bytes()[0..4].try_into().unwrap();

        let extended_keypair = ExtendedKeypair {
            keypair: Keypair { 
                secret: child_key,
                public: child_public,
            },
            chain_code: child_chain_code,
            depth: parent.depth + 1,
            parent_fingerprint,
            child_number: index,
        };
        
        self.derived_keys.push(extended_keypair);
        Ok(self.derived_keys.last().unwrap())
    }

    pub fn get_master_public_key(&self) -> &VerifyingKey {
        &self.master_keypair.keypair.public
    }

    pub fn get_master_secret_key(&self) -> &SigningKey {
        &self.master_keypair.keypair.secret
    }

    pub fn get_derived_keys(&self) -> &[ExtendedKeypair] {
        &self.derived_keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let mnemonic = "jazz exact bamboo hello situate degree fire taste math stock idea stock glimpse click elevator protect myself similar skate unfold ready cream cake march";
        let mut wallet = Wallet::generate_wallet_from_mnemonic(mnemonic).unwrap();

        println!("Master secret key: {:?}", hex::encode(wallet.get_master_secret_key().to_bytes()));
        println!("Master public key: {:?}", hex::encode(wallet.get_master_public_key().to_bytes()));
        
        let child0 = wallet.derive_child_key(0).unwrap();
        let child0_pubkey = child0.keypair.public.to_bytes();

        println!("Child 0 pubkey: {:?}", hex::encode(child0_pubkey));
        
        let child1 = wallet.derive_child_key(1).unwrap();
        let child1_pubkey = child1.keypair.public.to_bytes();
        
        println!("Child 1 pubkey: {:?}", hex::encode(child1_pubkey));
        
        assert_ne!(child0_pubkey, child1_pubkey);
    }
}