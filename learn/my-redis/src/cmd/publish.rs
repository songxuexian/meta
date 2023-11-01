use bytes::Bytes;
use tracing::debug;

use crate::{
    connection::{connect::Connection, error::ConnectionError, frame::Frame},
    storage::{db::Db, kvstore::KvStore},
};

use super::CommandToFrame;

#[derive(Debug)]
pub struct Publish {
    channel: String,
    message: Bytes,
}

impl Publish {
    pub fn new(channel: impl ToString, message: Bytes) -> Publish {
        Publish {
            channel: channel.to_string(),
            message,
        }
    }
    pub async fn apply(self, db: &Db, dst: &mut Connection) -> Result<(), ConnectionError> {
        let num_subsribers = db.publish(&self.channel, self.message);

        let response = Frame::Integer(num_subsribers as u64);
        debug!("apply command applied response: {}", response);

        dst.write_frame(&response).await?;
        Ok(())
    }
}

impl CommandToFrame for Publish {
    type Output = Publish;

    fn parse_frames(
        parse: &mut crate::connection::parse::Parse,
    ) -> Result<Self::Output, crate::connection::error::ParseError> {
        let channel = parse.next_string()?;
        let message = parse.next_bytes()?;
        Ok(Publish { channel, message })
    }

    fn into_frame(
        self,
    ) -> Result<crate::connection::frame::Frame, crate::connection::error::ParseError> {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("publish".as_bytes()))?;
        frame.push_bulk(Bytes::from(self.channel.into_bytes()))?;
        frame.push_bulk(Bytes::from(self.message))?;

        Ok(frame)
    }
}
