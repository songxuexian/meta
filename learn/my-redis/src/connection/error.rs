use std::{num::TryFromIntError, string::FromUtf8Error};

#[derive(Debug)]
pub enum ParseError {
    ParseArrayFrame,
    Incomplete,
    Unimplemented,
    Parse(String),
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
