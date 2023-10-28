use std::{
    collections::VecDeque,
    future::{self, Future},
    pin::Pin,
    task::Context,
};

use futures::task;

pub struct MyTokio {
    tasks: VecDeque<Task>,
}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl MyTokio {
    pub fn new() -> MyTokio {
        MyTokio {
            tasks: VecDeque::new(),
        }
    }

    pub fn swapn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    pub fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}
