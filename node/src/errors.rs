use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("No found by hash: {0}")]
    NotFoundHash(String),
    #[error("Not found by height: {0}")]
    NotFoundHeight(u64),
}