use super::CommandToFrame;

use crate::{
    connection::{connect::Connection, error::ConnectionError},
    server::shutdown::{self, Shutdown},
    storage::db::Db,
};

#[derive(Debug)]
pub struct Subscribe {}
impl Subscribe {
    pub async fn apply(
        self,
        db: &Db,
        dst: &mut Connection,
        shutdown: &mut Shutdown,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
}

impl CommandToFrame for Subscribe {
    type Output = Self;

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
