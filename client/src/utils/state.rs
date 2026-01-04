use std::{cell::RefCell, rc::Rc};

pub type State<T> = Rc<RefCell<T>>;
pub fn as_state<T>(v: T) -> State<T> {
    return Rc::new(RefCell::new(v));
}
