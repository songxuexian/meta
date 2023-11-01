use super::CommandToFrame;

use crate::{
    connection::{connect::Connection, error::ConnectionError},
    storage::db::Db,
};

#[derive(Debug)]
pub struct Unsubscribe {}
impl Unsubscribe {
    pub async fn apply(self, db: &Db, dst: &mut Connection) -> Result<(), ConnectionError> {
        Ok(())
    }
}
impl CommandToFrame for Unsubscribe {
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
