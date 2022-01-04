
use {
    thiserror::Error,
    url::ParseError,
};

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("url parse error")]
    UrlParseError(#[from] ParseError),

    #[error("unable to connect to server")]
    ConnectionError(#[from] tungstenite::Error),

    #[error("json parse error")]
    JsonParseError(#[from] serde_json::error::Error),

    #[error("unexpected message format: {0}")]
    UnexpectedMessageError(String),
}