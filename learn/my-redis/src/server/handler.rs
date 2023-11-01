use std::sync::mpsc;

use crate::{connection::connect::Connection, storage::db::Db};

use super::shutdown::Shutdown;

#[derive(Debug)]
pub struct Handler {
    pub db: Db,
    pub connection: Connection,
    pub shutdown: Shutdown,
    pub _shutdown_complete: mpsc::Sender<()>,
}
