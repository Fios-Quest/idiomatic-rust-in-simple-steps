use std::cell::RefCell;
use std::ops::DerefMut;
use std::pin::{Pin, pin};
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
    let timer_1_fut = async {
        ThreadedTimer::new(Duration::from_secs(2)).await;
        println!("Timer 1 complete");
    };
    let timer_2_fut = async {
        ThreadedTimer::new(Duration::from_secs(1)).await;
        println!("Timer 2 complete");
    };

    let fut = Join::new(timer_1_fut, timer_2_fut);

    let mut example = pin!(fut);

    let waker = Arc::new(ThreadWaker::new()).into();
    let mut context = Context::from_waker(&waker);

    let mut loop_counter = 0;
    while example.as_mut().poll(&mut context).is_pending() {
        print!(".");
        loop_counter += 1;
        std::thread::park();
    }

    println!();
    println!("All done!");
    println!("This time the loop was only called {loop_counter} times, yay!");
}

enum InnerCollapsableFuture<F: Future> {
    Pending(F),
    Ready(F::Output),
    Spent,
}

impl<F: Future> InnerCollapsableFuture<F> {
    fn new(future: F) -> Self {
        Self::Pending(future)
    }

    fn extract(self) -> Option<F::Output> {
        match self {
            InnerCollapsableFuture::Pending(_) => None,
            InnerCollapsableFuture::Ready(output) => Some(output),
            InnerCollapsableFuture::Spent => panic!("Attempted to extract a spent future"),
        }
    }
}

struct InnerFutureSpentError;

struct CollapsableFuture<F: Future>(RefCell<InnerCollapsableFuture<F>>);

impl<F: Future> CollapsableFuture<F> {
    fn new(future: F) -> Self {
        Self(RefCell::new(InnerCollapsableFuture::new(future)))
    }

    /// Warning: This will drop the future if the future is not Ready
    fn extract(&self) -> Option<F::Output> {
        let old_value = self.0.replace(InnerCollapsableFuture::Spent);
        old_value.extract()
    }
}

impl<F: Future> Future for CollapsableFuture<F> {
    type Output = Result<(), InnerFutureSpentError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut mutable_self = self.0.borrow_mut();
        let inner_future = mutable_self.deref_mut();

        match inner_future {
            InnerCollapsableFuture::Pending(future) => {
                // SAFETY: We own the future and are not moving it
                let pinned_future = unsafe { Pin::new_unchecked(future) };
                match pinned_future.poll(cx) {
                    Poll::Ready(output) => {
                        drop(mutable_self);
                        self.0.replace(InnerCollapsableFuture::Ready(output));
                        Poll::Ready(Ok(()))
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            InnerCollapsableFuture::Ready(_) => Poll::Ready(Ok(())),
            InnerCollapsableFuture::Spent => {
                Poll::Ready(Err(InnerFutureSpentError))
            }
        }
    }
}

struct Join<F1: Future, F2: Future>(
    Pin<Box<CollapsableFuture<F1>>>,
    Pin<Box<CollapsableFuture<F2>>>,
);

impl<F1: Future, F2: Future> Join<F1, F2> {
    fn new(future1: F1, future2: F2) -> Self {
        Self(
            Box::pin(CollapsableFuture::new(future1)),
            Box::pin(CollapsableFuture::new(future2)),
        )
    }
}

impl<F1: Future, F2: Future> Future for Join<F1, F2> {
    type Output = Result<(F1::Output, F2::Output), InnerFutureSpentError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = self.get_mut();

        let r1 = inner.0.as_mut().poll(cx);
        let r2 = inner.1.as_mut().poll(cx);

        match (r1, r2) {
            (Poll::Ready(r1), Poll::Ready(r2)) => {
                if r1.is_err() || r2.is_err() {
                    // This _shouldn't_ happen
                    Poll::Ready(Err(InnerFutureSpentError))
                } else {
                    Poll::Ready(Ok((inner.0.extract().unwrap(), inner.1.extract().unwrap())))
                }
            }
            _ => Poll::Pending,
        }
    }
}
