use crate::{
    connection::{connect::Connection, error::ConnectionError},
    storage::db::Db,
};

use super::CommandToFrame;

#[derive(Debug)]
pub struct Publish {}

impl Publish {
    pub async fn apply(self, db: &Db, dst: &mut Connection) -> Result<(), ConnectionError> {
        Ok(())
    }
}

impl CommandToFrame for Publish {
    type Output = Publish;

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
