use std::time::Duration;

use bytes::Bytes;
use tracing::{debug, error};

use crate::{
    cmd::{get::Get, ping::Ping, publish::Publish, set::Set, subscribe::Subscribe, CommandToFrame},
    connection::{connect::Connection, error::ConnectionError, frame::Frame},
};

use super::subscriber::Subscriber;

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

    pub async fn get(&mut self, key: &str) -> Result<Option<Bytes>, ConnectionError> {
        let frame = Get::new(key).into_frame()?;
        self.connection.write_frame(&frame).await?;

        match self.read_response().await? {
            Frame::Simple(value) => Ok(Some(value.into())),
            Frame::Bulk(value) => Ok(Some(value)),
            Frame::Null => Ok(None),
            frame => Err(ConnectionError::CommandExecute(frame.to_string())),
        }
    }

    pub async fn set(&mut self, key: &str, value: Bytes) -> Result<(), ConnectionError> {
        self.set_cmd(Set::new(key, value, None)).await
    }

    pub async fn set_expires(
        &mut self,
        key: &str,
        value: Bytes,
        expiration: Duration,
    ) -> Result<(), ConnectionError> {
        self.set_cmd(Set::new(key, value, Some(expiration))).await
    }

    pub async fn set_cmd(&mut self, cmd: Set) -> Result<(), ConnectionError> {
        let frame = cmd.into_frame()?;
        self.connection.write_frame(&frame).await?;

        match self.read_response().await? {
            Frame::Simple(value) if value == "OK" => Ok(()),
            frame => Err(ConnectionError::CommandExecute(frame.to_string())),
        }
    }

    pub async fn publish(&mut self, channel: &str, message: Bytes) -> Result<u64, ConnectionError> {
        let frame = Publish::new(channel, message).into_frame()?;
        self.connection.write_frame(&frame).await?;

        match self.read_response().await? {
            Frame::Integer(value) => Ok(value),
            frame => Err(ConnectionError::CommandExecute(frame.to_string())),
        }
    }

    pub async fn subscribe(mut self, channels: Vec<String>) -> Result<Subscriber, ConnectionError> {
        self.subscribe_cmd(&channels).await?;

        Ok(Subscriber {
            client: self,
            subscribed_channels: channels,
        })
    }

    pub async fn subscribe_cmd(&mut self, channels: &[String]) -> Result<(), ConnectionError> {
        let frame = Subscribe::new(channels).into_frame()?;
        self.connection.write_frame(&frame).await?;

        for channel in channels {
            let response = self.read_response().await?;
            match response {
                Frame::Array(ref frame) => match frame.as_slice() {
                    [subscribe, schannel, ..]
                        if *subscribe == "subscribe" && *schannel == channel.as_str() =>
                    {
                        debug!("subscribe channel: {} success", channel);
                    }
                    _ => {
                        error!("subscribe frame failed, response: {}", response);
                        return Err(ConnectionError::CommandExecute(response.to_string()));
                    }
                },
                frame => {
                    error!(
                        "subscribe frame failed, response frame type not match: {}",
                        frame
                    );
                    return Err(ConnectionError::InvalidFrameType);
                }
            }
        }

        Ok(())
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
