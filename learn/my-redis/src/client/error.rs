use thiserror::Error;

use crate::connection::error::{ConnectionError, ParseError};

#[derive(Error, Debug)]
pub enum ClientError {
    #[error(transparent)]
    Connect(#[from] ConnectionError),

    #[error(transparent)]
    Parse(#[from] ParseError),
}
