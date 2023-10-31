pub mod connection;
pub mod storage;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
