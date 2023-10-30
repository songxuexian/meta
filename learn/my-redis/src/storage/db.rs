use std::sync::{Arc, Mutex};

use tokio::{
    sync::{broadcast, Notify},
    time::{self, Instant},
};
use tracing::{debug, info};

use super::{
    kvstore::KvStore,
    store::{Entry, Store},
};

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

impl KvStore for Db {
    fn get(&self, key: &str) -> Option<bytes::Bytes> {
        let store = self.shared.store.lock().unwrap();
        store.entries.get(key).map(|entry| entry.data.clone())
    }

    fn set(&self, key: String, value: bytes::Bytes, expire: Option<time::Duration>) {
        let mut store = self.shared.store.lock().unwrap();

        let id = store.next_id;
        store.next_id += 1;

        let mut notify = false;

        // calc expires at and judge if the next expiration entry
        let expires_at = expire.map(|duration| {
            let when = Instant::now() + duration;
            notify = store
                .next_expiration()
                .map(|exp| exp > when)
                .unwrap_or(true);

            store.expirations.insert((when, id), key.clone());
            when
        });

        // insert will return the key's old kv
        let prev = store.entries.insert(
            key,
            Entry {
                id,
                data: value,
                expires_at,
            },
        );

        // set the same key is just like update, so need remove the old entry
        if let Some(prev) = prev {
            if let Some(when) = prev.expires_at {
                store.expirations.remove(&(when, prev.id));
            }
        }

        // drop the store lock
        drop(store);

        if notify {
            self.shared.background_task.notify_one();
        }
    }

    fn subscribe(&self, key: String) -> broadcast::Receiver<bytes::Bytes> {
        use std::collections::hash_map::Entry;

        let mut store = self.shared.store.lock().unwrap();

        match store.pub_sub.entry(key) {
            Entry::Occupied(e) => e.get().subscribe(),
            Entry::Vacant(e) => {
                let (tx, rx) = broadcast::channel(1024);
                e.insert(tx);
                rx
            }
        }
    }

    fn publish(&self, key: &str, value: bytes::Bytes) -> usize {
        debug!("publish: (key={},len(velue)={})", key, value.len());
        let state = self.shared.store.lock().unwrap();
        state
            .pub_sub
            .get(key)
            .map(|tx| tx.send(value).unwrap_or(0))
            .unwrap_or(0)
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

#[derive(Debug)]
pub struct DbDropGuard {
    db: Db,
}

impl DbDropGuard {
    pub fn new() -> DbDropGuard {
        DbDropGuard { db: Db::new() }
    }

    pub fn db(&self) -> Db {
        self.db.clone()
    }
}

impl Drop for DbDropGuard {
    fn drop(&mut self) {
        self.db.shutdown_purge_task();
    }
}
