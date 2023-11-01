use std::time::Duration;

use bytes::Bytes;
use tracing::{debug, warn};

use super::CommandToFrame;
use crate::{
    connection::{
        connect::Connection,
        error::{ConnectionError, ParseError},
        frame::Frame,
    },
    storage::{db::Db, kvstore::KvStore},
};

#[derive(Debug)]
pub struct Set {
    key: String,
    value: Bytes,
    expire: Option<Duration>,
}

impl Set {
    pub fn new(key: impl ToString, value: Bytes, expire: Option<Duration>) -> Self {
        Self {
            key: key.to_string(),
            value,
            expire,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn expire(&self) -> Option<Duration> {
        self.expire
    }

    pub fn value(&self) -> &Bytes {
        &self.value
    }

    pub async fn apply(self, db: &Db, dst: &mut Connection) -> Result<(), ConnectionError> {
        db.set(self.key, self.value, self.expire);
        let response = Frame::Simple("OK".to_string());
        debug!("applied set command response: {:?}", response);

        dst.write_frame(&response).await?;

        Ok(())
    }
}

impl CommandToFrame for Set {
    type Output = Set;

    fn parse_frames(
        parse: &mut crate::connection::parse::Parse,
    ) -> Result<Self::Output, crate::connection::error::ParseError> {
        let key = parse.next_string()?;
        let value = parse.next_bytes()?;
        let mut expire = None;

        match parse.next_string() {
            Ok(s) if s.to_uppercase() == "EX" => {
                let secs = parse.next_int()?;
                expire = Some(Duration::from_secs(secs));
            }
            Ok(s) if s.to_uppercase() == "PX" => {
                let ms = parse.next_int()?;
                expire = Some(Duration::from_millis(ms));
            }
            Ok(s) => {
                warn!("unsupported SET option: {}", s);
                return Err(ParseError::Parse(
                    "currently `SET` onlu supports the expiration option".into(),
                ));
            }
            Err(ParseError::EndOfStream) => {
                debug!("no extra SET option");
            }
            Err(e) => return Err(e),
        }

        Ok(Self { key, value, expire })
    }

    fn into_frame(
        self,
    ) -> Result<crate::connection::frame::Frame, crate::connection::error::ParseError> {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("set".as_bytes()))?;
        frame.push_bulk(Bytes::from(self.key.into_bytes()))?;
        frame.push_bulk(self.value)?;
        if let Some(ms) = self.expire {
            // Expirations in Redis procotol can be specified in two ways
            // 1. SET key value EX seconds
            // 2. SET key value PX milliseconds
            // We implement the second option because it allows greater precision and
            // src/bin/cli.rs parses the expiration argument as milliseconds
            // in duration_from_ms_str()
            frame.push_bulk(Bytes::from("px".as_bytes()))?;
            frame.push_int(ms.as_millis() as u64)?;
        }
        Ok(frame)
    }
}
