use std::sync::mpsc;

use tracing::debug;

use crate::{cmd::Command, connection::connect::Connection, storage::db::Db};

use super::shutdown::Shutdown;

#[derive(Debug)]
pub struct Handler {
    pub db: Db,
    pub connection: Connection,
    pub shutdown: Shutdown,
    pub _shutdown_complete: mpsc::Sender<()>,
}

impl Handler {
    pub async fn run(&mut self) -> crate::Result<()> {
        while !self.shutdown.is_shutdown() {
            let maybe_frame = tokio::select! {
                res = self.connection.read_frame() => res?,
                _ = self.shutdown.recv() => {
                    return Ok(());
                }
            };

            let frame = match maybe_frame {
                Some(frame) => frame,
                None => {
                    debug!("peer closed the socket, return");
                    return Ok(());
                }
            };

            let cmd = Command::from_frame(frame)?;

            debug!(?cmd);

            cmd.apply(&self.db, &mut self.connection, &mut self.shutdown)
                .await?;
        }

        Ok(())
    }
}
