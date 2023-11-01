use std::{future::Future, sync::Arc};

use tokio::{
    net::TcpListener,
    select,
    sync::{broadcast, mpsc, Semaphore},
};
use tracing::{debug, error, info};

use crate::{server::listener::Listener, storage::db::DbDropGuard, MAX_CONNECTIONS};

pub mod handler;
pub mod listener;
pub mod shutdown;

pub async fn run(listener: TcpListener, shutdown: impl Future) {
    info!(
        "my-redis server started listen on :{}",
        listener.local_addr().unwrap()
    );

    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

    let mut server = Listener {
        db_holder: DbDropGuard::new(),
        listener,
        limit_connections: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
        notify_shutdown,
        shutdown_complete_rx,
        shutdown_complete_tx,
    };

    select! {
        res = server.run() => {
            if let Err(err) = res{
                error!("failed to accept: {:?}",err);
            }
        }
        _=shutdown => {
            debug!("server is about to shutdown");
        }
    }

    let Listener {
        notify_shutdown,
        mut shutdown_complete_rx,
        shutdown_complete_tx,
        ..
    } = server;

    drop(notify_shutdown);
    drop(shutdown_complete_tx);

    let _ = shutdown_complete_rx.recv().await;
}
