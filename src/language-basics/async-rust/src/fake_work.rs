use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;

pub struct ThreadedFakeWorker {
    duration: Duration,
    join_handle: Option<JoinHandle<()>>,
    waker: Arc<Mutex<Waker>>,
}

impl ThreadedFakeWorker {
    pub fn new(duration: Duration) -> ThreadedFakeWorker {
        Self {
            duration,
            join_handle: None,
            waker: Arc::new(Mutex::new(Waker::noop().clone())),
        }
    }
}

impl Future for ThreadedFakeWorker {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let fut = self.get_mut();
        
        // We always need to update the waker whenever we're polled
        *fut.waker.lock().expect("Thread crashed with mutex lock") = cx.waker().clone();
        
        match &fut.join_handle {
            None => {
                let duration = fut.duration;
                let waker = fut.waker.clone();
                fut.join_handle = Some(spawn(move || {
                    sleep(duration);
                    waker
                        .lock()
                        .expect("Thread crashed with mutex lock")
                        .wake_by_ref();
                }));
                Poll::Pending
            }
            Some(join_handler) => {
                match join_handler.is_finished() {
                    true => Poll::Ready(()),
                    false => Poll::Pending,
                }
            }
        }
    }
}
