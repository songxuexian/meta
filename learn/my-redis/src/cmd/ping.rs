use bytes::Bytes;

use crate::connection::{connect::Connection, error::ParseError, frame::Frame, parse::Parse};

#[derive(Debug, Default)]
pub struct Ping {
    msg: Option<String>,
}

impl Ping {
    pub fn new(msg: Option<String>) -> Self {
        Ping { msg }
    }

    pub fn parse_frames(parse: &mut Parse) -> Result<Self, ParseError> {
        match parse.next_string() {
            Ok(msg) => Ok(Ping::new(Some(msg))),
            Err(ParseError::EndOfStream) => Ok(Ping::default()),
            Err(e) => Err(e),
        }
    }

    pub async fn apply(self, dst: &mut Connection) -> crate::Result<()> {
        let response = match self.msg {
            None => Frame::Simple("PONG".to_string()),
            Some(msg) => Frame::Bulk(Bytes::from(msg)),
        };

        dst.write_frame(&response).await?;
        Ok(())
    }

    pub fn into_frame(self) -> Result<Frame, ParseError> {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("ping".as_bytes()))?;
        if let Some(msg) = self.msg {
            frame.push_bulk(Bytes::from(msg))?;
        }
        Ok(frame)
    }
}
