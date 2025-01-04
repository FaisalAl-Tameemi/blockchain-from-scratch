use blockchain_from_scratch::crypto::wallet::Wallet;

fn main() {
    let mnemonic = Wallet::generate_mnemonic_phrase();
    println!("Mnemonic: {}", mnemonic);
}
