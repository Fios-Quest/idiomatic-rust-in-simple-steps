use crate::thread_waker::ThreadWaker;
use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub fn block_thread_on<F: Future>(future: F) -> F::Output {
    let mut example = pin!(future);

    let waker = Arc::new(ThreadWaker::new()).into();
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
