use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{JoinHandle, Thread, sleep, spawn};
use std::time::Duration;

struct ThreadedTimer {
    duration: Duration,
    join_handle: Option<JoinHandle<()>>,
    waker: Arc<Mutex<Waker>>,
}

impl ThreadedTimer {
    fn new(duration: Duration) -> ThreadedTimer {
        Self {
            duration,
            join_handle: None,
            waker: Arc::new(Mutex::new(Waker::noop().clone())),
        }
    }
}

impl Future for ThreadedTimer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let fut = self.get_mut();
        *fut.waker.lock().expect("Thread crashed with mutex lock") = cx.waker().clone();

        if fut.join_handle.is_none() {
            let duration = fut.duration;
            let waker = fut.waker.clone();
            fut.join_handle = Some(spawn(move || {
                sleep(duration);
                waker
                    .lock()
                    .expect("Thread crashed with mutex lock")
                    .wake_by_ref();
            }));
            return Poll::Pending;
        }

        if fut
            .join_handle
            .as_ref()
            .map(|h| h.is_finished())
            .unwrap_or_default()
        {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

struct ThreadWaker {
    thread: Thread,
}

impl ThreadWaker {
    fn new() -> Self {
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

fn main() {
    // We can also Pin a future by putting it in a Box
    let mut example = Box::pin(ThreadedTimer::new(Duration::from_secs(1)));

    let waker = Arc::new(ThreadWaker::new()).into();
    let mut context = Context::from_waker(&waker);

    let mut loop_counter = 0;
    while example.as_mut().poll(&mut context) == Poll::Pending {
        print!(".");
        loop_counter += 1;
        std::thread::park();
    }

    println!();
    println!("All done!");
    println!("This time the loop was only called {loop_counter} times, yay!");
}
