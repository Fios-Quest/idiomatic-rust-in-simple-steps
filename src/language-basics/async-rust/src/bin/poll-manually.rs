use std::pin::Pin;
use std::task::{Context, Poll, Waker};

struct ExampleFuture {
    work_remaining: u8,
}

impl Future for ExampleFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.work_remaining {
            0 => Poll::Ready("All done!"),
            _ => {
                self.get_mut().work_remaining -= 1;
                Poll::Pending
            }
        }
    }
}

fn main() {
    // We can also Pin a future by putting it in a Box
    let mut example = Box::pin(ExampleFuture { work_remaining: 3 });

    let mut context = Context::from_waker(Waker::noop());

    // The as_mut method will then give us a Pin of the Future
    assert_eq!(example.as_mut().poll(&mut context), Poll::Pending);

    // The pin is consumed by poll, so we need to repin each time
    assert_eq!(example.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(example.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(
        example.as_mut().poll(&mut context),
        Poll::Ready("All done!")
    );
}
