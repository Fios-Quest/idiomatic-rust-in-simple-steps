use std::ops::Add;
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, SystemTime};

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

fn main() {
    let future = Timer::new(Duration::from_secs(1));

    execute(future);
}
