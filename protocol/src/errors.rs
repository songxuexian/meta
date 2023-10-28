use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("marshal raw block: {0}")]
    MarshalRawBlock(u64),
    #[error("net params: {0}")]
    NetParams(String),
}
