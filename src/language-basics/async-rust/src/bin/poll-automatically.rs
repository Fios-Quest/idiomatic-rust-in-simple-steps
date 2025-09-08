use std::ops::Add;
use std::pin::{pin, Pin};
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

fn block_on<T, F: Future<Output = T>>(future: F) -> T {
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
    // We can also Pin a future by putting it in a Box
    let mut example = Box::pin(Timer::new(Duration::from_secs(1)));
    
    let mut context = Context::from_waker(Waker::noop());
    
    let mut loop_counter = 0;
    while example.as_mut().poll(&mut context) == Poll::Pending {
        print!(".");
        loop_counter += 1;
    }
    
    println!();
    println!("All done!");
    println!("But we called the loop {loop_counter} times, yikes!");
    
    block_on(Timer::new(Duration::from_secs(1)));
}
