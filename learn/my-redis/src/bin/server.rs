use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use clap::Parser;
use dotenv::dotenv;
use mini_redis::{
    Command::{self, Get, Set},
    Connection, Frame,
};
use my_redis::DEFAULT_PORT;
use tokio::{
    net::{TcpListener, TcpStream},
    signal,
};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

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
    let cli = init();
    let port = cli.port.unwrap_or(DEFAULT_PORT);
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    println!("Listening...");

    // server::run(listener, signal::ctrl_c()).await;
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }

    Ok(())
}

fn init() -> Cli {
    dotenv().ok();
    // logger::init();
    Cli::parse()
}

#[allow(dead_code)]
fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

async fn process(socket: TcpStream, db: Db) {
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
