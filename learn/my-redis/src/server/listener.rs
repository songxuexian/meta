// receive and handle tcp connection
// Wrap DbDropGuard
// Shutdown gracefully

use std::{
    net::TcpListener,
    sync::{mpsc, Arc},
};

use tokio::sync::{broadcast, Semaphore};

use crate::storage::db::DbDropGuard;

#[derive(Debug)]
pub struct Listener {
    pub db_holder: DbDropGuard,
    pub listener: TcpListener,
    pub limit_connections: Arc<Semaphore>,
    pub notify_shutdown: broadcast::Sender<()>,
    pub shutdown_complete_rx: mpsc::Receiver<()>,
    pub shutdown_complete_tx: mpsc::Sender<()>,
}
