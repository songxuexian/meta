use bytes::Bytes;

use super::CommandToFrame;

use crate::connection::{error::ParseError, frame::Frame};

#[derive(Debug)]
pub struct Unsubscribe {
    pub channels: Vec<String>,
}

impl Unsubscribe {
    pub fn new(channels: &[String]) -> Unsubscribe {
        Unsubscribe {
            channels: channels.to_vec(),
        }
    }
}

impl CommandToFrame for Unsubscribe {
    type Output = Self;

    fn parse_frames(
        parse: &mut crate::connection::parse::Parse,
    ) -> Result<Self::Output, crate::connection::error::ParseError> {
        let mut channels = vec![];

        loop {
            match parse.next_string() {
                Ok(s) => channels.push(s),
                Err(ParseError::EndOfStream) => break,
                Err(e) => return Err(e),
            }
        }

        Ok(Unsubscribe { channels })
    }

    fn into_frame(
        self,
    ) -> Result<crate::connection::frame::Frame, crate::connection::error::ParseError> {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from("unsubsribe".as_bytes()))?;
        for channel in self.channels {
            frame.push_bulk(Bytes::from(channel.into_bytes()))?;
        }

        Ok(frame)
    }
}

pub fn make_unsubscribe_frame(channel_name: String, num_subs: usize) -> Result<Frame, ParseError> {
    let mut response = Frame::array();
    response.push_bulk(Bytes::from_static(b"unsubsribe"))?;
    response.push_bulk(Bytes::from(channel_name))?;
    response.push_int(num_subs as u64)?;
    Ok(response)
}
