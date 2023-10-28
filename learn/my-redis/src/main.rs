use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_secs(1);
}
