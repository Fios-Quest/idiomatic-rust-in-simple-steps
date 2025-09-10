use std::sync::Arc;
use std::task::Wake;
use std::thread::Thread;

pub struct ThreadWaker {
    thread: Thread,
}

impl ThreadWaker {
    pub fn current_thread() -> Self {
        ThreadWaker {
            thread: std::thread::current(),
        }
    }
}

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.thread.unpark();
    }
}
