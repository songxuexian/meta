use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use my_tokio::{future::Delay, tokio::MyTokio};
use tokio::sync::Notify;

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

    delay(Duration::from_secs(1)).await;

    my_tokio.run();
}

async fn delay(dur: Duration) {
    let when = Instant::now() + dur;
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    thread::spawn(move || {
        let now = Instant::now();

        if now < when {
            thread::sleep(when - now);
        }

        notify2.notify_one();
    });

    notify.notified().await;
}
