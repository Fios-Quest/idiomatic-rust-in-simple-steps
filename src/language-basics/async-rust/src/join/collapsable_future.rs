use std::cell::RefCell;
use std::ops::DerefMut;
use std::pin::Pin;
use std::task::{Context, Poll};

enum InnerCollapsableFuture<F: Future> {
    Pending(F),
    Ready(F::Output),
    Spent,
}

impl<F: Future> InnerCollapsableFuture<F> {
    fn new(future: F) -> Self {
        Self::Pending(future)
    }

    fn extract(self) -> Option<F::Output> {
        match self {
            InnerCollapsableFuture::Pending(_) => None,
            InnerCollapsableFuture::Ready(output) => Some(output),
            InnerCollapsableFuture::Spent => panic!("Attempted to extract a spent future"),
        }
    }
}

#[derive(Debug)]
pub struct InnerFutureSpentError;

impl std::error::Error for InnerFutureSpentError {}

impl std::fmt::Display for InnerFutureSpentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tried to extract output from spent future")
    }
}


pub struct CollapsableFuture<F: Future>(RefCell<InnerCollapsableFuture<F>>);

impl<F: Future> CollapsableFuture<F> {
    pub fn new(future: F) -> Self {
        Self(RefCell::new(InnerCollapsableFuture::new(future)))
    }

    /// Warning: This will drop the future if the future is not Ready
    pub fn extract(&self) -> Option<F::Output> {
        let old_value = self.0.replace(InnerCollapsableFuture::Spent);
        old_value.extract()
    }
}

impl<F: Future> Future for CollapsableFuture<F> {
    type Output = Result<(), InnerFutureSpentError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut mutable_self = self.0.borrow_mut();
        let inner_future = mutable_self.deref_mut();

        match inner_future {
            InnerCollapsableFuture::Pending(future) => {
                // SAFETY: We own the future and are not moving it
                let pinned_future = unsafe { Pin::new_unchecked(future) };
                match pinned_future.poll(cx) {
                    Poll::Ready(output) => {
                        drop(mutable_self);
                        self.0.replace(InnerCollapsableFuture::Ready(output));
                        Poll::Ready(Ok(()))
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            InnerCollapsableFuture::Ready(_) => Poll::Ready(Ok(())),
            InnerCollapsableFuture::Spent => Poll::Ready(Err(InnerFutureSpentError)),
        }
    }
}
