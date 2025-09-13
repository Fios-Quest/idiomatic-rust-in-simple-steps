use crate::thread_waker::ThreadWaker;
use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub fn block_thread_on<F: Future>(future: F) -> F::Output {
    let mut example = pin!(future);

    let waker = Arc::new(ThreadWaker::current_thread()).into();
    let mut context = Context::from_waker(&waker);
    
    loop {
        match example.as_mut().poll(&mut context) {
            Poll::Pending => std::thread::park(),
            Poll::Ready(output) => break output,
        }
    }
}
