use bip39::{Mnemonic};

pub struct Wallet {}

impl Wallet {
    pub fn generate_mnemonic_phrase() -> Mnemonic {
        Mnemonic::generate(24).unwrap()
    }

    pub fn derive_wallet_from_mnemonic(mnemonic: &str, path: &str) -> Self {
        unimplemented!()
    }

    pub fn derive_wallet_from_seed(seed: &str, path: &str) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mnemonic_phrase() {
        let mnemonic = Wallet::generate_mnemonic_phrase();
        println!("Mnemonic: {}", mnemonic);
    }
}