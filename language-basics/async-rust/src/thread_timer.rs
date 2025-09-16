use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;

pub struct ThreadTimer {
    duration: Duration,
    join_handle: Option<JoinHandle<()>>,
    waker: Arc<Mutex<Waker>>,
    is_complete: Arc<Mutex<bool>>,
}

impl ThreadTimer {
    pub fn new(duration: Duration) -> ThreadTimer {
        Self {
            duration,
            join_handle: None,
            waker: Arc::new(Mutex::new(Waker::noop().clone())),
            is_complete: Arc::new(Mutex::new(false)),
        }
    }
}

impl Future for ThreadTimer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let fut = self.get_mut();

        // We always need to update the waker whenever we're polled
        *fut.waker.lock().expect("Thread crashed with mutex lock") = cx.waker().clone();

        // If we haven't started the thread, do so now
        if fut.join_handle.is_none() {
            let duration = fut.duration;
            let waker = fut.waker.clone();
            let timer_complete = fut.is_complete.clone();
            fut.join_handle = Some(spawn(move || {
                sleep(duration);
                *timer_complete
                    .lock()
                    .expect("Thread crashed with mutex lock") = true;
                waker
                    .lock()
                    .expect("Thread crashed with mutex lock")
                    .wake_by_ref();
            }));
        }

        match *fut
            .is_complete
            .lock()
            .expect("Thread crashed with mutex lock")
        {
            true => Poll::Ready(()),
            false => Poll::Pending,
        }
    }
}
