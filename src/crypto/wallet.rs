use bip39::Mnemonic;

pub struct Wallet {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            public_key: vec![],
            private_key: vec![],
        }
    }

    pub fn generate_mnemonic_phrase() -> Mnemonic {
        Mnemonic::generate(24).unwrap()
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