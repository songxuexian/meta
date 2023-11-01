use std::{net::Shutdown, sync::mpsc};

use crate::{connection::connect::Connection, storage::db::Db};

#[derive(Debug)]
pub struct Handler {
    pub db: Db,
    pub connection: Connection,
    pub shutdown: Shutdown,
    pub _shutdown_complete: mpsc::Sender<()>,
}
