use std::{fmt::Display, num::TryFromIntError, string::FromUtf8Error};

#[derive(Debug)]
pub enum ParseError {
    ParseArrayFrame,
    Incomplete,
    Unimplemented,
    Parse(crate::Error),
    Other(crate::Error),
    EndOfStream,
}
impl From<String> for ParseError {
    fn from(src: String) -> Self {
        ParseError::Other(src.into())
    }
}

impl From<&str> for ParseError {
    fn from(src: &str) -> Self {
        src.to_string().into()
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(_src: FromUtf8Error) -> Self {
        "protocol errer; invalid frame format".into()
    }
}

impl From<TryFromIntError> for ParseError {
    fn from(_src: TryFromIntError) -> ParseError {
        "protocol error; invalid frame format".into()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ParseArrayFrame => "protocol error; unexpected parse array frame".fmt(f),
            ParseError::Incomplete => "protocol error; unexpected incomplete".fmt(f),
            ParseError::Unimplemented => "protocol error; unexpected unimplemented".fmt(f),
            ParseError::Parse(err) => err.fmt(f),
            ParseError::Other(err) => err.fmt(f),
            ParseError::EndOfStream => "protocol error; unexpected end of stream".fmt(f),
        }
    }
}

impl std::error::Error for ParseError {}
