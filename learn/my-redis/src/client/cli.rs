use bytes::Bytes;

use crate::{
    cmd::{ping::Ping, CommandToFrame},
    connection::{connect::Connection, error::ConnectionError, frame::Frame},
};

pub struct Client {
    pub connection: Connection,
}

impl Client {
    pub async fn ping(&mut self, msg: Option<String>) -> Result<Bytes, ConnectionError> {
        let frame = Ping::new(msg).into_frame()?;
        self.connection.write_frame(&frame).await?;

        match self.read_response().await? {
            Frame::Simple(value) => Ok(value.into()),
            Frame::Bulk(value) => Ok(value),
            frame => Err(ConnectionError::CommandExecute(frame.to_string())),
        }
    }

    pub async fn read_response(&mut self) -> Result<Frame, ConnectionError> {
        let response = self.connection.read_frame().await?;
        match response {
            Some(Frame::Error(msg)) => Err(ConnectionError::CommandExecute(msg)),
            Some(frame) => Ok(frame),
            None => Err(ConnectionError::Disconnect),
        }
    }
}
