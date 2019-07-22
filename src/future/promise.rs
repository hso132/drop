// Dependencies

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

// Aliases

pub type Promise<Output> = Pin<Box<Shared<Output>>>;

// Structs

pub struct Solver<Output> {
    pub(super) shared: Shared<Output>
}

pub struct Shared<Output> {
    pub(super) state: Rc<RefCell<State<Output>>>
}

pub(super) struct State<Output> {
    pub(super) value: Option<Output>,
    pub(super) waker: Option<Waker>,
    pub(super) spent: bool
}

// Implementations

impl<Output> Future for Shared<Output> {
    type Output = Output;
    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        const PANIC_MESSAGE: &str = "Cannot `await` twice on the same `Promise`.";
        let mut state = self.state.borrow_mut();

        if state.spent { panic!(PANIC_MESSAGE); }

        if state.value.is_some() {
            if state.waker.is_none() || state.waker.as_ref().unwrap().will_wake(context.waker()) {
                state.spent = true;
                Poll::Ready(state.value.take().unwrap())
            } else { panic!(PANIC_MESSAGE) }
        } else {
            if state.waker.is_none() {
                state.waker = Some(context.waker().clone());
                Poll::Pending
            } else { panic!(PANIC_MESSAGE) }
        }
    }
}
