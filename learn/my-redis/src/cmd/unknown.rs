use tracing::debug;

use crate::connection::{connect::Connection, error::ConnectionError, frame::Frame};

#[derive(Debug)]
pub struct Unknown {
    command_name: String,
}

impl Unknown {
    pub(crate) fn new(key: impl ToString) -> Unknown {
        Self {
            command_name: key.to_string(),
        }
    }
    pub async fn apply(self, dst: &mut Connection) -> Result<(), ConnectionError> {
        let response = Frame::Error(format!("err unknown command '{}'", self.command_name));
        debug!("apply unknown command resp: {:?}", response);
        dst.write_frame(&response).await?;

        Ok(())
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.command_name
    }
}
