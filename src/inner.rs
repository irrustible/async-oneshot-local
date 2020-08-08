use core::task::Waker;
use core::cell::RefCell;

#[derive(Debug)]
pub struct Inner<T> {
    core: RefCell<Core<T>>,
}

#[derive(Debug)]
struct Core<T> {
    value: Option<T>,
    send: Option<Waker>,
    recv: Option<Waker>,
    done: bool,
}

impl<T> Inner<T> {

    pub fn new() -> Self {
        Inner {
            core: RefCell::new(Core {
                value: None,
                send: None,
                recv: None,
                done: false,
            }),
        }
    }

    pub fn is_closed(&self) -> bool { self.core.borrow().done }

    // Gets the receiver's waker.
    pub fn recv(&self) -> Option<Waker> {
        self.core.borrow_mut().recv.take()
    }

    // Sets the receiver's waker.
    pub fn set_recv(&self, waker: Waker) {
        self.core.borrow_mut().recv = Some(waker);
    }

    // Gets the sender's waker.
    pub fn send(&self) -> Option<Waker> {
        self.core.borrow_mut().send.take()
    }

    // Sets the sender's waker.
    pub fn set_send(&self, waker: Waker) {
        self.core.borrow_mut().send = Some(waker);
    }

    /// Takes the value
    pub fn take_value(&self) -> Option<T> {
        self.core.borrow_mut().value.take()
    }

    pub fn set_value(&self, value: T) {
        self.core.borrow_mut().value = Some(value);
    }

    pub fn close(&self) {
        self.core.borrow_mut().done = true;
    }

}
