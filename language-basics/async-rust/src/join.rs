use std::pin::Pin;
use std::task::{Context, Poll};

pub mod collapsable_future;

use crate::join::collapsable_future::{CollapsableFuture, InnerFutureSpentError};

pub struct Join<F1: Future, F2: Future>(
    Pin<Box<CollapsableFuture<F1>>>,
    Pin<Box<CollapsableFuture<F2>>>,
);

impl<F1: Future, F2: Future> Join<F1, F2> {
    pub fn new(future1: F1, future2: F2) -> Self {
        Self(
            Box::pin(CollapsableFuture::new(future1)),
            Box::pin(CollapsableFuture::new(future2)),
        )
    }
}

impl<F1: Future, F2: Future> Future for Join<F1, F2> {
    type Output = Result<(F1::Output, F2::Output), InnerFutureSpentError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = self.get_mut();

        let r1 = inner.0.as_mut().poll(cx);
        let r2 = inner.1.as_mut().poll(cx);

        match (r1, r2) {
            (Poll::Ready(r1), Poll::Ready(r2)) => {
                if r1.is_err() || r2.is_err() {
                    // This _shouldn't_ happen
                    Poll::Ready(Err(InnerFutureSpentError))
                } else {
                    Poll::Ready(Ok((inner.0.extract().unwrap(), inner.1.extract().unwrap())))
                }
            }
            _ => Poll::Pending,
        }
    }
}
