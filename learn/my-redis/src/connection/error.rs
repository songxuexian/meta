use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("not an array frame")]
    ParseArrayFrame,
    #[error("not enough data is available to parse a message")]
    Incomplete,
    #[error("unimplemented command")]
    Unimplemented,
    #[error("protocol error; unexpected end of stream")]
    EndOfStream,
    #[error("invalid message encode, parse failed")]
    Parse(String),
    #[error(transparent)]
    ParseInt(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    ParseUtf8(#[from] std::string::FromUtf8Error),
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("connection reset by peer")]
    Disconnect,

    #[error(transparent)]
    ParseFrame(#[from] ParseError),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error("command execute error")]
    CommandExecute(String),

    #[error("received next message failed, invalid frame type")]
    InvalidFrameType,

    #[error("invalid argument")]
    InvalidArgument(String),
}
