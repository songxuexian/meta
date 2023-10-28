use std::time::{Duration, Instant};

use my_tokio::{future::Delay, tokio::MyTokio};

#[tokio::main]
async fn main() {
    let my_tokio = MyTokio::new();
    my_tokio.swapn(async {
        let when = Instant::now() + Duration::from_secs(1);
        let future = Delay { when, waker: None };

        let out = future.await;
        println!("{}", out);
        assert!(out == "done")
    });

    my_tokio.run();
}
