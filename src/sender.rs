use crate::*;
use alloc::rc::Rc;
use core::task::Poll;
use futures_micro::poll_state;

/// The sending half of a oneshot channel.
#[derive(Debug)]
pub struct Sender<T> {
    inner: Rc<Inner<T>>,
    done: bool,
}

impl<T> Sender<T> {

    pub(crate) fn new(inner: Rc<Inner<T>>) -> Self {
        Sender { inner, done: false }
    }
        
    /// Closes the channel by causing an immediate drop
    pub fn close(self) { }

    /// true if the channel is closed
    pub fn is_closed(&self) -> bool { self.inner.is_closed() }

    // /// TODO: There is a bug in this i cannot be bothered to find right now.
    // /// Waits for a Receiver to be waiting for us to send something
    // /// (i.e. allows you to produce a value to send on demand).
    // /// Fails if the Receiver is dropped.
    // pub async fn wait(self) -> Result<Self, Closed> {
    //     poll_state(Some(self), |this, ctx| {
    //         let mut that = this.take().unwrap();
    //         if that.inner.is_closed() {
    //             that.done = true;
    //             Poll::Ready(Err(Closed()))
    //         } else if let Some(recv) = that.inner.recv() {
    //             that.inner.set_recv(recv);
    //             that.done = true;
    //             Poll::Ready(Ok(that))
    //         } else {
    //             that.inner.set_send(ctx.waker().clone());
    //             *this = Some(that);
    //             Poll::Pending
    //         }
    //     }).await
    // }

    /// Sends a message on the channel. Fails if the Receiver is dropped.
    pub fn send(mut self, value: T) -> Result<(), Closed> {
        self.done = true;
        if !self.inner.is_closed() {
            self.inner.set_value(value);
            maybe_wake(self.inner.recv());
            Ok(())
        } else {
            Err(Closed())
        }
    }        
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        if !self.done {
            if !self.inner.is_closed() {
                self.inner.close();
                maybe_wake(self.inner.recv());
            }            
        }
    }
}
