use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("No affected: {0}")]
    RowsAffected(String),
    #[error("Command not recognized: {0}")]
    CommandNotRecognized(String),
    #[error("Dynamic program error: {0}")]
    DynamicProgramError(String),
    #[error("RPC request error: {0}")]
    RpcRequestError(String),
    #[error("Keypair file not found: {0}")]
    KeypairFileNotFound(String),
}