use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid key")]
    InvalidKey,
    
    #[error("Bip39 error")]
    Bip39Error(#[from] bip39::Error),
    
    #[error("Ed25519 error")]
    Ed25519Error(#[from] ed25519_dalek::SignatureError),

    #[error("Invalid chain code")]
    InvalidChainCode,

    #[error("Hex error")]
    HexError(#[from] hex::FromHexError),
}
