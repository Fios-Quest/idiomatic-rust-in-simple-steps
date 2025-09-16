use std::pin::Pin;
use std::task::{Context, Poll, Waker};

struct ExampleFuture;

impl Future for ExampleFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready("The future ran")
    }
}

fn main() {
    let mut example = ExampleFuture;

    let example = Pin::new(&mut example);
    let mut context = Context::from_waker(Waker::noop());

    // The work doesn't actually happen until we call `poll`
    let result = example.poll(&mut context);
    assert_eq!(result, Poll::Ready("The future ran"));
}
