use bip39::Mnemonic;
use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SigningKey, VerifyingKey};

use crate::common::error::Error;

#[derive(Clone)]
pub struct Keypair {
    secret: SigningKey,
    public: VerifyingKey,
}

pub struct Wallet {
    keypair: Keypair,
}

impl Wallet {
    pub fn generate_mnemonic_phrase() -> Mnemonic {
        Mnemonic::generate(24).unwrap()
    }

    pub fn generate_wallet_from_mnemonic(mnemonic: &str, password: Option<&str>) -> Result<Self, Error> {
        let seed = Mnemonic::parse(mnemonic)?.to_seed(password.unwrap_or(""));

        // The first 32 bytes of the seed are used to derive the secret key
        // The other 32 bytes are used to derive the chain code (not used in this implementation)
        let secret = SigningKey::try_from(&seed[..32])?;
        let public = VerifyingKey::from(&secret);

        Ok(Self { 
            keypair: Keypair { secret, public },
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
            keypair: Keypair { secret, public }, 
        })
    }

    pub fn get_master_public_key(&self) -> &VerifyingKey {
        &self.keypair.public
    }

    pub fn get_master_secret_key(&self) -> &SigningKey {
        &self.keypair.secret
    }

    pub fn sign(&mut self, message: &[u8]) -> Result<Signature, Error> {
        let signature = self.keypair.secret.try_sign(message)?;
        Ok(signature)
    }

    pub fn verify(&self, message: &[u8], signature: &[u8; 64]) -> Result<(), Error> {
        let signature = Signature::from_bytes(signature);
        self.keypair.public.verify_strict(message, &signature)?;
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
        let wallet = Wallet::generate_wallet_from_mnemonic(mnemonic, None).unwrap();

        println!("Master secret key: {:?}", hex::encode(wallet.get_master_secret_key().to_bytes()));
        println!("Master public key: {:?}", hex::encode(wallet.get_master_public_key().to_bytes()));
        
        assert_ne!(wallet.get_master_secret_key().to_bytes(), wallet.get_master_public_key().to_bytes());
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

        wallet.verify(message, &signature.to_bytes()).unwrap();
        wallet.verify(b"fake message", &signature.to_bytes()).unwrap_err();
    }
}
