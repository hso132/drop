// Dependencies

use std::cell::RefCell;
use std::collections::VecDeque;
use std::task::Waker;

// Structs

pub struct Queue {
    queue: RefCell<VecDeque<Waker>>
}

// Implementations

impl Queue {
    pub fn new() -> Self {
        Queue{queue: RefCell::new(VecDeque::new())}
    }

    pub fn push(&self, waker: Waker) {
        self.queue.borrow_mut().push_back(waker);
    }

    pub fn step(&self) -> bool {
        match self.queue.borrow_mut().pop_front() {
            Some(waker) => {
                waker.wake();
                true
            },
            None => false
        }
    }
}

// Static variables

thread_local! {
    static QUEUE: Queue = Queue::new();
}
