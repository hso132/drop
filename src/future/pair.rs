// Dependencies

use std::cell::RefCell;
use std::rc::Rc;
use super::promise::Promise;
use super::promise::Solver;
use super::promise::Shared;
use super::promise::State;

// Functions

pub fn pair<Output>() -> (Promise<Output>, Solver<Output>) {
    let state = State{value: None, waker: None, spent: false};
    let rc = Rc::new(RefCell::new(state));
    let promise = Box::pin(Shared{state: rc.clone()});
    let solver = Solver{shared: Shared{state: rc}};

    (promise, solver)
}
