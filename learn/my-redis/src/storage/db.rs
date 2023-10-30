use std::sync::{Arc, Mutex};

use tokio::{
    sync::Notify,
    time::{self, Instant},
};
use tracing::info;

use super::store::Store;

#[derive(Debug, Clone)]
pub struct Db {
    shared: Arc<SharedDb>,
}

impl Db {
    pub fn new() -> Db {
        let shared = Arc::new(SharedDb::new());
        tokio::spawn(Db::purge_expired_tasks(shared.clone()));
        Db { shared }
    }

    async fn purge_expired_tasks(shared: Arc<SharedDb>) {
        while !shared.is_shutdown() {
            if let Some(when) = shared.purge_expired_keys() {
                tokio::select! {
                    _ = time::sleep_until(when) =>{},
                    _ = shared.background_task.notified()=>{},
                }
            } else {
                shared.background_task.notified().await;
            }

            info!("Purge background task shut down")
        }
    }

    fn shutdown_purge_task(&self) {
        let mut store = self.shared.store.lock().unwrap();
        store.shutdown = true;
        drop(store);
        self.shared.background_task.notify_one();
    }
}

#[derive(Debug)]
struct SharedDb {
    store: Mutex<Store>,
    background_task: Notify,
}

impl SharedDb {
    fn new() -> Self {
        SharedDb {
            store: Mutex::new(Store::new()),
            background_task: Notify::new(),
        }
    }

    fn purge_expired_keys(&self) -> Option<Instant> {
        let mut store = self.store.lock().unwrap();
        if store.shutdown {
            return None;
        }
        let store = &mut *store;

        let now = Instant::now();
        while let Some((&(when, id), key)) = store.expirations.iter().next() {
            if when > now {
                return Some(when);
            }

            store.entries.remove(key);
            store.expirations.remove(&(when, id));
        }

        None
    }

    fn is_shutdown(&self) -> bool {
        self.store.lock().unwrap().shutdown
    }
}
