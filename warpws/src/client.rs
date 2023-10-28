use std::net::SocketAddr;

use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

#[derive(Debug, Clone)]
pub struct Client {
    // conn: WebSocket, //ws.Conn
    pub disconnected: bool,
    pub addr: Option<SocketAddr>,
    pub user: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}
