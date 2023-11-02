use async_stream::try_stream;
use bytes::Bytes;
use tokio_stream::Stream;
use tracing::{debug, error};

use crate::cmd::unsubscribe::Unsubscribe;
use crate::cmd::CommandToFrame;
use crate::connection::error::ConnectionError;
use crate::connection::frame::Frame;

use super::cli::Client;

pub struct Subscriber {
    pub client: Client,
    pub subscribed_channels: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub channel: String,
    pub content: Bytes,
}

impl Subscriber {
    pub async fn subscribe(&mut self, channels: &[String]) -> Result<(), ConnectionError> {
        self.client.subscribe_cmd(channels).await?;

        self.subscribed_channels
            .extend(channels.iter().map(Clone::clone));

        Ok(())
    }

    pub fn get_subscribed_channels(&self) -> &[String] {
        &self.subscribed_channels
    }

    pub async fn next_message(&mut self) -> Result<Option<Message>, ConnectionError> {
        match self.client.connection.read_frame().await? {
            Some(frame) => {
                debug!(cause = ?frame,"subscribe received next message");
                match frame {
                    Frame::Array(ref frame) => match frame.as_slice() {
                        [message, channel, content] if *message == "message" => Ok(Some(Message {
                            channel: channel.to_string(),
                            content: Bytes::from(content.to_string()),
                        })),
                        _ => {
                            error!(cause = ?frame,"Invalid message");
                            Err(ConnectionError::InvalidFrameType)
                        }
                    },
                    frame => Err(ConnectionError::CommandExecute(frame.to_string())),
                }
            }
            None => Ok(None),
        }
    }

    pub fn into_stream(mut self) -> impl Stream<Item = Result<Message, ConnectionError>> {
        try_stream! {
            while let Some(message) = self.next_message().await? {
                yield message;
            }
        }
    }

    pub async fn unsubscribe(&mut self, channels: &[String]) -> Result<(), ConnectionError> {
        let frame = Unsubscribe::new(channels).into_frame()?;

        debug!(cause = ?frame, "unsubscribe command");

        self.client.connection.write_frame(&frame).await?;

        let num = if channels.is_empty() {
            self.subscribed_channels.len()
        } else {
            channels.len()
        };

        for _ in 0..num {
            let response = self.client.read_response().await?;

            match response {
                Frame::Array(ref frame) => match frame.as_slice() {
                    [unsubscribe, channel, ..] if *unsubscribe == "unsubscribe" => {
                        let len = self.subscribed_channels.len();

                        if len == 0 {
                            return Err(ConnectionError::InvalidArgument(response.to_string()));
                        }

                        self.subscribed_channels.retain(|c| *channel != &c[..]);

                        if self.subscribed_channels.len() != len - 1 {
                            return Err(ConnectionError::CommandExecute(response.to_string()));
                        }
                    }
                    _ => return Err(ConnectionError::InvalidFrameType),
                },
                frame => return Err(ConnectionError::CommandExecute(frame.to_string())),
            };
        }

        Ok(())
    }
}
