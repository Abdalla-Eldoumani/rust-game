use std::rc::Rc;
use std::cell::RefCell;

pub fn shared_push(n: i32) -> Vec<i32> {
    let data: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let a = Rc::clone(&data);
    let b = Rc::clone(&data);
    let mut f1 = || {
        for i in 1..=n { a.borrow_mut().push(i); }
    };
    let mut f2 = || {
        for i in 1..=n { b.borrow_mut().push(i); }
    };
    f1();
    f2();
    data.borrow().clone()
}


