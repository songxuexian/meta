use std::fmt::Display;

use bytes::Bytes;

use super::parse::ParseError;

#[derive(Debug, Clone)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}

impl PartialEq<&str> for Frame {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Frame::Simple(s) => s.eq(other),
            Frame::Bulk(s) => s.eq(other),
            _ => false,
        }
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::str;

        match self {
            Frame::Simple(response) => response.fmt(f),
            Frame::Error(msg) => write!(f, "error: {}", msg),
            Frame::Integer(num) => num.fmt(f),
            Frame::Bulk(msg) => match str::from_utf8(msg) {
                Ok(str) => str.fmt(f),
                Err(_) => write!(f, "{:?}", msg),
            },
            Frame::Null => "(nil)".fmt(f),
            Frame::Array(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                        part.fmt(f)?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl Frame {
    pub fn array() -> Frame {
        Frame::Array(vec![])
    }

    pub fn push_bulk(&mut self, bytes: Bytes) -> Result<(), ParseError> {
        match self {
            Frame::Array(vec) => {
                vec.push(Frame::Bulk(bytes));
                Ok(())
            }
            _ => Err(ParseError::ParseArrayFrame),
        }
    }
}
