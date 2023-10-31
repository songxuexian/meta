use std::num::TryFromIntError;

#[derive(Debug)]
pub enum ParseError {
    ParseArrayFrame,
    Incomplete,
    Unimplemented,
    Other(crate::Error),
}
impl From<String> for ParseError {
    fn from(value: String) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<&str> for ParseError {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<TryFromIntError> for ParseError {
    fn from(_src: TryFromIntError) -> ParseError {
        "protocol error; invalid frame format".into()
    }
}
