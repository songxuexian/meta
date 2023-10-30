use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};

use bytes::Bytes;
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct Store {
    pub entries: HashMap<String, Entry>,
    pub pub_sub: HashMap<String, broadcast::Sender<Bytes>>,
    pub expirations: BTreeMap<(Instant, u64), String>, // like prior queue, for scan expired key and remove it.
    pub next_id: u64,                                  // for expire key, avoid not found
    pub shutdown: bool,
}

#[derive(Debug)]
pub struct Entry {
    pub id: u64,
    pub data: Bytes,
    pub expires_at: Option<Instant>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            entries: HashMap::new(),
            pub_sub: HashMap::new(),
            expirations: BTreeMap::new(),
            next_id: 0,
            shutdown: false,
        }
    }

    pub fn next_expiration(&self) -> Option<Instant> {
        self.expirations.keys().next().map(|expire| expire.0)
    }
}
