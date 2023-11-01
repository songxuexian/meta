// receive and handle tcp connection
// Wrap DbDropGuard
// Shutdown gracefully

use std::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc},
};

use tokio::sync::{broadcast, Semaphore};

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

#[derive(Debug)]
enum ConnectionError {}
impl Listener {
    pub async fn run(&mut self) -> Result<(), ConnectionError> {
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
        }
    }

    async fn accept(&self) -> Result<TcpStream, ConnectionError> {
        Ok()
    }
}
