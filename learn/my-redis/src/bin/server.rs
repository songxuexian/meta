use clap::Parser;
use dotenv::dotenv;

use my_redis::{logger, server, DEFAULT_PORT};
use tokio::{net::TcpListener, signal};

#[derive(Debug, Parser)]
#[clap(
    name = "my-redis-server",
    version,
    author,
    about = "A mini redis server"
)]
struct Cli {
    #[clap(long)]
    port: Option<u16>,
}

#[derive(Debug)]
enum ServerError {}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    dotenv().ok();
    logger::init();

    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    println!("Listening...");

    server::run(listener, signal::ctrl_c()).await;
    Ok(())
}
