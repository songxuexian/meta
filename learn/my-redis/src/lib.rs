pub mod client;
pub mod cmd;
pub mod connection;
pub mod logger;
pub mod server;
pub mod storage;

pub const DEFAULT_PORT: u16 = 16379;
pub const MAX_CONNECTIONS: usize = 1024;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
pub static LOG_LEVEL: &str = "LOG_LEVEL";
