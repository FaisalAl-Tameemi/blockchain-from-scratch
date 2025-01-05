use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid key")]
    InvalidKey,
    
    #[error(transparent)]
    Bip39Error(#[from] bip39::Error),
    
    #[error(transparent)]
    Ed25519Error(#[from] ed25519_dalek::SignatureError),

    #[error("Invalid chain code")]
    InvalidChainCode,

    #[error("Hex error")]
    HexError(#[from] hex::FromHexError),
}
