use tokio::net::{TcpStream, ToSocketAddrs};

use crate::connection::{connect::Connection, error::ConnectionError};

use self::cli::Client;

pub mod cli;
pub mod cmd;
pub mod error;
pub mod subscriber;

pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client, ConnectionError> {
    let socket = TcpStream::connect(addr).await?;
    let connection = Connection::new(socket);

    Ok(Client { connection })
}
