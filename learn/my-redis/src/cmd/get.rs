use tracing::debug;

use crate::{
    connection::{
        connect::Connection,
        error::{ConnectionError, ParseError},
        frame::Frame,
        parse::Parse,
    },
    storage::{db::Db, kvstore::KvStore},
};

#[derive(Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn new(key: impl ToString) -> Get {
        Get {
            key: key.to_string(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn parse_frames(parse: &mut Parse) -> Result<Get, ParseError> {
        let key = parse.next_string()?;

        Ok(Get::new(key))
    }

    pub async fn apply(self, db: &Db, dst: &mut Connection) -> Result<(), ConnectionError> {
        let response = if let Some(value) = db.get(&self.key) {
            Frame::Bulk(value)
        } else {
            Frame::Null
        };

        debug!("get command applied resp: {:?}", response);

        dst.write_frame(&response).await?;
        Ok(())
    }
}
