use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::thread;
use std::{future::Future, time::Instant};

use futures::lock::Mutex;

pub struct Delay {
    pub when: Instant,
    pub waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if let Some(waker) = &self.waker {
            let mut waker = waker.try_lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());
            let when = self.when;

            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }

                let waker = waker.try_lock().unwrap();
                waker.wake_by_ref();
            });
        }

        if Instant::now() >= self.when {
            Poll::Ready("done")
        } else {
            Poll::Pending
        }
    }
}
