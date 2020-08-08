use crate::*;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

/// The receiving half of a oneshot channel.
#[derive(Debug)]
pub struct Receiver<T> {
    inner: Rc<Inner<T>>,
    done: bool,
}

impl<T> Receiver<T> {

    pub(crate) fn new(inner: Rc<Inner<T>>) -> Self {
        Receiver { inner, done: false }
    }

    /// Closes the channel by causing an immediate drop.
    pub fn close(self) { }

    /// Attempts to receive. On failure, if the channel is not closed,
    /// returns self to try again.
    pub fn try_recv(mut self) -> Result<T, TryRecvError<T>> {
        if let Some(value) = self.inner.take_value() {
            self.done = true;
            Ok(value)
        } else if self.inner.is_closed() {
            self.done = true;
            Err(TryRecvError::Closed)
        } else {
            Err(TryRecvError::Empty(self))
        }
    }
}

impl<T> Future for Receiver<T> {
    type Output = Result<T, Closed>;
    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Result<T, Closed>> {
        let this = Pin::into_inner(self);
        if let Some(value) = this.inner.take_value() {
            this.done = true;
            Poll::Ready(Ok(value))
        } else if this.inner.is_closed() {
            this.done = true;
            Poll::Ready(Err(Closed()))
        } else {
            this.inner.set_recv(ctx.waker().clone());
            Poll::Pending
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        if !self.done {
            self.inner.close();
            take_and_wake(&mut self.inner.send());
        }
    }
}
