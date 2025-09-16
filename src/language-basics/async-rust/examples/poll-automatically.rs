use async_rust::thread_timer::ThreadTimer;
use async_rust::thread_waker::ThreadWaker;
use std::ops::Add;
use std::pin::{Pin, pin};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, SystemTime};
use async_rust::fake_worker::FakeWorker;

struct Timer {
    time_to_end: SystemTime,
}

impl Timer {
    fn new(duration: Duration) -> Timer {
        Self {
            time_to_end: SystemTime::now().add(duration),
        }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.time_to_end <= SystemTime::now() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

fn loop_executor<F: Future>(future: F) -> F::Output {
    let mut pinned_future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());

    let mut loop_counter = 1;

    let result = loop {
        match pinned_future.as_mut().poll(&mut context) {
            Poll::Ready(r) => break r,
            Poll::Pending => loop_counter += 1,
        }
    };

    println!("All done!");
    println!("We called .poll() {loop_counter} times!");

    result
}

pub fn block_thread_on<F: Future>(future: F) -> F::Output {
    let mut example = pin!(future);

    let waker = Arc::new(ThreadWaker::current_thread()).into();
    let mut context = Context::from_waker(&waker);

    let mut loop_counter = 0;

    let output = loop {
        match example.as_mut().poll(&mut context) {
            Poll::Pending => {
                print!(".");
                loop_counter += 1;
                std::thread::park();
            }
            Poll::Ready(output) => break output,
        }
    };

    println!();
    println!("All done!");
    println!("This time the loop was only called {loop_counter} times, yay!");

    output
}

fn main() {
    let future = FakeWorker { work_remaining: 3 };
    loop_executor(future);

    let future = Timer::new(Duration::from_secs(1));
    loop_executor(future);

    let future = ThreadTimer::new(Duration::from_secs(1));
    loop_executor(future);

    let future = ThreadTimer::new(Duration::from_secs(1));
    block_thread_on(future);
}
