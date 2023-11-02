use clap::Parser;
use dotenv::dotenv;
use my_redis::{
    client::{self, cmd::Command, error::ClientError},
    connection::error::ConnectionError,
};
use tracing::debug;

#[derive(Parser, Debug)]
#[clap(name = "my-redis-cli", version, author, about = "Issue Redis commands")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
    #[clap(name = "hostname", long, default_value = "127.0.0.1")]
    host: String,
    port: u16,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ClientError> {
    dotenv().ok();
    let cli = Cli::parse();

    debug!(cause = ?cli, "get cli" );

    let addr = format!("{}:{}", cli.host, cli.port);

    let mut client = client::connect(&addr).await?;

    match cli.command {
        Command::Ping { msg } => {
            let value = client.ping(msg).await?;
            if let Ok(tmp) = std::str::from_utf8(&value) {
                println!("\"{}\"", tmp);
            } else {
                println!("{:?}", value);
            }
        }
        Command::Get { key } => {
            if let Some(value) = client.get(&key).await? {
                if let Ok(string) = std::str::from_utf8(&value) {
                    println!("\"{}\"", string);
                } else {
                    println!("{:?}", value);
                }
            } else {
                println!("(nil)");
            }
        }
        Command::Set {
            key,
            value,
            expires: None,
        } => {
            client.set(&key, value).await?;
            println!("OK");
        }
        Command::Set {
            key,
            value,
            expires: Some(expires),
        } => {
            client.set_expires(&key, value, expires).await?;
            println!("OK");
        }
        Command::Publish { channel, message } => {
            client.publish(&channel, message).await?;
            println!("Publish OK");
        }
        Command::Subscribe { channels } => {
            if channels.is_empty() {
                return Err(
                    ConnectionError::InvalidArgument("channel(s) must be provided".into()).into(),
                );
            }
            let mut subscriber = client.subscribe(channels).await?;

            // await messages on channels
            while let Some(msg) = subscriber.next_message().await? {
                println!(
                    "got message from the channel: {}; message = {:?}",
                    msg.channel, msg.content
                );
            }
        }
    }

    Ok(())
}
