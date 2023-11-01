use crate::{
    connection::{
        connect::Connection,
        error::{ConnectionError, ParseError},
        frame::Frame,
        parse::Parse,
    },
    storage::db::Db,
};

use self::{
    get::Get, ping::Ping, publish::Publish, set::Set, subscribe::Subscribe, unknown::Unknown,
    unsubscribe::Unsubscribe,
};

mod get;
mod ping;
mod publish;
mod set;
mod subscribe;
mod unknown;
mod unsubscribe;

#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
    Publish(Publish),
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
    Ping(Ping),
    Unknown(Unknown),
}

pub trait CommandToFrame {
    type Output;
    fn parse_frames(parse: &mut Parse) -> Result<Self::Output, ParseError>;
    fn into_frame(self) -> Result<Frame, ParseError>;
}
