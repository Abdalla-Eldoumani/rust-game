Goal: Safely share and mutate state across threads with `Arc<Mutex<...>>`.

Key ideas:
- Wrap shared data in `Arc<Mutex<T>>` so threads can clone the pointer and lock the data.
- Always lock before mutation; unwrap in controlled examples.
- Join all threads to ensure completion before reading the final value.

Example:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_count(threads: usize, iters: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..threads {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..iters { *c.lock().unwrap() += 1; }
        }));
    }
    for h in handles { let _ = h.join(); }
    *counter.lock().unwrap()
}
```

Why this matters: Understanding ownership + concurrency primitives is crucial for safe parallel Rust.




