use std::ops::Add;
use std::pin::{Pin, pin};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, SystemTime};
use async_rust::fake_work::ThreadedFakeWorker;
use async_rust::thread_waker::ThreadWaker;

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

fn execute<F: Future>(future: F) -> F::Output {
    let mut pinned_future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());

    let mut loop_counter = 0;

    let result = loop {
        match pinned_future.as_mut().poll(&mut context) {
            Poll::Ready(r) => break r,
            Poll::Pending => {
                print!(".");
                loop_counter += 1;
            }
        }
    };

    println!();
    println!("All done!");
    println!("But we called the loop {loop_counter} times, yikes!");

    result
}

pub fn block_thread_on<F: Future>(future: F) -> F::Output {
    // The pin macro also pins the thing you give it but does so by taking ownership and then
    // pinning. This does not require Heap storage which is more efficient.
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
    let future = Timer::new(Duration::from_secs(1));
    execute(future);
    
    let future = ThreadedFakeWorker::new(Duration::from_secs(1));
    execute(future);
    
    let future = ThreadedFakeWorker::new(Duration::from_secs(1));
    block_thread_on(future);
}
