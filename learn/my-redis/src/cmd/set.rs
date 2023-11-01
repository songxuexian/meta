use std::time::Duration;

use bytes::Bytes;

#[derive(Debug)]
pub struct Set {
    key: String,
    value: Bytes,
    expire: Option<Duration>,
}

impl Set {
    pub fn new(key: impl ToString, value: Bytes, expire: Option<Duration>) -> Self {
        Self {
            key: key.to_string(),
            value,
            expire,
        }
    }
}
