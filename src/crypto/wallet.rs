use bip39::Mnemonic;
use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SigningKey, VerifyingKey};
use hmac::{Hmac, Mac};
use sha2::Sha512;

use crate::common::error::Error;

const SEED_KEY: &[u8] = b"Bitcoin seed";

type HmacSha512 = Hmac<Sha512>;

#[derive(Clone)]
pub struct Keypair {
    secret: SigningKey,
    public: VerifyingKey,
}

#[derive(Clone)]
pub struct ExtendedKeypair {
    keypair: Keypair,
    chain_code: [u8; 32],
    depth: u8,
    parent_fingerprint: [u8; 4],
    child_number: u32,
}

pub struct Wallet {
    keypair: ExtendedKeypair,
}

impl Wallet {
    pub fn generate_mnemonic_phrase() -> Mnemonic {
        Mnemonic::generate(24).unwrap()
    }

    pub fn generate_wallet_from_mnemonic(mnemonic: &str, password: Option<&str>) -> Result<Self, Error> {
        let seed = Mnemonic::parse(mnemonic)?.to_seed(password.unwrap_or(""));
        
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
            keypair: master_keypair,
        })
    }

    pub fn generate_wallet_from_private_key(private_key: &str) -> Result<Self, Error> {
        // read 32 bytes from the private key
        let mut secret_bytes = [0u8; 32];
        hex::decode_to_slice(private_key, &mut secret_bytes)?;
        // convert the bytes to a keypair
        let secret = SigningKey::from_bytes(&secret_bytes);
        let public = VerifyingKey::from(&secret);
        
        Ok(Self { 
            keypair: ExtendedKeypair {
                keypair: Keypair { secret, public }, 
                chain_code: [0u8; 32], 
                depth: 0, 
                parent_fingerprint: [0u8; 4], 
                child_number: 0 
            },
        })
    }

    pub fn derive_child_key(&mut self, index: u32) -> Result<Self, Error> {
        let parent = &self.keypair;
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
        
        Ok(Self {
            keypair: extended_keypair,
        })
    }

    pub fn get_master_public_key(&self) -> &VerifyingKey {
        &self.keypair.keypair.public
    }

    pub fn get_master_secret_key(&self) -> &SigningKey {
        &self.keypair.keypair.secret
    }

    pub fn sign(&mut self, message: &[u8]) -> Result<Signature, Error> {
        let signature = self.keypair.keypair.secret.sign(message);
        Ok(signature)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), Error> {
        self.keypair.keypair.public.verify_strict(message, signature)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mnemonic_phrase() {
        let mnemonic = Wallet::generate_mnemonic_phrase();
        println!("Mnemonic: {:?}", mnemonic.to_string());
        assert!(mnemonic.word_count() == 24);
    }

    #[test]
    fn test_key_derivation() {
        let mnemonic = "jazz exact bamboo hello situate degree fire taste math stock idea stock glimpse click elevator protect myself similar skate unfold ready cream cake march";
        let mut wallet = Wallet::generate_wallet_from_mnemonic(mnemonic, None).unwrap();

        println!("Master secret key: {:?}", hex::encode(wallet.get_master_secret_key().to_bytes()));
        println!("Master public key: {:?}", hex::encode(wallet.get_master_public_key().to_bytes()));
        
        let child0 = wallet.derive_child_key(50).unwrap();
        let child0_pubkey = child0.keypair.keypair.public.to_bytes();

        println!("Child 0 pubkey: {:?}", hex::encode(child0_pubkey));
        
        let child1 = wallet.derive_child_key(51).unwrap();
        let child1_pubkey = child1.keypair.keypair.public.to_bytes();
        
        println!("Child 1 pubkey: {:?}", hex::encode(child1_pubkey));
        
        assert_ne!(child0_pubkey, child1_pubkey);
    }

    #[test]
    fn test_key_derivation_from_private_key() {
        let mnemonic = Wallet::generate_mnemonic_phrase();
        let wallet = Wallet::generate_wallet_from_mnemonic(&mnemonic.to_string(), None).unwrap();
        let private_key = hex::encode(wallet.get_master_secret_key().to_bytes());
        let wallet_from_priv_key = Wallet::generate_wallet_from_private_key(&private_key).unwrap();

        assert_eq!(wallet.get_master_secret_key().to_bytes(), wallet_from_priv_key.get_master_secret_key().to_bytes());
        assert_eq!(wallet.get_master_public_key().to_bytes(), wallet_from_priv_key.get_master_public_key().to_bytes());

        // Note: The derived keys will be the same but the child keys will be different because the chain code is different
    }

    #[test]
    fn test_sign_and_verify() {
        let message = b"Hello, world!";
        let mnemonic = Wallet::generate_mnemonic_phrase();
        let mut wallet = Wallet::generate_wallet_from_mnemonic(&mnemonic.to_string(), None).unwrap();
        let signature = wallet.sign(message).unwrap();

        println!("Signature: {:?}", hex::encode(signature.to_bytes()));

        wallet.verify(message, &signature).unwrap();
        wallet.verify(b"fake message", &signature).unwrap_err();
    }
}
