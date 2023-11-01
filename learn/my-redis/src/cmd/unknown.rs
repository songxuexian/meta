use super::CommandToFrame;

use crate::{
    connection::{connect::Connection, error::ConnectionError},
    storage::db::Db,
};

#[derive(Debug)]
pub struct Unknown {
    command_name: String,
}

impl Unknown {
    pub fn new(key: impl ToString) -> Unknown {
        Self {
            command_name: key.to_string(),
        }
    }
    pub async fn apply(self, db: &Db, dst: &mut Connection) -> Result<(), ConnectionError> {
        Ok(())
    }
}


impl CommandToFrame for Unknown {
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
