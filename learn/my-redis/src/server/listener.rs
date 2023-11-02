// receive and handle tcp connection
// Wrap DbDropGuard
// Shutdown gracefully

use std::{sync::Arc, time::Duration};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, mpsc, Semaphore},
    time,
};
use tracing::error;

use crate::{connection::connect::Connection, storage::db::DbDropGuard};

use super::{handler::Handler, shutdown::Shutdown};

#[derive(Debug)]
pub struct Listener {
    pub db_holder: DbDropGuard,
    pub listener: TcpListener,
    pub limit_connections: Arc<Semaphore>,
    pub notify_shutdown: broadcast::Sender<()>,
    pub shutdown_complete_rx: mpsc::Receiver<()>,
    pub shutdown_complete_tx: mpsc::Sender<()>,
}

impl Listener {
    pub async fn run(&mut self) -> crate::Result<()> {
        loop {
            let permit = self
                .limit_connections
                .clone()
                .acquire_owned()
                .await
                .unwrap();

            let socket = self.accept().await?;

            let mut handler = Handler {
                db: self.db_holder.db(),
                connection: Connection::new(socket),
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
                _shutdown_complete: self.shutdown_complete_tx.clone(),
            };
            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    error!(message = "connection error",cause = ?err);
                }
                drop(permit)
            });
        }
    }

    async fn accept(&self) -> crate::Result<TcpStream> {
        let backoff = 1;

        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(e) => {
                    if backoff > 64 {
                        error!("failed to accept socket after retry: {}", e);
                        return Err(e.into());
                    } else {
                        error!("failed to accept socket: {}", e);
                    }
                }
            }

            time::sleep(Duration::from_secs(backoff)).await;

            let _ = backoff << 2;
        }
    }
}
