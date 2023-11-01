use std::pin::Pin;

use bytes::Bytes;
use tokio::select;
use tokio_stream::{Stream, StreamExt, StreamMap};
use tracing::{debug, warn};

use super::{unknown::Unknown, Command, CommandToFrame};

use crate::{
    cmd::unsubscribe::make_unsubscribe_frame,
    connection::{
        connect::Connection,
        error::{ConnectionError, ParseError},
        frame::Frame,
        parse::Parse,
    },
    server::shutdown::Shutdown,
    storage::{db::Db, kvstore::KvStore},
};

#[derive(Debug)]
pub struct Subscribe {
    channels: Vec<String>,
}

type Message = Pin<Box<dyn Stream<Item = Bytes> + Send>>;

impl Subscribe {
    pub fn new(channels: &[String]) -> Subscribe {
        Self {
            channels: channels.to_vec(),
        }
    }

    pub async fn apply(
        mut self,
        db: &Db,
        dst: &mut Connection,
        shutdown: &mut Shutdown,
    ) -> Result<(), ConnectionError> {
        let mut subscriptions = StreamMap::new();

        loop {
            for channel_name in self.channels.drain(..) {
                Self::subscribe_to_channel(channel_name, &mut subscriptions, db, dst).await?;
            }

            select! {
                Some((channel_name,msg)) = subscriptions.next()=>{
                    dst.write_frame(&make_message_frame(channel_name, msg)?).await?;
                }
                res = dst.read_frame() => {
                    let frame = match res? {
                        Some(frame) =>frame,
                        None => {
                            warn!("remote subsribe client disconnected");
                            return Ok(());
                        }
                    };

                    handle_command(frame,&mut self.channels,&mut subscriptions,dst).await?;
                }
                _ = shutdown.recv() => {
                    warn!("server shutdown, stop subsribe");
                    return Ok(());
                }
            }
        }
    }

    async fn subscribe_to_channel(
        channel_name: String,
        subscriptions: &mut StreamMap<String, Message>,
        db: &Db,
        dst: &mut Connection,
    ) -> Result<(), ConnectionError> {
        let mut rx = db.subscribe(channel_name.clone());

        let rx = Box::pin(async_stream::stream! {
            loop{
                match rx.recv().await {
                    Ok(msg) => yield msg,
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(e)) =>{
                        warn!("subscribe received lagged: {}",e);
                    }
                    Err(e) => {
                        warn!("subscribe received error: {}",e);
                        break
                    }
                }
            }
        });

        subscriptions.insert(channel_name.clone(), rx);

        debug!("subscribed to channel success: {}", channel_name);
        let response = make_subscribe_frame(channel_name, subscriptions.len())?;
        dst.write_frame(&response).await?;
        Ok(())
    }
}

fn make_subscribe_frame(channel_name: String, num_subs: usize) -> Result<Frame, ParseError> {
    let mut response = Frame::array();
    response.push_bulk(Bytes::from_static(b"subscribe"))?;
    response.push_bulk(Bytes::from(channel_name))?;
    response.push_int(num_subs as u64)?;
    Ok(response)
}

fn make_message_frame(channel_name: String, msg: Bytes) -> Result<Frame, ParseError> {
    let mut response = Frame::array();
    response.push_bulk(Bytes::from_static(b"message"))?;
    response.push_bulk(Bytes::from(channel_name))?;
    response.push_bulk(msg)?;
    Ok(response)
}

async fn handle_command(
    frame: Frame,
    subscribe_to: &mut Vec<String>,
    subscriptions: &mut StreamMap<String, Message>,
    dst: &mut Connection,
) -> Result<(), ConnectionError> {
    match Command::from_frame(frame)? {
        Command::Subscribe(subscribe) => subscribe_to.extend(subscribe.channels.to_vec()),
        Command::Unsubscribe(mut unsubscribe) => {
            if unsubscribe.channels.is_empty() {
                unsubscribe.channels = subscriptions
                    .keys()
                    .map(|channel_name| channel_name.to_string())
                    .collect();
            }

            for channel_name in unsubscribe.channels {
                debug!("begin ubsubscribe: {}", channel_name);

                subscriptions.remove(&channel_name);

                let response = make_unsubscribe_frame(channel_name, subscriptions.len())?;
                dst.write_frame(&response).await?;
                debug!("unsubscribe success: {}", response);
            }
        }
        command => {
            let cmd = Unknown::new(command.get_name());
            cmd.apply(dst).await?;
        }
    }

    Ok(())
}

impl CommandToFrame for Subscribe {
    type Output = Self;

    fn parse_frames(parse: &mut Parse) -> Result<Self::Output, ParseError> {
        let mut channels = vec![parse.next_string()?];
        loop {
            match parse.next_string() {
                Ok(s) => channels.push(s),
                Err(ParseError::EndOfStream) => break,
                Err(e) => return Err(e),
            }
        }

        Ok(Self { channels })
    }

    fn into_frame(
        self,
    ) -> Result<crate::connection::frame::Frame, crate::connection::error::ParseError> {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("subscribe".as_bytes()))?;
        for channel in self.channels {
            frame.push_bulk(Bytes::from(channel.into_bytes()))?;
        }

        Ok(frame)
    }
}
