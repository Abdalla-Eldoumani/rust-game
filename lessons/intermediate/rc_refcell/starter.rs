use std::rc::Rc;
use std::cell::RefCell;

/// Share a vector across closures and push values 1..=n from two closures.
/// Return the final vector contents sorted.
pub fn shared_push(n: i32) -> Vec<i32> {
    let data: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    // TODO: clone Rc, borrow_mut, and push
    data.borrow().clone()
}


