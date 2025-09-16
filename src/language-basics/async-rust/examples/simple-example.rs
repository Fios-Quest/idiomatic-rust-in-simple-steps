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
    // We'll enstantiate our future and pin it
    let mut example = ExampleFuture;
    let example = Pin::new(&mut example);

    // Don't worry about context yet, it has no effect in this example
    let mut context = Context::from_waker(Waker::noop());

    // Nothing happens until we poll the future
    let result = example.poll(&mut context);
    assert_eq!(result, Poll::Ready("The future ran"));

    // The .poll() method returns a Poll enum we need to resolve
    match result {
        Poll::Ready(output) => println!("{output}"),
        Poll::Pending => panic!("This shouldn't happen!"),
    }
}