use std::{fmt::Display, io::Cursor};

use bytes::{Buf, Bytes};
use tracing::dispatcher::get_default;

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

    pub fn push_int(&mut self, value: u64) -> Result<(), ParseError> {
        match self {
            Frame::Array(vec) => {
                vec.push(Frame::Integer(value));
                Ok(())
            }
            _ => Err(ParseError::ParseArrayFrame),
        }
    }

    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), ParseError> {
        match get_u8(src)? {
            b'+' => {
                get_line(src)?;
                Ok(())
            }
            b'-' => {
                get_line(src)?;
                Ok(())
            }
            b':' => {
                let _ = get_decimal(src)?;
                Ok(())
            }
            b'$' => {
                if b'-' == peek_u8(src)? {
                    skip(src, 4)
                } else {
                    let len: usize = get_decimal(src)?.try_into()?;
                    skip(src, len + 2)
                }
            }
            b'*' => {
                let len = get_decimal(src)?;
                for _ in 0..len {
                    Frame::check(src)?
                }
                Ok(())
            }
            _ => Err(ParseError::Unimplemented),
        }
    }
}

fn peek_u8(src: &mut Cursor<&[u8]>) -> Result<u8, ParseError> {
    if !src.has_remaining() {
        return Err(ParseError::Incomplete);
    }
    Ok(src.chunk()[0])
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, ParseError> {
    if !src.has_remaining() {
        return Err(ParseError::Incomplete);
    }
    Ok(src.get_u8())
}

fn skip(src: &mut Cursor<&[u8]>, n: usize) -> Result<(), ParseError> {
    if src.remaining() < n {
        return Err(ParseError::Incomplete);
    }
    src.advance(n);
    Ok(())
}

fn get_decimal(src: &mut Cursor<&[u8]>) -> Result<u64, ParseError> {
    use atoi::atoi;
    let line = get_line(src)?;
    atoi::<u64>(line).ok_or_else(|| "protocol error; invalid frame format".into())
}

fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], ParseError> {
    let start = src.position() as usize;
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            src.set_position((i + 2) as u64);

            return Ok(&src.get_ref()[start..i]);
        }
    }

    Err(ParseError::Incomplete)
}
