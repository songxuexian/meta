use std::time::Duration;

use bytes::Bytes;
use tokio::sync::broadcast;

pub trait KvStore {
    fn get(&self, key: &str) -> Option<Bytes>;
    fn set(&self, key: String, value: Bytes, expire: Option<Duration>);
    fn subscribe(&self, key: String) -> broadcast::Receiver<Bytes>;
    fn publish(&self, key: &str, value: Bytes) -> usize;
}
