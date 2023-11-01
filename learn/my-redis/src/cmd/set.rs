use std::time::Duration;

use bytes::Bytes;

use super::CommandToFrame;
use crate::{
    connection::{connect::Connection, error::ConnectionError},
    storage::db::Db,
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
    pub async fn apply(self, db: &Db, dst: &mut Connection) -> Result<(), ConnectionError> {
        Ok(())
    }
}

impl CommandToFrame for Set {
    type Output = Set;

    fn parse_frames(
        parse: &mut crate::connection::parse::Parse,
    ) -> Result<Self::Output, crate::connection::error::ParseError> {
        todo!()
    }

    fn into_frame(
        self,
    ) -> Result<crate::connection::frame::Frame, crate::connection::error::ParseError> {
        todo!()
    }
}
