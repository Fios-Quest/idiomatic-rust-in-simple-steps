use async_rust::fake_work::ThreadedFakeWorker;
use async_rust::thread_waker::ThreadWaker;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

fn main() {
    // We can also Pin a future by putting it in a Box
    let mut example = Box::pin(ThreadedFakeWorker::new(Duration::from_secs(1)));

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
