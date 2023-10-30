use std::{
    collections::VecDeque,
    future::{self, Future},
    pin::Pin,
    sync::Arc,
    task::Context,
};

use crossbeam::channel;
use futures::{
    lock::Mutex,
    task::{self, ArcWake},
};

pub struct MyTokio {
    receiver: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }

    fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut future = self.future.try_lock().unwrap();
        let _ = future.as_mut().poll(&mut cx);
    }

    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

impl Default for MyTokio {
    fn default() -> Self {
        Self::new()
    }
}

impl MyTokio {
    pub fn new() -> MyTokio {
        let (sender, scheduled) = channel::unbounded();
        MyTokio {
            receiver: scheduled,
            sender,
        }
    }

    pub fn swapn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    pub fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            task.poll();
        }
    }
}
