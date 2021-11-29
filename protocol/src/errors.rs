use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("marshal raw block: {0}")]
    MarshalRawBlock(u64),
    #[error(transparent)]
    ClientError(#[from] ClientError),
    #[error("Command not recognized: {0}")]
    CommandNotRecognized(String),
    #[error("Account {1} has insufficient funds for fee ({0} SOL)")]
    InsufficientFundsForFee(f64, Pubkey),
}