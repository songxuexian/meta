use std::vec;

use bytes::Bytes;
use mini_redis::Frame;

use super::error::ParseError;

#[derive(Debug)]
pub struct Parse {
    parts: vec::IntoIter<Frame>,
}

impl Parse {
    pub fn new(frame: Frame) -> Result<Parse, ParseError> {
        let array = match frame {
            Frame::Array(array) => array,
            frame => {
                return Err(ParseError::Parse(
                    format!("protocol error; expected array, got {:?}", frame).into(),
                ));
            }
        };
        Ok(Parse {
            parts: array.into_iter(),
        })
    }

    fn next(&mut self) -> Result<Frame, ParseError> {
        self.parts.next().ok_or(ParseError::EndOfStream)
    }

    pub fn next_string(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s),
            Frame::Bulk(data) => std::str::from_utf8(&data[..])
                .map(|s| s.to_string())
                .map_err(|_| ParseError::Parse("protocol error; invalid string".into())),
            frame => Err(ParseError::Parse(
                format!(
                    "protocol error; expectd simple frame or bulk frame, got {:?}",
                    frame
                )
                .into(),
            )),
        }
    }

    pub fn next_bytes(&mut self) -> Result<Bytes, ParseError> {
        match self.next()? {
            Frame::Simple(s) => Ok(Bytes::from(s.into_bytes())),
            Frame::Bulk(data) => Ok(data),
            frame => Err(ParseError::Parse(
                format!(
                    "protocol error; expectd simple frame or bulk frame, got {:?}",
                    frame
                )
                .into(),
            )),
        }
    }

    pub fn next_int(&mut self) -> Result<u64, ParseError> {
        use atoi::atoi;

        match self.next()? {
            Frame::Integer(v) => Ok(v),
            Frame::Simple(s) => atoi::<u64>(s.as_bytes())
                .ok_or_else(|| ParseError::Parse("protocol error; invalid integer".into())),
            Frame::Bulk(data) => atoi::<u64>(&data)
                .ok_or_else(|| ParseError::Parse("protocol error; invalid integer".into())),
            frame => Err(ParseError::Parse(
                format!(
                    "protocol error; expectd simple frame or bulk frame, got {:?}",
                    frame
                )
                .into(),
            )),
        }
    }

    pub fn finish(&mut self) -> Result<(), ParseError> {
        if self.parts.next().is_none() {
            Ok(())
        } else {
            Err(ParseError::Parse(
                "protocol error; expected end of frame, but there was more".into(),
            ))
        }
    }
}
