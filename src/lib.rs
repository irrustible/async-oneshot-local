//! [`async_oneshot`]'s single-threaded counterpart.
//!
//! Unique feature: wait for receiver to be waiting.
//!
//! Also supports the full range of things you'd expect.
#![no_std]

extern crate alloc;
use alloc::rc::Rc;
use core::task::Waker;

mod inner;
pub(crate) use inner::Inner;

mod sender;
pub use sender::Sender;

mod receiver;
pub use receiver::Receiver;

/// Create a new oneshot channel pair.
pub fn oneshot<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Rc::new(Inner::new());
    let sender = Sender::new(inner.clone());
    let receiver = Receiver::new(inner);
    (sender, receiver)
}

/// An empty struct that signifies the channel is closed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Closed();

/// We couldn't receive a message.
#[derive(Debug)]
pub enum TryRecvError<T> {
    /// The Sender didn't send us a message yet.
    Empty(Receiver<T>),
    /// The Sender has dropped.
    Closed,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// rhymes with "wake and bake" hehehe
pub(crate) fn maybe_wake(w: Option<Waker>) {
    if let Some(waker) = w { waker.wake(); }
}
// rhymes with "wake and bake" hehehe
pub(crate) fn take_and_wake(w: &mut Option<Waker>) {
    maybe_wake(w.take())
}
