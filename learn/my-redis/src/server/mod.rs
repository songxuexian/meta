use std::{future::Future, net::TcpListener};

use tokio::sync::{broadcast, mpsc};
use tracing::info;

pub mod handler;
pub mod listener;
pub mod shutdown;

// pub async fn run(listener: TcpListener, shutdown: impl Future) {
//     info!(
//         "my-redis server started listen on :{}",
//         listener.local_addr().unwrap()
//     );

//     let (notify_shutdown, _) = broadcast::channel(1);
//     let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);
// }
