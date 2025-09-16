use async_rust::thread_timer::ThreadTimer;
use async_rust::thread_waker::ThreadWaker;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

fn main() {
    // We can also Pin a future by putting it in a Box. This might be more useful if you know the
    // generic of a future but not its concrete type. We obviously know the concrete type here
    // though so this is a little less useful.
    let mut example = Box::pin(ThreadTimer::new(Duration::from_secs(1)));

    // This time we'll use a real Waker
    let waker = Arc::new(ThreadWaker::current_thread()).into();
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
