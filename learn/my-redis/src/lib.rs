pub mod cmd;
pub mod connection;
pub mod server;
pub mod storage;

pub const DEFAULT_PORT: u16 = 16379;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
