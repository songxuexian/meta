use crate::{
    connection::{
        connect::Connection,
        error::{ConnectionError, ParseError},
        frame::{self, Frame},
        parse::{self, Parse},
    },
    server::shutdown::{self, Shutdown},
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

impl Command {
    pub fn from_frame(frame: Frame) -> Result<Command, ParseError> {
        let mut parse = Parse::new(frame)?;

        let command_name = parse.next_string()?.to_lowercase();

        let command = match &command_name[..] {
            "get" => Command::Get(Get::parse_frames(&mut parse)?),
            "ping" => Command::Ping(Ping::parse_frames(&mut parse)?),
            _ => return Ok(Command::Unknown(Unknown::new(command_name))),
        };

        parse.finish()?;

        Ok(command)
    }

    pub async fn apply(
        self,
        db: &Db,
        dst: &mut Connection,
        shutdown: &mut Shutdown,
    ) -> Result<(), ConnectionError> {
        match self {
            Command::Get(cmd) => cmd.apply(db, dst).await,
            Command::Set(cmd) => todo!(),
            Command::Publish(cmd) => todo!(),
            Command::Subscribe(cmd) => todo!(),
            Command::Unsubscribe(_) => todo!(),
            Command::Ping(_cmd) => todo!(),
            Command::Unknown(cmd) => todo!(),
        }
    }
}
