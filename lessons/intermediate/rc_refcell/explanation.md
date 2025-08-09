Goal: Share mutable state using Rc<RefCell<>>.

Key ideas:
- Rc provides shared ownership; RefCell enables interior mutability at runtime.
- Clone the Rc to share among closures; call borrow_mut to push values.

Why this matters: Combines ownership sharing with mutation in single-threaded code.


