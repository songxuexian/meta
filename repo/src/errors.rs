use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("No affected: {0}")]
    RowsAffected(String),
    #[error(transparent)]
    ClientError(#[from] ClientError),
    #[error("Command not recognized: {0}")]
    CommandNotRecognized(String),
    #[error("Account {1} has insufficient funds for fee ({0} SOL)")]
    InsufficientFundsForFee(f64, Pubkey),
    #[error("Account {1} has insufficient funds for spend ({0} SOL)")]
    InsufficientFundsForSpend(f64, Pubkey),
    #[error("Account {2} has insufficient funds for spend ({0} SOL) + fee ({1} SOL)")]
    InsufficientFundsForSpendAndFee(f64, f64, Pubkey),
    #[error(transparent)]
    InvalidNonce(nonce_utils::Error),
    #[error("Dynamic program error: {0}")]
    DynamicProgramError(String),
    #[error("RPC request error: {0}")]
    RpcRequestError(String),
    #[error("Keypair file not found: {0}")]
    KeypairFileNotFound(String),
}